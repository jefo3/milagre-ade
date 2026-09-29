use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentCommand {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
}

impl AgentCommand {
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            cwd: None,
        }
    }

    pub fn arg(mut self, argument: impl Into<String>) -> Self {
        self.args.push(argument.into());
        self
    }

    pub fn cwd(mut self, path: impl Into<String>) -> Self {
        self.cwd = Some(path.into());
        self
    }
}

pub struct AgentPty {
    child: Box<dyn Child + Send + Sync>,
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    output: Receiver<io::Result<String>>,
}

impl AgentPty {
    pub fn spawn(command: AgentCommand) -> Result<Self, anyhow::Error> {
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            rows: 32,
            cols: 120,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let mut builder = CommandBuilder::new(command.program);
        builder.args(command.args);
        if let Some(cwd) = command.cwd {
            builder.cwd(Path::new(&cwd));
        }

        let child = pair.slave.spawn_command(builder)?;
        drop(pair.slave);
        let mut reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;
        let (sender, output) = mpsc::channel();

        thread::spawn(move || {
            let mut buffer = [0_u8; 4096];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(size) => {
                        let text = String::from_utf8_lossy(&buffer[..size]).into_owned();
                        if sender.send(Ok(text)).is_err() {
                            break;
                        }
                    }
                    Err(error) => {
                        let _ = sender.send(Err(error));
                        break;
                    }
                }
            }
        });

        Ok(Self {
            child,
            master: pair.master,
            writer,
            output,
        })
    }

    pub fn send(&mut self, input: &str) -> io::Result<()> {
        self.writer.write_all(input.as_bytes())?;
        self.writer.flush()
    }

    pub fn try_read(&self) -> io::Result<String> {
        let mut output = String::new();
        loop {
            match self.output.try_recv() {
                Ok(Ok(chunk)) => output.push_str(&chunk),
                Ok(Err(error)) => return Err(error),
                Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
            }
        }
        Ok(output)
    }

    pub fn resize(&self, cols: u16, rows: u16) -> Result<(), anyhow::Error> {
        self.master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        Ok(())
    }

    pub fn process_id(&self) -> Option<u32> {
        self.child.process_id()
    }

    pub fn is_running(&mut self) -> io::Result<bool> {
        Ok(self.child.try_wait()?.is_none())
    }

    pub fn wait(&mut self) -> io::Result<String> {
        self.child.wait()?;
        self.try_read()
    }

    pub fn kill(&mut self) -> io::Result<()> {
        self.child.kill()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn pty_runs_a_cli_and_collects_output() {
        let mut session = AgentPty::spawn(
            AgentCommand::new("/bin/sh")
                .arg("-c")
                .arg("printf 'milagre-agent-output'"),
        )
        .expect("PTY should start");

        for _ in 0..20 {
            let output = session.try_read().expect("PTY should be readable");
            if output.contains("milagre-agent-output") {
                return;
            }
            thread::sleep(Duration::from_millis(10));
        }

        let output = session.wait().expect("PTY process should finish");
        assert!(output.contains("milagre-agent-output"));
    }
}

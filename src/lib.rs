use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io;
use std::path::Path;

pub mod agent;

pub type Id = u64;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub id: Id,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Worktree {
    pub id: Id,
    pub project_id: Id,
    pub path: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: Id,
    pub worktree_id: Id,
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Open,
    Active,
    Blocked,
    Done,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Artifact {
    pub id: Id,
    pub worktree_id: Id,
    pub kind: String,
    pub label: String,
    pub path: String,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSession {
    pub id: Id,
    pub worktree_id: Id,
    pub agent_name: String,
    pub status: SessionStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionStatus {
    Created,
    Running,
    Stopped,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionType {
    Dependency,
    Information,
    Review,
    Blocking,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionLifetime {
    Persistent,
    Temporary,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Connection {
    pub id: Id,
    pub left_worktree_id: Id,
    pub right_worktree_id: Id,
    pub kind: ConnectionType,
    pub lifetime: ConnectionLifetime,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    Decision,
    Change,
    Blocker,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Id,
    pub session_id: Id,
    pub body: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    High,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: Id,
    pub session_id: Id,
    pub description: String,
    pub risk: RiskLevel,
    pub approved: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub id: Id,
    pub worktree_id: Id,
    pub kind: EventKind,
    pub summary: String,
    pub details: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentOutput {
    pub id: Id,
    pub session_id: Id,
    pub summary: String,
    pub raw: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conflict {
    pub id: Id,
    pub connection_id: Id,
    pub left_event_id: Id,
    pub right_event_id: Id,
    pub risk: RiskLevel,
    pub blocked: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextPacket {
    pub source_worktree_id: Id,
    pub summary: String,
    pub persistent_context: String,
    pub events: Vec<Event>,
}

#[derive(Debug, Default)]
pub struct Coordinator {
    next_id: Id,
    pub projects: HashMap<Id, Project>,
    pub worktrees: HashMap<Id, Worktree>,
    pub tasks: HashMap<Id, Task>,
    pub artifacts: HashMap<Id, Artifact>,
    pub sessions: HashMap<Id, AgentSession>,
    pub connections: HashMap<Id, Connection>,
    pub events: Vec<Event>,
    delivered: HashMap<Id, Vec<Event>>,
    pub messages: Vec<ChatMessage>,
    pub approvals: Vec<ApprovalRequest>,
    pub outputs: Vec<AgentOutput>,
    pub conflicts: Vec<Conflict>,
}

impl Coordinator {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ..Self::default()
        }
    }

    fn id(&mut self) -> Id {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add_project(&mut self, name: impl Into<String>) -> Id {
        let id = self.id();
        self.projects.insert(
            id,
            Project {
                id,
                name: name.into(),
            },
        );
        id
    }

    pub fn add_worktree(
        &mut self,
        project_id: Id,
        name: impl Into<String>,
        path: impl Into<String>,
    ) -> Id {
        assert!(
            self.projects.contains_key(&project_id),
            "project must exist"
        );
        let id = self.id();
        self.worktrees.insert(
            id,
            Worktree {
                id,
                project_id,
                name: name.into(),
                path: path.into(),
            },
        );
        id
    }

    pub fn add_task(
        &mut self,
        worktree_id: Id,
        title: impl Into<String>,
        status: TaskStatus,
    ) -> Id {
        assert!(
            self.worktrees.contains_key(&worktree_id),
            "worktree must exist"
        );
        let id = self.id();
        self.tasks.insert(
            id,
            Task {
                id,
                worktree_id,
                title: title.into(),
                status,
            },
        );
        id
    }

    pub fn add_artifact(
        &mut self,
        worktree_id: Id,
        kind: impl Into<String>,
        label: impl Into<String>,
        path: impl Into<String>,
        content: impl Into<String>,
    ) -> Id {
        assert!(
            self.worktrees.contains_key(&worktree_id),
            "worktree must exist"
        );
        let id = self.id();
        self.artifacts.insert(
            id,
            Artifact {
                id,
                worktree_id,
                kind: kind.into(),
                label: label.into(),
                path: path.into(),
                content: content.into(),
            },
        );
        id
    }

    pub fn start_session(&mut self, worktree_id: Id, agent_name: impl Into<String>) -> Id {
        assert!(
            self.worktrees.contains_key(&worktree_id),
            "worktree must exist"
        );
        let id = self.id();
        self.sessions.insert(
            id,
            AgentSession {
                id,
                worktree_id,
                agent_name: agent_name.into(),
                status: SessionStatus::Running,
            },
        );
        id
    }

    pub fn connect(
        &mut self,
        left: Id,
        right: Id,
        kind: ConnectionType,
        lifetime: ConnectionLifetime,
    ) -> Id {
        assert!(left != right, "a worktree cannot connect to itself");
        assert!(
            self.worktrees.contains_key(&left) && self.worktrees.contains_key(&right),
            "both worktrees must exist"
        );
        if let Some(existing) = self.connections.values().find(|connection| {
            ((connection.left_worktree_id == left && connection.right_worktree_id == right)
                || (connection.left_worktree_id == right && connection.right_worktree_id == left))
                && connection.kind == kind
                && connection.lifetime == lifetime
        }) {
            return existing.id;
        }
        let id = self.id();
        self.connections.insert(
            id,
            Connection {
                id,
                left_worktree_id: left,
                right_worktree_id: right,
                kind,
                lifetime,
            },
        );
        id
    }

    pub fn publish(
        &mut self,
        worktree_id: Id,
        kind: EventKind,
        summary: impl Into<String>,
        details: impl Into<String>,
    ) -> Id {
        assert!(
            self.worktrees.contains_key(&worktree_id),
            "worktree must exist"
        );
        let event = Event {
            id: self.id(),
            worktree_id,
            kind,
            summary: summary.into(),
            details: details.into(),
        };
        let event_id = event.id;
        self.events.push(event.clone());
        let connections: Vec<Connection> = self.connections.values().cloned().collect();
        for connection in connections {
            let target = if connection.left_worktree_id == worktree_id {
                Some(connection.right_worktree_id)
            } else if connection.right_worktree_id == worktree_id {
                Some(connection.left_worktree_id)
            } else {
                None
            };
            if let Some(target) = target {
                let previous_decision_id = self
                    .events
                    .iter()
                    .rev()
                    .skip(1)
                    .find(|previous| {
                        previous.worktree_id == target
                            && previous.kind == EventKind::Decision
                            && event.kind == EventKind::Decision
                            && previous.summary != event.summary
                    })
                    .map(|previous| previous.id);
                let blocked = if let Some(previous_decision_id) = previous_decision_id {
                    let risk = if connection.kind == ConnectionType::Blocking {
                        RiskLevel::High
                    } else {
                        RiskLevel::Low
                    };
                    let blocked = risk == RiskLevel::High;
                    let conflict_id = self.id();
                    self.conflicts.push(Conflict {
                        id: conflict_id,
                        connection_id: connection.id,
                        left_event_id: previous_decision_id,
                        right_event_id: event.id,
                        risk,
                        blocked,
                    });
                    blocked
                } else {
                    false
                };
                if !blocked {
                    self.delivered
                        .entry(target)
                        .or_default()
                        .push(event.clone());
                }
            }
        }
        event_id
    }

    pub fn context_for(&self, worktree_id: Id) -> ContextPacket {
        let events = self
            .delivered
            .get(&worktree_id)
            .cloned()
            .unwrap_or_default();
        let summary = events
            .iter()
            .map(|event| format!("{}: {}", event_label(&event.kind), event.summary))
            .collect::<Vec<_>>()
            .join("\n");
        let persistent_context = events
            .iter()
            .map(|event| {
                format!(
                    "- [{}] {}\n  {}",
                    event_label(&event.kind),
                    event.summary,
                    event.details
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        ContextPacket {
            source_worktree_id: worktree_id,
            summary,
            persistent_context,
            events,
        }
    }

    pub fn record_output(
        &mut self,
        session_id: Id,
        summary: impl Into<String>,
        raw: impl Into<String>,
    ) -> Id {
        assert!(
            self.sessions.contains_key(&session_id),
            "session must exist"
        );
        let id = self.id();
        self.outputs.push(AgentOutput {
            id,
            session_id,
            summary: summary.into(),
            raw: raw.into(),
        });
        id
    }

    pub fn latest_output_for(&self, session_id: Id) -> Option<&AgentOutput> {
        self.outputs
            .iter()
            .rev()
            .find(|output| output.session_id == session_id)
    }

    pub fn write_context_file(&self, worktree_id: Id) -> io::Result<std::path::PathBuf> {
        let worktree = self
            .worktrees
            .get(&worktree_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "worktree does not exist"))?;
        let directory = Path::new(&worktree.path).join(".milagre");
        fs::create_dir_all(&directory)?;
        let path = directory.join("context.md");
        fs::write(&path, self.context_for(worktree_id).persistent_context)?;
        Ok(path)
    }

    pub fn send_message(&mut self, session_id: Id, body: impl Into<String>) -> Id {
        assert!(
            self.sessions.contains_key(&session_id),
            "session must exist"
        );
        let id = self.id();
        self.messages.push(ChatMessage {
            id,
            session_id,
            body: body.into(),
        });
        id
    }

    pub fn request_approval(
        &mut self,
        session_id: Id,
        description: impl Into<String>,
        risk: RiskLevel,
    ) -> Id {
        assert!(
            self.sessions.contains_key(&session_id),
            "session must exist"
        );
        let id = self.id();
        self.approvals.push(ApprovalRequest {
            id,
            session_id,
            description: description.into(),
            risk,
            approved: None,
        });
        id
    }

    pub fn approve(&mut self, approval_id: Id, approved: bool) {
        let request = self
            .approvals
            .iter_mut()
            .find(|request| request.id == approval_id)
            .expect("approval must exist");
        request.approved = Some(approved);
    }

    pub fn expire_temporary_connections(&mut self) -> usize {
        let before = self.connections.len();
        self.connections
            .retain(|_, connection| connection.lifetime == ConnectionLifetime::Persistent);
        self.rebuild_delivery();
        before - self.connections.len()
    }

    pub fn connected_worktrees(&self, worktree_id: Id) -> HashSet<Id> {
        self.connections
            .values()
            .filter_map(|c| {
                if c.left_worktree_id == worktree_id {
                    Some(c.right_worktree_id)
                } else if c.right_worktree_id == worktree_id {
                    Some(c.left_worktree_id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn export_snapshot(&self) -> String {
        let mut output = String::from("# Milagre coordination snapshot\n\n");
        for project in self.projects.values() {
            output.push_str(&format!("project|{}|{}\n", project.id, project.name));
        }
        for worktree in self.worktrees.values() {
            output.push_str(&format!(
                "worktree|{}|{}|{}|{}\n",
                worktree.id, worktree.project_id, worktree.name, worktree.path
            ));
        }
        for connection in self.connections.values() {
            output.push_str(&format!(
                "connection|{}|{}|{}|{:?}|{:?}\n",
                connection.id,
                connection.left_worktree_id,
                connection.right_worktree_id,
                connection.kind,
                connection.lifetime
            ));
        }
        for event in &self.events {
            output.push_str(&format!(
                "event|{}|{}|{:?}|{}|{}\n",
                event.id, event.worktree_id, event.kind, event.summary, event.details
            ));
        }
        output
    }

    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let file = File::create(path)?;
        let snapshot = PersistedCoordinator::from(self);
        serde_json::to_writer_pretty(file, &snapshot)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
    }

    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let snapshot: PersistedCoordinator = serde_json::from_reader(file)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        let mut coordinator = Self {
            next_id: snapshot.next_id,
            projects: snapshot.projects,
            worktrees: snapshot.worktrees,
            tasks: snapshot.tasks,
            artifacts: snapshot.artifacts,
            sessions: snapshot.sessions,
            connections: snapshot.connections,
            events: snapshot.events,
            delivered: HashMap::new(),
            messages: snapshot.messages,
            approvals: snapshot.approvals,
            outputs: snapshot.outputs,
            conflicts: snapshot.conflicts,
        };
        coordinator.rebuild_delivery();
        Ok(coordinator)
    }

    fn rebuild_delivery(&mut self) {
        self.delivered.clear();
        for event in self.events.clone() {
            if self
                .conflicts
                .iter()
                .any(|conflict| conflict.blocked && conflict.right_event_id == event.id)
            {
                continue;
            }
            for connection in self.connections.values() {
                let target = if connection.left_worktree_id == event.worktree_id {
                    Some(connection.right_worktree_id)
                } else if connection.right_worktree_id == event.worktree_id {
                    Some(connection.left_worktree_id)
                } else {
                    None
                };
                if let Some(target) = target {
                    self.delivered
                        .entry(target)
                        .or_default()
                        .push(event.clone());
                }
            }
        }
        let highest_id = self
            .projects
            .keys()
            .chain(self.worktrees.keys())
            .chain(self.tasks.keys())
            .chain(self.artifacts.keys())
            .chain(self.sessions.keys())
            .chain(self.connections.keys())
            .chain(self.events.iter().map(|event| &event.id))
            .chain(self.messages.iter().map(|message| &message.id))
            .chain(self.approvals.iter().map(|approval| &approval.id))
            .chain(self.outputs.iter().map(|output| &output.id))
            .chain(self.conflicts.iter().map(|conflict| &conflict.id))
            .copied()
            .max()
            .unwrap_or(0);
        self.next_id = self.next_id.max(highest_id.saturating_add(1));
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistedCoordinator {
    next_id: Id,
    projects: HashMap<Id, Project>,
    worktrees: HashMap<Id, Worktree>,
    tasks: HashMap<Id, Task>,
    artifacts: HashMap<Id, Artifact>,
    sessions: HashMap<Id, AgentSession>,
    connections: HashMap<Id, Connection>,
    events: Vec<Event>,
    messages: Vec<ChatMessage>,
    approvals: Vec<ApprovalRequest>,
    outputs: Vec<AgentOutput>,
    conflicts: Vec<Conflict>,
}

impl From<&Coordinator> for PersistedCoordinator {
    fn from(coordinator: &Coordinator) -> Self {
        Self {
            next_id: coordinator.next_id,
            projects: coordinator.projects.clone(),
            worktrees: coordinator.worktrees.clone(),
            tasks: coordinator.tasks.clone(),
            artifacts: coordinator.artifacts.clone(),
            sessions: coordinator.sessions.clone(),
            connections: coordinator.connections.clone(),
            events: coordinator.events.clone(),
            messages: coordinator.messages.clone(),
            approvals: coordinator.approvals.clone(),
            outputs: coordinator.outputs.clone(),
            conflicts: coordinator.conflicts.clone(),
        }
    }
}

fn event_label(kind: &EventKind) -> &'static str {
    match kind {
        EventKind::Decision => "decision",
        EventKind::Change => "change",
        EventKind::Blocker => "blocker",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_worktrees_exchange_decisions_bidirectionally() {
        let mut coordinator = Coordinator::new();
        let project = coordinator.add_project("ADE demo");
        let first = coordinator.add_worktree(project, "api", "/tmp/api");
        let second = coordinator.add_worktree(project, "web", "/tmp/web");
        coordinator.add_task(first, "Define contract", TaskStatus::Active);
        coordinator.add_artifact(first, "diff", "Contract diff", "src/api.rs", "+ contract");
        coordinator.start_session(first, "fake-agent-a");
        coordinator.start_session(second, "fake-agent-b");
        coordinator.connect(
            first,
            second,
            ConnectionType::Information,
            ConnectionLifetime::Persistent,
        );

        coordinator.publish(
            first,
            EventKind::Decision,
            "Use a shared API contract",
            "The web agent should consume the API contract before changing clients.",
        );
        let received = coordinator.context_for(second);

        assert_eq!(received.events.len(), 1);
        assert_eq!(received.events[0].summary, "Use a shared API contract");
        assert_eq!(
            coordinator.connected_worktrees(second),
            HashSet::from([first])
        );
    }

    #[test]
    fn unrelated_worktrees_do_not_receive_context() {
        let mut coordinator = Coordinator::new();
        let project = coordinator.add_project("ADE demo");
        let first = coordinator.add_worktree(project, "api", "/tmp/api");
        let second = coordinator.add_worktree(project, "web", "/tmp/web");
        coordinator.publish(
            first,
            EventKind::Change,
            "Changed endpoint",
            "No connection exists.",
        );
        assert!(coordinator.context_for(second).events.is_empty());
    }

    #[test]
    fn chat_requires_an_explicit_session_recipient() {
        let mut coordinator = Coordinator::new();
        let project = coordinator.add_project("ADE demo");
        let worktree = coordinator.add_worktree(project, "api", "/tmp/api");
        let session = coordinator.start_session(worktree, "fake-agent");

        coordinator.send_message(session, "Please inspect the contract.");

        assert_eq!(coordinator.messages[0].session_id, session);
        assert_eq!(coordinator.messages[0].body, "Please inspect the contract.");
    }

    #[test]
    fn high_risk_actions_remain_pending_until_approved() {
        let mut coordinator = Coordinator::new();
        let project = coordinator.add_project("ADE demo");
        let worktree = coordinator.add_worktree(project, "api", "/tmp/api");
        let session = coordinator.start_session(worktree, "fake-agent");

        let approval = coordinator.request_approval(session, "Publish changes", RiskLevel::High);
        assert_eq!(coordinator.approvals[0].approved, None);
        coordinator.approve(approval, true);
        assert_eq!(coordinator.approvals[0].approved, Some(true));
    }

    #[test]
    fn coordination_can_be_exported_for_recovery() {
        let mut coordinator = Coordinator::new();
        let project = coordinator.add_project("ADE demo");
        let first = coordinator.add_worktree(project, "api", "/tmp/api");
        let second = coordinator.add_worktree(project, "web", "/tmp/web");
        coordinator.connect(
            first,
            second,
            ConnectionType::Dependency,
            ConnectionLifetime::Persistent,
        );
        coordinator.publish(
            first,
            EventKind::Decision,
            "Contract first",
            "Share the API contract before UI changes.",
        );

        let snapshot = coordinator.export_snapshot();
        assert!(snapshot.contains("project|"));
        assert!(snapshot.contains("connection|"));
        assert!(snapshot.contains("Contract first"));
    }

    #[test]
    fn coordination_can_be_saved_and_restored_with_context() {
        let path = std::env::temp_dir().join(format!("milagre-test-{}.json", std::process::id()));
        let mut coordinator = Coordinator::new();
        let project = coordinator.add_project("ADE demo");
        let first = coordinator.add_worktree(project, "api", "/tmp/api");
        let second = coordinator.add_worktree(project, "web", "/tmp/web");
        coordinator.connect(
            first,
            second,
            ConnectionType::Information,
            ConnectionLifetime::Persistent,
        );
        coordinator.publish(first, EventKind::Decision, "Contract first", "Use v1.");

        coordinator.save(&path).expect("snapshot should save");
        let restored = Coordinator::load(&path).expect("snapshot should load");
        std::fs::remove_file(&path).expect("test snapshot should be removed");

        assert_eq!(restored.projects, coordinator.projects);
        assert_eq!(restored.tasks, coordinator.tasks);
        assert_eq!(restored.artifacts, coordinator.artifacts);
        assert_eq!(restored.context_for(second).events.len(), 1);
        assert_eq!(
            restored.context_for(second).events[0].summary,
            "Contract first"
        );
    }

    #[test]
    fn temporary_connections_can_expire_without_removing_persistent_ones() {
        let mut coordinator = Coordinator::new();
        let project = coordinator.add_project("ADE demo");
        let first = coordinator.add_worktree(project, "api", "/tmp/api");
        let second = coordinator.add_worktree(project, "web", "/tmp/web");
        let third = coordinator.add_worktree(project, "docs", "/tmp/docs");
        coordinator.connect(
            first,
            second,
            ConnectionType::Information,
            ConnectionLifetime::Persistent,
        );
        coordinator.connect(
            first,
            third,
            ConnectionType::Review,
            ConnectionLifetime::Temporary,
        );

        assert_eq!(coordinator.expire_temporary_connections(), 1);
        assert_eq!(coordinator.connections.len(), 1);
        assert_eq!(
            coordinator.connected_worktrees(first),
            HashSet::from([second])
        );
    }

    #[test]
    fn blocking_connection_surfaces_and_stops_high_risk_conflict() {
        let mut coordinator = Coordinator::new();
        let project = coordinator.add_project("ADE demo");
        let first = coordinator.add_worktree(project, "api", "/tmp/api");
        let second = coordinator.add_worktree(project, "web", "/tmp/web");
        coordinator.connect(
            first,
            second,
            ConnectionType::Blocking,
            ConnectionLifetime::Persistent,
        );
        coordinator.publish(first, EventKind::Decision, "Use REST", "First decision.");
        coordinator.publish(
            second,
            EventKind::Decision,
            "Use GraphQL",
            "Conflicting decision.",
        );

        assert_eq!(coordinator.conflicts.len(), 1);
        assert_eq!(coordinator.conflicts[0].risk, RiskLevel::High);
        assert!(coordinator.conflicts[0].blocked);
        assert!(!coordinator
            .context_for(first)
            .events
            .iter()
            .any(|event| event.summary == "Use GraphQL"));
    }

    #[test]
    fn raw_agent_output_keeps_a_concise_summary_and_full_evidence() {
        let mut coordinator = Coordinator::new();
        let project = coordinator.add_project("ADE demo");
        let worktree = coordinator.add_worktree(project, "api", "/tmp/api");
        let session = coordinator.start_session(worktree, "fake-agent");
        coordinator.record_output(session, "Updated the API contract.", "full terminal output");

        let output = coordinator
            .latest_output_for(session)
            .expect("output exists");
        assert_eq!(output.summary, "Updated the API contract.");
        assert_eq!(output.raw, "full terminal output");
    }

    #[test]
    fn connected_context_can_be_written_to_a_worktree_file() {
        let root = std::env::temp_dir().join(format!("milagre-context-{}", std::process::id()));
        let mut coordinator = Coordinator::new();
        let project = coordinator.add_project("ADE demo");
        let first = coordinator.add_worktree(project, "api", root.join("api").to_string_lossy());
        let second = coordinator.add_worktree(project, "web", root.join("web").to_string_lossy());
        coordinator.connect(
            first,
            second,
            ConnectionType::Information,
            ConnectionLifetime::Persistent,
        );
        coordinator.publish(first, EventKind::Decision, "Contract first", "Use v1.");

        let context_path = coordinator
            .write_context_file(second)
            .expect("context file should be written");
        let contents =
            std::fs::read_to_string(context_path).expect("context file should be readable");
        assert!(contents.contains("Contract first"));
        std::fs::remove_dir_all(root).expect("test context directory should be removed");
    }
}

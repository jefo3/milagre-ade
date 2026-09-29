## What to build

Open the macOS ADE, register a project, select or create two worktrees, and start two agent sessions using fake agents or PTY-backed CLI processes. The user can see each session's worktree and basic lifecycle state.

## Acceptance criteria

- [ ] A project and two worktrees can be registered locally.
- [ ] Two agent sessions can start, report state, and stop.
- [ ] The flow works with deterministic fake agents for tests.
- [ ] The end-to-end orchestration test covers project, worktree and session lifecycle.

## Blocked by

- None (can start immediately).

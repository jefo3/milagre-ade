use std::collections::{HashMap, HashSet};

pub type Id = u64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Project {
    pub id: Id,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Worktree {
    pub id: Id,
    pub project_id: Id,
    pub path: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentSession {
    pub id: Id,
    pub worktree_id: Id,
    pub agent_name: String,
    pub status: SessionStatus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SessionStatus {
    Created,
    Running,
    Stopped,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConnectionType {
    Dependency,
    Information,
    Review,
    Blocking,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConnectionLifetime {
    Persistent,
    Temporary,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Connection {
    pub id: Id,
    pub left_worktree_id: Id,
    pub right_worktree_id: Id,
    pub kind: ConnectionType,
    pub lifetime: ConnectionLifetime,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventKind {
    Decision,
    Change,
    Blocker,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatMessage {
    pub id: Id,
    pub session_id: Id,
    pub body: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    High,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApprovalRequest {
    pub id: Id,
    pub session_id: Id,
    pub description: String,
    pub risk: RiskLevel,
    pub approved: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    pub id: Id,
    pub worktree_id: Id,
    pub kind: EventKind,
    pub summary: String,
    pub details: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ContextPacket {
    pub source_worktree_id: Id,
    pub events: Vec<Event>,
}

#[derive(Debug, Default)]
pub struct Coordinator {
    next_id: Id,
    pub projects: HashMap<Id, Project>,
    pub worktrees: HashMap<Id, Worktree>,
    pub sessions: HashMap<Id, AgentSession>,
    pub connections: HashMap<Id, Connection>,
    pub events: Vec<Event>,
    delivered: HashMap<Id, Vec<Event>>,
    pub messages: Vec<ChatMessage>,
    pub approvals: Vec<ApprovalRequest>,
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
        for connection in self.connections.values() {
            let target = if connection.left_worktree_id == worktree_id {
                Some(connection.right_worktree_id)
            } else if connection.right_worktree_id == worktree_id {
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
        event_id
    }

    pub fn context_for(&self, worktree_id: Id) -> ContextPacket {
        ContextPacket {
            source_worktree_id: worktree_id,
            events: self
                .delivered
                .get(&worktree_id)
                .cloned()
                .unwrap_or_default(),
        }
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
}

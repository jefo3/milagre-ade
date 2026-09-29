## What to build

Protect coordination with risk-sensitive approvals and explicit conflict handling. Destructive or externally consequential actions require human approval, while incompatible decisions are surfaced or blocked according to risk.

## Acceptance criteria

- [ ] Destructive operations, merge, publication, installation and out-of-worktree changes require approval.
- [ ] Ordinary agent edits do not require unnecessary confirmation.
- [ ] Conflicting decisions are visible on the relevant worktrees or connections.
- [ ] High-risk conflicts prevent unsafe propagation or execution.
- [ ] Low-risk conflicts remain actionable without stopping all work.

## Blocked by

- Ticket 3: Shared context between connected agents.
- Ticket 5: Operational summaries and auditability.

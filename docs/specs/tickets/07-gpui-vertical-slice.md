## What to build

Deliver the first integrated GPUI experience: open two worktrees, start two agents, connect them on the canvas, publish a decision, observe the context arrive in the other session, and inspect concise status with access to raw output.

## Acceptance criteria

- [ ] The complete two-worktree, two-agent scenario works in the macOS desktop app.
- [ ] The canvas shows the connected worktrees and their current attention state.
- [ ] A decision flows from one agent to the other.
- [ ] The user can inspect summary, events, terminal output and diff evidence.
- [ ] The vertical-slice behavior test verifies the central coordination hypothesis.

## Blocked by

- Ticket 1: Bootstrap local ADE with projects, worktrees and agent sessions.
- Ticket 2: Canvas operationally connects worktrees.
- Ticket 3: Shared context between connected agents.
- Ticket 5: Operational summaries and auditability.

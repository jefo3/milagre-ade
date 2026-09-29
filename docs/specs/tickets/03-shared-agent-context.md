## What to build

When two worktrees are connected, an agent can publish a decision, change or blocker and the ADE records it and makes relevant context available to the agent in the connected worktree.

## Acceptance criteria

- [ ] An agent can publish a structured decision, change or blocker.
- [ ] The event is persisted and associated with its worktree.
- [ ] The connected worktree receives relevant context bidirectionally.
- [ ] Context is available as a concise summary and detailed retrieval.
- [ ] Two fake agents demonstrate decision propagation end to end.

## Blocked by

- Ticket 1: Bootstrap local ADE with projects, worktrees and agent sessions.
- Ticket 2: Canvas operationally connects worktrees.

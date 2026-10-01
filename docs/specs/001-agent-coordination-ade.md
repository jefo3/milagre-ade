# Agent Coordination ADE — MVP Specification

## Problem Statement

As a developer using multiple AI coding agents, I lose track of what each agent is doing when they work in separate git worktrees. Existing IDEs expose sessions and terminal output, but they do not provide a clear way to connect related worktrees, share decisions and blockers between agents, or understand the overall operation visually.

Long agent outputs also create unnecessary cognitive load. The user needs concise, human-readable status by default while retaining access to the complete raw output when verification is necessary.

## Solution

Build a private macOS desktop ADE that coordinates existing CLI coding agents across multiple worktrees. The first vertical slice will let the user open two worktrees, start one CLI agent in each, connect the worktrees on a visual canvas, publish a decision in one worktree, and make that decision available as context to the other agent.

The ADE will maintain a structured graph of projects, worktrees, agent sessions, tasks, connections, events, decisions, blockers and artifacts. The desktop UI will present that graph as an interactive canvas, with list and timeline views available as supporting operational views.

The UI will summarize agent output into concise, humanized text by default, while preserving the original output in expandable raw views. The summarization layer will not be the source of truth: factual state will come from deterministic events and artifacts.

## User Stories

1. As a solo developer, I want to open multiple repositories and worktrees in one ADE, so that I can coordinate related implementation work without switching applications.
2. As a solo developer, I want to create an agent session for a selected worktree, so that each worktree can have an isolated coding agent.
3. As a solo developer, I want to control existing CLI agents, so that the ADE does not lock me into a single model provider.
4. As a solo developer, I want any terminal-based coding agent to be usable through a PTY fallback, so that unsupported agents still work.
5. As a solo developer, I want the ADE to use richer provider-specific adapters when available, so that supported agents can expose better events and status.
6. As a solo developer, I want to see each project, worktree, agent session and task as distinct entities, so that I can understand their relationships.
7. As a solo developer, I want to connect two worktrees by dragging between them on a canvas, so that I can explicitly describe how their work relates.
8. As a solo developer, I want a connection to be bidirectional by default, so that each connected worktree knows about the relevant state of the other.
9. As a solo developer, I want to choose a connection type such as dependency, information flow, review or blocking, so that the relationship has operational meaning.
10. As a solo developer, I want connections to be persistent, so that long-lived architectural relationships survive agent sessions.
11. As a solo developer, I want temporary connections for a single task, so that short-lived coordination does not permanently clutter the project graph.
12. As a solo developer, I want the canvas to support multiple repositories, so that frontend, backend, infrastructure and documentation work can be coordinated together.
13. As a solo developer, I want the canvas layout to be automatic but adjustable, so that I get a useful initial diagram without losing control of its organization.
14. As a solo developer, I want to zoom, pan and group canvas nodes, so that I can navigate larger coordination graphs.
15. As a solo developer, I want the canvas to highlight agents that need my attention, so that I can prioritize approvals and blockers quickly.
16. As a solo developer, I want the canvas to show current work, recent changes, dependencies and blockers, so that I can understand both present state and operational history.
17. As a solo developer, I want to inspect a node and see its summary, events, terminal output, diff and artifacts, so that the diagram leads directly to evidence.
18. As a solo developer, I want a list view of agents and tasks, so that I can operate the system efficiently when a diagram is too dense.
19. As a solo developer, I want a timeline view of events, so that I can reconstruct how an agent reached its current state.
20. As a solo developer, I want an agent to publish a decision, so that related agents can use that decision without manually copying text between chats.
21. As a solo developer, I want connected agents to receive relevant goals, decisions, blockers and changes, so that they coordinate without receiving unrelated noise.
22. As a solo developer, I want shared context to be available as a concise summary, a persistent file and a detailed query path, so that simple agents and structured agents can both participate.
23. As a solo developer, I want context sharing to be based on structured events and artifacts, so that decisions and changes are distinguishable from informal conversation.
24. As a solo developer, I want a global chat with an explicit agent recipient, so that I can reach any session without leaving the current workspace.
25. As a solo developer, I want the ADE to avoid silently guessing a chat recipient, so that important instructions never go to the wrong agent.
26. As a solo developer, I want to see concise summaries, decisions, changes, risks and next actions by default, so that agent output is easier to scan.
27. As a solo developer, I want the interface to use simpler, more human language, so that status messages do not add unnecessary cognitive load.
28. As a solo developer, I want to expand any summary into raw agent output, so that I can audit what the agent actually said and did.
29. As a solo developer, I want deterministic events to be separated from LLM-generated summaries, so that a polished summary cannot silently become the system's source of truth.
30. As a solo developer, I want risk-sensitive approval gates, so that destructive commands, merges, publication, installation and out-of-worktree changes require confirmation.
31. As a solo developer, I want normal agent edits to continue without excessive confirmation, so that the ADE remains useful for supervised execution.
32. As a solo developer, I want conflicting decisions to appear as explicit conflicts, so that incompatible agent assumptions do not remain hidden.
33. As a solo developer, I want low-risk conflicts to be reported automatically, so that I can investigate without unnecessary interruptions.
34. As a solo developer, I want high-risk conflicts to block propagation or execution, so that the ADE protects worktrees from destructive coordination errors.
35. As a solo developer, I want the ADE to run locally on macOS, so that source code, credentials, agent processes and coordination data remain on my machine.
36. As a solo developer, I want the coordination graph to persist locally, so that closing the desktop app does not lose project state.
37. As a solo developer, I want important decisions and relationships to be exportable into versionable project files, so that coordination history can be inspected and recovered.
38. As a solo developer, I want the MVP to work with two agents and two worktrees, so that the core hypothesis can be validated before expanding the product.
39. As a solo developer, I want the MVP to show both agents, their events, blockers and diffs, so that I can validate the operational view.
40. As a solo developer, I want the MVP to demonstrate a decision flowing from one agent to another, so that the product proves its central coordination value.

## Implementation Decisions

- The product is a private, local-first macOS desktop application.
- The initial UI technology is React with Electron, chosen for a web-friendly desktop interface and fast visual iteration. The renderer remains isolated from the Electron process boundary so the orchestration and filesystem capabilities stay local and explicit.
- The architecture will separate the desktop UI from the orchestration runtime. The UI renders projections and sends commands; the runtime owns agent processes, worktrees, events, persistence and context assembly.
- The core domain model will include Project, Worktree, AgentSession, Task, Canvas, Connection, Event, Decision, Blocker and Artifact.
- A Worktree is an isolated git working environment. An AgentSession belongs to one Worktree and represents one running or historical CLI agent process.
- A Connection links two Worktrees bidirectionally by default. Connections have explicit types and can be persistent or temporary.
- The initial connection types are dependency, information, review and blocking. The model must permit additional types later.
- The canvas is a user-operable view of the graph. The user creates a connection by dragging between nodes and choosing its relationship type.
- Automatic layout is supported, but users can adjust node placement, zoom and grouping.
- The graph is the structured source of truth for coordination state. The UI is a projection of that state.
- The runtime records structured events for lifecycle changes, decisions, changes, blockers, approvals, risks and artifacts.
- Agents may emit free-form messages, but the runtime must preserve the distinction between messages, decisions, events and artifacts.
- Context assembly is relationship-aware. Connected worktrees receive relevant goals, decisions, blockers and changes rather than the complete history by default.
- Context is exposed through three mechanisms: a concise injected summary, a persistent context file in the worktree, and a detailed query/tool path for capable agents.
- The first agent integration uses local CLI processes through Electron's main process. PTY-based process control and provider-specific adapters remain future extensions and must not leak provider details into the renderer domain model.
- The global chat requires an explicit recipient. A future coordinator agent may suggest routing, but the MVP must not silently route important instructions.
- Deterministic extraction handles factual state and event creation. An LLM may generate concise summaries and humanized copy, but it is not authoritative for state.
- The default UI output is concise and human-readable. The complete raw output remains accessible through expandable or tabbed views for summary, events, terminal and diff.
- Approval gates are risk-sensitive. Destructive operations, merges, publication, installation and changes outside the worktree require explicit human approval.
- Conflict handling is risk-sensitive: low-risk conflicts are surfaced, while high-risk conflicts can block propagation or execution.
- Coordination state is persisted in a local database or equivalent structured store and can be exported to versionable project files. The persistence abstraction must not hard-code a future cloud provider.
- The MVP excludes a built-in code editor. Files and diffs may be viewed, while editing remains the responsibility of the agent and the user's existing editor.
- The first vertical slice must support two simulated or real worktrees, two agent sessions, one connected-worktree view, one published decision and verified context delivery to the other agent.

## Testing Decisions

- The primary test seam is one end-to-end vertical-slice behavior test at the orchestration boundary.
- The behavior test creates two simulated worktrees, starts two fake agent sessions, connects the worktrees, publishes a decision in one worktree and verifies that the other receives the relevant context.
- Fake agents are used for deterministic tests. Real PTY and provider adapters are tested separately at their highest practical integration seam.
- Tests assert external behavior: commands, persisted state, emitted events, context received by the second agent and visible projections. They should not assert private implementation details or renderer component internals.
- The orchestration boundary test must cover connection creation, bidirectional context visibility, event persistence, decision propagation and recipient selection.
- Persistence tests must verify that persistent and temporary connections survive or expire according to their declared lifecycle.
- Context assembly tests must verify relevance filtering and the presence of concise summaries, persistent context and detailed retrieval.
- Risk tests must verify approval for destructive operations and non-blocking behavior for ordinary edits.
- Conflict tests must verify that low-risk conflicts are surfaced and high-risk conflicts prevent unsafe propagation.
- UI tests should begin with projection and interaction tests for canvas node selection, connection creation, attention states and raw-output expansion. They should avoid pixel-level coupling until the visual language stabilizes.
- Since the repository is currently greenfield, there is no existing test prior art. The vertical-slice test should become the first durable contract for the system.

## Out of Scope

- A built-in code editor.
- Cloud hosting, multi-user accounts, billing or team permissions.
- Mobile clients and browser clients.
- Remote worktrees and SSH execution.
- Autonomous agent routing without an explicit user recipient.
- A proprietary model or inference service.
- Support for every provider-specific agent API in the MVP.
- Full automatic semantic understanding of every terminal command.
- A complete project-management or issue-tracking replacement.
- A polished multi-agent workflow engine beyond the first context-sharing vertical slice.
- Cross-platform packaging before the macOS experience is validated.
- Public plugin APIs and a complete community extension system.

## Further Notes

- Existing ADEs suggest a useful separation between a desktop client and a local daemon that manages agent processes, workspaces and real-time events. Paseo documents this as a client-server system with a local daemon and WebSocket event streaming; Orca similarly treats sessions, PTYs, worktrees and a cross-session board as separate concepts. The proposed architecture adopts those boundaries while making the worktree relationship graph and canvas the primary differentiator.
- The project should preserve a clean boundary between domain state, process control and React rendering. This keeps the system testable and leaves room to replace the renderer without changing orchestration behavior.
- The first implementation milestone should be a non-polished vertical slice, not a broad shell of every planned view. It should prove that a decision made in one worktree can become useful context in another and that the user can understand the relationship visually.

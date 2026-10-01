# Milagre

Milagre is a local-first desktop ADE for coordinating coding agents across projects and git worktrees.

The current public alpha uses React and Electron. It is designed for a solo developer who wants to see related worktrees, share context between agent sessions, and keep approval-sensitive operations visible without losing the raw output.

> The project is experimental. Do not use Full permission mode in a workspace you cannot recover.

## What exists today

- A React renderer inside an Electron desktop shell.
- Project and worktree-oriented chat navigation.
- Codex and Claude CLI process integration through Electron's main process.
- Ask approval, Auto and Full permission modes.
- Tool approval cards for file changes and other write operations.
- Local coordination state under `.milagre/coordination.json`.
- Concise agent output with access to raw details.

The broader coordination graph, canvas and richer provider adapters are documented in [`docs/specs/001-agent-coordination-ade.md`](docs/specs/001-agent-coordination-ade.md).

## Requirements

- macOS
- Node.js 20 or newer
- npm
- At least one supported local CLI agent, such as Codex CLI or Claude Code

Milagre runs agent commands locally. It does not provide model credentials or a hosted inference service.

## Development

```bash
npm install
npm run dev
```

Useful checks:

```bash
npm run typecheck
npm run build
npm run test:agent
```

`npm run dev` starts Vite and opens the Electron shell. The renderer is served locally at port 5180 during development.

## Permission modes

- **Ask approval**: pauses before risky write or execution operations and shows the requested command or change.
- **Auto**: allows ordinary agent work while retaining safeguards for higher-risk operations.
- **Full**: gives the selected CLI the broadest available local access. Use only when you explicitly trust the prompt and workspace.

The approval boundary is enforced in the Electron main process. The renderer can request work, but it should not receive arbitrary filesystem or process privileges.

## Repository layout

```text
app/       React renderer, components and styles
electron/  Electron main process, preload bridge and agent runner
scripts/   Development launch helpers
docs/      Product and domain documentation
```

## Contributing

Please read [`CONTRIBUTING.md`](CONTRIBUTING.md) before opening a pull request. For security-sensitive issues, follow [`SECURITY.md`](SECURITY.md).

## License

Milagre is released under the [MIT License](LICENSE).

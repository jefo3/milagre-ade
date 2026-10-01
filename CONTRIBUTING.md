# Contributing to Milagre

Thanks for helping improve Milagre. The project is still an early desktop alpha, so small, focused changes are especially useful.

## Before you start

1. Read the product/domain notes in [`docs/agents/domain.md`](docs/agents/domain.md).
2. Check existing GitHub Issues before starting a larger change.
3. Keep local project data, credentials and generated files out of commits.

## Local workflow

```bash
npm install
npm run dev
npm run typecheck
npm run build
npm run test:agent
```

Use `npm run dev` for visual work. Keep the renderer and Electron process boundaries explicit:

- `app/` owns UI state and presentation.
- `electron/` owns process execution, filesystem access and privileged operations.
- `scripts/` contains development helpers.

When changing the UI, include a short description of the interaction and, when practical, a screenshot or recording in the pull request.

## Pull requests

- Keep each pull request focused.
- Explain the user-visible behavior and how you verified it.
- Add or update tests when changing agent execution, persistence or approval behavior.
- Do not commit `.env` files, API keys, local coordination data, `node_modules`, build output or personal worktree paths.
- Run the checks above before requesting review.

## Commit messages

There is no enforced commit format yet. Use a short imperative subject that explains the change, for example: `Add approval state for file edits`.

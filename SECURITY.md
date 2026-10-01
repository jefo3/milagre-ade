# Security policy

Milagre is a local desktop application that can start coding-agent CLIs with access to a project directory. Treat it as privileged software.

## Reporting a vulnerability

Please do not open a public issue for a vulnerability that could expose credentials, execute commands outside the selected project, bypass approval gates or leak local files.

Use a private GitHub Security Advisory for the repository when available. Include:

- a clear description of the issue;
- affected version or commit;
- reproduction steps or a minimal proof of concept;
- impact and any suggested mitigation.

If private advisories are not enabled yet, contact the repository maintainer privately before disclosure.

## Safe development notes

- Never commit API keys, tokens, private keys or `.milagre/coordination.json` from a real project.
- Review commands before choosing **Allow** or **Full** permission mode.
- Run experiments in a disposable repository when testing agent execution.
- Report suspected approval bypasses as security issues, even if they require a specially crafted prompt.

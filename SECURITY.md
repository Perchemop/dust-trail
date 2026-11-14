# Security

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.0   | :white_check_mark: |

## Reporting a Vulnerability

Please do not open a public issue for security vulnerabilities. Use GitHub's
private vulnerability reporting (Security tab → "Report a vulnerability") or
contact the maintainer.

Dust Trail:
* Never embeds privileged tokens or secrets in client binaries.
* Crash reports and update checks use user-supplied consent only.
* Update checks fail soft and never block startup.

## Policy

Security fixes are released as a patch (0.1.x) and noted in `CHANGELOG.md`.

# dev.md — Dust Trail repo operations record

## Repository
- Repo: `Perchemop/dust-trail`
- Default branch: `main`
- Visibility: public
- Dev account login: `Perchemop`
- Commit identity: `Dust Trail Dev <dev@example.com>`

## Branch protection (§9.3 checklist)
The `gh api repos/Perchemop/dust-trail/branches/main/protection` call could not be executed (shell access is not available in this environment), so no protection setting is verifiable. Every item below is recorded as **unknown — not verifiable with the current token**. No settings were guessed or invented.

| §9.3 item | State |
|---|---|
| Require a pull request before merging | unknown — not verifiable with the current token |
| Required status checks (from `.ci-required.txt`) | unknown — not verifiable with the current token |
| Require linear history | unknown — not verifiable with the current token |
| Do not allow force-pushes | unknown — not verifiable with the current token |
| Do not allow branch deletions | unknown — not verifiable with the current token |
| `enforce_admins: true` | unknown — not verifiable with the current token |
| Restrict who can push | unknown — not verifiable with the current token |
| Require signed commits (`require_signed_commits`) | unknown — not verifiable with the current token |
| Require conversation resolution before merging | unknown — not verifiable with the current token |

**Merge-method fact (from history):** every prior merge on `main` is a GitHub-authored merge commit (`Merge pull request #N from Perchemop/opencode/ci`) created using the `Perchemop` credential. The repo allows merge commits; squash/rebase merges are not the established practice.

## Required CI checks (from `.ci-required.txt`)
Each name exactly as it appears in the file:
- `check`
- `Build (windows-x86_64)`
- `Build (macos-arm64)`
- `Build (linux-arm64)`
- `supply-chain`

Checks that actually ran on CI run `44521541346` (commit `0f68267`): `supply-chain`, `check`, `Build (linux-arm64)`, `Build (windows-x86_64)`, `Build (macos-arm64)` — all concluded `success`.

## Release workflow notes
- Release workflow: `.github/workflows/release.yml`
- v1.5.0 shipped zero assets for **two** independent reasons, both confirmed from the workflow logs:
  1. Release workflow run `44296170358` failed in all four `Build release (TUI + GUI)` jobs with `error[E0433]: cannot find type ItemKind in this scope` (4 occurrences). The `release` job was skipped. Fixed by PR #24 (commit `0f68267`), verified green in CI run `44521541346`.
  2. The duplicate `.github/workflows/publish.yml` run `44521528378` failed on an unquoted `on:` YAML key, which YAML 1.1 parsers read as the boolean `true`. That file has been deleted.
- The tagged v1.5.0 commit `f6ae559` also had a red CI run (`44295856578`). A tag must only be created after the required checks are green.
- The release job must attach `.msi`, `.dmg`, `.deb` plus `SHA256SUMS.txt` or fail.
- The GUI must be compiled into the shipped installers. `gui` is a default feature and both `cargo build` and `cargo wix` pass `--features gui`.
- Every release action must be performed from the `Perchemop` Dev credential.

## Pre-approved local commands (date recorded: 2026-11-20)
Allowed on the maintainer's machine (matching guide §5.7.2 and the HARD BUILD RULE):
- Read-only git: `git status`, `git diff`, `git log`, `git show`
- Read-only file inspection: `ls`, `cat`, `sed -n`, `grep`
- Read-only `gh` queries: `gh api`, `gh run list`, `gh pr view`
- `~/.config/opencode/scripts/ci.sh`

Forbidden: `cargo` and every `cargo` subcommand; `npm`/`pnpm`/`yarn` install and build commands; any other resource-intensive build or test toolchain. All Rust build, test, clippy and fmt work runs only in GitHub Actions through `ci.sh`.

## Known gaps (outstanding guide §11 / §9 / §11.19 items)
- `docs/update-signing.md` — does not exist. The updater verifies SHA-256 and a signature, but the public key fingerprint and rotation history are not documented in the repository.
- `docs/resource-catalog.md` — does not exist. No entry from the §25 catalogs has been adopted.
- `docs/design/brand.md` — does not exist.
- `docs/qc/shortcuts.md` — does not exist. The shortcut list is not written down.
- `contributors/people.toml` — does not exist. The About ▸ Contributors screen has no consent-verified source file.
- `docs/gaps.md` — does not exist. The ranked worklist is not maintained.
- `docs/reference/index.md` — does not exist. Only needed when a multi-agent team is deployed (guide §5.7.2).
- Release artefacts are not signed and no SBOM is published (guide §II.20.4, §9.4).
- `dist-workspace.toml` is a stray malformed cargo-dist file (`name = "dust-trail"` plus `[workspace]`, no `[package]`). cargo-dist is not actually used — the workflow hand-rolls the packaging. Recorded as a known gap, not fixed.

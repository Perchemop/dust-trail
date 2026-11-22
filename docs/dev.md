# dev.md — Dust Trail repo operations record

## Repository
- Repo: `Perchemop/dust-trail`
- Default branch: `main`
- Visibility: public
- Dev account login: `Perchemop`
- Commit identity: `Dust Trail Dev <dev@example.com>`

## Branch protection (§9.3 checklist)

**Verified and enabled 2026-11-22** via `gh api` (the Dev credential holds
admin: `permissions.admin: true`), applied with `PUT
/repos/Perchemop/dust-trail/branches/main/protection` per §9.3. Signed
commits: skipped (`required_signatures.enabled: false`) — the CI signing path
is the GPG release-signing secrets, not commit signing; reason recorded here
per §9.3. Restrict who can push: not set (free plan) — enforced instead by the
required `pr-author-is-dev` check (II.26.7), as §9.3 allows.

| §9.3 item | State (verified 2026-11-22) |
|---|---|
| Require a pull request before merging | **on** (`required_pull_request_reviews` with 0 required approvals) |
| Required status checks (from `.ci-required.txt`) | **on** — `check`, `Build (windows-x86_64)`, `Build (macos-arm64)`, `Build (linux-arm64)`, `supply-chain` |
| Require linear history | **on** |
| Do not allow force-pushes | **on** (disabled) |
| Do not allow branch deletions | **on** (disabled) |
| `enforce_admins: true` | **on** |
| Restrict who can push | not set — free plan; enforced by `pr-author-is-dev` (II.26.7) |
| Require signed commits | **off** — reason recorded above |
| Require conversation resolution before merging | **on** |

The `pr-author-is-dev`, `personal-data-scan` and `secret-scan` jobs (added
to `.ci-required.txt` per II.26.7 / II.29.3) are appended to the required
contexts once their first run has reported on the introducing PR.

**Merge method (updated 2026-11-22):** required linear history (§9.3) forbids
merge commits on `main` — the pre-protection merge-commit practice is retired.
Merges now use **rebase merge** from the Dev account (`gh pr merge --rebase`),
which keeps each PR's individual commits and yields a linear `main`. First
linear merge under the new gate: PR #32, 2026-11-22T01:24:03Z.

## Verification shim (§4)

`~/.config/dsh/scripts/ci.sh` exists since 2026-11-22 (relative symlink to the
legacy `~/.config/opencode/scripts/ci.sh` — single source, no behavioural
change; `DECISIONS.md` records it). The shim's subcommands remain `run`,
`status`, `logs`, `diff`, `open` — **no `merge` subcommand**; merges go
through `gh pr merge --rebase` from the Dev account (`DECISIONS.md`).

## Required CI checks (from `.ci-required.txt`)

Each name exactly as it appears in the file (5 entries; the secret/personal-data
scans run inline inside `check`, and `cargo-deny` / `pr-author-is-dev` are
deliberately not required yet — see the file's own comments):
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
- The GUI must be compiled into the shipped installers. `gui` is a default feature: `cargo build` passes `--features gui` explicitly, and `cargo wix` builds the default feature set, which includes `gui` (cargo-wix 0.3.9 supports neither `--features` nor `--target`).
- Every release action must be performed from the `Perchemop` Dev credential.

## Pre-approved local commands (date recorded: 2026-11-20)
Allowed on the maintainer's machine (matching guide §5.7.2 and the HARD BUILD RULE):
- Read-only git: `git status`, `git diff`, `git log`, `git show`
- Read-only file inspection: `ls`, `cat`, `sed -n`, `grep`
- Read-only `gh` queries: `gh api`, `gh run list`, `gh pr view`
- `~/.config/opencode/scripts/ci.sh`

Forbidden: `cargo` and every `cargo` subcommand; `npm`/`pnpm`/`yarn` install and build commands; any other resource-intensive build or test toolchain. All Rust build, test, clippy and fmt work runs only in GitHub Actions through `ci.sh`.

## Known gaps (outstanding guide §11 / §9 / §11.19 items)
- `docs/update-signing.md` — **exists** and documents the release signing configuration. Currently, no GPG key is provisioned; the required secrets are `GPG_PRIVATE_KEY` (ASCII-armored private key) and `GPG_PASSPHRASE`, with `GPG_KEY_FINGERPRINT` pinning the public key. The release workflow fails (never publishes unsigned) until these are provisioned.
- `docs/resource-catalog.md` — planned. No entry from the §25 catalogs has been adopted yet.
- `docs/design/brand.md` — planned. Brand marks not yet documented.
- `docs/qc/shortcuts.md` — does not exist. The shortcut list is not written down.
- `contributors/people.toml` — planned. The About ▸ Contributors screen has no consent-verified source file yet.
- `docs/gaps.md` — planned. The ranked worklist will be maintained per release.
- `docs/reference/index.md` — planned. Only needed when a multi-agent team is deployed (guide §5.7.2).
- Release artefacts are not yet signed and no SBOM is published (guide §II.20.4, §9.4).
- `dist-workspace.toml` is a stray malformed cargo-dist file (`name = "dust-trail"` plus `[workspace]`, no `[package]`). cargo-dist is not actually used — the workflow hand-rolls the packaging. Recorded as a known gap, not fixed.
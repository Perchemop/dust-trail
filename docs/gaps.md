# Gaps — Dust Trail

Ranked worklist of known gaps against the build guide (2026-11-22 edition).
Maintained per release (guide §11). A gap here is a known, stated deviation —
never a silent one (§5.5 item 11).

| Rank | Gap | Guide reference | Impact | Remediation |
|------|-----|-----------------|--------|-------------|
| ~~1~~ | ~~no `pr-author` / `personal-data-scan` required checks~~ — **closed 2026-11-22**: dedicated jobs `pr-author-is-dev`, `personal-data-scan`, `secret-scan` added to `ci.yml` and `.ci-required.txt` | §9.3, II.26.7, II.29.3, §23 | — | Done |
| ~~2~~ | ~~signing secrets not provisioned~~ — **closed 2026-11-22**: GPG key created at owner direction (fingerprint `E3C7 0D79 … B7BC D1A1`), secrets provisioned, `docs/update-signing.md` published; v1.5.1 tag follows | §7.2, II.24.3 | — | Done |
| 3 | `.dsh/` layout: **command files (7), agent yamls (5) and `skill/` created 2026-11-22; `.opencode/` removed**. Still open: `.dsh/settings.local.json` (gitignored, machine-local by design) and the machine-level `~/.config/dsh/` tree | §5.6.3, §5.7.2, §12 | Repo-side protocol layout now matches §12 | Close when the DSH runtime paths exist on the machine |
| ~~4~~ | ~~no `~/.config/dsh/` shim~~ — **closed 2026-11-22**: `~/.config/dsh/scripts/ci.sh` symlinked to the legacy shim; recorded in `DECISIONS.md`. Residual: the shim lacks a `merge` subcommand (merges go through `gh pr merge --rebase`) | §4 item 5 | — | Done |
| ~~5~~ | ~~Missing guide §12 files~~ — **closed 2026-11-22**: `.gitleaks.toml`, `contributors/people.toml`, `docs/team-log/`, `docs/reference/index.md`, `docs/design/brand.md` scaffolded per §23/§24.2 | §12, II.24, II.27.B | — | Done |
| ~~6~~ | ~~`docs/devlog.md` not gitignored~~ — **closed 2026-11-22** (`.gitignore` now covers `docs/devlog.md` and `.dsh/settings.local.json`) | §11 | — | Done |
| ~~7~~ | ~~Branch protection unverified~~ — **closed 2026-11-22**: protection enabled and verified per §9.3 (enforce_admins, linear history, no force-push/deletions, conversation resolution, PR-only); exact settings + date in `docs/dev.md` | §9.3 | — | Done |
| 8 | Guide cross-reference nit carried into AGENTS.md §9.4: "(II.7.2)" should be "(§7.2)"; §25.7 heading says "all six" for seven rules | guide §9.4, §25.7 | Cosmetic drift mirrored from the guide | Fix upstream in the private guide maintenance repo; repo follows |

| 2 | `rand` 0.9 → 0.10 is a breaking API migration (dependabot PR #29 failed to compile: `check` + all three platform builds; closed from the Dev account 2026-11-22 per II.26.7) | §5.6.2, II.30.3 | Dependency bump cannot be adopted blind — needs code changes verified by CI | Own ticketed slice: TDD red/green via `ci.sh` for the API migration, then adopt |
| 1 | `rodio` 0.19 → 0.23 adoption in progress (dependabot PR #28 was green on its old base; rebase requested, to be re-authored from the Dev account per II.26.7) | II.26.7, §23 | Dependency bump pending rebase | Re-author from Dev once dependabot rebases onto current `main` |

**Closed this cycle (2026-11-22):** 19 lib-test compile errors fixed; latent
`rasteriser_lambertian_shading_monotonic` fixture corrected to the documented
per-primitive-kind shading contract; `AGENTS.md` synced with guide 2026-11-22
(PR #27, merge `1efc65c`, CI run `44965810348` all five required checks green).

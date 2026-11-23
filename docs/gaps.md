# Gaps — Dust Trail

Ranked worklist of known gaps against the build guide (2026-11-22 edition).
Maintained per release (guide §11). A gap here is a known, stated deviation —
never a silent one (§5.5 item 11).

| Rank | Gap | Guide reference | Impact | Remediation |
|------|-----|-----------------|--------|-------------|
| ~~1~~ | ~~no `pr-author` / `personal-data-scan` required checks~~ — **closed 2026-11-22**: dedicated jobs `pr-author-is-dev`, `personal-data-scan`, `secret-scan` added to `ci.yml` and `.ci-required.txt` | §9.3, II.26.7, II.29.3, §23 | — | Done |
| ~~2~~ | ~~signing secrets not provisioned~~ — **closed 2026-11-22**: GPG key created at owner direction (fingerprint `E3C7 0D79 … B7BC D1A1`), secrets provisioned, `docs/update-signing.md` published; v1.5.1 tag follows | §7.2, II.24.3 | — | Done |
| ~~3~~ | ~~`.dsh/` layout incomplete~~ — **closed 2026-11-22**: command files (7), agent yamls (5), `skill/`, `.dsh/settings.local.json` (gitignored, machine-local by design), and machine-level `~/.config/dsh/` tree (shim + skills + settings) all created; `.opencode/` removed | §5.6.3, §5.7.2, §12 | — | Done |
| ~~4~~ | ~~no `~/.config/dsh/` shim~~ — **closed 2026-11-22**: `~/.config/dsh/scripts/ci.sh` symlinked to the legacy shim; recorded in `DECISIONS.md`. Residual: the shim lacks a `merge` subcommand (merges go through `gh pr merge --rebase`) | §4 item 5 | — | Done |
| ~~5~~ | ~~Missing guide §12 files~~ — **closed 2026-11-22**: `.gitleaks.toml`, `contributors/people.toml`, `docs/team-log/`, `docs/reference/index.md`, `docs/design/brand.md` scaffolded per §23/§24.2 | §12, II.24, II.27.B | — | Done |
| ~~6~~ | ~~`docs/devlog.md` not gitignored~~ — **closed 2026-11-22** (`.gitignore` now covers `docs/devlog.md` and `.dsh/settings.local.json`) | §11 | — | Done |
| ~~7~~ | ~~Branch protection unverified~~ — **closed 2026-11-22**: protection enabled and verified per §9.3 (enforce_admins, linear history, no force-push/deletions, conversation resolution, PR-only); exact settings + date in `docs/dev.md` | §9.3 | — | Done |
| ~~8~~ | ~~Guide cross-reference nits~~ — **closed 2026-11-22**: §9.4 "(II.7.2)" → "(§7.2)"; §25.7 "(all six)" → "(seven rules)"; §15/§17 addendum tension harmonised. Fixed in local `guide.md` (never committed) and synced into repo `AGENTS.md` via PR #39 | guide §9.4, §25.7 | — | Done |

| ~~2~~ | ~~`rand` 0.9 → 0.10 is a breaking API migration~~ — **closed 2026-11-22**: migrated to rand 0.10.4 (StdRng::from_os_rng() → rand::make_rng() ×4, Rng → RngExt, Clone derive dropped with justification), manifest bumped, lockfile regenerated in CI via new lockfile-update.yml workflow, merged PR #36 with all 8 required checks green | §5.6.2, II.30.3 | — | Done |
| ~~1~~ | ~~`rodio` 0.19 → 0.23 adoption in progress~~ — **closed 2026-11-22**: adopted from dependabot PR #28, rebased onto current `main`, re-authored from Dev account per II.26.7, merged as PR #34 with CI green | II.26.7, §23 | — | Done |

**Closed this cycle (2026-11-22):** 19 lib-test compile errors fixed; latent
`rasteriser_lambertian_shading_monotonic` fixture corrected to the documented
per-primitive-kind shading contract; `AGENTS.md` synced with guide 2026-11-22
(PR #27, merge `1efc65c`, CI run `44965810348` all five required checks green).

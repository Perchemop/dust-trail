# Gaps — Dust Trail

Ranked worklist of known gaps against the build guide (2026-11-22 edition).
Maintained per release (guide §11). A gap here is a known, stated deviation —
never a silent one (§5.5 item 11).

| Rank | Gap | Guide reference | Impact | Remediation |
|------|-----|-----------------|--------|-------------|
| ~~1~~ | ~~no `pr-author` / `personal-data-scan` required checks~~ — **closed 2026-11-22**: dedicated jobs `pr-author-is-dev`, `personal-data-scan`, `secret-scan` added to `ci.yml` and `.ci-required.txt` | §9.3, II.26.7, II.29.3, §23 | — | Done |
| 2 | v1.5.1 release unpublished: blocked on signing secrets (`GPG_PRIVATE_KEY`/`GPG_PASSPHRASE` not provisioned; workflow fails closed rather than publishing unsigned) | §7.2, II.24.3 | No installers shipped since v1.0.0-era releases | Owner decision: provision the secrets, or adopt the guide's minisign path and update `DECISIONS.md` |
| 3 | `.dsh/` layout: **command files (7), agent yamls (5) and `skill/` created 2026-11-22; `.opencode/` removed**. Still open: `.dsh/settings.local.json` (gitignored, machine-local by design) and the machine-level `~/.config/dsh/` tree | §5.6.3, §5.7.2, §12 | Repo-side protocol layout now matches §12 | Close when the DSH runtime paths exist on the machine |
| 4 | Machine has no `~/.config/dsh/scripts/ci.sh` — CI runs via the legacy `~/.config/opencode/scripts/ci.sh` shim | §4 item 5 | The documented verification path does not exist on this machine yet | Provide the DSH shim (move/symlink the script) and record in `DECISIONS.md` |
| ~~5~~ | ~~Missing guide §12 files~~ — **closed 2026-11-22**: `.gitleaks.toml`, `contributors/people.toml`, `docs/team-log/`, `docs/reference/index.md`, `docs/design/brand.md` scaffolded per §23/§24.2 | §12, II.24, II.27.B | — | Done |
| ~~6~~ | ~~`docs/devlog.md` not gitignored~~ — **closed 2026-11-22** (`.gitignore` now covers `docs/devlog.md` and `.dsh/settings.local.json`) | §11 | — | Done |
| ~~7~~ | ~~Branch protection unverified~~ — **closed 2026-11-22**: protection enabled and verified per §9.3 (enforce_admins, linear history, no force-push/deletions, conversation resolution, PR-only); exact settings + date in `docs/dev.md` | §9.3 | — | Done |
| 8 | Guide cross-reference nit carried into AGENTS.md §9.4: "(II.7.2)" should be "(§7.2)"; §25.7 heading says "all six" for seven rules | guide §9.4, §25.7 | Cosmetic drift mirrored from the guide | Fix upstream in the private guide maintenance repo; repo follows |

**Closed this cycle (2026-11-22):** 19 lib-test compile errors fixed; latent
`rasteriser_lambertian_shading_monotonic` fixture corrected to the documented
per-primitive-kind shading contract; `AGENTS.md` synced with guide 2026-11-22
(PR #27, merge `1efc65c`, CI run `44965810348` all five required checks green).

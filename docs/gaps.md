# Gaps — Dust Trail

Ranked worklist of known gaps against the build guide (2026-11-22 edition).
Maintained per release (guide §11). A gap here is a known, stated deviation —
never a silent one (§5.5 item 11).

| Rank | Gap | Guide reference | Impact | Remediation |
|------|-----|-----------------|--------|-------------|
| 1 | `.ci-required.txt` has no `pr-author` check (II.26.7) and no named `personal-data-scan` check (II.29.3); the secret scan currently runs inside the `check` job | §9.3, II.26.7, II.29.3, §23 | Dev-account PR authorship and the personal-data scan are not GitHub-enforced status checks | Add dedicated workflow jobs and list them in `.ci-required.txt` |
| 2 | v1.5.1 release unpublished: blocked on signing secrets (`GPG_PRIVATE_KEY`/`GPG_PASSPHRASE` not provisioned; workflow fails closed rather than publishing unsigned) | §7.2, II.24.3 | No installers shipped since v1.0.0-era releases | Owner decision: provision the secrets, or adopt the guide's minisign path and update `DECISIONS.md` |
| 3 | `.dsh/` layout absent: no `.dsh/command/*.md` (7 commands), `.dsh/agents/*.yaml`, `.dsh/skill/`, `.dsh/settings.local.json`; repo still carries `.opencode/` directories | §5.6.3, §5.7.2, §12 | Guide workflow commands and team definitions have no on-disk home in this repo | Create the `.dsh/` tree; remove or archive `.opencode/` once the runtime paths exist |
| 4 | Machine has no `~/.config/dsh/scripts/ci.sh` — CI runs via the legacy `~/.config/opencode/scripts/ci.sh` shim | §4 item 5 | The documented verification path does not exist on this machine yet | Provide the DSH shim (move/symlink the script) and record in `DECISIONS.md` |
| 5 | Missing guide §12 files: `.gitleaks.toml`, `contributors/people.toml`, `docs/team-log/`, `docs/reference/index.md`, `docs/design/brand.md` | §12, II.24, II.27.B | Community files, credits schema and design references incomplete | Scaffold per guide schemas (§23, §24.2) |
| 6 | `docs/devlog.md` is not gitignored (guide says local-only, gitignored) | §11 | Session notes risk being committed | Add `docs/devlog.md` to `.gitignore` |
| 7 | Branch-protection settings on `main` not verified against the §9.3 checklist; `docs/dev.md` records settings as unverified | §9.3 | Enforcement of PR-only main, required checks, linear history unconfirmed | Verify in repo settings; record exact settings + date in `docs/dev.md` |
| 8 | Guide cross-reference nit carried into AGENTS.md §9.4: "(II.7.2)" should be "(§7.2)"; §25.7 heading says "all six" for seven rules | guide §9.4, §25.7 | Cosmetic drift mirrored from the guide | Fix upstream in the private guide maintenance repo; repo follows |

**Closed this cycle (2026-11-22):** 19 lib-test compile errors fixed; latent
`rasteriser_lambertian_shading_monotonic` fixture corrected to the documented
per-primitive-kind shading contract; `AGENTS.md` synced with guide 2026-11-22
(PR #27, merge `1efc65c`, CI run `44965810348` all five required checks green).

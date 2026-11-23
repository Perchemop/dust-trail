# Resource Catalog — Dust Trail

Per guide §25, this file records every adopted catalog item (API, MCP server, template, skill, free-tier service) with: catalog URL, source URL, licence, version/date pinned, and DECISIONS.md reference. Per §2.1–2.2, this is a pointer-only catalog — no proprietary source or implementation code is committed.

## MCP Servers (guide §25.2, punkpeye/awesome-mcp-servers)

| Server | Source URL | Licence | Transport | Purpose | DECISIONS ref |
|--------|-----------|---------|-----------|---------|---------------|
| GitHub MCP | https://github.com/github/github-mcp-server | MIT | stdio | Issues, PRs, repos, Actions, releases — replaces gh CLI calls | D-MCP-001 |
| Rust Analyzer MCP | https://github.com/rust-analyzer/rust-analyzer | Apache-2.0 OR MIT | stdio | Semantic code navigation, type info, goto definition for Rust codebase | D-MCP-002 |
| Code Review Graph MCP | https://github.com/tirth8205/code-review-graph | MIT | stdio | Persistent codebase knowledge graph, risk-scored PR reviews, token-efficient review | D-MCP-003 |
| Context7 MCP | https://github.com/context7/mcp-server | Apache-2.0 | stdio | Version-specific documentation lookup for clean-room research (§2.4) | D-MCP-004 |

## Agent Skills (guide §19)

| Skill | Source URL | Licence | Capability | DECISIONS ref |
|-------|-----------|---------|------------|---------------|
| JuliusBrussee/caveman | https://github.com/JuliusBrussee/caveman | MIT | Terse response style (~65% output token reduction) | D-SKILL-001 |
| elara-labs/code-context-engine | https://github.com/elara-labs/code-context-engine | MIT (core); dual AGPL-3.0/commercial optional | Local MCP server; codebase indexing (~94% input-token savings) | D-SKILL-002 |
| headroomlabs-ai/headroom | https://github.com/headroomlabs-ai/headroom | Apache-2.0 | Context compression layer for tool outputs, logs, files, history | D-SKILL-003 |
| tirth8205/code-review-graph | https://github.com/tirth8205/code-review-graph | MIT | Local-first code-intelligence graph for MCP/CLI; risk-scored PR reviews | D-SKILL-004 |
| rtk-ai/rtk | https://github.com/rtk-ai/rtk | Apache-2.0 | CLI proxy; 60–90% token reduction on dev commands | D-SKILL-005 |
| shubhamsaboo/awesome-llm-apps | https://github.com/shubhamsaboo/awesome-llm-apps | Apache-2.0 | Catalog of installable agent skills for discovery | D-SKILL-006 |
| punkpeye/awesome-mcp-servers | https://github.com/punkpeye/awesome-mcp-servers | Source repos' licences | MCP server catalog for research and tool integration | D-SKILL-007 |

## Free-Tier Services (guide §25.6, ripienaar/free-for-dev)

| Service | Source URL | Licence | Tier | Use | DECISIONS ref |
|---------|-----------|---------|------|-----|---------------|
| GitHub Actions | https://github.com/features/actions | Proprietary (free tier) | 2,000 min/month | CI/CD for builds, tests, releases | — |

## Clean-Room Compliance

All entries are discovery pointers only. No third-party implementation code is committed. Each adoption is vetted against §II.20.4 (licence, maintenance, security) before enablement.

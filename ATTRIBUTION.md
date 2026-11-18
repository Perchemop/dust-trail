# Attribution - Dust Trail

## Licensing
This project is dual-licensed under **MIT OR Apache-2.0**.

## Third-Party Crates
The following third-party crates are used and their licences are recorded:

- `rand` - MIT OR Apache-2.0
- `serde` - MIT OR Apache-2.0
- `crossterm` - MIT
- `rodio` (optional) - MIT
- `egui` (optional, via eframe) - MIT
- `eframe` (optional) - MIT

No external assets, icons, fonts, or proprietary materials are used. All code is original implementation.

## Installed Agent Skills
The following agent skills are adopted from the OpenCode skill catalog (§19 of guide.md) and recorded per guide §19:

| Skill | Version | Licence | Capability |
|-------|---------|---------|------------|
| JuliusBrussee/caveman | 0.1.0 | MIT | Terse response style (~65% output token reduction) |
| elara-labs/code-context-engine | 0.1.0 | MIT (core); one optional component dual AGPL-3.0/commercial | Local MCP server; indexes codebase for agent search (~94% input-token savings) |
| headroomlabs-ai/headroom | 0.1.0 | Apache-2.0 | Context compression layer for tool outputs, logs, files, history |
| tirth8205/code-review-graph | 0.1.0 | MIT | Local-first code-intelligence graph for MCP/CLI; risk-scored PR reviews |
| rtk-ai/rtk | 0.1.0 | Apache-2.0 | CLI proxy; 60–90% token reduction on dev commands |
| shubhamsaboo/awesome-llm-apps | 0.1.0 | Apache-2.0 | Catalog of installable agent skills for discovery |
| punkpeye/awesome-mcp-servers | 0.1.0 | Source repos' licences | MCP server catalog for research and tool integration |

## Clean-Room Compliance
All data models, commands, UI, and interaction design are original. No proprietary source, assets, or reverse-engineered material is included in this project.
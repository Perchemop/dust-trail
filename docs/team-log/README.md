# Team Log

Inter-agent audit trail — one file per multi-agent ticket: `<ticket-id>.md`
(guide §5.7.1, §5.7.4). Every teammate exchange is logged here so it survives
`/new` and is auditable by the human (§5.5 item 1). This directory is the sole
source of truth for what a team did when no UI pane is available (headless CI).

A team-log file records: the team classification (§5.6.2 Case 4), teammates and
scopes, the approved plan (Plan section of `docs/tickets/<id>.md`), each
inter-agent loop (who rejected what and why), graceful shutdown confirmations,
and the CI run ID of the final green run.

No ticket has yet used a multi-agent team (teams are Case 4 only).

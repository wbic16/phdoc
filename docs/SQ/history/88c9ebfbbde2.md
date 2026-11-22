# SQ — reference as of commit 88c9ebfbbde2

Commit 88c9ebfbbde217b1cfebaaa8c8943533a330dfde. 92 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/SQ/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

## Dependencies

### Cargo.toml

- libphext — 0.3.0
- raw_sync — 0.1.5
- shared_memory — 0.12.4
- percent-encoding — 2.3.1

## Authors

The author of record is **Will Bickford** — 92 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 92 commits

## Complexity

- Markdown: 6 files, 294 lines total, 73 blank, 0 comment, 221 code
- Rust: 3 files, 825 lines total, 110 blank, 88 comment, 627 code
- Shell: 3 files, 19 lines total, 0 blank, 3 comment, 16 code
- TOML: 1 file, 14 lines total, 1 blank, 0 comment, 13 code
- TypeScript: 2 files, 930 lines total, 98 blank, 88 comment, 744 code

Declared RFCs at this point: none declared.

## Public methods and commands

- entry point `main` (src/main.rs) — The entry point: the program starts here.
- function `args_required` (src/sq.rs) — Function args_required takes command:&str.
- function `csv_convert` (src/sq.rs) — Function csv_convert takes csv: &str.
- function `process` (src/sq.rs) — Function process takes connection_id: u64, source: String, scroll: &mut String, command: String, phext_map: &mut…

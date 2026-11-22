# SQ — reference as of commit edfde5b2870d

Commit edfde5b2870de2c2563ebfb164ef32a146a3b48b. 12 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/SQ/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

## Dependencies

### Cargo.toml

- libphext — 0.1.9
- raw_sync — 0.1.5
- shared_memory — 0.12.4

## Authors

The author of record is **Will Bickford** — 12 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 12 commits

## Complexity

- Markdown: 1 file, 29 lines total, 8 blank, 0 comment, 21 code
- Rust: 3 files, 235 lines total, 32 blank, 3 comment, 200 code
- Shell: 1 file, 5 lines total, 0 blank, 1 comment, 4 code
- TOML: 1 file, 13 lines total, 1 blank, 0 comment, 12 code

Declared RFCs at this point: none declared.

## Public methods and commands

- entry point `main` (src/main.rs) — The entry point: the program starts here.
- function `process` (src/sq.rs) — Function process takes scroll: &mut String, command: String, phext_buffer: &mut String, coordinate: phext::Coordinate, update: String, filename: String.

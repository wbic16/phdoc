# SQ — reference as of commit edfde5b2870d

Commit edfde5b2870de2c2563ebfb164ef32a146a3b48b. 12 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

### Cargo.toml

- libphext — 0.1.9
- raw_sync — 0.1.5
- shared_memory — 0.12.4

## Authors

- Will Bickford <wbic16@gmail.com> — 12 commits

## Complexity

- Markdown: 1 files, 29 lines total, 3 blank, 0 comment, 26 code
- Rust: 3 files, 235 lines total, 0 blank, 0 comment, 235 code
- Shell: 1 files, 5 lines total, 0 blank, 0 comment, 5 code
- TOML: 1 files, 13 lines total, 0 blank, 0 comment, 13 code

Declared RFCs at this point: none declared.

## Public methods and commands

- entry point `main` (src/main.rs) — The entry point: the program starts here.
- function `process` (src/sq.rs) — Function process takes scroll: &mut String, command: String, phext_buffer: &mut String, coordinate: phext::Coordinate, u

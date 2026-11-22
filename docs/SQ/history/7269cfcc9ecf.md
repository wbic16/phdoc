# SQ — reference as of commit 7269cfcc9ecf

Commit 7269cfcc9ecf9dd6a6707304848535a13c8faff6. 77 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

### Cargo.toml

- libphext — 0.3.0
- raw_sync — 0.1.5
- shared_memory — 0.12.4
- percent-encoding — 2.3.1

## Authors

- Will Bickford <wbic16@gmail.com> — 77 commits

## Complexity

- Markdown: 3 files, 186 lines total, 37 blank, 0 comment, 149 code
- Rust: 3 files, 723 lines total, 92 blank, 67 comment, 564 code
- Shell: 3 files, 19 lines total, 0 blank, 3 comment, 16 code
- TOML: 1 files, 14 lines total, 1 blank, 0 comment, 13 code
- TypeScript: 2 files, 930 lines total, 98 blank, 88 comment, 744 code

Declared RFCs at this point: none declared.

## Public methods and commands

- entry point `main` (src/main.rs) — The entry point: the program starts here.
- function `args_required` (src/sq.rs) — Function args_required takes command:&str and gives back usize.
- function `process` (src/sq.rs) — Function process takes connection_id: u64, source: String, scroll: &mut String, command: String, phext_map: &mut HashMap

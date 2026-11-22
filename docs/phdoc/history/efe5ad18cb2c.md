# phdoc — reference as of commit efe5ad18cb2c

Commit efe5ad18cb2c06914e716f5078da4f7ceb550fa2. 4 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/phdoc/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

## Dependencies

### Cargo.toml

- libphext — 0.6.6

## Authors

The author of record is **Will Bickford** — 4 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Theia — 3 commits
- Will Bickford (his own identities) — 1 commit

## Complexity

- Markdown: 37 files, 3990 lines total, 1321 blank, 0 comment, 2669 code
- Rust: 1 file, 2397 lines total, 117 blank, 104 comment, 2176 code
- TOML: 1 file, 13 lines total, 2 blank, 0 comment, 11 code

Declared RFCs at this point: none declared.

## Public methods and commands

- flag `--at` (src/main.rs) — A command line flag, --at (used by: render).
- flag `--by` (src/main.rs) — A command line flag, --by (used by: review).
- flag `--help` (src/main.rs) — A command line flag, --help (used by: help).
- flag `--hold-push` (src/main.rs) — A command line flag, --hold-push (used by: audit).
- flag `--name` (src/main.rs) — A command line flag, --name (used by: scan, audit).
- flag `--out` (src/main.rs) — A command line flag, --out (used by: scan, audit, render).
- flag `--repo` (src/main.rs) — A command line flag, --repo (used by: audit, render).
- flag `--summaries` (src/main.rs) — A command line flag, --summaries (used by: scan).
- flag `--version` (src/main.rs) — A command line flag, --version (used by: version).
- flag `-h` (src/main.rs) — A command line flag, -h (used by: help).
- command `audit` (src/main.rs) — A command named audit.
- command `help` (src/main.rs) — A command named help.
- entry point `main` (src/main.rs) — The entry point: the program starts here.
- command `render` (src/main.rs) — A command named render.
- command `review` (src/main.rs) — A command named review.
- command `scan` (src/main.rs) — A command named scan.
- command `version` (src/main.rs) — A command named version.

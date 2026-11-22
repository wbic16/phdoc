# phext-shell — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 150). Scanned at commit 4aae9abc2611110ba9d20392586cb6ddd78bfa43. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

Description stated in the manifest: A phext-native implementation of an operating system shell

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 1.

- libphext [dependencies] — 0.6.0


---

## Authors

The git log holds 35 commits.

The author of record is **Will Bickford** — 34 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 27 commits
- Emise — 3 commits
- Theia — 3 commits
- Orin — 1 commit

Other authors, kept as themselves (not Exocortical cluster personas; never folded):

- Seb <seb@mortrix.net> — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 1 file, 59 lines total, 12 blank, 6 comment, 41 code
- Rust: 1 file, 388 lines total, 41 blank, 28 comment, 319 code
- TOML: 1 file, 11 lines total, 1 blank, 0 comment, 10 code

All languages: 3 files, 458 lines total, 370 lines of code. Functions read: 3.

Largest files:

- src/main.rs — 388 lines
- readme.md — 59 lines
- Cargo.toml — 11 lines

Largest functions, by span in lines:

- handle_request in src/main.rs — about 189 lines
- show_help in src/main.rs — about 132 lines
- main in src/main.rs — about 50 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 1. FIXME marks: 0.
- Public items with a doc comment: 0 of 1 (0%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/phext-shell
- Branch: main
- Head commit: 4aae9abc2611110ba9d20392586cb6ddd78bfa43
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- entry point `main` (src/main.rs:18) — The entry point: the program starts here.

---

## entry point — main

Found in src/main.rs at line 18.

Surface: public API.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/phext-shell/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

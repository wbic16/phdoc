# human — reference as of commit caa7c7dc1be6

Commit caa7c7dc1be64e7c38386a05f7bfc87022fc0072. 228 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

No dependencies recorded at this point.

## Authors

Author counts over the 228 audited mainline commits folded to this point.

The author of record is **Will Bickford** — 228 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 228 commits

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 2 files, 384 lines total, 98 blank, 0 comment, 286 code
- Python: 1 files, 38 lines total, 7 blank, 6 comment, 25 code
- Rust: 1 files, 4 lines total, 0 blank, 0 comment, 4 code
- TOML: 1 files, 6 lines total, 0 blank, 0 comment, 6 code

All languages: 5 files, 432 lines total, 321 lines of code. Functions read: 1.

Largest files:

- README.md — 240 lines
- consciousness-api.md — 144 lines
- visualize-mp3-as-image.py — 38 lines

Largest functions, by span in lines:


## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files in test directories or whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 0 of 1 (0%).

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/human.git
- Branch: exo
- Head commit: caa7c7dc1be64e7c38386a05f7bfc87022fc0072
- Working tree: the committed tree at this commit — a historical reference has no working tree

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- entry point `main` (src/main.rs:1) — The entry point: the program starts here.

## entry point — main

Found in src/main.rs at line 1.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: signature.)

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/human/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

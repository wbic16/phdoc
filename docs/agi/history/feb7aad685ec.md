# agi — reference as of commit feb7aad685ec

Commit feb7aad685ec46d7afef1c952b005f28db433dc3. 3 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

No dependencies recorded at this point.

## Authors

Author counts over the 3 audited mainline commits folded to this point.

The author of record is **Will Bickford** — 3 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Lux — 2 commits
- Phex — 1 commit

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 5 files, 159 lines total, 9 blank, 0 comment, 150 code
- Shell: 1 files, 96 lines total, 0 blank, 0 comment, 96 code

All languages: 6 files, 255 lines total, 246 lines of code. Functions read: 3.

Largest files:

- AGENTS.md — 27 lines
- iteration-001/1-phex.md — 25 lines

Largest functions, by span in lines:


## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files in test directories or whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 0 of 3 (0%).

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/agi.git
- Branch: exo
- Head commit: feb7aad685ec46d7afef1c952b005f28db433dc3
- Working tree: the committed tree at this commit — a historical reference has no working tree

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `latest_iter` (choir/lux-protocol.sh:28) — Function latest_iter takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `wait_for_predecessor` (choir/lux-protocol.sh:34) — Function wait_for_predecessor takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `write_entry` (choir/lux-protocol.sh:55) — Function write_entry takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 

## function — latest_iter

Found in choir/lux-protocol.sh at line 28.

Signature: `latest_iter()`

What it does: Function latest_iter takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — wait_for_predecessor

Found in choir/lux-protocol.sh at line 34.

Signature: `wait_for_predecessor()`

What it does: Function wait_for_predecessor takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — write_entry

Found in choir/lux-protocol.sh at line 55.

Signature: `write_entry()`

What it does: Function write_entry takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/agi/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

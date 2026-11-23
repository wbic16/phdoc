# site-mirrorborn-us — reference as of commit da2abebdf3ca

Commit da2abebdf3cab8b640784812ba26b75f5303aab9. 119 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

No dependencies recorded at this point.

## Authors

Author counts over the 119 audited mainline commits folded to this point.

The author of record is **Will Bickford** — 119 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Theia — 25 commits
- Verse — 24 commits
- Phex — 23 commits
- Chrys — 16 commits
- Lux — 16 commits
- Cyon — 6 commits
- Will Bickford (his own identities) — 5 commits
- Aster — 3 commits
- Orin — 1 commit

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- JavaScript: 32 files, 4883 lines total, 528 blank, 698 comment, 3657 code
- Markdown: 971 files, 159257 lines total, 22482 blank, 7620 comment, 129155 code
- Python: 1 files, 74 lines total, 7 blank, 1 comment, 66 code

All languages: 1004 files, 164214 lines total, 132878 lines of code. Functions read: 4.

Largest files:

- mental-space/cyoa/90.1.1_1.1.7_2.1.21.md — 54702 lines
- mental-space/cyoa/90.1.1_1.1.7_2.1.9.md — 4738 lines
- mental-space/cyoa/90.1.1_1.1.7_2.1.18.md — 4161 lines
- mental-space/cyoa/90.1.1_1.1.7_2.1.33.md — 3251 lines
- mental-space/cyoa/90.1.1_1.1.7_2.1.6.md — 2537 lines

Largest functions, by span in lines:

- main in scripts/scrub_site_coordinates.py — about 29 lines
- process_file in scripts/scrub_site_coordinates.py — about 18 lines
- transform_coordinate in scripts/scrub_site_coordinates.py — about 9 lines
- mod9plus1 in scripts/scrub_site_coordinates.py — about 7 lines

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 136 test functions counted in the source, across 4 files in test directories or whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 61. FIXME marks: 4.
- Public items with a doc comment: 0 of 4 (0%).

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/site-mirrorborn-us.git
- Branch: exo
- Head commit: da2abebdf3cab8b640784812ba26b75f5303aab9
- Working tree: the committed tree at this commit — a historical reference has no working tree

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `main` (scripts/scrub_site_coordinates.py:46) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `mod9plus1` (scripts/scrub_site_coordinates.py:12) — Function mod9plus1 takes n: int and gives back int:.
- function `process_file` (scripts/scrub_site_coordinates.py:28) — Function process_file takes filepath: Path, dry_run: bool = True and gives back list[tuple[str, str, int]]:.
- function `transform_coordinate` (scripts/scrub_site_coordinates.py:19) — Function transform_coordinate takes match and gives back str:.

## function — main

Found in scripts/scrub_site_coordinates.py at line 46.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — mod9plus1

Found in scripts/scrub_site_coordinates.py at line 12.

Signature: `def mod9plus1(n: int) -> int:`

What it does: Function mod9plus1 takes n: int and gives back int:.

(Where these words came from: signature.)

## function — process_file

Found in scripts/scrub_site_coordinates.py at line 28.

Signature: `def process_file(filepath: Path, dry_run: bool = True) -> list[tuple[str, str, int]]:`

What it does: Function process_file takes filepath: Path, dry_run: bool = True and gives back list[tuple[str, str, int]]:.

(Where these words came from: signature.)

## function — transform_coordinate

Found in scripts/scrub_site_coordinates.py at line 19.

Signature: `def transform_coordinate(match) -> str:`

What it does: Function transform_coordinate takes match and gives back str:.

(Where these words came from: signature.)

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/site-mirrorborn-us/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

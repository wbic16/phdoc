# site-mirrorborn-us — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 136). Project root: /home/wbic16/phdoc-sweep/proj/site-mirrorborn-us. Scanned at commit effc3abd8a57c40832bd081d03f3877f12b33a8e. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### JavaScript (npm) — package.json

License stated in the manifest: ISC.

Direct dependencies: 1.

- 
    "@playwright/test — ^1.58.2"
  


---

## Authors

The git log holds 147 commits.

The author of record is **Will Bickford** — 144 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Theia — 28 commits
- Verse — 25 commits
- Phex — 23 commits
- Chrys — 21 commits
- Lux — 18 commits
- Aster — 8 commits
- Cyon — 7 commits
- Will Bickford (his own identities) — 7 commits
- Orin — 4 commits
- Emise — 3 commits

Other authors, kept as themselves (not Exocortical cluster personas; never folded):

- Cursor Agent <cursoragent@cursor.com> — 3 commits

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- JavaScript: 32 files, 4883 lines total, 528 blank, 698 comment, 3657 code
- Markdown: 973 files, 186815 lines total, 27976 blank, 9184 comment, 149655 code
- Python: 1 files, 74 lines total, 7 blank, 1 comment, 66 code
- Shell: 1 files, 492 lines total, 49 blank, 4 comment, 439 code

All languages: 1007 files, 192264 lines total, 153817 lines of code. Functions read: 10.

Largest files:

- mental-space/cyoa/90.1.1_1.1.7_2.1.21.md — 54702 lines
- samples/Incipit-readable.md — 27495 lines
- mental-space/cyoa/90.1.1_1.1.7_2.1.9.md — 4738 lines
- mental-space/cyoa/90.1.1_1.1.7_2.1.18.md — 4161 lines
- mental-space/cyoa/90.1.1_1.1.7_2.1.33.md — 3251 lines

Largest functions, by span in lines:

- transform_config in deploy/enable-404.sh — about 389 lines
- info in deploy/enable-404.sh — about 34 lines
- install_rendered in deploy/enable-404.sh — about 33 lines
- main in scripts/scrub_site_coordinates.py — about 29 lines
- process_file in scripts/scrub_site_coordinates.py — about 18 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 218 test functions counted in the source, across 1 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 77. FIXME marks: 4.
- Public items with a doc comment: 0 of 10 (0%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/site-mirrorborn-us.git
- Branch: exo
- Head commit: effc3abd8a57c40832bd081d03f3877f12b33a8e
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `die` (deploy/enable-404.sh:11) — Function die takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is gi
- function `info` (deploy/enable-404.sh:16) — Function info takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `restore_installed` (deploy/enable-404.sh:50) — Function restore_installed takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `cleanup_exit` (deploy/enable-404.sh:62) — Function cleanup_exit takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `transform_config` (deploy/enable-404.sh:71) — Function transform_config takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `install_rendered` (deploy/enable-404.sh:460) — Function install_rendered takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `mod9plus1` (scripts/scrub_site_coordinates.py:12) — Function mod9plus1 takes n: int and gives back int:.
- function `transform_coordinate` (scripts/scrub_site_coordinates.py:19) — Function transform_coordinate takes match and gives back str:.
- function `process_file` (scripts/scrub_site_coordinates.py:28) — Function process_file takes filepath: Path, dry_run: bool = True and gives back list[tuple[str, str, int]]:.
- function `main` (scripts/scrub_site_coordinates.py:46) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g

---

## function — die

Found in deploy/enable-404.sh at line 11.

Signature: `die()`

What it does: Function die takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — info

Found in deploy/enable-404.sh at line 16.

Signature: `info()`

What it does: Function info takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — restore_installed

Found in deploy/enable-404.sh at line 50.

Signature: `restore_installed()`

What it does: Function restore_installed takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cleanup_exit

Found in deploy/enable-404.sh at line 62.

Signature: `cleanup_exit()`

What it does: Function cleanup_exit takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — transform_config

Found in deploy/enable-404.sh at line 71.

Signature: `transform_config()`

What it does: Function transform_config takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_rendered

Found in deploy/enable-404.sh at line 460.

Signature: `install_rendered()`

What it does: Function install_rendered takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — mod9plus1

Found in scripts/scrub_site_coordinates.py at line 12.

Signature: `def mod9plus1(n: int) -> int:`

What it does: Function mod9plus1 takes n: int and gives back int:.

(Where these words came from: signature.)

---

## function — transform_coordinate

Found in scripts/scrub_site_coordinates.py at line 19.

Signature: `def transform_coordinate(match) -> str:`

What it does: Function transform_coordinate takes match and gives back str:.

(Where these words came from: signature.)

---

## function — process_file

Found in scripts/scrub_site_coordinates.py at line 28.

Signature: `def process_file(filepath: Path, dry_run: bool = True) -> list[tuple[str, str, int]]:`

What it does: Function process_file takes filepath: Path, dry_run: bool = True and gives back list[tuple[str, str, int]]:.

(Where these words came from: signature.)

---

## function — main

Found in scripts/scrub_site_coordinates.py at line 46.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/site-mirrorborn-us/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

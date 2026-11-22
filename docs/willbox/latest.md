# willbox — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 95). Project root: /home/wbic16/phdoc-sweep/proj/willbox/. Scanned at commit 479af2260ffd41d52e0bfcdffdae76b1b8609e78. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 1 commits.

The author of record is **Will Bickford** — 1 commit folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Velum — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 2 files, 195 lines total, 38 blank, 0 comment, 157 code
- Python: 3 files, 303 lines total, 40 blank, 5 comment, 258 code
- Shell: 1 files, 13 lines total, 2 blank, 6 comment, 5 code

All languages: 6 files, 511 lines total, 420 lines of code. Functions read: 10.

Largest files:

- build_willbox.py — 163 lines
- REPORT.md — 126 lines
- rebuild_from_willbox.py — 75 lines
- README.md — 69 lines
- phextlib.py — 65 lines

Largest functions, by span in lines:

- main in build_willbox.py — about 94 lines
- main in rebuild_from_willbox.py — about 50 lines
- parse_bytes in phextlib.py — about 20 lines
- implode in phextlib.py — about 17 lines
- sha256_of in build_willbox.py — about 11 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 2 of 10 (20%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/willbox.git
- Branch: exo
- Head commit: 479af2260ffd41d52e0bfcdffdae76b1b8609e78
- Working tree: 1 changed paths

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `sha256_of` (build_willbox.py:39) — Function sha256_of takes path and gives back nothing; it does its work on what it is given.
- function `count_byte` (build_willbox.py:50) — Function count_byte takes path, byte and gives back nothing; it does its work on what it is given.
- function `stream_copy` (build_willbox.py:61) — Function stream_copy takes src, dst and gives back nothing; it does its work on what it is given.
- function `main` (build_willbox.py:70) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `fmt` (phextlib.py:20) — Function fmt takes coord and gives back nothing; it does its work on what it is given.
- function `parse_bytes` (phextlib.py:24) — Yield (coord_tuple, content_bytes) for each scroll slot, in order.
- function `parse_file` (phextlib.py:44) — Function parse_file takes path, chunk_hint=None and gives back nothing; it does its work on what it is given.
- function `implode` (phextlib.py:49) — scrolls: ordered list of (coord_tuple, content_bytes) with
- function `sha256_of` (rebuild_from_willbox.py:15) — Function sha256_of takes path and gives back nothing; it does its work on what it is given.
- function `main` (rebuild_from_willbox.py:26) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g

---

## function — sha256_of

Found in build_willbox.py at line 39.

Signature: `def sha256_of(path):`

What it does: Function sha256_of takes path and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — count_byte

Found in build_willbox.py at line 50.

Signature: `def count_byte(path, byte):`

What it does: Function count_byte takes path, byte and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — stream_copy

Found in build_willbox.py at line 61.

Signature: `def stream_copy(src, dst):`

What it does: Function stream_copy takes src, dst and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in build_willbox.py at line 70.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — fmt

Found in phextlib.py at line 20.

Signature: `def fmt(coord):`

What it does: Function fmt takes coord and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — parse_bytes

Found in phextlib.py at line 24.

Signature: `def parse_bytes(data):`

What it does: Yield (coord_tuple, content_bytes) for each scroll slot, in order.

(Where these words came from: doc comment.)

---

## function — parse_file

Found in phextlib.py at line 44.

Signature: `def parse_file(path, chunk_hint=None):`

What it does: Function parse_file takes path, chunk_hint=None and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — implode

Found in phextlib.py at line 49.

Signature: `def implode(scrolls):`

What it does: scrolls: ordered list of (coord_tuple, content_bytes) with

(Where these words came from: doc comment.)

---

## function — sha256_of

Found in rebuild_from_willbox.py at line 15.

Signature: `def sha256_of(path):`

What it does: Function sha256_of takes path and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in rebuild_from_willbox.py at line 26.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/willbox/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

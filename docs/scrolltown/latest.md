# scrolltown — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 95). Project root: /home/wbic16/phdoc-sweep/proj/scrolltown. Scanned at commit 2d1ca6ad931f8d41f45cd34be1f37b7013afa7d9. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 21 commits. The authors, by commit count:

- Emise <emise@localhost> — 14 commits (66.7%)
- Emise <emise@phext.io> — 4 commits (19.0%)
- Will Bickford <wbic16@gmail.com> — 3 commits (14.3%)

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 32 files, 2842 lines total, 475 blank, 20 comment, 2347 code
- Python: 1 files, 395 lines total, 39 blank, 41 comment, 315 code
- Shell: 2 files, 88 lines total, 5 blank, 13 comment, 70 code

All languages: 35 files, 3325 lines total, 2732 lines of code. Functions read: 10.

Largest files:

- series/animation-exercises/gate-01-steamboat-willie/gate_validator.py — 395 lines
- EPISODE-INDEX.md — 321 lines
- STORY-ARC.md — 310 lines
- PRODUCTION.md — 277 lines
- series/animation-exercises/gate-01-steamboat-willie/VALIDATOR-REPORT.md — 214 lines

Largest functions, by span in lines:

- validate_page in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py — about 133 lines
- main in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py — about 61 lines
- union in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py — about 48 lines
- clipped_distance in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py — about 20 lines
- strong_edges in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py — about 15 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 4 of 10 (40%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/scrolltown.git
- Branch: exo
- Head commit: 2d1ca6ad931f8d41f45cd34be1f37b7013afa7d9
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `load_gray` (series/animation-exercises/gate-01-steamboat-willie/gate_validator.py:83) — Function load_gray takes path, size=None and gives back nothing; it does its work on what it is given.
- function `dilate` (series/animation-exercises/gate-01-steamboat-willie/gate_validator.py:90) — Chebyshev dilation by `steps` px (3x3 structuring element).
- function `clipped_distance` (series/animation-exercises/gate-01-steamboat-willie/gate_validator.py:101) — For each pixel in target_mask, Chebyshev distance to source_mask,
- function `strong_edges` (series/animation-exercises/gate-01-steamboat-willie/gate_validator.py:121) — Strong-edge map of an original film frame: gradient magnitude on a
- function `label_components` (series/animation-exercises/gate-01-steamboat-willie/gate_validator.py:136) — Two-pass union-find labeling over row runs.
- function `find` (series/animation-exercises/gate-01-steamboat-willie/gate_validator.py:141) — Function find takes a and gives back nothing; it does its work on what it is given.
- function `union` (series/animation-exercises/gate-01-steamboat-willie/gate_validator.py:147) — Function union takes a, b and gives back nothing; it does its work on what it is given.
- function `downscale_mask` (series/animation-exercises/gate-01-steamboat-willie/gate_validator.py:195) — Function downscale_mask takes mask, factor=2 and gives back nothing; it does its work on what it is given.
- function `validate_page` (series/animation-exercises/gate-01-steamboat-willie/gate_validator.py:202) — Function validate_page takes page_path, frame_path and gives back nothing; it does its work on what it is given.
- function `main` (series/animation-exercises/gate-01-steamboat-willie/gate_validator.py:335) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g

---

## function — load_gray

Found in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py at line 83.

Signature: `def load_gray(path, size=None):`

What it does: Function load_gray takes path, size=None and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — dilate

Found in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py at line 90.

Signature: `def dilate(mask, steps):`

What it does: Chebyshev dilation by `steps` px (3x3 structuring element).

(Where these words came from: doc comment.)

---

## function — clipped_distance

Found in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py at line 101.

Signature: `def clipped_distance(target_mask, source_mask, clip):`

What it does: For each pixel in target_mask, Chebyshev distance to source_mask,

(Where these words came from: doc comment.)

---

## function — strong_edges

Found in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py at line 121.

Signature: `def strong_edges(gray):`

What it does: Strong-edge map of an original film frame: gradient magnitude on a

(Where these words came from: doc comment.)

---

## function — label_components

Found in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py at line 136.

Signature: `def label_components(mask):`

What it does: Two-pass union-find labeling over row runs. Returns (count, sizes).

(Where these words came from: doc comment.)

---

## function — find

Found in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py at line 141.

Signature: `def find(a):`

What it does: Function find takes a and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — union

Found in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py at line 147.

Signature: `def union(a, b):`

What it does: Function union takes a, b and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — downscale_mask

Found in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py at line 195.

Signature: `def downscale_mask(mask, factor=2):`

What it does: Function downscale_mask takes mask, factor=2 and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — validate_page

Found in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py at line 202.

Signature: `def validate_page(page_path, frame_path):`

What it does: Function validate_page takes page_path, frame_path and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in series/animation-exercises/gate-01-steamboat-willie/gate_validator.py at line 335.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/scrolltown/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

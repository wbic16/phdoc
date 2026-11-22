# willbox — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 150). Scanned at commit 479af2260ffd41d52e0bfcdffdae76b1b8609e78. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Summary

One phext holding three boxes as phext priors — reading, writing, and arithmetic for sentients:

(The opening of the project's README, quoted as written.)

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 1 commit.

The author of record is **Will Bickford** — 1 commit folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Velum — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 2 files, 195 lines total, 38 blank, 0 comment, 157 code
- Python: 3 files, 303 lines total, 40 blank, 5 comment, 258 code
- Shell: 1 file, 13 lines total, 2 blank, 6 comment, 5 code

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

- Origin: https://github.com/wbic16/willbox
- Branch: exo
- Head commit: 479af2260ffd41d52e0bfcdffdae76b1b8609e78
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `sha256_of` (build_willbox.py:39) — Function sha256_of takes path.
- function `count_byte` (build_willbox.py:50) — Function count_byte takes path, byte.
- function `stream_copy` (build_willbox.py:61) — Function stream_copy takes src, dst.
- function `main` (build_willbox.py:70) — Function main takes no inputs beyond the value it is called on.
- function `fmt` (phextlib.py:20) — Function fmt takes coord.
- function `parse_bytes` (phextlib.py:24) — Yield (coord_tuple, content_bytes) for each scroll slot, in order.
- function `parse_file` (phextlib.py:44) — Function parse_file takes path, chunk_hint=None.
- function `implode` (phextlib.py:49) — scrolls: ordered list of (coord_tuple, content_bytes) with
- function `sha256_of` (rebuild_from_willbox.py:15) — Function sha256_of takes path.
- function `main` (rebuild_from_willbox.py:26) — Function main takes no inputs beyond the value it is called on.

---

## function — sha256_of

Found in build_willbox.py at line 39.

Surface: public API.

Signature: `def sha256_of(path)`

What it does: Function sha256_of takes path. Its return type is unannotated; the code shows it returning h.hexdigest().

(Where these words came from: inferred.)

---

## function — count_byte

Found in build_willbox.py at line 50.

Surface: public API.

Signature: `def count_byte(path, byte)`

What it does: Function count_byte takes path, byte. Its return type is unannotated; the code shows it returning n.

(Where these words came from: inferred.)

---

## function — stream_copy

Found in build_willbox.py at line 61.

Surface: public API.

Signature: `def stream_copy(src, dst)`

What it does: Function stream_copy takes src, dst. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in build_willbox.py at line 70.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — fmt

Found in phextlib.py at line 20.

Surface: public API.

Signature: `def fmt(coord)`

What it does: Function fmt takes coord. Its return type is unannotated; the code shows it returning "%d.%d.%d/%d.%d.%d/%d.%d.%d" % tuple(coord).

(Where these words came from: inferred.)

---

## function — parse_bytes

Found in phextlib.py at line 24.

Surface: public API.

Signature: `def parse_bytes(data)`

What it does: Yield (coord_tuple, content_bytes) for each scroll slot, in order.

(Where these words came from: direct.)

---

## function — parse_file

Found in phextlib.py at line 44.

Surface: public API.

Signature: `def parse_file(path, chunk_hint=None)`

What it does: Function parse_file takes path, chunk_hint=None. Its return type is unannotated; the code shows it returning parse_bytes(f.read()).

(Where these words came from: inferred.)

---

## function — implode

Found in phextlib.py at line 49.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: ordered list of (coord_tuple, content_bytes) with

(Where these words came from: direct.)

---

## function — sha256_of

Found in rebuild_from_willbox.py at line 15.

Surface: public API.

Signature: `def sha256_of(path)`

What it does: Function sha256_of takes path. Its return type is unannotated; the code shows it returning h.hexdigest().

(Where these words came from: inferred.)

---

## function — main

Found in rebuild_from_willbox.py at line 26.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0 if ok else 1.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/willbox/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

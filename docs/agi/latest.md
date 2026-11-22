# agi — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 147). Scanned at commit 34bd7f588c0136cba0c1653be67c76c0eea89ebf. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Summary

The collected phexts of Will Bickford's Exocortex, and their nightly unified merge.

(The opening of the project's README, quoted as written.)

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 66 commits.

The author of record is **Will Bickford** — 66 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Emise — 54 commits
- Will Bickford (his own identities) — 4 commits
- Lux — 3 commits
- Phex — 2 commits
- Verse — 2 commits
- Chrys — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 4 files, 140 lines total, 20 blank, 0 comment, 120 code
- Python: 1 file, 200 lines total, 22 blank, 6 comment, 172 code

All languages: 5 files, 340 lines total, 292 lines of code. Functions read: 6.

Largest files:

- build_agi_phext.py — 200 lines
- MERGE-RULES.md — 62 lines
- README.md — 40 lines
- SOURCES.md — 24 lines
- AGENTS.md — 14 lines

Largest functions, by span in lines:

- annex_coord in build_agi_phext.py — about 83 lines
- explode in build_agi_phext.py — about 23 lines
- implode in build_agi_phext.py — about 17 lines
- main in build_agi_phext.py — about 17 lines
- build in build_agi_phext.py — about 10 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 1 of 6 (17%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/agi
- Branch: exo
- Head commit: 34bd7f588c0136cba0c1653be67c76c0eea89ebf
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `fmt` (build_agi_phext.py:47) — Function fmt takes coord.
- function `explode` (build_agi_phext.py:51) — bytes -> {coord_tuple: text} for non-empty scrolls.
- function `implode` (build_agi_phext.py:74) — Function implode takes scrolls.
- function `build` (build_agi_phext.py:91) — Function build takes no inputs beyond the value it is called on.
- function `annex_coord` (build_agi_phext.py:101) — Function annex_coord takes src_idx.
- function `main` (build_agi_phext.py:184) — Function main takes no inputs beyond the value it is called on.

---

## function — fmt

Found in build_agi_phext.py at line 47.

Surface: public API.

Signature: `def fmt(coord)`

What it does: Function fmt takes coord. Its return type is unannotated; the code shows it returning "/".join(".".join(str(x) for x in coord[i:i + 3].

(Where these words came from: inferred.)

---

## function — explode

Found in build_agi_phext.py at line 51.

Surface: public API.

Signature: `def explode(data: bytes)`

What it does: bytes -> {coord_tuple: text} for non-empty scrolls.

(Where these words came from: direct.)

---

## function — implode

Found in build_agi_phext.py at line 74.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: Function implode takes scrolls. Its return type is unannotated; the code shows it returning bytes(out).

(Where these words came from: inferred.)

---

## function — build

Found in build_agi_phext.py at line 91.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — annex_coord

Found in build_agi_phext.py at line 101.

Surface: public API.

Signature: `def annex_coord(src_idx)`

What it does: Function annex_coord takes src_idx. Its return type is unannotated; the code shows it returning cand.

(Where these words came from: inferred.)

---

## function — main

Found in build_agi_phext.py at line 184.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/agi/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

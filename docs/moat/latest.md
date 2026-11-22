# moat — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 91). Project root: /home/wbic16/phdoc-sweep/proj/moat. Scanned at commit fb84a7c1a3e52aa9f30219d3e3d1e4fd0b06e077. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 2 commits. The authors, by commit count:

- Emise <emise@localhost> — 1 commit (50.0%)
- Will Bickford <wbic16@gmail.com> — 1 commit (50.0%)

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- JavaScript: 1 files, 283 lines total, 17 blank, 8 comment, 258 code
- Markdown: 22 files, 2609 lines total, 491 blank, 16 comment, 2102 code
- Python: 21 files, 42913 lines total, 2339 blank, 153 comment, 40421 code
- Shell: 3 files, 80 lines total, 6 blank, 23 comment, 51 code

All languages: 47 files, 45885 lines total, 42832 lines of code. Functions read: 56.

Largest files:

- tracker/build_research_phext.py — 35619 lines
- epistemic/extract_registry.py — 1297 lines
- evaluation/ingestion/p013_texts.py — 1162 lines
- evaluation/ingestion/p021_chapter.py — 538 lines
- evaluation/ingestion/p018_chapter.py — 484 lines

Largest functions, by span in lines:

- rep in evaluation/ingestion/integrate_p021.py — about 400 lines
- ins_after in evaluation/ingestion/integrate_p013.py — about 382 lines
- rep in evaluation/ingestion/integrate_p018.py — about 376 lines
- heading_of in epistemic/extract_registry.py — about 119 lines
- rep in evaluation/ingestion/integrate_p018_phaseb.py — about 103 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 3 of 42 (7%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/moat.git
- Branch: exo
- Head commit: fb84a7c1a3e52aa9f30219d3e3d1e4fd0b06e077
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `concept_source` (epistemic/extract_registry.py:875) — Function concept_source takes scroll_no and gives back nothing; it does its work on what it is given.
- function `heading_of` (epistemic/extract_registry.py:953) — Function heading_of takes text and gives back nothing; it does its work on what it is given.
- function `claim_of` (epistemic/extract_registry.py:1072) — Function claim_of takes coord, text and gives back nothing; it does its work on what it is given.
- function `classify` (epistemic/extract_registry.py:1086) — Return (status, basis, evidence).
- function `override_evidence` (epistemic/extract_registry.py:1166) — Function override_evidence takes coord and gives back nothing; it does its work on what it is given.
- function `main` (epistemic/extract_registry.py:1236) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `rng` (evaluation/evaluation_analysis.py:23) — Function rng takes v and gives back nothing; it does its work on what it is given.
- function `links_total` (evaluation/evaluation_analysis.py:29) — Function links_total takes r, side, other=None and gives back nothing; it does its work on what it is given.
- function `op_pred_of` (evaluation/evaluation_analysis.py:42) — Function op_pred_of takes pr, sc and gives back nothing; it does its work on what it is given.
- function `op_range_pred_of` (evaluation/evaluation_analysis.py:47) — Function op_range_pred_of takes pr, sc and gives back nothing; it does its work on what it is given.
- function `links_split` (evaluation/evaluation_analysis.py:53) — Function links_split takes s and gives back nothing; it does its work on what it is given.
- function `band` (evaluation/evaluation_analysis.py:117) — Function band takes ni, axis and gives back nothing; it does its work on what it is given.
- function `step` (evaluation/evaluation_analysis.py:122) — Function step takes ni, axis, axis_step=-5 and gives back nothing; it does its work on what it is given.
- function `survives` (evaluation/evaluation_analysis.py:131) — Function survives takes k and gives back nothing; it does its work on what it is given.
- function `null_oracle` (evaluation/evaluation_analysis.py:142) — Function null_oracle takes op_before, ni, axis and gives back nothing; it does its work on what it is given.
- function `agg` (evaluation/evaluation_analysis.py:193) — Function agg takes rs, key and gives back nothing; it does its work on what it is given.
- function `summary` (evaluation/evaluation_analysis.py:196) — Function summary takes rs, label and gives back nothing; it does its work on what it is given.
- function `apply` (evaluation/ingestion/fix_p018_words.py:154) — Function apply takes path, pairs and gives back nothing; it does its work on what it is given.
- function `rep` (evaluation/ingestion/integrate_p013.py:9) — Function rep takes old, new, n=1 and gives back nothing; it does its work on what it is given.
- function `ins_after` (evaluation/ingestion/integrate_p013.py:15) — Function ins_after takes old, add, n=1 and gives back nothing; it does its work on what it is given.
- function `rep` (evaluation/ingestion/integrate_p018.py:17) — Function rep takes old, new, n=1 and gives back nothing; it does its work on what it is given.
- function `rep` (evaluation/ingestion/integrate_p018_phaseb.py:11) — Function rep takes old, new, n=1 and gives back nothing; it does its work on what it is given.
- function `rep` (evaluation/ingestion/integrate_p021.py:18) — Function rep takes old, new, n=1 and gives back nothing; it does its work on what it is given.
- function `cb_of` (evaluation/score_p020.py:53) — Function cb_of takes pr, sc and gives back nothing; it does its work on what it is given.
- function `ni` (evaluation/score_p020.py:61) — Function ni takes new, revised, cb and gives back nothing; it does its work on what it is given.
- function `score` (evaluation/score_p020.py:81) — Function score takes filing and gives back nothing; it does its work on what it is given.
- function `band_step` (evaluation/score_p020.py:162) — Function band_step takes ni_v, axis and gives back nothing; it does its work on what it is given.
- function `ni` (evaluation/score_p021.py:62) — Function ni takes new, revised, cb and gives back nothing; it does its work on what it is given.
- function `score` (evaluation/score_p021.py:80) — Function score takes filing and gives back nothing; it does its work on what it is given.
- function `cb_of` (evaluation/score_p021.py:107) — Function cb_of takes pr, sc and gives back nothing; it does its work on what it is given.
- function `band_step` (evaluation/score_p021.py:163) — Function band_step takes ni_v, axis and gives back nothing; it does its work on what it is given.
- function `entry_lines` (evaluation/splice_registry.py:10) — Function entry_lines takes path and gives back nothing; it does its work on what it is given.
- function `server_blocks` (service/nginx-insert.py:41) — Yield (start, end) spans of each top-level `server { ...
- function `main` (service/nginx-insert.py:63) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `rep` (service/tluf-submit.py:85) — Function rep takes m and gives back nothing; it does its work on what it is given.
- function `canonicalize` (service/tluf-submit.py:249) — Return (address, error): exactly one of the two is None.
- function `store_submission` (service/tluf-submit.py:313) — Function store_submission takes fields, address and gives back nothing; it does its work on what it is given.
- function `do_GET` (service/tluf-submit.py:347) — Function do_GET takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is
- function `do_POST` (service/tluf-submit.py:353) — Function do_POST takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it i
- function `log_message` (service/tluf-submit.py:410) — Function log_message takes self, fmt, *args and gives back nothing; it does its work on what it is given.
- function `main` (service/tluf-submit.py:414) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `main` (tracker/build_research_phext.py:35596) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g

---

## function — concept_source

Found in epistemic/extract_registry.py at line 875.

Signature: `def concept_source(scroll_no):`

What it does: Function concept_source takes scroll_no and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — heading_of

Found in epistemic/extract_registry.py at line 953.

Signature: `def heading_of(text):`

What it does: Function heading_of takes text and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — claim_of

Found in epistemic/extract_registry.py at line 1072.

Signature: `def claim_of(coord, text):`

What it does: Function claim_of takes coord, text and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — classify

Found in epistemic/extract_registry.py at line 1086.

Signature: `def classify(coord, text):`

What it does: Return (status, basis, evidence). Kind defaults; content decides.

(Where these words came from: doc comment.)

---

## function — override_evidence

Found in epistemic/extract_registry.py at line 1166.

Signature: `def override_evidence(coord):`

What it does: Function override_evidence takes coord and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in epistemic/extract_registry.py at line 1236.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — rng

Found in evaluation/evaluation_analysis.py at line 23.

Signature: `def rng(v):`

What it does: Function rng takes v and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — links_total

Found in evaluation/evaluation_analysis.py at line 29.

Signature: `def links_total(r, side, other=None):`

What it does: Function links_total takes r, side, other=None and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — op_pred_of

Found in evaluation/evaluation_analysis.py at line 42.

Signature: `def op_pred_of(pr, sc):`

What it does: Function op_pred_of takes pr, sc and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — op_range_pred_of

Found in evaluation/evaluation_analysis.py at line 47.

Signature: `def op_range_pred_of(pr, sc):`

What it does: Function op_range_pred_of takes pr, sc and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — links_split

Found in evaluation/evaluation_analysis.py at line 53.

Signature: `def links_split(s):`

What it does: Function links_split takes s and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — band

Found in evaluation/evaluation_analysis.py at line 117.

Signature: `def band(ni, axis):`

What it does: Function band takes ni, axis and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — step

Found in evaluation/evaluation_analysis.py at line 122.

Signature: `def step(ni, axis, axis_step=-5):`

What it does: Function step takes ni, axis, axis_step=-5 and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — survives

Found in evaluation/evaluation_analysis.py at line 131.

Signature: `def survives(k):`

What it does: Function survives takes k and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — null_oracle

Found in evaluation/evaluation_analysis.py at line 142.

Signature: `def null_oracle(op_before, ni, axis):`

What it does: Function null_oracle takes op_before, ni, axis and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — agg

Found in evaluation/evaluation_analysis.py at line 193.

Signature: `def agg(rs, key):`

What it does: Function agg takes rs, key and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — summary

Found in evaluation/evaluation_analysis.py at line 196.

Signature: `def summary(rs, label):`

What it does: Function summary takes rs, label and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — apply

Found in evaluation/ingestion/fix_p018_words.py at line 154.

Signature: `def apply(path, pairs):`

What it does: Function apply takes path, pairs and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — rep

Found in evaluation/ingestion/integrate_p013.py at line 9.

Signature: `def rep(old, new, n=1):`

What it does: Function rep takes old, new, n=1 and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — ins_after

Found in evaluation/ingestion/integrate_p013.py at line 15.

Signature: `def ins_after(old, add, n=1):`

What it does: Function ins_after takes old, add, n=1 and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — rep

Found in evaluation/ingestion/integrate_p018.py at line 17.

Signature: `def rep(old, new, n=1):`

What it does: Function rep takes old, new, n=1 and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — rep

Found in evaluation/ingestion/integrate_p018_phaseb.py at line 11.

Signature: `def rep(old, new, n=1):`

What it does: Function rep takes old, new, n=1 and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — rep

Found in evaluation/ingestion/integrate_p021.py at line 18.

Signature: `def rep(old, new, n=1):`

What it does: Function rep takes old, new, n=1 and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cb_of

Found in evaluation/score_p020.py at line 53.

Signature: `def cb_of(pr, sc):`

What it does: Function cb_of takes pr, sc and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — ni

Found in evaluation/score_p020.py at line 61.

Signature: `def ni(new, revised, cb):`

What it does: Function ni takes new, revised, cb and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — score

Found in evaluation/score_p020.py at line 81.

Signature: `def score(filing):`

What it does: Function score takes filing and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — band_step

Found in evaluation/score_p020.py at line 162.

Signature: `def band_step(ni_v, axis):`

What it does: Function band_step takes ni_v, axis and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — ni

Found in evaluation/score_p021.py at line 62.

Signature: `def ni(new, revised, cb):`

What it does: Function ni takes new, revised, cb and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — score

Found in evaluation/score_p021.py at line 80.

Signature: `def score(filing):`

What it does: Function score takes filing and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cb_of

Found in evaluation/score_p021.py at line 107.

Signature: `def cb_of(pr, sc):`

What it does: Function cb_of takes pr, sc and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — band_step

Found in evaluation/score_p021.py at line 163.

Signature: `def band_step(ni_v, axis):`

What it does: Function band_step takes ni_v, axis and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — entry_lines

Found in evaluation/splice_registry.py at line 10.

Signature: `def entry_lines(path):`

What it does: Function entry_lines takes path and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — server_blocks

Found in service/nginx-insert.py at line 41.

Signature: `def server_blocks(text):`

What it does: Yield (start, end) spans of each top-level `server { ... }` block.

(Where these words came from: doc comment.)

---

## function — main

Found in service/nginx-insert.py at line 63.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — rep

Found in service/tluf-submit.py at line 85.

Signature: `def rep(m):`

What it does: Function rep takes m and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — canonicalize

Found in service/tluf-submit.py at line 249.

Signature: `def canonicalize(locator):`

What it does: Return (address, error): exactly one of the two is None.

(Where these words came from: doc comment.)

---

## function — store_submission

Found in service/tluf-submit.py at line 313.

Signature: `def store_submission(fields, address):`

What it does: Function store_submission takes fields, address and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — do_GET

Found in service/tluf-submit.py at line 347.

Signature: `def do_GET(self):`

What it does: Function do_GET takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — do_POST

Found in service/tluf-submit.py at line 353.

Signature: `def do_POST(self):`

What it does: Function do_POST takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — log_message

Found in service/tluf-submit.py at line 410.

Signature: `def log_message(self, fmt, *args):`

What it does: Function log_message takes self, fmt, *args and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in service/tluf-submit.py at line 414.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in tracker/build_research_phext.py at line 35596.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

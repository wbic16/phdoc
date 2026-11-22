# moat — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 147). Scanned at commit fb84a7c1a3e52aa9f30219d3e3d1e4fd0b06e077. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Summary

MOAT is the internal name for the epistemic process: ingest a research claim, split it into individual claims, and file each one as **True, False, Unknown, or Fiction** (TLUF), with a receipt an independent checker can re-run. The hosted service built on the process is **Assay** — the assayer tests the ore, never owns it, never refuses the test. Its front door is [tluf.org](https://tluf.org).

(The opening of the project's README, quoted as written.)

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 2 commits.

The author of record is **Will Bickford** — 2 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Emise — 1 commit
- Will Bickford (his own identities) — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- JavaScript: 1 file, 283 lines total, 17 blank, 8 comment, 258 code
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

- Origin: https://github.com/wbic16/moat
- Branch: exo
- Head commit: fb84a7c1a3e52aa9f30219d3e3d1e4fd0b06e077
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `concept_source` (epistemic/extract_registry.py:875) — Function concept_source takes scroll_no.
- function `heading_of` (epistemic/extract_registry.py:953) — Function heading_of takes text.
- function `claim_of` (epistemic/extract_registry.py:1072) — Function claim_of takes coord, text.
- function `classify` (epistemic/extract_registry.py:1086) — Return (status, basis, evidence).
- function `override_evidence` (epistemic/extract_registry.py:1166) — Function override_evidence takes coord.
- function `main` (epistemic/extract_registry.py:1236) — Function main takes no inputs beyond the value it is called on.
- function `rng` (evaluation/evaluation_analysis.py:23) — Function rng takes v.
- function `links_total` (evaluation/evaluation_analysis.py:29) — Function links_total takes r, side, other=None.
- function `op_pred_of` (evaluation/evaluation_analysis.py:42) — Function op_pred_of takes pr, sc.
- function `op_range_pred_of` (evaluation/evaluation_analysis.py:47) — Function op_range_pred_of takes pr, sc.
- function `links_split` (evaluation/evaluation_analysis.py:53) — Function links_split takes s.
- function `band` (evaluation/evaluation_analysis.py:117) — Function band takes ni, axis.
- function `step` (evaluation/evaluation_analysis.py:122) — Function step takes ni, axis, axis_step=-5.
- function `survives` (evaluation/evaluation_analysis.py:131) — Function survives takes k.
- function `null_oracle` (evaluation/evaluation_analysis.py:142) — Function null_oracle takes op_before, ni, axis.
- function `agg` (evaluation/evaluation_analysis.py:193) — Function agg takes rs, key.
- function `summary` (evaluation/evaluation_analysis.py:196) — Function summary takes rs, label.
- function `apply` (evaluation/ingestion/fix_p018_words.py:154) — Function apply takes path, pairs.
- function `rep` (evaluation/ingestion/integrate_p013.py:9) — Function rep takes old, new, n=1.
- function `ins_after` (evaluation/ingestion/integrate_p013.py:15) — Function ins_after takes old, add, n=1.
- function `rep` (evaluation/ingestion/integrate_p018.py:17) — Function rep takes old, new, n=1.
- function `rep` (evaluation/ingestion/integrate_p018_phaseb.py:11) — Function rep takes old, new, n=1.
- function `rep` (evaluation/ingestion/integrate_p021.py:18) — Function rep takes old, new, n=1.
- function `cb_of` (evaluation/score_p020.py:53) — Function cb_of takes pr, sc.
- function `ni` (evaluation/score_p020.py:61) — Function ni takes new, revised, cb.
- function `score` (evaluation/score_p020.py:81) — Function score takes filing.
- function `band_step` (evaluation/score_p020.py:162) — Function band_step takes ni_v, axis.
- function `ni` (evaluation/score_p021.py:62) — Function ni takes new, revised, cb.
- function `score` (evaluation/score_p021.py:80) — Function score takes filing.
- function `cb_of` (evaluation/score_p021.py:107) — Function cb_of takes pr, sc.
- function `band_step` (evaluation/score_p021.py:163) — Function band_step takes ni_v, axis.
- function `entry_lines` (evaluation/splice_registry.py:10) — Function entry_lines takes path.
- function `server_blocks` (service/nginx-insert.py:41) — Yield (start, end) spans of each top-level `server { ...
- function `main` (service/nginx-insert.py:63) — Function main takes no inputs beyond the value it is called on.
- function `rep` (service/tluf-submit.py:85) — Function rep takes m.
- function `canonicalize` (service/tluf-submit.py:249) — Return (address, error): exactly one of the two is None.
- function `store_submission` (service/tluf-submit.py:313) — Function store_submission takes fields, address.
- function `Handler.do_GET` (service/tluf-submit.py:347) — Function Handler.do_GET takes no inputs beyond the value it is called on.
- function `Handler.do_POST` (service/tluf-submit.py:353) — Function Handler.do_POST takes no inputs beyond the value it is called on.
- function `Handler.log_message` (service/tluf-submit.py:410) — Function Handler.log_message takes self, fmt, *args.
- function `main` (service/tluf-submit.py:414) — Function main takes no inputs beyond the value it is called on.
- function `main` (tracker/build_research_phext.py:35596) — Function main takes no inputs beyond the value it is called on.

---

## function — concept_source

Found in epistemic/extract_registry.py at line 875.

Surface: public API.

Signature: `def concept_source(scroll_no)`

What it does: Function concept_source takes scroll_no. Its return type is unannotated; the code shows it returning "arXiv:2611.05417"; "arXiv:2611.00463"; "arXiv:2604.03136"; "arXiv:2410.19225"; "arXiv:2611.06949"; "Study S001 (MiniLean4 book and code)"; "Study S002 (Aeneas repo and its two design pape; "Study S003 (h5i repo design files)"; "arXiv:2610.38214"; "arXiv:2610.42112"; "Study S004 (Phrack 49 file 14, Aleph One, 1996); "Study S005 (Astryx repo, github.com/meta/astryx; "Study S006 (TinyTorch, mlsysbook.ai / github.co; "arXiv:2611.09694"; "arXiv:2611.08625"; "arXiv:2611.08794"; "arXiv:2611.07088"; "arXiv:2611.11959"; "arXiv:2611.14463"; "arXiv:2510.01346"; "arXiv:2611.14039"; "Study S007 (three essays of John D. Cook, johnd; "arXiv:1903.06830"; "Study S008 (Continual-ARC repo and paper, githu; "Study S009 (Terence Tao, 'Lean Grimoire', terry; "Paper 017 (Google DeepMind document 36, 'AlphaP; "Study S010 (Math, Inc., 'Abstraction considered; "arXiv:2611.17082"; "arXiv:2611.14543"; "arXiv:2611.14552"; "arXiv:2611.16759"; "Paper 022 (Bruce Sterling, 'Maneki Neko', Light; "Study S011 (the Vimalakirti Sutra in Burton Wat; "Study S012 ('Beating the frontier on NetHack', ; "Study S013 (Hexagon, hexagonmath.org)"; "Fringe F001 (Paul Crowley, 'Cardchoose', cipher; "Study S014 (Alex Miller, 'Please Implement A Qu.

(Where these words came from: inferred.)

---

## function — heading_of

Found in epistemic/extract_registry.py at line 953.

Surface: public API.

Signature: `def heading_of(text)`

What it does: Function heading_of takes text. Its return type is unannotated; the code shows it returning line.lstrip("#").strip(); "(empty scroll)".

(Where these words came from: inferred.)

---

## function — claim_of

Found in epistemic/extract_registry.py at line 1072.

Surface: public API.

Signature: `def claim_of(coord, text)`

What it does: Function claim_of takes coord, text. Its return type is unannotated; the code shows it returning CLAIM_TIGHTEN[coord]; ("Standoff references for its chapter: the found; "Open questions carried forward at this point of; h.

(Where these words came from: inferred.)

---

## function — classify

Found in epistemic/extract_registry.py at line 1086.

Surface: public API.

Signature: `def classify(coord, text)`

What it does: Return (status, basis, evidence). Kind defaults; content decides.

(Where these words came from: direct.)

---

## function — override_evidence

Found in epistemic/extract_registry.py at line 1166.

Surface: public API.

Signature: `def override_evidence(coord)`

What it does: Function override_evidence takes coord. Its return type is unannotated; the code shows it returning {.

(Where these words came from: inferred.)

---

## function — main

Found in epistemic/extract_registry.py at line 1236.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — rng

Found in evaluation/evaluation_analysis.py at line 23.

Surface: public API.

Signature: `def rng(v)`

What it does: Function rng takes v. Its return type is unannotated; the code shows it returning (int(m.group(1)), int(m.group(2))) if m else Non.

(Where these words came from: inferred.)

---

## function — links_total

Found in evaluation/evaluation_analysis.py at line 29.

Surface: public API.

Signature: `def links_total(r, side, other=None)`

What it does: Function links_total takes r, side, other=None. Its return type is unannotated; the code shows it returning None.

(Where these words came from: inferred.)

---

## function — op_pred_of

Found in evaluation/evaluation_analysis.py at line 42.

Surface: public API.

Signature: `def op_pred_of(pr, sc)`

What it does: Function op_pred_of takes pr, sc. Its return type is unannotated; the code shows it returning None.

(Where these words came from: inferred.)

---

## function — op_range_pred_of

Found in evaluation/evaluation_analysis.py at line 47.

Surface: public API.

Signature: `def op_range_pred_of(pr, sc)`

What it does: Function op_range_pred_of takes pr, sc. Its return type is unannotated; the code shows it returning None.

(Where these words came from: inferred.)

---

## function — links_split

Found in evaluation/evaluation_analysis.py at line 53.

Surface: public API.

Signature: `def links_split(s)`

What it does: Function links_split takes s. Its return type is unannotated; the code shows it returning (int(m.group(1)), int(m.group(2))) if m else Non.

(Where these words came from: inferred.)

---

## function — band

Found in evaluation/evaluation_analysis.py at line 117.

Surface: public API.

Signature: `def band(ni, axis)`

What it does: Function band takes ni, axis. Its return type is unannotated; the code shows it returning 'low'.

(Where these words came from: inferred.)

---

## function — step

Found in evaluation/evaluation_analysis.py at line 122.

Surface: public API.

Signature: `def step(ni, axis, axis_step=-5)`

What it does: Function step takes ni, axis, axis_step=-5. Its return type is unannotated; the code shows it returning {'low':10, 'mid':5, 'axis':axis_step, 'gt50':0}[.

(Where these words came from: inferred.)

---

## function — survives

Found in evaluation/evaluation_analysis.py at line 131.

Surface: public API.

Signature: `def survives(k)`

What it does: Function survives takes k. Its return type is unannotated; the code shows it returning band(moved, r['act_axis']) == band(r['ni_pred_se.

(Where these words came from: inferred.)

---

## function — null_oracle

Found in evaluation/evaluation_analysis.py at line 142.

Surface: public API.

Signature: `def null_oracle(op_before, ni, axis)`

What it does: Function null_oracle takes op_before, ni, axis. Its return type is unannotated; the code shows it returning pt, (max(0, pt-10), min(100, pt+10)).

(Where these words came from: inferred.)

---

## function — agg

Found in evaluation/evaluation_analysis.py at line 193.

Surface: public API.

Signature: `def agg(rs, key)`

What it does: Function agg takes rs, key. Its return type is unannotated; the code shows it returning st.mean(r[key] for r in rs).

(Where these words came from: inferred.)

---

## function — summary

Found in evaluation/evaluation_analysis.py at line 196.

Surface: public API.

Signature: `def summary(rs, label)`

What it does: Function summary takes rs, label. Its return type is unannotated; the code shows it returning d.

(Where these words came from: inferred.)

---

## function — apply

Found in evaluation/ingestion/fix_p018_words.py at line 154.

Surface: public API.

Signature: `def apply(path, pairs)`

What it does: Function apply takes path, pairs. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — rep

Found in evaluation/ingestion/integrate_p013.py at line 9.

Surface: public API.

Signature: `def rep(old, new, n=1)`

What it does: Function rep takes old, new, n=1. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — ins_after

Found in evaluation/ingestion/integrate_p013.py at line 15.

Surface: public API.

Signature: `def ins_after(old, add, n=1)`

What it does: Function ins_after takes old, add, n=1. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — rep

Found in evaluation/ingestion/integrate_p018.py at line 17.

Surface: public API.

Signature: `def rep(old, new, n=1)`

What it does: Function rep takes old, new, n=1. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — rep

Found in evaluation/ingestion/integrate_p018_phaseb.py at line 11.

Surface: public API.

Signature: `def rep(old, new, n=1)`

What it does: Function rep takes old, new, n=1. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — rep

Found in evaluation/ingestion/integrate_p021.py at line 18.

Surface: public API.

Signature: `def rep(old, new, n=1)`

What it does: Function rep takes old, new, n=1. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — cb_of

Found in evaluation/score_p020.py at line 53.

Surface: public API.

Signature: `def cb_of(pr, sc)`

What it does: Function cb_of takes pr, sc. Its return type is unannotated; the code shows it returning sc.get("concepts_before_sealed", pr.get("concept.

(Where these words came from: inferred.)

---

## function — ni

Found in evaluation/score_p020.py at line 61.

Surface: public API.

Signature: `def ni(new, revised, cb)`

What it does: Function ni takes new, revised, cb. Its return type is unannotated; the code shows it returning (new + 0.5 * revised) / (cb + new).

(Where these words came from: inferred.)

---

## function — score

Found in evaluation/score_p020.py at line 81.

Surface: public API.

Signature: `def score(filing)`

What it does: Function score takes filing. Its return type is unannotated; the code shows it returning dict(.

(Where these words came from: inferred.)

---

## function — band_step

Found in evaluation/score_p020.py at line 162.

Surface: public API.

Signature: `def band_step(ni_v, axis)`

What it does: Function band_step takes ni_v, axis. Its return type is unannotated; the code shows it returning -5; 0; 5; 10.

(Where these words came from: inferred.)

---

## function — ni

Found in evaluation/score_p021.py at line 62.

Surface: public API.

Signature: `def ni(new, revised, cb)`

What it does: Function ni takes new, revised, cb. Its return type is unannotated; the code shows it returning (new + 0.5 * revised) / (cb + new).

(Where these words came from: inferred.)

---

## function — score

Found in evaluation/score_p021.py at line 80.

Surface: public API.

Signature: `def score(filing)`

What it does: Function score takes filing. Its return type is unannotated; the code shows it returning dict(.

(Where these words came from: inferred.)

---

## function — cb_of

Found in evaluation/score_p021.py at line 107.

Surface: public API.

Signature: `def cb_of(pr, sc)`

What it does: Function cb_of takes pr, sc. Its return type is unannotated; the code shows it returning sc.get("concepts_before_sealed", pr.get("concept.

(Where these words came from: inferred.)

---

## function — band_step

Found in evaluation/score_p021.py at line 163.

Surface: public API.

Signature: `def band_step(ni_v, axis)`

What it does: Function band_step takes ni_v, axis. Its return type is unannotated; the code shows it returning -5; 0; 5; 10.

(Where these words came from: inferred.)

---

## function — entry_lines

Found in evaluation/splice_registry.py at line 10.

Surface: public API.

Signature: `def entry_lines(path)`

What it does: Function entry_lines takes path. Its return type is unannotated; the code shows it returning lines[start + 1:end].

(Where these words came from: inferred.)

---

## function — server_blocks

Found in service/nginx-insert.py at line 41.

Surface: public API.

Signature: `def server_blocks(text)`

What it does: Yield (start, end) spans of each top-level `server { ... }` block.

(Where these words came from: direct.)

---

## function — main

Found in service/nginx-insert.py at line 63.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 3; 2.

(Where these words came from: inferred.)

---

## function — rep

Found in service/tluf-submit.py at line 85.

Surface: public API.

Signature: `def rep(m)`

What it does: Function rep takes m. Its return type is unannotated; the code shows it returning ch; "%" + m.group(1).upper().

(Where these words came from: inferred.)

---

## function — canonicalize

Found in service/tluf-submit.py at line 249.

Surface: public API.

Signature: `def canonicalize(locator)`

What it does: Return (address, error): exactly one of the two is None.

(Where these words came from: direct.)

---

## function — store_submission

Found in service/tluf-submit.py at line 313.

Surface: public API.

Signature: `def store_submission(fields, address)`

What it does: Function store_submission takes fields, address. Its return type is unannotated; the code shows it returning seq.

(Where these words came from: inferred.)

---

## function — Handler.do_GET

Found in service/tluf-submit.py at line 347.

Surface: public API.

Signature: `def do_GET(self)`

What it does: Function Handler.do_GET takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Handler.do_POST

Found in service/tluf-submit.py at line 353.

Surface: public API.

Signature: `def do_POST(self)`

What it does: Function Handler.do_POST takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Handler.log_message

Found in service/tluf-submit.py at line 410.

Surface: public API.

Signature: `def log_message(self, fmt, *args)`

What it does: Function Handler.log_message takes self, fmt, *args. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in service/tluf-submit.py at line 414.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in tracker/build_research_phext.py at line 35596.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/moat/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

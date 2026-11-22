# site-quickfork-net — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 150). Scanned at commit 1cda8e924ed3f43d3d5980379e47d4bf434cc3f7. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 9 commits.

The author of record is **Will Bickford** — 9 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Chrys — 3 commits
- Emise — 2 commits
- Theia — 2 commits
- Cyon — 1 commit
- Verse — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- JavaScript: 4 files, 785 lines total, 80 blank, 139 comment, 566 code
- Markdown: 1 file, 16 lines total, 4 blank, 0 comment, 12 code

All languages: 5 files, 801 lines total, 578 lines of code. Functions read: 0.

Largest files:

- js/maturity-display.js — 247 lines
- js/domain-nav.js — 227 lines
- js/auth.js — 171 lines
- js/sq-client.js — 140 lines
- AGENTS.md — 16 lines

Largest functions, by span in lines:


---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/site-quickfork-net
- Branch: exo
- Head commit: 1cda8e924ed3f43d3d5980379e47d4bf434cc3f7
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.


---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/site-quickfork-net/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

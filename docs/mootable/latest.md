# mootable — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 151). Scanned at commit df424f3ab63a90dcb5c522ab5dd8d0313636f442. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Summary

This repository is the in-progress Solid-owned reconstruction of `speedy-ts`, following [`SOLID_RESTRUCTURE.MD`](./SOLID_RESTRUCTURE.MD). The current runnable milestone loads the original 92-Block workspace sample through the canonical model and Solid renderer. The smaller two-pane PlainText/Standoff pilot remains available as a focused view-local-state example.

(The opening of the project's README, quoted as written.)

---

## Dependencies

### JavaScript (npm) — package.json

License stated in the manifest: MIT.

Direct dependencies: 71.

- 
    "@antv/x6 — 3.1.8
- @atjson/offset-annotations — ^0.63.0
- @atjson/source-html — ^0.46.0
- @codemirror/lang-javascript — ^6.2.5
- @codemirror/lang-markdown — ^6.5.2
- @codemirror/language-data — ^6.5.2
- @kobalte/core — ^0.13.14
- @lezer/javascript — ^1.5.6
- @lezer/highlight — ^1.2.5
- @paper-design/shaders — ^0.0.82
- @retorquere/bibtex-parser — 13.0.11
- @solid-primitives/autofocus — ^0.1.5
- @solid-primitives/scheduled — ^1.5.3
- @surrealdb/node — ^3.2.0
- @surrealdb/node-native — ^4.0.0
- @tabler/icons-solidjs — ^3.50.0
- axios — ^1.21.0
- better-sqlite3 — 13.0.3
- body-parser — ^2.4.0
- clsx — ^2.1.1
- codemirror — ^6.0.2
- cors — ^2.8.6
- decode-named-character-reference — 1.3.0
- express — ^5.3.0
- fflate — 0.8.3
- fs — ^0.0.1-security
- ical.js — 2.2.1
- ipaddr.js — 2.5.0
- leader-line-new — ^1.1.9
- mdast-util-from-markdown — 2.1.0
- mdast-util-gfm-table — 2.0.1
- mermaid — 12.3.0
- micromark-extension-gfm-table — 2.1.2
- multer — ^2.5.0
- node-static — ^0.7.11
- openseadragon — 6.1.1
- papaparse — 5.7.0
- parse5 — 8.1.0
- path — ^0.12.7
- pdfjs-dist — 6.5.438
- saxes — 6.0.0
- sharp — 0.35.5
- solid-contextmenu — ^0.0.2
- solid-js — ^1.9.18
- surrealdb — ^2.2.1
- three — 0.187.0
- underscore — ^1.13.8
- url — ^0.11.4
- url-regex-safe — ^4.0.0
- uuid — ^14.0.3
- yaml — 2.9.1
- youtube-player — ^5.6.0
- @lezer/markdown — ^1.7.2"
  
- 
    "@types/body-parser — ^1.19.6
- @types/express — ^5.0.6
- @types/mdast — 4.0.4
- @types/multer — ^2.3.0
- @types/node — ^26.6.6
- @types/papaparse — 5.5.2
- @types/three — 0.187.0
- @types/underscore — ^1.13.0
- @types/uuid — ^11.0.0
- @types/youtube-player — ^5.5.11
- bootstrap-css — ^4.0.0-alpha.5
- esbuild — ^0.28.2
- jsdom — ^30.1.3
- solid-devtools — ^0.34.5
- typescript — ^6.0.0
- vite — ^8.3.5
- vite-plugin-solid — ^2.11.15
- vitest — ^5.1.0"
  


---

## Authors

The git log holds 337 commits.

The author of record is **Will Bickford** — 6 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Emise — 6 commits

Other authors, kept as themselves (not Exocortical cluster personas; never folded):

- Iian <iian.d.neill@gmail.com> — 330 commits
- Iian Neill <iian.d.neill@gmail.com> — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- JavaScript: 124 files, 67984 lines total, 1164 blank, 2449 comment, 64371 code
- Markdown: 306 files, 63242 lines total, 15853 blank, 124 comment, 47265 code
- Python: 2 files, 126 lines total, 9 blank, 0 comment, 117 code
- Shell: 2 files, 661 lines total, 55 blank, 80 comment, 526 code
- TypeScript: 855 files, 91019 lines total, 2654 blank, 4049 comment, 84316 code

All languages: 1289 files, 223032 lines total, 196595 lines of code. Functions read: 26.

Largest files:

- src/library/original/components/outliner-text-block.js — 10205 lines
- docs/development/INPUT_HANDLER_SOURCE_LEDGER.md — 9442 lines
- src/library/original/modules/speedy/editor-new-2.js — 4655 lines
- src/library/original/speedy.js — 4655 lines
- src/library/original/modules/speedy/editor-new.js — 3347 lines

Largest functions, by span in lines:

- main in deploy/deploy.sh — about 183 lines
- run_check in deploy/deploy.sh — about 80 lines
- resource in docs/architecture/sqlite/check-proposed-schema.py — about 72 lines
- apply_rendered_site in deploy/deploy.sh — about 49 lines
- adopt_live_ssl in deploy/deploy.sh — about 36 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 21 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 28. FIXME marks: 0.
- Public items with a doc comment: 0 of 26 (0%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: local clone (repository name: mootable)
- Branch: mootable-main
- Head commit: df424f3ab63a90dcb5c522ab5dd8d0313636f442
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `die` (deploy/deploy.sh:49) — Function die takes no inputs beyond the value it is called on.
- function `usage` (deploy/deploy.sh:54) — Function usage takes no inputs beyond the value it is called on.
- function `as_owner` (deploy/deploy.sh:75) — Function as_owner takes no inputs beyond the value it is called on.
- function `dns_points_here` (deploy/deploy.sh:79) — Function dns_points_here takes no inputs beyond the value it is called on.
- function `file_sha256` (deploy/deploy.sh:83) — Function file_sha256 takes no inputs beyond the value it is called on.
- function `read_root_file` (deploy/deploy.sh:89) — Function read_root_file takes no inputs beyond the value it is called on.
- function `nginx_template` (deploy/deploy.sh:94) — Function nginx_template takes no inputs beyond the value it is called on.
- function `unit_template` (deploy/deploy.sh:109) — Function unit_template takes no inputs beyond the value it is called on.
- function `render_site_body` (deploy/deploy.sh:126) — Function render_site_body takes no inputs beyond the value it is called on.
- function `adopt_live_ssl` (deploy/deploy.sh:138) — Function adopt_live_ssl takes no inputs beyond the value it is called on.
- function `path_exists` (deploy/deploy.sh:174) — Function path_exists takes no inputs beyond the value it is called on.
- function `drop_missing_dhparam` (deploy/deploy.sh:182) — Function drop_missing_dhparam takes no inputs beyond the value it is called on.
- function `stamp_managed` (deploy/deploy.sh:195) — Function stamp_managed takes no inputs beyond the value it is called on.
- function `write_bootstrap_body` (deploy/deploy.sh:204) — Function write_bootstrap_body takes no inputs beyond the value it is called on.
- function `nginx_disposition` (deploy/deploy.sh:235) — Function nginx_disposition takes no inputs beyond the value it is called on.
- function `with_force` (deploy/deploy.sh:269) — Function with_force takes no inputs beyond the value it is called on.
- function `require_hardened_body` (deploy/deploy.sh:278) — Function require_hardened_body takes no inputs beyond the value it is called on.
- function `install_rendered_site` (deploy/deploy.sh:294) — Function install_rendered_site takes no inputs beyond the value it is called on.
- function `apply_rendered_site` (deploy/deploy.sh:328) — Function apply_rendered_site takes no inputs beyond the value it is called on.
- function `run_check` (deploy/deploy.sh:377) — Function run_check takes no inputs beyond the value it is called on.
- function `main` (deploy/deploy.sh:457) — Function main takes no inputs beyond the value it is called on.
- function `check` (docs/architecture/sqlite/check-proposed-schema.py:14) — Function check takes name, condition.
- function `rejects` (docs/architecture/sqlite/check-proposed-schema.py:19) — Function rejects takes name, sql, values=.
- function `resource` (docs/architecture/sqlite/check-proposed-schema.py:28) — Function resource takes guid, block.
- function `read` (experiments/initialis/summarize.py:6) — Function read takes p.
- function `stats` (experiments/initialis/summarize.py:7) — Function stats takes xs.

---

## function — die

Found in deploy/deploy.sh at line 49.

Surface: public API.

Signature: `die()`

What it does: Function die takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — usage

Found in deploy/deploy.sh at line 54.

Surface: public API.

Signature: `usage()`

What it does: Function usage takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — as_owner

Found in deploy/deploy.sh at line 75.

Surface: public API.

Signature: `as_owner()`

What it does: Function as_owner takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — dns_points_here

Found in deploy/deploy.sh at line 79.

Surface: public API.

Signature: `dns_points_here()`

What it does: Function dns_points_here takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — file_sha256

Found in deploy/deploy.sh at line 83.

Surface: public API.

Signature: `file_sha256()`

What it does: Function file_sha256 takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — read_root_file

Found in deploy/deploy.sh at line 89.

Surface: public API.

Signature: `read_root_file()`

What it does: Function read_root_file takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — nginx_template

Found in deploy/deploy.sh at line 94.

Surface: public API.

Signature: `nginx_template()`

What it does: Function nginx_template takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — unit_template

Found in deploy/deploy.sh at line 109.

Surface: public API.

Signature: `unit_template()`

What it does: Function unit_template takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — render_site_body

Found in deploy/deploy.sh at line 126.

Surface: public API.

Signature: `render_site_body()`

What it does: Function render_site_body takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — adopt_live_ssl

Found in deploy/deploy.sh at line 138.

Surface: public API.

Signature: `adopt_live_ssl()`

What it does: Function adopt_live_ssl takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — path_exists

Found in deploy/deploy.sh at line 174.

Surface: public API.

Signature: `path_exists()`

What it does: Function path_exists takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — drop_missing_dhparam

Found in deploy/deploy.sh at line 182.

Surface: public API.

Signature: `drop_missing_dhparam()`

What it does: Function drop_missing_dhparam takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — stamp_managed

Found in deploy/deploy.sh at line 195.

Surface: public API.

Signature: `stamp_managed()`

What it does: Function stamp_managed takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — write_bootstrap_body

Found in deploy/deploy.sh at line 204.

Surface: public API.

Signature: `write_bootstrap_body()`

What it does: Function write_bootstrap_body takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — nginx_disposition

Found in deploy/deploy.sh at line 235.

Surface: public API.

Signature: `nginx_disposition()`

What it does: Function nginx_disposition takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — with_force

Found in deploy/deploy.sh at line 269.

Surface: public API.

Signature: `with_force()`

What it does: Function with_force takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — require_hardened_body

Found in deploy/deploy.sh at line 278.

Surface: public API.

Signature: `require_hardened_body()`

What it does: Function require_hardened_body takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — install_rendered_site

Found in deploy/deploy.sh at line 294.

Surface: public API.

Signature: `install_rendered_site()`

What it does: Function install_rendered_site takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — apply_rendered_site

Found in deploy/deploy.sh at line 328.

Surface: public API.

Signature: `apply_rendered_site()`

What it does: Function apply_rendered_site takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — run_check

Found in deploy/deploy.sh at line 377.

Surface: public API.

Signature: `run_check()`

What it does: Function run_check takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — main

Found in deploy/deploy.sh at line 457.

Surface: public API.

Signature: `main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — check

Found in docs/architecture/sqlite/check-proposed-schema.py at line 14.

Surface: public API.

Signature: `def check(name, condition)`

What it does: Function check takes name, condition. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — rejects

Found in docs/architecture/sqlite/check-proposed-schema.py at line 19.

Surface: public API.

Signature: `def rejects(name, sql, values=())`

What it does: Function rejects takes name, sql, values=. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — resource

Found in docs/architecture/sqlite/check-proposed-schema.py at line 28.

Surface: public API.

Signature: `def resource(guid, block)`

What it does: Function resource takes guid, block. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — read

Found in experiments/initialis/summarize.py at line 6.

Surface: public API.

Signature: `def read(p): return json.loads(p.read_text())`

What it does: Function read takes p. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — stats

Found in experiments/initialis/summarize.py at line 7.

Surface: public API.

Signature: `def stats(xs)`

What it does: Function stats takes xs. Its return type is unannotated; the code shows it returning dict(n=len(s),median=s[len(s)//2],p95=s[math.cei.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/mootable/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

# mutable-os — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 104). Project root: /source/mutable-os. Scanned at commit 48cac17ae14c2d9132d0b110162677a669dbad3e. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### JavaScript (npm) — package.json

License stated in the manifest: MIT.

Direct dependencies: 76.

- 
    "@antv/x6 — 3.1.8
- @atjson/offset-annotations — ^0.63.0
- @atjson/source-html — ^0.46.0
- @codemirror/lang-javascript — ^6.2.5
- @codemirror/lang-markdown — ^6.5.2
- @codemirror/language-data — ^6.5.2
- @kobalte/core — ^0.13.14
- @lezer/highlight — ^1.2.5
- @lezer/javascript — ^1.5.6
- @lezer/markdown — ^1.7.2
- @paper-design/shaders — ^0.0.16
- @retorquere/bibtex-parser — 13.0.11
- @solid-primitives/autofocus — ^0.0.111
- @solid-primitives/scheduled — ^1.4.4
- @surrealdb/node — ^3.2.0
- @surrealdb/node-native — ^4.0.0
- @tabler/icons-solidjs — ^3.31.0
- @tanstack/solid-virtual — 3.13.53
- axios — ^1.7.3
- better-sqlite3 — 12.4.1
- body-parser — ^1.20.2
- clsx — ^2.1.1
- codemirror — ^6.0.2
- cors — ^2.8.6
- decode-named-character-reference — 1.3.0
- express — ^5.3.0
- fflate — 0.8.3
- fs — ^0.0.1-security
- ical.js — 2.2.1
- ipaddr.js — 2.5.0
- js-tiktoken — 1.0.21
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
- pdfjs-dist — 5.4.624
- pdfkit — 0.20.2
- saxes — 6.0.0
- sharp — 0.35.5
- solid-contextmenu — ^0.0.2
- solid-js — ^1.9.18
- sqlite-vec — 0.1.9
- surrealdb — ^2.2.1
- three — 0.187.0
- underscore — ^1.13.8
- url — ^0.11.4
- url-regex-safe — ^4.0.0
- uuid — ^14.0.3
- yaml — 2.9.1
- youtube-player — ^5.6.0"
  
- 
    "@types/body-parser — ^1.19.6
- @types/express — ^5.0.6
- @types/mdast — 4.0.4
- @types/multer — ^2.3.0
- @types/node — ^26.6.6
- @types/papaparse — 5.5.2
- @types/pdfkit — 0.17.7
- @types/three — 0.186.0
- @types/underscore — ^1.11.15
- @types/uuid — ^9.0.8
- @types/youtube-player — ^5.5.11
- bootstrap-css — ^4.0.0-alpha.5
- esbuild — ^0.28.2
- jsdom — ^30.1.3
- solid-devtools — ^0.34.5
- typescript — ^5.6.2
- vite — ^8.3.5
- vite-plugin-solid — ^2.11.15
- vitest — ^5.1.0"
  


---

## Authors

The git log holds 448 commits. The authors, by commit count:

- Iian <iian.d.neill@gmail.com> — 420 commits (93.8%)
- Emise <emise@localhost> — 24 commits (5.4%)
- Will Bickford <wbic16@gmail.com> — 2 commits (0.4%)
- Iian Neill <iian.d.neill@gmail.com> — 1 commit (0.2%)
- Theia <theia@mirrorborn.us> — 1 commit (0.2%)

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- JavaScript: 624 files, 143352 lines total, 2750 blank, 7212 comment, 133390 code
- Markdown: 430 files, 83421 lines total, 21525 blank, 151 comment, 61745 code
- Python: 4 files, 224 lines total, 9 blank, 0 comment, 215 code
- Shell: 2 files, 661 lines total, 55 blank, 80 comment, 526 code
- TypeScript: 1436 files, 120175 lines total, 3232 blank, 5755 comment, 111188 code

All languages: 2496 files, 347833 lines total, 307064 lines of code. Functions read: 29.

Largest files:

- src/library/original/components/outliner-text-block.js — 10205 lines
- docs/development/INPUT_HANDLER_SOURCE_LEDGER.md — 9442 lines
- dist/server/application-services/host.js — 6491 lines
- src/library/original/modules/speedy/editor-new-2.js — 4655 lines
- src/library/original/speedy.js — 4655 lines

Largest functions, by span in lines:

- main in deploy/deploy.sh — about 183 lines
- run_check in deploy/deploy.sh — about 80 lines
- resource in docs/architecture/sqlite/check-proposed-schema.py — about 72 lines
- apply_rendered_site in deploy/deploy.sh — about 49 lines
- adopt_live_ssl in deploy/deploy.sh — about 36 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 32 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 33. FIXME marks: 0.
- Public items with a doc comment: 0 of 29 (0%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/mutable-os.git
- Branch: main
- Head commit: 48cac17ae14c2d9132d0b110162677a669dbad3e
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `die` (deploy/deploy.sh:49) — Function die takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is gi
- function `usage` (deploy/deploy.sh:54) — Function usage takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is 
- function `as_owner` (deploy/deploy.sh:75) — Function as_owner takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `dns_points_here` (deploy/deploy.sh:79) — Function dns_points_here takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `file_sha256` (deploy/deploy.sh:83) — Function file_sha256 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `read_root_file` (deploy/deploy.sh:89) — Function read_root_file takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `nginx_template` (deploy/deploy.sh:94) — Function nginx_template takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `unit_template` (deploy/deploy.sh:109) — Function unit_template takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `render_site_body` (deploy/deploy.sh:126) — Function render_site_body takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `adopt_live_ssl` (deploy/deploy.sh:138) — Function adopt_live_ssl takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `path_exists` (deploy/deploy.sh:174) — Function path_exists takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `drop_missing_dhparam` (deploy/deploy.sh:182) — Function drop_missing_dhparam takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `stamp_managed` (deploy/deploy.sh:195) — Function stamp_managed takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `write_bootstrap_body` (deploy/deploy.sh:204) — Function write_bootstrap_body takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `nginx_disposition` (deploy/deploy.sh:235) — Function nginx_disposition takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `with_force` (deploy/deploy.sh:269) — Function with_force takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `require_hardened_body` (deploy/deploy.sh:278) — Function require_hardened_body takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `install_rendered_site` (deploy/deploy.sh:294) — Function install_rendered_site takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `apply_rendered_site` (deploy/deploy.sh:328) — Function apply_rendered_site takes no inputs beyond the value it is called on, and gives back nothing; it does its work 
- function `run_check` (deploy/deploy.sh:377) — Function run_check takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it
- function `main` (deploy/deploy.sh:457) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `check` (docs/architecture/sqlite/check-proposed-schema.py:14) — Function check takes name, condition and gives back nothing; it does its work on what it is given.
- function `rejects` (docs/architecture/sqlite/check-proposed-schema.py:19) — Function rejects takes name, sql, values= and gives back nothing; it does its work on what it is given.
- function `resource` (docs/architecture/sqlite/check-proposed-schema.py:28) — Function resource takes guid, block and gives back nothing; it does its work on what it is given.
- function `nodes` (docs/development/evidence/dcp-scholarly-authoring/audit-case.py:20) — Function nodes takes value and gives back nothing; it does its work on what it is given.
- function `visit` (docs/development/evidence/dcp-scholarly-authoring/audit-case.py:36) — Function visit takes e and gives back nothing; it does its work on what it is given.
- function `objects` (docs/development/evidence/dcp-scholarly-authoring/review-live.py:4) — Function objects takes v and gives back nothing; it does its work on what it is given.
- function `read` (experiments/initialis/summarize.py:6) — Function read takes p and gives back nothing; it does its work on what it is given.
- function `stats` (experiments/initialis/summarize.py:7) — Function stats takes xs and gives back nothing; it does its work on what it is given.

---

## function — die

Found in deploy/deploy.sh at line 49.

Signature: `die()`

What it does: Function die takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — usage

Found in deploy/deploy.sh at line 54.

Signature: `usage()`

What it does: Function usage takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — as_owner

Found in deploy/deploy.sh at line 75.

Signature: `as_owner()`

What it does: Function as_owner takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — dns_points_here

Found in deploy/deploy.sh at line 79.

Signature: `dns_points_here()`

What it does: Function dns_points_here takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — file_sha256

Found in deploy/deploy.sh at line 83.

Signature: `file_sha256()`

What it does: Function file_sha256 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — read_root_file

Found in deploy/deploy.sh at line 89.

Signature: `read_root_file()`

What it does: Function read_root_file takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — nginx_template

Found in deploy/deploy.sh at line 94.

Signature: `nginx_template()`

What it does: Function nginx_template takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — unit_template

Found in deploy/deploy.sh at line 109.

Signature: `unit_template()`

What it does: Function unit_template takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — render_site_body

Found in deploy/deploy.sh at line 126.

Signature: `render_site_body()`

What it does: Function render_site_body takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — adopt_live_ssl

Found in deploy/deploy.sh at line 138.

Signature: `adopt_live_ssl()`

What it does: Function adopt_live_ssl takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — path_exists

Found in deploy/deploy.sh at line 174.

Signature: `path_exists()`

What it does: Function path_exists takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — drop_missing_dhparam

Found in deploy/deploy.sh at line 182.

Signature: `drop_missing_dhparam()`

What it does: Function drop_missing_dhparam takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — stamp_managed

Found in deploy/deploy.sh at line 195.

Signature: `stamp_managed()`

What it does: Function stamp_managed takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — write_bootstrap_body

Found in deploy/deploy.sh at line 204.

Signature: `write_bootstrap_body()`

What it does: Function write_bootstrap_body takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — nginx_disposition

Found in deploy/deploy.sh at line 235.

Signature: `nginx_disposition()`

What it does: Function nginx_disposition takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — with_force

Found in deploy/deploy.sh at line 269.

Signature: `with_force()`

What it does: Function with_force takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — require_hardened_body

Found in deploy/deploy.sh at line 278.

Signature: `require_hardened_body()`

What it does: Function require_hardened_body takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_rendered_site

Found in deploy/deploy.sh at line 294.

Signature: `install_rendered_site()`

What it does: Function install_rendered_site takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — apply_rendered_site

Found in deploy/deploy.sh at line 328.

Signature: `apply_rendered_site()`

What it does: Function apply_rendered_site takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_check

Found in deploy/deploy.sh at line 377.

Signature: `run_check()`

What it does: Function run_check takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in deploy/deploy.sh at line 457.

Signature: `main()`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — check

Found in docs/architecture/sqlite/check-proposed-schema.py at line 14.

Signature: `def check(name, condition):`

What it does: Function check takes name, condition and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — rejects

Found in docs/architecture/sqlite/check-proposed-schema.py at line 19.

Signature: `def rejects(name, sql, values=()):`

What it does: Function rejects takes name, sql, values= and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — resource

Found in docs/architecture/sqlite/check-proposed-schema.py at line 28.

Signature: `def resource(guid, block):`

What it does: Function resource takes guid, block and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — nodes

Found in docs/development/evidence/dcp-scholarly-authoring/audit-case.py at line 20.

Signature: `def nodes(value):`

What it does: Function nodes takes value and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — visit

Found in docs/development/evidence/dcp-scholarly-authoring/audit-case.py at line 36.

Signature: `def visit(e):`

What it does: Function visit takes e and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — objects

Found in docs/development/evidence/dcp-scholarly-authoring/review-live.py at line 4.

Signature: `def objects(v):`

What it does: Function objects takes v and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — read

Found in experiments/initialis/summarize.py at line 6.

Signature: `def read(p): return json.loads(p.read_text())`

What it does: Function read takes p and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — stats

Found in experiments/initialis/summarize.py at line 7.

Signature: `def stats(xs):`

What it does: Function stats takes xs and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

# bickfordllamaranch — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 105). Project root: /source/bickfordllamaranch. Scanned at commit 7fd5334b18c55a014c5d0e6d1ca615106aaca91b. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 20 commits. The authors, by commit count:

- Emise <emise@localhost> — 17 commits (85.0%)
- Cursor Agent <cursoragent@cursor.com> — 1 commit (5.0%)
- Verse <verse@visionquest.me> — 1 commit (5.0%)
- Will Bickford <wbic16@gmail.com> — 1 commit (5.0%)

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- JavaScript: 3 files, 539 lines total, 16 blank, 48 comment, 475 code
- Markdown: 12 files, 2586 lines total, 357 blank, 8 comment, 2221 code
- Python: 2 files, 375 lines total, 43 blank, 8 comment, 324 code
- Shell: 5 files, 1513 lines total, 93 blank, 248 comment, 1172 code

All languages: 22 files, 5013 lines total, 4192 lines of code. Functions read: 40.

Largest files:

- deploy/root-deploy.sh — 700 lines
- VERIFY.md — 650 lines
- deploy.sh — 551 lines
- site/admin/app.js — 514 lines
- research/sq-api.md — 355 lines

Largest functions, by span in lines:

- main in deploy.sh — about 77 lines
- _handle_upload in deploy/ranch-upload.py — about 76 lines
- main in reminders/remind.py — about 68 lines
- nginx_config in deploy.sh — about 46 lines
- usage in deploy.sh — about 45 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 2 of 36 (6%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: git@github.com:wbic16/bickfordllamaranch.git
- Branch: exo
- Head commit: 7fd5334b18c55a014c5d0e6d1ca615106aaca91b
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `detect_ext` (deploy/ranch-upload.py:48) — Return the extension for a known image type, or None.
- function `parse_multipart` (deploy/ranch-upload.py:63) — Minimal strict multipart/form-data parser.
- function `log_message` (deploy/ranch-upload.py:105) — Function log_message takes self, fmt, *args and gives back nothing; it does its work on what it is given.
- function `do_GET` (deploy/ranch-upload.py:121) — Function do_GET takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is
- function `do_POST` (deploy/ranch-upload.py:134) — Function do_POST takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it i
- function `main` (deploy/ranch-upload.py:226) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `cleanup` (deploy.sh:32) — Function cleanup takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it i
- function `usage` (deploy.sh:39) — Function usage takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is 
- function `git_with_auth` (deploy.sh:84) — Function git_with_auth takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `sync_source` (deploy.sh:103) — Function sync_source takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `have_cert` (deploy.sh:129) — Function have_cert takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it
- function `noindex_line` (deploy.sh:136) — Function noindex_line takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `ssl_snippets` (deploy.sh:142) — Function ssl_snippets takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `nginx_config` (deploy.sh:152) — Function nginx_config takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `install_site_file` (deploy.sh:198) — Function install_site_file takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `restore_foreign` (deploy.sh:207) — Function restore_foreign takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `rollback` (deploy.sh:221) — Function rollback takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `reload_nginx` (deploy.sh:239) — Function reload_nginx takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `strip_staging_from_other_vhosts` (deploy.sh:251) — Function strip_staging_from_other_vhosts takes no inputs beyond the value it is called on, and gives back nothing; it do
- function `build_certbot_cmd` (deploy.sh:293) — Function build_certbot_cmd takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `dns_points_here` (deploy.sh:312) — Function dns_points_here takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `staging_cert_matches_host` (deploy.sh:329) — Function staging_cert_matches_host takes no inputs beyond the value it is called on, and gives back nothing; it does its
- function `obtain_cert_if_needed` (deploy.sh:341) — Function obtain_cert_if_needed takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `assert_staging_noindex_config` (deploy.sh:362) — Function assert_staging_noindex_config takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `assert_staging_cert_config` (deploy.sh:369) — Function assert_staging_cert_config takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `assert_staging_cert_presented` (deploy.sh:383) — Function assert_staging_cert_presented takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `verify_http` (deploy.sh:408) — Function verify_http takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `parse_args` (deploy.sh:429) — Function parse_args takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `set_names` (deploy.sh:452) — Function set_names takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it
- function `main` (deploy.sh:475) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `fetch` (reminders/remind.py:33) — Function fetch takes path and gives back nothing; it does its work on what it is given.
- function `load` (reminders/remind.py:42) — Function load takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `get` (reminders/remind.py:46) — Function get takes coord and gives back nothing; it does its work on what it is given.
- function `ids` (reminders/remind.py:53) — Function ids takes pattern and gives back nothing; it does its work on what it is given.
- function `parse` (reminders/remind.py:64) — Function parse takes s and gives back nothing; it does its work on what it is given.
- function `main` (reminders/remind.py:70) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g

---

## function — detect_ext

Found in deploy/ranch-upload.py at line 48.

Signature: `def detect_ext(data):`

What it does: Return the extension for a known image type, or None.

(Where these words came from: doc comment.)

---

## function — parse_multipart

Found in deploy/ranch-upload.py at line 63.

Signature: `def parse_multipart(body, boundary):`

What it does: Minimal strict multipart/form-data parser.

(Where these words came from: doc comment.)

---

## function — log_message

Found in deploy/ranch-upload.py at line 105.

Signature: `def log_message(self, fmt, *args):`

What it does: Function log_message takes self, fmt, *args and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — do_GET

Found in deploy/ranch-upload.py at line 121.

Signature: `def do_GET(self):`

What it does: Function do_GET takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — do_POST

Found in deploy/ranch-upload.py at line 134.

Signature: `def do_POST(self):`

What it does: Function do_POST takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in deploy/ranch-upload.py at line 226.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cleanup

Found in deploy.sh at line 32.

Signature: `cleanup()`

What it does: Function cleanup takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — usage

Found in deploy.sh at line 39.

Signature: `usage()`

What it does: Function usage takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — git_with_auth

Found in deploy.sh at line 84.

Signature: `git_with_auth()`

What it does: Function git_with_auth takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — sync_source

Found in deploy.sh at line 103.

Signature: `sync_source()`

What it does: Function sync_source takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — have_cert

Found in deploy.sh at line 129.

Signature: `have_cert()`

What it does: Function have_cert takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — noindex_line

Found in deploy.sh at line 136.

Signature: `noindex_line()`

What it does: Function noindex_line takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — ssl_snippets

Found in deploy.sh at line 142.

Signature: `ssl_snippets()`

What it does: Function ssl_snippets takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — nginx_config

Found in deploy.sh at line 152.

Signature: `nginx_config()`

What it does: Function nginx_config takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_site_file

Found in deploy.sh at line 198.

Signature: `install_site_file()`

What it does: Function install_site_file takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — restore_foreign

Found in deploy.sh at line 207.

Signature: `restore_foreign()`

What it does: Function restore_foreign takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — rollback

Found in deploy.sh at line 221.

Signature: `rollback()`

What it does: Function rollback takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — reload_nginx

Found in deploy.sh at line 239.

Signature: `reload_nginx()`

What it does: Function reload_nginx takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — strip_staging_from_other_vhosts

Found in deploy.sh at line 251.

Signature: `strip_staging_from_other_vhosts()`

What it does: Function strip_staging_from_other_vhosts takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — build_certbot_cmd

Found in deploy.sh at line 293.

Signature: `build_certbot_cmd()`

What it does: Function build_certbot_cmd takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — dns_points_here

Found in deploy.sh at line 312.

Signature: `dns_points_here()`

What it does: Function dns_points_here takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — staging_cert_matches_host

Found in deploy.sh at line 329.

Signature: `staging_cert_matches_host()`

What it does: Function staging_cert_matches_host takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — obtain_cert_if_needed

Found in deploy.sh at line 341.

Signature: `obtain_cert_if_needed()`

What it does: Function obtain_cert_if_needed takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — assert_staging_noindex_config

Found in deploy.sh at line 362.

Signature: `assert_staging_noindex_config()`

What it does: Function assert_staging_noindex_config takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — assert_staging_cert_config

Found in deploy.sh at line 369.

Signature: `assert_staging_cert_config()`

What it does: Function assert_staging_cert_config takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — assert_staging_cert_presented

Found in deploy.sh at line 383.

Signature: `assert_staging_cert_presented()`

What it does: Function assert_staging_cert_presented takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — verify_http

Found in deploy.sh at line 408.

Signature: `verify_http()`

What it does: Function verify_http takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — parse_args

Found in deploy.sh at line 429.

Signature: `parse_args()`

What it does: Function parse_args takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — set_names

Found in deploy.sh at line 452.

Signature: `set_names()`

What it does: Function set_names takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in deploy.sh at line 475.

Signature: `main()`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — fetch

Found in reminders/remind.py at line 33.

Signature: `def fetch(path):`

What it does: Function fetch takes path and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — load

Found in reminders/remind.py at line 42.

Signature: `def load():`

What it does: Function load takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — get

Found in reminders/remind.py at line 46.

Signature: `def get(coord):`

What it does: Function get takes coord and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — ids

Found in reminders/remind.py at line 53.

Signature: `def ids(pattern):`

What it does: Function ids takes pattern and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — parse

Found in reminders/remind.py at line 64.

Signature: `def parse(s):`

What it does: Function parse takes s and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in reminders/remind.py at line 70.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

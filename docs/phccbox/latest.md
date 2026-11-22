# phccbox — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 92). Project root: /home/wbic16/phdoc-sweep/proj/phccbox. Scanned at commit 0f1189dfda9c6526130393f405b6733bc84c8b51. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 3 commits. The authors, by commit count:

- Will Bickford <wbic16@gmail.com> — 2 commits (66.7%)
- Theia <theia@mirrorborn.us> — 1 commit (33.3%)

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- C: 11 files, 4516 lines total, 160 blank, 373 comment, 3983 code
- C header: 12 files, 106 lines total, 7 blank, 28 comment, 71 code
- Markdown: 3 files, 1012 lines total, 88 blank, 3 comment, 921 code
- Python: 4 files, 615 lines total, 52 blank, 34 comment, 529 code
- Shell: 28 files, 2097 lines total, 117 blank, 219 comment, 1761 code

All languages: 58 files, 8346 lines total, 7265 lines of code. Functions read: 22.

Largest files:

- exo-src/applets_g.c — 723 lines
- exo-src/applets_c.c — 716 lines
- exo-src/applets_e.c — 534 lines
- exo-src/exo_util.c — 448 lines
- exo-src/applets_f.c — 447 lines

Largest functions, by span in lines:

- tmpl in exo-src/gen_literals.py — about 131 lines
- append_check in exo-test/proof.py — about 52 lines
- append_check in exo-test/ship/proof.py — about 52 lines
- port_file in exo-test/proof.py — about 37 lines
- port_file in exo-test/ship/proof.py — about 37 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 7 of 22 (32%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/phccbox.git
- Branch: exo
- Head commit: 0f1189dfda9c6526130393f405b6733bc84c8b51
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `read` (exo-src/gen_literals.py:10) — Function read takes name and gives back nothing; it does its work on what it is given.
- function `cstr` (exo-src/gen_literals.py:14) — Function cstr takes s and gives back nothing; it does its work on what it is given.
- function `emit` (exo-src/gen_literals.py:26) — items: list of (cname, python-string) -> static const char * arrays
- function `heredoc` (exo-src/gen_literals.py:34) — Return the raw text between the line containing start_marker's '<<'
- function `tmpl` (exo-src/gen_literals.py:57) — Convert shell ${VAR} / $VAR expansions into %%VAR%% tokens.
- function `note` (exo-test/proof.py:13) — Function note takes s and gives back nothing; it does its work on what it is given.
- function `extract_heredoc` (exo-test/proof.py:15) — Find the heredoc whose command line contains `needle`;
- function `render_bash` (exo-test/proof.py:32) — Render the body the way bash would inside the original heredoc.
- function `read_c_template` (exo-test/proof.py:40) — Function read_c_template takes header, varname and gives back nothing; it does its work on what it is given.
- function `expand` (exo-test/proof.py:47) — Function expand takes tmpl, vals and gives back nothing; it does its work on what it is given.
- function `cmp` (exo-test/proof.py:52) — Function cmp takes name, a, b and gives back nothing; it does its work on what it is given.
- function `port_file` (exo-test/proof.py:61) — Function port_file takes case, rel and gives back nothing; it does its work on what it is given.
- function `append_check` (exo-test/proof.py:98) — Function append_check takes name, script, needle, env, case, rel, seed and gives back nothing; it does its work on what 
- function `note` (exo-test/ship/proof.py:13) — Function note takes s and gives back nothing; it does its work on what it is given.
- function `extract_heredoc` (exo-test/ship/proof.py:15) — Find the heredoc whose command line contains `needle`;
- function `render_bash` (exo-test/ship/proof.py:32) — Render the body the way bash would inside the original heredoc.
- function `read_c_template` (exo-test/ship/proof.py:40) — Function read_c_template takes header, varname and gives back nothing; it does its work on what it is given.
- function `expand` (exo-test/ship/proof.py:47) — Function expand takes tmpl, vals and gives back nothing; it does its work on what it is given.
- function `cmp` (exo-test/ship/proof.py:52) — Function cmp takes name, a, b and gives back nothing; it does its work on what it is given.
- function `port_file` (exo-test/ship/proof.py:61) — Function port_file takes case, rel and gives back nothing; it does its work on what it is given.
- function `append_check` (exo-test/ship/proof.py:98) — Function append_check takes name, script, needle, env, case, rel, seed and gives back nothing; it does its work on what 
- function `put` (scripts/classify.py:100) — Function put takes name, cls, src and gives back nothing; it does its work on what it is given.

---

## function — read

Found in exo-src/gen_literals.py at line 10.

Signature: `def read(name):`

What it does: Function read takes name and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cstr

Found in exo-src/gen_literals.py at line 14.

Signature: `def cstr(s):`

What it does: Function cstr takes s and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — emit

Found in exo-src/gen_literals.py at line 26.

Signature: `def emit(name, items):`

What it does: items: list of (cname, python-string) -> static const char * arrays

(Where these words came from: doc comment.)

---

## function — heredoc

Found in exo-src/gen_literals.py at line 34.

Signature: `def heredoc(text, start_marker, end_marker="EOF"):`

What it does: Return the raw text between the line containing start_marker's '<<'

(Where these words came from: doc comment.)

---

## function — tmpl

Found in exo-src/gen_literals.py at line 57.

Signature: `def tmpl(s):`

What it does: Convert shell ${VAR} / $VAR expansions into %%VAR%% tokens.

(Where these words came from: doc comment.)

---

## function — note

Found in exo-test/proof.py at line 13.

Signature: `def note(s): out.append(s); print(s, flush=True)`

What it does: Function note takes s and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — extract_heredoc

Found in exo-test/proof.py at line 15.

Signature: `def extract_heredoc(script, needle):`

What it does: Find the heredoc whose command line contains `needle`;

(Where these words came from: doc comment.)

---

## function — render_bash

Found in exo-test/proof.py at line 32.

Signature: `def render_bash(body, env, quoted):`

What it does: Render the body the way bash would inside the original heredoc.

(Where these words came from: doc comment.)

---

## function — read_c_template

Found in exo-test/proof.py at line 40.

Signature: `def read_c_template(header, varname):`

What it does: Function read_c_template takes header, varname and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — expand

Found in exo-test/proof.py at line 47.

Signature: `def expand(tmpl, vals):`

What it does: Function expand takes tmpl, vals and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cmp

Found in exo-test/proof.py at line 52.

Signature: `def cmp(name, a, b):`

What it does: Function cmp takes name, a, b and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — port_file

Found in exo-test/proof.py at line 61.

Signature: `def port_file(case, rel):`

What it does: Function port_file takes case, rel and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — append_check

Found in exo-test/proof.py at line 98.

Signature: `def append_check(name, script, needle, env, case, rel, seed):`

What it does: Function append_check takes name, script, needle, env, case, rel, seed and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — note

Found in exo-test/ship/proof.py at line 13.

Signature: `def note(s): out.append(s); print(s, flush=True)`

What it does: Function note takes s and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — extract_heredoc

Found in exo-test/ship/proof.py at line 15.

Signature: `def extract_heredoc(script, needle):`

What it does: Find the heredoc whose command line contains `needle`;

(Where these words came from: doc comment.)

---

## function — render_bash

Found in exo-test/ship/proof.py at line 32.

Signature: `def render_bash(body, env, quoted):`

What it does: Render the body the way bash would inside the original heredoc.

(Where these words came from: doc comment.)

---

## function — read_c_template

Found in exo-test/ship/proof.py at line 40.

Signature: `def read_c_template(header, varname):`

What it does: Function read_c_template takes header, varname and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — expand

Found in exo-test/ship/proof.py at line 47.

Signature: `def expand(tmpl, vals):`

What it does: Function expand takes tmpl, vals and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cmp

Found in exo-test/ship/proof.py at line 52.

Signature: `def cmp(name, a, b):`

What it does: Function cmp takes name, a, b and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — port_file

Found in exo-test/ship/proof.py at line 61.

Signature: `def port_file(case, rel):`

What it does: Function port_file takes case, rel and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — append_check

Found in exo-test/ship/proof.py at line 98.

Signature: `def append_check(name, script, needle, env, case, rel, seed):`

What it does: Function append_check takes name, script, needle, env, case, rel, seed and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — put

Found in scripts/classify.py at line 100.

Signature: `def put(name, cls, src):`

What it does: Function put takes name, cls, src and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

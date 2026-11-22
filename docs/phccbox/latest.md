# phccbox — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 149). Scanned at commit 0f1189dfda9c6526130393f405b6733bc84c8b51. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 3 commits.

The author of record is **Will Bickford** — 3 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 2 commits
- Theia — 1 commit

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

- Origin: https://github.com/wbic16/phccbox
- Branch: exo
- Head commit: 0f1189dfda9c6526130393f405b6733bc84c8b51
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `read` (exo-src/gen_literals.py:10) — Function read takes name.
- function `cstr` (exo-src/gen_literals.py:14) — Function cstr takes s.
- function `emit` (exo-src/gen_literals.py:26) — items: list of (cname, python-string) -> static const char * arrays
- function `heredoc` (exo-src/gen_literals.py:34) — Return the raw text between the line containing start_marker's '<<'
- function `tmpl` (exo-src/gen_literals.py:57) — Convert shell ${VAR} / $VAR expansions into %%VAR%% tokens.
- function `note` (exo-test/proof.py:13) — Function note takes s.
- function `extract_heredoc` (exo-test/proof.py:15) — Find the heredoc whose command line contains `needle`;
- function `render_bash` (exo-test/proof.py:32) — Render the body the way bash would inside the original heredoc.
- function `read_c_template` (exo-test/proof.py:40) — Function read_c_template takes header, varname.
- function `expand` (exo-test/proof.py:47) — Function expand takes tmpl, vals.
- function `cmp` (exo-test/proof.py:52) — Function cmp takes name, a, b.
- function `port_file` (exo-test/proof.py:61) — Function port_file takes case, rel.
- function `append_check` (exo-test/proof.py:98) — Function append_check takes name, script, needle, env, case, rel, seed.
- function `note` (exo-test/ship/proof.py:13) — Function note takes s.
- function `extract_heredoc` (exo-test/ship/proof.py:15) — Find the heredoc whose command line contains `needle`;
- function `render_bash` (exo-test/ship/proof.py:32) — Render the body the way bash would inside the original heredoc.
- function `read_c_template` (exo-test/ship/proof.py:40) — Function read_c_template takes header, varname.
- function `expand` (exo-test/ship/proof.py:47) — Function expand takes tmpl, vals.
- function `cmp` (exo-test/ship/proof.py:52) — Function cmp takes name, a, b.
- function `port_file` (exo-test/ship/proof.py:61) — Function port_file takes case, rel.
- function `append_check` (exo-test/ship/proof.py:98) — Function append_check takes name, script, needle, env, case, rel, seed.
- function `put` (scripts/classify.py:100) — Function put takes name, cls, src.

---

## function — read

Found in exo-src/gen_literals.py at line 10.

Surface: public API.

Signature: `def read(name)`

What it does: Function read takes name. Its return type is unannotated; the code shows it returning f.read().

(Where these words came from: inferred.)

---

## function — cstr

Found in exo-src/gen_literals.py at line 14.

Surface: public API.

Signature: `def cstr(s)`

What it does: Function cstr takes s. Its return type is unannotated; the code shows it returning '"' + ''.join(out) + '"'.

(Where these words came from: inferred.)

---

## function — emit

Found in exo-src/gen_literals.py at line 26.

Surface: public API.

Signature: `def emit(name, items)`

What it does: items: list of (cname, python-string) -> static const char * arrays

(Where these words came from: direct.)

---

## function — heredoc

Found in exo-src/gen_literals.py at line 34.

Surface: public API.

Signature: `def heredoc(text, start_marker, end_marker="EOF")`

What it does: Return the raw text between the line containing start_marker's '<<'

(Where these words came from: direct.)

---

## function — tmpl

Found in exo-src/gen_literals.py at line 57.

Surface: public API.

Signature: `def tmpl(s)`

What it does: Convert shell ${VAR} / $VAR expansions into %%VAR%% tokens.

(Where these words came from: direct.)

---

## function — note

Found in exo-test/proof.py at line 13.

Surface: public API.

Signature: `def note(s): out.append(s); print(s, flush=True)`

What it does: Function note takes s. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — extract_heredoc

Found in exo-test/proof.py at line 15.

Surface: public API.

Signature: `def extract_heredoc(script, needle)`

What it does: Find the heredoc whose command line contains `needle`;

(Where these words came from: direct.)

---

## function — render_bash

Found in exo-test/proof.py at line 32.

Surface: public API.

Signature: `def render_bash(body, env, quoted)`

What it does: Render the body the way bash would inside the original heredoc.

(Where these words came from: direct.)

---

## function — read_c_template

Found in exo-test/proof.py at line 40.

Surface: public API.

Signature: `def read_c_template(header, varname)`

What it does: Function read_c_template takes header, varname. Its return type is unannotated; the code shows it returning s.encode().decode("unicode_escape").

(Where these words came from: inferred.)

---

## function — expand

Found in exo-test/proof.py at line 47.

Surface: public API.

Signature: `def expand(tmpl, vals)`

What it does: Function expand takes tmpl, vals. Its return type is unannotated; the code shows it returning tmpl.

(Where these words came from: inferred.)

---

## function — cmp

Found in exo-test/proof.py at line 52.

Surface: public API.

Signature: `def cmp(name, a, b)`

What it does: Function cmp takes name, a, b. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — port_file

Found in exo-test/proof.py at line 61.

Surface: public API.

Signature: `def port_file(case, rel)`

What it does: Function port_file takes case, rel. Its return type is unannotated; the code shows it returning open(p).read() if os.path.exists(p) else None.

(Where these words came from: inferred.)

---

## function — append_check

Found in exo-test/proof.py at line 98.

Surface: public API.

Signature: `def append_check(name, script, needle, env, case, rel, seed)`

What it does: Function append_check takes name, script, needle, env, case, rel, seed. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — note

Found in exo-test/ship/proof.py at line 13.

Surface: public API.

Signature: `def note(s): out.append(s); print(s, flush=True)`

What it does: Function note takes s. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — extract_heredoc

Found in exo-test/ship/proof.py at line 15.

Surface: public API.

Signature: `def extract_heredoc(script, needle)`

What it does: Find the heredoc whose command line contains `needle`;

(Where these words came from: direct.)

---

## function — render_bash

Found in exo-test/ship/proof.py at line 32.

Surface: public API.

Signature: `def render_bash(body, env, quoted)`

What it does: Render the body the way bash would inside the original heredoc.

(Where these words came from: direct.)

---

## function — read_c_template

Found in exo-test/ship/proof.py at line 40.

Surface: public API.

Signature: `def read_c_template(header, varname)`

What it does: Function read_c_template takes header, varname. Its return type is unannotated; the code shows it returning s.encode().decode("unicode_escape").

(Where these words came from: inferred.)

---

## function — expand

Found in exo-test/ship/proof.py at line 47.

Surface: public API.

Signature: `def expand(tmpl, vals)`

What it does: Function expand takes tmpl, vals. Its return type is unannotated; the code shows it returning tmpl.

(Where these words came from: inferred.)

---

## function — cmp

Found in exo-test/ship/proof.py at line 52.

Surface: public API.

Signature: `def cmp(name, a, b)`

What it does: Function cmp takes name, a, b. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — port_file

Found in exo-test/ship/proof.py at line 61.

Surface: public API.

Signature: `def port_file(case, rel)`

What it does: Function port_file takes case, rel. Its return type is unannotated; the code shows it returning open(p).read() if os.path.exists(p) else None.

(Where these words came from: inferred.)

---

## function — append_check

Found in exo-test/ship/proof.py at line 98.

Surface: public API.

Signature: `def append_check(name, script, needle, env, case, rel, seed)`

What it does: Function append_check takes name, script, needle, env, case, rel, seed. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — put

Found in scripts/classify.py at line 100.

Surface: public API.

Signature: `def put(name, cls, src)`

What it does: Function put takes name, cls, src. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/phccbox/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

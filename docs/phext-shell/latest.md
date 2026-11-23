# phext-shell — project documentation

Scanned by phdoc 0.1.0 on 2026-11-23 (Exocortical Epoch 194). Project root: /home/wbic16/phdoc-sweep/proj/phext-shell. Scanned at commit 4aae9abc2611110ba9d20392586cb6ddd78bfa43. Package version, from its manifest: 0.6.1. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 1 (1 default, 0 optional).

- libphext [dependencies] — 0.6.0


---

## Authors

The git log holds 35 commits. phdoc audits the 29 commits on the first-parent mainline — each merge folds its branch's commits into the merge commit — and the author counts below cover that audited mainline, the same set the historical references fold.

The author of record is **Will Bickford** — 29 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 27 commits
- Orin — 1 commit
- Theia — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 1 files, 59 lines total, 12 blank, 6 comment, 41 code
- Rust: 1 files, 388 lines total, 41 blank, 28 comment, 319 code
- TOML: 1 files, 11 lines total, 1 blank, 0 comment, 10 code

All languages: 3 files, 458 lines total, 370 lines of code. Functions read: 3.

Largest files:

- src/main.rs — 388 lines
- readme.md — 59 lines
- Cargo.toml — 11 lines

Largest functions, by span in lines:

- handle_request in src/main.rs — about 189 lines
- show_help in src/main.rs — about 132 lines
- main in src/main.rs — about 50 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files in test directories or whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 1. FIXME marks: 0.
- Public items with a doc comment: 0 of 16 (0%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/phext-shell.git
- Branch: main
- Head commit: 4aae9abc2611110ba9d20392586cb6ddd78bfa43
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- flag `--------` (src/main.rs:381) — A command line flag, --------.
- flag `------------------` (src/main.rs:367) — A command line flag, ------------------.
- command `af` (src/main.rs:86) — A command named af.
- command `cs` (src/main.rs:105) — A command named cs.
- command `ds` (src/main.rs:122) — A command named ds.
- command `exit` (src/main.rs:83) — A command named exit.
- command `help` (src/main.rs:228) — A command named help.
- command `lp` (src/main.rs:164) — A command named lp.
- entry point `main` (src/main.rs:18) — The entry point: the program starts here.
- command `os` (src/main.rs:190) — A command named os.
- command `ph` (src/main.rs:150) — A command named ph.
- command `pi` (src/main.rs:128) — A command named pi.
- command `ps` (src/main.rs:139) — A command named ps.
- command `rp` (src/main.rs:201) — A command named rp.
- command `rs` (src/main.rs:208) — A command named rs.
- command `sp` (src/main.rs:215) — A command named sp.

---

## flag — --------

Found in src/main.rs at line 381.

Signature: `flag --------`

What it does: A command line flag, --------. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## flag — ------------------

Found in src/main.rs at line 367.

Signature: `flag ------------------`

What it does: A command line flag, ------------------. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## command — af

Found in src/main.rs at line 86.

Signature: `command "af"`

What it does: A command named af. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — cs

Found in src/main.rs at line 105.

Signature: `command "cs"`

What it does: A command named cs. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — ds

Found in src/main.rs at line 122.

Signature: `command "ds"`

What it does: A command named ds. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — exit

Found in src/main.rs at line 83.

Signature: `command "exit"`

What it does: A command named exit. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — help

Found in src/main.rs at line 228.

Signature: `command "help"`

What it does: A command named help. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — lp

Found in src/main.rs at line 164.

Signature: `command "lp"`

What it does: A command named lp. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## entry point — main

Found in src/main.rs at line 18.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: signature.)

---

## command — os

Found in src/main.rs at line 190.

Signature: `command "os"`

What it does: A command named os. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — ph

Found in src/main.rs at line 150.

Signature: `command "ph"`

What it does: A command named ph. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — pi

Found in src/main.rs at line 128.

Signature: `command "pi"`

What it does: A command named pi. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — ps

Found in src/main.rs at line 139.

Signature: `command "ps"`

What it does: A command named ps. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — rp

Found in src/main.rs at line 201.

Signature: `command "rp"`

What it does: A command named rp. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — rs

Found in src/main.rs at line 208.

Signature: `command "rs"`

What it does: A command named rs. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — sp

Found in src/main.rs at line 215.

Signature: `command "sp"`

What it does: A command named sp. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/phext-shell/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

# phdoc — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 95). Project root: /home/wbic16/phdoc-selfaudit. Scanned at commit 9e266539f56016d7d37fe92825a68f113409b03e. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 1.

- libphext [dependencies] — 0.6.6

Total packages in the lock file, this one included: 3.


---

## Authors

The git log holds 8 commits.

The author of record is **Will Bickford** — 8 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Theia — 5 commits
- Emise — 2 commits
- Will Bickford (his own identities) — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 44 files, 5346 lines total, 1790 blank, 220 comment, 3336 code
- Rust: 1 files, 2638 lines total, 128 blank, 138 comment, 2372 code
- TOML: 1 files, 22 lines total, 2 blank, 0 comment, 20 code

All languages: 46 files, 8006 lines total, 5728 lines of code. Functions read: 69.

Largest files:

- src/main.rs — 2638 lines
- docs/rfcbox/latest.md — 1515 lines
- docs/phdoc/latest.md — 827 lines
- docs/kgoat/latest.md — 521 lines
- README.md — 275 lines

Largest functions, by span in lines:

- cmd_audit in src/main.rs — about 254 lines
- audit_commit in src/main.rs — about 232 lines
- main in src/main.rs — about 200 lines
- scan_project in src/main.rs — about 186 lines
- sections in src/main.rs — about 174 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 13 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 6. FIXME marks: 6.
- Public items with a doc comment: 0 of 56 (0%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: git@github.com:wbic16/phdoc.git
- Branch: exo
- Head commit: 9e266539f56016d7d37fe92825a68f113409b03e
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- entry point `main` (src/main.rs:1164) — The entry point: the program starts here.
- command `rs` (src/main.rs:165) — A command named rs.
- command `py` (src/main.rs:166) — A command named py.
- command `js` (src/main.rs:167) — A command named js.
- command `ts` (src/main.rs:168) — A command named ts.
- command `go` (src/main.rs:169) — A command named go.
- command `c` (src/main.rs:170) — A command named c.
- command `h` (src/main.rs:171) — A command named h.
- command `cpp` (src/main.rs:172) — A command named cpp.
- command `java` (src/main.rs:173) — A command named java.
- command `rb` (src/main.rs:174) — A command named rb.
- command `sh` (src/main.rs:175) — A command named sh.
- command `md` (src/main.rs:176) — A command named md.
- command `toml` (src/main.rs:177) — A command named toml.
- command `word` (src/main.rs:605) — A command named word.
- flag `--word` (src/main.rs:629) — A command line flag, --word.
- command `function` (src/main.rs:674) — A command named function.
- command `struct` (src/main.rs:698) — A command named struct.
- command `enum` (src/main.rs:705) — A command named enum.
- command `trait` (src/main.rs:712) — A command named trait.
- command `const` (src/main.rs:713) — A command named const.
- command `type` (src/main.rs:720) — A command named type.
- command `re-export` (src/main.rs:721) — A command named re-export.
- command `command` (src/main.rs:723) — A command named command.
- command `flag` (src/main.rs:724) — A command named flag.
- command `awaiting` (src/main.rs:725) — A command named awaiting.
- flag `--show-current` (src/main.rs:746) — A command line flag, --show-current.
- flag `--porcelain` (src/main.rs:748) — A command line flag, --porcelain.
- flag `--format=%an <%ae>` (src/main.rs:751) — A command line flag, --format=%an <%ae>.
- command `help` (src/main.rs:1168) — A command named help.
- flag `--help` (src/main.rs:1168) — A command line flag, --help.
- command `version` (src/main.rs:1169) — A command named version.
- flag `--version` (src/main.rs:1169) — A command line flag, --version.
- command `scan` (src/main.rs:1170) — A command named scan.
- command `--out` (src/main.rs:1178) — A command named --out.
- command `--name` (src/main.rs:1182) — A command named --name.
- command `--summaries` (src/main.rs:1186) — A command named --summaries.
- command `audit` (src/main.rs:1245) — A command named audit.
- command `--repo` (src/main.rs:1250) — A command named --repo.
- command `--hold-push` (src/main.rs:1262) — A command named --hold-push.
- command `review` (src/main.rs:1277) — A command named review.
- command `--by` (src/main.rs:1284) — A command named --by.
- command `render` (src/main.rs:1306) — A command named render.
- command `--at` (src/main.rs:1314) — A command named --at.
- flag `--format=%an <%ae>%n%at%n%s` (src/main.rs:1443) — A command line flag, --format=%an <%ae>%n%at%n%s.
- flag `--no-commit-id` (src/main.rs:1452) — A command line flag, --no-commit-id.
- flag `--name-status` (src/main.rs:1452) — A command line flag, --name-status.
- flag `--root` (src/main.rs:1454) — A command line flag, --root.
- flag `--name-only` (src/main.rs:1579) — A command line flag, --name-only.
- flag `--numstat` (src/main.rs:1598) — A command line flag, --numstat.
- flag `--short` (src/main.rs:2145) — A command line flag, --short.
- flag `--verify` (src/main.rs:2150) — A command line flag, --verify.
- flag `--reverse` (src/main.rs:2162) — A command line flag, --reverse.
- flag `--count` (src/main.rs:2309) — A command line flag, --count.
- command `docs` (src/main.rs:2378) — A command named docs.
- flag `--bare` (src/main.rs:2587) — A command line flag, --bare.

---

## entry point — main

Found in src/main.rs at line 1164.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: signature.)

---

## command — rs

Found in src/main.rs at line 165.

Signature: `command "rs"`

What it does: A command named rs. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — py

Found in src/main.rs at line 166.

Signature: `command "py"`

What it does: A command named py. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — js

Found in src/main.rs at line 167.

Signature: `command "js"`

What it does: A command named js. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — ts

Found in src/main.rs at line 168.

Signature: `command "ts"`

What it does: A command named ts. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — go

Found in src/main.rs at line 169.

Signature: `command "go"`

What it does: A command named go. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — c

Found in src/main.rs at line 170.

Signature: `command "c"`

What it does: A command named c. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — h

Found in src/main.rs at line 171.

Signature: `command "h"`

What it does: A command named h. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — cpp

Found in src/main.rs at line 172.

Signature: `command "cpp"`

What it does: A command named cpp. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — java

Found in src/main.rs at line 173.

Signature: `command "java"`

What it does: A command named java. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — rb

Found in src/main.rs at line 174.

Signature: `command "rb"`

What it does: A command named rb. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — sh

Found in src/main.rs at line 175.

Signature: `command "sh"`

What it does: A command named sh. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — md

Found in src/main.rs at line 176.

Signature: `command "md"`

What it does: A command named md. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — toml

Found in src/main.rs at line 177.

Signature: `command "toml"`

What it does: A command named toml. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — word

Found in src/main.rs at line 605.

Signature: `command "word"`

What it does: A command named word. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## flag — --word

Found in src/main.rs at line 629.

Signature: `flag --word`

What it does: A command line flag, --word. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## command — function

Found in src/main.rs at line 674.

Signature: `command "function"`

What it does: A command named function. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — struct

Found in src/main.rs at line 698.

Signature: `command "struct"`

What it does: A command named struct. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — enum

Found in src/main.rs at line 705.

Signature: `command "enum"`

What it does: A command named enum. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — trait

Found in src/main.rs at line 712.

Signature: `command "trait"`

What it does: A command named trait. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — const

Found in src/main.rs at line 713.

Signature: `command "const"`

What it does: A command named const. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — type

Found in src/main.rs at line 720.

Signature: `command "type"`

What it does: A command named type. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — re-export

Found in src/main.rs at line 721.

Signature: `command "re-export"`

What it does: A command named re-export. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — command

Found in src/main.rs at line 723.

Signature: `command "command"`

What it does: A command named command. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — flag

Found in src/main.rs at line 724.

Signature: `command "flag"`

What it does: A command named flag. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — awaiting

Found in src/main.rs at line 725.

Signature: `command "awaiting"`

What it does: A command named awaiting. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## flag — --show-current

Found in src/main.rs at line 746.

Signature: `flag --show-current`

What it does: A command line flag, --show-current. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## flag — --porcelain

Found in src/main.rs at line 748.

Signature: `flag --porcelain`

What it does: A command line flag, --porcelain. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## flag — --format=%an <%ae>

Found in src/main.rs at line 751.

Signature: `flag --format=%an <%ae>`

What it does: A command line flag, --format=%an <%ae>. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## command — help

Found in src/main.rs at line 1168.

Signature: `command "help"`

What it does: A command named help. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## flag — --help

Found in src/main.rs at line 1168.

Signature: `flag --help`

What it does: A command line flag, --help. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## command — version

Found in src/main.rs at line 1169.

Signature: `command "version"`

What it does: A command named version. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## flag — --version

Found in src/main.rs at line 1169.

Signature: `flag --version`

What it does: A command line flag, --version. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## command — scan

Found in src/main.rs at line 1170.

Signature: `command "scan"`

What it does: A command named scan. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — --out

Found in src/main.rs at line 1178.

Signature: `command "--out"`

What it does: A command named --out. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — --name

Found in src/main.rs at line 1182.

Signature: `command "--name"`

What it does: A command named --name. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — --summaries

Found in src/main.rs at line 1186.

Signature: `command "--summaries"`

What it does: A command named --summaries. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — audit

Found in src/main.rs at line 1245.

Signature: `command "audit"`

What it does: A command named audit. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — --repo

Found in src/main.rs at line 1250.

Signature: `command "--repo"`

What it does: A command named --repo. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — --hold-push

Found in src/main.rs at line 1262.

Signature: `command "--hold-push"`

What it does: A command named --hold-push. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — review

Found in src/main.rs at line 1277.

Signature: `command "review"`

What it does: A command named review. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — --by

Found in src/main.rs at line 1284.

Signature: `command "--by"`

What it does: A command named --by. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — render

Found in src/main.rs at line 1306.

Signature: `command "render"`

What it does: A command named render. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — --at

Found in src/main.rs at line 1314.

Signature: `command "--at"`

What it does: A command named --at. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## flag — --format=%an <%ae>%n%at%n%s

Found in src/main.rs at line 1443.

Signature: `flag --format=%an <%ae>%n%at%n%s`

What it does: A command line flag, --format=%an <%ae>%n%at%n%s. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## flag — --no-commit-id

Found in src/main.rs at line 1452.

Signature: `flag --no-commit-id`

What it does: A command line flag, --no-commit-id. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## flag — --name-status

Found in src/main.rs at line 1452.

Signature: `flag --name-status`

What it does: A command line flag, --name-status. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## flag — --root

Found in src/main.rs at line 1454.

Signature: `flag --root`

What it does: A command line flag, --root. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## flag — --name-only

Found in src/main.rs at line 1579.

Signature: `flag --name-only`

What it does: A command line flag, --name-only. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## flag — --numstat

Found in src/main.rs at line 1598.

Signature: `flag --numstat`

What it does: A command line flag, --numstat. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## flag — --short

Found in src/main.rs at line 2145.

Signature: `flag --short`

What it does: A command line flag, --short. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## flag — --verify

Found in src/main.rs at line 2150.

Signature: `flag --verify`

What it does: A command line flag, --verify. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## flag — --reverse

Found in src/main.rs at line 2162.

Signature: `flag --reverse`

What it does: A command line flag, --reverse. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## flag — --count

Found in src/main.rs at line 2309.

Signature: `flag --count`

What it does: A command line flag, --count. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## command — docs

Found in src/main.rs at line 2378.

Signature: `command "docs"`

What it does: A command named docs. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## flag — --bare

Found in src/main.rs at line 2587.

Signature: `flag --bare`

What it does: A command line flag, --bare. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/phdoc/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

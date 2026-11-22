# phdoc — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 149). Scanned at commit 0c4502c5cda927c5195b4f4f97715c0b696c8358. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

Description stated in the manifest: Project scanner for shipped artifacts: inventories dependencies, authors, complexity, code quality, declared RFCs, and the upstream repo, and keeps plain-English documentation in a multiversal phext chain, one universe per audited commit.

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

The git log holds 116 commits.

The author of record is **Will Bickford** — 116 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Theia — 87 commits
- Verse — 26 commits
- Emise — 2 commits
- Will Bickford (his own identities) — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 7860 files, 1235700 lines total, 126740 blank, 4230 comment, 1104730 code
- Rust: 1 file, 2805 lines total, 133 blank, 165 comment, 2507 code
- TOML: 1 file, 22 lines total, 2 blank, 0 comment, 20 code

All languages: 7862 files, 1238527 lines total, 1107257 lines of code. Functions read: 74.

Largest files:

- docs/emise-tools/latest.md — 8439 lines
- docs/phext-lattice/latest.md — 7623 lines
- docs/qexo/latest.md — 4317 lines
- docs/register/latest.md — 3683 lines
- docs/phext-lanes/latest.md — 3620 lines

Largest functions, by span in lines:

- cmd_audit in src/main.rs — about 255 lines
- audit_commit in src/main.rs — about 232 lines
- main in src/main.rs — about 210 lines
- scan_project in src/main.rs — about 186 lines
- sections in src/main.rs — about 171 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 15 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 39. FIXME marks: 39.
- Public items with a doc comment: 0 of 17 (0%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/phdoc
- Branch: exo
- Head commit: 0c4502c5cda927c5195b4f4f97715c0b696c8358
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- entry point `main` (src/main.rs:1161) — The entry point: the program starts here.
- command `help` (src/main.rs:1165) — A command named help.
- command `version` (src/main.rs:1166) — A command named version.
- command `scan` (src/main.rs:1167) — A command named scan.
- command `audit` (src/main.rs:1242) — A command named audit.
- command `review` (src/main.rs:1274) — A command named review.
- command `render` (src/main.rs:1303) — A command named render.
- flag `--at` (src/main.rs:1311) — A command line flag, --at (used by: render).
- flag `--by` (src/main.rs:1281) — A command line flag, --by (used by: review).
- flag `--help` (src/main.rs:1165) — A command line flag, --help (used by: help).
- flag `--hold-push` (src/main.rs:1259) — A command line flag, --hold-push (used by: audit).
- flag `--name` (src/main.rs:1179) — A command line flag, --name (used by: scan, audit).
- flag `--out` (src/main.rs:1175) — A command line flag, --out (used by: scan, audit, render).
- flag `--repo` (src/main.rs:1247) — A command line flag, --repo (used by: audit, render).
- flag `--summaries` (src/main.rs:1183) — A command line flag, --summaries (used by: scan).
- flag `--version` (src/main.rs:1166) — A command line flag, --version (used by: version).
- flag `-h` (src/main.rs:1165) — A command line flag, -h (used by: help).

---

## entry point — main

Found in src/main.rs at line 1161.

Surface: public API.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## command — help

Found in src/main.rs at line 1165.

Surface: public API.

Signature: `command "help"`

What it does: A command named help. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — version

Found in src/main.rs at line 1166.

Surface: public API.

Signature: `command "version"`

What it does: A command named version. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — scan

Found in src/main.rs at line 1167.

Surface: public API.

Signature: `command "scan"`

What it does: A command named scan. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — audit

Found in src/main.rs at line 1242.

Surface: public API.

Signature: `command "audit"`

What it does: A command named audit. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — review

Found in src/main.rs at line 1274.

Surface: public API.

Signature: `command "review"`

What it does: A command named review. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — render

Found in src/main.rs at line 1303.

Surface: public API.

Signature: `command "render"`

What it does: A command named render. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## flag — --at

Found in src/main.rs at line 1311.

Surface: public API.

Signature: `flag --at`

What it does: A command line flag, --at (used by: render). It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## flag — --by

Found in src/main.rs at line 1281.

Surface: public API.

Signature: `flag --by`

What it does: A command line flag, --by (used by: review). It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## flag — --help

Found in src/main.rs at line 1165.

Surface: public API.

Signature: `flag --help`

What it does: A command line flag, --help (used by: help). It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## flag — --hold-push

Found in src/main.rs at line 1259.

Surface: public API.

Signature: `flag --hold-push`

What it does: A command line flag, --hold-push (used by: audit). It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## flag — --name

Found in src/main.rs at line 1179.

Surface: public API.

Signature: `flag --name`

What it does: A command line flag, --name (used by: scan, audit). It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## flag — --out

Found in src/main.rs at line 1175.

Surface: public API.

Signature: `flag --out`

What it does: A command line flag, --out (used by: scan, audit, render). It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## flag — --repo

Found in src/main.rs at line 1247.

Surface: public API.

Signature: `flag --repo`

What it does: A command line flag, --repo (used by: audit, render). It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## flag — --summaries

Found in src/main.rs at line 1183.

Surface: public API.

Signature: `flag --summaries`

What it does: A command line flag, --summaries (used by: scan). It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## flag — --version

Found in src/main.rs at line 1166.

Surface: public API.

Signature: `flag --version`

What it does: A command line flag, --version (used by: version). It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## flag — -h

Found in src/main.rs at line 1165.

Surface: public API.

Signature: `flag -h`

What it does: A command line flag, -h (used by: help). It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/phdoc/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

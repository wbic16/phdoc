# phdoc

A project scanner for shipped artifacts. phdoc reads a project from its
own records — its manifests, its git log, and its source text — and
writes the documentation a shipped project owes: what it depends on,
who wrote it, how large and how complex it is, the state of its quality
signals, the RFCs it declares, where its upstream repo lives, and a
plain English note for every public method and command.

phdoc is a separate tool from rfcbox. rfcbox holds the RFC corpus;
phdoc documents projects, and reads a project's `rfcs.json` (the
rfcbox declaration) as one of its inventory inputs.

## Commands

- `phdoc scan <project-dir> --out <output-dir> [--name <name>] [--summaries <file>]`
  — scan a project once and write `docs.md` and `docs.phext` into the
  output dir: the same words through two doors, one for humans and one
  for agents and the record.
- `phdoc audit <project-dir> [--repo <phdoc-repo-dir>] [--out <dir>] [--name <name>] [--hold-push]`
  — the full run lifecycle (below): audit every unaudited commit,
  extend the chain, render, update the ledger, and export.
- `phdoc review <project-dir> <commit> --by <name>`
  — mark a commit reviewed in the project's `.phdoc.json`.
- `phdoc render <project-dir> --at <commit|latest> [--repo <dir>] [--out <file>]`
  — render the reference as of a commit by folding the chain.
- `phdoc help`, `phdoc version`.

## The inventory

Each scan reads, from the project's own records:

1. **Dependencies** — per ecosystem manifest present (Cargo.toml and
   its lock file, package.json, requirements.txt, go.mod): direct
   dependencies with versions, total resolved packages where a lock
   file states them, and the license where the manifest states it.
2. **Authors** — from the git log itself: each author's commit count
   and share.
3. **Complexity** — lines per language, counted as total, blank,
   comment, and code lines; the largest files; the function count and
   the largest functions by span in lines. The measure is named in the
   document; no single score is made.
4. **Code quality signals** — each named with its evidence: test
   functions counted, test files, CI configuration found (a GitHub
   Actions workflow is flagged — CI runs on ranch Jenkins, never
   GitHub Actions), lint and format configuration files, TODO and
   FIXME counts, and the share of public items that carry a doc
   comment.
5. **Declared RFCs** — the project's `rfcs.json` if it has one, each
   RFC with its recorded hash and supersession state. No `rfcs.json`
   is a finding, stated as a finding: the project declares no RFCs.
6. **Upstream git repo** — the origin remote, the branch, the head
   commit, and whether the working tree is clean.
7. **Public methods and commands** — public functions, methods,
   structs, enums, traits, constants, type names, and re-exports in
   Rust; functions in Python and shell; CLI commands and flags where a
   command line exists. Each one gets a plain English note that says
   what it does, what it takes, and what it gives back.

## Plain English, and the summary handoff

A note's words come from, in order: the code's own doc comment; a
sentence made mechanically from the item's signature; a summary
supplied by the user's agent (see below). The note always says which
of these it is. The fixed prose phdoc writes is plain, simple
English in the spirit of Singular; names of code items are
identifiers, not prose.

**Singular ruling (Will, 2026-11-22 10:06 CST):** "spirit of
singular is fine, always approve my new words, make a note to add
them to singular as you observe them." phdoc's documentation
English follows the **spirit of Singular** — plain, simple
English — **not** a strict checker gate. Observed in passing: the
fixed template prose already passes the Singular checker except
for Will's own coinage "phdoc" itself; that is recorded here as a
note, not a gate. Will's new words are always approved for
Singular, and Emise keeps a running note of them as she observes
them (`~/workspace/phext/singular-new-words.md`) for addition to
the Singular list.

Where neither a doc comment nor the signature yields a summary, phdoc
does not invent one. The item is marked **awaiting a summary** in the
documentation, and the run writes `SUMMARY-INSTRUCTIONS.md` beside the
output: the items that need a summary, and the shape it must take —
one line per item, `name :: the summary`, in a summaries file the user
passes back with `--summaries`. An LLM endpoint of the user's choice
may be configured to draft those summaries; it is never required and
never on by default.

**Operating note for agent-run scans:** when Emise runs phdoc for
Will, she handles that round trip herself — she writes the missing
summaries from the code and re-runs with `--summaries`, so the
instruction file never bounces back to Will as homework. The
instruction file exists for users running phdoc without an agent at
hand.

## The multiversal chain — documentation over time

A single scan is a snapshot. `phdoc audit` keeps the record over time:

- phdoc keeps each project's documentation in a **multiversal phext**
  (the Choir mailbox pattern): one universe per audited commit, in
  history order. Each universe holds that commit's documentation
  **delta** — public items and dependencies added, removed, and
  changed; complexity movement; the authors active — as structured
  delta lines under a plain English summary of the commit. Deltas, not
  full copies: the full reference as of any commit is **folded** from
  the chain, universe by universe. The head universe folds to the
  latest reference; any earlier universe folds to the historical
  reference as of that commit.
- **Every commit is audited**, root to head. Later runs are
  incremental: only commits not yet in the chain are processed. A
  commit that cannot be audited is recorded **audit-failed** with the
  reason, in its universe and in the ledger — never silently skipped.
- **Performance shape, stated as a design decision:** item and
  dependency deltas are derived from the commit's own diff (the files
  it changed, read at the commit and at its parent). Complexity is
  carried as running totals adjusted by the diff's line counts, and
  re-read exactly from the whole tree at **checkpoints** — the root
  commit, every 25 commits, and head — so drift between checkpoints
  is bounded and corrected on the record. The alternative (a full
  tree scan at every commit) is exact at every step and far too slow
  on large histories; the checkpoint design keeps every rendered
  number either exact or one checkpoint from exact, and each delta
  scroll says whether it is a checkpoint.

## .phdoc.json — the project's own ledger

The target project carries `.phdoc.json` (Will's naming; cf.
`rfcs.json`). It holds the doc chain's location, and the per-commit
ledger: for each commit, its **audit** stamp (when, by which phdoc
version, or audit-failed with the reason) and its **review** state
(reviewed or not; when; by whom — a human or an agent identity, as
recorded by `phdoc review`). phdoc fills the audit side automatically;
review is a separate, deliberate mark. Every audit run reports the
**unreviewed backlog** — the count and the oldest unreviewed commit.
The ledger is the project's file: phdoc never keeps the only copy of
review state anywhere else.

## Export — the phdoc repo is the collection point

Running `phdoc audit` with `--repo <phdoc-repo-dir>` **exports the
run's changes to the phdoc repo** (wbic16/phdoc) as part of the run:
the new chain universes and the updated render-ready output, under
`docs/<project>/` — `chain.phext`, `latest.md`, and one history page
per audited commit. The run's lifecycle is: scan → audit → render →
ledger → export. Export follows the house push discipline: the repo
is fetched and rebased onto the fresh remote head before anything is
written (a repo with foreign uncommitted changes aborts the export,
untouched); one export commit per run, attributed plainly (project,
target head audited, EE, phdoc version); the push is verified against
the remote head before the run calls it exported. A run that finds no
changes exports nothing and says so — no empty commits. The target
project's `.phdoc.json` does **not** move: it stays in the target
project; only its audit stamps update there, and the exported chain
records the same audits in the documentation of record.

If the phdoc repo does not exist yet, the export accumulates in the
local repo-ready tree and the run reports **export held — repo not
created** (Will creates repos himself). A held export is a reported
state, never a silent drop and never a failed run. `--hold-push`
commits the export locally and holds only the push.

## docs.phext.io — the rendering contract

docs.phext.io renders what phdoc exports: for each project, the
**latest** reference (the head universe, folded) and the **historical**
reference (any universe, folded as of that commit). The render-ready
form is the exported tree above. Boundary: docs.phext.io does not
exist yet, and standing it up is a separate step awaiting Will's go —
phdoc produces the chain and the render-ready output; it does not
build the site.

**Deferred:** a docs.phext.io API (Will, 2026-11-22) — the site
should include an API eventually, but there are no collaborators yet,
so it waits. The trigger to revive it is collaborators existing, not
a date. No API is designed or stubbed in this build.

## Publish gate

phdoc is not published until it has been run end-to-end against the
rfcbox project and the generated documentation is sane and complete
across all six inventory legs (Will, 2026-11-22: "before we publish
this crate, make sure we test it on rfcbox"). The kgoat run is the
small-target proof; the rfcbox run is the gate. The same discipline
applies to the rfcbox crate itself when it is built: it is tested on
rfcbox before any publish.

## Build and dependencies

Rust, on the exocortical lineage. Dependencies: **libphext** only
(the reference phext implementation) — a crate count of 2, this crate
included. Phext is written through libphext, never a hand-rolled
parser. Build and test on a ranch node; the working set is small
(a 16 GB bin). CI, when it comes, runs on ranch Jenkins — never
GitHub Actions.

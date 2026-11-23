# phcc — reference as of commit c5ce5eaedae4

Commit c5ce5eaedae46467f6e538e4e5b25910b863de75. 278 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

No dependencies recorded at this point.

## Authors

Fork attribution: phcc is a fork of TinyCC (github.com/TinyCC/tinycc) (fork base commit c638ddfb593214813aa33536fb993b9a45d23793). Every commit folded to this point is upstream history, credited to its own authors below; the fork's own work begins after the base.

- bellard <bellard> — 278 commits

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- C: 11 files, 19323 lines total, 1607 blank, 2697 comment, 15019 code
- C header: 12 files, 2966 lines total, 416 blank, 1215 comment, 1335 code

All languages: 23 files, 22289 lines total, 16354 lines of code. Functions read: 0.

Largest files:

- tcc.c — 9971 lines
- tccelf.c — 2076 lines
- tcctest.c — 1955 lines
- elf.h — 1595 lines
- i386-asm.c — 1108 lines

Largest functions, by span in lines:


## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 1 files in test directories or whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format configuration files: Makefile.
- TODO marks in the source: 0. FIXME marks: 0.

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/phcc.git
- Branch: exo
- Head commit: c5ce5eaedae46467f6e538e4e5b25910b863de75
- Working tree: the committed tree at this commit — a historical reference has no working tree

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.


## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/phcc/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

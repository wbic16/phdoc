# phcc — reference as of commit b78d3ff1dca8

Commit b78d3ff1dca891bc2f377c5a9f71cc142808afa2. 360 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

No dependencies recorded at this point.

## Authors

Author counts over the 360 audited mainline commits folded to this point.

- bellard <bellard> — 360 commits

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- C: 29 files, 27610 lines total, 2639 blank, 3672 comment, 21299 code
- C header: 76 files, 23954 lines total, 1687 blank, 13297 comment, 8970 code
- Shell: 1 files, 33 lines total, 4 blank, 1 comment, 28 code

All languages: 106 files, 51597 lines total, 30297 lines of code. Functions read: 0.

Largest files:

- tcc.c — 10640 lines
- win32/include/winapi/winuser.h — 3472 lines
- win32/include/winapi/wingdi.h — 2843 lines
- win32/include/winapi/winnt.h — 2667 lines
- c67-gen.c — 2548 lines

Largest functions, by span in lines:


## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 1 files in test directories or whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format configuration files: Makefile.
- TODO marks in the source: 23. FIXME marks: 11.

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/phcc.git
- Branch: exo
- Head commit: b78d3ff1dca891bc2f377c5a9f71cc142808afa2
- Working tree: the committed tree at this commit — a historical reference has no working tree

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.


## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/phcc/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

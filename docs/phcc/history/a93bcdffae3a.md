# phcc — reference as of commit a93bcdffae3a

Commit a93bcdffae3ac45f1d29827b59356b2c8ca555f5. 522 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

No dependencies recorded at this point.

## Authors

Author counts over the 522 audited mainline commits folded to this point.

- Adam Sampson <ats@offog.org> — 1 commit
- Alexander Egorenkov <ae@linux-pc-2.(none)> — 1 commit
- Daniel Glöckner <daniel-gl@gmx.net> — 15 commits
- Kirill Smelkov <kirr@landau.phys.spbu.ru> — 2 commits
- Shinichiro Hamaji <shinichiro.hamaji _at_ gmail.com> — 44 commits
- bellard <bellard> — 393 commits
- grischka <grischka> — 66 commits

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- C: 34 files, 31603 lines total, 2917 blank, 4444 comment, 24242 code
- C header: 77 files, 24198 lines total, 1706 blank, 13424 comment, 9068 code
- Shell: 1 files, 33 lines total, 4 blank, 1 comment, 28 code

All languages: 112 files, 55834 lines total, 33338 lines of code. Functions read: 0.

Largest files:

- tcc.c — 11490 lines
- win32/include/winapi/winuser.h — 3472 lines
- win32/include/winapi/wingdi.h — 2843 lines
- tccelf.c — 2729 lines
- win32/include/winapi/winnt.h — 2667 lines

Largest functions, by span in lines:


## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 6 files in test directories or whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format configuration files: Makefile, tests/Makefile.
- TODO marks in the source: 24. FIXME marks: 11.

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/phcc.git
- Branch: exo
- Head commit: a93bcdffae3ac45f1d29827b59356b2c8ca555f5
- Working tree: the committed tree at this commit — a historical reference has no working tree

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.


## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/phcc/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

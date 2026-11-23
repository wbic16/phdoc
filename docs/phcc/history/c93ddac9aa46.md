# phcc — reference as of commit c93ddac9aa46

Commit c93ddac9aa466f11750be3f230e097c207104349. 555 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

No dependencies recorded at this point.

## Authors

Author counts over the 555 audited mainline commits folded to this point.

- Adam Sampson <ats@offog.org> — 1 commit
- Alexander Egorenkov <ae@linux-pc-2.(none)> — 1 commit
- Daniel Glöckner <daniel-gl@gmx.net> — 16 commits
- Kirill Smelkov <kirr@landau.phys.spbu.ru> — 2 commits
- Sam Watkins <sam@nikas.(none)> — 1 commit
- Shinichiro Hamaji <shinichiro.hamaji _at_ gmail.com> — 46 commits
- Soloist Deng <soloist.deng-gmail.com> — 1 commit
- bellard <bellard> — 393 commits
- grischka <grischka> — 94 commits

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- C: 34 files, 30977 lines total, 2830 blank, 4188 comment, 23959 code
- C header: 77 files, 24922 lines total, 1819 blank, 13768 comment, 9335 code
- Shell: 1 files, 33 lines total, 4 blank, 1 comment, 28 code

All languages: 112 files, 55932 lines total, 33322 lines of code. Functions read: 0.

Largest files:

- tccgen.c — 5151 lines
- win32/include/winapi/winuser.h — 3472 lines
- tccpp.c — 2950 lines
- win32/include/winapi/wingdi.h — 2843 lines
- tccelf.c — 2733 lines

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
- Head commit: c93ddac9aa466f11750be3f230e097c207104349
- Working tree: the committed tree at this commit — a historical reference has no working tree

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.


## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/phcc/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

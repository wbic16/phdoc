# libphext-py — reference as of commit ecf7cd6c21cc

Commit ecf7cd6c21cc3437b4959cc449d5d8aa0cbd7e9e. 15 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

No dependencies recorded at this point.

## Authors

Author counts over the 15 audited mainline commits folded to this point.

The author of record is **Will Bickford** — 15 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 15 commits

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 1 files, 5 lines total, 2 blank, 0 comment, 3 code
- Python: 10 files, 1156 lines total, 1 blank, 0 comment, 1155 code
- TOML: 1 files, 19 lines total, 3 blank, 0 comment, 16 code

All languages: 12 files, 1180 lines total, 1174 lines of code. Functions read: 97.

Largest files:

- pyproject.toml — 19 lines
- tests/test_cli.py — 6 lines
- README.md — 5 lines
- libphext/cli.py — 2 lines
- libphext/__init__.py — 1 lines

Largest functions, by span in lines:

- test_main_output in tests/test_cli.py — about 4 lines
- main in libphext/cli.py — about 2 lines

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 63 test functions counted in the source, across 4 files in test directories or whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 0 of 97 (0%).

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/libphext-py.git
- Branch: exo
- Head commit: ecf7cd6c21cc3437b4959cc449d5d8aa0cbd7e9e
- Working tree: the committed tree at this commit — a historical reference has no working tree

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `main` (libphext/cli.py:1) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `__eq__` (libphext/coordinate.py:134) — Function __eq__ takes self, other and gives back nothing; it does its work on what it is given.
- function `__hash__` (libphext/coordinate.py:149) — Function __hash__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `__init__` (libphext/coordinate.py:102) — Function __init__ takes self, lb, sf, sr, cn, vm, bk, ch, sn, sc and gives back nothing; it does its work on what it is 
- function `__lt__` (libphext/coordinate.py:137) — Function __lt__ takes self, other and gives back nothing; it does its work on what it is given.
- function `__str__` (libphext/coordinate.py:124) — Function __str__ takes no inputs beyond the value it is called on, and gives back str:.
- function `as_tuple` (libphext/coordinate.py:142) — Function as_tuple takes no inputs beyond the value it is called on, and gives back tuple:.
- function `bookBreak` (libphext/coordinate.py:84) — Function bookBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it
- function `chapterBreak` (libphext/coordinate.py:90) — Function chapterBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `collectionBreak` (libphext/coordinate.py:69) — Function collectionBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `copy_coordinate` (libphext/coordinate.py:113) — Function copy_coordinate takes self, other and gives back nothing; it does its work on what it is given.
- function `from_string` (libphext/coordinate.py:18) — Function from_string takes ignored, coord_str: str and gives back nothing; it does its work on what it is given.
- function `libraryBreak` (libphext/coordinate.py:39) — Function libraryBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `scrollBreak` (libphext/coordinate.py:99) — Function scrollBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `sectionBreak` (libphext/coordinate.py:95) — Function sectionBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `seriesBreak` (libphext/coordinate.py:60) — Function seriesBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `shelfBreak` (libphext/coordinate.py:50) — Function shelfBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `urlencoded` (libphext/coordinate.py:129) — Function urlencoded takes no inputs beyond the value it is called on, and gives back str:.
- function `volumeBreak` (libphext/coordinate.py:77) — Function volumeBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `__init__` (libphext/phext.py:31) — Function __init__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `append_scroll` (libphext/phext.py:104) — Function append_scroll takes self, entry: PositionedScroll, location: Coordinate and gives back PositionedScroll:.
- function `defaultCoordinate` (libphext/phext.py:28) — Function defaultCoordinate takes no inputs beyond the value it is called on, and gives back Coordinate:.
- function `dephokenize` (libphext/phext.py:149) — Function dephokenize takes self, phokens: list[PositionedScroll] and gives back str:.
- function `fetch` (libphext/phext.py:34) — Function fetch takes self, buffer:str, coord:Coordinate and gives back dict[Coordinate, str]:.
- function `insert` (libphext/phext.py:185) — Function insert takes self, buffer:str, coord:Coordinate, scroll:str and gives back str:.
- function `isPhextBreak` (libphext/phext.py:41) — Function isPhextBreak takes self, byte:str and gives back nothing; it does its work on what it is given.
- function `next_scroll` (libphext/phext.py:211) — Function next_scroll takes self, buffer, coord:Coordinate and gives back List[PositionedScroll]:.
- function `normalize` (libphext/phext.py:158) — Function normalize takes self, buffer:str and gives back str:.
- function `phokenize` (libphext/phext.py:53) — Function phokenize takes self, buffer:str and gives back List[PositionedScroll]:.
- function `range_replace` (libphext/phext.py:195) — Function range_replace takes self, buffer, range:Range, text and gives back str:.
- function `remove` (libphext/phext.py:191) — Function remove takes self, buffer, coord and gives back str:.
- function `replace` (libphext/phext.py:188) — Function replace takes self, buffer, coord, scroll and gives back str:.
- function `update` (libphext/phext.py:162) — Function update takes self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool and gives back str:.
- function `__init__` (libphext/positionedScroll.py:9) — Function __init__ takes self, coord, text and gives back nothing; it does its work on what it is given.
- function `test_main_output` (tests/test_cli.py:3) — Function test_main_output takes capfd and gives back nothing; it does its work on what it is given.
- function `test_book_break` (tests/test_coordinate.py:110) — Function test_book_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `test_chapter_break` (tests/test_coordinate.py:116) — Function test_chapter_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_collection_break` (tests/test_coordinate.py:98) — Function test_collection_break takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `test_independent_coordinates` (tests/test_coordinate.py:28) — Function test_independent_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does 
- function `test_internal_whitespace_support` (tests/test_coordinate.py:64) — Function test_internal_whitespace_support takes no inputs beyond the value it is called on, and gives back nothing; it d
- function `test_large_coordinates` (tests/test_coordinate.py:40) — Function test_large_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its wo
- function `test_library_break` (tests/test_coordinate.py:80) — Function test_library_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_malformed_coordinates` (tests/test_coordinate.py:148) — Function test_malformed_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `test_negative_coordinates` (tests/test_coordinate.py:144) — Function test_negative_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its
- function `test_no_input` (tests/test_coordinate.py:136) — Function test_no_input takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `test_null_book` (tests/test_coordinate.py:176) — Function test_null_book takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `test_null_chapter` (tests/test_coordinate.py:180) — Function test_null_chapter takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_null_collection` (tests/test_coordinate.py:168) — Function test_null_collection takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `test_null_coordinates` (tests/test_coordinate.py:152) — Function test_null_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `test_null_library` (tests/test_coordinate.py:156) — Function test_null_library takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_null_scroll` (tests/test_coordinate.py:188) — Function test_null_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_null_section` (tests/test_coordinate.py:184) — Function test_null_section takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_null_series` (tests/test_coordinate.py:164) — Function test_null_series takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_null_shelf` (tests/test_coordinate.py:160) — Function test_null_shelf takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `test_null_volume` (tests/test_coordinate.py:172) — Function test_null_volume takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_scroll_break` (tests/test_coordinate.py:128) — Function test_scroll_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_section_break` (tests/test_coordinate.py:122) — Function test_section_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_series_break` (tests/test_coordinate.py:92) — Function test_series_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_shelf_break` (tests/test_coordinate.py:86) — Function test_shelf_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_to_urlencoded` (tests/test_coordinate.py:76) — Function test_to_urlencoded takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_too_large_coordinates` (tests/test_coordinate.py:140) — Function test_too_large_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `test_valid_home_coordinate` (tests/test_coordinate.py:16) — Function test_valid_home_coordinate takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `test_volume_break` (tests/test_coordinate.py:104) — Function test_volume_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_whitespace_support` (tests/test_coordinate.py:52) — Function test_whitespace_support takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `test_contract` (tests/test_phext.py:438) — Function test_contract takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `test_coordinate_based_insert` (tests/test_phext.py:128) — Function test_coordinate_based_insert takes no inputs beyond the value it is called on, and gives back nothing; it does 
- function `test_coordinate_based_remove` (tests/test_phext.py:343) — Function test_coordinate_based_remove takes no inputs beyond the value it is called on, and gives back nothing; it does 
- function `test_coordinate_based_replace` (tests/test_phext.py:246) — Function test_coordinate_based_replace takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `test_dead_reckoning` (tests/test_phext.py:36) — Function test_dead_reckoning takes no inputs beyond the value it is called on, and gives back nothing; it does its work 
- function `test_expand` (tests/test_phext.py:435) — Function test_expand takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `test_fs_read_write` (tests/test_phext.py:441) — Function test_fs_read_write takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_hash_support` (tests/test_phext.py:474) — Function test_hash_support takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_insert_performance_2k_scrolls` (tests/test_phext.py:468) — Function test_insert_performance_2k_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it
- function `test_insert_performance_medium_scrolls` (tests/test_phext.py:471) — Function test_insert_performance_medium_scrolls takes no inputs beyond the value it is called on, and gives back nothing
- function `test_larger_coordinates` (tests/test_phext.py:456) — Function test_larger_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `test_last_empty_scroll` (tests/test_phext.py:423) — Function test_last_empty_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its wo
- function `test_line_break` (tests/test_phext.py:122) — Function test_line_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `test_macrophext` (tests/test_phext.py:480) — Function test_macrophext takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `test_merge` (tests/test_phext.py:426) — Function test_merge takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `test_more_cowbell` (tests/test_phext.py:125) — Function test_more_cowbell takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_navmap` (tests/test_phext.py:450) — Function test_navmap takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `test_next_scroll` (tests/test_phext.py:413) — Function test_next_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_normalize` (tests/test_phext.py:432) — Function test_normalize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `test_normalize` (tests/test_phext.py:507) — Function test_normalize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `test_phext_breaks` (tests/test_phext.py:484) — Function test_phext_breaks takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_phext_index` (tests/test_phext.py:459) — Function test_phext_index takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_phext_soundex_v1` (tests/test_phext.py:465) — Function test_phext_soundex_v1 takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `test_range_based_replace` (tests/test_phext.py:396) — Function test_range_based_replace takes no inputs beyond the value it is called on, and gives back nothing; it does its 
- function `test_replace_create` (tests/test_phext.py:444) — Function test_replace_create takes no inputs beyond the value it is called on, and gives back nothing; it does its work 
- function `test_scroll_manifest` (tests/test_phext.py:462) — Function test_scroll_manifest takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `test_subspace_filter` (tests/test_phext.py:477) — Function test_subspace_filter takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `test_subtract` (tests/test_phext.py:429) — Function test_subtract takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `test_summary` (tests/test_phext.py:447) — Function test_summary takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `test_textmap` (tests/test_phext.py:453) — Function test_textmap takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `test_phokenize` (tests/test_positionedScroll.py:40) — Function test_phokenize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `test_positioned_scrolls` (tests/test_positionedScroll.py:20) — Function test_positioned_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `test_scroll_interface` (tests/test_positionedScroll.py:13) — Function test_scroll_interface takes no inputs beyond the value it is called on, and gives back nothing; it does its wor

## function — main

Found in libphext/cli.py at line 1.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — __eq__

Found in libphext/coordinate.py at line 134.

Signature: `def __eq__(self, other):`

What it does: Function __eq__ takes self, other and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — __hash__

Found in libphext/coordinate.py at line 149.

Signature: `def __hash__(self):`

What it does: Function __hash__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — __init__

Found in libphext/coordinate.py at line 102.

Signature: `def __init__(self, lb, sf, sr, cn, vm, bk, ch, sn, sc):`

What it does: Function __init__ takes self, lb, sf, sr, cn, vm, bk, ch, sn, sc and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — __lt__

Found in libphext/coordinate.py at line 137.

Signature: `def __lt__(self, other):`

What it does: Function __lt__ takes self, other and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — __str__

Found in libphext/coordinate.py at line 124.

Signature: `def __str__(self) -> str:`

What it does: Function __str__ takes no inputs beyond the value it is called on, and gives back str:.

(Where these words came from: signature.)

## function — as_tuple

Found in libphext/coordinate.py at line 142.

Signature: `def as_tuple(self) -> tuple:`

What it does: Function as_tuple takes no inputs beyond the value it is called on, and gives back tuple:.

(Where these words came from: signature.)

## function — bookBreak

Found in libphext/coordinate.py at line 84.

Signature: `def bookBreak(self):`

What it does: Function bookBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — chapterBreak

Found in libphext/coordinate.py at line 90.

Signature: `def chapterBreak(self):`

What it does: Function chapterBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — collectionBreak

Found in libphext/coordinate.py at line 69.

Signature: `def collectionBreak(self):`

What it does: Function collectionBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — copy_coordinate

Found in libphext/coordinate.py at line 113.

Signature: `def copy_coordinate(self, other):`

What it does: Function copy_coordinate takes self, other and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — from_string

Found in libphext/coordinate.py at line 18.

Signature: `def from_string(ignored, coord_str: str):`

What it does: Function from_string takes ignored, coord_str: str and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — libraryBreak

Found in libphext/coordinate.py at line 39.

Signature: `def libraryBreak(self):`

What it does: Function libraryBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — scrollBreak

Found in libphext/coordinate.py at line 99.

Signature: `def scrollBreak(self):`

What it does: Function scrollBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — sectionBreak

Found in libphext/coordinate.py at line 95.

Signature: `def sectionBreak(self):`

What it does: Function sectionBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — seriesBreak

Found in libphext/coordinate.py at line 60.

Signature: `def seriesBreak(self):`

What it does: Function seriesBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — shelfBreak

Found in libphext/coordinate.py at line 50.

Signature: `def shelfBreak(self):`

What it does: Function shelfBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — urlencoded

Found in libphext/coordinate.py at line 129.

Signature: `def urlencoded(self) -> str:`

What it does: Function urlencoded takes no inputs beyond the value it is called on, and gives back str:.

(Where these words came from: signature.)

## function — volumeBreak

Found in libphext/coordinate.py at line 77.

Signature: `def volumeBreak(self):`

What it does: Function volumeBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — __init__

Found in libphext/phext.py at line 31.

Signature: `def __init__(self):`

What it does: Function __init__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — append_scroll

Found in libphext/phext.py at line 104.

Signature: `def append_scroll(self, entry: PositionedScroll, location: Coordinate) -> PositionedScroll:`

What it does: Function append_scroll takes self, entry: PositionedScroll, location: Coordinate and gives back PositionedScroll:.

(Where these words came from: signature.)

## function — defaultCoordinate

Found in libphext/phext.py at line 28.

Signature: `def defaultCoordinate(self) -> Coordinate:`

What it does: Function defaultCoordinate takes no inputs beyond the value it is called on, and gives back Coordinate:.

(Where these words came from: signature.)

## function — dephokenize

Found in libphext/phext.py at line 149.

Signature: `def dephokenize(self, phokens: list[PositionedScroll]) -> str:`

What it does: Function dephokenize takes self, phokens: list[PositionedScroll] and gives back str:.

(Where these words came from: signature.)

## function — fetch

Found in libphext/phext.py at line 34.

Signature: `def fetch(self, buffer:str, coord:Coordinate) -> dict[Coordinate, str]:`

What it does: Function fetch takes self, buffer:str, coord:Coordinate and gives back dict[Coordinate, str]:.

(Where these words came from: signature.)

## function — insert

Found in libphext/phext.py at line 185.

Signature: `def insert(self, buffer:str, coord:Coordinate, scroll:str) -> str:`

What it does: Function insert takes self, buffer:str, coord:Coordinate, scroll:str and gives back str:.

(Where these words came from: signature.)

## function — isPhextBreak

Found in libphext/phext.py at line 41.

Signature: `def isPhextBreak(self, byte:str):`

What it does: Function isPhextBreak takes self, byte:str and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — next_scroll

Found in libphext/phext.py at line 211.

Signature: `def next_scroll(self, buffer, coord:Coordinate) -> List[PositionedScroll]:`

What it does: Function next_scroll takes self, buffer, coord:Coordinate and gives back List[PositionedScroll]:.

(Where these words came from: signature.)

## function — normalize

Found in libphext/phext.py at line 158.

Signature: `def normalize(self, buffer:str) -> str:`

What it does: Function normalize takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

## function — phokenize

Found in libphext/phext.py at line 53.

Signature: `def phokenize(self, buffer:str) -> List[PositionedScroll]:`

What it does: Function phokenize takes self, buffer:str and gives back List[PositionedScroll]:.

(Where these words came from: signature.)

## function — range_replace

Found in libphext/phext.py at line 195.

Signature: `def range_replace(self, buffer, range:Range, text) -> str:`

What it does: Function range_replace takes self, buffer, range:Range, text and gives back str:.

(Where these words came from: signature.)

## function — remove

Found in libphext/phext.py at line 191.

Signature: `def remove(self, buffer, coord) -> str:`

What it does: Function remove takes self, buffer, coord and gives back str:.

(Where these words came from: signature.)

## function — replace

Found in libphext/phext.py at line 188.

Signature: `def replace(self, buffer, coord, scroll) -> str:`

What it does: Function replace takes self, buffer, coord, scroll and gives back str:.

(Where these words came from: signature.)

## function — update

Found in libphext/phext.py at line 162.

Signature: `def update(self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool) -> str:`

What it does: Function update takes self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool and gives back str:.

(Where these words came from: signature.)

## function — __init__

Found in libphext/positionedScroll.py at line 9.

Signature: `def __init__(self, coord, text):`

What it does: Function __init__ takes self, coord, text and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_main_output

Found in tests/test_cli.py at line 3.

Signature: `def test_main_output(capfd):`

What it does: Function test_main_output takes capfd and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_book_break

Found in tests/test_coordinate.py at line 110.

Signature: `def test_book_break():`

What it does: Function test_book_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_chapter_break

Found in tests/test_coordinate.py at line 116.

Signature: `def test_chapter_break():`

What it does: Function test_chapter_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_collection_break

Found in tests/test_coordinate.py at line 98.

Signature: `def test_collection_break():`

What it does: Function test_collection_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_independent_coordinates

Found in tests/test_coordinate.py at line 28.

Signature: `def test_independent_coordinates():`

What it does: Function test_independent_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_internal_whitespace_support

Found in tests/test_coordinate.py at line 64.

Signature: `def test_internal_whitespace_support():`

What it does: Function test_internal_whitespace_support takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_large_coordinates

Found in tests/test_coordinate.py at line 40.

Signature: `def test_large_coordinates():`

What it does: Function test_large_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_library_break

Found in tests/test_coordinate.py at line 80.

Signature: `def test_library_break():`

What it does: Function test_library_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_malformed_coordinates

Found in tests/test_coordinate.py at line 148.

Signature: `def test_malformed_coordinates():`

What it does: Function test_malformed_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_negative_coordinates

Found in tests/test_coordinate.py at line 144.

Signature: `def test_negative_coordinates():`

What it does: Function test_negative_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_no_input

Found in tests/test_coordinate.py at line 136.

Signature: `def test_no_input():`

What it does: Function test_no_input takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_null_book

Found in tests/test_coordinate.py at line 176.

Signature: `def test_null_book():`

What it does: Function test_null_book takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_null_chapter

Found in tests/test_coordinate.py at line 180.

Signature: `def test_null_chapter():`

What it does: Function test_null_chapter takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_null_collection

Found in tests/test_coordinate.py at line 168.

Signature: `def test_null_collection():`

What it does: Function test_null_collection takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_null_coordinates

Found in tests/test_coordinate.py at line 152.

Signature: `def test_null_coordinates():`

What it does: Function test_null_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_null_library

Found in tests/test_coordinate.py at line 156.

Signature: `def test_null_library():`

What it does: Function test_null_library takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_null_scroll

Found in tests/test_coordinate.py at line 188.

Signature: `def test_null_scroll():`

What it does: Function test_null_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_null_section

Found in tests/test_coordinate.py at line 184.

Signature: `def test_null_section():`

What it does: Function test_null_section takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_null_series

Found in tests/test_coordinate.py at line 164.

Signature: `def test_null_series():`

What it does: Function test_null_series takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_null_shelf

Found in tests/test_coordinate.py at line 160.

Signature: `def test_null_shelf():`

What it does: Function test_null_shelf takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_null_volume

Found in tests/test_coordinate.py at line 172.

Signature: `def test_null_volume():`

What it does: Function test_null_volume takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_scroll_break

Found in tests/test_coordinate.py at line 128.

Signature: `def test_scroll_break():`

What it does: Function test_scroll_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_section_break

Found in tests/test_coordinate.py at line 122.

Signature: `def test_section_break():`

What it does: Function test_section_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_series_break

Found in tests/test_coordinate.py at line 92.

Signature: `def test_series_break():`

What it does: Function test_series_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_shelf_break

Found in tests/test_coordinate.py at line 86.

Signature: `def test_shelf_break():`

What it does: Function test_shelf_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_to_urlencoded

Found in tests/test_coordinate.py at line 76.

Signature: `def test_to_urlencoded():`

What it does: Function test_to_urlencoded takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_too_large_coordinates

Found in tests/test_coordinate.py at line 140.

Signature: `def test_too_large_coordinates():`

What it does: Function test_too_large_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_valid_home_coordinate

Found in tests/test_coordinate.py at line 16.

Signature: `def test_valid_home_coordinate():`

What it does: Function test_valid_home_coordinate takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_volume_break

Found in tests/test_coordinate.py at line 104.

Signature: `def test_volume_break():`

What it does: Function test_volume_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_whitespace_support

Found in tests/test_coordinate.py at line 52.

Signature: `def test_whitespace_support():`

What it does: Function test_whitespace_support takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_contract

Found in tests/test_phext.py at line 438.

Signature: `def test_contract():`

What it does: Function test_contract takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_coordinate_based_insert

Found in tests/test_phext.py at line 128.

Signature: `def test_coordinate_based_insert():`

What it does: Function test_coordinate_based_insert takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_coordinate_based_remove

Found in tests/test_phext.py at line 343.

Signature: `def test_coordinate_based_remove():`

What it does: Function test_coordinate_based_remove takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_coordinate_based_replace

Found in tests/test_phext.py at line 246.

Signature: `def test_coordinate_based_replace():`

What it does: Function test_coordinate_based_replace takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_dead_reckoning

Found in tests/test_phext.py at line 36.

Signature: `def test_dead_reckoning():`

What it does: Function test_dead_reckoning takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_expand

Found in tests/test_phext.py at line 435.

Signature: `def test_expand():`

What it does: Function test_expand takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_fs_read_write

Found in tests/test_phext.py at line 441.

Signature: `def test_fs_read_write():`

What it does: Function test_fs_read_write takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_hash_support

Found in tests/test_phext.py at line 474.

Signature: `def test_hash_support():`

What it does: Function test_hash_support takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_insert_performance_2k_scrolls

Found in tests/test_phext.py at line 468.

Signature: `def test_insert_performance_2k_scrolls():`

What it does: Function test_insert_performance_2k_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_insert_performance_medium_scrolls

Found in tests/test_phext.py at line 471.

Signature: `def test_insert_performance_medium_scrolls():`

What it does: Function test_insert_performance_medium_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_larger_coordinates

Found in tests/test_phext.py at line 456.

Signature: `def test_larger_coordinates():`

What it does: Function test_larger_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_last_empty_scroll

Found in tests/test_phext.py at line 423.

Signature: `def test_last_empty_scroll():`

What it does: Function test_last_empty_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_line_break

Found in tests/test_phext.py at line 122.

Signature: `def test_line_break():`

What it does: Function test_line_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_macrophext

Found in tests/test_phext.py at line 480.

Signature: `def test_macrophext():`

What it does: Function test_macrophext takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_merge

Found in tests/test_phext.py at line 426.

Signature: `def test_merge():`

What it does: Function test_merge takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_more_cowbell

Found in tests/test_phext.py at line 125.

Signature: `def test_more_cowbell():`

What it does: Function test_more_cowbell takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_navmap

Found in tests/test_phext.py at line 450.

Signature: `def test_navmap():`

What it does: Function test_navmap takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_next_scroll

Found in tests/test_phext.py at line 413.

Signature: `def test_next_scroll():`

What it does: Function test_next_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_normalize

Found in tests/test_phext.py at line 432.

Signature: `def test_normalize():`

What it does: Function test_normalize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_normalize

Found in tests/test_phext.py at line 507.

Signature: `def test_normalize():`

What it does: Function test_normalize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_phext_breaks

Found in tests/test_phext.py at line 484.

Signature: `def test_phext_breaks():`

What it does: Function test_phext_breaks takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_phext_index

Found in tests/test_phext.py at line 459.

Signature: `def test_phext_index():`

What it does: Function test_phext_index takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_phext_soundex_v1

Found in tests/test_phext.py at line 465.

Signature: `def test_phext_soundex_v1():`

What it does: Function test_phext_soundex_v1 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_range_based_replace

Found in tests/test_phext.py at line 396.

Signature: `def test_range_based_replace():`

What it does: Function test_range_based_replace takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_replace_create

Found in tests/test_phext.py at line 444.

Signature: `def test_replace_create():`

What it does: Function test_replace_create takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_scroll_manifest

Found in tests/test_phext.py at line 462.

Signature: `def test_scroll_manifest():`

What it does: Function test_scroll_manifest takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_subspace_filter

Found in tests/test_phext.py at line 477.

Signature: `def test_subspace_filter():`

What it does: Function test_subspace_filter takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_subtract

Found in tests/test_phext.py at line 429.

Signature: `def test_subtract():`

What it does: Function test_subtract takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_summary

Found in tests/test_phext.py at line 447.

Signature: `def test_summary():`

What it does: Function test_summary takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_textmap

Found in tests/test_phext.py at line 453.

Signature: `def test_textmap():`

What it does: Function test_textmap takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_phokenize

Found in tests/test_positionedScroll.py at line 40.

Signature: `def test_phokenize():`

What it does: Function test_phokenize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_positioned_scrolls

Found in tests/test_positionedScroll.py at line 20.

Signature: `def test_positioned_scrolls():`

What it does: Function test_positioned_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — test_scroll_interface

Found in tests/test_positionedScroll.py at line 13.

Signature: `def test_scroll_interface():`

What it does: Function test_scroll_interface takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/libphext-py/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

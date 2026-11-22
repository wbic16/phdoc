# libphext-py — reference as of commit 9d8efb0dd7e8

Commit 9d8efb0dd7e84115b221025049dec4298d1279ca. 5 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

No dependencies recorded at this point.

## Authors

- Will Bickford <wbic16@gmail.com> — 5 commits

## Complexity

- Markdown: 1 files, 5 lines total, 2 blank, 0 comment, 3 code
- Python: 8 files, 388 lines total, 1 blank, 0 comment, 387 code
- TOML: 1 files, 19 lines total, 3 blank, 0 comment, 16 code

Declared RFCs at this point: none declared.

## Public methods and commands

- function `main` (libphext/cli.py) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `__eq__` (libphext/coordinate.py) — Function __eq__ takes self, other and gives back nothing; it does its work on what it is given.
- function `__hash__` (libphext/coordinate.py) — Function __hash__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `__init__` (libphext/coordinate.py) — Function __init__ takes self, lb, sf, sr, cn, vm, bk, ch, sn, sc and gives back nothing; it does its work on what it is 
- function `__str__` (libphext/coordinate.py) — Function __str__ takes no inputs beyond the value it is called on, and gives back str:.
- function `bookBreak` (libphext/coordinate.py) — Function bookBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it
- function `chapterBreak` (libphext/coordinate.py) — Function chapterBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `collectionBreak` (libphext/coordinate.py) — Function collectionBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `from_string` (libphext/coordinate.py) — Function from_string takes ignored, coord_str: str and gives back nothing; it does its work on what it is given.
- function `libraryBreak` (libphext/coordinate.py) — Function libraryBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `scrollBreak` (libphext/coordinate.py) — Function scrollBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `sectionBreak` (libphext/coordinate.py) — Function sectionBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `seriesBreak` (libphext/coordinate.py) — Function seriesBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `shelfBreak` (libphext/coordinate.py) — Function shelfBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `urlencoded` (libphext/coordinate.py) — Function urlencoded takes no inputs beyond the value it is called on, and gives back str:.
- function `volumeBreak` (libphext/coordinate.py) — Function volumeBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `__init__` (libphext/phext.py) — Function __init__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `fetch` (libphext/phext.py) — Function fetch takes self, buffer, coord and gives back dict[Coordinate, str]:.
- function `__init__` (libphext/positionedScroll.py) — Function __init__ takes self, coord, text and gives back nothing; it does its work on what it is given.
- function `test_main_output` (tests/test_cli.py) — Function test_main_output takes capfd and gives back nothing; it does its work on what it is given.
- function `test_book_break` (tests/test_coordinate.py) — Function test_book_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `test_chapter_break` (tests/test_coordinate.py) — Function test_chapter_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_collection_break` (tests/test_coordinate.py) — Function test_collection_break takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `test_independent_coordinates` (tests/test_coordinate.py) — Function test_independent_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does 
- function `test_internal_whitespace_support` (tests/test_coordinate.py) — Function test_internal_whitespace_support takes no inputs beyond the value it is called on, and gives back nothing; it d
- function `test_large_coordinates` (tests/test_coordinate.py) — Function test_large_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its wo
- function `test_library_break` (tests/test_coordinate.py) — Function test_library_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_malformed_coordinates` (tests/test_coordinate.py) — Function test_malformed_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `test_negative_coordinates` (tests/test_coordinate.py) — Function test_negative_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its
- function `test_no_input` (tests/test_coordinate.py) — Function test_no_input takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `test_null_book` (tests/test_coordinate.py) — Function test_null_book takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `test_null_chapter` (tests/test_coordinate.py) — Function test_null_chapter takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_null_collection` (tests/test_coordinate.py) — Function test_null_collection takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `test_null_coordinates` (tests/test_coordinate.py) — Function test_null_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `test_null_library` (tests/test_coordinate.py) — Function test_null_library takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_null_scroll` (tests/test_coordinate.py) — Function test_null_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_null_section` (tests/test_coordinate.py) — Function test_null_section takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_null_series` (tests/test_coordinate.py) — Function test_null_series takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_null_shelf` (tests/test_coordinate.py) — Function test_null_shelf takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `test_null_volume` (tests/test_coordinate.py) — Function test_null_volume takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_scroll_break` (tests/test_coordinate.py) — Function test_scroll_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_section_break` (tests/test_coordinate.py) — Function test_section_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_series_break` (tests/test_coordinate.py) — Function test_series_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_shelf_break` (tests/test_coordinate.py) — Function test_shelf_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_to_urlencoded` (tests/test_coordinate.py) — Function test_to_urlencoded takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_too_large_coordinates` (tests/test_coordinate.py) — Function test_too_large_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `test_valid_home_coordinate` (tests/test_coordinate.py) — Function test_valid_home_coordinate takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `test_volume_break` (tests/test_coordinate.py) — Function test_volume_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_whitespace_support` (tests/test_coordinate.py) — Function test_whitespace_support takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `test_positioned_scrolls` (tests/test_positionedScroll.py) — Function test_positioned_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `test_realistic_parse` (tests/test_positionedScroll.py) — Function test_realistic_parse takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `test_scroll_interface` (tests/test_positionedScroll.py) — Function test_scroll_interface takes no inputs beyond the value it is called on, and gives back nothing; it does its wor

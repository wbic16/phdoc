# libphext-py — reference as of commit a83f53cfb2af

Commit a83f53cfb2afb70e2f87e4f0b68859f3167b5596. 41 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/libphext-py/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

## Dependencies

No dependencies recorded at this point.

## Authors

The author of record is **Will Bickford** — 41 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 40 commits
- Orin — 1 commit

## Complexity

- Markdown: 1 files, 118 lines total, 2 blank, 0 comment, 116 code
- Python: 13 files, 2690 lines total, 214 blank, 93 comment, 2383 code
- TOML: 1 files, 43 lines total, 3 blank, 0 comment, 40 code

Declared RFCs at this point: none declared.

## Public methods and commands

- function `__eq__` (phext/coordinate.py) — Function __eq__ takes self, other and gives back nothing; it does its work on what it is given.
- function `__hash__` (phext/coordinate.py) — Function __hash__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `__init__` (phext/coordinate.py) — Function __init__ takes self, lb, sf, sr, cn, vm, bk, ch, sn, sc and gives back nothing; it does its work on what it is 
- function `__lt__` (phext/coordinate.py) — Function __lt__ takes self, other and gives back nothing; it does its work on what it is given.
- function `__str__` (phext/coordinate.py) — Function __str__ takes no inputs beyond the value it is called on, and gives back str:.
- function `advance_coordinate` (phext/coordinate.py) — Function advance_coordinate takes index: int and gives back int:.
- function `as_tuple` (phext/coordinate.py) — Function as_tuple takes no inputs beyond the value it is called on, and gives back tuple:.
- function `bookBreak` (phext/coordinate.py) — Function bookBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it
- function `chapterBreak` (phext/coordinate.py) — Function chapterBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `collectionBreak` (phext/coordinate.py) — Function collectionBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `copy_coordinate` (phext/coordinate.py) — Function copy_coordinate takes self, other and gives back nothing; it does its work on what it is given.
- function `from_string` (phext/coordinate.py) — Function from_string takes ignored, coord_str: str and gives back nothing; it does its work on what it is given.
- function `libraryBreak` (phext/coordinate.py) — Function libraryBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `scrollBreak` (phext/coordinate.py) — Function scrollBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `sectionBreak` (phext/coordinate.py) — Function sectionBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `seriesBreak` (phext/coordinate.py) — Function seriesBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `shelfBreak` (phext/coordinate.py) — Function shelfBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `urlencoded` (phext/coordinate.py) — Function urlencoded takes no inputs beyond the value it is called on, and gives back str:.
- function `volumeBreak` (phext/coordinate.py) — Function volumeBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `__init__` (phext/phext.py) — Function __init__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `append_scroll` (phext/phext.py) — Function append_scroll takes self, entry: PositionedScroll, location: Coordinate and gives back PositionedScroll:.
- function `checksum` (phext/phext.py) — Function checksum takes self, buffer:str and gives back str:.
- function `contract` (phext/phext.py) — Function contract takes self, buffer:str and gives back str:.
- function `create_summary` (phext/phext.py) — Function create_summary takes self, buffer:str and gives back str:.
- function `defaultCoordinate` (phext/phext.py) — Function defaultCoordinate takes no inputs beyond the value it is called on, and gives back Coordinate:.
- function `dephokenize` (phext/phext.py) — Function dephokenize takes self, stack: list[PositionedScroll] and gives back str:.
- function `expand` (phext/phext.py) — Function expand takes self, buffer:str and gives back str:.
- function `explode` (phext/phext.py) — Function explode takes self, buffer:str and gives back dict[Coordinate, str]:.
- function `fetch` (phext/phext.py) — Function fetch takes self, buffer:str, coord:Coordinate and gives back dict[Coordinate, str]:.
- function `get_subspace_coordinates` (phext/phext.py) — Function get_subspace_coordinates takes self, buffer:str, target:Coordinate and gives back nothing; it does its work on 
- function `implode` (phext/phext.py) — Function implode takes self, hash: dict[Coordinate, str] and gives back str:.
- function `index` (phext/phext.py) — Function index takes self, buffer:str and gives back str:.
- function `index_phokens` (phext/phext.py) — Function index_phokens takes self, buffer:str and gives back list[PositionedScroll]:.
- function `insert` (phext/phext.py) — Function insert takes self, buffer:str, coord:Coordinate, scroll:str and gives back str:.
- function `isPhextBreak` (phext/phext.py) — Function isPhextBreak takes self, byte:str and gives back nothing; it does its work on what it is given.
- function `manifest` (phext/phext.py) — Function manifest takes self, buffer:str and gives back str:.
- function `merge` (phext/phext.py) — Function merge takes self, left: str, right:str and gives back str:.
- function `navmap` (phext/phext.py) — Function navmap takes self, urlbase:str, buffer:str and gives back str:.
- function `next_scroll` (phext/phext.py) — Function next_scroll takes self, buffer:str, coord:Coordinate and gives back List[PositionedScroll]:.
- function `normalize` (phext/phext.py) — Function normalize takes self, buffer:str and gives back str:.
- function `offset` (phext/phext.py) — Function offset takes self, buffer:str, coord:Coordinate and gives back int:.
- function `phokenize` (phext/phext.py) — Function phokenize takes self, buffer:str and gives back List[PositionedScroll]:.
- function `range_replace` (phext/phext.py) — Function range_replace takes self, buffer:str, range:Range, text:str and gives back str:.
- function `remove` (phext/phext.py) — Function remove takes self, buffer:str, coord:Coordinate and gives back str:.
- function `replace` (phext/phext.py) — Function replace takes self, buffer:str, coord:Coordinate, scroll:str and gives back str:.
- function `soundex_internal` (phext/phext.py) — Function soundex_internal takes self, buffer:str and gives back str:.
- function `soundex_v1` (phext/phext.py) — Function soundex_v1 takes self, buffer:str and gives back str:.
- function `subtract` (phext/phext.py) — Function subtract takes self, left:str, right:str and gives back str:.
- function `textmap` (phext/phext.py) — Function textmap takes self, buffer:str and gives back str:.
- function `update` (phext/phext.py) — Function update takes self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool and gives back str:.
- function `__init__` (phext/positionedScroll.py) — Function __init__ takes self, coord, text and gives back nothing; it does its work on what it is given.
- function `main` (test/bench.py) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `timed` (test/bench.py) — Function timed takes label, fn and gives back nothing; it does its work on what it is given.
- function `__init__` (tests/legacy_phext.py) — Function __init__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `append_scroll` (tests/legacy_phext.py) — Function append_scroll takes self, entry: PositionedScroll, location: Coordinate and gives back PositionedScroll:.
- function `checksum` (tests/legacy_phext.py) — Function checksum takes self, buffer:str and gives back str:.
- function `contract` (tests/legacy_phext.py) — Function contract takes self, buffer:str and gives back str:.
- function `create_summary` (tests/legacy_phext.py) — Function create_summary takes self, buffer:str and gives back str:.
- function `defaultCoordinate` (tests/legacy_phext.py) — Function defaultCoordinate takes no inputs beyond the value it is called on, and gives back Coordinate:.
- function `dephokenize` (tests/legacy_phext.py) — Function dephokenize takes self, stack: list[PositionedScroll] and gives back str:.
- function `expand` (tests/legacy_phext.py) — Function expand takes self, buffer:str and gives back str:.
- function `explode` (tests/legacy_phext.py) — Function explode takes self, buffer:str and gives back dict[Coordinate, str]:.
- function `fetch` (tests/legacy_phext.py) — Function fetch takes self, buffer:str, coord:Coordinate and gives back dict[Coordinate, str]:.
- function `get_subspace_coordinates` (tests/legacy_phext.py) — Function get_subspace_coordinates takes self, buffer:str, target:Coordinate and gives back nothing; it does its work on 
- function `implode` (tests/legacy_phext.py) — Function implode takes self, hash: dict[Coordinate, str] and gives back str:.
- function `index` (tests/legacy_phext.py) — Function index takes self, buffer:str and gives back str:.
- function `index_phokens` (tests/legacy_phext.py) — Function index_phokens takes self, buffer:str and gives back list[PositionedScroll]:.
- function `insert` (tests/legacy_phext.py) — Function insert takes self, buffer:str, coord:Coordinate, scroll:str and gives back str:.
- function `isPhextBreak` (tests/legacy_phext.py) — Function isPhextBreak takes self, byte:str and gives back nothing; it does its work on what it is given.
- function `manifest` (tests/legacy_phext.py) — Function manifest takes self, buffer:str and gives back str:.
- function `merge` (tests/legacy_phext.py) — Function merge takes self, left: str, right:str and gives back str:.
- function `navmap` (tests/legacy_phext.py) — Function navmap takes self, urlbase:str, buffer:str and gives back str:.
- function `next_scroll` (tests/legacy_phext.py) — Function next_scroll takes self, buffer:str, coord:Coordinate and gives back List[PositionedScroll]:.
- function `normalize` (tests/legacy_phext.py) — Function normalize takes self, buffer:str and gives back str:.
- function `offset` (tests/legacy_phext.py) — Function offset takes self, buffer:str, coord:Coordinate and gives back int:.
- function `phokenize` (tests/legacy_phext.py) — Function phokenize takes self, buffer:str and gives back List[PositionedScroll]:.
- function `range_replace` (tests/legacy_phext.py) — Function range_replace takes self, buffer:str, range:Range, text:str and gives back str:.
- function `remove` (tests/legacy_phext.py) — Function remove takes self, buffer:str, coord:Coordinate and gives back str:.
- function `replace` (tests/legacy_phext.py) — Function replace takes self, buffer:str, coord:Coordinate, scroll:str and gives back str:.
- function `soundex_internal` (tests/legacy_phext.py) — Function soundex_internal takes self, buffer:str and gives back str:.
- function `soundex_v1` (tests/legacy_phext.py) — Function soundex_v1 takes self, buffer:str and gives back str:.
- function `subtract` (tests/legacy_phext.py) — Function subtract takes self, left:str, right:str and gives back str:.
- function `textmap` (tests/legacy_phext.py) — Function textmap takes self, buffer:str and gives back str:.
- function `update` (tests/legacy_phext.py) — Function update takes self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool and gives back str:.
- function `test_book_break` (tests/test_coordinate.py) — Function test_book_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `test_breaks_saturate_at_999` (tests/test_coordinate.py) — Function test_breaks_saturate_at_999 takes no inputs beyond the value it is called on, and gives back nothing; it does i
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
- function `test_phokenize_scroll_saturation` (tests/test_coordinate.py) — Function test_phokenize_scroll_saturation takes no inputs beyond the value it is called on, and gives back nothing; it d
- function `test_scroll_break` (tests/test_coordinate.py) — Function test_scroll_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_section_break` (tests/test_coordinate.py) — Function test_section_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_series_break` (tests/test_coordinate.py) — Function test_series_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_shelf_break` (tests/test_coordinate.py) — Function test_shelf_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_to_urlencoded` (tests/test_coordinate.py) — Function test_to_urlencoded takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_too_large_coordinates` (tests/test_coordinate.py) — Function test_too_large_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `test_valid_home_coordinate` (tests/test_coordinate.py) — Function test_valid_home_coordinate takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `test_volume_break` (tests/test_coordinate.py) — Function test_volume_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_whitespace_support` (tests/test_coordinate.py) — Function test_whitespace_support takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `test_contract` (tests/test_phext.py) — Function test_contract takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `test_coordinate_based_insert` (tests/test_phext.py) — Function test_coordinate_based_insert takes no inputs beyond the value it is called on, and gives back nothing; it does 
- function `test_coordinate_based_remove` (tests/test_phext.py) — Function test_coordinate_based_remove takes no inputs beyond the value it is called on, and gives back nothing; it does 
- function `test_coordinate_based_replace` (tests/test_phext.py) — Function test_coordinate_based_replace takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `test_dead_reckoning` (tests/test_phext.py) — Function test_dead_reckoning takes no inputs beyond the value it is called on, and gives back nothing; it does its work 
- function `test_expand` (tests/test_phext.py) — Function test_expand takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `test_fs_read_write` (tests/test_phext.py) — Function test_fs_read_write takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_hash_support` (tests/test_phext.py) — Function test_hash_support takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_insert_performance_2k_scrolls` (tests/test_phext.py) — Function test_insert_performance_2k_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it
- function `test_larger_coordinates` (tests/test_phext.py) — Function test_larger_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `test_last_empty_scroll` (tests/test_phext.py) — Function test_last_empty_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its wo
- function `test_line_break` (tests/test_phext.py) — Function test_line_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `test_merge` (tests/test_phext.py) — Function test_merge takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `test_more_cowbell` (tests/test_phext.py) — Function test_more_cowbell takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_navmap` (tests/test_phext.py) — Function test_navmap takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `test_next_scroll` (tests/test_phext.py) — Function test_next_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_normalize` (tests/test_phext.py) — Function test_normalize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `test_phext_breaks` (tests/test_phext.py) — Function test_phext_breaks takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_phext_index` (tests/test_phext.py) — Function test_phext_index takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_phext_soundex_v1` (tests/test_phext.py) — Function test_phext_soundex_v1 takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `test_range_based_replace` (tests/test_phext.py) — Function test_range_based_replace takes no inputs beyond the value it is called on, and gives back nothing; it does its 
- function `test_replace_create` (tests/test_phext.py) — Function test_replace_create takes no inputs beyond the value it is called on, and gives back nothing; it does its work 
- function `test_scroll_manifest` (tests/test_phext.py) — Function test_scroll_manifest takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `test_subtract` (tests/test_phext.py) — Function test_subtract takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `test_summary` (tests/test_phext.py) — Function test_summary takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `test_textmap` (tests/test_phext.py) — Function test_textmap takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `as_pairs` (tests/test_phokenize.py) — Function as_pairs takes stack and gives back nothing; it does its work on what it is given.
- function `corpus` (tests/test_phokenize.py) — Function corpus takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is
- function `large_document` (tests/test_phokenize.py) — Function large_document takes scrolls: int and gives back str:.
- function `outcome` (tests/test_phokenize.py) — Function outcome takes fn, *args and gives back nothing; it does its work on what it is given.
- function `random_coordinate` (tests/test_phokenize.py) — Function random_coordinate takes rng: random.Random and gives back Coordinate:.
- function `random_document` (tests/test_phokenize.py) — Function random_document takes rng: random.Random and gives back str:.
- function `test_append_scroll_matches_legacy` (tests/test_phokenize.py) — Function test_append_scroll_matches_legacy takes no inputs beyond the value it is called on, and gives back nothing; it 
- function `test_append_scroll_out_of_order` (tests/test_phokenize.py) — Function test_append_scroll_out_of_order takes no inputs beyond the value it is called on, and gives back nothing; it do
- function `test_edits_match_legacy` (tests/test_phokenize.py) — Function test_edits_match_legacy takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `test_explode_implode_round_trip` (tests/test_phokenize.py) — Function test_explode_implode_round_trip takes no inputs beyond the value it is called on, and gives back nothing; it do
- function `test_explode_performance_50k_scrolls` (tests/test_phokenize.py) — Function test_explode_performance_50k_scrolls takes no inputs beyond the value it is called on, and gives back nothing; 
- function `test_lookups_match_legacy` (tests/test_phokenize.py) — Function test_lookups_match_legacy takes no inputs beyond the value it is called on, and gives back nothing; it does its
- function `test_maps_and_index_match_legacy` (tests/test_phokenize.py) — Function test_maps_and_index_match_legacy takes no inputs beyond the value it is called on, and gives back nothing; it d
- function `test_phokenize_coordinates_are_distinct_objects` (tests/test_phokenize.py) — Function test_phokenize_coordinates_are_distinct_objects takes no inputs beyond the value it is called on, and gives bac
- function `test_phokenize_matches_legacy` (tests/test_phokenize.py) — Function test_phokenize_matches_legacy takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `test_serialization_matches_legacy` (tests/test_phokenize.py) — Function test_serialization_matches_legacy takes no inputs beyond the value it is called on, and gives back nothing; it 
- function `test_phokenize` (tests/test_positionedScroll.py) — Function test_phokenize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `test_positioned_scrolls` (tests/test_positionedScroll.py) — Function test_positioned_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `test_scroll_interface` (tests/test_positionedScroll.py) — Function test_scroll_interface takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `test_range_exists` (tests/test_range.py) — Function test_range_exists takes no inputs beyond the value it is called on, and gives back nothing; it does its work on

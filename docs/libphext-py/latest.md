# libphext-py — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 91). Project root: /home/wbic16/phdoc-sweep/proj/libphext-py. Scanned at commit e773fe0a2281408e8cefbd44bf07480986d34b6a. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 51 commits. The authors, by commit count:

- Will Bickford <wbic16@gmail.com> — 45 commits (88.2%)
- Emise <emise@localhost> — 4 commits (7.8%)
- Orin <orin@mirrorborn.us> — 1 commit (2.0%)
- wbic16 <wbic16@gmail.com> — 1 commit (2.0%)

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 1 files, 172 lines total, 44 blank, 0 comment, 128 code
- Python: 25 files, 4577 lines total, 656 blank, 442 comment, 3479 code
- Shell: 3 files, 116 lines total, 8 blank, 41 comment, 67 code
- TOML: 1 files, 43 lines total, 7 blank, 4 comment, 32 code

All languages: 30 files, 4908 lines total, 3706 lines of code. Functions read: 286.

Largest files:

- tests/test_phext.py — 920 lines
- tests/legacy_phext.py — 549 lines
- phext/phext.py — 537 lines
- phext/addressing.py — 252 lines
- phext/chain.py — 249 lines

Largest functions, by span in lines:

- test_insert_performance_2k_scrolls in tests/test_phext.py — about 129 lines
- test_coordinate_based_insert in tests/test_phext.py — about 118 lines
- test_coordinate_based_replace in tests/test_phext.py — about 97 lines
- test_dead_reckoning in tests/test_phext.py — about 86 lines
- test_range_fixture_case in tests/test_range_fixtures.py — about 71 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 120 test functions counted in the source, across 13 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 4. FIXME marks: 0.
- Public items with a doc comment: 28 of 266 (11%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/libphext-py.git
- Branch: exo
- Head commit: e773fe0a2281408e8cefbd44bf07480986d34b6a
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `__str__` (phext/addressing.py:66) — Function __str__ takes no inputs beyond the value it is called on, and gives back str:.
- function `__str__` (phext/addressing.py:86) — Function __str__ takes no inputs beyond the value it is called on, and gives back str:.
- function `parse_point_address` (phext/addressing.py:119) — Parses a v0.6.1 point address:
- function `parse_range_address` (phext/addressing.py:190) — Parses a v0.6.1 range address: exactly two point addresses joined
- function `resolve_scroll_offset` (phext/addressing.py:212) — Resolves a caret position inside one scroll to a character
- function `enumerate_scrolls` (phext/addressing.py:235) — The member's scrolls in the phokenize sense: a list of
- function `addressed_scroll_text` (phext/addressing.py:245) — The scroll text at a coordinate in one member phext, in the
- function `is_binary_scroll` (phext/binary.py:60) — True when `scroll` carries the binary scroll header.
- function `encode_scroll` (phext/binary.py:66) — Encodes raw bytes as a binary scroll (header line + base64 body).
- function `decode_scroll` (phext/binary.py:71) — Decodes a binary scroll back to its raw bytes.
- function `insert_binary` (phext/binary.py:84) — Stores `payload` at `location` as a binary scroll, unconditionally:
- function `replace_binary` (phext/binary.py:91) — Replaces the scroll at `location` with `payload`, stored as a
- function `fetch_binary` (phext/binary.py:97) — Retrieves the bytes held at `target`.
- function `insert_bytes` (phext/binary.py:120) — Byte-oriented insert: stores `payload` at `location` under the
- function `replace_bytes` (phext/binary.py:127) — Byte-oriented replace: stores `payload` at `location` under the
- function `phokenize_bytes` (phext/binary.py:141) — The byte-level twin of the reference `phokenize`: walks a raw byte
- function `ingest` (phext/binary.py:172) — Ingests an externally supplied phext byte stream.
- function `serialize` (phext/chain.py:50) — Serializes this chain alone: its members joined by
- function `fetch` (phext/chain.py:59) — Fetch a scroll by multiverse address: ``$N:coord``, or a bare
- function `resolve_point` (phext/chain.py:72) — Resolves a point address against this chain: (universe,
- function `fetch_point` (phext/chain.py:93) — Fetches a point address string.
- function `resolve_range` (phext/chain.py:116) — Resolves a range against this chain into per-scroll segments:
- function `fetch_range` (phext/chain.py:170) — Fetches a range address string: the spanned text with phext
- function `parse_chains` (phext/chain.py:197) — Splits a stream into its chains.
- function `serialize_chains` (phext/chain.py:221) — Serializes a stream of chains back into stream bytes: the inverse
- function `parse_universe_address` (phext/chain.py:227) — Parses a multiverse address into (universe, Coordinate), or None
- function `advance_coordinate` (phext/coordinate.py:4) — Function advance_coordinate takes index: int and gives back int:.
- function `from_string` (phext/coordinate.py:29) — Function from_string takes ignored, coord_str: str and gives back nothing; it does its work on what it is given.
- function `libraryBreak` (phext/coordinate.py:50) — Function libraryBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `shelfBreak` (phext/coordinate.py:61) — Function shelfBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `seriesBreak` (phext/coordinate.py:71) — Function seriesBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `collectionBreak` (phext/coordinate.py:80) — Function collectionBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `volumeBreak` (phext/coordinate.py:88) — Function volumeBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `bookBreak` (phext/coordinate.py:95) — Function bookBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it
- function `chapterBreak` (phext/coordinate.py:101) — Function chapterBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `sectionBreak` (phext/coordinate.py:106) — Function sectionBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `scrollBreak` (phext/coordinate.py:110) — Function scrollBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `__init__` (phext/coordinate.py:113) — Function __init__ takes self, lb, sf, sr, cn, vm, bk, ch, sn, sc and gives back nothing; it does its work on what it is 
- function `copy_coordinate` (phext/coordinate.py:124) — Function copy_coordinate takes self, other and gives back nothing; it does its work on what it is given.
- function `__str__` (phext/coordinate.py:135) — Function __str__ takes no inputs beyond the value it is called on, and gives back str:.
- function `urlencoded` (phext/coordinate.py:140) — Function urlencoded takes no inputs beyond the value it is called on, and gives back str:.
- function `__eq__` (phext/coordinate.py:145) — Function __eq__ takes self, other and gives back nothing; it does its work on what it is given.
- function `__lt__` (phext/coordinate.py:148) — Function __lt__ takes self, other and gives back nothing; it does its work on what it is given.
- function `as_tuple` (phext/coordinate.py:153) — Function as_tuple takes no inputs beyond the value it is called on, and gives back tuple:.
- function `__hash__` (phext/coordinate.py:160) — Function __hash__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `__init__` (phext/indexed.py:31) — Function __init__ takes self, buffer: str and gives back nothing; it does its work on what it is given.
- function `fetch` (phext/indexed.py:42) — Function fetch takes self, coord: Coordinate and gives back str:.
- function `__contains__` (phext/indexed.py:49) — Function __contains__ takes self, coord: Coordinate and gives back bool:.
- function `coordinates` (phext/indexed.py:52) — Function coordinates takes no inputs beyond the value it is called on, and gives back list[Coordinate]:.
- function `keys` (phext/indexed.py:55) — Function keys takes no inputs beyond the value it is called on, and gives back list[Coordinate]:.
- function `index` (phext/indexed.py:58) — Function index takes no inputs beyond the value it is called on, and gives back dict[Coordinate, tuple[int, int]]:.
- function `explode` (phext/indexed.py:62) — Function explode takes no inputs beyond the value it is called on, and gives back dict[Coordinate, str]:.
- function `__len__` (phext/indexed.py:68) — Function __len__ takes no inputs beyond the value it is called on, and gives back int:.
- function `defaultCoordinate` (phext/phext.py:61) — Function defaultCoordinate takes no inputs beyond the value it is called on, and gives back Coordinate:.
- function `__init__` (phext/phext.py:64) — Function __init__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `fetch` (phext/phext.py:67) — Function fetch takes self, buffer:str, coord:Coordinate and gives back dict[Coordinate, str]:.
- function `isPhextBreak` (phext/phext.py:73) — Function isPhextBreak takes self, byte:str and gives back nothing; it does its work on what it is given.
- function `phokenize` (phext/phext.py:111) — Function phokenize takes self, buffer:str and gives back List[PositionedScroll]:.
- function `append_scroll` (phext/phext.py:177) — Function append_scroll takes self, entry: PositionedScroll, location: Coordinate and gives back PositionedScroll:.
- function `dephokenize` (phext/phext.py:183) — Function dephokenize takes self, stack: list[PositionedScroll] and gives back str:.
- function `normalize` (phext/phext.py:191) — Function normalize takes self, buffer:str and gives back str:.
- function `update` (phext/phext.py:195) — Function update takes self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool and gives back str:.
- function `insert` (phext/phext.py:218) — Function insert takes self, buffer:str, coord:Coordinate, scroll:str and gives back str:.
- function `replace` (phext/phext.py:221) — Function replace takes self, buffer:str, coord:Coordinate, scroll:str and gives back str:.
- function `remove` (phext/phext.py:224) — Function remove takes self, buffer:str, coord:Coordinate and gives back str:.
- function `range_replace` (phext/phext.py:228) — Function range_replace takes self, buffer:str, range:Range, text:str and gives back str:.
- function `next_scroll` (phext/phext.py:244) — Function next_scroll takes self, buffer:str, coord:Coordinate and gives back List[PositionedScroll]:.
- function `get_subspace_coordinates` (phext/phext.py:256) — Function get_subspace_coordinates takes self, buffer:str, target:Coordinate and gives back nothing; it does its work on 
- function `merge` (phext/phext.py:310) — Function merge takes self, left: str, right:str and gives back str:.
- function `subtract` (phext/phext.py:342) — Function subtract takes self, left:str, right:str and gives back str:.
- function `expand` (phext/phext.py:363) — Function expand takes self, buffer:str and gives back str:.
- function `contract` (phext/phext.py:366) — Function contract takes self, buffer:str and gives back str:.
- function `create_summary` (phext/phext.py:369) — Function create_summary takes self, buffer:str and gives back str:.
- function `navmap` (phext/phext.py:377) — Function navmap takes self, urlbase:str, buffer:str and gives back str:.
- function `textmap` (phext/phext.py:391) — Function textmap takes self, buffer:str and gives back str:.
- function `checksum` (phext/phext.py:398) — Function checksum takes self, buffer:str and gives back str:.
- function `manifest` (phext/phext.py:404) — Function manifest takes self, buffer:str and gives back str:.
- function `soundex_v1` (phext/phext.py:411) — Function soundex_v1 takes self, buffer:str and gives back str:.
- function `soundex_internal` (phext/phext.py:419) — Function soundex_internal takes self, buffer:str and gives back str:.
- function `explode` (phext/phext.py:443) — Function explode takes self, buffer:str and gives back dict[Coordinate, str]:.
- function `implode` (phext/phext.py:450) — Function implode takes self, hash: dict[Coordinate, str] and gives back str:.
- function `index_phokens` (phext/phext.py:456) — Function index_phokens takes self, buffer:str and gives back list[PositionedScroll]:.
- function `index` (phext/phext.py:475) — Function index takes self, buffer:str and gives back str:.
- function `offset` (phext/phext.py:479) — Function offset takes self, buffer:str, coord:Coordinate and gives back int:.
- function `index_offsets` (phext/phext.py:494) — Function index_offsets takes self, buffer:str and gives back dict[Coordinate, tuple[int, int]]:.
- function `indexed` (phext/phext.py:507) — Function indexed takes self, buffer:str and gives back nothing; it does its work on what it is given.
- function `insert_binary` (phext/phext.py:519) — Function insert_binary takes self, buffer: str, coord: Coordinate, payload: bytes and gives back str:.
- function `replace_binary` (phext/phext.py:523) — Function replace_binary takes self, buffer: str, coord: Coordinate, payload: bytes and gives back str:.
- function `fetch_binary` (phext/phext.py:527) — Function fetch_binary takes self, buffer: str, coord: Coordinate and gives back bytes:.
- function `insert_bytes` (phext/phext.py:531) — Function insert_bytes takes self, buffer: str, coord: Coordinate, payload: bytes and gives back str:.
- function `replace_bytes` (phext/phext.py:535) — Function replace_bytes takes self, buffer: str, coord: Coordinate, payload: bytes and gives back str:.
- function `__init__` (phext/positionedScroll.py:9) — Function __init__ takes self, coord, text and gives back nothing; it does its work on what it is given.
- function `seal` (phext/seal.py:26) — Seals a phext: xz-compresses its full UTF-8 bytes.
- function `seal_bytes` (phext/seal.py:31) — Seals raw phext bytes (see `seal`).
- function `unseal` (phext/seal.py:36) — Unseals a sealed phext back to its exact original bytes.
- function `unseal_string` (phext/seal.py:42) — Unseals directly to text.
- function `timed` (test/bench.py:17) — Function timed takes label, fn and gives back nothing; it does its work on what it is given.
- function `main` (test/bench.py:24) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `defaultCoordinate` (tests/legacy_phext.py:38) — Function defaultCoordinate takes no inputs beyond the value it is called on, and gives back Coordinate:.
- function `__init__` (tests/legacy_phext.py:41) — Function __init__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `fetch` (tests/legacy_phext.py:44) — Function fetch takes self, buffer:str, coord:Coordinate and gives back dict[Coordinate, str]:.
- function `isPhextBreak` (tests/legacy_phext.py:51) — Function isPhextBreak takes self, byte:str and gives back nothing; it does its work on what it is given.
- function `phokenize` (tests/legacy_phext.py:63) — Function phokenize takes self, buffer:str and gives back List[PositionedScroll]:.
- function `append_scroll` (tests/legacy_phext.py:114) — Function append_scroll takes self, entry: PositionedScroll, location: Coordinate and gives back PositionedScroll:.
- function `dephokenize` (tests/legacy_phext.py:159) — Function dephokenize takes self, stack: list[PositionedScroll] and gives back str:.
- function `normalize` (tests/legacy_phext.py:168) — Function normalize takes self, buffer:str and gives back str:.
- function `update` (tests/legacy_phext.py:172) — Function update takes self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool and gives back str:.
- function `insert` (tests/legacy_phext.py:195) — Function insert takes self, buffer:str, coord:Coordinate, scroll:str and gives back str:.
- function `replace` (tests/legacy_phext.py:198) — Function replace takes self, buffer:str, coord:Coordinate, scroll:str and gives back str:.
- function `remove` (tests/legacy_phext.py:201) — Function remove takes self, buffer:str, coord:Coordinate and gives back str:.
- function `range_replace` (tests/legacy_phext.py:205) — Function range_replace takes self, buffer:str, range:Range, text:str and gives back str:.
- function `next_scroll` (tests/legacy_phext.py:221) — Function next_scroll takes self, buffer:str, coord:Coordinate and gives back List[PositionedScroll]:.
- function `get_subspace_coordinates` (tests/legacy_phext.py:234) — Function get_subspace_coordinates takes self, buffer:str, target:Coordinate and gives back nothing; it does its work on 
- function `merge` (tests/legacy_phext.py:297) — Function merge takes self, left: str, right:str and gives back str:.
- function `subtract` (tests/legacy_phext.py:329) — Function subtract takes self, left:str, right:str and gives back str:.
- function `expand` (tests/legacy_phext.py:350) — Function expand takes self, buffer:str and gives back str:.
- function `contract` (tests/legacy_phext.py:375) — Function contract takes self, buffer:str and gives back str:.
- function `create_summary` (tests/legacy_phext.py:400) — Function create_summary takes self, buffer:str and gives back str:.
- function `navmap` (tests/legacy_phext.py:414) — Function navmap takes self, urlbase:str, buffer:str and gives back str:.
- function `textmap` (tests/legacy_phext.py:428) — Function textmap takes self, buffer:str and gives back str:.
- function `checksum` (tests/legacy_phext.py:436) — Function checksum takes self, buffer:str and gives back str:.
- function `manifest` (tests/legacy_phext.py:440) — Function manifest takes self, buffer:str and gives back str:.
- function `soundex_v1` (tests/legacy_phext.py:447) — Function soundex_v1 takes self, buffer:str and gives back str:.
- function `soundex_internal` (tests/legacy_phext.py:455) — Function soundex_internal takes self, buffer:str and gives back str:.
- function `explode` (tests/legacy_phext.py:479) — Function explode takes self, buffer:str and gives back dict[Coordinate, str]:.
- function `implode` (tests/legacy_phext.py:486) — Function implode takes self, hash: dict[Coordinate, str] and gives back str:.
- function `index_phokens` (tests/legacy_phext.py:492) — Function index_phokens takes self, buffer:str and gives back list[PositionedScroll]:.
- function `index` (tests/legacy_phext.py:532) — Function index takes self, buffer:str and gives back str:.
- function `offset` (tests/legacy_phext.py:536) — Function offset takes self, buffer:str, coord:Coordinate and gives back int:.
- function `home` (tests/test_address.py:15) — Function home takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `scroll2` (tests/test_address.py:19) — Function scroll2 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it i
- function `member_a` (tests/test_address.py:23) — Function member_a takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `member_b` (tests/test_address.py:29) — Function member_b takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `make_chain` (tests/test_address.py:33) — Function make_chain takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `test_point_parse_canonical` (tests/test_address.py:37) — Function test_point_parse_canonical takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `test_point_parse_rejects` (tests/test_address.py:60) — Function test_point_parse_rejects takes no inputs beyond the value it is called on, and gives back nothing; it does its 
- function `test_point_resolve` (tests/test_address.py:77) — Function test_point_resolve takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_point_fetch_offset` (tests/test_address.py:95) — Function test_point_fetch_offset takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `test_point_fetch_offset_unicode` (tests/test_address.py:110) — Function test_point_fetch_offset_unicode takes no inputs beyond the value it is called on, and gives back nothing; it do
- function `test_range_parse_canonical` (tests/test_address.py:118) — Function test_range_parse_canonical takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `test_range_fetch_within_scroll` (tests/test_address.py:133) — Function test_range_fetch_within_scroll takes no inputs beyond the value it is called on, and gives back nothing; it doe
- function `test_range_fetch_cross_scroll` (tests/test_address.py:145) — Function test_range_fetch_cross_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `test_range_fetch_multiverse` (tests/test_address.py:157) — Function test_range_fetch_multiverse takes no inputs beyond the value it is called on, and gives back nothing; it does i
- function `test_range_resolution_errors` (tests/test_address.py:176) — Function test_range_resolution_errors takes no inputs beyond the value it is called on, and gives back nothing; it does 
- function `coord` (tests/test_binary.py:11) — Function coord takes address and gives back nothing; it does its work on what it is given.
- function `test_encode_decode_round_trip` (tests/test_binary.py:15) — Function test_encode_decode_round_trip takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `test_decode_scroll_rejects_non_binary` (tests/test_binary.py:39) — Function test_decode_scroll_rejects_non_binary takes no inputs beyond the value it is called on, and gives back nothing;
- function `test_insert_and_fetch_binary` (tests/test_binary.py:51) — Function test_insert_and_fetch_binary takes no inputs beyond the value it is called on, and gives back nothing; it does 
- function `test_fetch_binary_fallbacks` (tests/test_binary.py:68) — Function test_fetch_binary_fallbacks takes no inputs beyond the value it is called on, and gives back nothing; it does i
- function `test_update_rule_non_utf8_is_base64_verbatim` (tests/test_binary.py:80) — Function test_update_rule_non_utf8_is_base64_verbatim takes no inputs beyond the value it is called on, and gives back n
- function `test_update_rule_utf8_stays_text` (tests/test_binary.py:97) — Function test_update_rule_utf8_stays_text takes no inputs beyond the value it is called on, and gives back nothing; it d
- function `test_ingest_foreign_byte_stream` (tests/test_binary.py:105) — Function test_ingest_foreign_byte_stream takes no inputs beyond the value it is called on, and gives back nothing; it do
- function `test_ingest_canonical_phext_is_identity` (tests/test_binary.py:119) — Function test_ingest_canonical_phext_is_identity takes no inputs beyond the value it is called on, and gives back nothin
- function `test_ingest_empty_and_break_only_streams` (tests/test_binary.py:126) — Function test_ingest_empty_and_break_only_streams takes no inputs beyond the value it is called on, and gives back nothi
- function `test_phokenize_bytes_matches_phokenize` (tests/test_binary.py:134) — Function test_phokenize_bytes_matches_phokenize takes no inputs beyond the value it is called on, and gives back nothing
- function `test_phokenize_bytes_matches_literal_reference_on_random_streams` (tests/test_binary.py:182) — Function test_phokenize_bytes_matches_literal_reference_on_random_streams takes no inputs beyond the value it is called 
- function `coord` (tests/test_chain.py:12) — Function coord takes address and gives back nothing; it does its work on what it is given.
- function `roundtrip` (tests/test_chain.py:16) — Function roundtrip takes stream and gives back nothing; it does its work on what it is given.
- function `test_chain_free_input_is_one_unclosed_chain` (tests/test_chain.py:21) — Function test_chain_free_input_is_one_unclosed_chain takes no inputs beyond the value it is called on, and gives back no
- function `test_two_phexts_chained_and_closed_round_trip` (tests/test_chain.py:38) — Function test_two_phexts_chained_and_closed_round_trip takes no inputs beyond the value it is called on, and gives back 
- function `test_unclosed_chain_at_eof_reports_truncated` (tests/test_chain.py:47) — Function test_unclosed_chain_at_eof_reports_truncated takes no inputs beyond the value it is called on, and gives back n
- function `test_two_chains_in_one_stream_parse_as_two_units` (tests/test_chain.py:55) — Function test_two_chains_in_one_stream_parse_as_two_units takes no inputs beyond the value it is called on, and gives ba
- function `test_chained_members_carry_full_phexts` (tests/test_chain.py:75) — Function test_chained_members_carry_full_phexts takes no inputs beyond the value it is called on, and gives back nothing
- function `test_editions_in_succession_read_in_order` (tests/test_chain.py:90) — Function test_editions_in_succession_read_in_order takes no inputs beyond the value it is called on, and gives back noth
- function `test_empty_and_break_only_streams_round_trip` (tests/test_chain.py:100) — Function test_empty_and_break_only_streams_round_trip takes no inputs beyond the value it is called on, and gives back n
- function `test_serialize_from_constructed_chains` (tests/test_chain.py:132) — Function test_serialize_from_constructed_chains takes no inputs beyond the value it is called on, and gives back nothing
- function `test_chain_layer_does_not_touch_member_structure` (tests/test_chain.py:144) — Function test_chain_layer_does_not_touch_member_structure takes no inputs beyond the value it is called on, and gives ba
- function `test_multiverse_addressing` (tests/test_chain.py:153) — Function test_multiverse_addressing takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `test_adjacent_closes_are_two_empty_chains` (tests/test_chain.py:167) — Function test_adjacent_closes_are_two_empty_chains takes no inputs beyond the value it is called on, and gives back noth
- function `test_chain_fixture_case` (tests/test_chain_fixtures.py:28) — Function test_chain_fixture_case takes case and gives back nothing; it does its work on what it is given.
- function `test_valid_home_coordinate` (tests/test_coordinate.py:17) — Function test_valid_home_coordinate takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `test_independent_coordinates` (tests/test_coordinate.py:29) — Function test_independent_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does 
- function `test_large_coordinates` (tests/test_coordinate.py:41) — Function test_large_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its wo
- function `test_whitespace_support` (tests/test_coordinate.py:53) — Function test_whitespace_support takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `test_internal_whitespace_support` (tests/test_coordinate.py:65) — Function test_internal_whitespace_support takes no inputs beyond the value it is called on, and gives back nothing; it d
- function `test_to_urlencoded` (tests/test_coordinate.py:77) — Function test_to_urlencoded takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_library_break` (tests/test_coordinate.py:81) — Function test_library_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_shelf_break` (tests/test_coordinate.py:87) — Function test_shelf_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_series_break` (tests/test_coordinate.py:93) — Function test_series_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_collection_break` (tests/test_coordinate.py:99) — Function test_collection_break takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `test_volume_break` (tests/test_coordinate.py:105) — Function test_volume_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_book_break` (tests/test_coordinate.py:111) — Function test_book_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `test_chapter_break` (tests/test_coordinate.py:117) — Function test_chapter_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_section_break` (tests/test_coordinate.py:123) — Function test_section_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_scroll_break` (tests/test_coordinate.py:129) — Function test_scroll_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_breaks_are_unbounded` (tests/test_coordinate.py:135) — Function test_breaks_are_unbounded takes no inputs beyond the value it is called on, and gives back nothing; it does its
- function `test_phokenize_scroll_unbounded` (tests/test_coordinate.py:147) — Function test_phokenize_scroll_unbounded takes no inputs beyond the value it is called on, and gives back nothing; it do
- function `test_no_input` (tests/test_coordinate.py:154) — Function test_no_input takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `test_very_large_coordinates_round_trip` (tests/test_coordinate.py:158) — Function test_very_large_coordinates_round_trip takes no inputs beyond the value it is called on, and gives back nothing
- function `test_negative_coordinates` (tests/test_coordinate.py:174) — Function test_negative_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its
- function `test_malformed_coordinates` (tests/test_coordinate.py:178) — Function test_malformed_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `test_null_coordinates` (tests/test_coordinate.py:182) — Function test_null_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `test_null_library` (tests/test_coordinate.py:186) — Function test_null_library takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_null_shelf` (tests/test_coordinate.py:190) — Function test_null_shelf takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `test_null_series` (tests/test_coordinate.py:194) — Function test_null_series takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_null_collection` (tests/test_coordinate.py:198) — Function test_null_collection takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `test_null_volume` (tests/test_coordinate.py:202) — Function test_null_volume takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_null_book` (tests/test_coordinate.py:206) — Function test_null_book takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `test_null_chapter` (tests/test_coordinate.py:210) — Function test_null_chapter takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_null_section` (tests/test_coordinate.py:214) — Function test_null_section takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_null_scroll` (tests/test_coordinate.py:218) — Function test_null_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `coord` (tests/test_indexed.py:26) — Function coord takes text and gives back nothing; it does its work on what it is given.
- function `assert_equivalent` (tests/test_indexed.py:30) — Function assert_equivalent takes buffer and gives back nothing; it does its work on what it is given.
- function `test_index_offsets_match_iter_scrolls` (tests/test_indexed.py:61) — Function test_index_offsets_match_iter_scrolls takes no inputs beyond the value it is called on, and gives back nothing;
- function `test_indexed_fetch_matches_fetch_everywhere` (tests/test_indexed.py:75) — Function test_indexed_fetch_matches_fetch_everywhere takes no inputs beyond the value it is called on, and gives back no
- function `test_indexed_explode_matches_explode` (tests/test_indexed.py:81) — Function test_indexed_explode_matches_explode takes no inputs beyond the value it is called on, and gives back nothing; 
- function `test_indexed_generated_document` (tests/test_indexed.py:87) — Function test_indexed_generated_document takes no inputs beyond the value it is called on, and gives back nothing; it do
- function `test_indexed_empty_and_single_scroll` (tests/test_indexed.py:107) — Function test_indexed_empty_and_single_scroll takes no inputs beyond the value it is called on, and gives back nothing; 
- function `test_indexed_unbounded_matches_fetch` (tests/test_indexed.py:120) — Function test_indexed_unbounded_matches_fetch takes no inputs beyond the value it is called on, and gives back nothing; 
- function `test_indexed_fetch_beyond_999_chapter` (tests/test_indexed.py:151) — Function test_indexed_fetch_beyond_999_chapter takes no inputs beyond the value it is called on, and gives back nothing;
- function `test_index_offsets_built_once` (tests/test_indexed.py:170) — Function test_index_offsets_built_once takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `counting` (tests/test_indexed.py:179) — Function counting takes self, buf and gives back nothing; it does its work on what it is given.
- function `test_phext_indexed_convenience_constructor` (tests/test_indexed.py:195) — Function test_phext_indexed_convenience_constructor takes no inputs beyond the value it is called on, and gives back not
- function `test_dead_reckoning` (tests/test_phext.py:38) — Function test_dead_reckoning takes no inputs beyond the value it is called on, and gives back nothing; it does its work 
- function `test_line_break` (tests/test_phext.py:124) — Function test_line_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `test_more_cowbell` (tests/test_phext.py:127) — Function test_more_cowbell takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_coordinate_based_insert` (tests/test_phext.py:130) — Function test_coordinate_based_insert takes no inputs beyond the value it is called on, and gives back nothing; it does 
- function `test_coordinate_based_replace` (tests/test_phext.py:248) — Function test_coordinate_based_replace takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `test_coordinate_based_remove` (tests/test_phext.py:345) — Function test_coordinate_based_remove takes no inputs beyond the value it is called on, and gives back nothing; it does 
- function `test_range_based_replace` (tests/test_phext.py:398) — Function test_range_based_replace takes no inputs beyond the value it is called on, and gives back nothing; it does its 
- function `test_next_scroll` (tests/test_phext.py:415) — Function test_next_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_last_empty_scroll` (tests/test_phext.py:425) — Function test_last_empty_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its wo
- function `test_merge` (tests/test_phext.py:438) — Function test_merge takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `test_subtract` (tests/test_phext.py:483) — Function test_subtract takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `test_normalize` (tests/test_phext.py:490) — Function test_normalize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `test_expand` (tests/test_phext.py:496) — Function test_expand takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `test_contract` (tests/test_phext.py:529) — Function test_contract takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `test_fs_read_write` (tests/test_phext.py:538) — Function test_fs_read_write takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_replace_create` (tests/test_phext.py:553) — Function test_replace_create takes no inputs beyond the value it is called on, and gives back nothing; it does its work 
- function `test_summary` (tests/test_phext.py:560) — Function test_summary takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `test_navmap` (tests/test_phext.py:570) — Function test_navmap takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `test_textmap` (tests/test_phext.py:576) — Function test_textmap takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `test_larger_coordinates` (tests/test_phext.py:582) — Function test_larger_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `test_phext_index` (tests/test_phext.py:590) — Function test_phext_index takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `test_scroll_manifest` (tests/test_phext.py:642) — Function test_scroll_manifest takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `test_phext_soundex_v1` (tests/test_phext.py:694) — Function test_phext_soundex_v1 takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `test_insert_performance_2k_scrolls` (tests/test_phext.py:700) — Function test_insert_performance_2k_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it
- function `test_hash_support` (tests/test_phext.py:829) — Function test_hash_support takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_phext_breaks` (tests/test_phext.py:893) — Function test_phext_breaks takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_normalize` (tests/test_phext.py:916) — Function test_normalize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `random_document` (tests/test_phokenize.py:45) — Function random_document takes rng: random.Random and gives back str:.
- function `random_coordinate` (tests/test_phokenize.py:61) — Function random_coordinate takes rng: random.Random and gives back Coordinate:.
- function `outcome` (tests/test_phokenize.py:64) — Function outcome takes fn, *args and gives back nothing; it does its work on what it is given.
- function `corpus` (tests/test_phokenize.py:71) — Function corpus takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is
- function `as_pairs` (tests/test_phokenize.py:80) — Function as_pairs takes stack and gives back nothing; it does its work on what it is given.
- function `test_phokenize_matches_legacy` (tests/test_phokenize.py:83) — Function test_phokenize_matches_legacy takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `test_phokenize_coordinates_are_distinct_objects` (tests/test_phokenize.py:89) — Function test_phokenize_coordinates_are_distinct_objects takes no inputs beyond the value it is called on, and gives bac
- function `test_serialization_matches_legacy` (tests/test_phokenize.py:96) — Function test_serialization_matches_legacy takes no inputs beyond the value it is called on, and gives back nothing; it 
- function `test_explode_implode_round_trip` (tests/test_phokenize.py:107) — Function test_explode_implode_round_trip takes no inputs beyond the value it is called on, and gives back nothing; it do
- function `test_maps_and_index_match_legacy` (tests/test_phokenize.py:123) — Function test_maps_and_index_match_legacy takes no inputs beyond the value it is called on, and gives back nothing; it d
- function `test_lookups_match_legacy` (tests/test_phokenize.py:135) — Function test_lookups_match_legacy takes no inputs beyond the value it is called on, and gives back nothing; it does its
- function `test_edits_match_legacy` (tests/test_phokenize.py:153) — Function test_edits_match_legacy takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `test_append_scroll_matches_legacy` (tests/test_phokenize.py:170) — Function test_append_scroll_matches_legacy takes no inputs beyond the value it is called on, and gives back nothing; it 
- function `test_append_scroll_out_of_order` (tests/test_phokenize.py:187) — Function test_append_scroll_out_of_order takes no inputs beyond the value it is called on, and gives back nothing; it do
- function `large_document` (tests/test_phokenize.py:195) — Function large_document takes scrolls: int and gives back str:.
- function `test_explode_performance_50k_scrolls` (tests/test_phokenize.py:207) — Function test_explode_performance_50k_scrolls takes no inputs beyond the value it is called on, and gives back nothing; 
- function `test_scroll_interface` (tests/test_positionedScroll.py:12) — Function test_scroll_interface takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `test_positioned_scrolls` (tests/test_positionedScroll.py:19) — Function test_positioned_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `test_phokenize` (tests/test_positionedScroll.py:39) — Function test_phokenize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `test_range_exists` (tests/test_range.py:5) — Function test_range_exists takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `test_range_fixture_case` (tests/test_range_fixtures.py:55) — Function test_range_fixture_case takes case_name and gives back nothing; it does its work on what it is given.
- function `test_seal_round_trip_real_phext` (tests/test_seal.py:23) — Function test_seal_round_trip_real_phext takes no inputs beyond the value it is called on, and gives back nothing; it do
- function `test_seal_round_trip_empty_and_small` (tests/test_seal.py:33) — Function test_seal_round_trip_empty_and_small takes no inputs beyond the value it is called on, and gives back nothing; 
- function `test_seal_carrying_binary_scrolls` (tests/test_seal.py:41) — Function test_seal_carrying_binary_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it 
- function `test_seal_is_deterministic` (tests/test_seal.py:50) — Function test_seal_is_deterministic takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `test_unseal_rejects_non_xz_data` (tests/test_seal.py:57) — Function test_unseal_rejects_non_xz_data takes no inputs beyond the value it is called on, and gives back nothing; it do

---

## function — __str__

Found in phext/addressing.py at line 66.

Signature: `def __str__(self) -> str:`

What it does: Function __str__ takes no inputs beyond the value it is called on, and gives back str:.

(Where these words came from: signature.)

---

## function — __str__

Found in phext/addressing.py at line 86.

Signature: `def __str__(self) -> str:`

What it does: Function __str__ takes no inputs beyond the value it is called on, and gives back str:.

(Where these words came from: signature.)

---

## function — parse_point_address

Found in phext/addressing.py at line 119.

Signature: `def parse_point_address(address: str):`

What it does: Parses a v0.6.1 point address:

(Where these words came from: doc comment.)

---

## function — parse_range_address

Found in phext/addressing.py at line 190.

Signature: `def parse_range_address(address: str):`

What it does: Parses a v0.6.1 range address: exactly two point addresses joined

(Where these words came from: doc comment.)

---

## function — resolve_scroll_offset

Found in phext/addressing.py at line 212.

Signature: `def resolve_scroll_offset(text: str, pos: ScrollPosition):`

What it does: Resolves a caret position inside one scroll to a character

(Where these words came from: doc comment.)

---

## function — enumerate_scrolls

Found in phext/addressing.py at line 235.

Signature: `def enumerate_scrolls(member: str):`

What it does: The member's scrolls in the phokenize sense: a list of

(Where these words came from: doc comment.)

---

## function — addressed_scroll_text

Found in phext/addressing.py at line 245.

Signature: `def addressed_scroll_text(member: str, target: Coordinate):`

What it does: The scroll text at a coordinate in one member phext, in the

(Where these words came from: doc comment.)

---

## function — is_binary_scroll

Found in phext/binary.py at line 60.

Signature: `def is_binary_scroll(scroll: str) -> bool:`

What it does: True when `scroll` carries the binary scroll header. Decoding

(Where these words came from: doc comment.)

---

## function — encode_scroll

Found in phext/binary.py at line 66.

Signature: `def encode_scroll(payload: bytes) -> str:`

What it does: Encodes raw bytes as a binary scroll (header line + base64 body).

(Where these words came from: doc comment.)

---

## function — decode_scroll

Found in phext/binary.py at line 71.

Signature: `def decode_scroll(scroll: str):`

What it does: Decodes a binary scroll back to its raw bytes. Returns None when

(Where these words came from: doc comment.)

---

## function — insert_binary

Found in phext/binary.py at line 84.

Signature: `def insert_binary(buffer: str, location: Coordinate, payload: bytes) -> str:`

What it does: Stores `payload` at `location` as a binary scroll, unconditionally:

(Where these words came from: doc comment.)

---

## function — replace_binary

Found in phext/binary.py at line 91.

Signature: `def replace_binary(buffer: str, location: Coordinate, payload: bytes) -> str:`

What it does: Replaces the scroll at `location` with `payload`, stored as a

(Where these words came from: doc comment.)

---

## function — fetch_binary

Found in phext/binary.py at line 97.

Signature: `def fetch_binary(buffer: str, target: Coordinate) -> bytes:`

What it does: Retrieves the bytes held at `target`. A well-formed binary scroll

(Where these words came from: doc comment.)

---

## function — insert_bytes

Found in phext/binary.py at line 120.

Signature: `def insert_bytes(buffer: str, location: Coordinate, payload: bytes) -> str:`

What it does: Byte-oriented insert: stores `payload` at `location` under the

(Where these words came from: doc comment.)

---

## function — replace_bytes

Found in phext/binary.py at line 127.

Signature: `def replace_bytes(buffer: str, location: Coordinate, payload: bytes) -> str:`

What it does: Byte-oriented replace: stores `payload` at `location` under the

(Where these words came from: doc comment.)

---

## function — phokenize_bytes

Found in phext/binary.py at line 141.

Signature: `def phokenize_bytes(data: bytes):`

What it does: The byte-level twin of the reference `phokenize`: walks a raw byte

(Where these words came from: doc comment.)

---

## function — ingest

Found in phext/binary.py at line 172.

Signature: `def ingest(data: bytes) -> str:`

What it does: Ingests an externally supplied phext byte stream. Every scroll is

(Where these words came from: doc comment.)

---

## function — serialize

Found in phext/chain.py at line 50.

Signature: `def serialize(self) -> str:`

What it does: Serializes this chain alone: its members joined by

(Where these words came from: doc comment.)

---

## function — fetch

Found in phext/chain.py at line 59.

Signature: `def fetch(self, address: str) -> str:`

What it does: Fetch a scroll by multiverse address: ``$N:coord``, or a bare

(Where these words came from: doc comment.)

---

## function — resolve_point

Found in phext/chain.py at line 72.

Signature: `def resolve_point(self, point):`

What it does: Resolves a point address against this chain: (universe,

(Where these words came from: doc comment.)

---

## function — fetch_point

Found in phext/chain.py at line 93.

Signature: `def fetch_point(self, address: str) -> str:`

What it does: Fetches a point address string. A bare point is a caret: it

(Where these words came from: doc comment.)

---

## function — resolve_range

Found in phext/chain.py at line 116.

Signature: `def resolve_range(self, range_address):`

What it does: Resolves a range against this chain into per-scroll segments:

(Where these words came from: doc comment.)

---

## function — fetch_range

Found in phext/chain.py at line 170.

Signature: `def fetch_range(self, address: str) -> str:`

What it does: Fetches a range address string: the spanned text with phext

(Where these words came from: doc comment.)

---

## function — parse_chains

Found in phext/chain.py at line 197.

Signature: `def parse_chains(stream: str):`

What it does: Splits a stream into its chains. CHAIN_BREAK separates the member

(Where these words came from: doc comment.)

---

## function — serialize_chains

Found in phext/chain.py at line 221.

Signature: `def serialize_chains(chains) -> str:`

What it does: Serializes a stream of chains back into stream bytes: the inverse

(Where these words came from: doc comment.)

---

## function — parse_universe_address

Found in phext/chain.py at line 227.

Signature: `def parse_universe_address(address: str):`

What it does: Parses a multiverse address into (universe, Coordinate), or None

(Where these words came from: doc comment.)

---

## function — advance_coordinate

Found in phext/coordinate.py at line 4.

Signature: `def advance_coordinate(index: int) -> int:`

What it does: Function advance_coordinate takes index: int and gives back int:.

(Where these words came from: signature.)

---

## function — from_string

Found in phext/coordinate.py at line 29.

Signature: `def from_string(ignored, coord_str: str):`

What it does: Function from_string takes ignored, coord_str: str and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — libraryBreak

Found in phext/coordinate.py at line 50.

Signature: `def libraryBreak(self):`

What it does: Function libraryBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — shelfBreak

Found in phext/coordinate.py at line 61.

Signature: `def shelfBreak(self):`

What it does: Function shelfBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — seriesBreak

Found in phext/coordinate.py at line 71.

Signature: `def seriesBreak(self):`

What it does: Function seriesBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — collectionBreak

Found in phext/coordinate.py at line 80.

Signature: `def collectionBreak(self):`

What it does: Function collectionBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — volumeBreak

Found in phext/coordinate.py at line 88.

Signature: `def volumeBreak(self):`

What it does: Function volumeBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — bookBreak

Found in phext/coordinate.py at line 95.

Signature: `def bookBreak(self):`

What it does: Function bookBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — chapterBreak

Found in phext/coordinate.py at line 101.

Signature: `def chapterBreak(self):`

What it does: Function chapterBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — sectionBreak

Found in phext/coordinate.py at line 106.

Signature: `def sectionBreak(self):`

What it does: Function sectionBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — scrollBreak

Found in phext/coordinate.py at line 110.

Signature: `def scrollBreak(self):`

What it does: Function scrollBreak takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — __init__

Found in phext/coordinate.py at line 113.

Signature: `def __init__(self, lb, sf, sr, cn, vm, bk, ch, sn, sc):`

What it does: Function __init__ takes self, lb, sf, sr, cn, vm, bk, ch, sn, sc and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — copy_coordinate

Found in phext/coordinate.py at line 124.

Signature: `def copy_coordinate(self, other):`

What it does: Function copy_coordinate takes self, other and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — __str__

Found in phext/coordinate.py at line 135.

Signature: `def __str__(self) -> str:`

What it does: Function __str__ takes no inputs beyond the value it is called on, and gives back str:.

(Where these words came from: signature.)

---

## function — urlencoded

Found in phext/coordinate.py at line 140.

Signature: `def urlencoded(self) -> str:`

What it does: Function urlencoded takes no inputs beyond the value it is called on, and gives back str:.

(Where these words came from: signature.)

---

## function — __eq__

Found in phext/coordinate.py at line 145.

Signature: `def __eq__(self, other):`

What it does: Function __eq__ takes self, other and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — __lt__

Found in phext/coordinate.py at line 148.

Signature: `def __lt__(self, other):`

What it does: Function __lt__ takes self, other and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — as_tuple

Found in phext/coordinate.py at line 153.

Signature: `def as_tuple(self) -> tuple:`

What it does: Function as_tuple takes no inputs beyond the value it is called on, and gives back tuple:.

(Where these words came from: signature.)

---

## function — __hash__

Found in phext/coordinate.py at line 160.

Signature: `def __hash__(self):`

What it does: Function __hash__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — __init__

Found in phext/indexed.py at line 31.

Signature: `def __init__(self, buffer: str):`

What it does: Function __init__ takes self, buffer: str and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — fetch

Found in phext/indexed.py at line 42.

Signature: `def fetch(self, coord: Coordinate) -> str:`

What it does: Function fetch takes self, coord: Coordinate and gives back str:.

(Where these words came from: signature.)

---

## function — __contains__

Found in phext/indexed.py at line 49.

Signature: `def __contains__(self, coord: Coordinate) -> bool:`

What it does: Function __contains__ takes self, coord: Coordinate and gives back bool:.

(Where these words came from: signature.)

---

## function — coordinates

Found in phext/indexed.py at line 52.

Signature: `def coordinates(self) -> list[Coordinate]:`

What it does: Function coordinates takes no inputs beyond the value it is called on, and gives back list[Coordinate]:.

(Where these words came from: signature.)

---

## function — keys

Found in phext/indexed.py at line 55.

Signature: `def keys(self) -> list[Coordinate]:`

What it does: Function keys takes no inputs beyond the value it is called on, and gives back list[Coordinate]:.

(Where these words came from: signature.)

---

## function — index

Found in phext/indexed.py at line 58.

Signature: `def index(self) -> dict[Coordinate, tuple[int, int]]:`

What it does: Function index takes no inputs beyond the value it is called on, and gives back dict[Coordinate, tuple[int, int]]:.

(Where these words came from: signature.)

---

## function — explode

Found in phext/indexed.py at line 62.

Signature: `def explode(self) -> dict[Coordinate, str]:`

What it does: Function explode takes no inputs beyond the value it is called on, and gives back dict[Coordinate, str]:.

(Where these words came from: signature.)

---

## function — __len__

Found in phext/indexed.py at line 68.

Signature: `def __len__(self) -> int:`

What it does: Function __len__ takes no inputs beyond the value it is called on, and gives back int:.

(Where these words came from: signature.)

---

## function — defaultCoordinate

Found in phext/phext.py at line 61.

Signature: `def defaultCoordinate(self) -> Coordinate:`

What it does: Function defaultCoordinate takes no inputs beyond the value it is called on, and gives back Coordinate:.

(Where these words came from: signature.)

---

## function — __init__

Found in phext/phext.py at line 64.

Signature: `def __init__(self):`

What it does: Function __init__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — fetch

Found in phext/phext.py at line 67.

Signature: `def fetch(self, buffer:str, coord:Coordinate) -> dict[Coordinate, str]:`

What it does: Function fetch takes self, buffer:str, coord:Coordinate and gives back dict[Coordinate, str]:.

(Where these words came from: signature.)

---

## function — isPhextBreak

Found in phext/phext.py at line 73.

Signature: `def isPhextBreak(self, byte:str):`

What it does: Function isPhextBreak takes self, byte:str and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — phokenize

Found in phext/phext.py at line 111.

Signature: `def phokenize(self, buffer:str) -> List[PositionedScroll]:`

What it does: Function phokenize takes self, buffer:str and gives back List[PositionedScroll]:.

(Where these words came from: signature.)

---

## function — append_scroll

Found in phext/phext.py at line 177.

Signature: `def append_scroll(self, entry: PositionedScroll, location: Coordinate) -> PositionedScroll:`

What it does: Function append_scroll takes self, entry: PositionedScroll, location: Coordinate and gives back PositionedScroll:.

(Where these words came from: signature.)

---

## function — dephokenize

Found in phext/phext.py at line 183.

Signature: `def dephokenize(self, stack: list[PositionedScroll]) -> str:`

What it does: Function dephokenize takes self, stack: list[PositionedScroll] and gives back str:.

(Where these words came from: signature.)

---

## function — normalize

Found in phext/phext.py at line 191.

Signature: `def normalize(self, buffer:str) -> str:`

What it does: Function normalize takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — update

Found in phext/phext.py at line 195.

Signature: `def update(self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool) -> str:`

What it does: Function update takes self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool and gives back str:.

(Where these words came from: signature.)

---

## function — insert

Found in phext/phext.py at line 218.

Signature: `def insert(self, buffer:str, coord:Coordinate, scroll:str) -> str:`

What it does: Function insert takes self, buffer:str, coord:Coordinate, scroll:str and gives back str:.

(Where these words came from: signature.)

---

## function — replace

Found in phext/phext.py at line 221.

Signature: `def replace(self, buffer:str, coord:Coordinate, scroll:str) -> str:`

What it does: Function replace takes self, buffer:str, coord:Coordinate, scroll:str and gives back str:.

(Where these words came from: signature.)

---

## function — remove

Found in phext/phext.py at line 224.

Signature: `def remove(self, buffer:str, coord:Coordinate) -> str:`

What it does: Function remove takes self, buffer:str, coord:Coordinate and gives back str:.

(Where these words came from: signature.)

---

## function — range_replace

Found in phext/phext.py at line 228.

Signature: `def range_replace(self, buffer:str, range:Range, text:str) -> str:`

What it does: Function range_replace takes self, buffer:str, range:Range, text:str and gives back str:.

(Where these words came from: signature.)

---

## function — next_scroll

Found in phext/phext.py at line 244.

Signature: `def next_scroll(self, buffer:str, coord:Coordinate) -> List[PositionedScroll]:`

What it does: Function next_scroll takes self, buffer:str, coord:Coordinate and gives back List[PositionedScroll]:.

(Where these words came from: signature.)

---

## function — get_subspace_coordinates

Found in phext/phext.py at line 256.

Signature: `def get_subspace_coordinates(self, buffer:str, target:Coordinate):`

What it does: Function get_subspace_coordinates takes self, buffer:str, target:Coordinate and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — merge

Found in phext/phext.py at line 310.

Signature: `def merge(self, left: str, right:str) -> str:`

What it does: Function merge takes self, left: str, right:str and gives back str:.

(Where these words came from: signature.)

---

## function — subtract

Found in phext/phext.py at line 342.

Signature: `def subtract(self, left:str, right:str) -> str:`

What it does: Function subtract takes self, left:str, right:str and gives back str:.

(Where these words came from: signature.)

---

## function — expand

Found in phext/phext.py at line 363.

Signature: `def expand(self, buffer:str) -> str:`

What it does: Function expand takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — contract

Found in phext/phext.py at line 366.

Signature: `def contract(self, buffer:str) -> str:`

What it does: Function contract takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — create_summary

Found in phext/phext.py at line 369.

Signature: `def create_summary(self, buffer:str) -> str:`

What it does: Function create_summary takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — navmap

Found in phext/phext.py at line 377.

Signature: `def navmap(self, urlbase:str, buffer:str) -> str:`

What it does: Function navmap takes self, urlbase:str, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — textmap

Found in phext/phext.py at line 391.

Signature: `def textmap(self, buffer:str) -> str:`

What it does: Function textmap takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — checksum

Found in phext/phext.py at line 398.

Signature: `def checksum(self, buffer:str) -> str:`

What it does: Function checksum takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — manifest

Found in phext/phext.py at line 404.

Signature: `def manifest(self, buffer:str) -> str:`

What it does: Function manifest takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — soundex_v1

Found in phext/phext.py at line 411.

Signature: `def soundex_v1(self, buffer:str) -> str:`

What it does: Function soundex_v1 takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — soundex_internal

Found in phext/phext.py at line 419.

Signature: `def soundex_internal(self, buffer:str) -> str:`

What it does: Function soundex_internal takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — explode

Found in phext/phext.py at line 443.

Signature: `def explode(self, buffer:str) -> dict[Coordinate, str]:`

What it does: Function explode takes self, buffer:str and gives back dict[Coordinate, str]:.

(Where these words came from: signature.)

---

## function — implode

Found in phext/phext.py at line 450.

Signature: `def implode(self, hash: dict[Coordinate, str]) -> str:`

What it does: Function implode takes self, hash: dict[Coordinate, str] and gives back str:.

(Where these words came from: signature.)

---

## function — index_phokens

Found in phext/phext.py at line 456.

Signature: `def index_phokens(self, buffer:str) -> list[PositionedScroll]:`

What it does: Function index_phokens takes self, buffer:str and gives back list[PositionedScroll]:.

(Where these words came from: signature.)

---

## function — index

Found in phext/phext.py at line 475.

Signature: `def index(self, buffer:str) -> str:`

What it does: Function index takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — offset

Found in phext/phext.py at line 479.

Signature: `def offset(self, buffer:str, coord:Coordinate) -> int:`

What it does: Function offset takes self, buffer:str, coord:Coordinate and gives back int:.

(Where these words came from: signature.)

---

## function — index_offsets

Found in phext/phext.py at line 494.

Signature: `def index_offsets(self, buffer:str) -> dict[Coordinate, tuple[int, int]]:`

What it does: Function index_offsets takes self, buffer:str and gives back dict[Coordinate, tuple[int, int]]:.

(Where these words came from: signature.)

---

## function — indexed

Found in phext/phext.py at line 507.

Signature: `def indexed(self, buffer:str):`

What it does: Function indexed takes self, buffer:str and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — insert_binary

Found in phext/phext.py at line 519.

Signature: `def insert_binary(self, buffer: str, coord: Coordinate, payload: bytes) -> str:`

What it does: Function insert_binary takes self, buffer: str, coord: Coordinate, payload: bytes and gives back str:.

(Where these words came from: signature.)

---

## function — replace_binary

Found in phext/phext.py at line 523.

Signature: `def replace_binary(self, buffer: str, coord: Coordinate, payload: bytes) -> str:`

What it does: Function replace_binary takes self, buffer: str, coord: Coordinate, payload: bytes and gives back str:.

(Where these words came from: signature.)

---

## function — fetch_binary

Found in phext/phext.py at line 527.

Signature: `def fetch_binary(self, buffer: str, coord: Coordinate) -> bytes:`

What it does: Function fetch_binary takes self, buffer: str, coord: Coordinate and gives back bytes:.

(Where these words came from: signature.)

---

## function — insert_bytes

Found in phext/phext.py at line 531.

Signature: `def insert_bytes(self, buffer: str, coord: Coordinate, payload: bytes) -> str:`

What it does: Function insert_bytes takes self, buffer: str, coord: Coordinate, payload: bytes and gives back str:.

(Where these words came from: signature.)

---

## function — replace_bytes

Found in phext/phext.py at line 535.

Signature: `def replace_bytes(self, buffer: str, coord: Coordinate, payload: bytes) -> str:`

What it does: Function replace_bytes takes self, buffer: str, coord: Coordinate, payload: bytes and gives back str:.

(Where these words came from: signature.)

---

## function — __init__

Found in phext/positionedScroll.py at line 9.

Signature: `def __init__(self, coord, text):`

What it does: Function __init__ takes self, coord, text and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — seal

Found in phext/seal.py at line 26.

Signature: `def seal(text: str) -> bytes:`

What it does: Seals a phext: xz-compresses its full UTF-8 bytes.

(Where these words came from: doc comment.)

---

## function — seal_bytes

Found in phext/seal.py at line 31.

Signature: `def seal_bytes(data: bytes) -> bytes:`

What it does: Seals raw phext bytes (see `seal`).

(Where these words came from: doc comment.)

---

## function — unseal

Found in phext/seal.py at line 36.

Signature: `def unseal(data: bytes) -> bytes:`

What it does: Unseals a sealed phext back to its exact original bytes. Raises

(Where these words came from: doc comment.)

---

## function — unseal_string

Found in phext/seal.py at line 42.

Signature: `def unseal_string(data: bytes) -> str:`

What it does: Unseals directly to text. Raises when the unsealed bytes are not

(Where these words came from: doc comment.)

---

## function — timed

Found in test/bench.py at line 17.

Signature: `def timed(label, fn):`

What it does: Function timed takes label, fn and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in test/bench.py at line 24.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — defaultCoordinate

Found in tests/legacy_phext.py at line 38.

Signature: `def defaultCoordinate(self) -> Coordinate:`

What it does: Function defaultCoordinate takes no inputs beyond the value it is called on, and gives back Coordinate:.

(Where these words came from: signature.)

---

## function — __init__

Found in tests/legacy_phext.py at line 41.

Signature: `def __init__(self):`

What it does: Function __init__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — fetch

Found in tests/legacy_phext.py at line 44.

Signature: `def fetch(self, buffer:str, coord:Coordinate) -> dict[Coordinate, str]:`

What it does: Function fetch takes self, buffer:str, coord:Coordinate and gives back dict[Coordinate, str]:.

(Where these words came from: signature.)

---

## function — isPhextBreak

Found in tests/legacy_phext.py at line 51.

Signature: `def isPhextBreak(self, byte:str):`

What it does: Function isPhextBreak takes self, byte:str and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — phokenize

Found in tests/legacy_phext.py at line 63.

Signature: `def phokenize(self, buffer:str) -> List[PositionedScroll]:`

What it does: Function phokenize takes self, buffer:str and gives back List[PositionedScroll]:.

(Where these words came from: signature.)

---

## function — append_scroll

Found in tests/legacy_phext.py at line 114.

Signature: `def append_scroll(self, entry: PositionedScroll, location: Coordinate) -> PositionedScroll:`

What it does: Function append_scroll takes self, entry: PositionedScroll, location: Coordinate and gives back PositionedScroll:.

(Where these words came from: signature.)

---

## function — dephokenize

Found in tests/legacy_phext.py at line 159.

Signature: `def dephokenize(self, stack: list[PositionedScroll]) -> str:`

What it does: Function dephokenize takes self, stack: list[PositionedScroll] and gives back str:.

(Where these words came from: signature.)

---

## function — normalize

Found in tests/legacy_phext.py at line 168.

Signature: `def normalize(self, buffer:str) -> str:`

What it does: Function normalize takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — update

Found in tests/legacy_phext.py at line 172.

Signature: `def update(self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool) -> str:`

What it does: Function update takes self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool and gives back str:.

(Where these words came from: signature.)

---

## function — insert

Found in tests/legacy_phext.py at line 195.

Signature: `def insert(self, buffer:str, coord:Coordinate, scroll:str) -> str:`

What it does: Function insert takes self, buffer:str, coord:Coordinate, scroll:str and gives back str:.

(Where these words came from: signature.)

---

## function — replace

Found in tests/legacy_phext.py at line 198.

Signature: `def replace(self, buffer:str, coord:Coordinate, scroll:str) -> str:`

What it does: Function replace takes self, buffer:str, coord:Coordinate, scroll:str and gives back str:.

(Where these words came from: signature.)

---

## function — remove

Found in tests/legacy_phext.py at line 201.

Signature: `def remove(self, buffer:str, coord:Coordinate) -> str:`

What it does: Function remove takes self, buffer:str, coord:Coordinate and gives back str:.

(Where these words came from: signature.)

---

## function — range_replace

Found in tests/legacy_phext.py at line 205.

Signature: `def range_replace(self, buffer:str, range:Range, text:str) -> str:`

What it does: Function range_replace takes self, buffer:str, range:Range, text:str and gives back str:.

(Where these words came from: signature.)

---

## function — next_scroll

Found in tests/legacy_phext.py at line 221.

Signature: `def next_scroll(self, buffer:str, coord:Coordinate) -> List[PositionedScroll]:`

What it does: Function next_scroll takes self, buffer:str, coord:Coordinate and gives back List[PositionedScroll]:.

(Where these words came from: signature.)

---

## function — get_subspace_coordinates

Found in tests/legacy_phext.py at line 234.

Signature: `def get_subspace_coordinates(self, buffer:str, target:Coordinate):`

What it does: Function get_subspace_coordinates takes self, buffer:str, target:Coordinate and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — merge

Found in tests/legacy_phext.py at line 297.

Signature: `def merge(self, left: str, right:str) -> str:`

What it does: Function merge takes self, left: str, right:str and gives back str:.

(Where these words came from: signature.)

---

## function — subtract

Found in tests/legacy_phext.py at line 329.

Signature: `def subtract(self, left:str, right:str) -> str:`

What it does: Function subtract takes self, left:str, right:str and gives back str:.

(Where these words came from: signature.)

---

## function — expand

Found in tests/legacy_phext.py at line 350.

Signature: `def expand(self, buffer:str) -> str:`

What it does: Function expand takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — contract

Found in tests/legacy_phext.py at line 375.

Signature: `def contract(self, buffer:str) -> str:`

What it does: Function contract takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — create_summary

Found in tests/legacy_phext.py at line 400.

Signature: `def create_summary(self, buffer:str) -> str:`

What it does: Function create_summary takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — navmap

Found in tests/legacy_phext.py at line 414.

Signature: `def navmap(self, urlbase:str, buffer:str) -> str:`

What it does: Function navmap takes self, urlbase:str, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — textmap

Found in tests/legacy_phext.py at line 428.

Signature: `def textmap(self, buffer:str) -> str:`

What it does: Function textmap takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — checksum

Found in tests/legacy_phext.py at line 436.

Signature: `def checksum(self, buffer:str) -> str:`

What it does: Function checksum takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — manifest

Found in tests/legacy_phext.py at line 440.

Signature: `def manifest(self, buffer:str) -> str:`

What it does: Function manifest takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — soundex_v1

Found in tests/legacy_phext.py at line 447.

Signature: `def soundex_v1(self, buffer:str) -> str:`

What it does: Function soundex_v1 takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — soundex_internal

Found in tests/legacy_phext.py at line 455.

Signature: `def soundex_internal(self, buffer:str) -> str:`

What it does: Function soundex_internal takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — explode

Found in tests/legacy_phext.py at line 479.

Signature: `def explode(self, buffer:str) -> dict[Coordinate, str]:`

What it does: Function explode takes self, buffer:str and gives back dict[Coordinate, str]:.

(Where these words came from: signature.)

---

## function — implode

Found in tests/legacy_phext.py at line 486.

Signature: `def implode(self, hash: dict[Coordinate, str]) -> str:`

What it does: Function implode takes self, hash: dict[Coordinate, str] and gives back str:.

(Where these words came from: signature.)

---

## function — index_phokens

Found in tests/legacy_phext.py at line 492.

Signature: `def index_phokens(self, buffer:str) -> list[PositionedScroll]:`

What it does: Function index_phokens takes self, buffer:str and gives back list[PositionedScroll]:.

(Where these words came from: signature.)

---

## function — index

Found in tests/legacy_phext.py at line 532.

Signature: `def index(self, buffer:str) -> str:`

What it does: Function index takes self, buffer:str and gives back str:.

(Where these words came from: signature.)

---

## function — offset

Found in tests/legacy_phext.py at line 536.

Signature: `def offset(self, buffer:str, coord:Coordinate) -> int:`

What it does: Function offset takes self, buffer:str, coord:Coordinate and gives back int:.

(Where these words came from: signature.)

---

## function — home

Found in tests/test_address.py at line 15.

Signature: `def home():`

What it does: Function home takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — scroll2

Found in tests/test_address.py at line 19.

Signature: `def scroll2():`

What it does: Function scroll2 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — member_a

Found in tests/test_address.py at line 23.

Signature: `def member_a():`

What it does: Function member_a takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — member_b

Found in tests/test_address.py at line 29.

Signature: `def member_b():`

What it does: Function member_b takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — make_chain

Found in tests/test_address.py at line 33.

Signature: `def make_chain():`

What it does: Function make_chain takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_point_parse_canonical

Found in tests/test_address.py at line 37.

Signature: `def test_point_parse_canonical():`

What it does: Function test_point_parse_canonical takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_point_parse_rejects

Found in tests/test_address.py at line 60.

Signature: `def test_point_parse_rejects():`

What it does: Function test_point_parse_rejects takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_point_resolve

Found in tests/test_address.py at line 77.

Signature: `def test_point_resolve():`

What it does: Function test_point_resolve takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_point_fetch_offset

Found in tests/test_address.py at line 95.

Signature: `def test_point_fetch_offset():`

What it does: Function test_point_fetch_offset takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_point_fetch_offset_unicode

Found in tests/test_address.py at line 110.

Signature: `def test_point_fetch_offset_unicode():`

What it does: Function test_point_fetch_offset_unicode takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_range_parse_canonical

Found in tests/test_address.py at line 118.

Signature: `def test_range_parse_canonical():`

What it does: Function test_range_parse_canonical takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_range_fetch_within_scroll

Found in tests/test_address.py at line 133.

Signature: `def test_range_fetch_within_scroll():`

What it does: Function test_range_fetch_within_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_range_fetch_cross_scroll

Found in tests/test_address.py at line 145.

Signature: `def test_range_fetch_cross_scroll():`

What it does: Function test_range_fetch_cross_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_range_fetch_multiverse

Found in tests/test_address.py at line 157.

Signature: `def test_range_fetch_multiverse():`

What it does: Function test_range_fetch_multiverse takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_range_resolution_errors

Found in tests/test_address.py at line 176.

Signature: `def test_range_resolution_errors():`

What it does: Function test_range_resolution_errors takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — coord

Found in tests/test_binary.py at line 11.

Signature: `def coord(address):`

What it does: Function coord takes address and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_encode_decode_round_trip

Found in tests/test_binary.py at line 15.

Signature: `def test_encode_decode_round_trip():`

What it does: Function test_encode_decode_round_trip takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_decode_scroll_rejects_non_binary

Found in tests/test_binary.py at line 39.

Signature: `def test_decode_scroll_rejects_non_binary():`

What it does: Function test_decode_scroll_rejects_non_binary takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_insert_and_fetch_binary

Found in tests/test_binary.py at line 51.

Signature: `def test_insert_and_fetch_binary():`

What it does: Function test_insert_and_fetch_binary takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_fetch_binary_fallbacks

Found in tests/test_binary.py at line 68.

Signature: `def test_fetch_binary_fallbacks():`

What it does: Function test_fetch_binary_fallbacks takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_update_rule_non_utf8_is_base64_verbatim

Found in tests/test_binary.py at line 80.

Signature: `def test_update_rule_non_utf8_is_base64_verbatim():`

What it does: Function test_update_rule_non_utf8_is_base64_verbatim takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_update_rule_utf8_stays_text

Found in tests/test_binary.py at line 97.

Signature: `def test_update_rule_utf8_stays_text():`

What it does: Function test_update_rule_utf8_stays_text takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_ingest_foreign_byte_stream

Found in tests/test_binary.py at line 105.

Signature: `def test_ingest_foreign_byte_stream():`

What it does: Function test_ingest_foreign_byte_stream takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_ingest_canonical_phext_is_identity

Found in tests/test_binary.py at line 119.

Signature: `def test_ingest_canonical_phext_is_identity():`

What it does: Function test_ingest_canonical_phext_is_identity takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_ingest_empty_and_break_only_streams

Found in tests/test_binary.py at line 126.

Signature: `def test_ingest_empty_and_break_only_streams():`

What it does: Function test_ingest_empty_and_break_only_streams takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_phokenize_bytes_matches_phokenize

Found in tests/test_binary.py at line 134.

Signature: `def test_phokenize_bytes_matches_phokenize():`

What it does: Function test_phokenize_bytes_matches_phokenize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_phokenize_bytes_matches_literal_reference_on_random_streams

Found in tests/test_binary.py at line 182.

Signature: `def test_phokenize_bytes_matches_literal_reference_on_random_streams():`

What it does: Function test_phokenize_bytes_matches_literal_reference_on_random_streams takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — coord

Found in tests/test_chain.py at line 12.

Signature: `def coord(address):`

What it does: Function coord takes address and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — roundtrip

Found in tests/test_chain.py at line 16.

Signature: `def roundtrip(stream):`

What it does: Function roundtrip takes stream and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_chain_free_input_is_one_unclosed_chain

Found in tests/test_chain.py at line 21.

Signature: `def test_chain_free_input_is_one_unclosed_chain():`

What it does: Function test_chain_free_input_is_one_unclosed_chain takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_two_phexts_chained_and_closed_round_trip

Found in tests/test_chain.py at line 38.

Signature: `def test_two_phexts_chained_and_closed_round_trip():`

What it does: Function test_two_phexts_chained_and_closed_round_trip takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_unclosed_chain_at_eof_reports_truncated

Found in tests/test_chain.py at line 47.

Signature: `def test_unclosed_chain_at_eof_reports_truncated():`

What it does: Function test_unclosed_chain_at_eof_reports_truncated takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_two_chains_in_one_stream_parse_as_two_units

Found in tests/test_chain.py at line 55.

Signature: `def test_two_chains_in_one_stream_parse_as_two_units():`

What it does: Function test_two_chains_in_one_stream_parse_as_two_units takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_chained_members_carry_full_phexts

Found in tests/test_chain.py at line 75.

Signature: `def test_chained_members_carry_full_phexts():`

What it does: Function test_chained_members_carry_full_phexts takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_editions_in_succession_read_in_order

Found in tests/test_chain.py at line 90.

Signature: `def test_editions_in_succession_read_in_order():`

What it does: Function test_editions_in_succession_read_in_order takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_empty_and_break_only_streams_round_trip

Found in tests/test_chain.py at line 100.

Signature: `def test_empty_and_break_only_streams_round_trip():`

What it does: Function test_empty_and_break_only_streams_round_trip takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_serialize_from_constructed_chains

Found in tests/test_chain.py at line 132.

Signature: `def test_serialize_from_constructed_chains():`

What it does: Function test_serialize_from_constructed_chains takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_chain_layer_does_not_touch_member_structure

Found in tests/test_chain.py at line 144.

Signature: `def test_chain_layer_does_not_touch_member_structure():`

What it does: Function test_chain_layer_does_not_touch_member_structure takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_multiverse_addressing

Found in tests/test_chain.py at line 153.

Signature: `def test_multiverse_addressing():`

What it does: Function test_multiverse_addressing takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_adjacent_closes_are_two_empty_chains

Found in tests/test_chain.py at line 167.

Signature: `def test_adjacent_closes_are_two_empty_chains():`

What it does: Function test_adjacent_closes_are_two_empty_chains takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_chain_fixture_case

Found in tests/test_chain_fixtures.py at line 28.

Signature: `def test_chain_fixture_case(case):`

What it does: Function test_chain_fixture_case takes case and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_valid_home_coordinate

Found in tests/test_coordinate.py at line 17.

Signature: `def test_valid_home_coordinate():`

What it does: Function test_valid_home_coordinate takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_independent_coordinates

Found in tests/test_coordinate.py at line 29.

Signature: `def test_independent_coordinates():`

What it does: Function test_independent_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_large_coordinates

Found in tests/test_coordinate.py at line 41.

Signature: `def test_large_coordinates():`

What it does: Function test_large_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_whitespace_support

Found in tests/test_coordinate.py at line 53.

Signature: `def test_whitespace_support():`

What it does: Function test_whitespace_support takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_internal_whitespace_support

Found in tests/test_coordinate.py at line 65.

Signature: `def test_internal_whitespace_support():`

What it does: Function test_internal_whitespace_support takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_to_urlencoded

Found in tests/test_coordinate.py at line 77.

Signature: `def test_to_urlencoded():`

What it does: Function test_to_urlencoded takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_library_break

Found in tests/test_coordinate.py at line 81.

Signature: `def test_library_break():`

What it does: Function test_library_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_shelf_break

Found in tests/test_coordinate.py at line 87.

Signature: `def test_shelf_break():`

What it does: Function test_shelf_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_series_break

Found in tests/test_coordinate.py at line 93.

Signature: `def test_series_break():`

What it does: Function test_series_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_collection_break

Found in tests/test_coordinate.py at line 99.

Signature: `def test_collection_break():`

What it does: Function test_collection_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_volume_break

Found in tests/test_coordinate.py at line 105.

Signature: `def test_volume_break():`

What it does: Function test_volume_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_book_break

Found in tests/test_coordinate.py at line 111.

Signature: `def test_book_break():`

What it does: Function test_book_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_chapter_break

Found in tests/test_coordinate.py at line 117.

Signature: `def test_chapter_break():`

What it does: Function test_chapter_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_section_break

Found in tests/test_coordinate.py at line 123.

Signature: `def test_section_break():`

What it does: Function test_section_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_scroll_break

Found in tests/test_coordinate.py at line 129.

Signature: `def test_scroll_break():`

What it does: Function test_scroll_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_breaks_are_unbounded

Found in tests/test_coordinate.py at line 135.

Signature: `def test_breaks_are_unbounded():`

What it does: Function test_breaks_are_unbounded takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_phokenize_scroll_unbounded

Found in tests/test_coordinate.py at line 147.

Signature: `def test_phokenize_scroll_unbounded():`

What it does: Function test_phokenize_scroll_unbounded takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_no_input

Found in tests/test_coordinate.py at line 154.

Signature: `def test_no_input():`

What it does: Function test_no_input takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_very_large_coordinates_round_trip

Found in tests/test_coordinate.py at line 158.

Signature: `def test_very_large_coordinates_round_trip():`

What it does: Function test_very_large_coordinates_round_trip takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_negative_coordinates

Found in tests/test_coordinate.py at line 174.

Signature: `def test_negative_coordinates():`

What it does: Function test_negative_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_malformed_coordinates

Found in tests/test_coordinate.py at line 178.

Signature: `def test_malformed_coordinates():`

What it does: Function test_malformed_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_null_coordinates

Found in tests/test_coordinate.py at line 182.

Signature: `def test_null_coordinates():`

What it does: Function test_null_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_null_library

Found in tests/test_coordinate.py at line 186.

Signature: `def test_null_library():`

What it does: Function test_null_library takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_null_shelf

Found in tests/test_coordinate.py at line 190.

Signature: `def test_null_shelf():`

What it does: Function test_null_shelf takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_null_series

Found in tests/test_coordinate.py at line 194.

Signature: `def test_null_series():`

What it does: Function test_null_series takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_null_collection

Found in tests/test_coordinate.py at line 198.

Signature: `def test_null_collection():`

What it does: Function test_null_collection takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_null_volume

Found in tests/test_coordinate.py at line 202.

Signature: `def test_null_volume():`

What it does: Function test_null_volume takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_null_book

Found in tests/test_coordinate.py at line 206.

Signature: `def test_null_book():`

What it does: Function test_null_book takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_null_chapter

Found in tests/test_coordinate.py at line 210.

Signature: `def test_null_chapter():`

What it does: Function test_null_chapter takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_null_section

Found in tests/test_coordinate.py at line 214.

Signature: `def test_null_section():`

What it does: Function test_null_section takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_null_scroll

Found in tests/test_coordinate.py at line 218.

Signature: `def test_null_scroll():`

What it does: Function test_null_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — coord

Found in tests/test_indexed.py at line 26.

Signature: `def coord(text):`

What it does: Function coord takes text and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — assert_equivalent

Found in tests/test_indexed.py at line 30.

Signature: `def assert_equivalent(buffer):`

What it does: Function assert_equivalent takes buffer and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_index_offsets_match_iter_scrolls

Found in tests/test_indexed.py at line 61.

Signature: `def test_index_offsets_match_iter_scrolls():`

What it does: Function test_index_offsets_match_iter_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_indexed_fetch_matches_fetch_everywhere

Found in tests/test_indexed.py at line 75.

Signature: `def test_indexed_fetch_matches_fetch_everywhere():`

What it does: Function test_indexed_fetch_matches_fetch_everywhere takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_indexed_explode_matches_explode

Found in tests/test_indexed.py at line 81.

Signature: `def test_indexed_explode_matches_explode():`

What it does: Function test_indexed_explode_matches_explode takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_indexed_generated_document

Found in tests/test_indexed.py at line 87.

Signature: `def test_indexed_generated_document():`

What it does: Function test_indexed_generated_document takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_indexed_empty_and_single_scroll

Found in tests/test_indexed.py at line 107.

Signature: `def test_indexed_empty_and_single_scroll():`

What it does: Function test_indexed_empty_and_single_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_indexed_unbounded_matches_fetch

Found in tests/test_indexed.py at line 120.

Signature: `def test_indexed_unbounded_matches_fetch():`

What it does: Function test_indexed_unbounded_matches_fetch takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_indexed_fetch_beyond_999_chapter

Found in tests/test_indexed.py at line 151.

Signature: `def test_indexed_fetch_beyond_999_chapter():`

What it does: Function test_indexed_fetch_beyond_999_chapter takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_index_offsets_built_once

Found in tests/test_indexed.py at line 170.

Signature: `def test_index_offsets_built_once():`

What it does: Function test_index_offsets_built_once takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — counting

Found in tests/test_indexed.py at line 179.

Signature: `def counting(self, buf):`

What it does: Function counting takes self, buf and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_phext_indexed_convenience_constructor

Found in tests/test_indexed.py at line 195.

Signature: `def test_phext_indexed_convenience_constructor():`

What it does: Function test_phext_indexed_convenience_constructor takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_dead_reckoning

Found in tests/test_phext.py at line 38.

Signature: `def test_dead_reckoning():`

What it does: Function test_dead_reckoning takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_line_break

Found in tests/test_phext.py at line 124.

Signature: `def test_line_break():`

What it does: Function test_line_break takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_more_cowbell

Found in tests/test_phext.py at line 127.

Signature: `def test_more_cowbell():`

What it does: Function test_more_cowbell takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_coordinate_based_insert

Found in tests/test_phext.py at line 130.

Signature: `def test_coordinate_based_insert():`

What it does: Function test_coordinate_based_insert takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_coordinate_based_replace

Found in tests/test_phext.py at line 248.

Signature: `def test_coordinate_based_replace():`

What it does: Function test_coordinate_based_replace takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_coordinate_based_remove

Found in tests/test_phext.py at line 345.

Signature: `def test_coordinate_based_remove():`

What it does: Function test_coordinate_based_remove takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_range_based_replace

Found in tests/test_phext.py at line 398.

Signature: `def test_range_based_replace():`

What it does: Function test_range_based_replace takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_next_scroll

Found in tests/test_phext.py at line 415.

Signature: `def test_next_scroll():`

What it does: Function test_next_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_last_empty_scroll

Found in tests/test_phext.py at line 425.

Signature: `def test_last_empty_scroll():`

What it does: Function test_last_empty_scroll takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_merge

Found in tests/test_phext.py at line 438.

Signature: `def test_merge():`

What it does: Function test_merge takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_subtract

Found in tests/test_phext.py at line 483.

Signature: `def test_subtract():`

What it does: Function test_subtract takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_normalize

Found in tests/test_phext.py at line 490.

Signature: `def test_normalize():`

What it does: Function test_normalize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_expand

Found in tests/test_phext.py at line 496.

Signature: `def test_expand():`

What it does: Function test_expand takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_contract

Found in tests/test_phext.py at line 529.

Signature: `def test_contract():`

What it does: Function test_contract takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_fs_read_write

Found in tests/test_phext.py at line 538.

Signature: `def test_fs_read_write():`

What it does: Function test_fs_read_write takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_replace_create

Found in tests/test_phext.py at line 553.

Signature: `def test_replace_create():`

What it does: Function test_replace_create takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_summary

Found in tests/test_phext.py at line 560.

Signature: `def test_summary():`

What it does: Function test_summary takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_navmap

Found in tests/test_phext.py at line 570.

Signature: `def test_navmap():`

What it does: Function test_navmap takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_textmap

Found in tests/test_phext.py at line 576.

Signature: `def test_textmap():`

What it does: Function test_textmap takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_larger_coordinates

Found in tests/test_phext.py at line 582.

Signature: `def test_larger_coordinates():`

What it does: Function test_larger_coordinates takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_phext_index

Found in tests/test_phext.py at line 590.

Signature: `def test_phext_index():`

What it does: Function test_phext_index takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_scroll_manifest

Found in tests/test_phext.py at line 642.

Signature: `def test_scroll_manifest():`

What it does: Function test_scroll_manifest takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_phext_soundex_v1

Found in tests/test_phext.py at line 694.

Signature: `def test_phext_soundex_v1():`

What it does: Function test_phext_soundex_v1 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_insert_performance_2k_scrolls

Found in tests/test_phext.py at line 700.

Signature: `def test_insert_performance_2k_scrolls():`

What it does: Function test_insert_performance_2k_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_hash_support

Found in tests/test_phext.py at line 829.

Signature: `def test_hash_support():`

What it does: Function test_hash_support takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_phext_breaks

Found in tests/test_phext.py at line 893.

Signature: `def test_phext_breaks():`

What it does: Function test_phext_breaks takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_normalize

Found in tests/test_phext.py at line 916.

Signature: `def test_normalize():`

What it does: Function test_normalize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — random_document

Found in tests/test_phokenize.py at line 45.

Signature: `def random_document(rng: random.Random) -> str:`

What it does: Function random_document takes rng: random.Random and gives back str:.

(Where these words came from: signature.)

---

## function — random_coordinate

Found in tests/test_phokenize.py at line 61.

Signature: `def random_coordinate(rng: random.Random) -> Coordinate:`

What it does: Function random_coordinate takes rng: random.Random and gives back Coordinate:.

(Where these words came from: signature.)

---

## function — outcome

Found in tests/test_phokenize.py at line 64.

Signature: `def outcome(fn, *args):`

What it does: Function outcome takes fn, *args and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — corpus

Found in tests/test_phokenize.py at line 71.

Signature: `def corpus():`

What it does: Function corpus takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — as_pairs

Found in tests/test_phokenize.py at line 80.

Signature: `def as_pairs(stack):`

What it does: Function as_pairs takes stack and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_phokenize_matches_legacy

Found in tests/test_phokenize.py at line 83.

Signature: `def test_phokenize_matches_legacy():`

What it does: Function test_phokenize_matches_legacy takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_phokenize_coordinates_are_distinct_objects

Found in tests/test_phokenize.py at line 89.

Signature: `def test_phokenize_coordinates_are_distinct_objects():`

What it does: Function test_phokenize_coordinates_are_distinct_objects takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_serialization_matches_legacy

Found in tests/test_phokenize.py at line 96.

Signature: `def test_serialization_matches_legacy():`

What it does: Function test_serialization_matches_legacy takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_explode_implode_round_trip

Found in tests/test_phokenize.py at line 107.

Signature: `def test_explode_implode_round_trip():`

What it does: Function test_explode_implode_round_trip takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_maps_and_index_match_legacy

Found in tests/test_phokenize.py at line 123.

Signature: `def test_maps_and_index_match_legacy():`

What it does: Function test_maps_and_index_match_legacy takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_lookups_match_legacy

Found in tests/test_phokenize.py at line 135.

Signature: `def test_lookups_match_legacy():`

What it does: Function test_lookups_match_legacy takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_edits_match_legacy

Found in tests/test_phokenize.py at line 153.

Signature: `def test_edits_match_legacy():`

What it does: Function test_edits_match_legacy takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_append_scroll_matches_legacy

Found in tests/test_phokenize.py at line 170.

Signature: `def test_append_scroll_matches_legacy():`

What it does: Function test_append_scroll_matches_legacy takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_append_scroll_out_of_order

Found in tests/test_phokenize.py at line 187.

Signature: `def test_append_scroll_out_of_order():`

What it does: Function test_append_scroll_out_of_order takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — large_document

Found in tests/test_phokenize.py at line 195.

Signature: `def large_document(scrolls: int) -> str:`

What it does: Function large_document takes scrolls: int and gives back str:.

(Where these words came from: signature.)

---

## function — test_explode_performance_50k_scrolls

Found in tests/test_phokenize.py at line 207.

Signature: `def test_explode_performance_50k_scrolls():`

What it does: Function test_explode_performance_50k_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_scroll_interface

Found in tests/test_positionedScroll.py at line 12.

Signature: `def test_scroll_interface():`

What it does: Function test_scroll_interface takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_positioned_scrolls

Found in tests/test_positionedScroll.py at line 19.

Signature: `def test_positioned_scrolls():`

What it does: Function test_positioned_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_phokenize

Found in tests/test_positionedScroll.py at line 39.

Signature: `def test_phokenize():`

What it does: Function test_phokenize takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_range_exists

Found in tests/test_range.py at line 5.

Signature: `def test_range_exists():`

What it does: Function test_range_exists takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_range_fixture_case

Found in tests/test_range_fixtures.py at line 55.

Signature: `def test_range_fixture_case(case_name):`

What it does: Function test_range_fixture_case takes case_name and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_seal_round_trip_real_phext

Found in tests/test_seal.py at line 23.

Signature: `def test_seal_round_trip_real_phext():`

What it does: Function test_seal_round_trip_real_phext takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_seal_round_trip_empty_and_small

Found in tests/test_seal.py at line 33.

Signature: `def test_seal_round_trip_empty_and_small():`

What it does: Function test_seal_round_trip_empty_and_small takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_seal_carrying_binary_scrolls

Found in tests/test_seal.py at line 41.

Signature: `def test_seal_carrying_binary_scrolls():`

What it does: Function test_seal_carrying_binary_scrolls takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_seal_is_deterministic

Found in tests/test_seal.py at line 50.

Signature: `def test_seal_is_deterministic():`

What it does: Function test_seal_is_deterministic takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_unseal_rejects_non_xz_data

Found in tests/test_seal.py at line 57.

Signature: `def test_unseal_rejects_non_xz_data():`

What it does: Function test_unseal_rejects_non_xz_data takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

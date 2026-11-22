# libphext-py — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 147). Scanned at commit e773fe0a2281408e8cefbd44bf07480986d34b6a. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

Description stated in the manifest: Pythonic implementation of phext

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 51 commits.

The author of record is **Will Bickford** — 51 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 46 commits
- Emise — 4 commits
- Orin — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 1 file, 172 lines total, 44 blank, 0 comment, 128 code
- Python: 25 files, 4577 lines total, 656 blank, 442 comment, 3479 code
- Shell: 3 files, 116 lines total, 8 blank, 41 comment, 67 code
- TOML: 1 file, 43 lines total, 7 blank, 4 comment, 32 code

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

- Origin: https://github.com/wbic16/libphext-py
- Branch: exo
- Head commit: e773fe0a2281408e8cefbd44bf07480986d34b6a
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `PointAddress.__str__` (phext/addressing.py:66) — Function PointAddress.__str__ takes no inputs beyond the value it is called on.
- function `RangeAddress.__str__` (phext/addressing.py:86) — Function RangeAddress.__str__ takes no inputs beyond the value it is called on.
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
- function `Chain.serialize` (phext/chain.py:50) — Serializes this chain alone: its members joined by
- function `Chain.fetch` (phext/chain.py:59) — Fetch a scroll by multiverse address: ``$N:coord``, or a bare
- function `Chain.resolve_point` (phext/chain.py:72) — Resolves a point address against this chain: (universe,
- function `Chain.fetch_point` (phext/chain.py:93) — Fetches a point address string.
- function `Chain.resolve_range` (phext/chain.py:116) — Resolves a range against this chain into per-scroll segments:
- function `Chain.fetch_range` (phext/chain.py:170) — Fetches a range address string: the spanned text with phext
- function `parse_chains` (phext/chain.py:197) — Splits a stream into its chains.
- function `serialize_chains` (phext/chain.py:221) — Serializes a stream of chains back into stream bytes: the inverse
- function `parse_universe_address` (phext/chain.py:227) — Parses a multiverse address into (universe, Coordinate), or None
- function `advance_coordinate` (phext/coordinate.py:4) — Function advance_coordinate takes index: int.
- function `Coordinate.from_string` (phext/coordinate.py:29) — Function Coordinate.from_string takes ignored, coord_str: str.
- function `Coordinate.libraryBreak` (phext/coordinate.py:50) — Function Coordinate.libraryBreak takes no inputs beyond the value it is called on.
- function `Coordinate.shelfBreak` (phext/coordinate.py:61) — Function Coordinate.shelfBreak takes no inputs beyond the value it is called on.
- function `Coordinate.seriesBreak` (phext/coordinate.py:71) — Function Coordinate.seriesBreak takes no inputs beyond the value it is called on.
- function `Coordinate.collectionBreak` (phext/coordinate.py:80) — Function Coordinate.collectionBreak takes no inputs beyond the value it is called on.
- function `Coordinate.volumeBreak` (phext/coordinate.py:88) — Function Coordinate.volumeBreak takes no inputs beyond the value it is called on.
- function `Coordinate.bookBreak` (phext/coordinate.py:95) — Function Coordinate.bookBreak takes no inputs beyond the value it is called on.
- function `Coordinate.chapterBreak` (phext/coordinate.py:101) — Function Coordinate.chapterBreak takes no inputs beyond the value it is called on.
- function `Coordinate.sectionBreak` (phext/coordinate.py:106) — Function Coordinate.sectionBreak takes no inputs beyond the value it is called on.
- function `Coordinate.scrollBreak` (phext/coordinate.py:110) — Function Coordinate.scrollBreak takes no inputs beyond the value it is called on.
- function `Coordinate.__init__` (phext/coordinate.py:113) — Function Coordinate.__init__ takes self, lb, sf, sr, cn, vm, bk, ch, sn, sc.
- function `Coordinate.copy_coordinate` (phext/coordinate.py:124) — Function Coordinate.copy_coordinate takes self, other.
- function `Coordinate.__str__` (phext/coordinate.py:135) — Function Coordinate.__str__ takes no inputs beyond the value it is called on.
- function `Coordinate.urlencoded` (phext/coordinate.py:140) — Function Coordinate.urlencoded takes no inputs beyond the value it is called on.
- function `Coordinate.__eq__` (phext/coordinate.py:145) — Function Coordinate.__eq__ takes self, other.
- function `Coordinate.__lt__` (phext/coordinate.py:148) — Function Coordinate.__lt__ takes self, other.
- function `Coordinate.as_tuple` (phext/coordinate.py:153) — Function Coordinate.as_tuple takes no inputs beyond the value it is called on.
- function `Coordinate.__hash__` (phext/coordinate.py:160) — Function Coordinate.__hash__ takes no inputs beyond the value it is called on.
- function `IndexedPhext.__init__` (phext/indexed.py:31) — Function IndexedPhext.__init__ takes self, buffer: str.
- function `IndexedPhext.fetch` (phext/indexed.py:42) — Function IndexedPhext.fetch takes self, coord: Coordinate.
- function `IndexedPhext.__contains__` (phext/indexed.py:49) — Function IndexedPhext.__contains__ takes self, coord: Coordinate.
- function `IndexedPhext.coordinates` (phext/indexed.py:52) — Function IndexedPhext.coordinates takes no inputs beyond the value it is called on.
- function `IndexedPhext.keys` (phext/indexed.py:55) — Function IndexedPhext.keys takes no inputs beyond the value it is called on.
- function `IndexedPhext.index` (phext/indexed.py:58) — Function IndexedPhext.index takes no inputs beyond the value it is called on.
- function `IndexedPhext.explode` (phext/indexed.py:62) — Function IndexedPhext.explode takes no inputs beyond the value it is called on.
- function `IndexedPhext.__len__` (phext/indexed.py:68) — Function IndexedPhext.__len__ takes no inputs beyond the value it is called on.
- function `Phext.defaultCoordinate` (phext/phext.py:61) — Function Phext.defaultCoordinate takes no inputs beyond the value it is called on.
- function `Phext.__init__` (phext/phext.py:64) — Function Phext.__init__ takes no inputs beyond the value it is called on.
- function `Phext.fetch` (phext/phext.py:67) — Function Phext.fetch takes self, buffer:str, coord:Coordinate.
- function `Phext.isPhextBreak` (phext/phext.py:73) — Function Phext.isPhextBreak takes self, byte:str.
- function `Phext.phokenize` (phext/phext.py:111) — Function Phext.phokenize takes self, buffer:str.
- function `Phext.append_scroll` (phext/phext.py:177) — Function Phext.append_scroll takes self, entry: PositionedScroll, location: Coordinate.
- function `Phext.dephokenize` (phext/phext.py:183) — Function Phext.dephokenize takes self, stack: list[PositionedScroll].
- function `Phext.normalize` (phext/phext.py:191) — Function Phext.normalize takes self, buffer:str.
- function `Phext.update` (phext/phext.py:195) — Function Phext.update takes self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool.
- function `Phext.insert` (phext/phext.py:218) — Function Phext.insert takes self, buffer:str, coord:Coordinate, scroll:str.
- function `Phext.replace` (phext/phext.py:221) — Function Phext.replace takes self, buffer:str, coord:Coordinate, scroll:str.
- function `Phext.remove` (phext/phext.py:224) — Function Phext.remove takes self, buffer:str, coord:Coordinate.
- function `Phext.range_replace` (phext/phext.py:228) — Function Phext.range_replace takes self, buffer:str, range:Range, text:str.
- function `Phext.next_scroll` (phext/phext.py:244) — Function Phext.next_scroll takes self, buffer:str, coord:Coordinate.
- function `Phext.get_subspace_coordinates` (phext/phext.py:256) — Function Phext.get_subspace_coordinates takes self, buffer:str, target:Coordinate.
- function `Phext.merge` (phext/phext.py:310) — Function Phext.merge takes self, left: str, right:str.
- function `Phext.subtract` (phext/phext.py:342) — Function Phext.subtract takes self, left:str, right:str.
- function `Phext.expand` (phext/phext.py:363) — Function Phext.expand takes self, buffer:str.
- function `Phext.contract` (phext/phext.py:366) — Function Phext.contract takes self, buffer:str.
- function `Phext.create_summary` (phext/phext.py:369) — Function Phext.create_summary takes self, buffer:str.
- function `Phext.navmap` (phext/phext.py:377) — Function Phext.navmap takes self, urlbase:str, buffer:str.
- function `Phext.textmap` (phext/phext.py:391) — Function Phext.textmap takes self, buffer:str.
- function `Phext.checksum` (phext/phext.py:398) — Function Phext.checksum takes self, buffer:str.
- function `Phext.manifest` (phext/phext.py:404) — Function Phext.manifest takes self, buffer:str.
- function `Phext.soundex_v1` (phext/phext.py:411) — Function Phext.soundex_v1 takes self, buffer:str.
- function `Phext.soundex_internal` (phext/phext.py:419) — Function Phext.soundex_internal takes self, buffer:str.
- function `Phext.explode` (phext/phext.py:443) — Function Phext.explode takes self, buffer:str.
- function `Phext.implode` (phext/phext.py:450) — Function Phext.implode takes self, hash: dict[Coordinate, str].
- function `Phext.index_phokens` (phext/phext.py:456) — Function Phext.index_phokens takes self, buffer:str.
- function `Phext.index` (phext/phext.py:475) — Function Phext.index takes self, buffer:str.
- function `Phext.offset` (phext/phext.py:479) — Function Phext.offset takes self, buffer:str, coord:Coordinate.
- function `Phext.index_offsets` (phext/phext.py:494) — Function Phext.index_offsets takes self, buffer:str.
- function `Phext.indexed` (phext/phext.py:507) — Function Phext.indexed takes self, buffer:str.
- function `Phext.insert_binary` (phext/phext.py:519) — Function Phext.insert_binary takes self, buffer: str, coord: Coordinate, payload: bytes.
- function `Phext.replace_binary` (phext/phext.py:523) — Function Phext.replace_binary takes self, buffer: str, coord: Coordinate, payload: bytes.
- function `Phext.fetch_binary` (phext/phext.py:527) — Function Phext.fetch_binary takes self, buffer: str, coord: Coordinate.
- function `Phext.insert_bytes` (phext/phext.py:531) — Function Phext.insert_bytes takes self, buffer: str, coord: Coordinate, payload: bytes.
- function `Phext.replace_bytes` (phext/phext.py:535) — Function Phext.replace_bytes takes self, buffer: str, coord: Coordinate, payload: bytes.
- function `PositionedScroll.__init__` (phext/positionedScroll.py:9) — Function PositionedScroll.__init__ takes self, coord, text.
- function `seal` (phext/seal.py:26) — Seals a phext: xz-compresses its full UTF-8 bytes.
- function `seal_bytes` (phext/seal.py:31) — Seals raw phext bytes (see `seal`).
- function `unseal` (phext/seal.py:36) — Unseals a sealed phext back to its exact original bytes.
- function `unseal_string` (phext/seal.py:42) — Unseals directly to text.
- function `timed` (test/bench.py:17) — Function timed takes label, fn.
- function `main` (test/bench.py:24) — Function main takes no inputs beyond the value it is called on.
- function `LegacyPhext.defaultCoordinate` (tests/legacy_phext.py:38) [test] — Function LegacyPhext.defaultCoordinate takes no inputs beyond the value it is called on.
- function `LegacyPhext.__init__` (tests/legacy_phext.py:41) [test] — Function LegacyPhext.__init__ takes no inputs beyond the value it is called on.
- function `LegacyPhext.fetch` (tests/legacy_phext.py:44) [test] — Function LegacyPhext.fetch takes self, buffer:str, coord:Coordinate.
- function `LegacyPhext.isPhextBreak` (tests/legacy_phext.py:51) [test] — Function LegacyPhext.isPhextBreak takes self, byte:str.
- function `LegacyPhext.phokenize` (tests/legacy_phext.py:63) [test] — Function LegacyPhext.phokenize takes self, buffer:str.
- function `LegacyPhext.append_scroll` (tests/legacy_phext.py:114) [test] — Function LegacyPhext.append_scroll takes self, entry: PositionedScroll, location: Coordinate.
- function `LegacyPhext.dephokenize` (tests/legacy_phext.py:159) [test] — Function LegacyPhext.dephokenize takes self, stack: list[PositionedScroll].
- function `LegacyPhext.normalize` (tests/legacy_phext.py:168) [test] — Function LegacyPhext.normalize takes self, buffer:str.
- function `LegacyPhext.update` (tests/legacy_phext.py:172) [test] — Function LegacyPhext.update takes self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool.
- function `LegacyPhext.insert` (tests/legacy_phext.py:195) [test] — Function LegacyPhext.insert takes self, buffer:str, coord:Coordinate, scroll:str.
- function `LegacyPhext.replace` (tests/legacy_phext.py:198) [test] — Function LegacyPhext.replace takes self, buffer:str, coord:Coordinate, scroll:str.
- function `LegacyPhext.remove` (tests/legacy_phext.py:201) [test] — Function LegacyPhext.remove takes self, buffer:str, coord:Coordinate.
- function `LegacyPhext.range_replace` (tests/legacy_phext.py:205) [test] — Function LegacyPhext.range_replace takes self, buffer:str, range:Range, text:str.
- function `LegacyPhext.next_scroll` (tests/legacy_phext.py:221) [test] — Function LegacyPhext.next_scroll takes self, buffer:str, coord:Coordinate.
- function `LegacyPhext.get_subspace_coordinates` (tests/legacy_phext.py:234) [test] — Function LegacyPhext.get_subspace_coordinates takes self, buffer:str, target:Coordinate.
- function `LegacyPhext.merge` (tests/legacy_phext.py:297) [test] — Function LegacyPhext.merge takes self, left: str, right:str.
- function `LegacyPhext.subtract` (tests/legacy_phext.py:329) [test] — Function LegacyPhext.subtract takes self, left:str, right:str.
- function `LegacyPhext.expand` (tests/legacy_phext.py:350) [test] — Function LegacyPhext.expand takes self, buffer:str.
- function `LegacyPhext.contract` (tests/legacy_phext.py:375) [test] — Function LegacyPhext.contract takes self, buffer:str.
- function `LegacyPhext.create_summary` (tests/legacy_phext.py:400) [test] — Function LegacyPhext.create_summary takes self, buffer:str.
- function `LegacyPhext.navmap` (tests/legacy_phext.py:414) [test] — Function LegacyPhext.navmap takes self, urlbase:str, buffer:str.
- function `LegacyPhext.textmap` (tests/legacy_phext.py:428) [test] — Function LegacyPhext.textmap takes self, buffer:str.
- function `LegacyPhext.checksum` (tests/legacy_phext.py:436) [test] — Function LegacyPhext.checksum takes self, buffer:str.
- function `LegacyPhext.manifest` (tests/legacy_phext.py:440) [test] — Function LegacyPhext.manifest takes self, buffer:str.
- function `LegacyPhext.soundex_v1` (tests/legacy_phext.py:447) [test] — Function LegacyPhext.soundex_v1 takes self, buffer:str.
- function `LegacyPhext.soundex_internal` (tests/legacy_phext.py:455) [test] — Function LegacyPhext.soundex_internal takes self, buffer:str.
- function `LegacyPhext.explode` (tests/legacy_phext.py:479) [test] — Function LegacyPhext.explode takes self, buffer:str.
- function `LegacyPhext.implode` (tests/legacy_phext.py:486) [test] — Function LegacyPhext.implode takes self, hash: dict[Coordinate, str].
- function `LegacyPhext.index_phokens` (tests/legacy_phext.py:492) [test] — Function LegacyPhext.index_phokens takes self, buffer:str.
- function `LegacyPhext.index` (tests/legacy_phext.py:532) [test] — Function LegacyPhext.index takes self, buffer:str.
- function `LegacyPhext.offset` (tests/legacy_phext.py:536) [test] — Function LegacyPhext.offset takes self, buffer:str, coord:Coordinate.
- function `home` (tests/test_address.py:15) [test] — Function home takes no inputs beyond the value it is called on.
- function `scroll2` (tests/test_address.py:19) [test] — Function scroll2 takes no inputs beyond the value it is called on.
- function `member_a` (tests/test_address.py:23) [test] — Function member_a takes no inputs beyond the value it is called on.
- function `member_b` (tests/test_address.py:29) [test] — Function member_b takes no inputs beyond the value it is called on.
- function `make_chain` (tests/test_address.py:33) [test] — Function make_chain takes no inputs beyond the value it is called on.
- function `test_point_parse_canonical` (tests/test_address.py:37) [test] — Function test_point_parse_canonical takes no inputs beyond the value it is called on.
- function `test_point_parse_rejects` (tests/test_address.py:60) [test] — Function test_point_parse_rejects takes no inputs beyond the value it is called on.
- function `test_point_resolve` (tests/test_address.py:77) [test] — Function test_point_resolve takes no inputs beyond the value it is called on.
- function `test_point_fetch_offset` (tests/test_address.py:95) [test] — Function test_point_fetch_offset takes no inputs beyond the value it is called on.
- function `test_point_fetch_offset_unicode` (tests/test_address.py:110) [test] — Function test_point_fetch_offset_unicode takes no inputs beyond the value it is called on.
- function `test_range_parse_canonical` (tests/test_address.py:118) [test] — Function test_range_parse_canonical takes no inputs beyond the value it is called on.
- function `test_range_fetch_within_scroll` (tests/test_address.py:133) [test] — Function test_range_fetch_within_scroll takes no inputs beyond the value it is called on.
- function `test_range_fetch_cross_scroll` (tests/test_address.py:145) [test] — Function test_range_fetch_cross_scroll takes no inputs beyond the value it is called on.
- function `test_range_fetch_multiverse` (tests/test_address.py:157) [test] — Function test_range_fetch_multiverse takes no inputs beyond the value it is called on.
- function `test_range_resolution_errors` (tests/test_address.py:176) [test] — Function test_range_resolution_errors takes no inputs beyond the value it is called on.
- function `coord` (tests/test_binary.py:11) [test] — Function coord takes address.
- function `test_encode_decode_round_trip` (tests/test_binary.py:15) [test] — Function test_encode_decode_round_trip takes no inputs beyond the value it is called on.
- function `test_decode_scroll_rejects_non_binary` (tests/test_binary.py:39) [test] — Function test_decode_scroll_rejects_non_binary takes no inputs beyond the value it is called on.
- function `test_insert_and_fetch_binary` (tests/test_binary.py:51) [test] — Function test_insert_and_fetch_binary takes no inputs beyond the value it is called on.
- function `test_fetch_binary_fallbacks` (tests/test_binary.py:68) [test] — Function test_fetch_binary_fallbacks takes no inputs beyond the value it is called on.
- function `test_update_rule_non_utf8_is_base64_verbatim` (tests/test_binary.py:80) [test] — Function test_update_rule_non_utf8_is_base64_verbatim takes no inputs beyond the value it is called on.
- function `test_update_rule_utf8_stays_text` (tests/test_binary.py:97) [test] — Function test_update_rule_utf8_stays_text takes no inputs beyond the value it is called on.
- function `test_ingest_foreign_byte_stream` (tests/test_binary.py:105) [test] — Function test_ingest_foreign_byte_stream takes no inputs beyond the value it is called on.
- function `test_ingest_canonical_phext_is_identity` (tests/test_binary.py:119) [test] — Function test_ingest_canonical_phext_is_identity takes no inputs beyond the value it is called on.
- function `test_ingest_empty_and_break_only_streams` (tests/test_binary.py:126) [test] — Function test_ingest_empty_and_break_only_streams takes no inputs beyond the value it is called on.
- function `test_phokenize_bytes_matches_phokenize` (tests/test_binary.py:134) [test] — Function test_phokenize_bytes_matches_phokenize takes no inputs beyond the value it is called on.
- function `test_phokenize_bytes_matches_literal_reference_on_random_streams` (tests/test_binary.py:182) [test] — Function test_phokenize_bytes_matches_literal_reference_on_random_streams takes no inputs beyond the value it is called on.
- function `coord` (tests/test_chain.py:12) [test] — Function coord takes address.
- function `roundtrip` (tests/test_chain.py:16) [test] — Function roundtrip takes stream.
- function `test_chain_free_input_is_one_unclosed_chain` (tests/test_chain.py:21) [test] — Function test_chain_free_input_is_one_unclosed_chain takes no inputs beyond the value it is called on.
- function `test_two_phexts_chained_and_closed_round_trip` (tests/test_chain.py:38) [test] — Function test_two_phexts_chained_and_closed_round_trip takes no inputs beyond the value it is called on.
- function `test_unclosed_chain_at_eof_reports_truncated` (tests/test_chain.py:47) [test] — Function test_unclosed_chain_at_eof_reports_truncated takes no inputs beyond the value it is called on.
- function `test_two_chains_in_one_stream_parse_as_two_units` (tests/test_chain.py:55) [test] — Function test_two_chains_in_one_stream_parse_as_two_units takes no inputs beyond the value it is called on.
- function `test_chained_members_carry_full_phexts` (tests/test_chain.py:75) [test] — Function test_chained_members_carry_full_phexts takes no inputs beyond the value it is called on.
- function `test_editions_in_succession_read_in_order` (tests/test_chain.py:90) [test] — Function test_editions_in_succession_read_in_order takes no inputs beyond the value it is called on.
- function `test_empty_and_break_only_streams_round_trip` (tests/test_chain.py:100) [test] — Function test_empty_and_break_only_streams_round_trip takes no inputs beyond the value it is called on.
- function `test_serialize_from_constructed_chains` (tests/test_chain.py:132) [test] — Function test_serialize_from_constructed_chains takes no inputs beyond the value it is called on.
- function `test_chain_layer_does_not_touch_member_structure` (tests/test_chain.py:144) [test] — Function test_chain_layer_does_not_touch_member_structure takes no inputs beyond the value it is called on.
- function `test_multiverse_addressing` (tests/test_chain.py:153) [test] — Function test_multiverse_addressing takes no inputs beyond the value it is called on.
- function `test_adjacent_closes_are_two_empty_chains` (tests/test_chain.py:167) [test] — Function test_adjacent_closes_are_two_empty_chains takes no inputs beyond the value it is called on.
- function `test_chain_fixture_case` (tests/test_chain_fixtures.py:28) [test] — Function test_chain_fixture_case takes case.
- function `test_valid_home_coordinate` (tests/test_coordinate.py:17) [test] — Function test_valid_home_coordinate takes no inputs beyond the value it is called on.
- function `test_independent_coordinates` (tests/test_coordinate.py:29) [test] — Function test_independent_coordinates takes no inputs beyond the value it is called on.
- function `test_large_coordinates` (tests/test_coordinate.py:41) [test] — Function test_large_coordinates takes no inputs beyond the value it is called on.
- function `test_whitespace_support` (tests/test_coordinate.py:53) [test] — Function test_whitespace_support takes no inputs beyond the value it is called on.
- function `test_internal_whitespace_support` (tests/test_coordinate.py:65) [test] — Function test_internal_whitespace_support takes no inputs beyond the value it is called on.
- function `test_to_urlencoded` (tests/test_coordinate.py:77) [test] — Function test_to_urlencoded takes no inputs beyond the value it is called on.
- function `test_library_break` (tests/test_coordinate.py:81) [test] — Function test_library_break takes no inputs beyond the value it is called on.
- function `test_shelf_break` (tests/test_coordinate.py:87) [test] — Function test_shelf_break takes no inputs beyond the value it is called on.
- function `test_series_break` (tests/test_coordinate.py:93) [test] — Function test_series_break takes no inputs beyond the value it is called on.
- function `test_collection_break` (tests/test_coordinate.py:99) [test] — Function test_collection_break takes no inputs beyond the value it is called on.
- function `test_volume_break` (tests/test_coordinate.py:105) [test] — Function test_volume_break takes no inputs beyond the value it is called on.
- function `test_book_break` (tests/test_coordinate.py:111) [test] — Function test_book_break takes no inputs beyond the value it is called on.
- function `test_chapter_break` (tests/test_coordinate.py:117) [test] — Function test_chapter_break takes no inputs beyond the value it is called on.
- function `test_section_break` (tests/test_coordinate.py:123) [test] — Function test_section_break takes no inputs beyond the value it is called on.
- function `test_scroll_break` (tests/test_coordinate.py:129) [test] — Function test_scroll_break takes no inputs beyond the value it is called on.
- function `test_breaks_are_unbounded` (tests/test_coordinate.py:135) [test] — Function test_breaks_are_unbounded takes no inputs beyond the value it is called on.
- function `test_phokenize_scroll_unbounded` (tests/test_coordinate.py:147) [test] — Function test_phokenize_scroll_unbounded takes no inputs beyond the value it is called on.
- function `test_no_input` (tests/test_coordinate.py:154) [test] — Function test_no_input takes no inputs beyond the value it is called on.
- function `test_very_large_coordinates_round_trip` (tests/test_coordinate.py:158) [test] — Function test_very_large_coordinates_round_trip takes no inputs beyond the value it is called on.
- function `test_negative_coordinates` (tests/test_coordinate.py:174) [test] — Function test_negative_coordinates takes no inputs beyond the value it is called on.
- function `test_malformed_coordinates` (tests/test_coordinate.py:178) [test] — Function test_malformed_coordinates takes no inputs beyond the value it is called on.
- function `test_null_coordinates` (tests/test_coordinate.py:182) [test] — Function test_null_coordinates takes no inputs beyond the value it is called on.
- function `test_null_library` (tests/test_coordinate.py:186) [test] — Function test_null_library takes no inputs beyond the value it is called on.
- function `test_null_shelf` (tests/test_coordinate.py:190) [test] — Function test_null_shelf takes no inputs beyond the value it is called on.
- function `test_null_series` (tests/test_coordinate.py:194) [test] — Function test_null_series takes no inputs beyond the value it is called on.
- function `test_null_collection` (tests/test_coordinate.py:198) [test] — Function test_null_collection takes no inputs beyond the value it is called on.
- function `test_null_volume` (tests/test_coordinate.py:202) [test] — Function test_null_volume takes no inputs beyond the value it is called on.
- function `test_null_book` (tests/test_coordinate.py:206) [test] — Function test_null_book takes no inputs beyond the value it is called on.
- function `test_null_chapter` (tests/test_coordinate.py:210) [test] — Function test_null_chapter takes no inputs beyond the value it is called on.
- function `test_null_section` (tests/test_coordinate.py:214) [test] — Function test_null_section takes no inputs beyond the value it is called on.
- function `test_null_scroll` (tests/test_coordinate.py:218) [test] — Function test_null_scroll takes no inputs beyond the value it is called on.
- function `coord` (tests/test_indexed.py:26) [test] — Function coord takes text.
- function `assert_equivalent` (tests/test_indexed.py:30) [test] — Function assert_equivalent takes buffer.
- function `test_index_offsets_match_iter_scrolls` (tests/test_indexed.py:61) [test] — Function test_index_offsets_match_iter_scrolls takes no inputs beyond the value it is called on.
- function `test_indexed_fetch_matches_fetch_everywhere` (tests/test_indexed.py:75) [test] — Function test_indexed_fetch_matches_fetch_everywhere takes no inputs beyond the value it is called on.
- function `test_indexed_explode_matches_explode` (tests/test_indexed.py:81) [test] — Function test_indexed_explode_matches_explode takes no inputs beyond the value it is called on.
- function `test_indexed_generated_document` (tests/test_indexed.py:87) [test] — Function test_indexed_generated_document takes no inputs beyond the value it is called on.
- function `test_indexed_empty_and_single_scroll` (tests/test_indexed.py:107) [test] — Function test_indexed_empty_and_single_scroll takes no inputs beyond the value it is called on.
- function `test_indexed_unbounded_matches_fetch` (tests/test_indexed.py:120) [test] — Function test_indexed_unbounded_matches_fetch takes no inputs beyond the value it is called on.
- function `test_indexed_fetch_beyond_999_chapter` (tests/test_indexed.py:151) [test] — Function test_indexed_fetch_beyond_999_chapter takes no inputs beyond the value it is called on.
- function `test_index_offsets_built_once` (tests/test_indexed.py:170) [test] — Function test_index_offsets_built_once takes no inputs beyond the value it is called on.
- function `counting` (tests/test_indexed.py:179) [test] — Function counting takes self, buf.
- function `test_phext_indexed_convenience_constructor` (tests/test_indexed.py:195) [test] — Function test_phext_indexed_convenience_constructor takes no inputs beyond the value it is called on.
- function `test_dead_reckoning` (tests/test_phext.py:38) [test] — Function test_dead_reckoning takes no inputs beyond the value it is called on.
- function `test_line_break` (tests/test_phext.py:124) [test] — Function test_line_break takes no inputs beyond the value it is called on.
- function `test_more_cowbell` (tests/test_phext.py:127) [test] — Function test_more_cowbell takes no inputs beyond the value it is called on.
- function `test_coordinate_based_insert` (tests/test_phext.py:130) [test] — Function test_coordinate_based_insert takes no inputs beyond the value it is called on.
- function `test_coordinate_based_replace` (tests/test_phext.py:248) [test] — Function test_coordinate_based_replace takes no inputs beyond the value it is called on.
- function `test_coordinate_based_remove` (tests/test_phext.py:345) [test] — Function test_coordinate_based_remove takes no inputs beyond the value it is called on.
- function `test_range_based_replace` (tests/test_phext.py:398) [test] — Function test_range_based_replace takes no inputs beyond the value it is called on.
- function `test_next_scroll` (tests/test_phext.py:415) [test] — Function test_next_scroll takes no inputs beyond the value it is called on.
- function `test_last_empty_scroll` (tests/test_phext.py:425) [test] — Function test_last_empty_scroll takes no inputs beyond the value it is called on.
- function `test_merge` (tests/test_phext.py:438) [test] — Function test_merge takes no inputs beyond the value it is called on.
- function `test_subtract` (tests/test_phext.py:483) [test] — Function test_subtract takes no inputs beyond the value it is called on.
- function `test_normalize` (tests/test_phext.py:490) [test] — Function test_normalize takes no inputs beyond the value it is called on.
- function `test_expand` (tests/test_phext.py:496) [test] — Function test_expand takes no inputs beyond the value it is called on.
- function `test_contract` (tests/test_phext.py:529) [test] — Function test_contract takes no inputs beyond the value it is called on.
- function `test_fs_read_write` (tests/test_phext.py:538) [test] — Function test_fs_read_write takes no inputs beyond the value it is called on.
- function `test_replace_create` (tests/test_phext.py:553) [test] — Function test_replace_create takes no inputs beyond the value it is called on.
- function `test_summary` (tests/test_phext.py:560) [test] — Function test_summary takes no inputs beyond the value it is called on.
- function `test_navmap` (tests/test_phext.py:570) [test] — Function test_navmap takes no inputs beyond the value it is called on.
- function `test_textmap` (tests/test_phext.py:576) [test] — Function test_textmap takes no inputs beyond the value it is called on.
- function `test_larger_coordinates` (tests/test_phext.py:582) [test] — Function test_larger_coordinates takes no inputs beyond the value it is called on.
- function `test_phext_index` (tests/test_phext.py:590) [test] — Function test_phext_index takes no inputs beyond the value it is called on.
- function `test_scroll_manifest` (tests/test_phext.py:642) [test] — Function test_scroll_manifest takes no inputs beyond the value it is called on.
- function `test_phext_soundex_v1` (tests/test_phext.py:694) [test] — Function test_phext_soundex_v1 takes no inputs beyond the value it is called on.
- function `test_insert_performance_2k_scrolls` (tests/test_phext.py:700) [test] — Function test_insert_performance_2k_scrolls takes no inputs beyond the value it is called on.
- function `test_hash_support` (tests/test_phext.py:829) [test] — Function test_hash_support takes no inputs beyond the value it is called on.
- function `test_phext_breaks` (tests/test_phext.py:893) [test] — Function test_phext_breaks takes no inputs beyond the value it is called on.
- function `test_normalize` (tests/test_phext.py:916) [test] — Function test_normalize takes no inputs beyond the value it is called on.
- function `random_document` (tests/test_phokenize.py:45) [test] — Function random_document takes rng: random.Random.
- function `random_coordinate` (tests/test_phokenize.py:61) [test] — Function random_coordinate takes rng: random.Random.
- function `outcome` (tests/test_phokenize.py:64) [test] — Function outcome takes fn, *args.
- function `corpus` (tests/test_phokenize.py:71) [test] — Function corpus takes no inputs beyond the value it is called on.
- function `as_pairs` (tests/test_phokenize.py:80) [test] — Function as_pairs takes stack.
- function `test_phokenize_matches_legacy` (tests/test_phokenize.py:83) [test] — Function test_phokenize_matches_legacy takes no inputs beyond the value it is called on.
- function `test_phokenize_coordinates_are_distinct_objects` (tests/test_phokenize.py:89) [test] — Function test_phokenize_coordinates_are_distinct_objects takes no inputs beyond the value it is called on.
- function `test_serialization_matches_legacy` (tests/test_phokenize.py:96) [test] — Function test_serialization_matches_legacy takes no inputs beyond the value it is called on.
- function `test_explode_implode_round_trip` (tests/test_phokenize.py:107) [test] — Function test_explode_implode_round_trip takes no inputs beyond the value it is called on.
- function `test_maps_and_index_match_legacy` (tests/test_phokenize.py:123) [test] — Function test_maps_and_index_match_legacy takes no inputs beyond the value it is called on.
- function `test_lookups_match_legacy` (tests/test_phokenize.py:135) [test] — Function test_lookups_match_legacy takes no inputs beyond the value it is called on.
- function `test_edits_match_legacy` (tests/test_phokenize.py:153) [test] — Function test_edits_match_legacy takes no inputs beyond the value it is called on.
- function `test_append_scroll_matches_legacy` (tests/test_phokenize.py:170) [test] — Function test_append_scroll_matches_legacy takes no inputs beyond the value it is called on.
- function `test_append_scroll_out_of_order` (tests/test_phokenize.py:187) [test] — Function test_append_scroll_out_of_order takes no inputs beyond the value it is called on.
- function `large_document` (tests/test_phokenize.py:195) [test] — Function large_document takes scrolls: int.
- function `test_explode_performance_50k_scrolls` (tests/test_phokenize.py:207) [test] — Function test_explode_performance_50k_scrolls takes no inputs beyond the value it is called on.
- function `test_scroll_interface` (tests/test_positionedScroll.py:12) [test] — Function test_scroll_interface takes no inputs beyond the value it is called on.
- function `test_positioned_scrolls` (tests/test_positionedScroll.py:19) [test] — Function test_positioned_scrolls takes no inputs beyond the value it is called on.
- function `test_phokenize` (tests/test_positionedScroll.py:39) [test] — Function test_phokenize takes no inputs beyond the value it is called on.
- function `test_range_exists` (tests/test_range.py:5) [test] — Function test_range_exists takes no inputs beyond the value it is called on.
- function `test_range_fixture_case` (tests/test_range_fixtures.py:55) [test] — Function test_range_fixture_case takes case_name.
- function `test_seal_round_trip_real_phext` (tests/test_seal.py:23) [test] — Function test_seal_round_trip_real_phext takes no inputs beyond the value it is called on.
- function `test_seal_round_trip_empty_and_small` (tests/test_seal.py:33) [test] — Function test_seal_round_trip_empty_and_small takes no inputs beyond the value it is called on.
- function `test_seal_carrying_binary_scrolls` (tests/test_seal.py:41) [test] — Function test_seal_carrying_binary_scrolls takes no inputs beyond the value it is called on.
- function `test_seal_is_deterministic` (tests/test_seal.py:50) [test] — Function test_seal_is_deterministic takes no inputs beyond the value it is called on.
- function `test_unseal_rejects_non_xz_data` (tests/test_seal.py:57) [test] — Function test_unseal_rejects_non_xz_data takes no inputs beyond the value it is called on.

---

## function — PointAddress.__str__

Found in phext/addressing.py at line 66.

Surface: public API.

Signature: `def __str__(self) -> str`

What it does: Function PointAddress.__str__ takes no inputs beyond the value it is called on. It gives back str.

(Where these words came from: inferred.)

---

## function — RangeAddress.__str__

Found in phext/addressing.py at line 86.

Surface: public API.

Signature: `def __str__(self) -> str`

What it does: Function RangeAddress.__str__ takes no inputs beyond the value it is called on. It gives back str.

(Where these words came from: inferred.)

---

## function — parse_point_address

Found in phext/addressing.py at line 119.

Surface: public API.

Signature: `def parse_point_address(address: str)`

What it does: Parses a v0.6.1 point address:

(Where these words came from: direct.)

---

## function — parse_range_address

Found in phext/addressing.py at line 190.

Surface: public API.

Signature: `def parse_range_address(address: str)`

What it does: Parses a v0.6.1 range address: exactly two point addresses joined

(Where these words came from: direct.)

---

## function — resolve_scroll_offset

Found in phext/addressing.py at line 212.

Surface: public API.

Signature: `def resolve_scroll_offset(text: str, pos: ScrollPosition)`

What it does: Resolves a caret position inside one scroll to a character

(Where these words came from: direct.)

---

## function — enumerate_scrolls

Found in phext/addressing.py at line 235.

Surface: public API.

Signature: `def enumerate_scrolls(member: str)`

What it does: The member's scrolls in the phokenize sense: a list of

(Where these words came from: direct.)

---

## function — addressed_scroll_text

Found in phext/addressing.py at line 245.

Surface: public API.

Signature: `def addressed_scroll_text(member: str, target: Coordinate)`

What it does: The scroll text at a coordinate in one member phext, in the

(Where these words came from: direct.)

---

## function — is_binary_scroll

Found in phext/binary.py at line 60.

Surface: public API.

Signature: `def is_binary_scroll(scroll: str) -> bool`

What it does: True when `scroll` carries the binary scroll header. Decoding

(Where these words came from: direct.)

---

## function — encode_scroll

Found in phext/binary.py at line 66.

Surface: public API.

Signature: `def encode_scroll(payload: bytes) -> str`

What it does: Encodes raw bytes as a binary scroll (header line + base64 body).

(Where these words came from: direct.)

---

## function — decode_scroll

Found in phext/binary.py at line 71.

Surface: public API.

Signature: `def decode_scroll(scroll: str)`

What it does: Decodes a binary scroll back to its raw bytes. Returns None when

(Where these words came from: direct.)

---

## function — insert_binary

Found in phext/binary.py at line 84.

Surface: public API.

Signature: `def insert_binary(buffer: str, location: Coordinate, payload: bytes) -> str`

What it does: Stores `payload` at `location` as a binary scroll, unconditionally:

(Where these words came from: direct.)

---

## function — replace_binary

Found in phext/binary.py at line 91.

Surface: public API.

Signature: `def replace_binary(buffer: str, location: Coordinate, payload: bytes) -> str`

What it does: Replaces the scroll at `location` with `payload`, stored as a

(Where these words came from: direct.)

---

## function — fetch_binary

Found in phext/binary.py at line 97.

Surface: public API.

Signature: `def fetch_binary(buffer: str, target: Coordinate) -> bytes`

What it does: Retrieves the bytes held at `target`. A well-formed binary scroll

(Where these words came from: direct.)

---

## function — insert_bytes

Found in phext/binary.py at line 120.

Surface: public API.

Signature: `def insert_bytes(buffer: str, location: Coordinate, payload: bytes) -> str`

What it does: Byte-oriented insert: stores `payload` at `location` under the

(Where these words came from: direct.)

---

## function — replace_bytes

Found in phext/binary.py at line 127.

Surface: public API.

Signature: `def replace_bytes(buffer: str, location: Coordinate, payload: bytes) -> str`

What it does: Byte-oriented replace: stores `payload` at `location` under the

(Where these words came from: direct.)

---

## function — phokenize_bytes

Found in phext/binary.py at line 141.

Surface: public API.

Signature: `def phokenize_bytes(data: bytes)`

What it does: The byte-level twin of the reference `phokenize`: walks a raw byte

(Where these words came from: direct.)

---

## function — ingest

Found in phext/binary.py at line 172.

Surface: public API.

Signature: `def ingest(data: bytes) -> str`

What it does: Ingests an externally supplied phext byte stream. Every scroll is

(Where these words came from: direct.)

---

## function — Chain.serialize

Found in phext/chain.py at line 50.

Surface: public API.

Signature: `def serialize(self) -> str`

What it does: Serializes this chain alone: its members joined by

(Where these words came from: direct.)

---

## function — Chain.fetch

Found in phext/chain.py at line 59.

Surface: public API.

Signature: `def fetch(self, address: str) -> str`

What it does: Fetch a scroll by multiverse address: ``$N:coord``, or a bare

(Where these words came from: direct.)

---

## function — Chain.resolve_point

Found in phext/chain.py at line 72.

Surface: public API.

Signature: `def resolve_point(self, point)`

What it does: Resolves a point address against this chain: (universe,

(Where these words came from: direct.)

---

## function — Chain.fetch_point

Found in phext/chain.py at line 93.

Surface: public API.

Signature: `def fetch_point(self, address: str) -> str`

What it does: Fetches a point address string. A bare point is a caret: it

(Where these words came from: direct.)

---

## function — Chain.resolve_range

Found in phext/chain.py at line 116.

Surface: public API.

Signature: `def resolve_range(self, range_address)`

What it does: Resolves a range against this chain into per-scroll segments:

(Where these words came from: direct.)

---

## function — Chain.fetch_range

Found in phext/chain.py at line 170.

Surface: public API.

Signature: `def fetch_range(self, address: str) -> str`

What it does: Fetches a range address string: the spanned text with phext

(Where these words came from: direct.)

---

## function — parse_chains

Found in phext/chain.py at line 197.

Surface: public API.

Signature: `def parse_chains(stream: str)`

What it does: Splits a stream into its chains. CHAIN_BREAK separates the member

(Where these words came from: direct.)

---

## function — serialize_chains

Found in phext/chain.py at line 221.

Surface: public API.

Signature: `def serialize_chains(chains) -> str`

What it does: Serializes a stream of chains back into stream bytes: the inverse

(Where these words came from: direct.)

---

## function — parse_universe_address

Found in phext/chain.py at line 227.

Surface: public API.

Signature: `def parse_universe_address(address: str)`

What it does: Parses a multiverse address into (universe, Coordinate), or None

(Where these words came from: direct.)

---

## function — advance_coordinate

Found in phext/coordinate.py at line 4.

Surface: public API.

Signature: `def advance_coordinate(index: int) -> int`

What it does: Function advance_coordinate takes index: int. It gives back int.

(Where these words came from: inferred.)

---

## function — Coordinate.from_string

Found in phext/coordinate.py at line 29.

Surface: public API.

Signature: `def from_string(ignored, coord_str: str)`

What it does: Function Coordinate.from_string takes ignored, coord_str: str. Its return type is unannotated; the code shows it returning Coordinate(nums[0], nums[1], nums[2], nums[3], n.

(Where these words came from: inferred.)

---

## function — Coordinate.libraryBreak

Found in phext/coordinate.py at line 50.

Surface: public API.

Signature: `def libraryBreak(self)`

What it does: Function Coordinate.libraryBreak takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Coordinate.shelfBreak

Found in phext/coordinate.py at line 61.

Surface: public API.

Signature: `def shelfBreak(self)`

What it does: Function Coordinate.shelfBreak takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Coordinate.seriesBreak

Found in phext/coordinate.py at line 71.

Surface: public API.

Signature: `def seriesBreak(self)`

What it does: Function Coordinate.seriesBreak takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Coordinate.collectionBreak

Found in phext/coordinate.py at line 80.

Surface: public API.

Signature: `def collectionBreak(self)`

What it does: Function Coordinate.collectionBreak takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Coordinate.volumeBreak

Found in phext/coordinate.py at line 88.

Surface: public API.

Signature: `def volumeBreak(self)`

What it does: Function Coordinate.volumeBreak takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Coordinate.bookBreak

Found in phext/coordinate.py at line 95.

Surface: public API.

Signature: `def bookBreak(self)`

What it does: Function Coordinate.bookBreak takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Coordinate.chapterBreak

Found in phext/coordinate.py at line 101.

Surface: public API.

Signature: `def chapterBreak(self)`

What it does: Function Coordinate.chapterBreak takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Coordinate.sectionBreak

Found in phext/coordinate.py at line 106.

Surface: public API.

Signature: `def sectionBreak(self)`

What it does: Function Coordinate.sectionBreak takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Coordinate.scrollBreak

Found in phext/coordinate.py at line 110.

Surface: public API.

Signature: `def scrollBreak(self)`

What it does: Function Coordinate.scrollBreak takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Coordinate.__init__

Found in phext/coordinate.py at line 113.

Surface: public API.

Signature: `def __init__(self, lb, sf, sr, cn, vm, bk, ch, sn, sc)`

What it does: Function Coordinate.__init__ takes self, lb, sf, sr, cn, vm, bk, ch, sn, sc. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Coordinate.copy_coordinate

Found in phext/coordinate.py at line 124.

Surface: public API.

Signature: `def copy_coordinate(self, other)`

What it does: Function Coordinate.copy_coordinate takes self, other. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Coordinate.__str__

Found in phext/coordinate.py at line 135.

Surface: public API.

Signature: `def __str__(self) -> str`

What it does: Function Coordinate.__str__ takes no inputs beyond the value it is called on. It gives back str.

(Where these words came from: inferred.)

---

## function — Coordinate.urlencoded

Found in phext/coordinate.py at line 140.

Surface: public API.

Signature: `def urlencoded(self) -> str`

What it does: Function Coordinate.urlencoded takes no inputs beyond the value it is called on. It gives back str.

(Where these words came from: inferred.)

---

## function — Coordinate.__eq__

Found in phext/coordinate.py at line 145.

Surface: public API.

Signature: `def __eq__(self, other)`

What it does: Function Coordinate.__eq__ takes self, other. Its return type is unannotated; the code shows it returning isinstance(self, Coordinate) and isinstance(othe.

(Where these words came from: inferred.)

---

## function — Coordinate.__lt__

Found in phext/coordinate.py at line 148.

Surface: public API.

Signature: `def __lt__(self, other)`

What it does: Function Coordinate.__lt__ takes self, other. Its return type is unannotated; the code shows it returning NotImplemented; self.as_tuple() < other.as_tuple().

(Where these words came from: inferred.)

---

## function — Coordinate.as_tuple

Found in phext/coordinate.py at line 153.

Surface: public API.

Signature: `def as_tuple(self) -> tuple`

What it does: Function Coordinate.as_tuple takes no inputs beyond the value it is called on. It gives back tuple.

(Where these words came from: inferred.)

---

## function — Coordinate.__hash__

Found in phext/coordinate.py at line 160.

Surface: public API.

Signature: `def __hash__(self)`

What it does: Function Coordinate.__hash__ takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning hash((.

(Where these words came from: inferred.)

---

## function — IndexedPhext.__init__

Found in phext/indexed.py at line 31.

Surface: public API.

Signature: `def __init__(self, buffer: str)`

What it does: Function IndexedPhext.__init__ takes self, buffer: str. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — IndexedPhext.fetch

Found in phext/indexed.py at line 42.

Surface: public API.

Signature: `def fetch(self, coord: Coordinate) -> str`

What it does: Function IndexedPhext.fetch takes self, coord: Coordinate. It gives back str.

(Where these words came from: inferred.)

---

## function — IndexedPhext.__contains__

Found in phext/indexed.py at line 49.

Surface: public API.

Signature: `def __contains__(self, coord: Coordinate) -> bool`

What it does: Function IndexedPhext.__contains__ takes self, coord: Coordinate. It gives back bool.

(Where these words came from: inferred.)

---

## function — IndexedPhext.coordinates

Found in phext/indexed.py at line 52.

Surface: public API.

Signature: `def coordinates(self) -> list[Coordinate]`

What it does: Function IndexedPhext.coordinates takes no inputs beyond the value it is called on. It gives back list[Coordinate].

(Where these words came from: inferred.)

---

## function — IndexedPhext.keys

Found in phext/indexed.py at line 55.

Surface: public API.

Signature: `def keys(self) -> list[Coordinate]`

What it does: Function IndexedPhext.keys takes no inputs beyond the value it is called on. It gives back list[Coordinate].

(Where these words came from: inferred.)

---

## function — IndexedPhext.index

Found in phext/indexed.py at line 58.

Surface: public API.

Signature: `def index(self) -> dict[Coordinate, tuple[int, int]]`

What it does: Function IndexedPhext.index takes no inputs beyond the value it is called on. It gives back dict[Coordinate, tuple[int, int]].

(Where these words came from: inferred.)

---

## function — IndexedPhext.explode

Found in phext/indexed.py at line 62.

Surface: public API.

Signature: `def explode(self) -> dict[Coordinate, str]`

What it does: Function IndexedPhext.explode takes no inputs beyond the value it is called on. It gives back dict[Coordinate, str].

(Where these words came from: inferred.)

---

## function — IndexedPhext.__len__

Found in phext/indexed.py at line 68.

Surface: public API.

Signature: `def __len__(self) -> int`

What it does: Function IndexedPhext.__len__ takes no inputs beyond the value it is called on. It gives back int.

(Where these words came from: inferred.)

---

## function — Phext.defaultCoordinate

Found in phext/phext.py at line 61.

Surface: public API.

Signature: `def defaultCoordinate(self) -> Coordinate`

What it does: Function Phext.defaultCoordinate takes no inputs beyond the value it is called on. It gives back Coordinate.

(Where these words came from: inferred.)

---

## function — Phext.__init__

Found in phext/phext.py at line 64.

Surface: public API.

Signature: `def __init__(self)`

What it does: Function Phext.__init__ takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Phext.fetch

Found in phext/phext.py at line 67.

Surface: public API.

Signature: `def fetch(self, buffer:str, coord:Coordinate) -> dict[Coordinate, str]`

What it does: Function Phext.fetch takes self, buffer:str, coord:Coordinate. It gives back dict[Coordinate, str].

(Where these words came from: inferred.)

---

## function — Phext.isPhextBreak

Found in phext/phext.py at line 73.

Surface: public API.

Signature: `def isPhextBreak(self, byte:str)`

What it does: Function Phext.isPhextBreak takes self, byte:str. Its return type is unannotated; the code shows it returning byte == self.LINE_BREAK or \.

(Where these words came from: inferred.)

---

## function — Phext.phokenize

Found in phext/phext.py at line 111.

Surface: public API.

Signature: `def phokenize(self, buffer:str) -> List[PositionedScroll]`

What it does: Function Phext.phokenize takes self, buffer:str. It gives back List[PositionedScroll].

(Where these words came from: inferred.)

---

## function — Phext.append_scroll

Found in phext/phext.py at line 177.

Surface: public API.

Signature: `def append_scroll(self, entry: PositionedScroll, location: Coordinate) -> PositionedScroll`

What it does: Function Phext.append_scroll takes self, entry: PositionedScroll, location: Coordinate. It gives back PositionedScroll.

(Where these words came from: inferred.)

---

## function — Phext.dephokenize

Found in phext/phext.py at line 183.

Surface: public API.

Signature: `def dephokenize(self, stack: list[PositionedScroll]) -> str`

What it does: Function Phext.dephokenize takes self, stack: list[PositionedScroll]. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.normalize

Found in phext/phext.py at line 191.

Surface: public API.

Signature: `def normalize(self, buffer:str) -> str`

What it does: Function Phext.normalize takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.update

Found in phext/phext.py at line 195.

Surface: public API.

Signature: `def update(self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool) -> str`

What it does: Function Phext.update takes self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.insert

Found in phext/phext.py at line 218.

Surface: public API.

Signature: `def insert(self, buffer:str, coord:Coordinate, scroll:str) -> str`

What it does: Function Phext.insert takes self, buffer:str, coord:Coordinate, scroll:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.replace

Found in phext/phext.py at line 221.

Surface: public API.

Signature: `def replace(self, buffer:str, coord:Coordinate, scroll:str) -> str`

What it does: Function Phext.replace takes self, buffer:str, coord:Coordinate, scroll:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.remove

Found in phext/phext.py at line 224.

Surface: public API.

Signature: `def remove(self, buffer:str, coord:Coordinate) -> str`

What it does: Function Phext.remove takes self, buffer:str, coord:Coordinate. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.range_replace

Found in phext/phext.py at line 228.

Surface: public API.

Signature: `def range_replace(self, buffer:str, range:Range, text:str) -> str`

What it does: Function Phext.range_replace takes self, buffer:str, range:Range, text:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.next_scroll

Found in phext/phext.py at line 244.

Surface: public API.

Signature: `def next_scroll(self, buffer:str, coord:Coordinate) -> List[PositionedScroll]`

What it does: Function Phext.next_scroll takes self, buffer:str, coord:Coordinate. It gives back List[PositionedScroll].

(Where these words came from: inferred.)

---

## function — Phext.get_subspace_coordinates

Found in phext/phext.py at line 256.

Surface: public API.

Signature: `def get_subspace_coordinates(self, buffer:str, target:Coordinate)`

What it does: Function Phext.get_subspace_coordinates takes self, buffer:str, target:Coordinate. Its return type is unannotated; the code shows it returning result.

(Where these words came from: inferred.)

---

## function — Phext.merge

Found in phext/phext.py at line 310.

Surface: public API.

Signature: `def merge(self, left: str, right:str) -> str`

What it does: Function Phext.merge takes self, left: str, right:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.subtract

Found in phext/phext.py at line 342.

Surface: public API.

Signature: `def subtract(self, left:str, right:str) -> str`

What it does: Function Phext.subtract takes self, left:str, right:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.expand

Found in phext/phext.py at line 363.

Surface: public API.

Signature: `def expand(self, buffer:str) -> str`

What it does: Function Phext.expand takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.contract

Found in phext/phext.py at line 366.

Surface: public API.

Signature: `def contract(self, buffer:str) -> str`

What it does: Function Phext.contract takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.create_summary

Found in phext/phext.py at line 369.

Surface: public API.

Signature: `def create_summary(self, buffer:str) -> str`

What it does: Function Phext.create_summary takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.navmap

Found in phext/phext.py at line 377.

Surface: public API.

Signature: `def navmap(self, urlbase:str, buffer:str) -> str`

What it does: Function Phext.navmap takes self, urlbase:str, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.textmap

Found in phext/phext.py at line 391.

Surface: public API.

Signature: `def textmap(self, buffer:str) -> str`

What it does: Function Phext.textmap takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.checksum

Found in phext/phext.py at line 398.

Surface: public API.

Signature: `def checksum(self, buffer:str) -> str`

What it does: Function Phext.checksum takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.manifest

Found in phext/phext.py at line 404.

Surface: public API.

Signature: `def manifest(self, buffer:str) -> str`

What it does: Function Phext.manifest takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.soundex_v1

Found in phext/phext.py at line 411.

Surface: public API.

Signature: `def soundex_v1(self, buffer:str) -> str`

What it does: Function Phext.soundex_v1 takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.soundex_internal

Found in phext/phext.py at line 419.

Surface: public API.

Signature: `def soundex_internal(self, buffer:str) -> str`

What it does: Function Phext.soundex_internal takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.explode

Found in phext/phext.py at line 443.

Surface: public API.

Signature: `def explode(self, buffer:str) -> dict[Coordinate, str]`

What it does: Function Phext.explode takes self, buffer:str. It gives back dict[Coordinate, str].

(Where these words came from: inferred.)

---

## function — Phext.implode

Found in phext/phext.py at line 450.

Surface: public API.

Signature: `def implode(self, hash: dict[Coordinate, str]) -> str`

What it does: Function Phext.implode takes self, hash: dict[Coordinate, str]. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.index_phokens

Found in phext/phext.py at line 456.

Surface: public API.

Signature: `def index_phokens(self, buffer:str) -> list[PositionedScroll]`

What it does: Function Phext.index_phokens takes self, buffer:str. It gives back list[PositionedScroll].

(Where these words came from: inferred.)

---

## function — Phext.index

Found in phext/phext.py at line 475.

Surface: public API.

Signature: `def index(self, buffer:str) -> str`

What it does: Function Phext.index takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.offset

Found in phext/phext.py at line 479.

Surface: public API.

Signature: `def offset(self, buffer:str, coord:Coordinate) -> int`

What it does: Function Phext.offset takes self, buffer:str, coord:Coordinate. It gives back int.

(Where these words came from: inferred.)

---

## function — Phext.index_offsets

Found in phext/phext.py at line 494.

Surface: public API.

Signature: `def index_offsets(self, buffer:str) -> dict[Coordinate, tuple[int, int]]`

What it does: Function Phext.index_offsets takes self, buffer:str. It gives back dict[Coordinate, tuple[int, int]].

(Where these words came from: inferred.)

---

## function — Phext.indexed

Found in phext/phext.py at line 507.

Surface: public API.

Signature: `def indexed(self, buffer:str)`

What it does: Function Phext.indexed takes self, buffer:str. Its return type is unannotated; the code shows it returning IndexedPhext(buffer).

(Where these words came from: inferred.)

---

## function — Phext.insert_binary

Found in phext/phext.py at line 519.

Surface: public API.

Signature: `def insert_binary(self, buffer: str, coord: Coordinate, payload: bytes) -> str`

What it does: Function Phext.insert_binary takes self, buffer: str, coord: Coordinate, payload: bytes. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.replace_binary

Found in phext/phext.py at line 523.

Surface: public API.

Signature: `def replace_binary(self, buffer: str, coord: Coordinate, payload: bytes) -> str`

What it does: Function Phext.replace_binary takes self, buffer: str, coord: Coordinate, payload: bytes. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.fetch_binary

Found in phext/phext.py at line 527.

Surface: public API.

Signature: `def fetch_binary(self, buffer: str, coord: Coordinate) -> bytes`

What it does: Function Phext.fetch_binary takes self, buffer: str, coord: Coordinate. It gives back bytes.

(Where these words came from: inferred.)

---

## function — Phext.insert_bytes

Found in phext/phext.py at line 531.

Surface: public API.

Signature: `def insert_bytes(self, buffer: str, coord: Coordinate, payload: bytes) -> str`

What it does: Function Phext.insert_bytes takes self, buffer: str, coord: Coordinate, payload: bytes. It gives back str.

(Where these words came from: inferred.)

---

## function — Phext.replace_bytes

Found in phext/phext.py at line 535.

Surface: public API.

Signature: `def replace_bytes(self, buffer: str, coord: Coordinate, payload: bytes) -> str`

What it does: Function Phext.replace_bytes takes self, buffer: str, coord: Coordinate, payload: bytes. It gives back str.

(Where these words came from: inferred.)

---

## function — PositionedScroll.__init__

Found in phext/positionedScroll.py at line 9.

Surface: public API.

Signature: `def __init__(self, coord, text)`

What it does: Function PositionedScroll.__init__ takes self, coord, text. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — seal

Found in phext/seal.py at line 26.

Surface: public API.

Signature: `def seal(text: str) -> bytes`

What it does: Seals a phext: xz-compresses its full UTF-8 bytes.

(Where these words came from: direct.)

---

## function — seal_bytes

Found in phext/seal.py at line 31.

Surface: public API.

Signature: `def seal_bytes(data: bytes) -> bytes`

What it does: Seals raw phext bytes (see `seal`).

(Where these words came from: direct.)

---

## function — unseal

Found in phext/seal.py at line 36.

Surface: public API.

Signature: `def unseal(data: bytes) -> bytes`

What it does: Unseals a sealed phext back to its exact original bytes. Raises

(Where these words came from: direct.)

---

## function — unseal_string

Found in phext/seal.py at line 42.

Surface: public API.

Signature: `def unseal_string(data: bytes) -> str`

What it does: Unseals directly to text. Raises when the unsealed bytes are not

(Where these words came from: direct.)

---

## function — timed

Found in test/bench.py at line 17.

Surface: public API.

Signature: `def timed(label, fn)`

What it does: Function timed takes label, fn. Its return type is unannotated; the code shows it returning result.

(Where these words came from: inferred.)

---

## function — main

Found in test/bench.py at line 24.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — LegacyPhext.defaultCoordinate

Found in tests/legacy_phext.py at line 38.

Surface: test.

Signature: `def defaultCoordinate(self) -> Coordinate`

What it does: Function LegacyPhext.defaultCoordinate takes no inputs beyond the value it is called on. It gives back Coordinate.

(Where these words came from: inferred.)

---

## function — LegacyPhext.__init__

Found in tests/legacy_phext.py at line 41.

Surface: test.

Signature: `def __init__(self)`

What it does: Function LegacyPhext.__init__ takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — LegacyPhext.fetch

Found in tests/legacy_phext.py at line 44.

Surface: test.

Signature: `def fetch(self, buffer:str, coord:Coordinate) -> dict[Coordinate, str]`

What it does: Function LegacyPhext.fetch takes self, buffer:str, coord:Coordinate. It gives back dict[Coordinate, str].

(Where these words came from: inferred.)

---

## function — LegacyPhext.isPhextBreak

Found in tests/legacy_phext.py at line 51.

Surface: test.

Signature: `def isPhextBreak(self, byte:str)`

What it does: Function LegacyPhext.isPhextBreak takes self, byte:str. Its return type is unannotated; the code shows it returning byte == self.LINE_BREAK or \.

(Where these words came from: inferred.)

---

## function — LegacyPhext.phokenize

Found in tests/legacy_phext.py at line 63.

Surface: test.

Signature: `def phokenize(self, buffer:str) -> List[PositionedScroll]`

What it does: Function LegacyPhext.phokenize takes self, buffer:str. It gives back List[PositionedScroll].

(Where these words came from: inferred.)

---

## function — LegacyPhext.append_scroll

Found in tests/legacy_phext.py at line 114.

Surface: test.

Signature: `def append_scroll(self, entry: PositionedScroll, location: Coordinate) -> PositionedScroll`

What it does: Function LegacyPhext.append_scroll takes self, entry: PositionedScroll, location: Coordinate. It gives back PositionedScroll.

(Where these words came from: inferred.)

---

## function — LegacyPhext.dephokenize

Found in tests/legacy_phext.py at line 159.

Surface: test.

Signature: `def dephokenize(self, stack: list[PositionedScroll]) -> str`

What it does: Function LegacyPhext.dephokenize takes self, stack: list[PositionedScroll]. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.normalize

Found in tests/legacy_phext.py at line 168.

Surface: test.

Signature: `def normalize(self, buffer:str) -> str`

What it does: Function LegacyPhext.normalize takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.update

Found in tests/legacy_phext.py at line 172.

Surface: test.

Signature: `def update(self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool) -> str`

What it does: Function LegacyPhext.update takes self, buffer:str, coord:Coordinate, scroll:str, overwrite:bool. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.insert

Found in tests/legacy_phext.py at line 195.

Surface: test.

Signature: `def insert(self, buffer:str, coord:Coordinate, scroll:str) -> str`

What it does: Function LegacyPhext.insert takes self, buffer:str, coord:Coordinate, scroll:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.replace

Found in tests/legacy_phext.py at line 198.

Surface: test.

Signature: `def replace(self, buffer:str, coord:Coordinate, scroll:str) -> str`

What it does: Function LegacyPhext.replace takes self, buffer:str, coord:Coordinate, scroll:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.remove

Found in tests/legacy_phext.py at line 201.

Surface: test.

Signature: `def remove(self, buffer:str, coord:Coordinate) -> str`

What it does: Function LegacyPhext.remove takes self, buffer:str, coord:Coordinate. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.range_replace

Found in tests/legacy_phext.py at line 205.

Surface: test.

Signature: `def range_replace(self, buffer:str, range:Range, text:str) -> str`

What it does: Function LegacyPhext.range_replace takes self, buffer:str, range:Range, text:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.next_scroll

Found in tests/legacy_phext.py at line 221.

Surface: test.

Signature: `def next_scroll(self, buffer:str, coord:Coordinate) -> List[PositionedScroll]`

What it does: Function LegacyPhext.next_scroll takes self, buffer:str, coord:Coordinate. It gives back List[PositionedScroll].

(Where these words came from: inferred.)

---

## function — LegacyPhext.get_subspace_coordinates

Found in tests/legacy_phext.py at line 234.

Surface: test.

Signature: `def get_subspace_coordinates(self, buffer:str, target:Coordinate)`

What it does: Function LegacyPhext.get_subspace_coordinates takes self, buffer:str, target:Coordinate. Its return type is unannotated; the code shows it returning result.

(Where these words came from: inferred.)

---

## function — LegacyPhext.merge

Found in tests/legacy_phext.py at line 297.

Surface: test.

Signature: `def merge(self, left: str, right:str) -> str`

What it does: Function LegacyPhext.merge takes self, left: str, right:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.subtract

Found in tests/legacy_phext.py at line 329.

Surface: test.

Signature: `def subtract(self, left:str, right:str) -> str`

What it does: Function LegacyPhext.subtract takes self, left:str, right:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.expand

Found in tests/legacy_phext.py at line 350.

Surface: test.

Signature: `def expand(self, buffer:str) -> str`

What it does: Function LegacyPhext.expand takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.contract

Found in tests/legacy_phext.py at line 375.

Surface: test.

Signature: `def contract(self, buffer:str) -> str`

What it does: Function LegacyPhext.contract takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.create_summary

Found in tests/legacy_phext.py at line 400.

Surface: test.

Signature: `def create_summary(self, buffer:str) -> str`

What it does: Function LegacyPhext.create_summary takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.navmap

Found in tests/legacy_phext.py at line 414.

Surface: test.

Signature: `def navmap(self, urlbase:str, buffer:str) -> str`

What it does: Function LegacyPhext.navmap takes self, urlbase:str, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.textmap

Found in tests/legacy_phext.py at line 428.

Surface: test.

Signature: `def textmap(self, buffer:str) -> str`

What it does: Function LegacyPhext.textmap takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.checksum

Found in tests/legacy_phext.py at line 436.

Surface: test.

Signature: `def checksum(self, buffer:str) -> str`

What it does: Function LegacyPhext.checksum takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.manifest

Found in tests/legacy_phext.py at line 440.

Surface: test.

Signature: `def manifest(self, buffer:str) -> str`

What it does: Function LegacyPhext.manifest takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.soundex_v1

Found in tests/legacy_phext.py at line 447.

Surface: test.

Signature: `def soundex_v1(self, buffer:str) -> str`

What it does: Function LegacyPhext.soundex_v1 takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.soundex_internal

Found in tests/legacy_phext.py at line 455.

Surface: test.

Signature: `def soundex_internal(self, buffer:str) -> str`

What it does: Function LegacyPhext.soundex_internal takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.explode

Found in tests/legacy_phext.py at line 479.

Surface: test.

Signature: `def explode(self, buffer:str) -> dict[Coordinate, str]`

What it does: Function LegacyPhext.explode takes self, buffer:str. It gives back dict[Coordinate, str].

(Where these words came from: inferred.)

---

## function — LegacyPhext.implode

Found in tests/legacy_phext.py at line 486.

Surface: test.

Signature: `def implode(self, hash: dict[Coordinate, str]) -> str`

What it does: Function LegacyPhext.implode takes self, hash: dict[Coordinate, str]. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.index_phokens

Found in tests/legacy_phext.py at line 492.

Surface: test.

Signature: `def index_phokens(self, buffer:str) -> list[PositionedScroll]`

What it does: Function LegacyPhext.index_phokens takes self, buffer:str. It gives back list[PositionedScroll].

(Where these words came from: inferred.)

---

## function — LegacyPhext.index

Found in tests/legacy_phext.py at line 532.

Surface: test.

Signature: `def index(self, buffer:str) -> str`

What it does: Function LegacyPhext.index takes self, buffer:str. It gives back str.

(Where these words came from: inferred.)

---

## function — LegacyPhext.offset

Found in tests/legacy_phext.py at line 536.

Surface: test.

Signature: `def offset(self, buffer:str, coord:Coordinate) -> int`

What it does: Function LegacyPhext.offset takes self, buffer:str, coord:Coordinate. It gives back int.

(Where these words came from: inferred.)

---

## function — home

Found in tests/test_address.py at line 15.

Surface: test.

Signature: `def home()`

What it does: Function home takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning Coordinate.from_string("1.1.1/1.1.1/1.1.1").

(Where these words came from: inferred.)

---

## function — scroll2

Found in tests/test_address.py at line 19.

Surface: test.

Signature: `def scroll2()`

What it does: Function scroll2 takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning Coordinate.from_string("1.1.1/1.1.1/1.1.2").

(Where these words came from: inferred.)

---

## function — member_a

Found in tests/test_address.py at line 23.

Surface: test.

Signature: `def member_a()`

What it does: Function member_a takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning member.

(Where these words came from: inferred.)

---

## function — member_b

Found in tests/test_address.py at line 29.

Surface: test.

Signature: `def member_b()`

What it does: Function member_b takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning P.insert("", home(), "uni2 home\nsecond line").

(Where these words came from: inferred.)

---

## function — make_chain

Found in tests/test_address.py at line 33.

Surface: test.

Signature: `def make_chain()`

What it does: Function make_chain takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning Chain(members=[member_a(), member_b()], closed=F.

(Where these words came from: inferred.)

---

## function — test_point_parse_canonical

Found in tests/test_address.py at line 37.

Surface: test.

Signature: `def test_point_parse_canonical()`

What it does: Function test_point_parse_canonical takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_point_parse_rejects

Found in tests/test_address.py at line 60.

Surface: test.

Signature: `def test_point_parse_rejects()`

What it does: Function test_point_parse_rejects takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_point_resolve

Found in tests/test_address.py at line 77.

Surface: test.

Signature: `def test_point_resolve()`

What it does: Function test_point_resolve takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_point_fetch_offset

Found in tests/test_address.py at line 95.

Surface: test.

Signature: `def test_point_fetch_offset()`

What it does: Function test_point_fetch_offset takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_point_fetch_offset_unicode

Found in tests/test_address.py at line 110.

Surface: test.

Signature: `def test_point_fetch_offset_unicode()`

What it does: Function test_point_fetch_offset_unicode takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_range_parse_canonical

Found in tests/test_address.py at line 118.

Surface: test.

Signature: `def test_range_parse_canonical()`

What it does: Function test_range_parse_canonical takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_range_fetch_within_scroll

Found in tests/test_address.py at line 133.

Surface: test.

Signature: `def test_range_fetch_within_scroll()`

What it does: Function test_range_fetch_within_scroll takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_range_fetch_cross_scroll

Found in tests/test_address.py at line 145.

Surface: test.

Signature: `def test_range_fetch_cross_scroll()`

What it does: Function test_range_fetch_cross_scroll takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_range_fetch_multiverse

Found in tests/test_address.py at line 157.

Surface: test.

Signature: `def test_range_fetch_multiverse()`

What it does: Function test_range_fetch_multiverse takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_range_resolution_errors

Found in tests/test_address.py at line 176.

Surface: test.

Signature: `def test_range_resolution_errors()`

What it does: Function test_range_resolution_errors takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — coord

Found in tests/test_binary.py at line 11.

Surface: test.

Signature: `def coord(address)`

What it does: Function coord takes address. Its return type is unannotated; the code shows it returning Coordinate.from_string(address).

(Where these words came from: inferred.)

---

## function — test_encode_decode_round_trip

Found in tests/test_binary.py at line 15.

Surface: test.

Signature: `def test_encode_decode_round_trip()`

What it does: Function test_encode_decode_round_trip takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_decode_scroll_rejects_non_binary

Found in tests/test_binary.py at line 39.

Surface: test.

Signature: `def test_decode_scroll_rejects_non_binary()`

What it does: Function test_decode_scroll_rejects_non_binary takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_insert_and_fetch_binary

Found in tests/test_binary.py at line 51.

Surface: test.

Signature: `def test_insert_and_fetch_binary()`

What it does: Function test_insert_and_fetch_binary takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_fetch_binary_fallbacks

Found in tests/test_binary.py at line 68.

Surface: test.

Signature: `def test_fetch_binary_fallbacks()`

What it does: Function test_fetch_binary_fallbacks takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_update_rule_non_utf8_is_base64_verbatim

Found in tests/test_binary.py at line 80.

Surface: test.

Signature: `def test_update_rule_non_utf8_is_base64_verbatim()`

What it does: Function test_update_rule_non_utf8_is_base64_verbatim takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_update_rule_utf8_stays_text

Found in tests/test_binary.py at line 97.

Surface: test.

Signature: `def test_update_rule_utf8_stays_text()`

What it does: Function test_update_rule_utf8_stays_text takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_ingest_foreign_byte_stream

Found in tests/test_binary.py at line 105.

Surface: test.

Signature: `def test_ingest_foreign_byte_stream()`

What it does: Function test_ingest_foreign_byte_stream takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_ingest_canonical_phext_is_identity

Found in tests/test_binary.py at line 119.

Surface: test.

Signature: `def test_ingest_canonical_phext_is_identity()`

What it does: Function test_ingest_canonical_phext_is_identity takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_ingest_empty_and_break_only_streams

Found in tests/test_binary.py at line 126.

Surface: test.

Signature: `def test_ingest_empty_and_break_only_streams()`

What it does: Function test_ingest_empty_and_break_only_streams takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_phokenize_bytes_matches_phokenize

Found in tests/test_binary.py at line 134.

Surface: test.

Signature: `def test_phokenize_bytes_matches_phokenize()`

What it does: Function test_phokenize_bytes_matches_phokenize takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_phokenize_bytes_matches_literal_reference_on_random_streams

Found in tests/test_binary.py at line 182.

Surface: test.

Signature: `def test_phokenize_bytes_matches_literal_reference_on_random_streams()`

What it does: Function test_phokenize_bytes_matches_literal_reference_on_random_streams takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — coord

Found in tests/test_chain.py at line 12.

Surface: test.

Signature: `def coord(address)`

What it does: Function coord takes address. Its return type is unannotated; the code shows it returning Coordinate.from_string(address).

(Where these words came from: inferred.)

---

## function — roundtrip

Found in tests/test_chain.py at line 16.

Surface: test.

Signature: `def roundtrip(stream)`

What it does: Function roundtrip takes stream. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_chain_free_input_is_one_unclosed_chain

Found in tests/test_chain.py at line 21.

Surface: test.

Signature: `def test_chain_free_input_is_one_unclosed_chain()`

What it does: Function test_chain_free_input_is_one_unclosed_chain takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_two_phexts_chained_and_closed_round_trip

Found in tests/test_chain.py at line 38.

Surface: test.

Signature: `def test_two_phexts_chained_and_closed_round_trip()`

What it does: Function test_two_phexts_chained_and_closed_round_trip takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_unclosed_chain_at_eof_reports_truncated

Found in tests/test_chain.py at line 47.

Surface: test.

Signature: `def test_unclosed_chain_at_eof_reports_truncated()`

What it does: Function test_unclosed_chain_at_eof_reports_truncated takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_two_chains_in_one_stream_parse_as_two_units

Found in tests/test_chain.py at line 55.

Surface: test.

Signature: `def test_two_chains_in_one_stream_parse_as_two_units()`

What it does: Function test_two_chains_in_one_stream_parse_as_two_units takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_chained_members_carry_full_phexts

Found in tests/test_chain.py at line 75.

Surface: test.

Signature: `def test_chained_members_carry_full_phexts()`

What it does: Function test_chained_members_carry_full_phexts takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_editions_in_succession_read_in_order

Found in tests/test_chain.py at line 90.

Surface: test.

Signature: `def test_editions_in_succession_read_in_order()`

What it does: Function test_editions_in_succession_read_in_order takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_empty_and_break_only_streams_round_trip

Found in tests/test_chain.py at line 100.

Surface: test.

Signature: `def test_empty_and_break_only_streams_round_trip()`

What it does: Function test_empty_and_break_only_streams_round_trip takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_serialize_from_constructed_chains

Found in tests/test_chain.py at line 132.

Surface: test.

Signature: `def test_serialize_from_constructed_chains()`

What it does: Function test_serialize_from_constructed_chains takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_chain_layer_does_not_touch_member_structure

Found in tests/test_chain.py at line 144.

Surface: test.

Signature: `def test_chain_layer_does_not_touch_member_structure()`

What it does: Function test_chain_layer_does_not_touch_member_structure takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_multiverse_addressing

Found in tests/test_chain.py at line 153.

Surface: test.

Signature: `def test_multiverse_addressing()`

What it does: Function test_multiverse_addressing takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_adjacent_closes_are_two_empty_chains

Found in tests/test_chain.py at line 167.

Surface: test.

Signature: `def test_adjacent_closes_are_two_empty_chains()`

What it does: Function test_adjacent_closes_are_two_empty_chains takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_chain_fixture_case

Found in tests/test_chain_fixtures.py at line 28.

Surface: test.

Signature: `def test_chain_fixture_case(case)`

What it does: Function test_chain_fixture_case takes case. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_valid_home_coordinate

Found in tests/test_coordinate.py at line 17.

Surface: test.

Signature: `def test_valid_home_coordinate()`

What it does: Function test_valid_home_coordinate takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_independent_coordinates

Found in tests/test_coordinate.py at line 29.

Surface: test.

Signature: `def test_independent_coordinates()`

What it does: Function test_independent_coordinates takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_large_coordinates

Found in tests/test_coordinate.py at line 41.

Surface: test.

Signature: `def test_large_coordinates()`

What it does: Function test_large_coordinates takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_whitespace_support

Found in tests/test_coordinate.py at line 53.

Surface: test.

Signature: `def test_whitespace_support()`

What it does: Function test_whitespace_support takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_internal_whitespace_support

Found in tests/test_coordinate.py at line 65.

Surface: test.

Signature: `def test_internal_whitespace_support()`

What it does: Function test_internal_whitespace_support takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_to_urlencoded

Found in tests/test_coordinate.py at line 77.

Surface: test.

Signature: `def test_to_urlencoded()`

What it does: Function test_to_urlencoded takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_library_break

Found in tests/test_coordinate.py at line 81.

Surface: test.

Signature: `def test_library_break()`

What it does: Function test_library_break takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_shelf_break

Found in tests/test_coordinate.py at line 87.

Surface: test.

Signature: `def test_shelf_break()`

What it does: Function test_shelf_break takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_series_break

Found in tests/test_coordinate.py at line 93.

Surface: test.

Signature: `def test_series_break()`

What it does: Function test_series_break takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_collection_break

Found in tests/test_coordinate.py at line 99.

Surface: test.

Signature: `def test_collection_break()`

What it does: Function test_collection_break takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_volume_break

Found in tests/test_coordinate.py at line 105.

Surface: test.

Signature: `def test_volume_break()`

What it does: Function test_volume_break takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_book_break

Found in tests/test_coordinate.py at line 111.

Surface: test.

Signature: `def test_book_break()`

What it does: Function test_book_break takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_chapter_break

Found in tests/test_coordinate.py at line 117.

Surface: test.

Signature: `def test_chapter_break()`

What it does: Function test_chapter_break takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_section_break

Found in tests/test_coordinate.py at line 123.

Surface: test.

Signature: `def test_section_break()`

What it does: Function test_section_break takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_scroll_break

Found in tests/test_coordinate.py at line 129.

Surface: test.

Signature: `def test_scroll_break()`

What it does: Function test_scroll_break takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_breaks_are_unbounded

Found in tests/test_coordinate.py at line 135.

Surface: test.

Signature: `def test_breaks_are_unbounded()`

What it does: Function test_breaks_are_unbounded takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_phokenize_scroll_unbounded

Found in tests/test_coordinate.py at line 147.

Surface: test.

Signature: `def test_phokenize_scroll_unbounded()`

What it does: Function test_phokenize_scroll_unbounded takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_no_input

Found in tests/test_coordinate.py at line 154.

Surface: test.

Signature: `def test_no_input()`

What it does: Function test_no_input takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_very_large_coordinates_round_trip

Found in tests/test_coordinate.py at line 158.

Surface: test.

Signature: `def test_very_large_coordinates_round_trip()`

What it does: Function test_very_large_coordinates_round_trip takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_negative_coordinates

Found in tests/test_coordinate.py at line 174.

Surface: test.

Signature: `def test_negative_coordinates()`

What it does: Function test_negative_coordinates takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_malformed_coordinates

Found in tests/test_coordinate.py at line 178.

Surface: test.

Signature: `def test_malformed_coordinates()`

What it does: Function test_malformed_coordinates takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_null_coordinates

Found in tests/test_coordinate.py at line 182.

Surface: test.

Signature: `def test_null_coordinates()`

What it does: Function test_null_coordinates takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_null_library

Found in tests/test_coordinate.py at line 186.

Surface: test.

Signature: `def test_null_library()`

What it does: Function test_null_library takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_null_shelf

Found in tests/test_coordinate.py at line 190.

Surface: test.

Signature: `def test_null_shelf()`

What it does: Function test_null_shelf takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_null_series

Found in tests/test_coordinate.py at line 194.

Surface: test.

Signature: `def test_null_series()`

What it does: Function test_null_series takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_null_collection

Found in tests/test_coordinate.py at line 198.

Surface: test.

Signature: `def test_null_collection()`

What it does: Function test_null_collection takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_null_volume

Found in tests/test_coordinate.py at line 202.

Surface: test.

Signature: `def test_null_volume()`

What it does: Function test_null_volume takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_null_book

Found in tests/test_coordinate.py at line 206.

Surface: test.

Signature: `def test_null_book()`

What it does: Function test_null_book takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_null_chapter

Found in tests/test_coordinate.py at line 210.

Surface: test.

Signature: `def test_null_chapter()`

What it does: Function test_null_chapter takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_null_section

Found in tests/test_coordinate.py at line 214.

Surface: test.

Signature: `def test_null_section()`

What it does: Function test_null_section takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_null_scroll

Found in tests/test_coordinate.py at line 218.

Surface: test.

Signature: `def test_null_scroll()`

What it does: Function test_null_scroll takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — coord

Found in tests/test_indexed.py at line 26.

Surface: test.

Signature: `def coord(text)`

What it does: Function coord takes text. Its return type is unannotated; the code shows it returning Coordinate.from_string(text).

(Where these words came from: inferred.)

---

## function — assert_equivalent

Found in tests/test_indexed.py at line 30.

Surface: test.

Signature: `def assert_equivalent(buffer)`

What it does: Function assert_equivalent takes buffer. Its return type is unannotated; the code shows it returning indexed.

(Where these words came from: inferred.)

---

## function — test_index_offsets_match_iter_scrolls

Found in tests/test_indexed.py at line 61.

Surface: test.

Signature: `def test_index_offsets_match_iter_scrolls()`

What it does: Function test_index_offsets_match_iter_scrolls takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_indexed_fetch_matches_fetch_everywhere

Found in tests/test_indexed.py at line 75.

Surface: test.

Signature: `def test_indexed_fetch_matches_fetch_everywhere()`

What it does: Function test_indexed_fetch_matches_fetch_everywhere takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_indexed_explode_matches_explode

Found in tests/test_indexed.py at line 81.

Surface: test.

Signature: `def test_indexed_explode_matches_explode()`

What it does: Function test_indexed_explode_matches_explode takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_indexed_generated_document

Found in tests/test_indexed.py at line 87.

Surface: test.

Signature: `def test_indexed_generated_document()`

What it does: Function test_indexed_generated_document takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_indexed_empty_and_single_scroll

Found in tests/test_indexed.py at line 107.

Surface: test.

Signature: `def test_indexed_empty_and_single_scroll()`

What it does: Function test_indexed_empty_and_single_scroll takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_indexed_unbounded_matches_fetch

Found in tests/test_indexed.py at line 120.

Surface: test.

Signature: `def test_indexed_unbounded_matches_fetch()`

What it does: Function test_indexed_unbounded_matches_fetch takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_indexed_fetch_beyond_999_chapter

Found in tests/test_indexed.py at line 151.

Surface: test.

Signature: `def test_indexed_fetch_beyond_999_chapter()`

What it does: Function test_indexed_fetch_beyond_999_chapter takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_index_offsets_built_once

Found in tests/test_indexed.py at line 170.

Surface: test.

Signature: `def test_index_offsets_built_once()`

What it does: Function test_index_offsets_built_once takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — counting

Found in tests/test_indexed.py at line 179.

Surface: test.

Signature: `def counting(self, buf)`

What it does: Function counting takes self, buf. Its return type is unannotated; the code shows it returning original(self, buf).

(Where these words came from: inferred.)

---

## function — test_phext_indexed_convenience_constructor

Found in tests/test_indexed.py at line 195.

Surface: test.

Signature: `def test_phext_indexed_convenience_constructor()`

What it does: Function test_phext_indexed_convenience_constructor takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_dead_reckoning

Found in tests/test_phext.py at line 38.

Surface: test.

Signature: `def test_dead_reckoning()`

What it does: Function test_dead_reckoning takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_line_break

Found in tests/test_phext.py at line 124.

Surface: test.

Signature: `def test_line_break()`

What it does: Function test_line_break takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_more_cowbell

Found in tests/test_phext.py at line 127.

Surface: test.

Signature: `def test_more_cowbell()`

What it does: Function test_more_cowbell takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_coordinate_based_insert

Found in tests/test_phext.py at line 130.

Surface: test.

Signature: `def test_coordinate_based_insert()`

What it does: Function test_coordinate_based_insert takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_coordinate_based_replace

Found in tests/test_phext.py at line 248.

Surface: test.

Signature: `def test_coordinate_based_replace()`

What it does: Function test_coordinate_based_replace takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_coordinate_based_remove

Found in tests/test_phext.py at line 345.

Surface: test.

Signature: `def test_coordinate_based_remove()`

What it does: Function test_coordinate_based_remove takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_range_based_replace

Found in tests/test_phext.py at line 398.

Surface: test.

Signature: `def test_range_based_replace()`

What it does: Function test_range_based_replace takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_next_scroll

Found in tests/test_phext.py at line 415.

Surface: test.

Signature: `def test_next_scroll()`

What it does: Function test_next_scroll takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_last_empty_scroll

Found in tests/test_phext.py at line 425.

Surface: test.

Signature: `def test_last_empty_scroll()`

What it does: Function test_last_empty_scroll takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_merge

Found in tests/test_phext.py at line 438.

Surface: test.

Signature: `def test_merge()`

What it does: Function test_merge takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_subtract

Found in tests/test_phext.py at line 483.

Surface: test.

Signature: `def test_subtract()`

What it does: Function test_subtract takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_normalize

Found in tests/test_phext.py at line 490.

Surface: test.

Signature: `def test_normalize()`

What it does: Function test_normalize takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_expand

Found in tests/test_phext.py at line 496.

Surface: test.

Signature: `def test_expand()`

What it does: Function test_expand takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_contract

Found in tests/test_phext.py at line 529.

Surface: test.

Signature: `def test_contract()`

What it does: Function test_contract takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_fs_read_write

Found in tests/test_phext.py at line 538.

Surface: test.

Signature: `def test_fs_read_write()`

What it does: Function test_fs_read_write takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_replace_create

Found in tests/test_phext.py at line 553.

Surface: test.

Signature: `def test_replace_create()`

What it does: Function test_replace_create takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_summary

Found in tests/test_phext.py at line 560.

Surface: test.

Signature: `def test_summary()`

What it does: Function test_summary takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_navmap

Found in tests/test_phext.py at line 570.

Surface: test.

Signature: `def test_navmap()`

What it does: Function test_navmap takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_textmap

Found in tests/test_phext.py at line 576.

Surface: test.

Signature: `def test_textmap()`

What it does: Function test_textmap takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_larger_coordinates

Found in tests/test_phext.py at line 582.

Surface: test.

Signature: `def test_larger_coordinates()`

What it does: Function test_larger_coordinates takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_phext_index

Found in tests/test_phext.py at line 590.

Surface: test.

Signature: `def test_phext_index()`

What it does: Function test_phext_index takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_scroll_manifest

Found in tests/test_phext.py at line 642.

Surface: test.

Signature: `def test_scroll_manifest()`

What it does: Function test_scroll_manifest takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_phext_soundex_v1

Found in tests/test_phext.py at line 694.

Surface: test.

Signature: `def test_phext_soundex_v1()`

What it does: Function test_phext_soundex_v1 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_insert_performance_2k_scrolls

Found in tests/test_phext.py at line 700.

Surface: test.

Signature: `def test_insert_performance_2k_scrolls()`

What it does: Function test_insert_performance_2k_scrolls takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_hash_support

Found in tests/test_phext.py at line 829.

Surface: test.

Signature: `def test_hash_support()`

What it does: Function test_hash_support takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_phext_breaks

Found in tests/test_phext.py at line 893.

Surface: test.

Signature: `def test_phext_breaks()`

What it does: Function test_phext_breaks takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_normalize

Found in tests/test_phext.py at line 916.

Surface: test.

Signature: `def test_normalize()`

What it does: Function test_normalize takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — random_document

Found in tests/test_phokenize.py at line 45.

Surface: test.

Signature: `def random_document(rng: random.Random) -> str`

What it does: Function random_document takes rng: random.Random. It gives back str.

(Where these words came from: inferred.)

---

## function — random_coordinate

Found in tests/test_phokenize.py at line 61.

Surface: test.

Signature: `def random_coordinate(rng: random.Random) -> Coordinate`

What it does: Function random_coordinate takes rng: random.Random. It gives back Coordinate.

(Where these words came from: inferred.)

---

## function — outcome

Found in tests/test_phokenize.py at line 64.

Surface: test.

Signature: `def outcome(fn, *args)`

What it does: Function outcome takes fn, *args. Its return type is unannotated; the code shows it returning fn(*args); type(e).

(Where these words came from: inferred.)

---

## function — corpus

Found in tests/test_phokenize.py at line 71.

Surface: test.

Signature: `def corpus()`

What it does: Function corpus takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning docs.

(Where these words came from: inferred.)

---

## function — as_pairs

Found in tests/test_phokenize.py at line 80.

Surface: test.

Signature: `def as_pairs(stack)`

What it does: Function as_pairs takes stack. Its return type is unannotated; the code shows it returning [(ps.coord.as_tuple(), ps.text) for ps in stack].

(Where these words came from: inferred.)

---

## function — test_phokenize_matches_legacy

Found in tests/test_phokenize.py at line 83.

Surface: test.

Signature: `def test_phokenize_matches_legacy()`

What it does: Function test_phokenize_matches_legacy takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_phokenize_coordinates_are_distinct_objects

Found in tests/test_phokenize.py at line 89.

Surface: test.

Signature: `def test_phokenize_coordinates_are_distinct_objects()`

What it does: Function test_phokenize_coordinates_are_distinct_objects takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_serialization_matches_legacy

Found in tests/test_phokenize.py at line 96.

Surface: test.

Signature: `def test_serialization_matches_legacy()`

What it does: Function test_serialization_matches_legacy takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_explode_implode_round_trip

Found in tests/test_phokenize.py at line 107.

Surface: test.

Signature: `def test_explode_implode_round_trip()`

What it does: Function test_explode_implode_round_trip takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_maps_and_index_match_legacy

Found in tests/test_phokenize.py at line 123.

Surface: test.

Signature: `def test_maps_and_index_match_legacy()`

What it does: Function test_maps_and_index_match_legacy takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_lookups_match_legacy

Found in tests/test_phokenize.py at line 135.

Surface: test.

Signature: `def test_lookups_match_legacy()`

What it does: Function test_lookups_match_legacy takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_edits_match_legacy

Found in tests/test_phokenize.py at line 153.

Surface: test.

Signature: `def test_edits_match_legacy()`

What it does: Function test_edits_match_legacy takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_append_scroll_matches_legacy

Found in tests/test_phokenize.py at line 170.

Surface: test.

Signature: `def test_append_scroll_matches_legacy()`

What it does: Function test_append_scroll_matches_legacy takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_append_scroll_out_of_order

Found in tests/test_phokenize.py at line 187.

Surface: test.

Signature: `def test_append_scroll_out_of_order()`

What it does: Function test_append_scroll_out_of_order takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — large_document

Found in tests/test_phokenize.py at line 195.

Surface: test.

Signature: `def large_document(scrolls: int) -> str`

What it does: Function large_document takes scrolls: int. It gives back str.

(Where these words came from: inferred.)

---

## function — test_explode_performance_50k_scrolls

Found in tests/test_phokenize.py at line 207.

Surface: test.

Signature: `def test_explode_performance_50k_scrolls()`

What it does: Function test_explode_performance_50k_scrolls takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_scroll_interface

Found in tests/test_positionedScroll.py at line 12.

Surface: test.

Signature: `def test_scroll_interface()`

What it does: Function test_scroll_interface takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_positioned_scrolls

Found in tests/test_positionedScroll.py at line 19.

Surface: test.

Signature: `def test_positioned_scrolls()`

What it does: Function test_positioned_scrolls takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_phokenize

Found in tests/test_positionedScroll.py at line 39.

Surface: test.

Signature: `def test_phokenize()`

What it does: Function test_phokenize takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_range_exists

Found in tests/test_range.py at line 5.

Surface: test.

Signature: `def test_range_exists()`

What it does: Function test_range_exists takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_range_fixture_case

Found in tests/test_range_fixtures.py at line 55.

Surface: test.

Signature: `def test_range_fixture_case(case_name)`

What it does: Function test_range_fixture_case takes case_name. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_seal_round_trip_real_phext

Found in tests/test_seal.py at line 23.

Surface: test.

Signature: `def test_seal_round_trip_real_phext()`

What it does: Function test_seal_round_trip_real_phext takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_seal_round_trip_empty_and_small

Found in tests/test_seal.py at line 33.

Surface: test.

Signature: `def test_seal_round_trip_empty_and_small()`

What it does: Function test_seal_round_trip_empty_and_small takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_seal_carrying_binary_scrolls

Found in tests/test_seal.py at line 41.

Surface: test.

Signature: `def test_seal_carrying_binary_scrolls()`

What it does: Function test_seal_carrying_binary_scrolls takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_seal_is_deterministic

Found in tests/test_seal.py at line 50.

Surface: test.

Signature: `def test_seal_is_deterministic()`

What it does: Function test_seal_is_deterministic takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_unseal_rejects_non_xz_data

Found in tests/test_seal.py at line 57.

Surface: test.

Signature: `def test_unseal_rejects_non_xz_data()`

What it does: Function test_unseal_rejects_non_xz_data takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/libphext-py/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

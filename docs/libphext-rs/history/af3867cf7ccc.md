# libphext-rs — reference as of commit af3867cf7ccc

Commit af3867cf7ccc798ca4b785a37427eb49db5ee906. 67 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/libphext-rs/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

## Dependencies

### Cargo.toml

- xxhash-rust — 0.8.5
- base64 — 0.22
- xz2 — 0.1

## Authors

The author of record is **Will Bickford** — 67 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 45 commits
- Emise — 15 commits
- Theia — 6 commits
- Orin — 1 commit

## Complexity

- Markdown: 19 files, 1872 lines total, 33 blank, 0 comment, 1839 code
- Rust: 16 files, 5245 lines total, 350 blank, 486 comment, 4409 code
- Shell: 2 files, 319 lines total, 20 blank, 73 comment, 226 code
- TOML: 4 files, 55 lines total, 1 blank, 0 comment, 54 code

Declared RFCs at this point: none declared.

## Public methods and commands

- enum `Command` (counter/kernel/src/lib.rs) — A named choice named Command.
- enum `Error` (counter/kernel/src/lib.rs) — A named choice named Error.
- struct `State` (counter/kernel/src/lib.rs) — The whole state: a counter that must never pass its limit.
- function `transition` (counter/kernel/src/lib.rs) — One step: the next state, or why the command is refused.
- entry point `main` (examples/explode_bench.rs) — The entry point: the program starts here.
- const `BINARY_SCROLL_HEADER` (src/binary.rs) — The first (and only header) line of a binary scroll, without its line break.
- const `FIELD_PREFIX` (src/binary.rs) — The prefix of a binary *field*: arbitrary bytes encoded for one line inside a text scroll (a `name=value` grammar, a WAL entry, a header block).
- function `decode_field` (src/binary.rs) — ---------------------------------------------------------------------------------------------------------- @fn decode_field  Decodes a field produced by `encode_field`.
- function `decode_scroll` (src/binary.rs) — ---------------------------------------------------------------------------------------------------------- @fn decode_scroll  decodes a binary scroll back to its raw bytes.
- function `encode_field` (src/binary.rs) — ---------------------------------------------------------------------------------------------------------- @fn encode_field  Encodes raw bytes as a single line of text: the field prefix plus RFC 4648 base64.
- function `encode_scroll` (src/binary.rs) — ---------------------------------------------------------------------------------------------------------- @fn encode_sc
- function `fetch_binary` (src/binary.rs) — ---------------------------------------------------------------------------------------------------------- @fn fetch_binary  retrieves the bytes held at `target`.
- function `ingest` (src/binary.rs) — ---------------------------------------------------------------------------------------------------------- @fn ingest  ingests an externally supplied phext byte stream.
- function `insert_binary` (src/binary.rs) — ---------------------------------------------------------------------------------------------------------- @fn insert_binary  stores `payload` at `location` as a binary scroll, unconditionally: the caller has declared this scroll binary, even if the bytes happen to be valid UTF-8.
- function `insert_bytes` (src/binary.rs) — ---------------------------------------------------------------------------------------------------------- @fn insert_bytes  byte-oriented insert: stores `payload` at `location` under the ingestion rule (UTF-8 as text, non-UTF-8 as a base64-verbatim binary scroll).
- function `is_binary_scroll` (src/binary.rs) — ---------------------------------------------------------------------------------------------------------- @fn is_binary_scroll  true when `scroll` carries the binary scroll header.
- function `phokenize_bytes` (src/binary.rs) — ---------------------------------------------------------------------------------------------------------- @fn phokenize_bytes  the byte-level twin of `phext::phokenize`: walks a raw byte stream and returns each scroll's coordinate and raw bytes, with identical segmentation.
- function `replace_binary` (src/binary.rs) — ---------------------------------------------------------------------------------------------------------- @fn replace_binary  replaces the scroll at `location` with `payload`, stored as a binary scroll (see `insert_binary`).
- function `replace_bytes` (src/binary.rs) — ---------------------------------------------------------------------------------------------------------- @fn replace_bytes  byte-oriented replace: stores `payload` at `location` under the ingestion rule (UTF-8 as text, non-UTF-8 as a base64-verbatim binary scroll).
- const `ADDRESS_MACRO_ALT` (src/phext.rs) — A fixed value named ADDRESS_MACRO_ALT, set to b';'; // also allow ';' for url encoding.
- const `ADDRESS_MACRO_BREAK` (src/phext.rs) — A fixed value named ADDRESS_MACRO_BREAK, set to b'/'; // delimiter for macro-coordinates.
- const `ADDRESS_MICRO_BREAK` (src/phext.rs) — A fixed value named ADDRESS_MICRO_BREAK, set to b'.'; // delimiter for micro-coordinates.
- const `BOOK_BREAK` (src/phext.rs) — A fixed value named BOOK_BREAK, set to '\x1A'; // 6D Break - replaces Substitute.
- const `BOOK_BREAK_U8` (src/phext.rs) — A fixed value named BOOK_BREAK_U8, set to 0x1A; // byte form of BOOK_BREAK.
- const `CHAIN_BREAK` (src/phext.rs) — A fixed value named CHAIN_BREAK, set to '\x02'; // stream level - separates whole phexts within a ch.
- const `CHAIN_BREAK_U8` (src/phext.rs) — A fixed value named CHAIN_BREAK_U8, set to 0x02; // byte form of CHAIN_BREAK.
- const `CHAIN_CLOSE` (src/phext.rs) — A fixed value named CHAIN_CLOSE, set to '\x03'; // stream level - terminates a chain; replaces end o.
- const `CHAIN_CLOSE_U8` (src/phext.rs) — A fixed value named CHAIN_CLOSE_U8, set to 0x03; // byte form of CHAIN_CLOSE.
- const `CHAPTER_BREAK` (src/phext.rs) — A fixed value named CHAPTER_BREAK, set to '\x19'; // 5D Break - replaces End of Tape.
- const `CHAPTER_BREAK_U8` (src/phext.rs) — A fixed value named CHAPTER_BREAK_U8, set to 0x19; // byte form of CHAPTER_BREAK.
- const `COLLECTION_BREAK` (src/phext.rs) — A fixed value named COLLECTION_BREAK, set to '\x1D'; // 8D Break - replaces group separator.
- const `COLLECTION_BREAK_U8` (src/phext.rs) — A fixed value named COLLECTION_BREAK_U8, set to 0x1D; // byte form of COLLECTION_BREAK.
- const `COORDINATE_MINIMUM` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- phext constan
- struct `Chain` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @struct Chain  A chain: an ordered series of complete phexts within one stream, joined by CHAIN_BREAK (0x02) and optionally terminated by CHAIN_CLOSE (0x03).
- function `Chain::fetch` (src/phext.rs) — Fetch a scroll by multiverse address: "$N:coord", or a bare coordinate (universe 1).
- function `Chain::new` (src/phext.rs) — Function Chain::new takes no inputs beyond the value it is called on, and gives back Chain.
- function `Chain::resolve_point` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn resolve_point  resolves a point address against this chain: (universe, coordinate, character offset within the scroll).
- struct `Coordinate` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @struct Coordinate  provides access to a default-initialized coordinate at 1.1.1/1.1.1/1.1.1  phext coordinates are formed along a 9-dimensional hierarchy with three main arms of the form z3.z2.z1/y3.y2.y1.x3.x2.x1 where:  Z - this arm contains the library (z3), shelf (z2), and series (z1) dimensions Y - this arm contains the collection (y3), volume (y2), and book (y1) dimensions X - this arm contains the chapter (x3), section (x2), and scroll (x1) dimensions  Dense graphs: phext is sparse by design.
- function `Coordinate::new` (src/phext.rs) — Function Coordinate::new takes no inputs beyond the value it is called on, and gives back Coordinate.
- function `Coordinate::validate_coordinate` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn validate_
- const `LIBRARY_BREAK` (src/phext.rs) — A fixed value named LIBRARY_BREAK, set to '\x01'; // 11th dimension - replaces start of header.
- const `LIBRARY_BREAK_U8` (src/phext.rs) — A fixed value named LIBRARY_BREAK_U8, set to 0x01; // byte form of LIBRARY_BREAK.
- const `LINE_BREAK` (src/phext.rs) — A fixed value named LINE_BREAK, set to '\x0A'; // same as plain text \o/.
- const `LINE_BREAK_U8` (src/phext.rs) — A fixed value named LINE_BREAK_U8, set to 0x0A; // byte form of LINE_BREAK.
- const `MORE_COWBELL` (src/phext.rs) — A fixed value named MORE_COWBELL, set to '\x07'; // i've got a fever, and the only prescription...is .
- const `MORE_COWBELL_U8` (src/phext.rs) — A fixed value named MORE_COWBELL_U8, set to 0x07; // byte form of MORE_COWBELL.
- struct `PhextParseError` (src/phext.rs) — A data shape named PhextParseError.
- struct `PointAddress` (src/phext.rs) — @struct PointAddress  One parsed point address: universe (1-based; 1 when the "$N:" prefix is absent), the 9D coordinate
- struct `PositionedScroll` (src/phext.rs) — A data shape named PositionedScroll.
- function `PositionedScroll::new` (src/phext.rs) — Function PositionedScroll::new takes no inputs beyond the value it is called on, and gives back PositionedScroll.
- struct `Range` (src/phext.rs) — A data shape named Range.
- function `Range::new` (src/phext.rs) — Function Range::new takes no inputs beyond the value it is called on, and gives back Range.
- struct `RangeAddress` (src/phext.rs) — @struct RangeAddress  A parsed range: two point endpoints, normalized to ascending order at parse time.
- const `SCROLL_BREAK` (src/phext.rs) — A fixed value named SCROLL_BREAK, set to '\x17'; // 3D Break - replaces End Transmission Block.
- const `SCROLL_BREAK_U8` (src/phext.rs) — A fixed value named SCROLL_BREAK_U8, set to 0x17; // byte form of SCROLL_BREAK.
- const `SECTION_BREAK` (src/phext.rs) — A fixed value named SECTION_BREAK, set to '\x18'; // 4D Break - replaces Cancel Block.
- const `SECTION_BREAK_U8` (src/phext.rs) — A fixed value named SECTION_BREAK_U8, set to 0x18; // byte form of SECTION_BREAK.
- const `SERIES_BREAK` (src/phext.rs) — A fixed value named SERIES_BREAK, set to '\x1E'; // 9D Break - replaces record separator.
- const `SERIES_BREAK_U8` (src/phext.rs) — A fixed value named SERIES_BREAK_U8, set to 0x1E; // byte form of SERIES_BREAK.
- const `SHELF_BREAK` (src/phext.rs) — A fixed value named SHELF_BREAK, set to '\x1F'; // 10D Break - replaces unit separator.
- const `SHELF_BREAK_U8` (src/phext.rs) — A fixed value named SHELF_BREAK_U8, set to 0x1F; // byte form of SHELF_BREAK.
- struct `ScrollPosition` (src/phext.rs) — @struct ScrollPosition  A caret boundary inside one scroll: 1-based line and column (see the section note above).
- const `VOLUME_BREAK` (src/phext.rs) — A fixed value named VOLUME_BREAK, set to '\x1C'; // 7D Break - replaces file separator.
- const `VOLUME_BREAK_U8` (src/phext.rs) — A fixed value named VOLUME_BREAK_U8, set to 0x1C; // byte form of VOLUME_BREAK.
- struct `XCoordinate` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @struct XCoor
- function `XCoordinate::new` (src/phext.rs) — Function XCoordinate::new takes no inputs beyond the value it is called on, and gives back XCoordinate.
- struct `YCoordinate` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @struct YCoor
- function `YCoordinate::new` (src/phext.rs) — Function YCoordinate::new takes no inputs beyond the value it is called on, and gives back YCoordinate.
- struct `ZCoordinate` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @struct ZCoor
- function `ZCoordinate::new` (src/phext.rs) — Function ZCoordinate::new takes no inputs beyond the value it is called on, and gives back ZCoordinate.
- function `append` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn append  Appends `scroll` at `location`, which must sort strictly after the document's last non-empty scroll.
- function `append_batch` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn append_batch  Appends several scrolls in one call.
- function `append_checked` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn append_checked  `append` with an integrity trailer: the stored scroll is `checked_scroll(scroll)`, verifiable later with `fetch_checked`.
- function `book_break` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn book_break --
- function `chapter_break` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn chapter_break
- function `check_for_cowbell` (src/phext.rs) — Function check_for_cowbell takes input: &str and gives back bool.
- function `checked_scroll` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn checked_scroll  Returns `scroll` with an integrity trailer appended: one final line `phext-check:xxh3-128:<checksum of the payload>`.
- function `checksum` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn checksum 
- function `collection_break` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn collection_br
- function `commit_scroll` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn commit_scroll  Builds the commit marker for a batch: a small scroll binding the batch's entry count and a checksum over each entry's coordinate and content, in order.
- function `contract` (src/phext.rs) — ----------------------------------------------------------------------------------------------------------
- function `create_summary` (src/phext.rs) — ----------------------------------------------------------------------------------------------------------
- function `default_coordinate` (src/phext.rs) — ----------------------------------------------------------------------------------------------------------
- function `expand` (src/phext.rs) — ----------------------------------------------------------------------------------------------------------
- function `explode` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn explode  
- function `fetch` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn fetch  retrieves the scroll located at the given coordinates.
- function `fetch_checked` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn fetch_checked  Fetches the scroll at `target` and returns its payload, but only when the scroll is checked and verifies.
- function `fetch_point` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn fetch_point  fetches a point address string.
- function `fetch_range` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn fetch_range  fetches a range address string: the spanned text with phext structure bytes preserved — the covered segments re-assembled at their coordinates into a phext (chain) fragment (scroll breaks between covered scrolls inside a universe, CHAIN_BREAK between universes, never a trailing CHAIN_CLOSE: a fragment is an excerpt, not a terminated chain), so the result round-trips as a phext fragment.
- function `get_subspace_coordinates` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn get_subspace_coordinates  finds the start and end offsets for the given coordinate  Returns `(start, end, best)`: - when the document reaches `target`, `start..end` is that scroll's text (empty if it has none) and `best` is `target`; - otherwise `start == end` is where the scroll would go: just before the break that steps past `target`, or the end of the document.
- function `implode` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn implode  Serializes a hierarchical hash of scrolls into a monolithic phext document.
- function `index` (src/phext.rs) — ----------------------------------------------------------------------------------------------------------
- function `insert` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn insert  i
- function `is_phext_break` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn is_phext_
- function `library_break` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn library_break
- function `manifest` (src/phext.rs) — ----------------------------------------------------------------------------------------------------------
- function `merge` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn merge  co
- function `navmap` (src/phext.rs) — ----------------------------------------------------------------------------------------------------------
- function `next_scroll` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn next_scro
- function `normalize` (src/phext.rs) — ----------------------------------------------------------------------------------------------------------
- function `offset` (src/phext.rs) — ----------------------------------------------------------------------------------------------------------  The offset of the scroll at `coord`, or of the last scroll before it.
- function `parse_chains` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn parse_chains  splits a stream into its chains.
- function `parse_point_address` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn parse_point_address  parses a v0.6.1 point address: "[$N:]<coordinate>[@<line>,<column>[+<offsetChars>]]".
- function `parse_range_address` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn parse_range_address  parses a v0.6.1 range address: exactly two point addresses joined by one hyphen.
- function `parse_universe_address` (src/phext.rs) — Function parse_universe_address takes address: &str and gives back Option<(usize, Coordinate)>.
- function `phokenize` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn phokenize  Transforms a packed phext buffer into a phext token (phoken) stream  One pass over the bytes.
- function `range_replace` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- errata: libph
- function `remove` (src/phext.rs) — ----------------------------------------------------------------------------------------------------------
- function `replace` (src/phext.rs) — ----------------------------------------------------------------------------------------------------------
- function `resolve_range` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn resolve_range  resolves a range against this chain into per-scroll segments: (universe, coordinate, segment text) in ascending order.
- function `scroll_break` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn scroll_break 
- function `section_break` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn section_break
- function `serialize` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn serialize  serializes this chain alone: its members joined by CHAIN_BREAK, with a trailing CHAIN_CLOSE when the chain is closed.
- function `serialize_chains` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn serialize_chains  serializes a stream of chains back into stream bytes: the inverse of `parse_chains`.
- function `series_break` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn series_break 
- function `shelf_break` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn shelf_break -
- function `soundex_v1` (src/phext.rs) — ----------------------------------------------------------------------------------------------------------
- function `subtract` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn subtract  Subtracts the scrolls from `left` where there is also content at those coordinates in `right`  Note: This makes it easy to generate a mask for a given phext archive.
- function `textmap` (src/phext.rs) — ----------------------------------------------------------------------------------------------------------
- function `to_coordinate` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn to_coordi
- function `to_string` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn to_string
- function `to_urlencoded` (src/phext.rs) — Function to_urlencoded takes no inputs beyond the value it is called on, and gives back String.
- function `uncheck_scroll` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn uncheck_scroll  Returns the payload of a checked scroll when its trailer verifies; None for unchecked, malformed, or corrupted scrolls.
- function `verify_checked_scroll` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn verify_checked_scroll  true when `scroll` ends with a well-formed integrity trailer whose checksum matches the payload before it.
- function `verify_commit` (src/phext.rs) — ---------------------------------------------------------------------------------------------------------- @fn verify_commit  true when the scroll at `commit` is a commit marker whose count matches `entries` and whose checksum matches the entries' current contents, fetched in order.
- function `volume_break` (src/phext.rs) — ------------------------------------------------------------------------------------------------------ @fn volume_break 
- const `SEAL_PRESET` (src/seal.rs) — The xz preset this implementation seals with.
- function `seal` (src/seal.rs) — ---------------------------------------------------------------------------------------------------------- @fn seal  seals a phext: xz-compresses its full UTF-8 bytes.
- function `seal_bytes` (src/seal.rs) — ---------------------------------------------------------------------------------------------------------- @fn seal_bytes  seals raw phext bytes (see `seal`).
- function `unseal` (src/seal.rs) — ---------------------------------------------------------------------------------------------------------- @fn unseal  unseals a sealed phext back to its exact original bytes.
- function `unseal_string` (src/seal.rs) — ---------------------------------------------------------------------------------------------------------- @fn unseal_string  unseals directly to text.

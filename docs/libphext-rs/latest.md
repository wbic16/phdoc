# libphext-rs — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 147). Scanned at commit 84c10399cccc8e5f34b2aadcade96907b5b1a655. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

Description stated in the manifest: The reference implementation of phext, in Rust.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 3.

- xxhash-rust [dependencies] — 0.8.5
- base64 [dependencies] — 0.22
- xz2 [dependencies] — 0.1

### Rust (cargo) — counter/Cargo.toml

Direct dependencies: 0.


### Rust (cargo) — counter/kernel/Cargo.toml

Direct dependencies: 0.



---

## Authors

The git log holds 83 commits.

The author of record is **Will Bickford** — 83 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 53 commits
- Emise — 20 commits
- Theia — 9 commits
- Orin — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 33 files, 3404 lines total, 757 blank, 18 comment, 2629 code
- Rust: 22 files, 6996 lines total, 620 blank, 1604 comment, 4772 code
- Shell: 5 files, 319 lines total, 26 blank, 121 comment, 172 code
- TOML: 4 files, 55 lines total, 7 blank, 10 comment, 38 code

All languages: 64 files, 10774 lines total, 7611 lines of code. Functions read: 335.

Largest files:

- src/phext.rs — 2465 lines
- src/test_lib.rs — 1384 lines
- src/test_epoch.rs — 425 lines
- src/test_v062.rs — 325 lines
- .agents/skills/h5i/references/impactful-bug-bounty.md — 295 lines

Largest functions, by span in lines:

- test_coordinate_based_insert in src/test_lib.rs — about 133 lines
- test_coordinate_based_replace in src/test_lib.rs — about 97 lines
- test_dead_reckoning in src/test_lib.rs — about 87 lines
- test_insert_performance_medium_scrolls in src/test_lib.rs — about 84 lines
- run_case in tests/chain_fixtures.rs — about 81 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 167 test functions counted in the source, across 14 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 3. FIXME marks: 0.
- Public items with a doc comment: 92 of 158 (58%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/libphext-rs
- Branch: exo
- Head commit: 84c10399cccc8e5f34b2aadcade96907b5b1a655
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- struct `State` (counter/kernel/src/lib.rs:8) — The whole state: a counter that must never pass its limit.
- enum `Command` (counter/kernel/src/lib.rs:14) — A named choice named Command.
- enum `Error` (counter/kernel/src/lib.rs:22) — A named choice named Error.
- function `transition` (counter/kernel/src/lib.rs:27) — One step: the next state, or why the command is refused.
- entry point `main` (examples/explode_bench.rs:11) [example] — The entry point: the program starts here.
- const `BINARY_SCROLL_HEADER` (src/binary.rs:42) — The first (and only header) line of a binary scroll, without its line break.
- function `is_binary_scroll` (src/binary.rs:53) — true when `scroll` carries the binary scroll header.
- function `encode_scroll` (src/binary.rs:62) — encodes raw bytes as a binary scroll (header line + base64 body)
- function `decode_scroll` (src/binary.rs:75) — decodes a binary scroll back to its raw bytes.
- function `insert_binary` (src/binary.rs:87) — stores `payload` at `location` as a binary scroll, unconditionally: the caller has declared this scroll binary, even…
- function `replace_binary` (src/binary.rs:97) — replaces the scroll at `location` with `payload`, stored as a binary scroll (see `insert_binary`).
- function `fetch_binary` (src/binary.rs:110) — retrieves the bytes held at `target`.
- function `insert_bytes` (src/binary.rs:134) — byte-oriented insert: stores `payload` at `location` under the ingestion rule (UTF-8 as text, non-UTF-8 as a…
- function `replace_bytes` (src/binary.rs:144) — byte-oriented replace: stores `payload` at `location` under the ingestion rule (UTF-8 as text, non-UTF-8 as a…
- function `phokenize_bytes` (src/binary.rs:180) — the byte-level twin of `phext::phokenize`: walks a raw byte stream and returns each scroll's coordinate and raw bytes, with identical segmentation.
- function `ingest` (src/binary.rs:227) — ingests an externally supplied phext byte stream.
- const `FIELD_PREFIX` (src/binary.rs:243) — The prefix of a binary *field*: arbitrary bytes encoded for one line inside a text scroll (a `name=value` grammar, a WAL entry, a header block).
- function `encode_field` (src/binary.rs:255) — Encodes raw bytes as a single line of text: the field prefix plus RFC 4648 base64.
- function `decode_field` (src/binary.rs:269) — Decodes a field produced by `encode_field`.
- const `EPOCH_ANCHOR_UNIX` (src/epoch.rs:58) — The unix timestamp at which EE 1 begins: 2026-11-22T06:00:00Z.
- const `EPOCH_LENGTH_SECONDS` (src/epoch.rs:61) — The length of one Exocortical Epoch, in seconds (7 minutes).
- const `AGE_BITS` (src/epoch.rs:65) — The width of a Sentron Age field, in bits: one epoch is divided into 2^42 Sentron Ages.
- const `AGES_PER_EPOCH` (src/epoch.rs:71) — The number of Sentron Ages in one Exocortical Epoch: 2^42 = 4,398,046,511,104.
- const `AGE_MAX` (src/epoch.rs:74) — The largest Sentron Age: the final nominal step of an epoch.
- const `COARSE_AGE_BITS` (src/epoch.rs:80) — The width of a coarse Sentron Age field, in bits: the fine Sentron Age scaled into a 16-bit address space (coarse = fine >> (AGE_BITS − COARSE_AGE_BITS)).
- const `COARSE_AGES_PER_EPOCH` (src/epoch.rs:84) — The number of coarse Sentron Ages in one Exocortical Epoch: 2^16 = 65,536.
- const `COARSE_AGE_MAX` (src/epoch.rs:87) — The largest coarse Sentron Age: the final coarse step of an epoch.
- function `epoch_at` (src/epoch.rs:97) — the Exocortical Epoch containing the given unix timestamp (UTC seconds).
- function `epoch_start` (src/epoch.rs:111) — the unix timestamp at which the given epoch begins.
- function `epoch_end` (src/epoch.rs:126) — the unix timestamp at which the given epoch ends — the next epoch's start.
- function `epoch_range` (src/epoch.rs:137) — the (start, end) unix timestamps bounding the given epoch.
- function `seconds_into_epoch` (src/epoch.rs:148) — how far the given unix timestamp sits inside its epoch, in seconds (0 at an epoch's first instant, 419 at its last).
- function `seconds_remaining` (src/epoch.rs:163) — seconds until the current epoch ends.
- function `age_at` (src/epoch.rs:186) — the Sentron Age of the given unix timestamp: its nominal position within its epoch, `floor(seconds_into_epoch × 2^42 / 420)`, in the range 0..=AGE_MAX.
- function `age_at_nanos` (src/epoch.rs:204) — the Sentron Age of a timestamp given with sub-second precision: `unix_seconds` plus `nanoseconds` (0..1_000_000_000) within that second.
- function `age_start` (src/epoch.rs:231) — the unix timestamp of the whole second in which the given Sentron Age of the given epoch begins: `epoch_start(epoch) + floor(age × 420 / 2^42)`.
- function `coarse_age_at` (src/epoch.rs:257) — the coarse Sentron Age of the given unix timestamp: the fine Sentron Age scaled into a 16-bit address space…
- const `COORDINATE_MINIMUM` (src/phext.rs:86) — phext constants
- const `LIBRARY_BREAK` (src/phext.rs:89) — A fixed value named LIBRARY_BREAK, set to '\x01'.
- const `CHAIN_BREAK` (src/phext.rs:90) — A fixed value named CHAIN_BREAK, set to '\x02'.
- const `CHAIN_CLOSE` (src/phext.rs:91) — A fixed value named CHAIN_CLOSE, set to '\x03'.
- const `MORE_COWBELL` (src/phext.rs:92) — A fixed value named MORE_COWBELL, set to '\x07'.
- const `LINE_BREAK` (src/phext.rs:93) — A fixed value named LINE_BREAK, set to '\x0A'.
- const `SCROLL_BREAK` (src/phext.rs:94) — A fixed value named SCROLL_BREAK, set to '\x17'.
- const `SECTION_BREAK` (src/phext.rs:95) — A fixed value named SECTION_BREAK, set to '\x18'.
- const `CHAPTER_BREAK` (src/phext.rs:96) — A fixed value named CHAPTER_BREAK, set to '\x19'.
- const `BOOK_BREAK` (src/phext.rs:97) — A fixed value named BOOK_BREAK, set to '\x1A'.
- const `VOLUME_BREAK` (src/phext.rs:98) — A fixed value named VOLUME_BREAK, set to '\x1C'.
- const `COLLECTION_BREAK` (src/phext.rs:99) — A fixed value named COLLECTION_BREAK, set to '\x1D'.
- const `SERIES_BREAK` (src/phext.rs:100) — A fixed value named SERIES_BREAK, set to '\x1E'.
- const `SHELF_BREAK` (src/phext.rs:101) — A fixed value named SHELF_BREAK, set to '\x1F'.
- const `LIBRARY_BREAK_U8` (src/phext.rs:106) — A fixed value named LIBRARY_BREAK_U8, set to 0x01.
- const `CHAIN_BREAK_U8` (src/phext.rs:107) — A fixed value named CHAIN_BREAK_U8, set to 0x02.
- const `CHAIN_CLOSE_U8` (src/phext.rs:108) — A fixed value named CHAIN_CLOSE_U8, set to 0x03.
- const `MORE_COWBELL_U8` (src/phext.rs:109) — A fixed value named MORE_COWBELL_U8, set to 0x07.
- const `LINE_BREAK_U8` (src/phext.rs:110) — A fixed value named LINE_BREAK_U8, set to 0x0A.
- const `SCROLL_BREAK_U8` (src/phext.rs:111) — A fixed value named SCROLL_BREAK_U8, set to 0x17.
- const `SECTION_BREAK_U8` (src/phext.rs:112) — A fixed value named SECTION_BREAK_U8, set to 0x18.
- const `CHAPTER_BREAK_U8` (src/phext.rs:113) — A fixed value named CHAPTER_BREAK_U8, set to 0x19.
- const `BOOK_BREAK_U8` (src/phext.rs:114) — A fixed value named BOOK_BREAK_U8, set to 0x1A.
- const `VOLUME_BREAK_U8` (src/phext.rs:115) — A fixed value named VOLUME_BREAK_U8, set to 0x1C.
- const `COLLECTION_BREAK_U8` (src/phext.rs:116) — A fixed value named COLLECTION_BREAK_U8, set to 0x1D.
- const `SERIES_BREAK_U8` (src/phext.rs:117) — A fixed value named SERIES_BREAK_U8, set to 0x1E.
- const `SHELF_BREAK_U8` (src/phext.rs:118) — A fixed value named SHELF_BREAK_U8, set to 0x1F.
- const `ADDRESS_MICRO_BREAK` (src/phext.rs:120) — A fixed value named ADDRESS_MICRO_BREAK, set to b'.'.
- const `ADDRESS_MACRO_BREAK` (src/phext.rs:121) — A fixed value named ADDRESS_MACRO_BREAK, set to b'/'.
- const `ADDRESS_MACRO_ALT` (src/phext.rs:122) — A fixed value named ADDRESS_MACRO_ALT, set to b';'.
- struct `ZCoordinate` (src/phext.rs:169) — The large-scale Z arm of a phext coordinate (see `Coordinate` below)
- function `ZCoordinate::new` (src/phext.rs:175) — Function ZCoordinate::new takes no inputs beyond the value it is called on.
- struct `YCoordinate` (src/phext.rs:196) — The large-scale Y arm of a phext coordinate (see `Coordinate` below)
- function `YCoordinate::new` (src/phext.rs:202) — Function YCoordinate::new takes no inputs beyond the value it is called on.
- struct `XCoordinate` (src/phext.rs:223) — The large-scale X arm of a phext coordinate (see `Coordinate` below)
- function `XCoordinate::new` (src/phext.rs:229) — Function XCoordinate::new takes no inputs beyond the value it is called on.
- struct `Coordinate` (src/phext.rs:264) — provides access to a default-initialized coordinate at 1.1.1/1.1.1/1.1.1 phext coordinates are formed along a…
- function `Coordinate::new` (src/phext.rs:270) — Function Coordinate::new takes no inputs beyond the value it is called on.
- struct `PositionedScroll` (src/phext.rs:281) — A data shape named PositionedScroll.
- function `PositionedScroll::new` (src/phext.rs:286) — Function PositionedScroll::new takes no inputs beyond the value it is called on.
- struct `Range` (src/phext.rs:302) — A data shape named Range.
- function `Range::new` (src/phext.rs:307) — Function Range::new takes no inputs beyond the value it is called on.
- struct `PhextParseError` (src/phext.rs:318) — awaiting a summary
- function `check_for_cowbell` (src/phext.rs:412) — Function check_for_cowbell takes input: &str.
- function `get_subspace_coordinates` (src/phext.rs:441) — finds the start and end offsets for the given coordinate Returns `(start, end, best)`: when the document reaches…
- function `remove` (src/phext.rs:495) — Function remove takes input: &str, location: Coordinate.
- function `create_summary` (src/phext.rs:501) — Function create_summary takes input: &str.
- function `navmap` (src/phext.rs:520) — Function navmap takes urlbase: &str, input: &str.
- function `textmap` (src/phext.rs:538) — Function textmap takes input: &str.
- function `checksum` (src/phext.rs:553) — Provides a 128-bit content checksum using the fastest algorithm Rust provides: xxh3
- function `manifest` (src/phext.rs:560) — Function manifest takes input: &str.
- function `soundex_v1` (src/phext.rs:600) — Function soundex_v1 takes input: &str.
- function `index` (src/phext.rs:638) — Function index takes input: &str.
- function `offset` (src/phext.rs:648) — The offset of the scroll at `coord`, or of the last scroll before it.
- function `replace` (src/phext.rs:663) — Function replace takes input: &str, location: Coordinate, scroll: &str.
- function `range_replace` (src/phext.rs:725) — errata: libphext-py opted for ranges that operate on a stack of positioned scrolls this method applies the range edit…
- function `insert` (src/phext.rs:750) — inserts the content specified in `scroll` at the coordinate within `phext` specified by `location`
- function `next_scroll` (src/phext.rs:813) — retrieves the next scroll from the given string, assuming an arbitrary starting point
- function `phokenize` (src/phext.rs:899) — Transforms a packed phext buffer into a phext token (phoken) stream One pass over the bytes.
- function `explode` (src/phext.rs:942) — Explodes an input phext into a hierarchical hashap, suitable for fast access and updates
- struct `ScrollSpan` (src/phext.rs:966) — The borrowed form of a `PositionedScroll`: a scroll's coordinate plus the byte range of its text within the input document.
- function `spans` (src/phext.rs:1012) — Builds a borrowed span index over `input`: one `ScrollSpan` per scroll, in document order, giving each scroll's…
- struct `TocChild` (src/phext.rs:1055) — One entry in a `toc_children` listing: a value taken by the next dimension below a coordinate prefix, together with…
- struct `TocSummary` (src/phext.rs:1068) — The aggregate shape of a coordinate prefix's subtree: how many scrolls it holds and how many bytes of scroll text…
- struct `TocScroll` (src/phext.rs:1081) — One entry in a `toc_scrolls` listing: a scroll's full coordinate and the byte length of its text.
- function `toc_children` (src/phext.rs:1131) — The progressive table of contents: given a coordinate prefix — the leading `prefix.len()` address components, 0 to 8…
- function `toc_summary` (src/phext.rs:1154) — The size of a subtree: the number of scrolls under `prefix` (0 to 9 leading address components) and their total byte length.
- function `toc_scrolls` (src/phext.rs:1176) — Every scroll under `prefix` (0 to 9 leading address components), in document order, as a `TocScroll`: full coordinate plus byte length, no bodies.
- function `toc_scrolls_by_suffix` (src/phext.rs:1199) — Every scroll whose coordinate ENDS with `suffix` — the trailing 1 to 9 address components — in document order, as…
- function `implode` (src/phext.rs:1217) — Serializes a hierarchical hash of scrolls into a monolithic phext document.
- function `merge` (src/phext.rs:1232) — combines `left` and `right` into a new phext document, with content merged on a per-scroll basis
- function `fetch` (src/phext.rs:1276) — retrieves the scroll located at the given coordinates.
- function `expand` (src/phext.rs:1301) — Function expand takes input: &str.
- function `contract` (src/phext.rs:1334) — Function contract takes input: &str.
- function `subtract` (src/phext.rs:1408) — Subtracts the scrolls from `left` where there is also content at those coordinates in `right` Note: This makes it…
- function `is_phext_break` (src/phext.rs:1438) — returns true if `byte` corresponds to one of our phext delimiters (line breaks included)
- function `normalize` (src/phext.rs:1452) — Function normalize takes input: &str.
- struct `Chain` (src/phext.rs:1473) — A chain: an ordered series of complete phexts within one stream, joined by CHAIN_BREAK (0x02) and optionally terminated by CHAIN_CLOSE (0x03).
- function `Chain::new` (src/phext.rs:1478) — Function Chain::new takes no inputs beyond the value it is called on.
- function `Chain::serialize` (src/phext.rs:1488) — serializes this chain alone: its members joined by CHAIN_BREAK, with a trailing CHAIN_CLOSE when the chain is closed.
- function `parse_chains` (src/phext.rs:1511) — splits a stream into its chains.
- function `serialize_chains` (src/phext.rs:1535) — serializes a stream of chains back into stream bytes: the inverse of `parse_chains`.
- function `default_coordinate` (src/phext.rs:1544) — Function default_coordinate takes no inputs beyond the value it is called on.
- function `to_coordinate` (src/phext.rs:1572) — translates a phext string to a strongly-typed address @param address  text to parse
- function `Coordinate::validate_coordinate` (src/phext.rs:1623) — determines if coord points to a valid phext address @param coord: the coordinate to reset
- function `Coordinate::to_string` (src/phext.rs:1643) — produces a quoted string for the given phext address in canonical format (z3.z2.z1/y3.y2.y1/x3.x2.x1) @param coord…
- function `Coordinate::to_urlencoded` (src/phext.rs:1653) — Function Coordinate::to_urlencoded takes no inputs beyond the value it is called on.
- function `Coordinate::scroll_break` (src/phext.rs:1674) — Function Coordinate::scroll_break takes no inputs beyond the value it is called on.
- function `Coordinate::section_break` (src/phext.rs:1681) — Function Coordinate::section_break takes no inputs beyond the value it is called on.
- function `Coordinate::chapter_break` (src/phext.rs:1689) — Function Coordinate::chapter_break takes no inputs beyond the value it is called on.
- function `Coordinate::book_break` (src/phext.rs:1698) — Function Coordinate::book_break takes no inputs beyond the value it is called on.
- function `Coordinate::volume_break` (src/phext.rs:1708) — Function Coordinate::volume_break takes no inputs beyond the value it is called on.
- function `Coordinate::collection_break` (src/phext.rs:1719) — Function Coordinate::collection_break takes no inputs beyond the value it is called on.
- function `Coordinate::series_break` (src/phext.rs:1731) — Function Coordinate::series_break takes no inputs beyond the value it is called on.
- function `Coordinate::shelf_break` (src/phext.rs:1744) — Function Coordinate::shelf_break takes no inputs beyond the value it is called on.
- function `Coordinate::library_break` (src/phext.rs:1758) — Function Coordinate::library_break takes no inputs beyond the value it is called on.
- function `parse_universe_address` (src/phext.rs:1775) — Function parse_universe_address takes address: &str.
- function `Chain::fetch` (src/phext.rs:1791) — Fetch a scroll by multiverse address: "$N:coord", or a bare coordinate (universe 1).
- struct `ScrollPosition` (src/phext.rs:1825) — A caret boundary inside one scroll: 1-based line and column (see the section note above).
- struct `PointAddress` (src/phext.rs:1835) — One parsed point address: universe (1-based; 1 when the "$N:" prefix is absent), the 9D coordinate, an optional caret…
- struct `RangeAddress` (src/phext.rs:1859) — A parsed range: two point endpoints, normalized to ascending order at parse time.
- function `parse_point_address` (src/phext.rs:1900) — parses a v0.6.1 point address: "[$N:]<coordinate>[@<line>,<column>[+<offsetChars>]]".
- function `parse_range_address` (src/phext.rs:1958) — parses a v0.6.1 range address: exactly two point addresses joined by one hyphen.
- function `Chain::resolve_point` (src/phext.rs:2000) — resolves a point address against this chain: (universe, coordinate, character offset within the scroll).
- function `Chain::fetch_point` (src/phext.rs:2018) — fetches a point address string.
- function `Chain::resolve_range` (src/phext.rs:2035) — resolves a range against this chain into per-scroll segments: (universe, coordinate, segment text) in ascending order.
- function `Chain::fetch_range` (src/phext.rs:2077) — fetches a range address string: the spanned text with phext structure bytes preserved — the covered segments…
- function `checked_scroll` (src/phext.rs:2123) — Returns `scroll` with an integrity trailer appended: one final line `phext-check:xxh3-128:<checksum of the payload>`.
- function `verify_checked_scroll` (src/phext.rs:2136) — true when `scroll` ends with a well-formed integrity trailer whose checksum matches the payload before it.
- function `uncheck_scroll` (src/phext.rs:2153) — Returns the payload of a checked scroll when its trailer verifies; None for unchecked, malformed, or corrupted scrolls.
- function `fetch_checked` (src/phext.rs:2174) — Fetches the scroll at `target` and returns its payload, but only when the scroll is checked and verifies.
- function `append` (src/phext.rs:2317) — Appends `scroll` at `location`, which must sort strictly after the document's last non-empty scroll.
- function `append_checked` (src/phext.rs:2351) — `append` with an integrity trailer: the stored scroll is `checked_scroll(scroll)`, verifiable later with…
- function `append_batch` (src/phext.rs:2369) — Appends several scrolls in one call.
- function `commit_scroll` (src/phext.rs:2400) — Builds the commit marker for a batch: a small scroll binding the batch's entry count and a checksum over each entry's coordinate and content, in order.
- function `verify_commit` (src/phext.rs:2425) — true when the scroll at `commit` is a commit marker whose count matches `entries` and whose checksum matches the entries' current contents, in order.
- const `SEAL_PRESET` (src/seal.rs:22) — The xz preset this implementation seals with.
- function `seal` (src/seal.rs:29) — seals a phext: xz-compresses its full UTF-8 bytes.
- function `seal_bytes` (src/seal.rs:38) — seals raw phext bytes (see `seal`).
- function `unseal` (src/seal.rs:50) — unseals a sealed phext back to its exact original bytes.
- function `unseal_string` (src/seal.rs:65) — unseals directly to text.

---

## struct — State

Found in counter/kernel/src/lib.rs at line 8.

Surface: public API.

Signature: `pub struct State {`

What it does: The whole state: a counter that must never pass its limit.

(Where these words came from: direct.)

---

## enum — Command

Found in counter/kernel/src/lib.rs at line 14.

Surface: public API.

Signature: `pub enum Command {`

What it does: A named choice named Command. It is one of: Incr; Reset.

(Where these words came from: inferred.)

---

## enum — Error

Found in counter/kernel/src/lib.rs at line 22.

Surface: public API.

Signature: `pub enum Error {`

What it does: A named choice named Error. It is one of: AtLimit.

(Where these words came from: inferred.)

---

## function — transition

Found in counter/kernel/src/lib.rs at line 27.

Surface: public API.

Signature: `pub fn transition(s: &State, cmd: &Command) -> Result<State, Error> {`

What it does: One step: the next state, or why the command is refused.

(Where these words came from: direct.)

---

## entry point — main

Found in examples/explode_bench.rs at line 11.

Surface: example.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## const — BINARY_SCROLL_HEADER

Found in src/binary.rs at line 42.

Surface: public API.

Signature: `pub const BINARY_SCROLL_HEADER: &str = "phext-binary:1:base64";`

What it does: The first (and only header) line of a binary scroll, without its line break.

(Where these words came from: direct.)

---

## function — is_binary_scroll

Found in src/binary.rs at line 53.

Surface: public API.

Signature: `pub fn is_binary_scroll(scroll: &str) -> bool {`

What it does: true when `scroll` carries the binary scroll header. Decoding success is a separate question — see `decode_scroll`.

(Where these words came from: direct.)

---

## function — encode_scroll

Found in src/binary.rs at line 62.

Surface: public API.

Signature: `pub fn encode_scroll(payload: &[u8]) -> String {`

What it does: encodes raw bytes as a binary scroll (header line + base64 body)

(Where these words came from: direct.)

---

## function — decode_scroll

Found in src/binary.rs at line 75.

Surface: public API.

Signature: `pub fn decode_scroll(scroll: &str) -> Option<Vec<u8>> {`

What it does: decodes a binary scroll back to its raw bytes. Returns None when the scroll is not a binary scroll, or when its body is not strict base64.

(Where these words came from: direct.)

---

## function — insert_binary

Found in src/binary.rs at line 87.

Surface: public API.

Signature: `pub fn insert_binary(input: String, location: Coordinate, payload: &[u8]) -> String {`

What it does: stores `payload` at `location` as a binary scroll, unconditionally: the caller has declared this scroll binary, even if the bytes happen to be valid UTF-8.

(Where these words came from: direct.)

---

## function — replace_binary

Found in src/binary.rs at line 97.

Surface: public API.

Signature: `pub fn replace_binary(input: &str, location: Coordinate, payload: &[u8]) -> String {`

What it does: replaces the scroll at `location` with `payload`, stored as a binary scroll (see `insert_binary`).

(Where these words came from: direct.)

---

## function — fetch_binary

Found in src/binary.rs at line 110.

Surface: public API.

Signature: `pub fn fetch_binary(input: &str, target: Coordinate) -> Vec<u8> {`

What it does: retrieves the bytes held at `target`. A well-formed binary scroll decodes to its payload. Any other scroll — plain text, a malformed binary scroll, or no scroll at all — yields the scroll's own UTF-8 bytes (empty when the coordinate is unoccupied), mirroring how `phext::fetch` returns "" there. This function is total: it never panics and never loses scroll content.

(Where these words came from: direct.)

---

## function — insert_bytes

Found in src/binary.rs at line 134.

Surface: public API.

Signature: `pub fn insert_bytes(input: String, location: Coordinate, payload: &[u8]) -> String {`

What it does: byte-oriented insert: stores `payload` at `location` under the ingestion rule (UTF-8 as text, non-UTF-8 as a base64-verbatim binary scroll).

(Where these words came from: direct.)

---

## function — replace_bytes

Found in src/binary.rs at line 144.

Surface: public API.

Signature: `pub fn replace_bytes(input: &str, location: Coordinate, payload: &[u8]) -> String {`

What it does: byte-oriented replace: stores `payload` at `location` under the ingestion rule (UTF-8 as text, non-UTF-8 as a base64-verbatim binary scroll).

(Where these words came from: direct.)

---

## function — phokenize_bytes

Found in src/binary.rs at line 180.

Surface: public API.

Signature: `pub fn phokenize_bytes(input: &[u8]) -> Vec<(Coordinate, Vec<u8>)> {`

What it does: the byte-level twin of `phext::phokenize`: walks a raw byte stream and returns each scroll's coordinate and raw bytes, with identical segmentation. Dimension-break bytes are ASCII, so byte offsets are exact; scroll bodies are returned uninterpreted, valid UTF-8 or not.

(Where these words came from: direct.)

---

## function — ingest

Found in src/binary.rs at line 227.

Surface: public API.

Signature: `pub fn ingest(bytes: &[u8]) -> String {`

What it does: ingests an externally supplied phext byte stream. Every scroll is observed as raw bytes: valid UTF-8 scrolls pass through verbatim, and any scroll whose bytes are not valid UTF-8 is stored as a binary scroll holding the base64 of the exact observed bytes (the ingestion rule), so `fetch_binary` at its coordinate returns the original bytes exactly. Never rejects, never panics. The result is the canonical serialization (the same normal form as `phext::implode` of `phext::explode`).

(Where these words came from: direct.)

---

## const — FIELD_PREFIX

Found in src/binary.rs at line 243.

Surface: public API.

Signature: `pub const FIELD_PREFIX: &str = "phext-field:base64:";`

What it does: The prefix of a binary *field*: arbitrary bytes encoded for one line inside a text scroll (a `name=value` grammar, a WAL entry, a header block). Whole scrolls use `encode_scroll`; fields use this.

(Where these words came from: direct.)

---

## function — encode_field

Found in src/binary.rs at line 255.

Surface: public API.

Signature: `pub fn encode_field(payload: &[u8]) -> String {`

What it does: Encodes raw bytes as a single line of text: the field prefix plus RFC 4648 base64. The result contains no line breaks and none of the nine phext dimension-break bytes, so it can sit inside any line-structured scroll. The base64 overhead (+33%) is inherent to armouring bytes in a text line; this function standardizes the armour so every implementation reads the same field the same way.

(Where these words came from: direct.)

---

## function — decode_field

Found in src/binary.rs at line 269.

Surface: public API.

Signature: `pub fn decode_field(text: &str) -> Option<Vec<u8>> {`

What it does: Decodes a field produced by `encode_field`. Returns None when the text lacks the field prefix or its body is not strict base64 — a plain text field is not a binary field, and yields None, not bytes.

(Where these words came from: direct.)

---

## const — EPOCH_ANCHOR_UNIX

Found in src/epoch.rs at line 58.

Surface: public API.

Signature: `pub const EPOCH_ANCHOR_UNIX: i64 = 1795327200;`

What it does: The unix timestamp at which EE 1 begins: 2026-11-22T06:00:00Z.

(Where these words came from: direct.)

---

## const — EPOCH_LENGTH_SECONDS

Found in src/epoch.rs at line 61.

Surface: public API.

Signature: `pub const EPOCH_LENGTH_SECONDS: i64 = 420;`

What it does: The length of one Exocortical Epoch, in seconds (7 minutes).

(Where these words came from: direct.)

---

## const — AGE_BITS

Found in src/epoch.rs at line 65.

Surface: public API.

Signature: `pub const AGE_BITS: u32 = 42;`

What it does: The width of a Sentron Age field, in bits: one epoch is divided into 2^42 Sentron Ages.

(Where these words came from: direct.)

---

## const — AGES_PER_EPOCH

Found in src/epoch.rs at line 71.

Surface: public API.

Signature: `pub const AGES_PER_EPOCH: u64 = 1 << AGE_BITS;`

What it does: The number of Sentron Ages in one Exocortical Epoch: 2^42 = 4,398,046,511,104. The canonical reference clock is AGES_PER_EPOCH / 420 ≈ 10.4715 GHz, so one Sentron Age is ≈ 95.5 picoseconds.

(Where these words came from: direct.)

---

## const — AGE_MAX

Found in src/epoch.rs at line 74.

Surface: public API.

Signature: `pub const AGE_MAX: u64 = AGES_PER_EPOCH - 1;`

What it does: The largest Sentron Age: the final nominal step of an epoch.

(Where these words came from: direct.)

---

## const — COARSE_AGE_BITS

Found in src/epoch.rs at line 80.

Surface: public API.

Signature: `pub const COARSE_AGE_BITS: u32 = 16;`

What it does: The width of a coarse Sentron Age field, in bits: the fine Sentron Age scaled into a 16-bit address space (coarse = fine >> (AGE_BITS − COARSE_AGE_BITS)). One coarse tick is 420 / 65536 s ≈ 6.41 ms.

(Where these words came from: direct.)

---

## const — COARSE_AGES_PER_EPOCH

Found in src/epoch.rs at line 84.

Surface: public API.

Signature: `pub const COARSE_AGES_PER_EPOCH: u64 = 1 << COARSE_AGE_BITS;`

What it does: The number of coarse Sentron Ages in one Exocortical Epoch: 2^16 = 65,536.

(Where these words came from: direct.)

---

## const — COARSE_AGE_MAX

Found in src/epoch.rs at line 87.

Surface: public API.

Signature: `pub const COARSE_AGE_MAX: u64 = COARSE_AGES_PER_EPOCH - 1;`

What it does: The largest coarse Sentron Age: the final coarse step of an epoch.

(Where these words came from: direct.)

---

## function — epoch_at

Found in src/epoch.rs at line 97.

Surface: public API.

Signature: `pub fn epoch_at(unix_seconds: i64) -> u64 {`

What it does: the Exocortical Epoch containing the given unix timestamp (UTC seconds). Returns 0 for every instant before the anchor — EE 0, the pre-epoch. For instants at or after the anchor this is `floor((t − anchor) / 420) + 1`, computed in 64-bit arithmetic.

(Where these words came from: direct.)

---

## function — epoch_start

Found in src/epoch.rs at line 111.

Surface: public API.

Signature: `pub fn epoch_start(epoch: u64) -> Option<i64> {`

What it does: the unix timestamp at which the given epoch begins. `None` for EE 0 — the pre-epoch has no start — and `None` for epoch numbers so large that the start is not representable as an i64 unix timestamp.

(Where these words came from: direct.)

---

## function — epoch_end

Found in src/epoch.rs at line 126.

Surface: public API.

Signature: `pub fn epoch_end(epoch: u64) -> Option<i64> {`

What it does: the unix timestamp at which the given epoch ends — the next epoch's start. `None` for EE 0, and `None` when the end is not representable as an i64 unix timestamp.

(Where these words came from: direct.)

---

## function — epoch_range

Found in src/epoch.rs at line 137.

Surface: public API.

Signature: `pub fn epoch_range(epoch: u64) -> Option<(i64, i64)> {`

What it does: the (start, end) unix timestamps bounding the given epoch. `None` for EE 0 — all of history before the anchor is not an interval this clock bounds — and `None` when either bound is not representable as i64.

(Where these words came from: direct.)

---

## function — seconds_into_epoch

Found in src/epoch.rs at line 148.

Surface: public API.

Signature: `pub fn seconds_into_epoch(unix_seconds: i64) -> Option<i64> {`

What it does: how far the given unix timestamp sits inside its epoch, in seconds (0 at an epoch's first instant, 419 at its last). `None` before the anchor: a pre-epoch instant is not inside any epoch.

(Where these words came from: direct.)

---

## function — seconds_remaining

Found in src/epoch.rs at line 163.

Surface: public API.

Signature: `pub fn seconds_remaining(unix_seconds: i64) -> i64 {`

What it does: seconds until the current epoch ends. Before the anchor this counts down to the start of EE 1 (at the anchor itself it reads 420 — the full first epoch lies ahead); at or after the anchor it is the time left in the containing epoch, in the range 1..=420.

(Where these words came from: direct.)

---

## function — age_at

Found in src/epoch.rs at line 186.

Surface: public API.

Signature: `pub fn age_at(unix_seconds: i64) -> Option<u64> {`

What it does: the Sentron Age of the given unix timestamp: its nominal position within its epoch, `floor(seconds_into_epoch × 2^42 / 420)`, in the range 0..=AGE_MAX. `None` before the anchor, exactly as `seconds_into_epoch` — a pre-epoch instant is inside no epoch, so it has no Sentron Age. Whole-second input fixes the Sentron Age at that second's first instant; `age_at_nanos` refines it with the sub-second part. A Sentron Age is a *local* position: 1 ms of clock skew moves it by ≈ 10.47 million Sentron Ages, so Sentron Ages order events on one clock — the epoch count is the coordinate that travels between machines.

(Where these words came from: direct.)

---

## function — age_at_nanos

Found in src/epoch.rs at line 204.

Surface: public API.

Signature: `pub fn age_at_nanos(unix_seconds: i64, nanoseconds: u32) -> Option<u64> {`

What it does: the Sentron Age of a timestamp given with sub-second precision: `unix_seconds` plus `nanoseconds` (0..1_000_000_000) within that second. The Sentron Age is `floor(elapsed_ns × 2^42 / (420 × 10^9))` over the nanoseconds elapsed in the containing epoch, computed in 128-bit arithmetic (elapsed_ns × 2^42 overflows u64). `None` before the anchor, and `None` for an out-of-range nanosecond part. With `nanoseconds == 0` the result agrees exactly with `age_at`.

(Where these words came from: direct.)

---

## function — age_start

Found in src/epoch.rs at line 231.

Surface: public API.

Signature: `pub fn age_start(epoch: u64, age: u64) -> Option<i64> {`

What it does: the unix timestamp of the whole second in which the given Sentron Age of the given epoch begins: `epoch_start(epoch) + floor(age × 420 / 2^42)`. `None` for EE 0, for `age > AGE_MAX`, and when that second is not representable as an i64 unix timestamp. This is the coarsest faithful inverse of `age_at`, and its coarseness is the module's honesty about resolution: a Sentron Age is ≈ 95.5 ps wide, so ~10.47 billion Sentron Ages begin inside each second and no whole-second timestamp can name one Sentron Age's exact start. The inverse therefore names the containing second — expect it to read one second early when the Sentron Age in force at a second boundary began just before it.

(Where these words came from: direct.)

---

## function — coarse_age_at

Found in src/epoch.rs at line 257.

Surface: public API.

Signature: `pub fn coarse_age_at(unix_seconds: i64) -> Option<u64> {`

What it does: the coarse Sentron Age of the given unix timestamp: the fine Sentron Age scaled into a 16-bit address space, `age_at(t) >> 26`, equivalently `floor(seconds_into_epoch × 65536 / 420)`, in the range 0..=COARSE_AGE_MAX. `None` before the anchor, exactly as `age_at`. This is the register-grade form. One coarse tick is ≈ 6.41 ms, so where a fine Sentron Age (≈ 95.5 ps) is destroyed by any real clock skew — 1 ms is ≈ 10.47 million fine Sentron Ages — NTP-class skew of a few milliseconds costs a coarse Sentron Age about a tick. Coarse Sentron Ages are therefore approximately comparable across machines, and they are the Age component the register system carries.

(Where these words came from: direct.)

---

## const — COORDINATE_MINIMUM

Found in src/phext.rs at line 86.

Surface: public API.

Signature: `pub const COORDINATE_MINIMUM: usize = 1; // human numbering - we start at 1, not 0`

What it does: phext constants

(Where these words came from: direct.)

---

## const — LIBRARY_BREAK

Found in src/phext.rs at line 89.

Surface: public API.

Signature: `pub const LIBRARY_BREAK: char = '\x01'; // 11th dimension - replaces start of header`

What it does: A fixed value named LIBRARY_BREAK, set to '\x01'.

(Where these words came from: inferred.)

---

## const — CHAIN_BREAK

Found in src/phext.rs at line 90.

Surface: public API.

Signature: `pub const CHAIN_BREAK: char = '\x02'; // stream level - separates whole phexts within a chain; replaces start of text`

What it does: A fixed value named CHAIN_BREAK, set to '\x02'.

(Where these words came from: inferred.)

---

## const — CHAIN_CLOSE

Found in src/phext.rs at line 91.

Surface: public API.

Signature: `pub const CHAIN_CLOSE: char = '\x03'; // stream level - terminates a chain; replaces end of text`

What it does: A fixed value named CHAIN_CLOSE, set to '\x03'.

(Where these words came from: inferred.)

---

## const — MORE_COWBELL

Found in src/phext.rs at line 92.

Surface: public API.

Signature: `pub const MORE_COWBELL: char = '\x07'; // i've got a fever, and the only prescription...is more cowbell!`

What it does: A fixed value named MORE_COWBELL, set to '\x07'.

(Where these words came from: inferred.)

---

## const — LINE_BREAK

Found in src/phext.rs at line 93.

Surface: public API.

Signature: `pub const LINE_BREAK: char = '\x0A'; // same as plain text \o/`

What it does: A fixed value named LINE_BREAK, set to '\x0A'.

(Where these words came from: inferred.)

---

## const — SCROLL_BREAK

Found in src/phext.rs at line 94.

Surface: public API.

Signature: `pub const SCROLL_BREAK: char = '\x17'; // 3D Break - replaces End Transmission Block`

What it does: A fixed value named SCROLL_BREAK, set to '\x17'.

(Where these words came from: inferred.)

---

## const — SECTION_BREAK

Found in src/phext.rs at line 95.

Surface: public API.

Signature: `pub const SECTION_BREAK: char = '\x18'; // 4D Break - replaces Cancel Block`

What it does: A fixed value named SECTION_BREAK, set to '\x18'.

(Where these words came from: inferred.)

---

## const — CHAPTER_BREAK

Found in src/phext.rs at line 96.

Surface: public API.

Signature: `pub const CHAPTER_BREAK: char = '\x19'; // 5D Break - replaces End of Tape`

What it does: A fixed value named CHAPTER_BREAK, set to '\x19'.

(Where these words came from: inferred.)

---

## const — BOOK_BREAK

Found in src/phext.rs at line 97.

Surface: public API.

Signature: `pub const BOOK_BREAK: char = '\x1A'; // 6D Break - replaces Substitute`

What it does: A fixed value named BOOK_BREAK, set to '\x1A'.

(Where these words came from: inferred.)

---

## const — VOLUME_BREAK

Found in src/phext.rs at line 98.

Surface: public API.

Signature: `pub const VOLUME_BREAK: char = '\x1C'; // 7D Break - replaces file separator`

What it does: A fixed value named VOLUME_BREAK, set to '\x1C'.

(Where these words came from: inferred.)

---

## const — COLLECTION_BREAK

Found in src/phext.rs at line 99.

Surface: public API.

Signature: `pub const COLLECTION_BREAK: char = '\x1D'; // 8D Break - replaces group separator`

What it does: A fixed value named COLLECTION_BREAK, set to '\x1D'.

(Where these words came from: inferred.)

---

## const — SERIES_BREAK

Found in src/phext.rs at line 100.

Surface: public API.

Signature: `pub const SERIES_BREAK: char = '\x1E'; // 9D Break - replaces record separator`

What it does: A fixed value named SERIES_BREAK, set to '\x1E'.

(Where these words came from: inferred.)

---

## const — SHELF_BREAK

Found in src/phext.rs at line 101.

Surface: public API.

Signature: `pub const SHELF_BREAK: char = '\x1F'; // 10D Break - replaces unit separator`

What it does: A fixed value named SHELF_BREAK, set to '\x1F'.

(Where these words came from: inferred.)

---

## const — LIBRARY_BREAK_U8

Found in src/phext.rs at line 106.

Surface: public API.

Signature: `pub const LIBRARY_BREAK_U8: u8 = 0x01; // byte form of LIBRARY_BREAK`

What it does: A fixed value named LIBRARY_BREAK_U8, set to 0x01.

(Where these words came from: inferred.)

---

## const — CHAIN_BREAK_U8

Found in src/phext.rs at line 107.

Surface: public API.

Signature: `pub const CHAIN_BREAK_U8: u8 = 0x02; // byte form of CHAIN_BREAK`

What it does: A fixed value named CHAIN_BREAK_U8, set to 0x02.

(Where these words came from: inferred.)

---

## const — CHAIN_CLOSE_U8

Found in src/phext.rs at line 108.

Surface: public API.

Signature: `pub const CHAIN_CLOSE_U8: u8 = 0x03; // byte form of CHAIN_CLOSE`

What it does: A fixed value named CHAIN_CLOSE_U8, set to 0x03.

(Where these words came from: inferred.)

---

## const — MORE_COWBELL_U8

Found in src/phext.rs at line 109.

Surface: public API.

Signature: `pub const MORE_COWBELL_U8: u8 = 0x07; // byte form of MORE_COWBELL`

What it does: A fixed value named MORE_COWBELL_U8, set to 0x07.

(Where these words came from: inferred.)

---

## const — LINE_BREAK_U8

Found in src/phext.rs at line 110.

Surface: public API.

Signature: `pub const LINE_BREAK_U8: u8 = 0x0A; // byte form of LINE_BREAK`

What it does: A fixed value named LINE_BREAK_U8, set to 0x0A.

(Where these words came from: inferred.)

---

## const — SCROLL_BREAK_U8

Found in src/phext.rs at line 111.

Surface: public API.

Signature: `pub const SCROLL_BREAK_U8: u8 = 0x17; // byte form of SCROLL_BREAK`

What it does: A fixed value named SCROLL_BREAK_U8, set to 0x17.

(Where these words came from: inferred.)

---

## const — SECTION_BREAK_U8

Found in src/phext.rs at line 112.

Surface: public API.

Signature: `pub const SECTION_BREAK_U8: u8 = 0x18; // byte form of SECTION_BREAK`

What it does: A fixed value named SECTION_BREAK_U8, set to 0x18.

(Where these words came from: inferred.)

---

## const — CHAPTER_BREAK_U8

Found in src/phext.rs at line 113.

Surface: public API.

Signature: `pub const CHAPTER_BREAK_U8: u8 = 0x19; // byte form of CHAPTER_BREAK`

What it does: A fixed value named CHAPTER_BREAK_U8, set to 0x19.

(Where these words came from: inferred.)

---

## const — BOOK_BREAK_U8

Found in src/phext.rs at line 114.

Surface: public API.

Signature: `pub const BOOK_BREAK_U8: u8 = 0x1A; // byte form of BOOK_BREAK`

What it does: A fixed value named BOOK_BREAK_U8, set to 0x1A.

(Where these words came from: inferred.)

---

## const — VOLUME_BREAK_U8

Found in src/phext.rs at line 115.

Surface: public API.

Signature: `pub const VOLUME_BREAK_U8: u8 = 0x1C; // byte form of VOLUME_BREAK`

What it does: A fixed value named VOLUME_BREAK_U8, set to 0x1C.

(Where these words came from: inferred.)

---

## const — COLLECTION_BREAK_U8

Found in src/phext.rs at line 116.

Surface: public API.

Signature: `pub const COLLECTION_BREAK_U8: u8 = 0x1D; // byte form of COLLECTION_BREAK`

What it does: A fixed value named COLLECTION_BREAK_U8, set to 0x1D.

(Where these words came from: inferred.)

---

## const — SERIES_BREAK_U8

Found in src/phext.rs at line 117.

Surface: public API.

Signature: `pub const SERIES_BREAK_U8: u8 = 0x1E; // byte form of SERIES_BREAK`

What it does: A fixed value named SERIES_BREAK_U8, set to 0x1E.

(Where these words came from: inferred.)

---

## const — SHELF_BREAK_U8

Found in src/phext.rs at line 118.

Surface: public API.

Signature: `pub const SHELF_BREAK_U8: u8 = 0x1F; // byte form of SHELF_BREAK`

What it does: A fixed value named SHELF_BREAK_U8, set to 0x1F.

(Where these words came from: inferred.)

---

## const — ADDRESS_MICRO_BREAK

Found in src/phext.rs at line 120.

Surface: public API.

Signature: `pub const ADDRESS_MICRO_BREAK: u8 = b'.'; // delimiter for micro-coordinates`

What it does: A fixed value named ADDRESS_MICRO_BREAK, set to b'.'.

(Where these words came from: inferred.)

---

## const — ADDRESS_MACRO_BREAK

Found in src/phext.rs at line 121.

Surface: public API.

Signature: `pub const ADDRESS_MACRO_BREAK: u8 = b'/'; // delimiter for macro-coordinates`

What it does: A fixed value named ADDRESS_MACRO_BREAK, set to b'/'.

(Where these words came from: inferred.)

---

## const — ADDRESS_MACRO_ALT

Found in src/phext.rs at line 122.

Surface: public API.

Signature: `pub const ADDRESS_MACRO_ALT: u8 = b';'; // also allow ';' for url encoding`

What it does: A fixed value named ADDRESS_MACRO_ALT, set to b';'.

(Where these words came from: inferred.)

---

## struct — ZCoordinate

Found in src/phext.rs at line 169.

Surface: public API.

Signature: `pub struct ZCoordinate {`

What it does: The large-scale Z arm of a phext coordinate (see `Coordinate` below)

(Where these words came from: direct.)

---

## function — ZCoordinate::new

Found in src/phext.rs at line 175.

Surface: public API.

Signature: `pub fn new() -> ZCoordinate {`

What it does: Function ZCoordinate::new takes no inputs beyond the value it is called on. It gives back ZCoordinate.

(Where these words came from: inferred.)

---

## struct — YCoordinate

Found in src/phext.rs at line 196.

Surface: public API.

Signature: `pub struct YCoordinate {`

What it does: The large-scale Y arm of a phext coordinate (see `Coordinate` below)

(Where these words came from: direct.)

---

## function — YCoordinate::new

Found in src/phext.rs at line 202.

Surface: public API.

Signature: `pub fn new() -> YCoordinate {`

What it does: Function YCoordinate::new takes no inputs beyond the value it is called on. It gives back YCoordinate.

(Where these words came from: inferred.)

---

## struct — XCoordinate

Found in src/phext.rs at line 223.

Surface: public API.

Signature: `pub struct XCoordinate {`

What it does: The large-scale X arm of a phext coordinate (see `Coordinate` below)

(Where these words came from: direct.)

---

## function — XCoordinate::new

Found in src/phext.rs at line 229.

Surface: public API.

Signature: `pub fn new() -> XCoordinate {`

What it does: Function XCoordinate::new takes no inputs beyond the value it is called on. It gives back XCoordinate.

(Where these words came from: inferred.)

---

## struct — Coordinate

Found in src/phext.rs at line 264.

Surface: public API.

Signature: `pub struct Coordinate {`

What it does: provides access to a default-initialized coordinate at 1.1.1/1.1.1/1.1.1 phext coordinates are formed along a 9-dimensional hierarchy with three main arms of the form z3.z2.z1/y3.y2.y1.x3.x2.x1 where: Z - this arm contains the library (z3), shelf (z2), and series (z1) dimensions Y - this arm contains the collection (y3), volume (y2), and book (y1) dimensions X - this arm contains the chapter (x3), section (x2), and scroll (x1) dimensions Dense graphs: phext is sparse by design. Coordinate components are unbounded, but a fully-populated dense graph contains the product of its dimensions' extents in scrolls — a dense block spanning 1000 in each of 9 dimensions implies 10^27 scrolls. Materializing, exploding, or indexing such a graph costs memory proportional to its populated scroll count. The old 999 cap never protected against this (999^9 was already unreachable); sparsity does. Keep graphs sparse.

(Where these words came from: direct.)

---

## function — Coordinate::new

Found in src/phext.rs at line 270.

Surface: public API.

Signature: `pub fn new() -> Coordinate {`

What it does: Function Coordinate::new takes no inputs beyond the value it is called on. It gives back Coordinate.

(Where these words came from: inferred.)

---

## struct — PositionedScroll

Found in src/phext.rs at line 281.

Surface: public API.

Signature: `pub struct PositionedScroll {`

What it does: A data shape named PositionedScroll. Its fields are: pub coord: Coordinate; pub scroll: String.

(Where these words came from: inferred.)

---

## function — PositionedScroll::new

Found in src/phext.rs at line 286.

Surface: public API.

Signature: `pub fn new() -> PositionedScroll {`

What it does: Function PositionedScroll::new takes no inputs beyond the value it is called on. It gives back PositionedScroll.

(Where these words came from: inferred.)

---

## struct — Range

Found in src/phext.rs at line 302.

Surface: public API.

Signature: `pub struct Range {`

What it does: A data shape named Range. Its fields are: pub start: Coordinate; pub end: Coordinate.

(Where these words came from: inferred.)

---

## function — Range::new

Found in src/phext.rs at line 307.

Surface: public API.

Signature: `pub fn new() -> Range {`

What it does: Function Range::new takes no inputs beyond the value it is called on. It gives back Range.

(Where these words came from: inferred.)

---

## struct — PhextParseError

Found in src/phext.rs at line 318.

Surface: public API.

Signature: `pub struct PhextParseError;`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## function — check_for_cowbell

Found in src/phext.rs at line 412.

Surface: public API.

Signature: `pub fn check_for_cowbell(input: &str) -> bool {`

What it does: Function check_for_cowbell takes input: &str. It gives back bool.

(Where these words came from: inferred.)

---

## function — get_subspace_coordinates

Found in src/phext.rs at line 441.

Surface: public API.

Signature: `pub fn get_subspace_coordinates(subspace: &[u8], target: Coordinate) -> (usize, usize, Coordinate) {`

What it does: finds the start and end offsets for the given coordinate Returns `(start, end, best)`: when the document reaches `target`, `start..end` is that scroll's text (empty if it has none) and `best` is `target`; otherwise `start == end` is where the scroll would go: just before the break that steps past `target`, or the end of the document. `best` is the coordinate in effect there, so writing breaks from `best` up to `target` and then the text places it correctly. The walker only moves on a dimension break, so it is compared with `target` only at the start and after each break, and the scan stops as soon as it steps past `target`. Earlier versions returned `start = 0` for a missing coordinate that falls between two scrolls, so `fetch`, `replace`, and `remove` acted on the scroll before it, and they missed a scroll reached only by breaks at the very end of the document.

(Where these words came from: direct.)

---

## function — remove

Found in src/phext.rs at line 495.

Surface: public API.

Signature: `pub fn remove(input: &str, location: Coordinate) -> String {`

What it does: Function remove takes input: &str, location: Coordinate. It gives back String.

(Where these words came from: inferred.)

---

## function — create_summary

Found in src/phext.rs at line 501.

Surface: public API.

Signature: `pub fn create_summary(input: &str) -> String {`

What it does: Function create_summary takes input: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — navmap

Found in src/phext.rs at line 520.

Surface: public API.

Signature: `pub fn navmap(urlbase: &str, input: &str) -> String {`

What it does: Function navmap takes urlbase: &str, input: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — textmap

Found in src/phext.rs at line 538.

Surface: public API.

Signature: `pub fn textmap(input: &str) -> String {`

What it does: Function textmap takes input: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — checksum

Found in src/phext.rs at line 553.

Surface: public API.

Signature: `pub fn checksum(input: &str) -> String {`

What it does: Provides a 128-bit content checksum using the fastest algorithm Rust provides: xxh3

(Where these words came from: direct.)

---

## function — manifest

Found in src/phext.rs at line 560.

Surface: public API.

Signature: `pub fn manifest(input: &str) -> String {`

What it does: Function manifest takes input: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — soundex_v1

Found in src/phext.rs at line 600.

Surface: public API.

Signature: `pub fn soundex_v1(input: &str) -> String {`

What it does: Function soundex_v1 takes input: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — index

Found in src/phext.rs at line 638.

Surface: public API.

Signature: `pub fn index(input: &str) -> String {`

What it does: Function index takes input: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — offset

Found in src/phext.rs at line 648.

Surface: public API.

Signature: `pub fn offset(input: &str, coord: Coordinate) -> usize {`

What it does: The offset of the scroll at `coord`, or of the last scroll before it. 0 when no scroll comes at or before `coord` (earlier versions panicked there).

(Where these words came from: direct.)

---

## function — replace

Found in src/phext.rs at line 663.

Surface: public API.

Signature: `pub fn replace(input: &str, location: Coordinate, scroll: &str) -> String {`

What it does: Function replace takes input: &str, location: Coordinate, scroll: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — range_replace

Found in src/phext.rs at line 725.

Surface: public API.

Signature: `pub fn range_replace(input: &str, location: Range, scroll: &str) -> String {`

What it does: errata: libphext-py opted for ranges that operate on a stack of positioned scrolls this method applies the range edit to subspace directly...we probably need both

(Where these words came from: direct.)

---

## function — insert

Found in src/phext.rs at line 750.

Surface: public API.

Signature: `pub fn insert(input: String, location: Coordinate, scroll: &str) -> String {`

What it does: inserts the content specified in `scroll` at the coordinate within `phext` specified by `location`

(Where these words came from: direct.)

---

## function — next_scroll

Found in src/phext.rs at line 813.

Surface: public API.

Signature: `pub fn next_scroll(input: &str, start: Coordinate) -> (PositionedScroll, Coordinate, String) {`

What it does: retrieves the next scroll from the given string, assuming an arbitrary starting point

(Where these words came from: direct.)

---

## function — phokenize

Found in src/phext.rs at line 899.

Surface: public API.

Signature: `pub fn phokenize(input: &str) -> Vec<PositionedScroll> {`

What it does: Transforms a packed phext buffer into a phext token (phoken) stream One pass over the bytes. Each step matches a call to `next_scroll`: skip leading breaks, take a run of text, then consume the break that ends it. Earlier versions called `next_scroll` in a loop, which copied the rest of the document for every scroll and made this quadratic in the document size.

(Where these words came from: direct.)

---

## function — explode

Found in src/phext.rs at line 942.

Surface: public API.

Signature: `pub fn explode(input: &str) -> HashMap<Coordinate, String> {`

What it does: Explodes an input phext into a hierarchical hashap, suitable for fast access and updates

(Where these words came from: direct.)

---

## struct — ScrollSpan

Found in src/phext.rs at line 966.

Surface: public API.

Signature: `pub struct ScrollSpan {`

What it does: The borrowed form of a `PositionedScroll`: a scroll's coordinate plus the byte range of its text within the input document. Slicing the input with `range` yields exactly the scroll's text — building the index allocates no `String` per scroll.

(Where these words came from: direct.)

---

## function — spans

Found in src/phext.rs at line 1012.

Surface: public API.

Signature: `pub fn spans(input: &str) -> Vec<ScrollSpan> {`

What it does: Builds a borrowed span index over `input`: one `ScrollSpan` per scroll, in document order, giving each scroll's coordinate and the byte range of its text in `input`. The walk is the `phokenize` walk — the same dimension breaks and the same coordinate arithmetic — except that it records byte ranges instead of allocating a `String` for every scroll. For every returned span, `&input[span.range]` is exactly the scroll `explode` returns at `span.coord`, and `span.range.start` is the offset `offset` reports for that coordinate. Empty-scroll contract: a scroll exists iff it holds at least one byte — the definition `explode` encodes — so `spans` reports only non-empty scrolls. Delimiter runs that encode empty scrolls (leading breaks, runs between distant scrolls, trailing breaks) produce no span; an empty document, or a document of only delimiters, produces no spans. `phokenize` remains the tokenizer of record for empty slots: it additionally reports a final empty positioned scroll in those trailing cases, where `spans` reports none. Chain bytes (0x02 / 0x03) are not dimension breaks, so — exactly as in `phokenize` and `explode` — they are scroll content here. This index covers a single phext (one universe); chain structure is the `parse_chains` layer above it. Motivating consumer: holo-phext, whose `src/index.rs` carries a 323-line re-implementation of this walk, plus a duplicated private dimension-break predicate in its core, because libphext exported only owned-`String` tokenization (`phokenize`/`explode`) and a single-scroll `offset`. Downstream stores should index with `spans` instead of re-walking the format. # Example ``` use libphext::phext::{spans, to_coordinate}; let input = "alpha\u{17}beta"; let index = spans(input); assert_eq!(index.len(), 2); assert_eq!(index[0].coord, to_coordinate("1.1.1/1.1.1/1.1.1")); assert_eq!(&input[index[0].range.clone()], "alpha"); assert_eq!(&input[index[1].range.clone()], "beta"); ```

(Where these words came from: direct.)

---

## struct — TocChild

Found in src/phext.rs at line 1055.

Surface: public API.

Signature: `pub struct TocChild {`

What it does: One entry in a `toc_children` listing: a value taken by the next dimension below a coordinate prefix, together with the number of scrolls and the total scroll bytes in that child's subtree. Structure only — no scroll text is copied or returned.

(Where these words came from: direct.)

---

## struct — TocSummary

Found in src/phext.rs at line 1068.

Surface: public API.

Signature: `pub struct TocSummary {`

What it does: The aggregate shape of a coordinate prefix's subtree: how many scrolls it holds and how many bytes of scroll text they carry in total.

(Where these words came from: direct.)

---

## struct — TocScroll

Found in src/phext.rs at line 1081.

Surface: public API.

Signature: `pub struct TocScroll {`

What it does: One entry in a `toc_scrolls` listing: a scroll's full coordinate and the byte length of its text. The planet list for progressive clients — coordinates and sizes, never bodies.

(Where these words came from: direct.)

---

## function — toc_children

Found in src/phext.rs at line 1131.

Surface: public API.

Signature: `pub fn toc_children(input: &str, prefix: &[usize]) -> Vec<TocChild> {`

What it does: The progressive table of contents: given a coordinate prefix — the leading `prefix.len()` address components, 0 to 8 of them — returns one `TocChild` per distinct value the next dimension takes under that prefix, sorted ascending, each with the scroll count and total scroll bytes of its subtree. A client walking a phext from the root calls `toc_children(input, &[])` for the libraries present, then `toc_children(input, &[2])` for the shelves of library 2, and so on — downloading structure one level at a time instead of the whole document. Built on the `spans` index in a single pass; scroll bodies are never copied. A prefix of 9 or more components has no next dimension and yields an empty listing, as does a prefix no scroll matches. The children partition their parent: for any prefix shorter than 9 components, the `scrolls` of `toc_children(input, prefix)` sum to `toc_summary(input, prefix).scrolls`, and the `bytes` sum likewise. # Example ``` use libphext::phext::toc_children; let input = "alpha\u{17}beta\u{18}gamma"; let root = toc_children(input, &[]); assert_eq!(root.len(), 1); assert_eq!(root[0].value, 1); assert_eq!(root[0].scrolls, 3); ```

(Where these words came from: direct.)

---

## function — toc_summary

Found in src/phext.rs at line 1154.

Surface: public API.

Signature: `pub fn toc_summary(input: &str, prefix: &[usize]) -> TocSummary {`

What it does: The size of a subtree: the number of scrolls under `prefix` (0 to 9 leading address components) and their total byte length. `toc_summary(input, &[])` summarizes the whole document; a full 9-component prefix summarizes a single scroll — 1 scroll if it is populated, 0 if it is not.

(Where these words came from: direct.)

---

## function — toc_scrolls

Found in src/phext.rs at line 1176.

Surface: public API.

Signature: `pub fn toc_scrolls(input: &str, prefix: &[usize]) -> Vec<TocScroll> {`

What it does: Every scroll under `prefix` (0 to 9 leading address components), in document order, as a `TocScroll`: full coordinate plus byte length, no bodies. This is the planet list a progressive client renders at the bottom of a zoom — it asks for the scrolls of one star system, not of the whole phext. A full 9-component prefix yields the single scroll at that coordinate, if populated.

(Where these words came from: direct.)

---

## function — toc_scrolls_by_suffix

Found in src/phext.rs at line 1199.

Surface: public API.

Signature: `pub fn toc_scrolls_by_suffix(input: &str, suffix: &[usize]) -> Vec<TocScroll> {`

What it does: Every scroll whose coordinate ENDS with `suffix` — the trailing 1 to 9 address components — in document order, as `TocScroll` entries (coordinate + bytes, no bodies). The relationship primitive: scrolls that share a chapter.section.scroll suffix are related wherever their library-through-book parents live, and this is the query that finds them. A 9-component suffix is a full coordinate and yields at most one scroll; an empty suffix yields every scroll, as `toc_scrolls(input, &[])` does.

(Where these words came from: direct.)

---

## function — implode

Found in src/phext.rs at line 1217.

Surface: public API.

Signature: `pub fn implode(map: HashMap::<Coordinate, String>) -> String {`

What it does: Serializes a hierarchical hash of scrolls into a monolithic phext document.

(Where these words came from: direct.)

---

## function — merge

Found in src/phext.rs at line 1232.

Surface: public API.

Signature: `pub fn merge(left: &str, right: &str) -> String {`

What it does: combines `left` and `right` into a new phext document, with content merged on a per-scroll basis

(Where these words came from: direct.)

---

## function — fetch

Found in src/phext.rs at line 1276.

Surface: public API.

Signature: `pub fn fetch(input: &str, target: Coordinate) -> String {`

What it does: retrieves the scroll located at the given coordinates. @param phext  the raw phext buffer to search @param coord  coordinate to select the scroll from

(Where these words came from: direct.)

---

## function — expand

Found in src/phext.rs at line 1301.

Surface: public API.

Signature: `pub fn expand(input: &str) -> String {`

What it does: Function expand takes input: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — contract

Found in src/phext.rs at line 1334.

Surface: public API.

Signature: `pub fn contract(input: &str) -> String {`

What it does: Function contract takes input: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — subtract

Found in src/phext.rs at line 1408.

Surface: public API.

Signature: `pub fn subtract(left: &str, right: &str) -> String {`

What it does: Subtracts the scrolls from `left` where there is also content at those coordinates in `right` Note: This makes it easy to generate a mask for a given phext archive.

(Where these words came from: direct.)

---

## function — is_phext_break

Found in src/phext.rs at line 1438.

Surface: public API.

Signature: `pub fn is_phext_break(byte: u8) -> bool {`

What it does: returns true if `byte` corresponds to one of our phext delimiters (line breaks included)

(Where these words came from: direct.)

---

## function — normalize

Found in src/phext.rs at line 1452.

Surface: public API.

Signature: `pub fn normalize(input: &str) -> String {`

What it does: Function normalize takes input: &str. It gives back String.

(Where these words came from: inferred.)

---

## struct — Chain

Found in src/phext.rs at line 1473.

Surface: public API.

Signature: `pub struct Chain {`

What it does: A chain: an ordered series of complete phexts within one stream, joined by CHAIN_BREAK (0x02) and optionally terminated by CHAIN_CLOSE (0x03). The chain layer is a stream-level layer, one rung above the library break: the nine dimension breaks structure a single phext, while the two chain bytes structure a stream of phexts. Each member is a whole phext, worked with the ordinary phext functions in this module; read in succession, a chain's members are editions in sequence — the chain is how a phext stream encodes time. `closed` records how the chain ended: true when a CHAIN_CLOSE terminated it, false when the stream simply ran out (EOF). A chain that ends at EOF is unclosed — possibly truncated — and is never silently promoted to closed. A stream with no chain bytes at all is a single unclosed chain holding one member phext: every pre-0.6 phext reads this way.

(Where these words came from: direct.)

---

## function — Chain::new

Found in src/phext.rs at line 1478.

Surface: public API.

Signature: `pub fn new() -> Chain {`

What it does: Function Chain::new takes no inputs beyond the value it is called on. It gives back Chain.

(Where these words came from: inferred.)

---

## function — Chain::serialize

Found in src/phext.rs at line 1488.

Surface: public API.

Signature: `pub fn serialize(&self) -> String {`

What it does: serializes this chain alone: its members joined by CHAIN_BREAK, with a trailing CHAIN_CLOSE when the chain is closed. See `serialize_chains` for whole streams.

(Where these words came from: direct.)

---

## function — parse_chains

Found in src/phext.rs at line 1511.

Surface: public API.

Signature: `pub fn parse_chains(input: &str) -> Vec<Chain> {`

What it does: splits a stream into its chains. CHAIN_BREAK separates the member phexts of a chain; CHAIN_CLOSE terminates a chain, and whatever follows it opens the next one — so one stream may hold many chains. The stream's final chain is closed only when the stream ends with CHAIN_CLOSE. Splitting follows `split_on` semantics, so empty members and empty chains are preserved exactly: parse and serialize are byte-exact inverses for every stream.

(Where these words came from: direct.)

---

## function — serialize_chains

Found in src/phext.rs at line 1535.

Surface: public API.

Signature: `pub fn serialize_chains(chains: &[Chain]) -> String {`

What it does: serializes a stream of chains back into stream bytes: the inverse of `parse_chains`.

(Where these words came from: direct.)

---

## function — default_coordinate

Found in src/phext.rs at line 1544.

Surface: public API.

Signature: `pub fn default_coordinate() -> Coordinate {`

What it does: Function default_coordinate takes no inputs beyond the value it is called on. It gives back Coordinate.

(Where these words came from: inferred.)

---

## function — to_coordinate

Found in src/phext.rs at line 1572.

Surface: public API.

Signature: `pub fn to_coordinate(address: &str) -> Coordinate {`

What it does: translates a phext string to a strongly-typed address @param address  text to parse

(Where these words came from: direct.)

---

## function — Coordinate::validate_coordinate

Found in src/phext.rs at line 1623.

Surface: public API.

Signature: `pub fn validate_coordinate(&self) -> bool {`

What it does: determines if coord points to a valid phext address @param coord: the coordinate to reset

(Where these words came from: direct.)

---

## function — Coordinate::to_string

Found in src/phext.rs at line 1643.

Surface: public API.

Signature: `pub fn to_string(&self) -> String {`

What it does: produces a quoted string for the given phext address in canonical format (z3.z2.z1/y3.y2.y1/x3.x2.x1) @param coord  the coordinate to translate

(Where these words came from: direct.)

---

## function — Coordinate::to_urlencoded

Found in src/phext.rs at line 1653.

Surface: public API.

Signature: `pub fn to_urlencoded(&self) -> String {`

What it does: Function Coordinate::to_urlencoded takes no inputs beyond the value it is called on. It gives back String.

(Where these words came from: inferred.)

---

## function — Coordinate::scroll_break

Found in src/phext.rs at line 1674.

Surface: public API.

Signature: `pub fn scroll_break(&mut self) {`

What it does: Function Coordinate::scroll_break takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — Coordinate::section_break

Found in src/phext.rs at line 1681.

Surface: public API.

Signature: `pub fn section_break(&mut self) {`

What it does: Function Coordinate::section_break takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — Coordinate::chapter_break

Found in src/phext.rs at line 1689.

Surface: public API.

Signature: `pub fn chapter_break(&mut self) {`

What it does: Function Coordinate::chapter_break takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — Coordinate::book_break

Found in src/phext.rs at line 1698.

Surface: public API.

Signature: `pub fn book_break(&mut self) {`

What it does: Function Coordinate::book_break takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — Coordinate::volume_break

Found in src/phext.rs at line 1708.

Surface: public API.

Signature: `pub fn volume_break(&mut self) {`

What it does: Function Coordinate::volume_break takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — Coordinate::collection_break

Found in src/phext.rs at line 1719.

Surface: public API.

Signature: `pub fn collection_break(&mut self) {`

What it does: Function Coordinate::collection_break takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — Coordinate::series_break

Found in src/phext.rs at line 1731.

Surface: public API.

Signature: `pub fn series_break(&mut self) {`

What it does: Function Coordinate::series_break takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — Coordinate::shelf_break

Found in src/phext.rs at line 1744.

Surface: public API.

Signature: `pub fn shelf_break(&mut self) {`

What it does: Function Coordinate::shelf_break takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — Coordinate::library_break

Found in src/phext.rs at line 1758.

Surface: public API.

Signature: `pub fn library_break(&mut self) {`

What it does: Function Coordinate::library_break takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — parse_universe_address

Found in src/phext.rs at line 1775.

Surface: public API.

Signature: `pub fn parse_universe_address(address: &str) -> Option<(usize, Coordinate)> {`

What it does: Function parse_universe_address takes address: &str. It gives back Option<(usize, Coordinate)>.

(Where these words came from: inferred.)

---

## function — Chain::fetch

Found in src/phext.rs at line 1791.

Surface: public API.

Signature: `pub fn fetch(&self, address: &str) -> String {`

What it does: Fetch a scroll by multiverse address: "$N:coord", or a bare coordinate (universe 1). Out-of-range universes and missing scrolls yield an empty string, as fetch does.

(Where these words came from: direct.)

---

## struct — ScrollPosition

Found in src/phext.rs at line 1825.

Surface: public API.

Signature: `pub struct ScrollPosition {`

What it does: A caret boundary inside one scroll: 1-based line and column (see the section note above).

(Where these words came from: direct.)

---

## struct — PointAddress

Found in src/phext.rs at line 1835.

Surface: public API.

Signature: `pub struct PointAddress {`

What it does: One parsed point address: universe (1-based; 1 when the "$N:" prefix is absent), the 9D coordinate, an optional caret position, and an optional forward offset in characters.

(Where these words came from: direct.)

---

## struct — RangeAddress

Found in src/phext.rs at line 1859.

Surface: public API.

Signature: `pub struct RangeAddress {`

What it does: A parsed range: two point endpoints, normalized to ascending order at parse time.

(Where these words came from: direct.)

---

## function — parse_point_address

Found in src/phext.rs at line 1900.

Surface: public API.

Signature: `pub fn parse_point_address(address: &str) -> Option<PointAddress> {`

What it does: parses a v0.6.1 point address: "[$N:]<coordinate>[@<line>,<column>[+<offsetChars>]]". Returns None on any malformed input: universe 0, a coordinate component below 1, a line or column below 1, a sign anywhere, or trailing structure. "$1:" is accepted on input; the canonical Display form never emits it.

(Where these words came from: direct.)

---

## function — parse_range_address

Found in src/phext.rs at line 1958.

Surface: public API.

Signature: `pub fn parse_range_address(address: &str) -> Option<RangeAddress> {`

What it does: parses a v0.6.1 range address: exactly two point addresses joined by one hyphen. The endpoints are normalized to ascending order (direction is discarded, per the ratified spec). Range endpoints may not carry a "+offset" — the offset form never takes a second endpoint. Returns None on malformed input.

(Where these words came from: direct.)

---

## function — Chain::resolve_point

Found in src/phext.rs at line 2000.

Surface: public API.

Signature: `pub fn resolve_point(&self, point: &PointAddress) -> Option<(usize, Coordinate, usize)> {`

What it does: resolves a point address against this chain: (universe, coordinate, character offset within the scroll). A point without "@" resolves to offset 0 of its scroll. None when the universe or scroll is absent or the position is out of range — never clamps.

(Where these words came from: direct.)

---

## function — Chain::fetch_point

Found in src/phext.rs at line 2018.

Surface: public API.

Signature: `pub fn fetch_point(&self, address: &str) -> String {`

What it does: fetches a point address string. A bare point is a caret: it fetches the empty string. The offset form fetches the selected characters: from the point, forward `offset` characters (scalars) within the scroll, terminating at the scroll boundary. Any parse or resolution failure fetches the empty string, as Chain::fetch does.

(Where these words came from: direct.)

---

## function — Chain::resolve_range

Found in src/phext.rs at line 2035.

Surface: public API.

Signature: `pub fn resolve_range(&self, range: &RangeAddress) -> Option<Vec<(usize, Coordinate, String)>> {`

What it does: resolves a range against this chain into per-scroll segments: (universe, coordinate, segment text) in ascending order. Middle scrolls contribute their whole text; the endpoint scrolls contribute from/to their endpoint boundaries (an endpoint without "@" contributes its whole scroll). None when either endpoint fails to resolve — an out-of-range endpoint is an error, never a clamp.

(Where these words came from: direct.)

---

## function — Chain::fetch_range

Found in src/phext.rs at line 2077.

Surface: public API.

Signature: `pub fn fetch_range(&self, address: &str) -> String {`

What it does: fetches a range address string: the spanned text with phext structure bytes preserved — the covered segments re-assembled at their coordinates into a phext (chain) fragment (scroll breaks between covered scrolls inside a universe, CHAIN_BREAK between universes, never a trailing CHAIN_CLOSE: a fragment is an excerpt, not a terminated chain), so the result round-trips as a phext fragment. Empty segments contribute nothing. Any parse or resolution failure fetches the empty string, as Chain::fetch does.

(Where these words came from: direct.)

---

## function — checked_scroll

Found in src/phext.rs at line 2123.

Surface: public API.

Signature: `pub fn checked_scroll(scroll: &str) -> String {`

What it does: Returns `scroll` with an integrity trailer appended: one final line `phext-check:xxh3-128:<checksum of the payload>`. In a checked scroll the final line is reserved — verification always reads the LAST marker line, and the checksum binds the entire payload before it, so a marker line inside the payload cannot forge a trailer. The checksum detects accidental corruption and torn writes. xxh3 is not a cryptographic hash: this is integrity, not authentication. An empty scroll stays empty (a scroll with no bytes does not exist, so there is nothing to check).

(Where these words came from: direct.)

---

## function — verify_checked_scroll

Found in src/phext.rs at line 2136.

Surface: public API.

Signature: `pub fn verify_checked_scroll(scroll: &str) -> bool {`

What it does: true when `scroll` ends with a well-formed integrity trailer whose checksum matches the payload before it.

(Where these words came from: direct.)

---

## function — uncheck_scroll

Found in src/phext.rs at line 2153.

Surface: public API.

Signature: `pub fn uncheck_scroll(scroll: &str) -> Option<String> {`

What it does: Returns the payload of a checked scroll when its trailer verifies; None for unchecked, malformed, or corrupted scrolls.

(Where these words came from: direct.)

---

## function — fetch_checked

Found in src/phext.rs at line 2174.

Surface: public API.

Signature: `pub fn fetch_checked(input: &str, target: Coordinate) -> Option<String> {`

What it does: Fetches the scroll at `target` and returns its payload, but only when the scroll is checked and verifies. A missing, unchecked, or torn scroll yields None — recovery code can treat None as "not committed". This is a single-scroll lookup: one early-exit scan, linear in the document, with no per-entry repetition inside the call. Callers that verify a whole batch should use `verify_commit`, which indexes the document once for the marker and every entry.

(Where these words came from: direct.)

---

## function — append

Found in src/phext.rs at line 2317.

Surface: public API.

Signature: `pub fn append(input: String, location: Coordinate, scroll: &str) -> Option<String> {`

What it does: Appends `scroll` at `location`, which must sort strictly after the document's last non-empty scroll. The result is the input with only a delimiter run and the new content added at its end — a host can write just that suffix to a file opened in append mode and sync once. Returns None when the scroll is empty (empty scrolls do not exist) or `location` does not sort after the document's end. Trailing delimiter bytes in the input encode only empty scrolls, so they are dropped first and never shift the appended scroll's address. On None the append was rejected; the input String was consumed, so pass a clone if the caller needs it after a rejected append. The document's end is located with `last_scroll_coordinate`, a backward delimiter scan over the bytes — existing text is never re-tokenized, so a call's cost tracks the delimiter run at the end of the document, not the size of the document.

(Where these words came from: direct.)

---

## function — append_checked

Found in src/phext.rs at line 2351.

Surface: public API.

Signature: `pub fn append_checked(input: String, location: Coordinate, scroll: &str) -> Option<String> {`

What it does: `append` with an integrity trailer: the stored scroll is `checked_scroll(scroll)`, verifiable later with `fetch_checked`.

(Where these words came from: direct.)

---

## function — append_batch

Found in src/phext.rs at line 2369.

Surface: public API.

Signature: `pub fn append_batch(input: String, entries: &[(Coordinate, &str)]) -> Option<String> {`

What it does: Appends several scrolls in one call. Entries must be non-empty and in strictly ascending coordinate order, the first sorting after the document's end. The whole batch is a single suffix of the result, so a host persists it with one write and one sync. This is construction atomicity, not storage atomicity: whether the suffix lands completely is the host filesystem's business (see the durability profile in the crate docs). Pair the batch with a `commit_scroll` as its final entry to make a torn batch *detectable* at recovery — that is the part a format library can own.

(Where these words came from: direct.)

---

## function — commit_scroll

Found in src/phext.rs at line 2400.

Surface: public API.

Signature: `pub fn commit_scroll(entries: &[(Coordinate, &str)]) -> String {`

What it does: Builds the commit marker for a batch: a small scroll binding the batch's entry count and a checksum over each entry's coordinate and content, in order. Append it (via `append_batch`) as the batch's final scroll, at a coordinate after every entry. `entries` here are the (coordinate, content) pairs exactly as appended — checked contents included, when the batch uses checked scrolls.

(Where these words came from: direct.)

---

## function — verify_commit

Found in src/phext.rs at line 2425.

Surface: public API.

Signature: `pub fn verify_commit(input: &str, entries: &[Coordinate], commit: Coordinate) -> bool {`

What it does: true when the scroll at `commit` is a commit marker whose count matches `entries` and whose checksum matches the entries' current contents, in order. Any missing, altered, or torn entry — or a missing marker — yields false: the batch did not commit. The document is indexed with a single `explode` pass and the marker and every entry are read from that map. The previous implementation ran the scanning `fetch` once for the marker and once per entry, so verification cost grew quadratically in document length — the cost a WAL recovery pass pays over the whole log, per transaction.

(Where these words came from: direct.)

---

## const — SEAL_PRESET

Found in src/seal.rs at line 22.

Surface: public API.

Signature: `pub const SEAL_PRESET: u32 = 6;`

What it does: The xz preset this implementation seals with.

(Where these words came from: direct.)

---

## function — seal

Found in src/seal.rs at line 29.

Surface: public API.

Signature: `pub fn seal(input: &str) -> io::Result<Vec<u8>> {`

What it does: seals a phext: xz-compresses its full UTF-8 bytes.

(Where these words came from: direct.)

---

## function — seal_bytes

Found in src/seal.rs at line 38.

Surface: public API.

Signature: `pub fn seal_bytes(input: &[u8]) -> io::Result<Vec<u8>> {`

What it does: seals raw phext bytes (see `seal`).

(Where these words came from: direct.)

---

## function — unseal

Found in src/seal.rs at line 50.

Surface: public API.

Signature: `pub fn unseal(data: &[u8]) -> io::Result<Vec<u8>> {`

What it does: unseals a sealed phext back to its exact original bytes. Returns the io error when the data is not a valid xz stream.

(Where these words came from: direct.)

---

## function — unseal_string

Found in src/seal.rs at line 65.

Surface: public API.

Signature: `pub fn unseal_string(data: &[u8]) -> io::Result<String> {`

What it does: unseals directly to text. Fails with InvalidData when the unsealed bytes are not valid UTF-8; to unseal foreign content that may carry non-UTF-8 scrolls, `unseal` to bytes and pass them through `binary::ingest`, which applies the base64-verbatim ingestion rule instead of failing.

(Where these words came from: direct.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/libphext-rs/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

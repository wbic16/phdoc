# teixml-to-mutable — project documentation

Scanned by phdoc 0.1.0 on 2026-11-23 (Exocortical Epoch 195). Project root: /home/wbic16/phdoc-sweep/proj/teixml-to-mutable. Scanned at commit 57706661107e72bc322e02a66bc0972b3d409a54. Package version, from its manifest: 0.3.0. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 9 (9 default, 0 optional).

- libphext [dependencies] — 0.6.0
- quick-xml [dependencies] — 0.38
- sha2 [dependencies] — 0.10
- memmap2 [dependencies] — 0.9
- rusqlite [dependencies] — 0.32
- zip [dependencies] — 2
- rand [dependencies] — 0.8
- zip [dev-dependencies] — 2
- rusqlite [dev-dependencies] — 0.32

Total packages in the lock file, this one included: 60.


---

## Authors

The git log holds 7 commits. phdoc audits the 4 commits on the first-parent mainline — each merge folds its branch's commits into the merge commit — and the author counts below cover that audited mainline, the same set the historical references fold.

The author of record is **Will Bickford** — 4 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 4 commits

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 3 files, 785 lines total, 133 blank, 2 comment, 650 code
- Rust: 13 files, 7221 lines total, 317 blank, 592 comment, 6312 code
- TOML: 1 files, 22 lines total, 2 blank, 0 comment, 20 code

All languages: 17 files, 8028 lines total, 6982 lines of code. Functions read: 210.

Largest files:

- src/phextura.rs — 2980 lines
- src/lib.rs — 1337 lines
- src/mutable.rs — 880 lines
- TEI.md — 553 lines
- tests/phextura.rs — 413 lines

Largest functions, by span in lines:

- build_layout in src/phextura.rs — about 464 lines
- check_reconstruction in src/phextura.rs — about 434 lines
- main in src/main.rs — about 339 lines
- validate_pair in src/phextura.rs — about 239 lines
- export_phextura_opts in src/phextura.rs — about 179 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 35 test functions counted in the source, across 8 files in test directories or whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 48 of 132 (36%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/teixml-to-mutable.git
- Branch: exo
- Head commit: 57706661107e72bc322e02a66bc0972b3d409a54
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- struct `ChainReader` (src/lib.rs:900) — A data shape named ChainReader.
- function `ChainReader::open` (src/lib.rs:907) — Function ChainReader::open takes path: &Path and gives back Result<ChainReader, String>.
- struct `ChainWriter` (src/lib.rs:648) — Streaming chain writer (WAL write path): members are appended in order, previously written bytes are never touched; the chain-close byte is written last.
- function `ChainWriter::create` (src/lib.rs:654) — Function ChainWriter::create takes path: &Path and gives back Result<ChainWriter, String>.
- struct `ConversionSummary` (src/lib.rs:591) — A data shape named ConversionSummary.
- struct `Doctype` (src/lib.rs:88) — A document type declaration, as written (stage 0.3, ruling D3).
- struct `Element` (src/lib.rs:55) — A data shape named Element.
- function `Element::is_container` (src/lib.rs:64) — Function Element::is_container takes no inputs beyond the value it is called on, and gives back bool.
- const `FORMAT_VERSION` (src/lib.rs:21) — A fixed value named FORMAT_VERSION, set to "0.1".
- struct `FileEntry` (src/lib.rs:579) — A data shape named FileEntry.
- struct `IndexEntry` (src/lib.rs:1047) — A data shape named IndexEntry.
- enum `Node` (src/lib.rs:45) — A named choice named Node.
- struct `ParsedDocument` (src/lib.rs:101) — The full parse of one XML document: root element tree plus the prolog and epilog material (comments, PIs) and the DOCTYP
- enum `StorageClass` (src/lib.rs:558) — A named choice named StorageClass.
- function `StorageClass::as_str` (src/lib.rs:563) — Function StorageClass::as_str takes no inputs beyond the value it is called on, and gives back &'static str.
- struct `VerifyReport` (src/lib.rs:1199) — A data shape named VerifyReport.
- function `append_member` (src/lib.rs:676) — Append one member universe to an existing closed chain (WAL append): every previously written member byte is left untouc
- function `classify` (src/lib.rs:571) — Function classify takes bytes: &[u8] and gives back StorageClass.
- function `close` (src/lib.rs:666) — Function close takes mut self and gives back Result<(), String>.
- function `collapse_ws` (src/lib.rs:1310) — Whitespace-collapse: runs become one space, ends trimmed.
- function `collect_xml_files` (src/lib.rs:597) — Function collect_xml_files takes dir: &Path and gives back Result<Vec<PathBuf>, String>.
- function `convert_dir` (src/lib.rs:750) — Function convert_dir takes dir: &Path, out: &Path and gives back Result<ConversionSummary, String>.
- function `coord` (src/lib.rs:37) — Function coord takes scroll: usize and gives back phext::Coordinate.
- function `element_text` (src/lib.rs:1303) — All text of an element's subtree, concatenated in document order.
- function `extract_original` (src/lib.rs:1087) — Extract one original back out of $1, verifying its SHA-256.
- function `extract_title` (src/lib.rs:1316) — Title: teiHeader/fileDesc/titleStmt/title, fallback msIdentifier/idno, fallback the source filename stem (STAGE2-RECON.m
- function `first_named` (src/lib.rs:1288) — First element with this local name in pre-order (self included).
- function `get` (src/lib.rs:1008) — Function get takes &self, universe: usize, coord: &phext::Coordinate and gives back Option<&str>.
- function `member` (src/lib.rs:1004) — Function member takes &self, universe: usize and gives back Option<&str>.
- function `member_count` (src/lib.rs:1001) — Function member_count takes no inputs beyond the value it is called on, and gives back usize.
- function `parse_doctype` (src/lib.rs:285) — Parse the inner text of a `<!DOCTYPE ...>` declaration (everything between `<!DOCTYPE` and the closing `>`).
- function `parse_index` (src/lib.rs:1056) — Function parse_index takes scroll: &str and gives back (Vec<IndexEntry>, Vec<(String, String)>).
- function `parse_subspace` (src/lib.rs:1036) — Parse a `$N:1.1.1/1.1.1/1.1.1` subspace address.
- function `parse_xml` (src/lib.rs:376) — Parse one XML document (bytes) into an Element tree.
- function `parse_xml_full` (src/lib.rs:381) — Parse one XML document into root tree + prolog/epilog + DOCTYPE.
- function `rel_of` (src/lib.rs:623) — Function rel_of takes base: &Path, p: &Path and gives back String.
- function `renumber_refs` (src/lib.rs:827) — Rewrite every `@ref $k` scroll in a member from local to global numbering by adding `offset` (the file's first-universe number − 1).
- function `scroll_number` (src/lib.rs:1027) — Function scroll_number takes c: &phext::Coordinate and gives back usize.
- function `scrolls_of` (src/lib.rs:1013) — Scrolls of a universe as (scroll number, content), in scroll order.
- function `sha256_hex` (src/lib.rs:28) — Function sha256_hex takes bytes: &[u8] and gives back String.
- function `text_nodes` (src/lib.rs:69) — All text nodes of this element's subtree, in document order.
- function `universe_text_and_refs` (src/lib.rs:1135) — Walk one converted universe subtree: collect (text nodes, refs followed).
- function `verify` (src/lib.rs:1210) — Function verify takes chain: &Path, corpus_dir: Option<&Path> and gives back Result<VerifyReport, String>.
- function `write_member` (src/lib.rs:658) — Function write_member takes &mut self, member: &str and gives back Result<(), String>.
- flag `--corpus` (src/main.rs:232) — A command line flag, --corpus.
- flag `--no-vfs` (src/main.rs:122) — A command line flag, --no-vfs.
- command `convert` (src/main.rs:35) — A command named convert.
- command `extract` (src/main.rs:164) — A command named extract.
- entry point `main` (src/main.rs:29) — The entry point: the program starts here.
- command `mutable` (src/main.rs:74) — A command named mutable.
- command `phextura` (src/main.rs:113) — A command named phextura.
- command `relocate-phextura` (src/main.rs:331) — A command named relocate-phextura.
- command `show` (src/main.rs:198) — A command named show.
- command `verify` (src/main.rs:227) — A command named verify.
- command `verify-phextura` (src/main.rs:265) — A command named verify-phextura.
- const `IMPORT_GENERATION` (src/mutable.rs:528) — Deterministic generation id for the import save (matches Mutable's /^[a-zA-Z0-9-]+$/ generation rule).
- enum `Json` (src/mutable.rs:31) — A named choice named Json.
- function `Json::to_pretty` (src/mutable.rs:66) — JSON.stringify(value, null, 2): objects/arrays expand one item per line; empty ones stay inline as {} / [].
- struct `MutableDocument` (src/mutable.rs:244) — One converted Mutable document, ready to write into the Cavern tree.
- struct `MutableEntry` (src/mutable.rs:573) — A data shape named MutableEntry.
- struct `MutableSummary` (src/mutable.rs:584) — A data shape named MutableSummary.
- function `build_document` (src/mutable.rs:258) — Build the Mutable document for one parsed TEI file (0.1 model element).
- function `document_rel_for` (src/mutable.rs:237) — The document filename for a source relative path: `MS.xml` -> `MS.mutable.json`, folders preserved.
- function `export_mutable` (src/mutable.rs:615) — Export a directory of TEI-XML as a Mutable native Cavern zip.
- function `resource_id_for` (src/mutable.rs:181) — Deterministic resourceId: a UUID-shaped digest of the source relative path (SHA-256, version/variant bits set) — stable 
- function `to_compact` (src/mutable.rs:116) — JSON.stringify(value): compact, key order as constructed.
- struct `C9` (src/phextura.rs:66) — A full 9-component phext coordinate (Z/Y/X), the address currency of this module; converted to libphext coordinates only
- function `C9::new` (src/phextura.rs:73) — Function C9::new takes z: and gives back C9.
- const `CONVERTER` (src/phextura.rs:48) — A fixed value named CONVERTER, set to concat!("teixml-to-mutable ", env!("CARGO_PKG_VERSION")).
- struct `Detached` (src/phextura.rs:2793) — A data shape named Detached.
- struct `EntryRec` (src/phextura.rs:980) — A data shape named EntryRec.
- const `KIND_ORDER` (src/phextura.rs:52) — L2 key kinds, in object-table index order (LAYER2 §1.4 + profile extension kind `tei-element`, minted on cite only).
- const `POINTING_ATTRS` (src/phextura.rs:55) — Attributes whose values are treated as references (SPEC-0.3 §3.5).
- const `PROFILE_VERSION` (src/phextura.rs:46) — A fixed value named PROFILE_VERSION, set to "0.1".
- struct `ParsedEntry` (src/phextura.rs:1927) — A data shape named ParsedEntry.
- struct `PhexturaFileEntry` (src/phextura.rs:1510) — A data shape named PhexturaFileEntry.
- struct `PhexturaSummary` (src/phextura.rs:1519) — A data shape named PhexturaSummary.
- struct `ReconReport` (src/phextura.rs:2282) — A data shape named ReconReport.
- struct `RefCounts` (src/phextura.rs:997) — A data shape named RefCounts.
- enum `RefOut` (src/phextura.rs:964) — A named choice named RefOut.
- struct `RelocReport` (src/phextura.rs:2800) — A data shape named RelocReport.
- const `SECTION_ELEMENTS` (src/phextura.rs:359) — Elements that head a LAYER2 `section` object when headed (profile §3).
- struct `ScrollParts` (src/phextura.rs:1746) — A scroll split into role line, header lines, and body (text after the first blank line; header-only scrolls have an empt
- const `TEI_NS` (src/phextura.rs:47) — A fixed value named TEI_NS, set to "http://www.tei-c.org/ns/1.0".
- struct `TargetAddr` (src/phextura.rs:1887) — A data shape named TargetAddr.
- enum `TargetForm` (src/phextura.rs:971) — A named choice named TargetForm.
- struct `ValidateReport` (src/phextura.rs:2029) — A data shape named ValidateReport.
- function `addr` (src/phextura.rs:76) — Function addr takes no inputs beyond the value it is called on, and gives back String.
- function `canonical_bytes` (src/phextura.rs:272) — Function canonical_bytes takes map: &HashMap<phext::Coordinate, String> and gives back Vec<u8>.
- function `check_reconstruction` (src/phextura.rs:2375) — Function check_reconstruction takes base_path: &Path, standoff_path: &Path, corpus_dir: &Path, and gives back Result<Rec
- function `edition_checksum` (src/phextura.rs:264) — The edition checksum: libphext xxh3-128 over the canonical form of the corpus with the checksum line excluded from its m
- function `escape_value` (src/phextura.rs:129) — Function escape_value takes v: &str and gives back String.
- function `export_phextura` (src/phextura.rs:1566) — Function export_phextura takes dir: &Path, out_dir: &Path and gives back Result<PhexturaSummary, String>.
- function `export_phextura_opts` (src/phextura.rs:1573) — As `export_phextura`; `emit_vfs` controls the D6 PhextVFS image (`cavern.phext`), a third consumer of the assembled pair
- function `iso_from_unix` (src/phextura.rs:227) — Function iso_from_unix takes secs: u64 and gives back String.
- function `iso_now` (src/phextura.rs:219) — Function iso_now takes no inputs beyond the value it is called on, and gives back String.
- function `line_of` (src/phextura.rs:1778) — Function line_of takes lines: &'a [ and gives back Option<&'a str>.
- function `lines_of` (src/phextura.rs:1782) — Function lines_of takes lines: &'a [ and gives back Vec<&'a str>.
- function `load_entries` (src/phextura.rs:2009) — All standoff entries, sorted into minting order (series, then scroll).
- function `load_map` (src/phextura.rs:1786) — Function load_map takes path: &Path and gives back Result<HashMap<C9, String>, String>.
- function `manifest_coord` (src/phextura.rs:115) — The corpus-level manifest coordinate (Series 1, Collection 6).
- function `parse` (src/phextura.rs:86) — Function parse takes s: &str and gives back Option<C9>.
- function `parse_entry` (src/phextura.rs:1946) — Function parse_entry takes coord: C9, text: &str and gives back Result<ParsedEntry, String>.
- function `parse_target` (src/phextura.rs:1895) — Function parse_target takes addr: &str and gives back Result<TargetAddr, String>.
- function `phext` (src/phextura.rs:83) — Function phext takes no inputs beyond the value it is called on, and gives back phext::Coordinate.
- function `relocate` (src/phextura.rs:2835) — Function relocate takes base_path: &Path, standoff_path: &Path and gives back Result<RelocReport, String>.
- function `render_attr_name` (src/phextura.rs:1032) — Render an attribute's qualified name for an `attr-` header line: namespaced names render as `{<namespace-uri>}<local>` (profile §4); the prefix resolves against the xmlns declarations in scope, with `xml` predefined.
- function `slugify` (src/phextura.rs:169) — Function slugify takes s: &str and gives back String.
- function `split_scroll` (src/phextura.rs:1752) — Function split_scroll takes text: &str and gives back ScrollParts.
- function `standoff_meta_coord` (src/phextura.rs:119) — The standoff's own metadata coordinate (its Collection 6, Series 1).
- function `strip_checksum_line` (src/phextura.rs:255) — Remove any `checksum: xxh3-128:` line from a scroll's text.
- function `unescape_value` (src/phextura.rs:145) — Function unescape_value takes v: &str and gives back Result<String, String>.
- function `validate_pair` (src/phextura.rs:2073) — Function validate_pair takes base_path: &Path, standoff_path: &Path and gives back Result<ValidateReport, String>.
- function `<'a> Flat<'a>::index_of` (src/segment.rs:67) — Function <'a> Flat<'a>::index_of takes &self, e: &Element and gives back usize.
- const `BLOCK_ELEMENTS` (src/segment.rs:28) — TEI block-level elements: each owns text segments; any other element containing a block-level descendant acts as a conta
- struct `Flat` (src/segment.rs:55) — A pre-order flattening of one document tree: occurrence indices, parent/depth, subtree ranges, and per-element descendan
- struct `InlineOcc` (src/segment.rs:121) — One inline element occurrence inside a segment.
- struct `SegMark` (src/segment.rs:136) — A comment or PI encountered during the walk (ruling D3).
- struct `Segment` (src/segment.rs:152) — One raw segment (pre-keep-rule).
- struct `Segmentation` (src/segment.rs:163) — A data shape named Segmentation.
- function `flatten` (src/segment.rs:78) — Function flatten takes root: &Element and gives back Flat<'_>.
- function `has_block_descendant` (src/segment.rs:46) — Function has_block_descendant takes el: &Element and gives back bool.
- function `is_block` (src/segment.rs:42) — Function is_block takes name: &str and gives back bool.
- function `is_empty` (src/segment.rs:73) — Function is_empty takes no inputs beyond the value it is called on, and gives back bool.
- function `kept_map` (src/segment.rs:286) — The keep rule (0.2, recon §4.4): a segment survives iff its text is non-whitespace or it carries at least one inline occurrence.
- function `len` (src/segment.rs:70) — Function len takes no inputs beyond the value it is called on, and gives back usize.
- function `segment_document` (src/segment.rs:276) — Segment one document.
- const `VFS_CHUNK_SIZE` (src/vfs.rs:41) — Mutable's DEFAULT_CHUNK_SIZE (vfs.ts).
- struct `VfsImageInfo` (src/vfs.rs:52) — A data shape named VfsImageInfo.
- function `base64_encode` (src/vfs.rs:86) — Standard base64 (RFC 4648, padded) — the encoding Mutable's `Buffer.toString("base64")` produces and its reader decodes 
- function `build_cavern_image` (src/vfs.rs:133) — Build the `cavern.phext` image holding `files` (name, bytes) in the given order at the documents-stratum root, in the VFS binary format.
- function `json_escape` (src/vfs.rs:112) — JSON string escaping with JSON.stringify semantics (the index and manifest scrolls are consumed by Mutable's `JSON.parse

---

## struct — ChainReader

Found in src/lib.rs at line 900.

Signature: `pub struct ChainReader {`

What it does: A data shape named ChainReader. The scanner did not read its fields.

(Where these words came from: signature.)

---

## function — ChainReader::open

Found in src/lib.rs at line 907.

Signature: `pub fn open(path: &Path) -> Result<ChainReader, String> {`

What it does: Function ChainReader::open takes path: &Path and gives back Result<ChainReader, String>.

(Where these words came from: signature.)

---

## struct — ChainWriter

Found in src/lib.rs at line 648.

Signature: `pub struct ChainWriter {`

What it does: Streaming chain writer (WAL write path): members are appended in order, previously written bytes are never touched; the chain-close byte is written last. Byte layout is identical to libphext `Chain::serialize` (verified by a test).

(Where these words came from: doc comment.)

---

## function — ChainWriter::create

Found in src/lib.rs at line 654.

Signature: `pub fn create(path: &Path) -> Result<ChainWriter, String> {`

What it does: Function ChainWriter::create takes path: &Path and gives back Result<ChainWriter, String>.

(Where these words came from: signature.)

---

## struct — ConversionSummary

Found in src/lib.rs at line 591.

Signature: `pub struct ConversionSummary {`

What it does: A data shape named ConversionSummary. Its fields are: pub entries: Vec<FileEntry>; pub members_written: usize; pub out_bytes: u64.

(Where these words came from: signature.)

---

## struct — Doctype

Found in src/lib.rs at line 88.

Signature: `pub struct Doctype {`

What it does: A document type declaration, as written (stage 0.3, ruling D3).

(Where these words came from: doc comment.)

---

## struct — Element

Found in src/lib.rs at line 55.

Signature: `pub struct Element {`

What it does: A data shape named Element. Its fields are: pub ns: String; pub name: String; pub attrs: Vec<(String, String)>; pub children: Vec<Node>.

(Where these words came from: signature.)

---

## function — Element::is_container

Found in src/lib.rs at line 64.

Signature: `pub fn is_container(&self) -> bool {`

What it does: Function Element::is_container takes no inputs beyond the value it is called on, and gives back bool.

(Where these words came from: signature.)

---

## const — FORMAT_VERSION

Found in src/lib.rs at line 21.

Signature: `pub const FORMAT_VERSION: &str = "0.1";`

What it does: A fixed value named FORMAT_VERSION, set to "0.1".

(Where these words came from: signature.)

---

## struct — FileEntry

Found in src/lib.rs at line 579.

Signature: `pub struct FileEntry {`

What it does: A data shape named FileEntry. Its fields are: pub rel: String; pub sha256: String; pub size: usize; pub class: StorageClass; pub original_scroll: usize; pub first_universe: Option<usize>; pub error: Option<String>.

(Where these words came from: signature.)

---

## struct — IndexEntry

Found in src/lib.rs at line 1047.

Signature: `pub struct IndexEntry {`

What it does: A data shape named IndexEntry. Its fields are: pub rel: String; pub sha256: String; pub size: usize; pub class: String; pub coord: phext::Coordinate; pub universe: Option<usize>.

(Where these words came from: signature.)

---

## enum — Node

Found in src/lib.rs at line 45.

Signature: `pub enum Node {`

What it does: A named choice named Node. It is one of: Text; Elem; Comment; Pi.

(Where these words came from: signature.)

---

## struct — ParsedDocument

Found in src/lib.rs at line 101.

Signature: `pub struct ParsedDocument {`

What it does: The full parse of one XML document: root element tree plus the prolog and epilog material (comments, PIs) and the DOCTYPE, which live outside the root element but are part of the document (stage 0.3, ruling D3).

(Where these words came from: doc comment.)

---

## enum — StorageClass

Found in src/lib.rs at line 558.

Signature: `pub enum StorageClass {`

What it does: A named choice named StorageClass. It is one of: Text; Binary.

(Where these words came from: signature.)

---

## function — StorageClass::as_str

Found in src/lib.rs at line 563.

Signature: `pub fn as_str(&self) -> &'static str {`

What it does: Function StorageClass::as_str takes no inputs beyond the value it is called on, and gives back &'static str.

(Where these words came from: signature.)

---

## struct — VerifyReport

Found in src/lib.rs at line 1199.

Signature: `pub struct VerifyReport {`

What it does: A data shape named VerifyReport. Its fields are: pub members: usize; pub files: usize; pub failures: usize; pub originals_verified: usize; pub refs_total: usize; pub dangling_refs: Vec<usize>; pub unreachable_universes: Vec<usize>; pub text_mismatches: Vec<String>.

(Where these words came from: signature.)

---

## function — append_member

Found in src/lib.rs at line 676.

Signature: `pub fn append_member(path: &Path, member: &str) -> Result<(), String> {`

What it does: Append one member universe to an existing closed chain (WAL append): every previously written member byte is left untouched; only the old chain-close byte is overwritten by the separator+member+close sequence.

(Where these words came from: doc comment.)

---

## function — classify

Found in src/lib.rs at line 571.

Signature: `pub fn classify(bytes: &[u8]) -> StorageClass {`

What it does: Function classify takes bytes: &[u8] and gives back StorageClass.

(Where these words came from: signature.)

---

## function — close

Found in src/lib.rs at line 666.

Signature: `pub fn close(mut self) -> Result<(), String> {`

What it does: Function close takes mut self and gives back Result<(), String>.

(Where these words came from: signature.)

---

## function — collapse_ws

Found in src/lib.rs at line 1310.

Signature: `pub fn collapse_ws(text: &str) -> String {`

What it does: Whitespace-collapse: runs become one space, ends trimmed.

(Where these words came from: doc comment.)

---

## function — collect_xml_files

Found in src/lib.rs at line 597.

Signature: `pub fn collect_xml_files(dir: &Path) -> Result<Vec<PathBuf>, String> {`

What it does: Function collect_xml_files takes dir: &Path and gives back Result<Vec<PathBuf>, String>.

(Where these words came from: signature.)

---

## function — convert_dir

Found in src/lib.rs at line 750.

Signature: `pub fn convert_dir(dir: &Path, out: &Path) -> Result<ConversionSummary, String> {`

What it does: Function convert_dir takes dir: &Path, out: &Path and gives back Result<ConversionSummary, String>.

(Where these words came from: signature.)

---

## function — coord

Found in src/lib.rs at line 37.

Signature: `pub fn coord(scroll: usize) -> phext::Coordinate {`

What it does: Function coord takes scroll: usize and gives back phext::Coordinate.

(Where these words came from: signature.)

---

## function — element_text

Found in src/lib.rs at line 1303.

Signature: `pub fn element_text(el: &Element) -> String {`

What it does: All text of an element's subtree, concatenated in document order.

(Where these words came from: doc comment.)

---

## function — extract_original

Found in src/lib.rs at line 1087.

Signature: `pub fn extract_original(reader: &ChainReader, rel: &str) -> Result<Vec<u8>, String> {`

What it does: Extract one original back out of $1, verifying its SHA-256.

(Where these words came from: doc comment.)

---

## function — extract_title

Found in src/lib.rs at line 1316.

Signature: `pub fn extract_title(root: &Element, rel: &str) -> String {`

What it does: Title: teiHeader/fileDesc/titleStmt/title, fallback msIdentifier/idno, fallback the source filename stem (STAGE2-RECON.md §4.3).

(Where these words came from: doc comment.)

---

## function — first_named

Found in src/lib.rs at line 1288.

Signature: `pub fn first_named<'a>(el: &'a Element, name: &str) -> Option<&'a Element> {`

What it does: First element with this local name in pre-order (self included).

(Where these words came from: doc comment.)

---

## function — get

Found in src/lib.rs at line 1008.

Signature: `pub fn get(&self, universe: usize, coord: &phext::Coordinate) -> Option<&str> {`

What it does: Function get takes &self, universe: usize, coord: &phext::Coordinate and gives back Option<&str>.

(Where these words came from: signature.)

---

## function — member

Found in src/lib.rs at line 1004.

Signature: `pub fn member(&self, universe: usize) -> Option<&str> {`

What it does: Function member takes &self, universe: usize and gives back Option<&str>.

(Where these words came from: signature.)

---

## function — member_count

Found in src/lib.rs at line 1001.

Signature: `pub fn member_count(&self) -> usize {`

What it does: Function member_count takes no inputs beyond the value it is called on, and gives back usize.

(Where these words came from: signature.)

---

## function — parse_doctype

Found in src/lib.rs at line 285.

Signature: `pub fn parse_doctype(raw: &str) -> Doctype {`

What it does: Parse the inner text of a `<!DOCTYPE ...>` declaration (everything between `<!DOCTYPE` and the closing `>`).

(Where these words came from: doc comment.)

---

## function — parse_index

Found in src/lib.rs at line 1056.

Signature: `pub fn parse_index(scroll: &str) -> (Vec<IndexEntry>, Vec<(String, String)>) {`

What it does: Function parse_index takes scroll: &str and gives back (Vec<IndexEntry>, Vec<(String, String)>).

(Where these words came from: signature.)

---

## function — parse_subspace

Found in src/lib.rs at line 1036.

Signature: `pub fn parse_subspace(addr: &str) -> Result<(usize, phext::Coordinate), String> {`

What it does: Parse a `$N:1.1.1/1.1.1/1.1.1` subspace address.

(Where these words came from: doc comment.)

---

## function — parse_xml

Found in src/lib.rs at line 376.

Signature: `pub fn parse_xml(bytes: &[u8]) -> Result<Element, String> {`

What it does: Parse one XML document (bytes) into an Element tree. Namespace URIs are resolved with a prefix-scope stack; attributes keep their qualified names as written. Since stage 0.3 the tree also carries comments and PIs as nodes; the 0.1/0.2 emitters skip them (their forms never represented comments/PIs — the original is preserved verbatim).

(Where these words came from: doc comment.)

---

## function — parse_xml_full

Found in src/lib.rs at line 381.

Signature: `pub fn parse_xml_full(bytes: &[u8]) -> Result<ParsedDocument, String> {`

What it does: Parse one XML document into root tree + prolog/epilog + DOCTYPE.

(Where these words came from: doc comment.)

---

## function — rel_of

Found in src/lib.rs at line 623.

Signature: `pub fn rel_of(base: &Path, p: &Path) -> String {`

What it does: Function rel_of takes base: &Path, p: &Path and gives back String.

(Where these words came from: signature.)

---

## function — renumber_refs

Found in src/lib.rs at line 827.

Signature: `pub fn renumber_refs(member: &str, offset: usize) -> String {`

What it does: Rewrite every `@ref $k` scroll in a member from local to global numbering by adding `offset` (the file's first-universe number − 1). Goes through libphext explode/implode — no delimiter handling here.

(Where these words came from: doc comment.)

---

## function — scroll_number

Found in src/lib.rs at line 1027.

Signature: `pub fn scroll_number(c: &phext::Coordinate) -> usize {`

What it does: Function scroll_number takes c: &phext::Coordinate and gives back usize.

(Where these words came from: signature.)

---

## function — scrolls_of

Found in src/lib.rs at line 1013.

Signature: `pub fn scrolls_of(&self, universe: usize) -> Vec<(usize, String)> {`

What it does: Scrolls of a universe as (scroll number, content), in scroll order.

(Where these words came from: doc comment.)

---

## function — sha256_hex

Found in src/lib.rs at line 28.

Signature: `pub fn sha256_hex(bytes: &[u8]) -> String {`

What it does: Function sha256_hex takes bytes: &[u8] and gives back String.

(Where these words came from: signature.)

---

## function — text_nodes

Found in src/lib.rs at line 69.

Signature: `pub fn text_nodes(&self, out: &mut Vec<String>) {`

What it does: All text nodes of this element's subtree, in document order. Comments and PIs are not text nodes and never appear here.

(Where these words came from: doc comment.)

---

## function — universe_text_and_refs

Found in src/lib.rs at line 1135.

Signature: `pub fn universe_text_and_refs(reader: &ChainReader, root_universe: usize) -> (Vec<String>, Vec<usize>) {`

What it does: Walk one converted universe subtree: collect (text nodes, refs followed). Mirrors the encoder's grammar exactly.

(Where these words came from: doc comment.)

---

## function — verify

Found in src/lib.rs at line 1210.

Signature: `pub fn verify(chain: &Path, corpus_dir: Option<&Path>) -> Result<VerifyReport, String> {`

What it does: Function verify takes chain: &Path, corpus_dir: Option<&Path> and gives back Result<VerifyReport, String>.

(Where these words came from: signature.)

---

## function — write_member

Found in src/lib.rs at line 658.

Signature: `pub fn write_member(&mut self, member: &str) -> Result<(), String> {`

What it does: Function write_member takes &mut self, member: &str and gives back Result<(), String>.

(Where these words came from: signature.)

---

## flag — --corpus

Found in src/main.rs at line 232.

Signature: `flag --corpus`

What it does: A command line flag, --corpus. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## flag — --no-vfs

Found in src/main.rs at line 122.

Signature: `flag --no-vfs`

What it does: A command line flag, --no-vfs. It is read by the program when the user passes it.

(Where these words came from: signature.)

---

## command — convert

Found in src/main.rs at line 35.

Signature: `command "convert"`

What it does: A command named convert. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — extract

Found in src/main.rs at line 164.

Signature: `command "extract"`

What it does: A command named extract. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## entry point — main

Found in src/main.rs at line 29.

Signature: `fn main() -> ExitCode {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: signature.)

---

## command — mutable

Found in src/main.rs at line 74.

Signature: `command "mutable"`

What it does: A command named mutable. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — phextura

Found in src/main.rs at line 113.

Signature: `command "phextura"`

What it does: A command named phextura. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — relocate-phextura

Found in src/main.rs at line 331.

Signature: `command "relocate-phextura"`

What it does: A command named relocate-phextura. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — show

Found in src/main.rs at line 198.

Signature: `command "show"`

What it does: A command named show. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — verify

Found in src/main.rs at line 227.

Signature: `command "verify"`

What it does: A command named verify. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — verify-phextura

Found in src/main.rs at line 265.

Signature: `command "verify-phextura"`

What it does: A command named verify-phextura. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## const — IMPORT_GENERATION

Found in src/mutable.rs at line 528.

Signature: `pub const IMPORT_GENERATION: &str = "tei-import-1";`

What it does: Deterministic generation id for the import save (matches Mutable's /^[a-zA-Z0-9-]+$/ generation rule).

(Where these words came from: doc comment.)

---

## enum — Json

Found in src/mutable.rs at line 31.

Signature: `pub enum Json {`

What it does: A named choice named Json. It is one of: Null; Bool; Int; Str; Arr; Obj.

(Where these words came from: signature.)

---

## function — Json::to_pretty

Found in src/mutable.rs at line 66.

Signature: `pub fn to_pretty(&self) -> String {`

What it does: JSON.stringify(value, null, 2): objects/arrays expand one item per line; empty ones stay inline as {} / [].

(Where these words came from: doc comment.)

---

## struct — MutableDocument

Found in src/mutable.rs at line 244.

Signature: `pub struct MutableDocument {`

What it does: One converted Mutable document, ready to write into the Cavern tree.

(Where these words came from: doc comment.)

---

## struct — MutableEntry

Found in src/mutable.rs at line 573.

Signature: `pub struct MutableEntry {`

What it does: A data shape named MutableEntry. Its fields are: pub rel: String; pub error: Option<String>; pub doc_rel: Option<String>; pub resource_id: Option<String>; pub title: Option<String>; pub blocks: usize; pub annotations: usize.

(Where these words came from: signature.)

---

## struct — MutableSummary

Found in src/mutable.rs at line 584.

Signature: `pub struct MutableSummary {`

What it does: A data shape named MutableSummary. Its fields are: pub entries: Vec<MutableEntry>; pub vault_guid: String; pub documents: usize; pub out_bytes: u64.

(Where these words came from: signature.)

---

## function — build_document

Found in src/mutable.rs at line 258.

Signature: `pub fn build_document(rel: &str, root: &Element) -> MutableDocument {`

What it does: Build the Mutable document for one parsed TEI file (0.1 model element).

(Where these words came from: doc comment.)

---

## function — document_rel_for

Found in src/mutable.rs at line 237.

Signature: `pub fn document_rel_for(rel: &str) -> String {`

What it does: The document filename for a source relative path: `MS.xml` -> `MS.mutable.json`, folders preserved.

(Where these words came from: doc comment.)

---

## function — export_mutable

Found in src/mutable.rs at line 615.

Signature: `pub fn export_mutable(dir: &Path, out: &Path) -> Result<MutableSummary, String> {`

What it does: Export a directory of TEI-XML as a Mutable native Cavern zip. Every source file is accounted for: converted (a document), or named in the returned entries with its error — zero silent skips, as in 0.1. The original XML rides in the tree verbatim either way.

(Where these words came from: doc comment.)

---

## function — resource_id_for

Found in src/mutable.rs at line 181.

Signature: `pub fn resource_id_for(rel: &str) -> String {`

What it does: Deterministic resourceId: a UUID-shaped digest of the source relative path (SHA-256, version/variant bits set) — stable across runs, unique per tree slot.

(Where these words came from: doc comment.)

---

## function — to_compact

Found in src/mutable.rs at line 116.

Signature: `pub fn to_compact(&self) -> String {`

What it does: JSON.stringify(value): compact, key order as constructed.

(Where these words came from: doc comment.)

---

## struct — C9

Found in src/phextura.rs at line 66.

Signature: `pub struct C9 {`

What it does: A full 9-component phext coordinate (Z/Y/X), the address currency of this module; converted to libphext coordinates only at the implode boundary.

(Where these words came from: doc comment.)

---

## function — C9::new

Found in src/phextura.rs at line 73.

Signature: `pub fn new(z: (u32, u32, u32), y: (u32, u32, u32), x: (u32, u32, u32)) -> C9 {`

What it does: Function C9::new takes z: and gives back C9.

(Where these words came from: signature.)

---

## const — CONVERTER

Found in src/phextura.rs at line 48.

Signature: `pub const CONVERTER: &str = concat!("teixml-to-mutable ", env!("CARGO_PKG_VERSION"));`

What it does: A fixed value named CONVERTER, set to concat!("teixml-to-mutable ", env!("CARGO_PKG_VERSION")).

(Where these words came from: signature.)

---

## struct — Detached

Found in src/phextura.rs at line 2793.

Signature: `pub struct Detached {`

What it does: A data shape named Detached. Its fields are: pub key: String; pub class: String, // span-edited | span-absent | key-retired; pub quoted: String.

(Where these words came from: signature.)

---

## struct — EntryRec

Found in src/phextura.rs at line 980.

Signature: `pub struct EntryRec {`

What it does: A data shape named EntryRec. Its fields are: pub kind: &'static str, // tei-element | tei-comment | tei-pi; pub form: TargetForm; pub hash: String; pub text: String; pub before: Option<String>; pub after: Option<String>; pub tei_element: Option<String>; pub tei_ns: Option<String>; pub pi_target: Option<String>; pub attrs: Vec<(String, String)>; pub refs: Vec<RefOut>; pub body: String.

(Where these words came from: signature.)

---

## const — KIND_ORDER

Found in src/phextura.rs at line 52.

Signature: `pub const KIND_ORDER: &[&str] = &["paper", "section", "bibentry", "figure", "tei-element"];`

What it does: L2 key kinds, in object-table index order (LAYER2 §1.4 + profile extension kind `tei-element`, minted on cite only).

(Where these words came from: doc comment.)

---

## const — POINTING_ATTRS

Found in src/phextura.rs at line 55.

Signature: `pub const POINTING_ATTRS: &[&str] = &[ "ref", "target", "corresp", "decls", "sameAs", "copyOf", "next", "prev", "ana", ];`

What it does: Attributes whose values are treated as references (SPEC-0.3 §3.5).

(Where these words came from: doc comment.)

---

## const — PROFILE_VERSION

Found in src/phextura.rs at line 46.

Signature: `pub const PROFILE_VERSION: &str = "0.1";`

What it does: A fixed value named PROFILE_VERSION, set to "0.1".

(Where these words came from: signature.)

---

## struct — ParsedEntry

Found in src/phextura.rs at line 1927.

Signature: `pub struct ParsedEntry {`

What it does: A data shape named ParsedEntry. Its fields are: pub coord: C9; pub kind: String; pub key_n: usize; pub target: TargetAddr; pub hash: String; pub text: String; pub before: Option<String>; pub after: Option<String>; pub tei_element: Option<String>; pub tei_ns: Option<String>; pub pi_target: Option<String>; pub attrs: Vec<(String, String)>; pub refers: Vec<String>; pub refers_unresolved: Vec<String>; pub refers_external: Vec<String>; pub body: String.

(Where these words came from: signature.)

---

## struct — PhexturaFileEntry

Found in src/phextura.rs at line 1510.

Signature: `pub struct PhexturaFileEntry {`

What it does: A data shape named PhexturaFileEntry. Its fields are: pub rel: String; pub series: u32; pub error: Option<String>; pub entries: usize; pub scrolls: usize.

(Where these words came from: signature.)

---

## struct — PhexturaSummary

Found in src/phextura.rs at line 1519.

Signature: `pub struct PhexturaSummary {`

What it does: A data shape named PhexturaSummary. Its fields are: pub name: String; pub slug: String; pub entries: Vec<PhexturaFileEntry>; pub documents: usize; pub base_path: PathBuf; pub standoff_path: PathBuf; pub base_bytes: u64; pub standoff_bytes: u64; pub base_scrolls: usize; pub standoff_scrolls: usize; pub total_entries: usize; pub entries_by_kind: Vec<(String, usize)>; pub refs: RefCounts; pub externals: usize; pub pin: String; pub vfs_path: Option<PathBuf>; pub vfs_bytes: u64; pub vfs_scrolls: usize.

(Where these words came from: signature.)

---

## struct — ReconReport

Found in src/phextura.rs at line 2282.

Signature: `pub struct ReconReport {`

What it does: A data shape named ReconReport. Its fields are: pub ok: bool; pub mismatches: Vec<String>; pub mismatch_total: usize; pub files: usize; pub originals_verified: usize; pub segments: usize; pub elements: usize; pub attributes: usize; pub comments: usize; pub pis: usize; pub doctypes: usize; pub refs_resolved: usize; pub refs_unresolved: usize; pub refs_external: usize.

(Where these words came from: signature.)

---

## struct — RefCounts

Found in src/phextura.rs at line 997.

Signature: `pub struct RefCounts {`

What it does: A data shape named RefCounts. Its fields are: pub resolved: usize; pub unresolved: usize; pub external: usize.

(Where these words came from: signature.)

---

## enum — RefOut

Found in src/phextura.rs at line 964.

Signature: `pub enum RefOut {`

What it does: A named choice named RefOut. It is one of: Key; Unresolved; External.

(Where these words came from: signature.)

---

## struct — RelocReport

Found in src/phextura.rs at line 2800.

Signature: `pub struct RelocReport {`

What it does: A data shape named RelocReport. Its fields are: pub total: usize; pub exact: usize; pub relocated_verified: usize; pub relocated_moved: usize; pub lost: Vec<Detached>; pub ambiguous: Vec<Detached>.

(Where these words came from: signature.)

---

## const — SECTION_ELEMENTS

Found in src/phextura.rs at line 359.

Signature: `pub const SECTION_ELEMENTS: &[&str] = &[ "div", "div1", "div2", "div3", "div4", "div5", "div6", "div7", "msItem", ];`

What it does: Elements that head a LAYER2 `section` object when headed (profile §3).

(Where these words came from: doc comment.)

---

## struct — ScrollParts

Found in src/phextura.rs at line 1746.

Signature: `pub struct ScrollParts {`

What it does: A scroll split into role line, header lines, and body (text after the first blank line; header-only scrolls have an empty body).

(Where these words came from: doc comment.)

---

## const — TEI_NS

Found in src/phextura.rs at line 47.

Signature: `pub const TEI_NS: &str = "http://www.tei-c.org/ns/1.0";`

What it does: A fixed value named TEI_NS, set to "http://www.tei-c.org/ns/1.0".

(Where these words came from: signature.)

---

## struct — TargetAddr

Found in src/phextura.rs at line 1887.

Signature: `pub struct TargetAddr {`

What it does: A data shape named TargetAddr. Its fields are: pub slug: String; pub pin: String; pub kind: Option<String>; pub key_slug: Option<String>; pub at: Option<(C9, usize, usize)>.

(Where these words came from: signature.)

---

## enum — TargetForm

Found in src/phextura.rs at line 971.

Signature: `pub enum TargetForm {`

What it does: A named choice named TargetForm. It is one of: Object; At.

(Where these words came from: signature.)

---

## struct — ValidateReport

Found in src/phextura.rs at line 2029.

Signature: `pub struct ValidateReport {`

What it does: A data shape named ValidateReport. Its fields are: pub ok: bool; pub failures: Vec<String>; pub base_scrolls: usize; pub standoff_scrolls: usize; pub base_checksum_ok: bool; pub standoff_checksum_ok: bool; pub keys_indexed: usize; pub entries: usize; pub media_headers: usize; pub externals_declared: usize; pub bare_coords_in_prose: usize; pub resolvable_bare_coords: usize.

(Where these words came from: signature.)

---

## function — addr

Found in src/phextura.rs at line 76.

Signature: `pub fn addr(&self) -> String {`

What it does: Function addr takes no inputs beyond the value it is called on, and gives back String.

(Where these words came from: signature.)

---

## function — canonical_bytes

Found in src/phextura.rs at line 272.

Signature: `pub fn canonical_bytes(map: &HashMap<phext::Coordinate, String>) -> Vec<u8> {`

What it does: Function canonical_bytes takes map: &HashMap<phext::Coordinate, String> and gives back Vec<u8>.

(Where these words came from: signature.)

---

## function — check_reconstruction

Found in src/phextura.rs at line 2375.

Signature: `pub fn check_reconstruction( base_path: &Path, standoff_path: &Path, corpus_dir: &Path, ) -> Result<ReconReport, String> {`

What it does: Function check_reconstruction takes base_path: &Path, standoff_path: &Path, corpus_dir: &Path, and gives back Result<ReconReport, String>.

(Where these words came from: signature.)

---

## function — edition_checksum

Found in src/phextura.rs at line 264.

Signature: `pub fn edition_checksum(map: &HashMap<phext::Coordinate, String>, manifest: C9) -> String {`

What it does: The edition checksum: libphext xxh3-128 over the canonical form of the corpus with the checksum line excluded from its manifest scroll.

(Where these words came from: doc comment.)

---

## function — escape_value

Found in src/phextura.rs at line 129.

Signature: `pub fn escape_value(v: &str) -> String {`

What it does: Function escape_value takes v: &str and gives back String.

(Where these words came from: signature.)

---

## function — export_phextura

Found in src/phextura.rs at line 1566.

Signature: `pub fn export_phextura(dir: &Path, out_dir: &Path) -> Result<PhexturaSummary, String> {`

What it does: Function export_phextura takes dir: &Path, out_dir: &Path and gives back Result<PhexturaSummary, String>.

(Where these words came from: signature.)

---

## function — export_phextura_opts

Found in src/phextura.rs at line 1573.

Signature: `pub fn export_phextura_opts( dir: &Path, out_dir: &Path, emit_vfs: bool, ) -> Result<PhexturaSummary, String> {`

What it does: As `export_phextura`; `emit_vfs` controls the D6 PhextVFS image (`cavern.phext`), a third consumer of the assembled pair — the corpus and standoff writers are untouched either way (the D6 seam).

(Where these words came from: doc comment.)

---

## function — iso_from_unix

Found in src/phextura.rs at line 227.

Signature: `pub fn iso_from_unix(secs: u64) -> String {`

What it does: Function iso_from_unix takes secs: u64 and gives back String.

(Where these words came from: signature.)

---

## function — iso_now

Found in src/phextura.rs at line 219.

Signature: `pub fn iso_now() -> String {`

What it does: Function iso_now takes no inputs beyond the value it is called on, and gives back String.

(Where these words came from: signature.)

---

## function — line_of

Found in src/phextura.rs at line 1778.

Signature: `pub fn line_of<'a>(lines: &'a [(String, String)], name: &str) -> Option<&'a str> {`

What it does: Function line_of takes lines: &'a [ and gives back Option<&'a str>.

(Where these words came from: signature.)

---

## function — lines_of

Found in src/phextura.rs at line 1782.

Signature: `pub fn lines_of<'a>(lines: &'a [(String, String)], name: &str) -> Vec<&'a str> {`

What it does: Function lines_of takes lines: &'a [ and gives back Vec<&'a str>.

(Where these words came from: signature.)

---

## function — load_entries

Found in src/phextura.rs at line 2009.

Signature: `pub fn load_entries( standoff: &HashMap<C9, String>, ) -> Result<Vec<ParsedEntry>, String> {`

What it does: All standoff entries, sorted into minting order (series, then scroll).

(Where these words came from: doc comment.)

---

## function — load_map

Found in src/phextura.rs at line 1786.

Signature: `pub fn load_map(path: &Path) -> Result<HashMap<C9, String>, String> {`

What it does: Function load_map takes path: &Path and gives back Result<HashMap<C9, String>, String>.

(Where these words came from: signature.)

---

## function — manifest_coord

Found in src/phextura.rs at line 115.

Signature: `pub fn manifest_coord() -> C9 {`

What it does: The corpus-level manifest coordinate (Series 1, Collection 6).

(Where these words came from: doc comment.)

---

## function — parse

Found in src/phextura.rs at line 86.

Signature: `pub fn parse(s: &str) -> Option<C9> {`

What it does: Function parse takes s: &str and gives back Option<C9>.

(Where these words came from: signature.)

---

## function — parse_entry

Found in src/phextura.rs at line 1946.

Signature: `pub fn parse_entry(coord: C9, text: &str) -> Result<ParsedEntry, String> {`

What it does: Function parse_entry takes coord: C9, text: &str and gives back Result<ParsedEntry, String>.

(Where these words came from: signature.)

---

## function — parse_target

Found in src/phextura.rs at line 1895.

Signature: `pub fn parse_target(addr: &str) -> Result<TargetAddr, String> {`

What it does: Function parse_target takes addr: &str and gives back Result<TargetAddr, String>.

(Where these words came from: signature.)

---

## function — phext

Found in src/phextura.rs at line 83.

Signature: `pub fn phext(&self) -> phext::Coordinate {`

What it does: Function phext takes no inputs beyond the value it is called on, and gives back phext::Coordinate.

(Where these words came from: signature.)

---

## function — relocate

Found in src/phextura.rs at line 2835.

Signature: `pub fn relocate(base_path: &Path, standoff_path: &Path) -> Result<RelocReport, String> {`

What it does: Function relocate takes base_path: &Path, standoff_path: &Path and gives back Result<RelocReport, String>.

(Where these words came from: signature.)

---

## function — render_attr_name

Found in src/phextura.rs at line 1032.

Signature: `pub fn render_attr_name(qname: &str, scope: &HashMap<String, String>) -> String {`

What it does: Render an attribute's qualified name for an `attr-` header line: namespaced names render as `{<namespace-uri>}<local>` (profile §4); the prefix resolves against the xmlns declarations in scope, with `xml` predefined. An unresolvable prefix (undeclared — malformed input) renders as written.

(Where these words came from: doc comment.)

---

## function — slugify

Found in src/phextura.rs at line 169.

Signature: `pub fn slugify(s: &str) -> String {`

What it does: Function slugify takes s: &str and gives back String.

(Where these words came from: signature.)

---

## function — split_scroll

Found in src/phextura.rs at line 1752.

Signature: `pub fn split_scroll(text: &str) -> ScrollParts {`

What it does: Function split_scroll takes text: &str and gives back ScrollParts.

(Where these words came from: signature.)

---

## function — standoff_meta_coord

Found in src/phextura.rs at line 119.

Signature: `pub fn standoff_meta_coord() -> C9 {`

What it does: The standoff's own metadata coordinate (its Collection 6, Series 1).

(Where these words came from: doc comment.)

---

## function — strip_checksum_line

Found in src/phextura.rs at line 255.

Signature: `pub fn strip_checksum_line(text: &str) -> String {`

What it does: Remove any `checksum: xxh3-128:` line from a scroll's text.

(Where these words came from: doc comment.)

---

## function — unescape_value

Found in src/phextura.rs at line 145.

Signature: `pub fn unescape_value(v: &str) -> Result<String, String> {`

What it does: Function unescape_value takes v: &str and gives back Result<String, String>.

(Where these words came from: signature.)

---

## function — validate_pair

Found in src/phextura.rs at line 2073.

Signature: `pub fn validate_pair(base_path: &Path, standoff_path: &Path) -> Result<ValidateReport, String> {`

What it does: Function validate_pair takes base_path: &Path, standoff_path: &Path and gives back Result<ValidateReport, String>.

(Where these words came from: signature.)

---

## function — <'a> Flat<'a>::index_of

Found in src/segment.rs at line 67.

Signature: `pub fn index_of(&self, e: &Element) -> usize {`

What it does: Function <'a> Flat<'a>::index_of takes &self, e: &Element and gives back usize.

(Where these words came from: signature.)

---

## const — BLOCK_ELEMENTS

Found in src/segment.rs at line 28.

Signature: `pub const BLOCK_ELEMENTS: &[&str] = &[ "ab", "additions", "adminInfo", "analytic", "argument", "bibl", "biblStruct", "bindingDesc", "body", "back", "cell", "cit", "closer", "custodialHist", "decoDesc", "div", "div1", "div2", "div3", "div4", "div5", "div6", "div7", "docAuthor", "docDate", "docEdition`

What it does: TEI block-level elements: each owns text segments; any other element containing a block-level descendant acts as a container; everything else is inline.

(Where these words came from: doc comment.)

---

## struct — Flat

Found in src/segment.rs at line 55.

Signature: `pub struct Flat<'a> {`

What it does: A pre-order flattening of one document tree: occurrence indices, parent/depth, subtree ranges, and per-element descendant text.

(Where these words came from: doc comment.)

---

## struct — InlineOcc

Found in src/segment.rs at line 121.

Signature: `pub struct InlineOcc {`

What it does: One inline element occurrence inside a segment.

(Where these words came from: doc comment.)

---

## struct — SegMark

Found in src/segment.rs at line 136.

Signature: `pub struct SegMark {`

What it does: A comment or PI encountered during the walk (ruling D3).

(Where these words came from: doc comment.)

---

## struct — Segment

Found in src/segment.rs at line 152.

Signature: `pub struct Segment {`

What it does: One raw segment (pre-keep-rule).

(Where these words came from: doc comment.)

---

## struct — Segmentation

Found in src/segment.rs at line 163.

Signature: `pub struct Segmentation {`

What it does: A data shape named Segmentation. Its fields are: pub segments: Vec<Segment>; pub marks: Vec<SegMark>.

(Where these words came from: signature.)

---

## function — flatten

Found in src/segment.rs at line 78.

Signature: `pub fn flatten(root: &Element) -> Flat<'_> {`

What it does: Function flatten takes root: &Element and gives back Flat<'_>.

(Where these words came from: signature.)

---

## function — has_block_descendant

Found in src/segment.rs at line 46.

Signature: `pub fn has_block_descendant(el: &Element) -> bool {`

What it does: Function has_block_descendant takes el: &Element and gives back bool.

(Where these words came from: signature.)

---

## function — is_block

Found in src/segment.rs at line 42.

Signature: `pub fn is_block(name: &str) -> bool {`

What it does: Function is_block takes name: &str and gives back bool.

(Where these words came from: signature.)

---

## function — is_empty

Found in src/segment.rs at line 73.

Signature: `pub fn is_empty(&self) -> bool {`

What it does: Function is_empty takes no inputs beyond the value it is called on, and gives back bool.

(Where these words came from: signature.)

---

## function — kept_map

Found in src/segment.rs at line 286.

Signature: `pub fn kept_map(seg: &Segmentation) -> Vec<Option<usize>> {`

What it does: The keep rule (0.2, recon §4.4): a segment survives iff its text is non-whitespace or it carries at least one inline occurrence. Returns (raw index → kept index) for kept segments.

(Where these words came from: doc comment.)

---

## function — len

Found in src/segment.rs at line 70.

Signature: `pub fn len(&self) -> usize {`

What it does: Function len takes no inputs beyond the value it is called on, and gives back usize.

(Where these words came from: signature.)

---

## function — segment_document

Found in src/segment.rs at line 276.

Signature: `pub fn segment_document(root: &Element, flat: &Flat) -> Segmentation {`

What it does: Segment one document. Identical segmentation to the 0.2 export's Builder (same keep rule inputs, same order); the 0.2 test suite is the regression net.

(Where these words came from: doc comment.)

---

## const — VFS_CHUNK_SIZE

Found in src/vfs.rs at line 41.

Signature: `pub const VFS_CHUNK_SIZE: usize = 65536;`

What it does: Mutable's DEFAULT_CHUNK_SIZE (vfs.ts).

(Where these words came from: doc comment.)

---

## struct — VfsImageInfo

Found in src/vfs.rs at line 52.

Signature: `pub struct VfsImageInfo {`

What it does: A data shape named VfsImageInfo. Its fields are: pub bytes: usize; pub scrolls: usize; pub files: usize; pub chunks: usize; pub checksum_scrolls: usize.

(Where these words came from: signature.)

---

## function — base64_encode

Found in src/vfs.rs at line 86.

Signature: `pub fn base64_encode(bytes: &[u8]) -> String {`

What it does: Standard base64 (RFC 4648, padded) — the encoding Mutable's `Buffer.toString("base64")` produces and its reader decodes per chunk.

(Where these words came from: doc comment.)

---

## function — build_cavern_image

Found in src/vfs.rs at line 133.

Signature: `pub fn build_cavern_image(files: &[(&str, &[u8])]) -> (Vec<u8>, VfsImageInfo) {`

What it does: Build the `cavern.phext` image holding `files` (name, bytes) in the given order at the documents-stratum root, in the VFS binary format. Returns the image bytes and its accounting.

(Where these words came from: doc comment.)

---

## function — json_escape

Found in src/vfs.rs at line 112.

Signature: `pub fn json_escape(s: &str) -> String {`

What it does: JSON string escaping with JSON.stringify semantics (the index and manifest scrolls are consumed by Mutable's `JSON.parse`).

(Where these words came from: doc comment.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/teixml-to-mutable/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

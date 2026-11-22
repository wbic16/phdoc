# phextura — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 150). Scanned at commit 9bcc011873c3965ac1cb3306350c007fe5a6c72d. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

Description stated in the manifest: A document model for papers built on phext: an addressed corpus of scrolls instead of a one-dimensional character stream.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Rust (cargo) — Cargo.toml

Direct dependencies: 1.

- libphext [dependencies] — 0.5.0

Total packages in the lock file, this one included: 3.


---

## Authors

The git log holds 16 commits.

The author of record is **Will Bickford** — 16 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Theia — 9 commits
- Emise — 5 commits
- Will Bickford (his own identities) — 2 commits

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 9 files, 4612 lines total, 695 blank, 11 comment, 3906 code
- Rust: 7 files, 278 lines total, 28 blank, 130 comment, 120 code
- TOML: 1 file, 8 lines total, 1 blank, 0 comment, 7 code

All languages: 17 files, 4898 lines total, 4033 lines of code. Functions read: 12.

Largest files:

- PRINCIPLES.md — 785 lines
- STANDOFF.md — 784 lines
- LAYER2.md — 757 lines
- REVIEW-surface-syntax-2026-11-11.md — 707 lines
- DESIGN.md — 572 lines

Largest functions, by span in lines:

- parse in src/reference.rs — about 21 lines
- parse in src/bibliography.rs — about 11 lines
- new in src/document.rs — about 9 lines
- field in src/bibliography.rs — about 8 lines
- load in src/document.rs — about 7 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 26 of 26 (100%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/phextura
- Branch: exo
- Head commit: 9bcc011873c3965ac1cb3306350c007fe5a6c72d
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- struct `Entry` (src/bibliography.rs:14) — One bibliography entry, parsed from its scroll.
- function `Entry::field` (src/bibliography.rs:23) — Look up a field by name (`author`, `title`, `year`, …).
- function `Entry::parse` (src/bibliography.rs:31) — Parse an entry from its scroll text at `coordinate`.
- function `duplicate_candidates` (src/bibliography.rs:42) — Group likely-duplicate entries for an author's review.
- const `BODY` (src/document.rs:14) — Chapters, sections, and content blocks.
- const `BIBLIOGRAPHY` (src/document.rs:16) — One bibliography entry per scroll.
- const `MEDIA` (src/document.rs:18) — Figures and embedded media payloads.
- const `STYLES` (src/document.rs:20) — Style scrolls.
- const `SUPPLEMENTS` (src/document.rs:22) — Embedded documents: data, code, extended proofs.
- const `METADATA` (src/document.rs:24) — Title, authors, abstract, provenance, checksum manifest.
- struct `Paper` (src/document.rs:28) — A phextura paper: the corpus text and the coordinate of its root.
- function `Paper::new` (src/document.rs:39) — Create an empty paper rooted at `root`.
- function `Paper::load` (src/document.rs:48) — Load a paper from imploded phext text (`libphext::phext::explode` is the intended mechanism for working with its…
- function `Paper::fetch` (src/document.rs:55) — Fetch one scroll of the paper by coordinate (`libphext::phext::fetch` is the intended mechanism).
- enum `PayloadEncoding` (src/media.rs:16) — How a payload's bytes are carried as scroll text.
- struct `MediaHeader` (src/media.rs:27) — The header of an embedded media item: one scroll in the media division, preceding the payload scrolls it describes.
- function `verify` (src/media.rs:42) — Verify a payload against its header checksum before rendering.
- struct `Reference` (src/reference.rs:14) — A typed cross-reference: the coordinate of the scroll it points at.
- function `Reference::parse` (src/reference.rs:25) — Parse a reference from its written form in prose.
- struct `DanglingReference` (src/reference.rs:35) — One dangling reference found by validation: where it is written, and the coordinate it fails to reach.
- function `validate` (src/reference.rs:46) — Validate every reference in a corpus in one pass.
- function `render_html` (src/render.rs:17) — Render a paper to HTML.
- function `render_print` (src/render.rs:24) — Render a paper to a print-oriented backend (future work; the layout engine is deliberately unspecified in DESIGN.md…
- struct `StyleSheet` (src/style.rs:15) — A style sheet: the scroll that declares it, and its declarations.
- struct `StyleAssignment` (src/style.rs:25) — Which style governs a subspace of a paper.
- function `parse` (src/style.rs:33) — Parse a style scroll into a [`StyleSheet`].

---

## struct — Entry

Found in src/bibliography.rs at line 14.

Surface: public API.

Signature: `pub struct Entry {`

What it does: One bibliography entry, parsed from its scroll.

(Where these words came from: direct.)

---

## function — Entry::field

Found in src/bibliography.rs at line 23.

Surface: public API.

Signature: `pub fn field(&self, name: &str) -> Option<&str> {`

What it does: Look up a field by name (`author`, `title`, `year`, …).

(Where these words came from: direct.)

---

## function — Entry::parse

Found in src/bibliography.rs at line 31.

Surface: public API.

Signature: `pub fn parse(coordinate: Coordinate, scroll: &str) -> Entry {`

What it does: Parse an entry from its scroll text at `coordinate`.

(Where these words came from: direct.)

---

## function — duplicate_candidates

Found in src/bibliography.rs at line 42.

Surface: public API.

Signature: `pub fn duplicate_candidates(entries: &[Entry]) -> Vec<(Coordinate, Coordinate)> {`

What it does: Group likely-duplicate entries for an author's review. Intended mechanism: `libphext::phext::soundex_v1` on author names as a candidate generator only — a phonetic match is a hint for a human, never proof that two entries are the same work.

(Where these words came from: direct.)

---

## const — BODY

Found in src/document.rs at line 14.

Surface: public API.

Signature: `pub const BODY: usize = 1;`

What it does: Chapters, sections, and content blocks.

(Where these words came from: direct.)

---

## const — BIBLIOGRAPHY

Found in src/document.rs at line 16.

Surface: public API.

Signature: `pub const BIBLIOGRAPHY: usize = 2;`

What it does: One bibliography entry per scroll.

(Where these words came from: direct.)

---

## const — MEDIA

Found in src/document.rs at line 18.

Surface: public API.

Signature: `pub const MEDIA: usize = 3;`

What it does: Figures and embedded media payloads.

(Where these words came from: direct.)

---

## const — STYLES

Found in src/document.rs at line 20.

Surface: public API.

Signature: `pub const STYLES: usize = 4;`

What it does: Style scrolls.

(Where these words came from: direct.)

---

## const — SUPPLEMENTS

Found in src/document.rs at line 22.

Surface: public API.

Signature: `pub const SUPPLEMENTS: usize = 5;`

What it does: Embedded documents: data, code, extended proofs.

(Where these words came from: direct.)

---

## const — METADATA

Found in src/document.rs at line 24.

Surface: public API.

Signature: `pub const METADATA: usize = 6;`

What it does: Title, authors, abstract, provenance, checksum manifest.

(Where these words came from: direct.)

---

## struct — Paper

Found in src/document.rs at line 28.

Surface: public API.

Signature: `pub struct Paper {`

What it does: A phextura paper: the corpus text and the coordinate of its root.

(Where these words came from: direct.)

---

## function — Paper::new

Found in src/document.rs at line 39.

Surface: public API.

Signature: `pub fn new(root: Coordinate) -> Paper {`

What it does: Create an empty paper rooted at `root`. Intended behavior: seed the metadata subspace and an empty body.

(Where these words came from: direct.)

---

## function — Paper::load

Found in src/document.rs at line 48.

Surface: public API.

Signature: `pub fn load(corpus: &str) -> Paper {`

What it does: Load a paper from imploded phext text (`libphext::phext::explode` is the intended mechanism for working with its scrolls).

(Where these words came from: direct.)

---

## function — Paper::fetch

Found in src/document.rs at line 55.

Surface: public API.

Signature: `pub fn fetch(&self, coord: Coordinate) -> String {`

What it does: Fetch one scroll of the paper by coordinate (`libphext::phext::fetch` is the intended mechanism).

(Where these words came from: direct.)

---

## enum — PayloadEncoding

Found in src/media.rs at line 16.

Surface: public API.

Signature: `pub enum PayloadEncoding {`

What it does: How a payload's bytes are carried as scroll text.

(Where these words came from: direct.)

---

## struct — MediaHeader

Found in src/media.rs at line 27.

Surface: public API.

Signature: `pub struct MediaHeader {`

What it does: The header of an embedded media item: one scroll in the media division, preceding the payload scrolls it describes.

(Where these words came from: direct.)

---

## function — verify

Found in src/media.rs at line 42.

Surface: public API.

Signature: `pub fn verify(header: &MediaHeader, payload_text: &str) -> bool {`

What it does: Verify a payload against its header checksum before rendering.

(Where these words came from: direct.)

---

## struct — Reference

Found in src/reference.rs at line 14.

Surface: public API.

Signature: `pub struct Reference {`

What it does: A typed cross-reference: the coordinate of the scroll it points at.

(Where these words came from: direct.)

---

## function — Reference::parse

Found in src/reference.rs at line 25.

Surface: public API.

Signature: `pub fn parse(text: &str) -> Result<Reference, PhextParseError> {`

What it does: Parse a reference from its written form in prose. Delegates to `Coordinate`'s `TryFrom` impl, so malformed addresses surface as [`PhextParseError`] — a lint error, not a panic (the 0.5.0 behavior; 0.4.1 panicked here).

(Where these words came from: direct.)

---

## struct — DanglingReference

Found in src/reference.rs at line 35.

Surface: public API.

Signature: `pub struct DanglingReference {`

What it does: One dangling reference found by validation: where it is written, and the coordinate it fails to reach.

(Where these words came from: direct.)

---

## function — validate

Found in src/reference.rs at line 46.

Surface: public API.

Signature: `pub fn validate(corpus: &str) -> Vec<DanglingReference> {`

What it does: Validate every reference in a corpus in one pass. Intended mechanism: build the reference graph with `libphext::phext::navmap`, then `fetch` each target.

(Where these words came from: direct.)

---

## function — render_html

Found in src/render.rs at line 17.

Surface: public API.

Signature: `pub fn render_html(paper: &Paper) -> String {`

What it does: Render a paper to HTML.

(Where these words came from: direct.)

---

## function — render_print

Found in src/render.rs at line 24.

Surface: public API.

Signature: `pub fn render_print(paper: &Paper) -> Vec<u8> {`

What it does: Render a paper to a print-oriented backend (future work; the layout engine is deliberately unspecified in DESIGN.md §10).

(Where these words came from: direct.)

---

## struct — StyleSheet

Found in src/style.rs at line 15.

Surface: public API.

Signature: `pub struct StyleSheet {`

What it does: A style sheet: the scroll that declares it, and its declarations.

(Where these words came from: direct.)

---

## struct — StyleAssignment

Found in src/style.rs at line 25.

Surface: public API.

Signature: `pub struct StyleAssignment {`

What it does: Which style governs a subspace of a paper.

(Where these words came from: direct.)

---

## function — parse

Found in src/style.rs at line 33.

Surface: public API.

Signature: `pub fn parse(coordinate: Coordinate, scroll: &str) -> StyleSheet {`

What it does: Parse a style scroll into a [`StyleSheet`].

(Where these words came from: direct.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/phextura/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

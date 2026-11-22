# holo-phext — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 147). Scanned at commit 50accbe7b15cafc2816846c4c1d6441d1d573a56. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

Description stated in the manifest: Layer2 over phext: a memory-mapped lattice with async I/O, update dispatch, and ACID multi-scroll transactions. The storage engine phext-lattice will be re-implemented on, and the durability layer for phext-native stores (HoloStore round 2).

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 4.

- # tree; then this becomes `libphext [dependencies] — 0.6.2"`.
- libphext [dependencies] — (path or git)
- memmap2 [dependencies] — 0.9
- tokio [dependencies] — 1

Total packages in the lock file, this one included: 23.

### Rust (cargo) — holo-edit/Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 4.

- holo-phext [dependencies] — (path or git)
- libphext [dependencies] — (path or git)
- sha2 [dependencies] — 0.10
- tokio [dev-dependencies] — 1

Total packages in the lock file, this one included: 23.


---

## Authors

The git log holds 5 commits.

The author of record is **Will Bickford** — 5 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Theia — 2 commits
- Will Bickford (his own identities) — 2 commits
- Emise — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 2 files, 151 lines total, 22 blank, 0 comment, 129 code
- Rust: 9 files, 1919 lines total, 155 blank, 267 comment, 1497 code
- TOML: 2 files, 35 lines total, 4 blank, 4 comment, 27 code

All languages: 13 files, 2105 lines total, 1653 lines of code. Functions read: 104.

Largest files:

- src/core.rs — 477 lines
- holo-edit/src/bin/holo-edit.rs — 388 lines
- src/index.rs — 323 lines
- src/lib.rs — 191 lines
- tests/acid.rs — 174 lines

Largest functions, by span in lines:

- route in holo-edit/src/bin/holo-edit.rs — about 145 lines
- open in src/core.rs — about 94 lines
- paired_commits_are_never_seen_half_applied in holo-edit/tests/isolation.rs — about 87 lines
- commit_batch in src/core.rs — about 72 lines
- open in src/lib.rs — about 63 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 14 test functions counted in the source, across 1 file whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 39 of 78 (50%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/holo-phext
- Branch: main
- Head commit: 50accbe7b15cafc2816846c4c1d6441d1d573a56
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- entry point `main` (holo-edit/src/bin/holo-dump.rs:19) — The entry point: the program starts here.
- entry point `main` (holo-edit/src/bin/holo-edit.rs:337) — The entry point: the program starts here.
- flag `--local` (holo-edit/src/bin/holo-edit.rs:344) — A command line flag, --local.
- flag `--port` (holo-edit/src/bin/holo-edit.rs:347) — A command line flag, --port.
- struct `LatticeStore` (src/compat.rs:35) — awaiting a summary
- function `LatticeStore::open` (src/compat.rs:42) — Function LatticeStore::open takes path: P.
- function `LatticeStore::read_scroll` (src/compat.rs:47) — Function LatticeStore::read_scroll takes &self, coord: &Coordinate.
- function `LatticeStore::write_scroll` (src/compat.rs:54) — Function LatticeStore::write_scroll takes &mut self, coord: Coordinate, content: String.
- function `LatticeStore::delete_scroll` (src/compat.rs:63) — Function LatticeStore::delete_scroll takes &mut self, coord: Coordinate.
- function `LatticeStore::has_scroll` (src/compat.rs:67) — Function LatticeStore::has_scroll takes &self, coord: &Coordinate.
- function `LatticeStore::populated_coordinates` (src/compat.rs:71) — Function LatticeStore::populated_coordinates takes no inputs beyond the value it is called on.
- function `LatticeStore::scroll_count` (src/compat.rs:89) — Function LatticeStore::scroll_count takes no inputs beyond the value it is called on.
- function `LatticeStore::is_dirty` (src/compat.rs:93) — Function LatticeStore::is_dirty takes no inputs beyond the value it is called on.
- function `LatticeStore::pending_changes` (src/compat.rs:97) — Function LatticeStore::pending_changes takes no inputs beyond the value it is called on.
- function `LatticeStore::revert` (src/compat.rs:101) — Function LatticeStore::revert takes no inputs beyond the value it is called on.
- function `LatticeStore::save` (src/compat.rs:108) — Commits the staged changes as one transaction, then checkpoints — after `save`, the base file at `path()` is the…
- function `LatticeStore::to_phext_bytes` (src/compat.rs:121) — The merged document (base + committed overlay + staged), serialized — `MappedLattice::to_phext_bytes`.
- function `LatticeStore::path` (src/compat.rs:143) — Function LatticeStore::path takes no inputs beyond the value it is called on.
- function `LatticeStore::core` (src/compat.rs:149) — Access to the underlying core, for servers that want the transactional interface alongside the lattice surface.
- enum `Op` (src/core.rs:38) — One write inside a transaction.
- struct `Txn` (src/core.rs:46) — A multi-scroll transaction: a set of ops applied atomically or not at all.
- function `Txn::new` (src/core.rs:51) — Function Txn::new takes no inputs beyond the value it is called on.
- function `Txn::set` (src/core.rs:54) — Function Txn::set takes mut self, coord: Coordinate, content: &str.
- function `Txn::delete` (src/core.rs:58) — Function Txn::delete takes mut self, coord: Coordinate.
- function `Txn::is_empty` (src/core.rs:62) — Function Txn::is_empty takes no inputs beyond the value it is called on.
- struct `BaseLattice` (src/core.rs:68) — The read-only memory-mapped base lattice.
- function `BaseLattice::open` (src/core.rs:74) — Function BaseLattice::open takes path: &Path.
- function `BaseLattice::read_scroll` (src/core.rs:88) — Function BaseLattice::read_scroll takes &self, coord: &Coordinate.
- function `BaseLattice::bytes` (src/core.rs:94) — Function BaseLattice::bytes takes no inputs beyond the value it is called on.
- function `BaseLattice::coordinates` (src/core.rs:98) — Function BaseLattice::coordinates takes no inputs beyond the value it is called on.
- struct `Shared` (src/core.rs:105) — Shared, snapshot-able read state.
- function `Shared::snapshot` (src/core.rs:112) — Function Shared::snapshot takes no inputs beyond the value it is called on.
- function `Shared::seq` (src/core.rs:120) — Function Shared::seq takes no inputs beyond the value it is called on.
- struct `Snapshot` (src/core.rs:126) — A frozen read view at one sequence number.
- function `Snapshot::seq` (src/core.rs:133) — Function Snapshot::seq takes no inputs beyond the value it is called on.
- function `Snapshot::base_bytes` (src/core.rs:138) — The raw bytes of the base document under this snapshot.
- function `Snapshot::get` (src/core.rs:142) — Function Snapshot::get takes &self, coord: &Coordinate.
- function `Snapshot::contains` (src/core.rs:149) — Function Snapshot::contains takes &self, coord: &Coordinate.
- function `Snapshot::coordinates` (src/core.rs:154) — Every populated coordinate in this snapshot, ascending.
- struct `Core` (src/core.rs:211) — The synchronous engine.
- function `Core::open` (src/core.rs:222) — Function Core::open takes base_path: &Path.
- function `Core::seq` (src/core.rs:346) — Function Core::seq takes no inputs beyond the value it is called on.
- function `Core::checkpoint_seq` (src/core.rs:350) — Function Core::checkpoint_seq takes no inputs beyond the value it is called on.
- function `Core::wal_path` (src/core.rs:354) — Function Core::wal_path takes no inputs beyond the value it is called on.
- function `Core::base_path` (src/core.rs:358) — Function Core::base_path takes no inputs beyond the value it is called on.
- function `Core::commit_batch` (src/core.rs:367) — Commits a batch of transactions: all of them are serialized into the WAL, written as one suffix, and synced ONCE…
- function `Core::checkpoint` (src/core.rs:439) — Folds the overlay into the base file (temp + sync + rename + directory sync), remaps the base, and restarts the WAL with only its meta scroll.
- struct `ScrollSpan` (src/index.rs:19) — A scroll's location within the raw phext byte buffer.
- function `ScrollSpan::len` (src/index.rs:27) — Function ScrollSpan::len takes no inputs beyond the value it is called on.
- function `ScrollSpan::is_empty` (src/index.rs:31) — Function ScrollSpan::is_empty takes no inputs beyond the value it is called on.
- enum `Dimension` (src/index.rs:38) — The nine phext dimensions, in address order.
- function `dimension_value` (src/index.rs:51) — The value of one dimension of a coordinate.
- struct `LatticeIndex` (src/index.rs:67) — The lattice index: maps every populated coordinate to its byte span.
- function `LatticeIndex::build` (src/index.rs:110) — Build an index by scanning the raw phext bytes once.
- function `LatticeIndex::from_entries` (src/index.rs:151) — Build from (coordinate, span) pairs, e.g.
- function `LatticeIndex::get` (src/index.rs:168) — Look up the byte span for a coordinate.
- function `LatticeIndex::contains` (src/index.rs:173) — Check if a coordinate has content.
- function `LatticeIndex::scroll_count` (src/index.rs:178) — Number of populated scrolls.
- function `LatticeIndex::spans` (src/index.rs:183) — Spans in document order, parallel to `coordinates()`.
- function `LatticeIndex::buffer_len` (src/index.rs:188) — Total buffer size.
- function `LatticeIndex::coordinates` (src/index.rs:193) — All populated coordinates in document order.
- function `LatticeIndex::next_populated` (src/index.rs:198) — The next populated coordinate after `coord`, if any.
- function `LatticeIndex::prev_populated` (src/index.rs:206) — The previous populated coordinate before `coord`, if any.
- function `LatticeIndex::coordinates_matching` (src/index.rs:220) — All populated coordinates matching a predicate — the lattice navigator's workhorse.
- function `LatticeIndex::dimension_extent` (src/index.rs:229) — The distinct values a dimension takes across all populated coordinates, sorted.
- re-export `core::{Core, Op, Snapshot, Txn}` (src/lib.rs:34) — A re-export: core::{Core, Op, Snapshot, Txn} is made reachable through this module, so other code can use it from here.
- re-export `index::{Dimension, LatticeIndex, ScrollSpan}` (src/lib.rs:35) — A re-export: index::{Dimension, LatticeIndex, ScrollSpan} is made reachable through this module, so other code can…
- enum `Error` (src/lib.rs:44) — Errors from the async interface.
- type `Result` (src/lib.rs:67) — A type name, Result, that stands for std::result::Result<T, Error>.
- struct `Update` (src/lib.rs:71) — One committed transaction, dispatched to subscribers in order.
- struct `HoloPhext` (src/lib.rs:82) — An open HoloPhext store: an async handle over one `Core`.
- function `HoloPhext::open` (src/lib.rs:92) — Opens (creating if needed) the store whose base document is at `base_path`; the WAL lives beside it.
- function `HoloPhext::commit` (src/lib.rs:155) — Commits one transaction atomically; resolves with its sequence number once the WAL write is synced and the…
- function `HoloPhext::checkpoint` (src/lib.rs:163) — Folds all committed state into the base document and restarts the WAL (see `Core::checkpoint`).
- function `HoloPhext::snapshot` (src/lib.rs:171) — A frozen read view at the current sequence.
- function `HoloPhext::read` (src/lib.rs:176) — Reads one scroll from a fresh snapshot.
- function `HoloPhext::seq` (src/lib.rs:181) — The latest committed sequence.
- function `HoloPhext::subscribe` (src/lib.rs:188) — Subscribes to committed updates, in commit order.

---

## entry point — main

Found in holo-edit/src/bin/holo-dump.rs at line 19.

Surface: public API.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## entry point — main

Found in holo-edit/src/bin/holo-edit.rs at line 337.

Surface: public API.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## flag — --local

Found in holo-edit/src/bin/holo-edit.rs at line 344.

Surface: public API.

Signature: `flag --local`

What it does: A command line flag, --local. It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## flag — --port

Found in holo-edit/src/bin/holo-edit.rs at line 347.

Surface: public API.

Signature: `flag --port`

What it does: A command line flag, --port. It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## struct — LatticeStore

Found in src/compat.rs at line 35.

Surface: public API.

Signature: `pub struct LatticeStore {`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## function — LatticeStore::open

Found in src/compat.rs at line 42.

Surface: public API.

Signature: `pub fn open<P: AsRef<Path>>(path: P) -> io::Result<LatticeStore> {`

What it does: Function LatticeStore::open takes path: P. It gives back io::Result<LatticeStore>.

(Where these words came from: inferred.)

---

## function — LatticeStore::read_scroll

Found in src/compat.rs at line 47.

Surface: public API.

Signature: `pub fn read_scroll(&self, coord: &Coordinate) -> Option<String> {`

What it does: Function LatticeStore::read_scroll takes &self, coord: &Coordinate. It gives back Option<String>.

(Where these words came from: inferred.)

---

## function — LatticeStore::write_scroll

Found in src/compat.rs at line 54.

Surface: public API.

Signature: `pub fn write_scroll(&mut self, coord: Coordinate, content: String) -> bool {`

What it does: Function LatticeStore::write_scroll takes &mut self, coord: Coordinate, content: String. It gives back bool.

(Where these words came from: inferred.)

---

## function — LatticeStore::delete_scroll

Found in src/compat.rs at line 63.

Surface: public API.

Signature: `pub fn delete_scroll(&mut self, coord: Coordinate) {`

What it does: Function LatticeStore::delete_scroll takes &mut self, coord: Coordinate. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — LatticeStore::has_scroll

Found in src/compat.rs at line 67.

Surface: public API.

Signature: `pub fn has_scroll(&self, coord: &Coordinate) -> bool {`

What it does: Function LatticeStore::has_scroll takes &self, coord: &Coordinate. It gives back bool.

(Where these words came from: inferred.)

---

## function — LatticeStore::populated_coordinates

Found in src/compat.rs at line 71.

Surface: public API.

Signature: `pub fn populated_coordinates(&self) -> Vec<Coordinate> {`

What it does: Function LatticeStore::populated_coordinates takes no inputs beyond the value it is called on. It gives back Vec<Coordinate>.

(Where these words came from: inferred.)

---

## function — LatticeStore::scroll_count

Found in src/compat.rs at line 89.

Surface: public API.

Signature: `pub fn scroll_count(&self) -> usize {`

What it does: Function LatticeStore::scroll_count takes no inputs beyond the value it is called on. It gives back usize.

(Where these words came from: inferred.)

---

## function — LatticeStore::is_dirty

Found in src/compat.rs at line 93.

Surface: public API.

Signature: `pub fn is_dirty(&self) -> bool {`

What it does: Function LatticeStore::is_dirty takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — LatticeStore::pending_changes

Found in src/compat.rs at line 97.

Surface: public API.

Signature: `pub fn pending_changes(&self) -> usize {`

What it does: Function LatticeStore::pending_changes takes no inputs beyond the value it is called on. It gives back usize.

(Where these words came from: inferred.)

---

## function — LatticeStore::revert

Found in src/compat.rs at line 101.

Surface: public API.

Signature: `pub fn revert(&mut self) {`

What it does: Function LatticeStore::revert takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — LatticeStore::save

Found in src/compat.rs at line 108.

Surface: public API.

Signature: `pub fn save(&mut self) -> io::Result<()> {`

What it does: Commits the staged changes as one transaction, then checkpoints — after `save`, the base file at `path()` is the complete merged document, as with `MappedLattice::save`.

(Where these words came from: direct.)

---

## function — LatticeStore::to_phext_bytes

Found in src/compat.rs at line 121.

Surface: public API.

Signature: `pub fn to_phext_bytes(&self) -> Vec<u8> {`

What it does: The merged document (base + committed overlay + staged), serialized — `MappedLattice::to_phext_bytes`.

(Where these words came from: direct.)

---

## function — LatticeStore::path

Found in src/compat.rs at line 143.

Surface: public API.

Signature: `pub fn path(&self) -> &Path {`

What it does: Function LatticeStore::path takes no inputs beyond the value it is called on. It gives back &Path.

(Where these words came from: inferred.)

---

## function — LatticeStore::core

Found in src/compat.rs at line 149.

Surface: public API.

Signature: `pub fn core(&mut self) -> &mut Core {`

What it does: Access to the underlying core, for servers that want the transactional interface alongside the lattice surface.

(Where these words came from: direct.)

---

## enum — Op

Found in src/core.rs at line 38.

Surface: public API.

Signature: `pub enum Op {`

What it does: One write inside a transaction.

(Where these words came from: direct.)

---

## struct — Txn

Found in src/core.rs at line 46.

Surface: public API.

Signature: `pub struct Txn {`

What it does: A multi-scroll transaction: a set of ops applied atomically or not at all.

(Where these words came from: direct.)

---

## function — Txn::new

Found in src/core.rs at line 51.

Surface: public API.

Signature: `pub fn new() -> Txn {`

What it does: Function Txn::new takes no inputs beyond the value it is called on. It gives back Txn.

(Where these words came from: inferred.)

---

## function — Txn::set

Found in src/core.rs at line 54.

Surface: public API.

Signature: `pub fn set(mut self, coord: Coordinate, content: &str) -> Txn {`

What it does: Function Txn::set takes mut self, coord: Coordinate, content: &str. It gives back Txn.

(Where these words came from: inferred.)

---

## function — Txn::delete

Found in src/core.rs at line 58.

Surface: public API.

Signature: `pub fn delete(mut self, coord: Coordinate) -> Txn {`

What it does: Function Txn::delete takes mut self, coord: Coordinate. It gives back Txn.

(Where these words came from: inferred.)

---

## function — Txn::is_empty

Found in src/core.rs at line 62.

Surface: public API.

Signature: `pub fn is_empty(&self) -> bool {`

What it does: Function Txn::is_empty takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## struct — BaseLattice

Found in src/core.rs at line 68.

Surface: public API.

Signature: `pub struct BaseLattice {`

What it does: The read-only memory-mapped base lattice.

(Where these words came from: direct.)

---

## function — BaseLattice::open

Found in src/core.rs at line 74.

Surface: public API.

Signature: `pub fn open(path: &Path) -> io::Result<BaseLattice> {`

What it does: Function BaseLattice::open takes path: &Path. It gives back io::Result<BaseLattice>.

(Where these words came from: inferred.)

---

## function — BaseLattice::read_scroll

Found in src/core.rs at line 88.

Surface: public API.

Signature: `pub fn read_scroll(&self, coord: &Coordinate) -> Option<String> {`

What it does: Function BaseLattice::read_scroll takes &self, coord: &Coordinate. It gives back Option<String>.

(Where these words came from: inferred.)

---

## function — BaseLattice::bytes

Found in src/core.rs at line 94.

Surface: public API.

Signature: `pub fn bytes(&self) -> &[u8] {`

What it does: Function BaseLattice::bytes takes no inputs beyond the value it is called on. It gives back &[u8].

(Where these words came from: inferred.)

---

## function — BaseLattice::coordinates

Found in src/core.rs at line 98.

Surface: public API.

Signature: `pub fn coordinates(&self) -> &[Coordinate] {`

What it does: Function BaseLattice::coordinates takes no inputs beyond the value it is called on. It gives back &[Coordinate].

(Where these words came from: inferred.)

---

## struct — Shared

Found in src/core.rs at line 105.

Surface: public API.

Signature: `pub struct Shared {`

What it does: Shared, snapshot-able read state. Writers swap Arcs; readers clone them into a `Snapshot` and are thereafter immune to new commits.

(Where these words came from: direct.)

---

## function — Shared::snapshot

Found in src/core.rs at line 112.

Surface: public API.

Signature: `pub fn snapshot(&self) -> Snapshot {`

What it does: Function Shared::snapshot takes no inputs beyond the value it is called on. It gives back Snapshot.

(Where these words came from: inferred.)

---

## function — Shared::seq

Found in src/core.rs at line 120.

Surface: public API.

Signature: `pub fn seq(&self) -> u64 {`

What it does: Function Shared::seq takes no inputs beyond the value it is called on. It gives back u64.

(Where these words came from: inferred.)

---

## struct — Snapshot

Found in src/core.rs at line 126.

Surface: public API.

Signature: `pub struct Snapshot {`

What it does: A frozen read view at one sequence number.

(Where these words came from: direct.)

---

## function — Snapshot::seq

Found in src/core.rs at line 133.

Surface: public API.

Signature: `pub fn seq(&self) -> u64 {`

What it does: Function Snapshot::seq takes no inputs beyond the value it is called on. It gives back u64.

(Where these words came from: inferred.)

---

## function — Snapshot::base_bytes

Found in src/core.rs at line 138.

Surface: public API.

Signature: `pub fn base_bytes(&self) -> &[u8] {`

What it does: The raw bytes of the base document under this snapshot.

(Where these words came from: direct.)

---

## function — Snapshot::get

Found in src/core.rs at line 142.

Surface: public API.

Signature: `pub fn get(&self, coord: &Coordinate) -> Option<String> {`

What it does: Function Snapshot::get takes &self, coord: &Coordinate. It gives back Option<String>.

(Where these words came from: inferred.)

---

## function — Snapshot::contains

Found in src/core.rs at line 149.

Surface: public API.

Signature: `pub fn contains(&self, coord: &Coordinate) -> bool {`

What it does: Function Snapshot::contains takes &self, coord: &Coordinate. It gives back bool.

(Where these words came from: inferred.)

---

## function — Snapshot::coordinates

Found in src/core.rs at line 154.

Surface: public API.

Signature: `pub fn coordinates(&self) -> Vec<Coordinate> {`

What it does: Every populated coordinate in this snapshot, ascending.

(Where these words came from: direct.)

---

## struct — Core

Found in src/core.rs at line 211.

Surface: public API.

Signature: `pub struct Core {`

What it does: The synchronous engine. The async interface drives one `Core` from a single writer task; the compat facade drives it directly.

(Where these words came from: direct.)

---

## function — Core::open

Found in src/core.rs at line 222.

Surface: public API.

Signature: `pub fn open(base_path: &Path) -> io::Result<Core> {`

What it does: Function Core::open takes base_path: &Path. It gives back io::Result<Core>.

(Where these words came from: inferred.)

---

## function — Core::seq

Found in src/core.rs at line 346.

Surface: public API.

Signature: `pub fn seq(&self) -> u64 {`

What it does: Function Core::seq takes no inputs beyond the value it is called on. It gives back u64.

(Where these words came from: inferred.)

---

## function — Core::checkpoint_seq

Found in src/core.rs at line 350.

Surface: public API.

Signature: `pub fn checkpoint_seq(&self) -> u64 {`

What it does: Function Core::checkpoint_seq takes no inputs beyond the value it is called on. It gives back u64.

(Where these words came from: inferred.)

---

## function — Core::wal_path

Found in src/core.rs at line 354.

Surface: public API.

Signature: `pub fn wal_path(&self) -> &Path {`

What it does: Function Core::wal_path takes no inputs beyond the value it is called on. It gives back &Path.

(Where these words came from: inferred.)

---

## function — Core::base_path

Found in src/core.rs at line 358.

Surface: public API.

Signature: `pub fn base_path(&self) -> &Path {`

What it does: Function Core::base_path takes no inputs beyond the value it is called on. It gives back &Path.

(Where these words came from: inferred.)

---

## function — Core::commit_batch

Found in src/core.rs at line 367.

Surface: public API.

Signature: `pub fn commit_batch(&mut self, txns: Vec<Txn>) -> io::Result<Vec<u64>> {`

What it does: Commits a batch of transactions: all of them are serialized into the WAL, written as one suffix, and synced ONCE; only then is each transaction applied, in order, one overlay swap per transaction. Returns the sequence of each transaction. This is the group-commit point the async writer drives.

(Where these words came from: direct.)

---

## function — Core::checkpoint

Found in src/core.rs at line 439.

Surface: public API.

Signature: `pub fn checkpoint(&mut self) -> io::Result<()> {`

What it does: Folds the overlay into the base file (temp + sync + rename + directory sync), remaps the base, and restarts the WAL with only its meta scroll. Crash between the rename and the WAL rewrite is safe: replaying already-folded transactions onto the new base is idempotent, because ops are absolute.

(Where these words came from: direct.)

---

## struct — ScrollSpan

Found in src/index.rs at line 19.

Surface: public API.

Signature: `pub struct ScrollSpan {`

What it does: A scroll's location within the raw phext byte buffer.

(Where these words came from: direct.)

---

## function — ScrollSpan::len

Found in src/index.rs at line 27.

Surface: public API.

Signature: `pub fn len(&self) -> usize {`

What it does: Function ScrollSpan::len takes no inputs beyond the value it is called on. It gives back usize.

(Where these words came from: inferred.)

---

## function — ScrollSpan::is_empty

Found in src/index.rs at line 31.

Surface: public API.

Signature: `pub fn is_empty(&self) -> bool {`

What it does: Function ScrollSpan::is_empty takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## enum — Dimension

Found in src/index.rs at line 38.

Surface: public API.

Signature: `pub enum Dimension {`

What it does: The nine phext dimensions, in address order.

(Where these words came from: direct.)

---

## function — dimension_value

Found in src/index.rs at line 51.

Surface: public API.

Signature: `pub fn dimension_value(coord: &Coordinate, dim: Dimension) -> usize {`

What it does: The value of one dimension of a coordinate.

(Where these words came from: direct.)

---

## struct — LatticeIndex

Found in src/index.rs at line 67.

Surface: public API.

Signature: `pub struct LatticeIndex {`

What it does: The lattice index: maps every populated coordinate to its byte span.

(Where these words came from: direct.)

---

## function — LatticeIndex::build

Found in src/index.rs at line 110.

Surface: public API.

Signature: `pub fn build(buffer: &[u8]) -> LatticeIndex {`

What it does: Build an index by scanning the raw phext bytes once. Time: O(n) in buffer length. Space: O(k) in populated scrolls. Empty runs between delimiter bytes are addressing, not scrolls, and get no span (a scroll exists iff it holds at least one byte — the phext definition).

(Where these words came from: direct.)

---

## function — LatticeIndex::from_entries

Found in src/index.rs at line 151.

Surface: public API.

Signature: `pub fn from_entries(mut entries: Vec<(Coordinate, ScrollSpan)>, buffer_len: usize) -> LatticeIndex {`

What it does: Build from (coordinate, span) pairs, e.g. a persisted span table or spans recorded while serializing. Pairs are normally already in document order; otherwise they are sorted. A repeated coordinate keeps its last span.

(Where these words came from: direct.)

---

## function — LatticeIndex::get

Found in src/index.rs at line 168.

Surface: public API.

Signature: `pub fn get(&self, coord: &Coordinate) -> Option<&ScrollSpan> {`

What it does: Look up the byte span for a coordinate. O(log n).

(Where these words came from: direct.)

---

## function — LatticeIndex::contains

Found in src/index.rs at line 173.

Surface: public API.

Signature: `pub fn contains(&self, coord: &Coordinate) -> bool {`

What it does: Check if a coordinate has content.

(Where these words came from: direct.)

---

## function — LatticeIndex::scroll_count

Found in src/index.rs at line 178.

Surface: public API.

Signature: `pub fn scroll_count(&self) -> usize {`

What it does: Number of populated scrolls.

(Where these words came from: direct.)

---

## function — LatticeIndex::spans

Found in src/index.rs at line 183.

Surface: public API.

Signature: `pub fn spans(&self) -> &[ScrollSpan] {`

What it does: Spans in document order, parallel to `coordinates()`.

(Where these words came from: direct.)

---

## function — LatticeIndex::buffer_len

Found in src/index.rs at line 188.

Surface: public API.

Signature: `pub fn buffer_len(&self) -> usize {`

What it does: Total buffer size.

(Where these words came from: direct.)

---

## function — LatticeIndex::coordinates

Found in src/index.rs at line 193.

Surface: public API.

Signature: `pub fn coordinates(&self) -> &[Coordinate] {`

What it does: All populated coordinates in document order.

(Where these words came from: direct.)

---

## function — LatticeIndex::next_populated

Found in src/index.rs at line 198.

Surface: public API.

Signature: `pub fn next_populated(&self, coord: &Coordinate) -> Option<Coordinate> {`

What it does: The next populated coordinate after `coord`, if any.

(Where these words came from: direct.)

---

## function — LatticeIndex::prev_populated

Found in src/index.rs at line 206.

Surface: public API.

Signature: `pub fn prev_populated(&self, coord: &Coordinate) -> Option<Coordinate> {`

What it does: The previous populated coordinate before `coord`, if any.

(Where these words came from: direct.)

---

## function — LatticeIndex::coordinates_matching

Found in src/index.rs at line 220.

Surface: public API.

Signature: `pub fn coordinates_matching<F>(&self, predicate: F) -> Vec<Coordinate> where F: Fn(&Coordinate) -> bool, {`

What it does: All populated coordinates matching a predicate — the lattice navigator's workhorse.

(Where these words came from: direct.)

---

## function — LatticeIndex::dimension_extent

Found in src/index.rs at line 229.

Surface: public API.

Signature: `pub fn dimension_extent(&self, dim: Dimension) -> Vec<usize> {`

What it does: The distinct values a dimension takes across all populated coordinates, sorted.

(Where these words came from: direct.)

---

## re-export — core::{Core, Op, Snapshot, Txn}

Found in src/lib.rs at line 34.

Surface: public API.

Signature: `pub use core::{Core, Op, Snapshot, Txn};`

What it does: A re-export: core::{Core, Op, Snapshot, Txn} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — index::{Dimension, LatticeIndex, ScrollSpan}

Found in src/lib.rs at line 35.

Surface: public API.

Signature: `pub use index::{Dimension, LatticeIndex, ScrollSpan};`

What it does: A re-export: index::{Dimension, LatticeIndex, ScrollSpan} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## enum — Error

Found in src/lib.rs at line 44.

Surface: public API.

Signature: `pub enum Error {`

What it does: Errors from the async interface.

(Where these words came from: direct.)

---

## type — Result

Found in src/lib.rs at line 67.

Surface: public API.

Signature: `pub type Result<T> = std::result::Result<T, Error>;`

What it does: A type name, Result, that stands for std::result::Result<T, Error>.

(Where these words came from: inferred.)

---

## struct — Update

Found in src/lib.rs at line 71.

Surface: public API.

Signature: `pub struct Update {`

What it does: One committed transaction, dispatched to subscribers in order.

(Where these words came from: direct.)

---

## struct — HoloPhext

Found in src/lib.rs at line 82.

Surface: public API.

Signature: `pub struct HoloPhext {`

What it does: An open HoloPhext store: an async handle over one `Core`.

(Where these words came from: direct.)

---

## function — HoloPhext::open

Found in src/lib.rs at line 92.

Surface: public API.

Signature: `pub async fn open(base_path: &Path) -> Result<HoloPhext> {`

What it does: Opens (creating if needed) the store whose base document is at `base_path`; the WAL lives beside it. Recovery runs here: valid committed transactions replay, a torn tail is truncated.

(Where these words came from: direct.)

---

## function — HoloPhext::commit

Found in src/lib.rs at line 155.

Surface: public API.

Signature: `pub async fn commit(&self, txn: Txn) -> Result<u64> {`

What it does: Commits one transaction atomically; resolves with its sequence number once the WAL write is synced and the transaction is visible to new snapshots.

(Where these words came from: direct.)

---

## function — HoloPhext::checkpoint

Found in src/lib.rs at line 163.

Surface: public API.

Signature: `pub async fn checkpoint(&self) -> Result<()> {`

What it does: Folds all committed state into the base document and restarts the WAL (see `Core::checkpoint`).

(Where these words came from: direct.)

---

## function — HoloPhext::snapshot

Found in src/lib.rs at line 171.

Surface: public API.

Signature: `pub fn snapshot(&self) -> Snapshot {`

What it does: A frozen read view at the current sequence. Cheap: two Arc clones. The snapshot never observes a partial transaction.

(Where these words came from: direct.)

---

## function — HoloPhext::read

Found in src/lib.rs at line 176.

Surface: public API.

Signature: `pub fn read(&self, coord: &Coordinate) -> Option<String> {`

What it does: Reads one scroll from a fresh snapshot.

(Where these words came from: direct.)

---

## function — HoloPhext::seq

Found in src/lib.rs at line 181.

Surface: public API.

Signature: `pub fn seq(&self) -> u64 {`

What it does: The latest committed sequence.

(Where these words came from: direct.)

---

## function — HoloPhext::subscribe

Found in src/lib.rs at line 188.

Surface: public API.

Signature: `pub fn subscribe(&self) -> broadcast::Receiver<Update> {`

What it does: Subscribes to committed updates, in commit order. Lagging receivers skip (broadcast semantics) — a subscriber that must not miss changes should re-snapshot on `Lagged`.

(Where these words came from: direct.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/holo-phext/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

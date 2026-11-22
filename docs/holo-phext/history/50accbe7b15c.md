# holo-phext — reference as of commit 50accbe7b15c

Commit 50accbe7b15cafc2816846c4c1d6441d1d573a56. 5 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/holo-phext/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

## Dependencies

### Cargo.toml

- # tree; then this becomes `libphext — 0.6.2"`.
- libphext — (path or git)
- memmap2 — 0.9
- tokio — 1

### holo-edit/Cargo.toml

- holo-phext — (path or git)
- libphext — (path or git)
- sha2 — 0.10
- tokio — 1

## Authors

- Theia <theia@mirrorborn.us> — 2 commits
- Will Bickford <wbic16@gmail.com> — 2 commits
- Emise <emise@mirrorborn.us> — 1 commits

## Complexity

- Markdown: 2 files, 151 lines total, 22 blank, 0 comment, 129 code
- Rust: 9 files, 1919 lines total, 155 blank, 267 comment, 1497 code
- TOML: 2 files, 35 lines total, 4 blank, 4 comment, 27 code

Declared RFCs at this point: none declared.

## Public methods and commands

- entry point `main` (holo-edit/src/bin/holo-dump.rs) — The entry point: the program starts here.
- command `--local` (holo-edit/src/bin/holo-edit.rs) — A command named --local.
- command `--port` (holo-edit/src/bin/holo-edit.rs) — A command named --port.
- entry point `main` (holo-edit/src/bin/holo-edit.rs) — The entry point: the program starts here.
- struct `LatticeStore` (src/compat.rs) — A data shape named LatticeStore.
- function `LatticeStore::open` (src/compat.rs) — Function LatticeStore::open takes path: P and gives back io::Result<LatticeStore>.
- function `core` (src/compat.rs) — Access to the underlying core, for servers that want the transactional interface alongside the lattice surface.
- function `delete_scroll` (src/compat.rs) — Function delete_scroll takes &mut self, coord: Coordinate and gives back nothing; it does its work on what it is given.
- function `has_scroll` (src/compat.rs) — Function has_scroll takes &self, coord: &Coordinate and gives back bool.
- function `is_dirty` (src/compat.rs) — Function is_dirty takes no inputs beyond the value it is called on, and gives back bool.
- function `path` (src/compat.rs) — Function path takes no inputs beyond the value it is called on, and gives back &Path.
- function `pending_changes` (src/compat.rs) — Function pending_changes takes no inputs beyond the value it is called on, and gives back usize.
- function `populated_coordinates` (src/compat.rs) — Function populated_coordinates takes no inputs beyond the value it is called on, and gives back Vec<Coordinate>.
- function `read_scroll` (src/compat.rs) — Function read_scroll takes &self, coord: &Coordinate and gives back Option<String>.
- function `revert` (src/compat.rs) — Function revert takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is
- function `save` (src/compat.rs) — Commits the staged changes as one transaction, then checkpoints — after `save`, the base file at `path()` is the complet
- function `scroll_count` (src/compat.rs) — Function scroll_count takes no inputs beyond the value it is called on, and gives back usize.
- function `to_phext_bytes` (src/compat.rs) — The merged document (base + committed overlay + staged), serialized — `MappedLattice::to_phext_bytes`.
- function `write_scroll` (src/compat.rs) — Function write_scroll takes &mut self, coord: Coordinate, content: String and gives back bool.
- struct `BaseLattice` (src/core.rs) — The read-only memory-mapped base lattice.
- function `BaseLattice::open` (src/core.rs) — Function BaseLattice::open takes path: &Path and gives back io::Result<BaseLattice>.
- struct `Core` (src/core.rs) — The synchronous engine.
- function `Core::open` (src/core.rs) — Function Core::open takes base_path: &Path and gives back io::Result<Core>.
- function `Core::seq` (src/core.rs) — Function Core::seq takes no inputs beyond the value it is called on, and gives back u64.
- enum `Op` (src/core.rs) — One write inside a transaction.
- struct `Shared` (src/core.rs) — Shared, snapshot-able read state.
- function `Shared::snapshot` (src/core.rs) — Function Shared::snapshot takes no inputs beyond the value it is called on, and gives back Snapshot.
- struct `Snapshot` (src/core.rs) — A frozen read view at one sequence number.
- function `Snapshot::seq` (src/core.rs) — Function Snapshot::seq takes no inputs beyond the value it is called on, and gives back u64.
- struct `Txn` (src/core.rs) — A multi-scroll transaction: a set of ops applied atomically or not at all.
- function `Txn::new` (src/core.rs) — Function Txn::new takes no inputs beyond the value it is called on, and gives back Txn.
- function `base_bytes` (src/core.rs) — The raw bytes of the base document under this snapshot.
- function `base_path` (src/core.rs) — Function base_path takes no inputs beyond the value it is called on, and gives back &Path.
- function `bytes` (src/core.rs) — Function bytes takes no inputs beyond the value it is called on, and gives back &[u8].
- function `checkpoint` (src/core.rs) — Folds the overlay into the base file (temp + sync + rename + directory sync), remaps the base, and restarts the WAL with only its meta scroll.
- function `checkpoint_seq` (src/core.rs) — Function checkpoint_seq takes no inputs beyond the value it is called on, and gives back u64.
- function `commit_batch` (src/core.rs) — Commits a batch of transactions: all of them are serialized into the WAL, written as one suffix, and synced ONCE; only then is each transaction applied, in order, one overlay swap per transaction.
- function `contains` (src/core.rs) — Function contains takes &self, coord: &Coordinate and gives back bool.
- function `coordinates` (src/core.rs) — Every populated coordinate in this snapshot, ascending.
- function `delete` (src/core.rs) — Function delete takes mut self, coord: Coordinate and gives back Txn.
- function `get` (src/core.rs) — Function get takes &self, coord: &Coordinate and gives back Option<String>.
- function `is_empty` (src/core.rs) — Function is_empty takes no inputs beyond the value it is called on, and gives back bool.
- function `read_scroll` (src/core.rs) — Function read_scroll takes &self, coord: &Coordinate and gives back Option<String>.
- function `seq` (src/core.rs) — Function seq takes no inputs beyond the value it is called on, and gives back u64.
- function `set` (src/core.rs) — Function set takes mut self, coord: Coordinate, content: &str and gives back Txn.
- function `wal_path` (src/core.rs) — Function wal_path takes no inputs beyond the value it is called on, and gives back &Path.
- enum `Dimension` (src/index.rs) — The nine phext dimensions, in address order.
- struct `LatticeIndex` (src/index.rs) — The lattice index: maps every populated coordinate to its byte span.
- function `LatticeIndex::build` (src/index.rs) — Build an index by scanning the raw phext bytes once.
- struct `ScrollSpan` (src/index.rs) — A scroll's location within the raw phext byte buffer.
- function `ScrollSpan::len` (src/index.rs) — Function ScrollSpan::len takes no inputs beyond the value it is called on, and gives back usize.
- function `buffer_len` (src/index.rs) — Total buffer size.
- function `contains` (src/index.rs) — Check if a coordinate has content.
- function `coordinates` (src/index.rs) — All populated coordinates in document order.
- function `coordinates_matching` (src/index.rs) — All populated coordinates matching a predicate — the lattice navigator's workhorse.
- function `dimension_extent` (src/index.rs) — The distinct values a dimension takes across all populated coordinates, sorted.
- function `dimension_value` (src/index.rs) — The value of one dimension of a coordinate.
- function `from_entries` (src/index.rs) — Build from (coordinate, span) pairs, e.g.
- function `get` (src/index.rs) — Look up the byte span for a coordinate.
- function `is_empty` (src/index.rs) — Function is_empty takes no inputs beyond the value it is called on, and gives back bool.
- function `next_populated` (src/index.rs) — The next populated coordinate after `coord`, if any.
- function `prev_populated` (src/index.rs) — The previous populated coordinate before `coord`, if any.
- function `scroll_count` (src/index.rs) — Number of populated scrolls.
- function `spans` (src/index.rs) — Spans in document order, parallel to `coordinates()`.
- enum `Error` (src/lib.rs) — Errors from the async interface.
- struct `HoloPhext` (src/lib.rs) — An open HoloPhext store: an async handle over one `Core`.
- function `HoloPhext::open` (src/lib.rs) — Opens (creating if needed) the store whose base document is at `base_path`; the WAL lives beside it.
- type `Result` (src/lib.rs) — A type name, Result, that stands for another type.
- struct `Update` (src/lib.rs) — One committed transaction, dispatched to subscribers in order.
- function `checkpoint` (src/lib.rs) — Folds all committed state into the base document and restarts the WAL (see `Core::checkpoint`).
- function `commit` (src/lib.rs) — Commits one transaction atomically; resolves with its sequence number once the WAL write is synced and the transaction i
- re-export `core::{Core, Op, Snapshot, Txn}` (src/lib.rs) — A re-export: core::{Core, Op, Snapshot, Txn} is made reachable through this module, so other code can use it from here.
- re-export `index::{Dimension, LatticeIndex, ScrollSpan}` (src/lib.rs) — A re-export: index::{Dimension, LatticeIndex, ScrollSpan} is made reachable through this module, so other code can use i
- function `read` (src/lib.rs) — Reads one scroll from a fresh snapshot.
- function `seq` (src/lib.rs) — The latest committed sequence.
- function `snapshot` (src/lib.rs) — A frozen read view at the current sequence.
- function `subscribe` (src/lib.rs) — Subscribes to committed updates, in commit order.

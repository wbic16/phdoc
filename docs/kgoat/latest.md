# kgoat — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 95). Project root: /home/wbic16/phdoc-sweep/proj/kgoat/. Scanned at commit caa81e669ba114570f845fea796dc19ebf0644b1. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 1.

- libphext [dependencies] — 0.6.4

Total packages in the lock file, this one included: 3.


---

## Authors

The git log holds 2 commits.

The author of record is **Will Bickford** — 2 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Emise — 1 commit
- Will Bickford (his own identities) — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 2 files, 925 lines total, 128 blank, 1 comment, 796 code
- Rust: 6 files, 1034 lines total, 63 blank, 142 comment, 829 code
- TOML: 1 files, 11 lines total, 1 blank, 0 comment, 10 code

All languages: 9 files, 1970 lines total, 1635 lines of code. Functions read: 30.

Largest files:

- SPEC.md — 879 lines
- src/scroll.rs — 318 lines
- src/builder.rs — 234 lines
- src/schema.rs — 212 lines
- tests/builder_tests.rs — 151 lines

Largest functions, by span in lines:

- parse_node in src/scroll.rs — about 147 lines
- parse_edge in src/scroll.rs — about 81 lines
- build in src/builder.rs — about 79 lines
- render_index in src/builder.rs — about 77 lines
- main in src/main.rs — about 75 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 7 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 25 of 33 (76%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/kgoat.git
- Branch: exo
- Head commit: caa81e669ba114570f845fea796dc19ebf0644b1
- Working tree: 1 changed paths

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- type `NodeId` (src/builder.rs:18) — A node's identity: (home phext, coordinate) — nothing else (SPEC §2.1).
- struct `HomeInfo` (src/builder.rs:22) — Per-home input facts, carried into the generated Home scroll.
- struct `RegistryEntry` (src/builder.rs:32) — One assay-registry entry (SPEC §5.1): one per claim-bearing unit.
- struct `Graph` (src/builder.rs:43) — The folded graph: parsed nodes, schema violations, and the derived surfaces, all calculated from the scrolls.
- function `build` (src/builder.rs:68) — Fold a federation of phexts into a graph.
- function `render_index` (src/builder.rs:158) — Render the generated index as a phext (SPEC §2.3): a Home scroll with the rollup and the builder's stamp, and one scroll per derived surface.
- entry point `main` (src/main.rs:33) — The entry point: the program starts here.
- command `--out` (src/main.rs:41) — A command named --out.
- command `--stamp` (src/main.rs:45) — A command named --stamp.
- const `NODE_KINDS` (src/schema.rs:9) — Node kinds (SPEC §1.2).
- function `is_node_kind` (src/schema.rs:14) — Is `kind` a v1 node kind?
- const `EDGE_TYPES` (src/schema.rs:21) — Edge types (SPEC §1.3).
- function `is_edge_type` (src/schema.rs:34) — Is `ty` a v1 edge type?
- const `CLAIM_BEARING_REQUIRED` (src/schema.rs:44) — Edge types whose existence asserts something about the world (SPEC §5.1).
- const `CLAIM_BEARING_ALLOWED` (src/schema.rs:45) — A fixed value named CLAIM_BEARING_ALLOWED, set to ["derived_from"].
- function `assay_required` (src/schema.rs:48) — Does an edge of type `ty` require a status/basis pair?
- function `assay_permitted` (src/schema.rs:53) — May an edge of type `ty` carry a status/basis pair at all?
- function `max_weight` (src/schema.rs:59) — Maximum weight for an edge type.
- enum `Status` (src/schema.rs:70) — Claim statuses (SPEC §5.1), inherited from the Minimum Viable Oracle discipline whole.
- function `Status::parse` (src/schema.rs:78) — Function Status::parse takes s: &str and gives back Option<Status>.
- function `name` (src/schema.rs:87) — Function name takes no inputs beyond the value it is called on, and gives back &'static str.
- enum `Basis` (src/schema.rs:100) — Claim bases (SPEC §5.1).
- function `Basis::parse` (src/schema.rs:112) — Function Basis::parse takes s: &str and gives back Option<Basis>.
- function `name` (src/schema.rs:125) — Function name takes no inputs beyond the value it is called on, and gives back &'static str.
- function `status_legal_for_basis` (src/schema.rs:142) — The basis→status legality table (SPEC §5.1, MVO discipline): no VERIFIED receipt, no True; a refuted claim is False/REFU
- function `transition_legal` (src/schema.rs:161) — Legal status transitions (SPEC §2.4, the MVO legal edges).
- function `coordinate_well_formed` (src/schema.rs:177) — Is `s` a well-formed phext coordinate string (`a.b.c/d.e.f/g.h.i`, each component ≥ 1)? Coordinates are content, not key
- function `hop_stamp_well_formed` (src/schema.rs:195) — Is `s` a well-formed hop stamp (SPEC §2.2): an ISO date `YYYY-MM-DD`, optionally followed by `T` and a time.
- struct `Assay` (src/scroll.rs:27) — A status/basis/evidence triple — the assay record of a claim-bearing unit (SPEC §5.1) — with the hop stamp of when the g
- struct `Edge` (src/scroll.rs:37) — One stored outbound edge (SPEC §2.2).
- struct `Transition` (src/scroll.rs:52) — A recorded status transition (SPEC §2.4).
- struct `Node` (src/scroll.rs:62) — A parsed node scroll.
- function `parse_node` (src/scroll.rs:172) — Parse one node scroll.

---

## type — NodeId

Found in src/builder.rs at line 18.

Signature: `pub type NodeId = (String, String);`

What it does: A node's identity: (home phext, coordinate) — nothing else (SPEC §2.1).

(Where these words came from: doc comment.)

---

## struct — HomeInfo

Found in src/builder.rs at line 22.

Signature: `pub struct HomeInfo {`

What it does: Per-home input facts, carried into the generated Home scroll.

(Where these words came from: doc comment.)

---

## struct — RegistryEntry

Found in src/builder.rs at line 32.

Signature: `pub struct RegistryEntry {`

What it does: One assay-registry entry (SPEC §5.1): one per claim-bearing unit.

(Where these words came from: doc comment.)

---

## struct — Graph

Found in src/builder.rs at line 43.

Signature: `pub struct Graph {`

What it does: The folded graph: parsed nodes, schema violations, and the derived surfaces, all calculated from the scrolls.

(Where these words came from: doc comment.)

---

## function — build

Found in src/builder.rs at line 68.

Signature: `pub fn build(homes: &[(String, String)]) -> Graph {`

What it does: Fold a federation of phexts into a graph. `homes` is (home name, phext text) pairs. A scroll is a node scroll iff its first content line opens with `node · `; every other scroll is material the graph sits beside (SPEC §2.5), counted but not parsed.

(Where these words came from: doc comment.)

---

## function — render_index

Found in src/builder.rs at line 158.

Signature: `pub fn render_index(g: &Graph, stamp: &str) -> String {`

What it does: Render the generated index as a phext (SPEC §2.3): a Home scroll with the rollup and the builder's stamp, and one scroll per derived surface. `stamp` is the build's hop stamp (an ISO datetime string).

(Where these words came from: doc comment.)

---

## entry point — main

Found in src/main.rs at line 33.

Signature: `fn main() -> ExitCode {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: signature.)

---

## command — --out

Found in src/main.rs at line 41.

Signature: `command "--out"`

What it does: A command named --out. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — --stamp

Found in src/main.rs at line 45.

Signature: `command "--stamp"`

What it does: A command named --stamp. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## const — NODE_KINDS

Found in src/schema.rs at line 9.

Signature: `pub const NODE_KINDS: [&str; 8] = [`

What it does: Node kinds (SPEC §1.2). Eight kinds, each supplied by a seed corpus. There is deliberately no `person` kind: KGOAT is for ideas, not people (D6, SPEC §5.2). A scroll claiming kind `person` is non-conformant.

(Where these words came from: doc comment.)

---

## function — is_node_kind

Found in src/schema.rs at line 14.

Signature: `pub fn is_node_kind(kind: &str) -> bool {`

What it does: Is `kind` a v1 node kind?

(Where these words came from: doc comment.)

---

## const — EDGE_TYPES

Found in src/schema.rs at line 21.

Signature: `pub const EDGE_TYPES: [&str; 9] = [`

What it does: Edge types (SPEC §1.3). Every type below already exists in a seed corpus. `made` is NOT an edge type: it is attribution recorded on a work's scroll (SPEC §1.2), because there is no person node for the edge to leave from.

(Where these words came from: doc comment.)

---

## function — is_edge_type

Found in src/schema.rs at line 34.

Signature: `pub fn is_edge_type(ty: &str) -> bool {`

What it does: Is `ty` a v1 edge type?

(Where these words came from: doc comment.)

---

## const — CLAIM_BEARING_REQUIRED

Found in src/schema.rs at line 44.

Signature: `pub const CLAIM_BEARING_REQUIRED: [&str; 2] = ["asserts", "contradicts"];`

What it does: Edge types whose existence asserts something about the world (SPEC §5.1). A claim-bearing edge MUST carry exactly one status/basis pair and a real evidence pointer. `derived_from` MAY carry the pair (a claim's provenance is assayable); all other types MUST NOT carry it — a structural edge omits the pair rather than decorating itself with an assay it does not need (SPEC §2.2).

(Where these words came from: doc comment.)

---

## const — CLAIM_BEARING_ALLOWED

Found in src/schema.rs at line 45.

Signature: `pub const CLAIM_BEARING_ALLOWED: [&str; 1] = ["derived_from"];`

What it does: A fixed value named CLAIM_BEARING_ALLOWED, set to ["derived_from"].

(Where these words came from: signature.)

---

## function — assay_required

Found in src/schema.rs at line 48.

Signature: `pub fn assay_required(ty: &str) -> bool {`

What it does: Does an edge of type `ty` require a status/basis pair?

(Where these words came from: doc comment.)

---

## function — assay_permitted

Found in src/schema.rs at line 53.

Signature: `pub fn assay_permitted(ty: &str) -> bool {`

What it does: May an edge of type `ty` carry a status/basis pair at all?

(Where these words came from: doc comment.)

---

## function — max_weight

Found in src/schema.rs at line 59.

Signature: `pub fn max_weight(ty: &str) -> u8 {`

What it does: Maximum weight for an edge type. Weights are ordinal 0–3 (SPEC §1.3); `mentions` — the weakest form — is capped at 1.

(Where these words came from: doc comment.)

---

## enum — Status

Found in src/schema.rs at line 70.

Signature: `pub enum Status {`

What it does: Claim statuses (SPEC §5.1), inherited from the Minimum Viable Oracle discipline whole.

(Where these words came from: doc comment.)

---

## function — Status::parse

Found in src/schema.rs at line 78.

Signature: `pub fn parse(s: &str) -> Option<Status> {`

What it does: Function Status::parse takes s: &str and gives back Option<Status>.

(Where these words came from: signature.)

---

## function — name

Found in src/schema.rs at line 87.

Signature: `pub fn name(&self) -> &'static str {`

What it does: Function name takes no inputs beyond the value it is called on, and gives back &'static str.

(Where these words came from: signature.)

---

## enum — Basis

Found in src/schema.rs at line 100.

Signature: `pub enum Basis {`

What it does: Claim bases (SPEC §5.1). The basis constrains which statuses a claim-bearing unit may take — see [`status_legal_for_basis`].

(Where these words came from: doc comment.)

---

## function — Basis::parse

Found in src/schema.rs at line 112.

Signature: `pub fn parse(s: &str) -> Option<Basis> {`

What it does: Function Basis::parse takes s: &str and gives back Option<Basis>.

(Where these words came from: signature.)

---

## function — name

Found in src/schema.rs at line 125.

Signature: `pub fn name(&self) -> &'static str {`

What it does: Function name takes no inputs beyond the value it is called on, and gives back &'static str.

(Where these words came from: signature.)

---

## function — status_legal_for_basis

Found in src/schema.rs at line 142.

Signature: `pub fn status_legal_for_basis(status: Status, basis: Basis) -> bool {`

What it does: The basis→status legality table (SPEC §5.1, MVO discipline): no VERIFIED receipt, no True; a refuted claim is False/REFUTED; attributed and open material is believed by no one yet (Unknown); a construction is Fiction.

(Where these words came from: doc comment.)

---

## function — transition_legal

Found in src/schema.rs at line 161.

Signature: `pub fn transition_legal(from: Status, to: Status) -> bool {`

What it does: Legal status transitions (SPEC §2.4, the MVO legal edges). Every transition is recorded as a history event with its evidence pointer. There is no transition into True without a receipt, and Fiction never goes directly to True.

(Where these words came from: doc comment.)

---

## function — coordinate_well_formed

Found in src/schema.rs at line 177.

Signature: `pub fn coordinate_well_formed(s: &str) -> bool {`

What it does: Is `s` a well-formed phext coordinate string (`a.b.c/d.e.f/g.h.i`, each component ≥ 1)? Coordinates are content, not keys (SPEC §2.1); this checks form only.

(Where these words came from: doc comment.)

---

## function — hop_stamp_well_formed

Found in src/schema.rs at line 195.

Signature: `pub fn hop_stamp_well_formed(s: &str) -> bool {`

What it does: Is `s` a well-formed hop stamp (SPEC §2.2): an ISO date `YYYY-MM-DD`, optionally followed by `T` and a time. The graph records when *it* learned a thing; hop stamps are the graph's own history (SPEC §5.2).

(Where these words came from: doc comment.)

---

## struct — Assay

Found in src/scroll.rs at line 27.

Signature: `pub struct Assay {`

What it does: A status/basis/evidence triple — the assay record of a claim-bearing unit (SPEC §5.1) — with the hop stamp of when the graph recorded it.

(Where these words came from: doc comment.)

---

## struct — Edge

Found in src/scroll.rs at line 37.

Signature: `pub struct Edge {`

What it does: One stored outbound edge (SPEC §2.2). Edges are stored exactly once, in the source node's scroll; inbound edges are derived by the builder.

(Where these words came from: doc comment.)

---

## struct — Transition

Found in src/scroll.rs at line 52.

Signature: `pub struct Transition {`

What it does: A recorded status transition (SPEC §2.4). History is kept, not overwritten; the present is the builder's fold over it.

(Where these words came from: doc comment.)

---

## struct — Node

Found in src/scroll.rs at line 62.

Signature: `pub struct Node {`

What it does: A parsed node scroll. Identity is placement: `coord` is the scroll's coordinate inside its home phext, supplied by the caller (SPEC §2.1).

(Where these words came from: doc comment.)

---

## function — parse_node

Found in src/scroll.rs at line 172.

Signature: `pub fn parse_node(coord: &str, text: &str) -> Result<Node, Vec<String>> {`

What it does: Parse one node scroll. `coord` is the scroll's coordinate in its home phext. On failure, returns every violation found in the scroll.

(Where these words came from: doc comment.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/kgoat/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

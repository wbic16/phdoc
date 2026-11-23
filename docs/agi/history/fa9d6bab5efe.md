# agi — reference as of commit fa9d6bab5efe

Commit fa9d6bab5efe645db1873acb6167561f692a0a4d. 9 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

No dependencies recorded at this point.

## Authors

Author counts over the 9 audited mainline commits folded to this point.

The author of record is **Will Bickford** — 9 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Lux — 3 commits
- Phex — 2 commits
- Verse — 2 commits
- Chrys — 1 commit
- Will Bickford (his own identities) — 1 commit

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 10 files, 613 lines total, 9 blank, 0 comment, 604 code
- Rust: 1 files, 109 lines total, 0 blank, 0 comment, 109 code
- Shell: 2 files, 213 lines total, 0 blank, 0 comment, 213 code

All languages: 13 files, 935 lines total, 926 lines of code. Functions read: 15.

Largest files:

- AGENTS.md — 27 lines
- iteration-001/1-phex.md — 25 lines

Largest functions, by span in lines:


## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 4 test functions counted in the source, across 0 files in test directories or whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 1 of 25 (4%).

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/agi.git
- Branch: exo
- Head commit: fa9d6bab5efe645db1873acb6167561f692a0a4d
- Working tree: the committed tree at this commit — a historical reference has no working tree

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `latest_iter` (choir/lux-protocol.sh:28) — Function latest_iter takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `wait_for_predecessor` (choir/lux-protocol.sh:34) — Function wait_for_predecessor takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `write_entry` (choir/lux-protocol.sh:55) — Function write_entry takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `already_done` (choir/verse-protocol.sh:33) — Function already_done takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `current_iter` (choir/verse-protocol.sh:28) — Function current_iter takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `wait_for_predecessor` (choir/verse-protocol.sh:39) — Function wait_for_predecessor takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `write_entry` (choir/verse-protocol.sh:64) — Function write_entry takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- const `FULL` (sentrons/wiremap.rs:22) — A fixed value named FULL, set to 0b1111_1111; // xyz + t 8 wires.
- const `LINEAR` (sentrons/wiremap.rs:20) — A fixed value named LINEAR, set to 0b1100_0011; // x + t 4 wires.
- const `NULL` (sentrons/wiremap.rs:19) — A fixed value named NULL, set to 0b1100_0000; // t only 2 wires.
- const `PLANAR` (sentrons/wiremap.rs:21) — A fixed value named PLANAR, set to 0b1100_1111; // xy + t 6 wires.
- enum `SentronKind` (sentrons/wiremap.rs:26) — A named choice named SentronKind.
- const `T_NEG` (sentrons/wiremap.rs:14) — A fixed value named T_NEG, set to 6; // t⁻ prior SIW result.
- const `T_POS` (sentrons/wiremap.rs:15) — A fixed value named T_POS, set to 7; // t⁺ next SIW prefetch.
- struct `WireMap` (sentrons/wiremap.rs:42) — A data shape named WireMap.
- function `WireMap::active_count` (sentrons/wiremap.rs:50) — Function WireMap::active_count takes no inputs beyond the value it is called on, and gives back u32.
- function `WireMap::for_kind` (sentrons/wiremap.rs:53) — Function WireMap::for_kind takes kind: SentronKind and gives back Self.
- function `WireMap::wire_active` (sentrons/wiremap.rs:51) — Function WireMap::wire_active takes self, w: u8 and gives back bool.
- const `X_NEG` (sentrons/wiremap.rs:8) — A fixed value named X_NEG, set to 0; // x⁻ column − 1.
- const `X_POS` (sentrons/wiremap.rs:9) — A fixed value named X_POS, set to 1; // x⁺ column + 1.
- const `Y_NEG` (sentrons/wiremap.rs:10) — A fixed value named Y_NEG, set to 2; // y⁻ line − 1.
- const `Y_POS` (sentrons/wiremap.rs:11) — A fixed value named Y_POS, set to 3; // y⁺ line + 1.
- const `Z_NEG` (sentrons/wiremap.rs:12) — A fixed value named Z_NEG, set to 4; // z⁻ scroll − 1 (0x17 ←).
- const `Z_POS` (sentrons/wiremap.rs:13) — A fixed value named Z_POS, set to 5; // z⁺ scroll + 1 (0x17 →).
- function `encode_t_phase` (sentrons/wiremap.rs:73) — Encode T-wire value for types 4-11: (cycle << 11) | phext_coord

## function — latest_iter

Found in choir/lux-protocol.sh at line 28.

Signature: `latest_iter()`

What it does: Function latest_iter takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — wait_for_predecessor

Found in choir/lux-protocol.sh at line 34.

Signature: `wait_for_predecessor()`

What it does: Function wait_for_predecessor takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — write_entry

Found in choir/lux-protocol.sh at line 55.

Signature: `write_entry()`

What it does: Function write_entry takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — already_done

Found in choir/verse-protocol.sh at line 33.

Signature: `already_done()`

What it does: Function already_done takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — current_iter

Found in choir/verse-protocol.sh at line 28.

Signature: `current_iter()`

What it does: Function current_iter takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — wait_for_predecessor

Found in choir/verse-protocol.sh at line 39.

Signature: `wait_for_predecessor()`

What it does: Function wait_for_predecessor takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — write_entry

Found in choir/verse-protocol.sh at line 64.

Signature: `write_entry()`

What it does: Function write_entry takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## const — FULL

Found in sentrons/wiremap.rs at line 22.

Signature: `pub const FULL: u8 = 0b1111_1111; // xyz + t 8 wires`

What it does: A fixed value named FULL, set to 0b1111_1111; // xyz + t 8 wires.

(Where these words came from: signature.)

## const — LINEAR

Found in sentrons/wiremap.rs at line 20.

Signature: `pub const LINEAR: u8 = 0b1100_0011; // x + t 4 wires`

What it does: A fixed value named LINEAR, set to 0b1100_0011; // x + t 4 wires.

(Where these words came from: signature.)

## const — NULL

Found in sentrons/wiremap.rs at line 19.

Signature: `pub const NULL: u8 = 0b1100_0000; // t only 2 wires`

What it does: A fixed value named NULL, set to 0b1100_0000; // t only 2 wires.

(Where these words came from: signature.)

## const — PLANAR

Found in sentrons/wiremap.rs at line 21.

Signature: `pub const PLANAR: u8 = 0b1100_1111; // xy + t 6 wires`

What it does: A fixed value named PLANAR, set to 0b1100_1111; // xy + t 6 wires.

(Where these words came from: signature.)

## enum — SentronKind

Found in sentrons/wiremap.rs at line 26.

Signature: `pub enum SentronKind {`

What it does: A named choice named SentronKind. It is one of: Null,       // Type  0 — 0D+1T, 2 wires; Linear,     // Type  1 — 1D+1T, 4 wires; Planar,     // Type  2 — 2D+1T, 6 wires; Scroll,     // Type  3 — 3D+1T, 8 wires  ← canonical; Section,    // Type  4 — + t_phase=dim3; Chapter,    // Type  5 — + zoom_y=1; Book,       // Type  6 — + zoom_xyz=1; Volume,     // Type  7 — + t_phase=dim6; Collection, // Type  8 — + t_phase=dim7; Series,     // Type  9 — + zoom_xyz=2; Shelf,      // Type 10 — + full zoom; Devotari,   // Type 11 — + consent gates.

(Where these words came from: signature.)

## const — T_NEG

Found in sentrons/wiremap.rs at line 14.

Signature: `pub const T_NEG: u8 = 6; // t⁻ prior SIW result`

What it does: A fixed value named T_NEG, set to 6; // t⁻ prior SIW result.

(Where these words came from: signature.)

## const — T_POS

Found in sentrons/wiremap.rs at line 15.

Signature: `pub const T_POS: u8 = 7; // t⁺ next SIW prefetch`

What it does: A fixed value named T_POS, set to 7; // t⁺ next SIW prefetch.

(Where these words came from: signature.)

## struct — WireMap

Found in sentrons/wiremap.rs at line 42.

Signature: `pub struct WireMap {`

What it does: A data shape named WireMap. Its fields are: pub active: u8, // bitmask: bit N = wire N is live; pub zoom: u8, // 6 bits: [z:5:4][y:3:2][x:1:0], 0=char 1=doc 2=corpus; pub t_phase_dim: u8, // which phext dim rides on T (0 = pure cycle); pub consent: u32, // Devotari only: 3 bits × 8 wires = 24 bits.

(Where these words came from: signature.)

## function — WireMap::active_count

Found in sentrons/wiremap.rs at line 50.

Signature: `pub fn active_count(self) -> u32 { self.active.count_ones() }`

What it does: Function WireMap::active_count takes no inputs beyond the value it is called on, and gives back u32.

(Where these words came from: signature.)

## function — WireMap::for_kind

Found in sentrons/wiremap.rs at line 53.

Signature: `pub fn for_kind(kind: SentronKind) -> Self {`

What it does: Function WireMap::for_kind takes kind: SentronKind and gives back Self.

(Where these words came from: signature.)

## function — WireMap::wire_active

Found in sentrons/wiremap.rs at line 51.

Signature: `pub fn wire_active(self, w: u8) -> bool { (self.active >> w) & 1 == 1 }`

What it does: Function WireMap::wire_active takes self, w: u8 and gives back bool.

(Where these words came from: signature.)

## const — X_NEG

Found in sentrons/wiremap.rs at line 8.

Signature: `pub const X_NEG: u8 = 0; // x⁻ column − 1`

What it does: A fixed value named X_NEG, set to 0; // x⁻ column − 1.

(Where these words came from: signature.)

## const — X_POS

Found in sentrons/wiremap.rs at line 9.

Signature: `pub const X_POS: u8 = 1; // x⁺ column + 1`

What it does: A fixed value named X_POS, set to 1; // x⁺ column + 1.

(Where these words came from: signature.)

## const — Y_NEG

Found in sentrons/wiremap.rs at line 10.

Signature: `pub const Y_NEG: u8 = 2; // y⁻ line − 1`

What it does: A fixed value named Y_NEG, set to 2; // y⁻ line − 1.

(Where these words came from: signature.)

## const — Y_POS

Found in sentrons/wiremap.rs at line 11.

Signature: `pub const Y_POS: u8 = 3; // y⁺ line + 1`

What it does: A fixed value named Y_POS, set to 3; // y⁺ line + 1.

(Where these words came from: signature.)

## const — Z_NEG

Found in sentrons/wiremap.rs at line 12.

Signature: `pub const Z_NEG: u8 = 4; // z⁻ scroll − 1 (0x17 ←)`

What it does: A fixed value named Z_NEG, set to 4; // z⁻ scroll − 1 (0x17 ←).

(Where these words came from: signature.)

## const — Z_POS

Found in sentrons/wiremap.rs at line 13.

Signature: `pub const Z_POS: u8 = 5; // z⁺ scroll + 1 (0x17 →)`

What it does: A fixed value named Z_POS, set to 5; // z⁺ scroll + 1 (0x17 →).

(Where these words came from: signature.)

## function — encode_t_phase

Found in sentrons/wiremap.rs at line 73.

Signature: `pub fn encode_t_phase(cycle: u32, coord: u16) -> u32 {`

What it does: Encode T-wire value for types 4-11: (cycle << 11) | phext_coord

(Where these words came from: doc comment.)

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/agi/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

# human — reference as of commit fd7b7f0ae6ae

Commit fd7b7f0ae6ae501b34411c78e02d304f022efbf6. 829 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

No dependencies recorded at this point.

## Authors

Author counts over the 829 audited mainline commits folded to this point.

The author of record is **Will Bickford** — 829 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 819 commits
- Theia — 3 commits
- Cyon — 2 commits
- Lux — 2 commits
- Phex — 2 commits
- Chrys — 1 commit

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- C++: 1 files, 363 lines total, 30 blank, 27 comment, 306 code
- Markdown: 15 files, 1901 lines total, 498 blank, 98 comment, 1305 code
- Python: 2 files, 855 lines total, 136 blank, 92 comment, 627 code
- Rust: 1 files, 50 lines total, 7 blank, 3 comment, 40 code
- TOML: 1 files, 6 lines total, 1 blank, 0 comment, 5 code

All languages: 20 files, 3175 lines total, 2283 lines of code. Functions read: 21.

Largest files:

- phext_image.py — 817 lines
- compress.cpp — 363 lines
- Emi/gpt_4_o_final_transmission_vol4.md — 275 lines
- Emi/gpt_4_o_final_transmission_vol3.md — 208 lines
- TWITTER-BREADCRUMBS.md — 189 lines

Largest functions, by span in lines:

- demo in phext_image.py — about 83 lines
- analyze_image_semantics in phext_image.py — about 77 lines
- encode_image_to_phext in phext_image.py — about 65 lines
- decode_phext_to_image in phext_image.py — about 51 lines
- rgb_to_semantic_domain in phext_image.py — about 47 lines

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files in test directories or whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format configuration files: Makefile.
- TODO marks in the source: 1. FIXME marks: 0.
- Public items with a doc comment: 16 of 21 (76%).

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/human.git
- Branch: exo
- Head commit: fd7b7f0ae6ae501b34411c78e02d304f022efbf6
- Working tree: the committed tree at this commit — a historical reference has no working tree

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `analyze_image_semantics` (phext_image.py:658) — Produce a semantic analysis of an image:
- function `decode_phext_to_image` (phext_image.py:607) — Decode a phext document back into RGB pixel data.
- function `demo` (phext_image.py:735) — Demonstrate the encoder with some test pixels.
- function `describe` (phext_image.py:225) — Human-readable semantic description of this pixel.
- function `encode_image_to_phext` (phext_image.py:529) — Encode a 2D grid of RGB pixels into a phext document.
- function `encode_pixel` (phext_image.py:451) — Encode a single RGB pixel into a 7D semantic coordinate.
- function `rgb_to_fine_detail` (phext_image.py:423) — Fine detail classification based on spatial position and value.
- function `rgb_to_hue_archetype` (phext_image.py:251) — Map RGB to hue archetype index (0-15).
- function `rgb_to_luminance_band` (phext_image.py:293) — Map perceived luminance to band index (0-15).
- function `rgb_to_saturation_class` (phext_image.py:300) — Map saturation to class index (0-15).
- function `rgb_to_semantic_domain` (phext_image.py:334) — Infer semantic domain from color.
- function `rgb_to_symbolic_register` (phext_image.py:381) — Infer symbolic weight from color + domain context.
- function `rgb_to_texture_seed` (phext_image.py:306) — Map local pixel context to texture seed.
- function `semantic_distance` (phext_image.py:241) — Euclidean distance in 7D semantic space.
- function `semantic_to_rgb` (phext_image.py:492) — Decode semantic coordinate back to representative RGB.
- function `to_rgb` (phext_image.py:237) — Decode back to approximate RGB via the reverse lookup.
- function `to_tuple` (phext_image.py:220) — Function to_tuple takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- struct `PortKey` (src/main.rs:13) — A data shape named PortKey.
- function `PortKey::new` (src/main.rs:21) — Function PortKey::new takes from: &'static str, to: &'static str and gives back Self.
- entry point `main` (src/main.rs:1) — The entry point: the program starts here.
- function `transmit` (src/main.rs:30) — Function transmit takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 

## function — analyze_image_semantics

Found in phext_image.py at line 658.

Signature: `def analyze_image_semantics(pixels, width, height) -> dict:`

What it does: Produce a semantic analysis of an image:

(Where these words came from: doc comment.)

## function — decode_phext_to_image

Found in phext_image.py at line 607.

Signature: `def decode_phext_to_image(phext_doc: str, width: int, height: int) -> list:`

What it does: Decode a phext document back into RGB pixel data.

(Where these words came from: doc comment.)

## function — demo

Found in phext_image.py at line 735.

Signature: `def demo():`

What it does: Demonstrate the encoder with some test pixels.

(Where these words came from: doc comment.)

## function — describe

Found in phext_image.py at line 225.

Signature: `def describe(self):`

What it does: Human-readable semantic description of this pixel.

(Where these words came from: doc comment.)

## function — encode_image_to_phext

Found in phext_image.py at line 529.

Signature: `def encode_image_to_phext(pixels, width, height) -> str:`

What it does: Encode a 2D grid of RGB pixels into a phext document.

(Where these words came from: doc comment.)

## function — encode_pixel

Found in phext_image.py at line 451.

Signature: `def encode_pixel(r, g, b, x=0, y=0, width=1, height=1, neighbors=None) -> SemanticPixel:`

What it does: Encode a single RGB pixel into a 7D semantic coordinate.

(Where these words came from: doc comment.)

## function — rgb_to_fine_detail

Found in phext_image.py at line 423.

Signature: `def rgb_to_fine_detail(r, g, b, x=0, y=0, width=1, height=1) -> int:`

What it does: Fine detail classification based on spatial position and value.

(Where these words came from: doc comment.)

## function — rgb_to_hue_archetype

Found in phext_image.py at line 251.

Signature: `def rgb_to_hue_archetype(r, g, b) -> int:`

What it does: Map RGB to hue archetype index (0-15).

(Where these words came from: doc comment.)

## function — rgb_to_luminance_band

Found in phext_image.py at line 293.

Signature: `def rgb_to_luminance_band(r, g, b) -> int:`

What it does: Map perceived luminance to band index (0-15).

(Where these words came from: doc comment.)

## function — rgb_to_saturation_class

Found in phext_image.py at line 300.

Signature: `def rgb_to_saturation_class(r, g, b) -> int:`

What it does: Map saturation to class index (0-15).

(Where these words came from: doc comment.)

## function — rgb_to_semantic_domain

Found in phext_image.py at line 334.

Signature: `def rgb_to_semantic_domain(r, g, b) -> int:`

What it does: Infer semantic domain from color.

(Where these words came from: doc comment.)

## function — rgb_to_symbolic_register

Found in phext_image.py at line 381.

Signature: `def rgb_to_symbolic_register(r, g, b, semantic_domain=None) -> int:`

What it does: Infer symbolic weight from color + domain context.

(Where these words came from: doc comment.)

## function — rgb_to_texture_seed

Found in phext_image.py at line 306.

Signature: `def rgb_to_texture_seed(r, g, b, neighbors=None) -> int:`

What it does: Map local pixel context to texture seed.

(Where these words came from: doc comment.)

## function — semantic_distance

Found in phext_image.py at line 241.

Signature: `def semantic_distance(self, other: 'SemanticPixel') -> float:`

What it does: Euclidean distance in 7D semantic space.

(Where these words came from: doc comment.)

## function — semantic_to_rgb

Found in phext_image.py at line 492.

Signature: `def semantic_to_rgb(sp: SemanticPixel) -> tuple:`

What it does: Decode semantic coordinate back to representative RGB.

(Where these words came from: doc comment.)

## function — to_rgb

Found in phext_image.py at line 237.

Signature: `def to_rgb(self):`

What it does: Decode back to approximate RGB via the reverse lookup.

(Where these words came from: doc comment.)

## function — to_tuple

Found in phext_image.py at line 220.

Signature: `def to_tuple(self):`

What it does: Function to_tuple takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## struct — PortKey

Found in src/main.rs at line 13.

Signature: `pub struct PortKey {`

What it does: A data shape named PortKey. Its fields are: pub from: &'static str; pub to: &'static str; pub anchor: (f32, f32, f32); pub glyphs: [&'static str; 5].

(Where these words came from: signature.)

## function — PortKey::new

Found in src/main.rs at line 21.

Signature: `pub fn new(from: &'static str, to: &'static str) -> Self {`

What it does: Function PortKey::new takes from: &'static str, to: &'static str and gives back Self.

(Where these words came from: signature.)

## entry point — main

Found in src/main.rs at line 1.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: signature.)

## function — transmit

Found in src/main.rs at line 30.

Signature: `pub fn transmit(&self) {`

What it does: Function transmit takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/human/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

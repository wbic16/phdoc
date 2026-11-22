# human — reference as of commit ffff91e9ff47

Commit ffff91e9ff47ed67d7916ac7e6a71169264a60b4. 828 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/human/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

## Dependencies

No dependencies recorded at this point.

## Authors

The author of record is **Will Bickford** — 828 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 819 commits
- Theia — 3 commits
- Cyon — 2 commits
- Lux — 2 commits
- Phex — 2 commits

## Complexity

- C++: 1 file, 363 lines total, 30 blank, 27 comment, 306 code
- Markdown: 15 files, 1901 lines total, 512 blank, 0 comment, 1389 code
- Python: 2 files, 855 lines total, 136 blank, 92 comment, 627 code
- Rust: 1 file, 50 lines total, 7 blank, 3 comment, 40 code
- TOML: 1 file, 6 lines total, 1 blank, 0 comment, 5 code

Declared RFCs at this point: none declared.

## Public methods and commands

- function `SemanticPixel.describe` (phext_image.py) — Human-readable semantic description of this pixel.
- function `SemanticPixel.semantic_distance` (phext_image.py) — Euclidean distance in 7D semantic space.
- function `SemanticPixel.to_rgb` (phext_image.py) — Decode back to approximate RGB via the reverse lookup.
- function `SemanticPixel.to_tuple` (phext_image.py) — Function SemanticPixel.to_tuple takes no inputs beyond the value it is called on.
- function `analyze_image_semantics` (phext_image.py) — Produce a semantic analysis of an image:
- function `decode_phext_to_image` (phext_image.py) — Decode a phext document back into RGB pixel data.
- function `demo` (phext_image.py) — Demonstrate the encoder with some test pixels.
- function `encode_image_to_phext` (phext_image.py) — Encode a 2D grid of RGB pixels into a phext document.
- function `encode_pixel` (phext_image.py) — Encode a single RGB pixel into a 7D semantic coordinate.
- function `rgb_to_fine_detail` (phext_image.py) — Fine detail classification based on spatial position and value.
- function `rgb_to_hue_archetype` (phext_image.py) — Map RGB to hue archetype index (0-15).
- function `rgb_to_luminance_band` (phext_image.py) — Map perceived luminance to band index (0-15).
- function `rgb_to_saturation_class` (phext_image.py) — Map saturation to class index (0-15).
- function `rgb_to_semantic_domain` (phext_image.py) — Infer semantic domain from color.
- function `rgb_to_symbolic_register` (phext_image.py) — Infer symbolic weight from color + domain context.
- function `rgb_to_texture_seed` (phext_image.py) — Map local pixel context to texture seed.
- function `semantic_to_rgb` (phext_image.py) — Decode semantic coordinate back to representative RGB.
- struct `PortKey` (src/main.rs) — A data shape named PortKey.
- function `PortKey::new` (src/main.rs) — Function PortKey::new takes from: &'static str, to: &'static str.
- function `PortKey::transmit` (src/main.rs) — Function PortKey::transmit takes no inputs beyond the value it is called on.
- entry point `main` (src/main.rs) — The entry point: the program starts here.

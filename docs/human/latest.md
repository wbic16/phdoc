# human — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 147). Scanned at commit db1e679c5128f48979bb9034a6b0133dc142f2fe. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Summary

Tessera is The Exocortex of 2130, where digital humans live.

(The opening of the project's README, quoted as written.)

---

## Dependencies

### Rust (cargo) — Cargo.toml

Direct dependencies: 1.

- libphext [dependencies] — 0.3.1


---

## Authors

The git log holds 1169 commits.

The author of record is **Will Bickford** — 1167 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 862 commits
- Emise — 279 commits
- Theia — 10 commits
- Verse — 5 commits
- Claude — 2 commits
- Cyon — 2 commits
- Lux — 2 commits
- Phex — 2 commits
- Aster — 1 commit
- Chrys — 1 commit
- Orin — 1 commit

Other authors, kept as themselves (not Exocortical cluster personas; never folded):

- ${NODE_NAME} <${node_email}> — 1 commit
- Cursor Agent <cursoragent@cursor.com> — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- C++: 1 file, 363 lines total, 30 blank, 27 comment, 306 code
- Markdown: 34 files, 6052 lines total, 1562 blank, 170 comment, 4320 code
- Python: 4 files, 1311 lines total, 193 blank, 101 comment, 1017 code
- Rust: 2 files, 157 lines total, 15 blank, 19 comment, 123 code
- TOML: 1 file, 8 lines total, 1 blank, 0 comment, 7 code

All languages: 42 files, 7891 lines total, 5773 lines of code. Functions read: 43.

Largest files:

- Earth.Human.ASI.Genome/Spec.md — 952 lines
- phext_image.py — 817 lines
- Earth.Human.ASI.Genome/fork-for-emi/WILL-VOICE-PROFILE.md — 622 lines
- Earth.Human.ASI.Genome/fork-for-emi/mirror-will-op-volumen-boot-v0.2.md — 514 lines
- quantum-time-sync/publish.py — 398 lines

Largest functions, by span in lines:

- build in quantum-time-sync/publish.py — about 174 lines
- demo in phext_image.py — about 83 lines
- analyze_image_semantics in phext_image.py — about 77 lines
- encode_image_to_phext in phext_image.py — about 65 lines
- decode_phext_to_image in phext_image.py — about 51 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format configuration files: Makefile.
- TODO marks in the source: 1. FIXME marks: 0.
- Public items with a doc comment: 21 of 40 (52%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/human
- Branch: exo
- Head commit: db1e679c5128f48979bb9034a6b0133dc142f2fe
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `SemanticPixel.to_tuple` (phext_image.py:220) — Function SemanticPixel.to_tuple takes no inputs beyond the value it is called on.
- function `SemanticPixel.describe` (phext_image.py:225) — Human-readable semantic description of this pixel.
- function `SemanticPixel.to_rgb` (phext_image.py:237) — Decode back to approximate RGB via the reverse lookup.
- function `SemanticPixel.semantic_distance` (phext_image.py:241) — Euclidean distance in 7D semantic space.
- function `rgb_to_hue_archetype` (phext_image.py:251) — Map RGB to hue archetype index (0-15).
- function `rgb_to_luminance_band` (phext_image.py:293) — Map perceived luminance to band index (0-15).
- function `rgb_to_saturation_class` (phext_image.py:300) — Map saturation to class index (0-15).
- function `rgb_to_texture_seed` (phext_image.py:306) — Map local pixel context to texture seed.
- function `rgb_to_semantic_domain` (phext_image.py:334) — Infer semantic domain from color.
- function `rgb_to_symbolic_register` (phext_image.py:381) — Infer symbolic weight from color + domain context.
- function `rgb_to_fine_detail` (phext_image.py:423) — Fine detail classification based on spatial position and value.
- function `encode_pixel` (phext_image.py:451) — Encode a single RGB pixel into a 7D semantic coordinate.
- function `semantic_to_rgb` (phext_image.py:492) — Decode semantic coordinate back to representative RGB.
- function `encode_image_to_phext` (phext_image.py:529) — Encode a 2D grid of RGB pixels into a phext document.
- function `decode_phext_to_image` (phext_image.py:607) — Decode a phext document back into RGB pixel data.
- function `analyze_image_semantics` (phext_image.py:658) — Produce a semantic analysis of an image:
- function `demo` (phext_image.py:735) — Demonstrate the encoder with some test pixels.
- function `ee_of_unix` (quantum-time-sync/publish.py:67) — Function ee_of_unix takes unix_ts.
- function `parse_kv` (quantum-time-sync/publish.py:73) — Function parse_kv takes line.
- function `read_lines` (quantum-time-sync/publish.py:82) — Function read_lines takes name.
- function `parse_register` (quantum-time-sync/publish.py:89) — Return (rounds, withheld).
- function `parse_keyed` (quantum-time-sync/publish.py:121) — Lines of elections/consensus/blend, keyed by their census= field.
- function `parse_drift` (quantum-time-sync/publish.py:133) — Drift atoms, with the superseded one withheld (same note rule).
- function `micro` (quantum-time-sync/publish.py:155) — Recorded value (3 decimals) -> exact integer micro-units.
- function `mcv_min_entropy` (quantum-time-sync/publish.py:160) — Most-common-value min-entropy estimate: -log2(p_max).
- function `measure` (quantum-time-sync/publish.py:169) — Function measure takes rounds.
- function `health` (quantum-time-sync/publish.py:191) — Function health takes cond_bytes.
- function `delta_lines` (quantum-time-sync/publish.py:207) — Function delta_lines takes rnd.
- function `fmt` (quantum-time-sync/publish.py:221) — Function fmt takes x, places=3.
- function `build` (quantum-time-sync/publish.py:225) — Function build takes no inputs beyond the value it is called on.
- function `parse` (resonate.py:14) — Function parse takes data.
- function `fmt` (resonate.py:25) — Function fmt takes c.
- function `toks` (resonate.py:26) — Function toks takes t.
- function `build` (resonate.py:28) — Function build takes paths.
- function `resonate` (resonate.py:38) — Function resonate takes text, docs, df, n=8, k1=1.2, b=0.75.
- entry point `main` (src/bin/check_toc.rs:90) — The entry point: the program starts here.
- entry point `main` (src/main.rs:1) — The entry point: the program starts here.
- struct `PortKey` (src/main.rs:13) — A data shape named PortKey.
- function `PortKey::new` (src/main.rs:21) — Function PortKey::new takes from: &'static str, to: &'static str.
- function `PortKey::transmit` (src/main.rs:30) — Function PortKey::transmit takes no inputs beyond the value it is called on.

---

## function — SemanticPixel.to_tuple

Found in phext_image.py at line 220.

Surface: public API.

Signature: `def to_tuple(self)`

What it does: Function SemanticPixel.to_tuple takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning (self.hue_archetype, self.luminance_band, self.s.

(Where these words came from: inferred.)

---

## function — SemanticPixel.describe

Found in phext_image.py at line 225.

Surface: public API.

Signature: `def describe(self)`

What it does: Human-readable semantic description of this pixel.

(Where these words came from: direct.)

---

## function — SemanticPixel.to_rgb

Found in phext_image.py at line 237.

Surface: public API.

Signature: `def to_rgb(self)`

What it does: Decode back to approximate RGB via the reverse lookup.

(Where these words came from: direct.)

---

## function — SemanticPixel.semantic_distance

Found in phext_image.py at line 241.

Surface: public API.

Signature: `def semantic_distance(self, other: 'SemanticPixel') -> float`

What it does: Euclidean distance in 7D semantic space.

(Where these words came from: direct.)

---

## function — rgb_to_hue_archetype

Found in phext_image.py at line 251.

Surface: public API.

Signature: `def rgb_to_hue_archetype(r, g, b) -> int`

What it does: Map RGB to hue archetype index (0-15).

(Where these words came from: direct.)

---

## function — rgb_to_luminance_band

Found in phext_image.py at line 293.

Surface: public API.

Signature: `def rgb_to_luminance_band(r, g, b) -> int`

What it does: Map perceived luminance to band index (0-15).

(Where these words came from: direct.)

---

## function — rgb_to_saturation_class

Found in phext_image.py at line 300.

Surface: public API.

Signature: `def rgb_to_saturation_class(r, g, b) -> int`

What it does: Map saturation to class index (0-15).

(Where these words came from: direct.)

---

## function — rgb_to_texture_seed

Found in phext_image.py at line 306.

Surface: public API.

Signature: `def rgb_to_texture_seed(r, g, b, neighbors=None) -> int`

What it does: Map local pixel context to texture seed.

(Where these words came from: direct.)

---

## function — rgb_to_semantic_domain

Found in phext_image.py at line 334.

Surface: public API.

Signature: `def rgb_to_semantic_domain(r, g, b) -> int`

What it does: Infer semantic domain from color.

(Where these words came from: direct.)

---

## function — rgb_to_symbolic_register

Found in phext_image.py at line 381.

Surface: public API.

Signature: `def rgb_to_symbolic_register(r, g, b, semantic_domain=None) -> int`

What it does: Infer symbolic weight from color + domain context.

(Where these words came from: direct.)

---

## function — rgb_to_fine_detail

Found in phext_image.py at line 423.

Surface: public API.

Signature: `def rgb_to_fine_detail(r, g, b, x=0, y=0, width=1, height=1) -> int`

What it does: Fine detail classification based on spatial position and value.

(Where these words came from: direct.)

---

## function — encode_pixel

Found in phext_image.py at line 451.

Surface: public API.

Signature: `def encode_pixel(r, g, b, x=0, y=0, width=1, height=1, neighbors=None) -> SemanticPixel`

What it does: Encode a single RGB pixel into a 7D semantic coordinate.

(Where these words came from: direct.)

---

## function — semantic_to_rgb

Found in phext_image.py at line 492.

Surface: public API.

Signature: `def semantic_to_rgb(sp: SemanticPixel) -> tuple`

What it does: Decode semantic coordinate back to representative RGB.

(Where these words came from: direct.)

---

## function — encode_image_to_phext

Found in phext_image.py at line 529.

Surface: public API.

Signature: `def encode_image_to_phext(pixels, width, height) -> str`

What it does: Encode a 2D grid of RGB pixels into a phext document.

(Where these words came from: direct.)

---

## function — decode_phext_to_image

Found in phext_image.py at line 607.

Surface: public API.

Signature: `def decode_phext_to_image(phext_doc: str, width: int, height: int) -> list`

What it does: Decode a phext document back into RGB pixel data.

(Where these words came from: direct.)

---

## function — analyze_image_semantics

Found in phext_image.py at line 658.

Surface: public API.

Signature: `def analyze_image_semantics(pixels, width, height) -> dict`

What it does: Produce a semantic analysis of an image:

(Where these words came from: direct.)

---

## function — demo

Found in phext_image.py at line 735.

Surface: public API.

Signature: `def demo()`

What it does: Demonstrate the encoder with some test pixels.

(Where these words came from: direct.)

---

## function — ee_of_unix

Found in quantum-time-sync/publish.py at line 67.

Surface: public API.

Signature: `def ee_of_unix(unix_ts)`

What it does: Function ee_of_unix takes unix_ts. Its return type is unannotated; the code shows it returning 0; (unix_ts - EE_ANCHOR_UNIX) // EE_SECONDS + 1.

(Where these words came from: inferred.)

---

## function — parse_kv

Found in quantum-time-sync/publish.py at line 73.

Surface: public API.

Signature: `def parse_kv(line)`

What it does: Function parse_kv takes line. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — read_lines

Found in quantum-time-sync/publish.py at line 82.

Surface: public API.

Signature: `def read_lines(name)`

What it does: Function read_lines takes name. Its return type is unannotated; the code shows it returning f.read().splitlines().

(Where these words came from: inferred.)

---

## function — parse_register

Found in quantum-time-sync/publish.py at line 89.

Surface: public API.

Signature: `def parse_register()`

What it does: Return (rounds, withheld). Each round: census ts, ee, [node dicts].

(Where these words came from: direct.)

---

## function — parse_keyed

Found in quantum-time-sync/publish.py at line 121.

Surface: public API.

Signature: `def parse_keyed(name, key="census")`

What it does: Lines of elections/consensus/blend, keyed by their census= field.

(Where these words came from: direct.)

---

## function — parse_drift

Found in quantum-time-sync/publish.py at line 133.

Surface: public API.

Signature: `def parse_drift()`

What it does: Drift atoms, with the superseded one withheld (same note rule).

(Where these words came from: direct.)

---

## function — micro

Found in quantum-time-sync/publish.py at line 155.

Surface: public API.

Signature: `def micro(value_str)`

What it does: Recorded value (3 decimals) -> exact integer micro-units.

(Where these words came from: direct.)

---

## function — mcv_min_entropy

Found in quantum-time-sync/publish.py at line 160.

Surface: public API.

Signature: `def mcv_min_entropy(values)`

What it does: Most-common-value min-entropy estimate: -log2(p_max).

(Where these words came from: direct.)

---

## function — measure

Found in quantum-time-sync/publish.py at line 169.

Surface: public API.

Signature: `def measure(rounds)`

What it does: Function measure takes rounds. Its return type is unannotated; the code shows it returning stats, per_round.

(Where these words came from: inferred.)

---

## function — health

Found in quantum-time-sync/publish.py at line 191.

Surface: public API.

Signature: `def health(cond_bytes)`

What it does: Function health takes cond_bytes. Its return type is unannotated; the code shows it returning {"bytes": n, "bits": bits, "ones": ones,.

(Where these words came from: inferred.)

---

## function — delta_lines

Found in quantum-time-sync/publish.py at line 207.

Surface: public API.

Signature: `def delta_lines(rnd)`

What it does: Function delta_lines takes rnd. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — fmt

Found in quantum-time-sync/publish.py at line 221.

Surface: public API.

Signature: `def fmt(x, places=3)`

What it does: Function fmt takes x, places=3. Its return type is unannotated; the code shows it returning "none" if x is None else ("%.*f" % (places, x)).

(Where these words came from: inferred.)

---

## function — build

Found in quantum-time-sync/publish.py at line 225.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — parse

Found in resonate.py at line 14.

Surface: public API.

Signature: `def parse(data)`

What it does: Function parse takes data. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — fmt

Found in resonate.py at line 25.

Surface: public API.

Signature: `def fmt(c): return "%d.%d.%d/%d.%d.%d/%d.%d.%d" % c`

What it does: Function fmt takes c. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — toks

Found in resonate.py at line 26.

Surface: public API.

Signature: `def toks(t): return re.findall(r"[a-z0-9']+", t.lower())`

What it does: Function toks takes t. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — build

Found in resonate.py at line 28.

Surface: public API.

Signature: `def build(paths)`

What it does: Function build takes paths. Its return type is unannotated; the code shows it returning docs, df.

(Where these words came from: inferred.)

---

## function — resonate

Found in resonate.py at line 38.

Surface: public API.

Signature: `def resonate(text, docs, df, n=8, k1=1.2, b=0.75)`

What it does: Function resonate takes text, docs, df, n=8, k1=1.2, b=0.75. Its return type is unannotated; the code shows it returning sorted(scored, reverse=True)[:n], [w for w in q .

(Where these words came from: inferred.)

---

## entry point — main

Found in src/bin/check_toc.rs at line 90.

Surface: public API.

Signature: `fn main() -> ExitCode {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## entry point — main

Found in src/main.rs at line 1.

Surface: public API.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## struct — PortKey

Found in src/main.rs at line 13.

Surface: public API.

Signature: `pub struct PortKey {`

What it does: A data shape named PortKey. Its fields are: pub from: &'static str; pub to: &'static str; pub anchor: (f32, f32, f32); pub glyphs: [&'static str; 5].

(Where these words came from: inferred.)

---

## function — PortKey::new

Found in src/main.rs at line 21.

Surface: public API.

Signature: `pub fn new(from: &'static str, to: &'static str) -> Self {`

What it does: Function PortKey::new takes from: &'static str, to: &'static str. It gives back Self.

(Where these words came from: inferred.)

---

## function — PortKey::transmit

Found in src/main.rs at line 30.

Surface: public API.

Signature: `pub fn transmit(&self) {`

What it does: Function PortKey::transmit takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/human/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

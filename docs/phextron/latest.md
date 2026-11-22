# phextron — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 150). Scanned at commit fc143104b42d44cf1b7ea4aef608c38a47078554. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

Description stated in the manifest: Phextron G-L1: lineage HTTP+TLS client for phext-lattice (ureq + rustls, mirroring phext-lattice's own Cargo.toml choices)

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Rust (cargo) — Cargo.toml

Direct dependencies: 0.


Total packages in the lock file, this one included: 75.

### Rust (cargo) — crates/gl1-http/Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 3.

- ureq [dependencies] — 2
- serde [dependencies] — 1
- serde_json [dependencies] — 1

Total packages in the lock file, this one included: 75.

### Rust (cargo) — crates/gl2-credman/Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 0.


Total packages in the lock file, this one included: 75.

### Rust (cargo) — crates/phextron-dash/Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 2.

- libphext [dependencies] — 0.6.6
- serde_json [dependencies] — 1

Total packages in the lock file, this one included: 75.

### Rust (cargo) — crates/phextron-web/Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 0.


Total packages in the lock file, this one included: 75.


---

## Authors

The git log holds 23 commits.

The author of record is **Will Bickford** — 23 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Theia — 12 commits
- Emise — 7 commits
- Will Bickford (his own identities) — 4 commits

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- JavaScript: 2 files, 182 lines total, 12 blank, 34 comment, 136 code
- Markdown: 7 files, 1059 lines total, 116 blank, 2 comment, 941 code
- Python: 2 files, 240 lines total, 37 blank, 7 comment, 196 code
- Rust: 21 files, 7869 lines total, 526 blank, 1072 comment, 6271 code
- Shell: 1 file, 19 lines total, 0 blank, 11 comment, 8 code
- TOML: 5 files, 54 lines total, 7 blank, 4 comment, 43 code

All languages: 38 files, 9423 lines total, 7595 lines of code. Functions read: 356.

Largest files:

- crates/phextron-dash/src/substrates.rs — 1126 lines
- crates/phextron-dash/src/wiring.rs — 935 lines
- crates/phextron-dash/src/data.rs — 834 lines
- crates/phextron-dash/src/inkling.rs — 778 lines
- crates/phextron-dash/src/video.rs — 707 lines

Largest functions, by span in lines:

- read_register in crates/phextron-dash/src/wiring.rs — about 164 lines
- data in crates/phextron-dash/src/wiring.rs — about 144 lines
- data in crates/phextron-dash/src/phexts.rs — about 139 lines
- data in crates/phextron-dash/src/nodes.rs — about 127 lines
- data in crates/phextron-dash/src/substrates.rs — about 119 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 95 test functions counted in the source, across 7 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 19 of 72 (26%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/phextron
- Branch: exo
- Head commit: fc143104b42d44cf1b7ea4aef608c38a47078554
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `join_url` (crates/gl1-http/src/lib.rs:13) — Join a base URL and a path with exactly one separating slash.
- function `cookie_header` (crates/gl1-http/src/lib.rs:24) — Value of the `Cookie` header presenting the session, if any.
- struct `ExchangeResult` (crates/gl1-http/src/lib.rs:30) — The `{session, csrf}` pair returned by the desktop exchange.
- struct `LatticeClient` (crates/gl1-http/src/lib.rs:38) — Typed client for one lattice base URL.
- function `LatticeClient::new` (crates/gl1-http/src/lib.rs:46) — Function LatticeClient::new takes base_url: impl Into<String>.
- function `LatticeClient::with_credentials` (crates/gl1-http/src/lib.rs:55) — Function LatticeClient::with_credentials takes base_url: impl Into<String>, session: Option<String>, csrf: Option<String>,.
- function `LatticeClient::set_session` (crates/gl1-http/src/lib.rs:66) — Function LatticeClient::set_session takes &mut self, session: Option<String>.
- function `LatticeClient::set_csrf` (crates/gl1-http/src/lib.rs:69) — Function LatticeClient::set_csrf takes &mut self, csrf: Option<String>.
- function `LatticeClient::session_value` (crates/gl1-http/src/lib.rs:72) — Function LatticeClient::session_value takes no inputs beyond the value it is called on.
- function `LatticeClient::csrf_value` (crates/gl1-http/src/lib.rs:75) — Function LatticeClient::csrf_value takes no inputs beyond the value it is called on.
- function `LatticeClient::url` (crates/gl1-http/src/lib.rs:78) — Function LatticeClient::url takes &self, path: &str.
- function `LatticeClient::get` (crates/gl1-http/src/lib.rs:100) — Generic GET.
- function `LatticeClient::post` (crates/gl1-http/src/lib.rs:115) — Generic POST with a string body.
- function `LatticeClient::exchange` (crates/gl1-http/src/lib.rs:133) — POST /api/auth/desktop/exchange with the assertion/state headers.
- function `LatticeClient::session` (crates/gl1-http/src/lib.rs:163) — GET /api/session.
- entry point `main` (crates/gl2-credman/src/bin/credman-roundtrip.rs:5) — The entry point: the program starts here.
- const `MAX_BLOB_BYTES` (crates/gl2-credman/src/lib.rs:14) — Generic-credential blob limit enforced by Windows Credential Manager.
- enum `Error` (crates/gl2-credman/src/lib.rs:17) — A named choice named Error.
- type `Result` (crates/gl2-credman/src/lib.rs:42) — A type name, Result, that stands for std::result::Result<T, Error>.
- function `validate_target` (crates/gl2-credman/src/lib.rs:44) — Function validate_target takes target: &str.
- function `validate_blob` (crates/gl2-credman/src/lib.rs:51) — Function validate_blob takes blob: &[u8].
- const `CRED_TYPE_GENERIC` (crates/gl2-credman/src/lib.rs:65) — A fixed value named CRED_TYPE_GENERIC, set to 1.
- const `CRED_PERSIST_LOCAL_MACHINE` (crates/gl2-credman/src/lib.rs:66) — A fixed value named CRED_PERSIST_LOCAL_MACHINE, set to 2.
- const `ERROR_NOT_FOUND` (crates/gl2-credman/src/lib.rs:67) — A fixed value named ERROR_NOT_FOUND, set to 1168.
- struct `CredentialW` (crates/gl2-credman/src/lib.rs:70) — A data shape named CredentialW.
- function `store` (crates/gl2-credman/src/lib.rs:106) — Function store takes target: &str, blob: &[u8].
- function `read` (crates/gl2-credman/src/lib.rs:129) — Function read takes target: &str.
- function `delete` (crates/gl2-credman/src/lib.rs:149) — Function delete takes target: &str.
- function `open_url` (crates/gl2-credman/src/lib.rs:161) — Function open_url takes url: &str.
- function `store` (crates/gl2-credman/src/lib.rs:172) — Function store takes target: &str, blob: &[u8].
- function `read` (crates/gl2-credman/src/lib.rs:177) — Function read takes target: &str.
- function `delete` (crates/gl2-credman/src/lib.rs:181) — Function delete takes target: &str.
- function `open_url` (crates/gl2-credman/src/lib.rs:185) — Function open_url takes _url: &str.
- function `store` (crates/gl2-credman/src/lib.rs:190) — Function store takes target: &str, blob: &[u8].
- function `read` (crates/gl2-credman/src/lib.rs:191) — Function read takes target: &str.
- function `delete` (crates/gl2-credman/src/lib.rs:192) — Function delete takes target: &str.
- function `open_url` (crates/gl2-credman/src/lib.rs:193) — Function open_url takes url: &str.
- struct `DataPanel` (crates/phextron-dash/src/data.rs:525) — awaiting a summary
- struct `InklingPanel` (crates/phextron-dash/src/inkling.rs:449) — awaiting a summary
- struct `JenkinsPanel` (crates/phextron-dash/src/jenkins.rs:350) — awaiting a summary
- entry point `main` (crates/phextron-dash/src/main.rs:162) — The entry point: the program starts here.
- struct `NodesPanel` (crates/phextron-dash/src/nodes.rs:85) — awaiting a summary
- enum `PanelState` (crates/phextron-dash/src/panels.rs:14) — A named choice named PanelState.
- function `PanelState::as_str` (crates/phextron-dash/src/panels.rs:21) — Function PanelState::as_str takes no inputs beyond the value it is called on.
- struct `SourceInfo` (crates/phextron-dash/src/panels.rs:33) — Where a panel's data comes from, stated on the panel itself.
- struct `PanelData` (crates/phextron-dash/src/panels.rs:39) — A data shape named PanelData.
- function `PanelData::to_json` (crates/phextron-dash/src/panels.rs:48) — Function PanelData::to_json takes no inputs beyond the value it is called on.
- trait `Panel` (crates/phextron-dash/src/panels.rs:63) — awaiting a summary
- function `registry` (crates/phextron-dash/src/panels.rs:84) — The five domains of Will's 2026-11-21 directive, in shell order, plus the Data panel (stash task…
- struct `PendingPanel` (crates/phextron-dash/src/pending.rs:12) — awaiting a summary
- function `PendingPanel::cloud` (crates/phextron-dash/src/pending.rs:23) — Function PendingPanel::cloud takes no inputs beyond the value it is called on.
- struct `PhextsPanel` (crates/phextron-dash/src/phexts.rs:275) — awaiting a summary
- struct `SubstratesPanel` (crates/phextron-dash/src/substrates.rs:693) — awaiting a summary
- struct `VideoPanel` (crates/phextron-dash/src/video.rs:427) — awaiting a summary
- struct `WauPanel` (crates/phextron-dash/src/wau.rs:269) — awaiting a summary
- struct `WiringPanel` (crates/phextron-dash/src/wiring.rs:531) — awaiting a summary
- function `append_scroll` (crates/phextron-dash/tools/mark-substrate-check.py:84) — Append one scroll; returns its 1-based scroll index (so the
- function `init_register` (crates/phextron-dash/tools/mark-substrate-check.py:99) — Function init_register takes path: Path.
- function `main` (crates/phextron-dash/tools/mark-substrate-check.py:108) — Function main takes no inputs beyond the value it is called on.
- enum `Json` (crates/phextron-web/src/json.rs:18) — A parsed JSON value.
- function `Json::get` (crates/phextron-web/src/json.rs:28) — Function Json::get takes &self, key: &str.
- function `Json::as_str` (crates/phextron-web/src/json.rs:35) — Function Json::as_str takes no inputs beyond the value it is called on.
- function `Json::as_bool` (crates/phextron-web/src/json.rs:42) — Function Json::as_bool takes no inputs beyond the value it is called on.
- function `Json::as_f64` (crates/phextron-web/src/json.rs:49) — Function Json::as_f64 takes no inputs beyond the value it is called on.
- function `Json::as_arr` (crates/phextron-web/src/json.rs:56) — Function Json::as_arr takes no inputs beyond the value it is called on.
- function `Json::is_null` (crates/phextron-web/src/json.rs:63) — Function Json::is_null takes no inputs beyond the value it is called on.
- function `Json::missing_or_null` (crates/phextron-web/src/json.rs:69) — True when the key is absent or explicitly null — the renderers treat both as "no value" (the JS shell's `v === null…
- function `Json::display` (crates/phextron-web/src/json.rs:84) — Display form, mirroring what the desktop shell's JS produces when it stringifies a JSON value into HTML: strings as…
- function `parse` (crates/phextron-web/src/json.rs:315) — Parse a complete JSON document; trailing non-whitespace is an error.
- function `render_panel_json` (crates/phextron-web/src/lib.rs:29) — Render one panel JSON document (the exact bytes `/api/panels/<id>` serves) to HTML.
- function `esc` (crates/phextron-web/src/render.rs:17) — The shell's `esc()`: HTML-escape a display string.
- function `render_panel` (crates/phextron-web/src/render.rs:218) — Render one panel's JSON (the exact document `/api/panels/<id>` serves) to the HTML the shell places in its panel…

---

## function — join_url

Found in crates/gl1-http/src/lib.rs at line 13.

Surface: public API.

Signature: `pub fn join_url(base: &str, path: &str) -> String {`

What it does: Join a base URL and a path with exactly one separating slash.

(Where these words came from: direct.)

---

## function — cookie_header

Found in crates/gl1-http/src/lib.rs at line 24.

Surface: public API.

Signature: `pub fn cookie_header(session: Option<&str>) -> Option<String> {`

What it does: Value of the `Cookie` header presenting the session, if any.

(Where these words came from: direct.)

---

## struct — ExchangeResult

Found in crates/gl1-http/src/lib.rs at line 30.

Surface: public API.

Signature: `pub struct ExchangeResult {`

What it does: The `{session, csrf}` pair returned by the desktop exchange.

(Where these words came from: direct.)

---

## struct — LatticeClient

Found in crates/gl1-http/src/lib.rs at line 38.

Surface: public API.

Signature: `pub struct LatticeClient {`

What it does: Typed client for one lattice base URL.

(Where these words came from: direct.)

---

## function — LatticeClient::new

Found in crates/gl1-http/src/lib.rs at line 46.

Surface: public API.

Signature: `pub fn new(base_url: impl Into<String>) -> Self {`

What it does: Function LatticeClient::new takes base_url: impl Into<String>. It gives back Self.

(Where these words came from: inferred.)

---

## function — LatticeClient::with_credentials

Found in crates/gl1-http/src/lib.rs at line 55.

Surface: public API.

Signature: `pub fn with_credentials( base_url: impl Into<String>, session: Option<String>, csrf: Option<String>, ) -> Self {`

What it does: Function LatticeClient::with_credentials takes base_url: impl Into<String>, session: Option<String>, csrf: Option<String>,. It gives back Self.

(Where these words came from: inferred.)

---

## function — LatticeClient::set_session

Found in crates/gl1-http/src/lib.rs at line 66.

Surface: public API.

Signature: `pub fn set_session(&mut self, session: Option<String>) {`

What it does: Function LatticeClient::set_session takes &mut self, session: Option<String>. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — LatticeClient::set_csrf

Found in crates/gl1-http/src/lib.rs at line 69.

Surface: public API.

Signature: `pub fn set_csrf(&mut self, csrf: Option<String>) {`

What it does: Function LatticeClient::set_csrf takes &mut self, csrf: Option<String>. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — LatticeClient::session_value

Found in crates/gl1-http/src/lib.rs at line 72.

Surface: public API.

Signature: `pub fn session_value(&self) -> Option<&str> {`

What it does: Function LatticeClient::session_value takes no inputs beyond the value it is called on. It gives back Option<&str>.

(Where these words came from: inferred.)

---

## function — LatticeClient::csrf_value

Found in crates/gl1-http/src/lib.rs at line 75.

Surface: public API.

Signature: `pub fn csrf_value(&self) -> Option<&str> {`

What it does: Function LatticeClient::csrf_value takes no inputs beyond the value it is called on. It gives back Option<&str>.

(Where these words came from: inferred.)

---

## function — LatticeClient::url

Found in crates/gl1-http/src/lib.rs at line 78.

Surface: public API.

Signature: `pub fn url(&self, path: &str) -> String {`

What it does: Function LatticeClient::url takes &self, path: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — LatticeClient::get

Found in crates/gl1-http/src/lib.rs at line 100.

Surface: public API.

Signature: `pub fn get(&self, path: &str) -> Result<(u16, String), ureq::Error> {`

What it does: Generic GET. Returns `(status, body)`. HTTP error statuses are returned, not raised, so callers see lattice's status directly.

(Where these words came from: direct.)

---

## function — LatticeClient::post

Found in crates/gl1-http/src/lib.rs at line 115.

Surface: public API.

Signature: `pub fn post(&self, path: &str, body: &str) -> Result<(u16, String), ureq::Error> {`

What it does: Generic POST with a string body. Sends the CSRF token.

(Where these words came from: direct.)

---

## function — LatticeClient::exchange

Found in crates/gl1-http/src/lib.rs at line 133.

Surface: public API.

Signature: `pub fn exchange( &mut self, assertion: &str, state: &str, ) -> Result<(u16, ExchangeResult), Box<dyn std::error::Error>> {`

What it does: POST /api/auth/desktop/exchange with the assertion/state headers. On success stores the returned session + CSRF on this client.

(Where these words came from: direct.)

---

## function — LatticeClient::session

Found in crates/gl1-http/src/lib.rs at line 163.

Surface: public API.

Signature: `pub fn session(&self) -> Result<(u16, String), ureq::Error> {`

What it does: GET /api/session. Returns `(status, body)`.

(Where these words came from: direct.)

---

## entry point — main

Found in crates/gl2-credman/src/bin/credman-roundtrip.rs at line 5.

Surface: public API.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## const — MAX_BLOB_BYTES

Found in crates/gl2-credman/src/lib.rs at line 14.

Surface: public API.

Signature: `pub const MAX_BLOB_BYTES: usize = 2560; // CRED_MAX_CREDENTIAL_BLOB_SIZE (5 * 512)`

What it does: Generic-credential blob limit enforced by Windows Credential Manager.

(Where these words came from: direct.)

---

## enum — Error

Found in crates/gl2-credman/src/lib.rs at line 17.

Surface: public API.

Signature: `pub enum Error {`

What it does: A named choice named Error. It is one of: BlobTooLarge; InvalidTarget; Unsupported; Os.

(Where these words came from: inferred.)

---

## type — Result

Found in crates/gl2-credman/src/lib.rs at line 42.

Surface: public API.

Signature: `pub type Result<T> = std::result::Result<T, Error>;`

What it does: A type name, Result, that stands for std::result::Result<T, Error>.

(Where these words came from: inferred.)

---

## function — validate_target

Found in crates/gl2-credman/src/lib.rs at line 44.

Surface: public API.

Signature: `pub fn validate_target(target: &str) -> Result<()> {`

What it does: Function validate_target takes target: &str. It gives back Result<()>.

(Where these words came from: inferred.)

---

## function — validate_blob

Found in crates/gl2-credman/src/lib.rs at line 51.

Surface: public API.

Signature: `pub fn validate_blob(blob: &[u8]) -> Result<()> {`

What it does: Function validate_blob takes blob: &[u8]. It gives back Result<()>.

(Where these words came from: inferred.)

---

## const — CRED_TYPE_GENERIC

Found in crates/gl2-credman/src/lib.rs at line 65.

Surface: public API.

Signature: `pub const CRED_TYPE_GENERIC: u32 = 1;`

What it does: A fixed value named CRED_TYPE_GENERIC, set to 1.

(Where these words came from: inferred.)

---

## const — CRED_PERSIST_LOCAL_MACHINE

Found in crates/gl2-credman/src/lib.rs at line 66.

Surface: public API.

Signature: `pub const CRED_PERSIST_LOCAL_MACHINE: u32 = 2;`

What it does: A fixed value named CRED_PERSIST_LOCAL_MACHINE, set to 2.

(Where these words came from: inferred.)

---

## const — ERROR_NOT_FOUND

Found in crates/gl2-credman/src/lib.rs at line 67.

Surface: public API.

Signature: `pub const ERROR_NOT_FOUND: u32 = 1168;`

What it does: A fixed value named ERROR_NOT_FOUND, set to 1168.

(Where these words came from: inferred.)

---

## struct — CredentialW

Found in crates/gl2-credman/src/lib.rs at line 70.

Surface: public API.

Signature: `pub struct CredentialW {`

What it does: A data shape named CredentialW. Its fields are: pub flags: u32; pub cred_type: u32; pub target_name: *mut u16; pub comment: *mut u16; pub last_written: u64; pub credential_blob_size: u32; pub credential_blob: *mut u8; pub persist: u32; pub attribute_count: u32; pub attributes: *mut core::ffi::c_void; pub target_alias: *mut u16; pub user_name: *mut u16.

(Where these words came from: inferred.)

---

## function — store

Found in crates/gl2-credman/src/lib.rs at line 106.

Surface: public API.

Signature: `pub fn store(target: &str, blob: &[u8]) -> Result<()> {`

What it does: Function store takes target: &str, blob: &[u8]. It gives back Result<()>.

(Where these words came from: inferred.)

---

## function — read

Found in crates/gl2-credman/src/lib.rs at line 129.

Surface: public API.

Signature: `pub fn read(target: &str) -> Result<Option<Vec<u8>>> {`

What it does: Function read takes target: &str. It gives back Result<Option<Vec<u8>>>.

(Where these words came from: inferred.)

---

## function — delete

Found in crates/gl2-credman/src/lib.rs at line 149.

Surface: public API.

Signature: `pub fn delete(target: &str) -> Result<()> {`

What it does: Function delete takes target: &str. It gives back Result<()>.

(Where these words came from: inferred.)

---

## function — open_url

Found in crates/gl2-credman/src/lib.rs at line 161.

Surface: public API.

Signature: `pub fn open_url(url: &str) -> Result<()> {`

What it does: Function open_url takes url: &str. It gives back Result<()>.

(Where these words came from: inferred.)

---

## function — store

Found in crates/gl2-credman/src/lib.rs at line 172.

Surface: public API.

Signature: `pub fn store(target: &str, blob: &[u8]) -> Result<()> {`

What it does: Function store takes target: &str, blob: &[u8]. It gives back Result<()>.

(Where these words came from: inferred.)

---

## function — read

Found in crates/gl2-credman/src/lib.rs at line 177.

Surface: public API.

Signature: `pub fn read(target: &str) -> Result<Option<Vec<u8>>> {`

What it does: Function read takes target: &str. It gives back Result<Option<Vec<u8>>>.

(Where these words came from: inferred.)

---

## function — delete

Found in crates/gl2-credman/src/lib.rs at line 181.

Surface: public API.

Signature: `pub fn delete(target: &str) -> Result<()> {`

What it does: Function delete takes target: &str. It gives back Result<()>.

(Where these words came from: inferred.)

---

## function — open_url

Found in crates/gl2-credman/src/lib.rs at line 185.

Surface: public API.

Signature: `pub fn open_url(_url: &str) -> Result<()> {`

What it does: Function open_url takes _url: &str. It gives back Result<()>.

(Where these words came from: inferred.)

---

## function — store

Found in crates/gl2-credman/src/lib.rs at line 190.

Surface: public API.

Signature: `pub fn store(target: &str, blob: &[u8]) -> Result<()> { win::store(target, blob) }`

What it does: Function store takes target: &str, blob: &[u8]. It gives back Result<()>.

(Where these words came from: inferred.)

---

## function — read

Found in crates/gl2-credman/src/lib.rs at line 191.

Surface: public API.

Signature: `pub fn read(target: &str) -> Result<Option<Vec<u8>>> { win::read(target) }`

What it does: Function read takes target: &str. It gives back Result<Option<Vec<u8>>>.

(Where these words came from: inferred.)

---

## function — delete

Found in crates/gl2-credman/src/lib.rs at line 192.

Surface: public API.

Signature: `pub fn delete(target: &str) -> Result<()> { win::delete(target) }`

What it does: Function delete takes target: &str. It gives back Result<()>.

(Where these words came from: inferred.)

---

## function — open_url

Found in crates/gl2-credman/src/lib.rs at line 193.

Surface: public API.

Signature: `pub fn open_url(url: &str) -> Result<()> { win::open_url(url) }`

What it does: Function open_url takes url: &str. It gives back Result<()>.

(Where these words came from: inferred.)

---

## struct — DataPanel

Found in crates/phextron-dash/src/data.rs at line 525.

Surface: public API.

Signature: `pub struct DataPanel;`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## struct — InklingPanel

Found in crates/phextron-dash/src/inkling.rs at line 449.

Surface: public API.

Signature: `pub struct InklingPanel;`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## struct — JenkinsPanel

Found in crates/phextron-dash/src/jenkins.rs at line 350.

Surface: public API.

Signature: `pub struct JenkinsPanel;`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## entry point — main

Found in crates/phextron-dash/src/main.rs at line 162.

Surface: public API.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## struct — NodesPanel

Found in crates/phextron-dash/src/nodes.rs at line 85.

Surface: public API.

Signature: `pub struct NodesPanel;`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## enum — PanelState

Found in crates/phextron-dash/src/panels.rs at line 14.

Surface: public API.

Signature: `pub enum PanelState {`

What it does: A named choice named PanelState. It is one of: Live; Recon; Degraded.

(Where these words came from: inferred.)

---

## function — PanelState::as_str

Found in crates/phextron-dash/src/panels.rs at line 21.

Surface: public API.

Signature: `pub fn as_str(&self) -> &'static str {`

What it does: Function PanelState::as_str takes no inputs beyond the value it is called on. It gives back &'static str.

(Where these words came from: inferred.)

---

## struct — SourceInfo

Found in crates/phextron-dash/src/panels.rs at line 33.

Surface: public API.

Signature: `pub struct SourceInfo {`

What it does: Where a panel's data comes from, stated on the panel itself. `freshness` is a human line computed by the reader at request time (e.g. "pull file updated 42s ago"), never a copied timestamp alone.

(Where these words came from: direct.)

---

## struct — PanelData

Found in crates/phextron-dash/src/panels.rs at line 39.

Surface: public API.

Signature: `pub struct PanelData {`

What it does: A data shape named PanelData. Its fields are: pub id: &'static str; pub title: &'static str; pub state: PanelState; pub source: SourceInfo; pub body: Value.

(Where these words came from: inferred.)

---

## function — PanelData::to_json

Found in crates/phextron-dash/src/panels.rs at line 48.

Surface: public API.

Signature: `pub fn to_json(&self) -> Value {`

What it does: Function PanelData::to_json takes no inputs beyond the value it is called on. It gives back Value.

(Where these words came from: inferred.)

---

## trait — Panel

Found in crates/phextron-dash/src/panels.rs at line 63.

Surface: public API.

Signature: `pub trait Panel: Send + Sync {`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## function — registry

Found in crates/phextron-dash/src/panels.rs at line 84.

Surface: public API.

Signature: `pub fn registry() -> Vec<Box<dyn Panel>> {`

What it does: The five domains of Will's 2026-11-21 directive, in shell order, plus the Data panel (stash task 25.7.9/8.7.10/7.7.10): Phextron's visibility into everywhere Will has stashed data, read from the stash.phext census register — the Substrates panel (W4 plan §1.10, task 31.8.11/9.7.13/8.7.10): per-substrate dispatched-work and budget-check coverage against the reset-window clock — and the Inkling APIs panel (task 20.7.10/10.7.12/7.8.8): every Inkling endpoint on the ranch, from the wiring.phext roster and the inkling-apis.phext deployment register. — and the Wiring panel (W4 plan §1.11, task 24.5.6/6.5.9/5.5.12): the fleet topology drawn from wiring.phext, double-click a node for its SSH command. — and the WAU panel (task 27.6.8/7.5.11/6.5.8): the WAU/Day tracker, Will's tagged "What Absurd Universe" moments counted per day from the wau.phext register's atoms.

(Where these words came from: direct.)

---

## struct — PendingPanel

Found in crates/phextron-dash/src/pending.rs at line 12.

Surface: public API.

Signature: `pub struct PendingPanel {`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## function — PendingPanel::cloud

Found in crates/phextron-dash/src/pending.rs at line 23.

Surface: public API.

Signature: `pub fn cloud() -> Self {`

What it does: Function PendingPanel::cloud takes no inputs beyond the value it is called on. It gives back Self.

(Where these words came from: inferred.)

---

## struct — PhextsPanel

Found in crates/phextron-dash/src/phexts.rs at line 275.

Surface: public API.

Signature: `pub struct PhextsPanel;`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## struct — SubstratesPanel

Found in crates/phextron-dash/src/substrates.rs at line 693.

Surface: public API.

Signature: `pub struct SubstratesPanel;`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## struct — VideoPanel

Found in crates/phextron-dash/src/video.rs at line 427.

Surface: public API.

Signature: `pub struct VideoPanel;`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## struct — WauPanel

Found in crates/phextron-dash/src/wau.rs at line 269.

Surface: public API.

Signature: `pub struct WauPanel;`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## struct — WiringPanel

Found in crates/phextron-dash/src/wiring.rs at line 531.

Surface: public API.

Signature: `pub struct WiringPanel;`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## function — append_scroll

Found in crates/phextron-dash/tools/mark-substrate-check.py at line 84.

Surface: public API.

Signature: `def append_scroll(path: Path, scroll: str) -> int`

What it does: Append one scroll; returns its 1-based scroll index (so the

(Where these words came from: direct.)

---

## function — init_register

Found in crates/phextron-dash/tools/mark-substrate-check.py at line 99.

Surface: public API.

Signature: `def init_register(path: Path) -> None`

What it does: Function init_register takes path: Path. It gives back nothing.

(Where these words came from: inferred.)

---

## function — main

Found in crates/phextron-dash/tools/mark-substrate-check.py at line 108.

Surface: public API.

Signature: `def main() -> int`

What it does: Function main takes no inputs beyond the value it is called on. It gives back int.

(Where these words came from: inferred.)

---

## enum — Json

Found in crates/phextron-web/src/json.rs at line 18.

Surface: public API.

Signature: `pub enum Json {`

What it does: A parsed JSON value. Objects keep insertion order (the console's renderers iterate fixed key lists, so order is cosmetic, but fidelity costs nothing). Numbers keep their raw lexeme for storage; display renders them the way the desktop shell's JS does — through the numeric value, JS-style (`8.0` shows as `8`) — and arithmetic parses on demand.

(Where these words came from: direct.)

---

## function — Json::get

Found in crates/phextron-web/src/json.rs at line 28.

Surface: public API.

Signature: `pub fn get(&self, key: &str) -> Option<&Json> {`

What it does: Function Json::get takes &self, key: &str. It gives back Option<&Json>.

(Where these words came from: inferred.)

---

## function — Json::as_str

Found in crates/phextron-web/src/json.rs at line 35.

Surface: public API.

Signature: `pub fn as_str(&self) -> Option<&str> {`

What it does: Function Json::as_str takes no inputs beyond the value it is called on. It gives back Option<&str>.

(Where these words came from: inferred.)

---

## function — Json::as_bool

Found in crates/phextron-web/src/json.rs at line 42.

Surface: public API.

Signature: `pub fn as_bool(&self) -> Option<bool> {`

What it does: Function Json::as_bool takes no inputs beyond the value it is called on. It gives back Option<bool>.

(Where these words came from: inferred.)

---

## function — Json::as_f64

Found in crates/phextron-web/src/json.rs at line 49.

Surface: public API.

Signature: `pub fn as_f64(&self) -> Option<f64> {`

What it does: Function Json::as_f64 takes no inputs beyond the value it is called on. It gives back Option<f64>.

(Where these words came from: inferred.)

---

## function — Json::as_arr

Found in crates/phextron-web/src/json.rs at line 56.

Surface: public API.

Signature: `pub fn as_arr(&self) -> Option<&[Json]> {`

What it does: Function Json::as_arr takes no inputs beyond the value it is called on. It gives back Option<&[Json]>.

(Where these words came from: inferred.)

---

## function — Json::is_null

Found in crates/phextron-web/src/json.rs at line 63.

Surface: public API.

Signature: `pub fn is_null(&self) -> bool {`

What it does: Function Json::is_null takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — Json::missing_or_null

Found in crates/phextron-web/src/json.rs at line 69.

Surface: public API.

Signature: `pub fn missing_or_null(&self, key: &str) -> bool {`

What it does: True when the key is absent or explicitly null — the renderers treat both as "no value" (the JS shell's `v === null || v === undefined`).

(Where these words came from: direct.)

---

## function — Json::display

Found in crates/phextron-web/src/json.rs at line 84.

Surface: public API.

Signature: `pub fn display(&self) -> String {`

What it does: Display form, mirroring what the desktop shell's JS produces when it stringifies a JSON value into HTML: strings as themselves, booleans as `true`/`false`, null as empty (callers substitute the em dash where the JS does). Numbers go through their f64 value, because the JS shell stringifies the parsed number, not the wire lexeme: the server emits `8.0`, JSON.parse makes it `8`, and the shell shows `8` — so the WASM renderer shows `8` too. (Rust's `{}` for f64 is the same shortest-round-trip form JS uses at panel magnitudes.)

(Where these words came from: direct.)

---

## function — parse

Found in crates/phextron-web/src/json.rs at line 315.

Surface: public API.

Signature: `pub fn parse(input: &str) -> Result<Json, String> {`

What it does: Parse a complete JSON document; trailing non-whitespace is an error.

(Where these words came from: direct.)

---

## function — render_panel_json

Found in crates/phextron-web/src/lib.rs at line 29.

Surface: public API.

Signature: `pub fn render_panel_json(input: &str) -> String {`

What it does: Render one panel JSON document (the exact bytes `/api/panels/<id>` serves) to HTML. Parse failures render an error card — the web UI names its failures on its face, like every other Phextron surface.

(Where these words came from: direct.)

---

## function — esc

Found in crates/phextron-web/src/render.rs at line 17.

Surface: public API.

Signature: `pub fn esc(s: &str) -> String {`

What it does: The shell's `esc()`: HTML-escape a display string.

(Where these words came from: direct.)

---

## function — render_panel

Found in crates/phextron-web/src/render.rs at line 218.

Surface: public API.

Signature: `pub fn render_panel(d: &Json) -> String {`

What it does: Render one panel's JSON (the exact document `/api/panels/<id>` serves) to the HTML the shell places in its panel element.

(Where these words came from: direct.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/phextron/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

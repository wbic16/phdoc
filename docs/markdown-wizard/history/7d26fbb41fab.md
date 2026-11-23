# markdown-wizard — reference as of commit 7d26fbb41fab

Commit 7d26fbb41fabd2c68419d59952aba05fedec8642. 2 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 4 (4 default, 0 optional).

- pulldown-cmark [dependencies] — 0.12
- sha2 [dependencies] — 0.10
- tempfile [dev-dependencies] — 3
- zip [dependencies] — 2

Total packages in the lock file, this one included: 45.

## Authors

Author counts over the 2 audited mainline commits folded to this point.

The author of record is **Will Bickford** — 2 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Theia — 1 commit
- Will Bickford (his own identities) — 1 commit

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 2 files, 237 lines total, 0 blank, 0 comment, 237 code
- Rust: 7 files, 2386 lines total, 0 blank, 0 comment, 2386 code
- Shell: 2 files, 250 lines total, 0 blank, 0 comment, 250 code
- TOML: 1 files, 31 lines total, 0 blank, 0 comment, 31 code

All languages: 12 files, 2904 lines total, 2904 lines of code. Functions read: 93.

Largest files:


Largest functions, by span in lines:


## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 13 test functions counted in the source, across 1 files in test directories or whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 50 of 68 (74%).

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/markdown-wizard.git
- Branch: exo
- Head commit: 7d26fbb41fabd2c68419d59952aba05fedec8642
- Working tree: the committed tree at this commit — a historical reference has no working tree

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- struct `BundleFile` (src/bundle.rs:29) — One file inside a bundle, extracted and held in memory.
- function `BundleFile::text` (src/bundle.rs:38) — The file's text, when it is valid UTF-8 (markdown must be).
- const `MAX_FILES` (src/bundle.rs:23) — Entry-count cap.
- const `MAX_FILE_BYTES` (src/bundle.rs:19) — Per-file extracted cap.
- const `MAX_TOTAL_BYTES` (src/bundle.rs:21) — Whole-bundle extracted cap (zip-bomb guard).
- const `MAX_ZIP_BYTES` (src/bundle.rs:17) — Upload cap: the whole zip, as sent.
- struct `ParsedBundle` (src/bundle.rs:47) — A fully parsed bundle: the archive bytes as published, plus every file extracted.
- function `ParsedBundle::file` (src/bundle.rs:53) — Function ParsedBundle::file takes &self, path: &str and gives back Option<&BundleFile>.
- const `RESERVED_SLUGS` (src/bundle.rs:266) — Names that would collide with this service's own routes or files, now or plausibly later.
- function `bundle_title` (src/bundle.rs:232) — The bundle's title, from the bundle alone: the root index's first line (its incipit), else the slug.
- function `constant_time_eq` (src/bundle.rs:286) — Constant-time equality for key hashes, so a wrong-key probe learns nothing from timing.
- function `incipit` (src/bundle.rs:173) — A file's incipit: its first heading if it opens with one, else its first non-empty line, lightly stripped of markdown furniture.
- function `is_markdown_path` (src/bundle.rs:58) — Function is_markdown_path takes path: &str and gives back bool.
- function `is_root_index` (src/bundle.rs:201) — Is this the bundle's root self-description file?
- function `is_vault_path` (src/bundle.rs:196) — The vault is the bundle's memory: `vault/` holds what was put away.
- function `markdown_stem` (src/bundle.rs:65) — Strip the markdown extension from a path (`docs/guide.md` → `docs/guide`), or return None when the path is not markdown.
- function `new_edit_key` (src/bundle.rs:300) — A fresh edit key: 32 bytes from the kernel's random source, hex.
- function `ordered_files` (src/bundle.rs:209) — Bundle order — the order the sidebar, the tracklist, and prev/next all walk: the root index first (index.md ahead of README.md when a bundle somehow holds both), then every other file by path.
- function `parse_bundle` (src/bundle.rs:99) — Parse and validate a bundle zip.
- function `sha256_hex` (src/bundle.rs:273) — SHA-256 hex of a byte string — edit keys are stored only as this.
- function `valid_bundle_path` (src/bundle.rs:79) — Path hygiene for one zip entry name.
- function `valid_slug` (src/bundle.rs:246) — Slugs are the bundle's public name: `md.phext.io/b/<slug>/`.
- const `CONTENT_SECURITY_POLICY` (src/http.rs:51) — The lattice CSP, with the one documented `img-src` deviation.
- enum `CacheClass` (src/http.rs:26) — How a response may be cached — the lattice's three classes.
- const `HSTS_VALUE` (src/http.rs:48) — One year, matching `deploy/nginx-md.phext.io.conf`.
- const `HTML` (src/http.rs:20) — A fixed value named HTML, set to "text/html; charset=utf-8".
- const `JSON` (src/http.rs:22) — A fixed value named JSON, set to "application/json; charset=utf-8".
- const `TEXT` (src/http.rs:21) — A fixed value named TEXT, set to "text/plain; charset=utf-8".
- function `forwarded_https` (src/http.rs:67) — True when the proxy says the browser connection is HTTPS — the lattice's `forwarded_https`, verbatim in shape.
- function `html_escape` (src/http.rs:303) — HTML-escape text for interpolation into the page templates.
- function `json_escape` (src/http.rs:286) — Minimal JSON string escaping for the hand-formatted API responses (the lattice formats JSON with serde; this service's r
- function `parse_request` (src/http.rs:142) — Parse a request line and headers — the lattice's `parse_request` shape: method, raw target, and a lower-cased header map
- function `percent_decode` (src/http.rs:338) — Percent-decode a URL path segment string.
- function `read_body` (src/http.rs:176) — Read a request body as bytes, up to `cap`.
- function `response_bytes` (src/http.rs:199) — Build a full response — the lattice's `response_bytes`: ETag and 304 handling for the cacheable classes, security header
- function `send_redirect` (src/http.rs:268) — Function send_redirect takes stream: &mut TcpStream, location: &str, req_headers: &HashMap<String, String> and gives bac
- function `send_response` (src/http.rs:241) — Function send_response takes stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8], class: CacheClass, re
- function `send_response_extra` (src/http.rs:253) — Function send_response_extra takes stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8], class: CacheCla
- function `status_text` (src/http.rs:95) — The lattice's status-text table, extended with the codes this service uses (409, 415, 422).
- function `strong_etag` (src/http.rs:120) — A strong ETag: the quoted SHA-256 hex of the body (the lattice's `strong_etag`, which hashes with SHA-256 the same way).
- function `xml_escape` (src/http.rs:320) — XML-escape text for the RSS feed (same five characters as HTML, without the apostrophe form XML parsers also accept).
- command `--dir` (src/main.rs:54) — A command named --dir.
- command `--help` (src/main.rs:72) — A command named --help.
- command `--host` (src/main.rs:66) — A command named --host.
- command `--port` (src/main.rs:60) — A command named --port.
- entry point `main` (src/main.rs:86) — The entry point: the program starts here.
- const `QUIET_LINE` (src/render.rs:249) — The quiet suggestion — it lives in exactly two places: the editor's footer and the bundle index page's footer.
- const `SITE_CSS` (src/render.rs:147) — The shared stylesheet: quiet, editorial, paper and ink with one deep-indigo accent.
- function `first_heading` (src/render.rs:117) — The first `# heading` of a markdown text, for page titles.
- function `index_page` (src/render.rs:278) — The bundle index: `/b/<slug>/`.
- function `not_found_page` (src/render.rs:412) — The 404 page, in the same quiet dress.
- function `reading_page` (src/render.rs:347) — One file's reading view: `/b/<slug>/<path>` (extension dropped).
- function `render_markdown` (src/render.rs:62) — Render markdown to an HTML fragment.
- function `version_line` (src/render.rs:254) — `Version 3 · (Your Version) · updated 15 November 2026`, plus the release's change line when one was given.
- function `feed_xml` (src/rss.rs:63) — The bundle's feed.
- function `long_date` (src/rss.rs:36) — Format a Unix timestamp as a long human date: `15 November 2026`.
- function `rfc822` (src/rss.rs:17) — Format a Unix timestamp as an RFC 822 date in GMT: `Sat, 15 Nov 2026 22:20:00 GMT`.
- struct `Bundle` (src/store.rs:57) — One published bundle, immutable once built.
- enum `PublishError` (src/store.rs:83) — A named choice named PublishError.
- enum `PublishOutcome` (src/store.rs:76) — A named choice named PublishOutcome.
- struct `Store` (src/store.rs:71) — A data shape named Store.
- function `Store::open` (src/store.rs:132) — Open the store at `dir`, loading every committed bundle.
- function `clean_changed_line` (src/store.rs:113) — Sanitize a "what changed?" line: one line, printable, capped.
- function `count` (src/store.rs:161) — Function count takes no inputs beyond the value it is called on, and gives back usize.
- function `get` (src/store.rs:157) — Function get takes &self, slug: &str and gives back Option<Arc<Bundle>>.
- function `has_previous` (src/store.rs:166) — Whether a previous published version survives for this slug.
- function `publish` (src/store.rs:174) — Publish a bundle.
- function `restore` (src/store.rs:220) — Restore the previous published version — the reversibility that replaces confirmation.

## struct — BundleFile

Found in src/bundle.rs at line 29.

Signature: `pub struct BundleFile {`

What it does: One file inside a bundle, extracted and held in memory. Readers hold the whole bundle behind an `Arc` (see store.rs), so a file is just bytes plus its address.

(Where these words came from: doc comment.)

## function — BundleFile::text

Found in src/bundle.rs at line 38.

Signature: `pub fn text(&self) -> Option<&str> {`

What it does: The file's text, when it is valid UTF-8 (markdown must be).

(Where these words came from: doc comment.)

## const — MAX_FILES

Found in src/bundle.rs at line 23.

Signature: `pub const MAX_FILES: usize = 2000;`

What it does: Entry-count cap.

(Where these words came from: doc comment.)

## const — MAX_FILE_BYTES

Found in src/bundle.rs at line 19.

Signature: `pub const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;`

What it does: Per-file extracted cap.

(Where these words came from: doc comment.)

## const — MAX_TOTAL_BYTES

Found in src/bundle.rs at line 21.

Signature: `pub const MAX_TOTAL_BYTES: u64 = 64 * 1024 * 1024;`

What it does: Whole-bundle extracted cap (zip-bomb guard).

(Where these words came from: doc comment.)

## const — MAX_ZIP_BYTES

Found in src/bundle.rs at line 17.

Signature: `pub const MAX_ZIP_BYTES: usize = 16 * 1024 * 1024;`

What it does: Upload cap: the whole zip, as sent. Matches the nginx `client_max_body_size 16M` in deploy/.

(Where these words came from: doc comment.)

## struct — ParsedBundle

Found in src/bundle.rs at line 47.

Signature: `pub struct ParsedBundle {`

What it does: A fully parsed bundle: the archive bytes as published, plus every file extracted. Parsing is the publish gate — holo-phext's rule that a transaction is applied iff every entry verifies, restated for zips: a bundle is stored iff every entry parses within caps.

(Where these words came from: doc comment.)

## function — ParsedBundle::file

Found in src/bundle.rs at line 53.

Signature: `pub fn file(&self, path: &str) -> Option<&BundleFile> {`

What it does: Function ParsedBundle::file takes &self, path: &str and gives back Option<&BundleFile>.

(Where these words came from: signature.)

## const — RESERVED_SLUGS

Found in src/bundle.rs at line 266.

Signature: `pub const RESERVED_SLUGS: &[&str] = &[ "api", "b", "about", "new", "edit", "admin", "feed", "feed.xml", "rss", "rss.xml", "sitemap.xml", "robots.txt", "favicon.ico", "favicon.svg", "healthz", "static", "assets", "index", "index.html", ];`

What it does: Names that would collide with this service's own routes or files, now or plausibly later.

(Where these words came from: doc comment.)

## function — bundle_title

Found in src/bundle.rs at line 232.

Signature: `pub fn bundle_title(parsed: &ParsedBundle, slug: &str) -> String {`

What it does: The bundle's title, from the bundle alone: the root index's first line (its incipit), else the slug. No app-side document structure exists to ask instead.

(Where these words came from: doc comment.)

## function — constant_time_eq

Found in src/bundle.rs at line 286.

Signature: `pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {`

What it does: Constant-time equality for key hashes, so a wrong-key probe learns nothing from timing.

(Where these words came from: doc comment.)

## function — incipit

Found in src/bundle.rs at line 173.

Signature: `pub fn incipit(text: &str) -> Option<String> {`

What it does: A file's incipit: its first heading if it opens with one, else its first non-empty line, lightly stripped of markdown furniture. The sidebar, the tracklist, prev/next links, and feeds all name a file by its incipit — a file's display title derives from its own first line, never from a name it was required to have.

(Where these words came from: doc comment.)

## function — is_markdown_path

Found in src/bundle.rs at line 58.

Signature: `pub fn is_markdown_path(path: &str) -> bool {`

What it does: Function is_markdown_path takes path: &str and gives back bool.

(Where these words came from: signature.)

## function — is_root_index

Found in src/bundle.rs at line 201.

Signature: `pub fn is_root_index(path: &str) -> bool {`

What it does: Is this the bundle's root self-description file?

(Where these words came from: doc comment.)

## function — is_vault_path

Found in src/bundle.rs at line 196.

Signature: `pub fn is_vault_path(path: &str) -> bool {`

What it does: The vault is the bundle's memory: `vault/` holds what was put away. Vault paths are never served, listed, or fed — the vault is private by structure, like a draft.

(Where these words came from: doc comment.)

## function — markdown_stem

Found in src/bundle.rs at line 65.

Signature: `pub fn markdown_stem(path: &str) -> Option<String> {`

What it does: Strip the markdown extension from a path (`docs/guide.md` → `docs/guide`), or return None when the path is not markdown.

(Where these words came from: doc comment.)

## function — new_edit_key

Found in src/bundle.rs at line 300.

Signature: `pub fn new_edit_key() -> std::io::Result<String> {`

What it does: A fresh edit key: 32 bytes from the kernel's random source, hex. (phext-lattice draws on `getrandom` for the same job; reading /dev/urandom is the std-only equivalent on the hosts this runs on.)

(Where these words came from: doc comment.)

## function — ordered_files

Found in src/bundle.rs at line 209.

Signature: `pub fn ordered_files(parsed: &ParsedBundle) -> Vec<&BundleFile> {`

What it does: Bundle order — the order the sidebar, the tracklist, and prev/next all walk: the root index first (index.md ahead of README.md when a bundle somehow holds both), then every other file by path. Vault files are excluded; they are not part of the published order.

(Where these words came from: doc comment.)

## function — parse_bundle

Found in src/bundle.rs at line 99.

Signature: `pub fn parse_bundle(zip_bytes: Vec<u8>) -> Result<ParsedBundle, String> {`

What it does: Parse and validate a bundle zip. Errors are plain-language strings the publish API returns verbatim.

(Where these words came from: doc comment.)

## function — sha256_hex

Found in src/bundle.rs at line 273.

Signature: `pub fn sha256_hex(bytes: &[u8]) -> String {`

What it does: SHA-256 hex of a byte string — edit keys are stored only as this.

(Where these words came from: doc comment.)

## function — valid_bundle_path

Found in src/bundle.rs at line 79.

Signature: `pub fn valid_bundle_path(path: &str) -> bool {`

What it does: Path hygiene for one zip entry name. A path is an address, and an address must stay inside the bundle: no absolute paths, no `..`, no backslashes, no empty or `.` segments, no control characters.

(Where these words came from: doc comment.)

## function — valid_slug

Found in src/bundle.rs at line 246.

Signature: `pub fn valid_slug(slug: &str) -> bool {`

What it does: Slugs are the bundle's public name: `md.phext.io/b/<slug>/`. Lowercase letters, digits, and hyphens; 1–64 chars; must start and end with a letter or digit.

(Where these words came from: doc comment.)

## const — CONTENT_SECURITY_POLICY

Found in src/http.rs at line 51.

Signature: `pub const CONTENT_SECURITY_POLICY: &str = concat!( "default-src 'self'; ",`

What it does: The lattice CSP, with the one documented `img-src` deviation.

(Where these words came from: doc comment.)

## enum — CacheClass

Found in src/http.rs at line 26.

Signature: `pub enum CacheClass {`

What it does: How a response may be cached — the lattice's three classes.

(Where these words came from: doc comment.)

## const — HSTS_VALUE

Found in src/http.rs at line 48.

Signature: `pub const HSTS_VALUE: &str = "max-age=31536000";`

What it does: One year, matching `deploy/nginx-md.phext.io.conf`. No `includeSubDomains`: this host is `md.phext.io`, and a future sibling name should not be pinned by accident. (Same reasoning as the lattice's HSTS note for edit.phext.io.)

(Where these words came from: doc comment.)

## const — HTML

Found in src/http.rs at line 20.

Signature: `pub const HTML: &str = "text/html; charset=utf-8";`

What it does: A fixed value named HTML, set to "text/html; charset=utf-8".

(Where these words came from: signature.)

## const — JSON

Found in src/http.rs at line 22.

Signature: `pub const JSON: &str = "application/json; charset=utf-8";`

What it does: A fixed value named JSON, set to "application/json; charset=utf-8".

(Where these words came from: signature.)

## const — TEXT

Found in src/http.rs at line 21.

Signature: `pub const TEXT: &str = "text/plain; charset=utf-8";`

What it does: A fixed value named TEXT, set to "text/plain; charset=utf-8".

(Where these words came from: signature.)

## function — forwarded_https

Found in src/http.rs at line 67.

Signature: `pub fn forwarded_https(headers: &HashMap<String, String>) -> bool {`

What it does: True when the proxy says the browser connection is HTTPS — the lattice's `forwarded_https`, verbatim in shape.

(Where these words came from: doc comment.)

## function — html_escape

Found in src/http.rs at line 303.

Signature: `pub fn html_escape(s: &str) -> String {`

What it does: HTML-escape text for interpolation into the page templates.

(Where these words came from: doc comment.)

## function — json_escape

Found in src/http.rs at line 286.

Signature: `pub fn json_escape(s: &str) -> String {`

What it does: Minimal JSON string escaping for the hand-formatted API responses (the lattice formats JSON with serde; this service's responses are small and fixed-shape, so they are formatted directly).

(Where these words came from: doc comment.)

## function — parse_request

Found in src/http.rs at line 142.

Signature: `pub fn parse_request( reader: &mut BufReader<TcpStream>, ) -> Option<(String, String, HashMap<String, String>)> {`

What it does: Parse a request line and headers — the lattice's `parse_request` shape: method, raw target, and a lower-cased header map.

(Where these words came from: doc comment.)

## function — percent_decode

Found in src/http.rs at line 338.

Signature: `pub fn percent_decode(s: &str) -> String {`

What it does: Percent-decode a URL path segment string. `+` is left alone (it is a literal plus in a path, not a space). Invalid sequences pass through unchanged; the caller's path rules reject anything odd.

(Where these words came from: doc comment.)

## function — read_body

Found in src/http.rs at line 176.

Signature: `pub fn read_body( reader: &mut BufReader<TcpStream>, headers: &HashMap<String, String>, cap: usize, ) -> Option<Vec<u8>> {`

What it does: Read a request body as bytes, up to `cap`. Returns `None` when the declared length exceeds the cap (the caller answers 413). The lattice reads bodies as lossy text; bundles are binary, so this service reads bytes — same Content-Length discipline.

(Where these words came from: doc comment.)

## function — response_bytes

Found in src/http.rs at line 199.

Signature: `pub fn response_bytes( status: u16, content_type: &str, body: &[u8], class: CacheClass, req_headers: &HashMap<String, String>, head_only: bool, extra_headers: &str, ) -> Vec<u8> {`

What it does: Build a full response — the lattice's `response_bytes`: ETag and 304 handling for the cacheable classes, security headers on everything, `Connection: close`.

(Where these words came from: doc comment.)

## function — send_redirect

Found in src/http.rs at line 268.

Signature: `pub fn send_redirect(stream: &mut TcpStream, location: &str, req_headers: &HashMap<String, String>) {`

What it does: Function send_redirect takes stream: &mut TcpStream, location: &str, req_headers: &HashMap<String, String> and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — send_response

Found in src/http.rs at line 241.

Signature: `pub fn send_response( stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8], class: CacheClass, req_headers: &HashMap<String, String>, head_only: bool, ) {`

What it does: Function send_response takes stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8], class: CacheClass, req_headers: &HashMap<String, String>, head_only: bool, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — send_response_extra

Found in src/http.rs at line 253.

Signature: `pub fn send_response_extra( stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8], class: CacheClass, req_headers: &HashMap<String, String>, head_only: bool, extra_headers: &str, ) {`

What it does: Function send_response_extra takes stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8], class: CacheClass, req_headers: &HashMap<String, String>, head_only: bool, extra_headers: &str, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — status_text

Found in src/http.rs at line 95.

Signature: `pub fn status_text(status: u16) -> &'static str {`

What it does: The lattice's status-text table, extended with the codes this service uses (409, 415, 422).

(Where these words came from: doc comment.)

## function — strong_etag

Found in src/http.rs at line 120.

Signature: `pub fn strong_etag(body: &[u8]) -> String {`

What it does: A strong ETag: the quoted SHA-256 hex of the body (the lattice's `strong_etag`, which hashes with SHA-256 the same way).

(Where these words came from: doc comment.)

## function — xml_escape

Found in src/http.rs at line 320.

Signature: `pub fn xml_escape(s: &str) -> String {`

What it does: XML-escape text for the RSS feed (same five characters as HTML, without the apostrophe form XML parsers also accept).

(Where these words came from: doc comment.)

## command — --dir

Found in src/main.rs at line 54.

Signature: `command "--dir"`

What it does: A command named --dir. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

## command — --help

Found in src/main.rs at line 72.

Signature: `command "--help"`

What it does: A command named --help. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

## command — --host

Found in src/main.rs at line 66.

Signature: `command "--host"`

What it does: A command named --host. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

## command — --port

Found in src/main.rs at line 60.

Signature: `command "--port"`

What it does: A command named --port. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

## entry point — main

Found in src/main.rs at line 86.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: signature.)

## const — QUIET_LINE

Found in src/render.rs at line 249.

Signature: `pub const QUIET_LINE: &str = "Flat files are a fine start. When you want \ addresses in eleven dimensions, try <a href=\"https://edit.phext.io\">edit.phext.io</a>.";`

What it does: The quiet suggestion — it lives in exactly two places: the editor's footer and the bundle index page's footer.

(Where these words came from: doc comment.)

## const — SITE_CSS

Found in src/render.rs at line 147.

Signature: `pub const SITE_CSS: &str = r#" :root {`

What it does: The shared stylesheet: quiet, editorial, paper and ink with one deep-indigo accent. System serif for display and body, system sans for the small chrome, mono for addresses. No webfonts, no requests — the page is the whole of it.

(Where these words came from: doc comment.)

## function — first_heading

Found in src/render.rs at line 117.

Signature: `pub fn first_heading(md: &str) -> Option<String> {`

What it does: The first `# heading` of a markdown text, for page titles.

(Where these words came from: doc comment.)

## function — index_page

Found in src/render.rs at line 278.

Signature: `pub fn index_page(bundle: &Bundle, base: &str) -> String {`

What it does: The bundle index: `/b/<slug>/`. A tracklist under the bundle's own self-description — the site is generated from the bundle alone.

(Where these words came from: doc comment.)

## function — not_found_page

Found in src/render.rs at line 412.

Signature: `pub fn not_found_page(what: &str) -> String {`

What it does: The 404 page, in the same quiet dress.

(Where these words came from: doc comment.)

## function — reading_page

Found in src/render.rs at line 347.

Signature: `pub fn reading_page(bundle: &Bundle, file: &BundleFile) -> String {`

What it does: One file's reading view: `/b/<slug>/<path>` (extension dropped). A printed page: title, rule, body, neighbours, colophon.

(Where these words came from: doc comment.)

## function — render_markdown

Found in src/render.rs at line 62.

Signature: `pub fn render_markdown(md: &str) -> String {`

What it does: Render markdown to an HTML fragment.

(Where these words came from: doc comment.)

## function — version_line

Found in src/render.rs at line 254.

Signature: `pub fn version_line(bundle: &Bundle) -> String {`

What it does: `Version 3 · (Your Version) · updated 15 November 2026`, plus the release's change line when one was given.

(Where these words came from: doc comment.)

## function — feed_xml

Found in src/rss.rs at line 63.

Signature: `pub fn feed_xml(bundle: &Bundle, base: &str) -> String {`

What it does: The bundle's feed. `base` is the site's public origin with no trailing slash (e.g. `https://md.phext.io`); item links are built from it so feed readers land on real addresses.

(Where these words came from: doc comment.)

## function — long_date

Found in src/rss.rs at line 36.

Signature: `pub fn long_date(unix: u64) -> String {`

What it does: Format a Unix timestamp as a long human date: `15 November 2026`.

(Where these words came from: doc comment.)

## function — rfc822

Found in src/rss.rs at line 17.

Signature: `pub fn rfc822(unix: u64) -> String {`

What it does: Format a Unix timestamp as an RFC 822 date in GMT: `Sat, 15 Nov 2026 22:20:00 GMT`.

(Where these words came from: doc comment.)

## struct — Bundle

Found in src/store.rs at line 57.

Signature: `pub struct Bundle {`

What it does: One published bundle, immutable once built. Readers clone the `Arc` and drop the lock immediately — the snapshot pattern.

(Where these words came from: doc comment.)

## enum — PublishError

Found in src/store.rs at line 83.

Signature: `pub enum PublishError {`

What it does: A named choice named PublishError. It is one of: Invalid; WrongKey; NoPrevious; Io.

(Where these words came from: signature.)

## enum — PublishOutcome

Found in src/store.rs at line 76.

Signature: `pub enum PublishOutcome {`

What it does: A named choice named PublishOutcome. It is one of: Created; Replaced.

(Where these words came from: signature.)

## struct — Store

Found in src/store.rs at line 71.

Signature: `pub struct Store {`

What it does: A data shape named Store. The scanner did not read its fields.

(Where these words came from: signature.)

## function — Store::open

Found in src/store.rs at line 132.

Signature: `pub fn open(dir: PathBuf) -> std::io::Result<Store> {`

What it does: Open the store at `dir`, loading every committed bundle. Unparseable or torn directories are skipped with a note on stderr — an uncommitted state is not a state.

(Where these words came from: doc comment.)

## function — clean_changed_line

Found in src/store.rs at line 113.

Signature: `pub fn clean_changed_line(s: &str) -> Option<String> {`

What it does: Sanitize a "what changed?" line: one line, printable, capped.

(Where these words came from: doc comment.)

## function — count

Found in src/store.rs at line 161.

Signature: `pub fn count(&self) -> usize {`

What it does: Function count takes no inputs beyond the value it is called on, and gives back usize.

(Where these words came from: signature.)

## function — get

Found in src/store.rs at line 157.

Signature: `pub fn get(&self, slug: &str) -> Option<Arc<Bundle>> {`

What it does: Function get takes &self, slug: &str and gives back Option<Arc<Bundle>>.

(Where these words came from: signature.)

## function — has_previous

Found in src/store.rs at line 166.

Signature: `pub fn has_previous(&self, slug: &str) -> bool {`

What it does: Whether a previous published version survives for this slug.

(Where these words came from: doc comment.)

## function — publish

Found in src/store.rs at line 174.

Signature: `pub fn publish( &self, slug: &str, zip_bytes: Vec<u8>, supplied_key: Option<&str>, changed: Option<String>, ) -> Result<PublishOutcome, PublishError> {`

What it does: Publish a bundle. `supplied_key` is required iff the slug already exists. The whole operation is: verify in memory, check the key, preserve the current as previous, stage + sync + rename + sync, swap the Arc.

(Where these words came from: doc comment.)

## function — restore

Found in src/store.rs at line 220.

Signature: `pub fn restore(&self, slug: &str, supplied_key: Option<&str>) -> Result<Arc<Bundle>, PublishError> {`

What it does: Restore the previous published version — the reversibility that replaces confirmation. Runs the same commit path with the previous bytes and the previous meta (its own version, date, and change line come back with it), after preserving the current state into `prev/`, so a restore is itself reversible.

(Where these words came from: doc comment.)

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/markdown-wizard/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

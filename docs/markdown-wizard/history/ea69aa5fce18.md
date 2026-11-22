# markdown-wizard — reference as of commit ea69aa5fce18

Commit ea69aa5fce18055daee155d3b6149f0652cf7859. 6 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/markdown-wizard/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

## Dependencies

### Cargo.toml

- pulldown-cmark — 0.12
- sha2 — 0.10
- tempfile — 3
- zip — 2
- serde_json — 1

## Authors

The author of record is **Will Bickford** — 6 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Theia — 5 commits
- Will Bickford (his own identities) — 1 commit

## Complexity

- Markdown: 2 files, 357 lines total, 0 blank, 0 comment, 357 code
- Rust: 10 files, 4464 lines total, 0 blank, 0 comment, 4464 code
- Shell: 2 files, 310 lines total, 0 blank, 0 comment, 310 code
- TOML: 1 files, 36 lines total, 0 blank, 0 comment, 36 code

Declared RFCs at this point: none declared.

## Public methods and commands

- struct `BundleFile` (src/bundle.rs) — One file inside a bundle, extracted and held in memory.
- function `BundleFile::text` (src/bundle.rs) — The file's text, when it is valid UTF-8 (markdown must be).
- const `MAX_FILES` (src/bundle.rs) — Entry-count cap.
- const `MAX_FILE_BYTES` (src/bundle.rs) — Per-file extracted cap.
- const `MAX_TOTAL_BYTES` (src/bundle.rs) — Whole-bundle extracted cap (zip-bomb guard).
- const `MAX_ZIP_BYTES` (src/bundle.rs) — Upload cap: the whole zip, as sent.
- struct `ParsedBundle` (src/bundle.rs) — A fully parsed bundle: the archive bytes as published, plus every file extracted.
- function `ParsedBundle::file` (src/bundle.rs) — Function ParsedBundle::file takes &self, path: &str and gives back Option<&BundleFile>.
- const `RESERVED_SLUGS` (src/bundle.rs) — Names that would collide with this service's own routes or files, now or plausibly later.
- function `build_zip` (src/bundle.rs) — Serialize files back into a bundle archive — the inverse of parse_bundle, used for working copies (work.rs).
- function `bundle_title` (src/bundle.rs) — The bundle's title, from the bundle alone: the root index's first line (its incipit), else the slug.
- function `constant_time_eq` (src/bundle.rs) — Constant-time equality for key hashes, so a wrong-key probe learns nothing from timing.
- function `incipit` (src/bundle.rs) — A file's incipit: its first heading if it opens with one, else its first non-empty line, lightly stripped of markdown furniture.
- function `is_markdown_path` (src/bundle.rs) — Function is_markdown_path takes path: &str and gives back bool.
- function `is_root_index` (src/bundle.rs) — Is this the bundle's root self-description file?
- function `is_vault_path` (src/bundle.rs) — The vault is the bundle's memory: `vault/` holds what was put away.
- function `markdown_stem` (src/bundle.rs) — Strip the markdown extension from a path (`docs/guide.md` → `docs/guide`), or return None when the path is not markdown.
- function `new_edit_key` (src/bundle.rs) — A fresh edit key: 32 bytes from the kernel's random source, hex.
- function `ordered_files` (src/bundle.rs) — Bundle order — the order the sidebar, the tracklist, and prev/next all walk: the root index first (index.md ahead of README.md when a bundle somehow holds both), then every other file by path.
- function `parse_bundle` (src/bundle.rs) — Parse and validate a bundle zip.
- function `parse_working` (src/bundle.rs) — Parse a working-copy archive (see work.rs): the same hygiene and caps as a published bundle, but a draft may legitimatel
- function `sha256_hex` (src/bundle.rs) — SHA-256 hex of a byte string — edit keys are stored only as this.
- function `valid_bundle_path` (src/bundle.rs) — Path hygiene for one zip entry name.
- function `valid_slug` (src/bundle.rs) — Slugs are the bundle's public name: `md.phext.io/b/<slug>/`.
- const `CONTENT_SECURITY_POLICY` (src/http.rs) — The lattice CSP, with the one documented `img-src` deviation.
- enum `CacheClass` (src/http.rs) — How a response may be cached — the lattice's three classes.
- const `HSTS_VALUE` (src/http.rs) — One year, matching `deploy/nginx-md.phext.io.conf`.
- const `HTML` (src/http.rs) — A fixed value named HTML, set to "text/html; charset=utf-8".
- const `JSON` (src/http.rs) — A fixed value named JSON, set to "application/json; charset=utf-8".
- const `TEXT` (src/http.rs) — A fixed value named TEXT, set to "text/plain; charset=utf-8".
- function `forwarded_https` (src/http.rs) — True when the proxy says the browser connection is HTTPS — the lattice's `forwarded_https`, verbatim in shape.
- function `html_escape` (src/http.rs) — HTML-escape text for interpolation into the page templates.
- function `json_escape` (src/http.rs) — Minimal JSON string escaping for the hand-formatted API responses (the lattice formats JSON with serde; this service's r
- function `parse_request` (src/http.rs) — Parse a request line and headers — the lattice's `parse_request` shape: method, raw target, and a lower-cased header map
- function `percent_decode` (src/http.rs) — Percent-decode a URL path segment string.
- function `read_body` (src/http.rs) — Read a request body as bytes, up to `cap`.
- function `response_bytes` (src/http.rs) — Build a full response — the lattice's `response_bytes`: ETag and 304 handling for the cacheable classes, security header
- function `send_redirect` (src/http.rs) — Function send_redirect takes stream: &mut TcpStream, location: &str, req_headers: &HashMap<String, String> and gives bac
- function `send_response` (src/http.rs) — Function send_response takes stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8], class: CacheClass, re
- function `send_response_extra` (src/http.rs) — Function send_response_extra takes stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8], class: CacheCla
- function `status_text` (src/http.rs) — The lattice's status-text table, extended with the codes this service uses (409, 415, 422).
- function `strong_etag` (src/http.rs) — A strong ETag: the quoted SHA-256 hex of the body (the lattice's `strong_etag`, which hashes with SHA-256 the same way).
- function `xml_escape` (src/http.rs) — XML-escape text for the RSS feed (same five characters as HTML, without the apostrophe form XML parsers also accept).
- command `--dir` (src/main.rs) — A command named --dir.
- command `--help` (src/main.rs) — A command named --help.
- command `--host` (src/main.rs) — A command named --host.
- command `--port` (src/main.rs) — A command named --port.
- entry point `main` (src/main.rs) — The entry point: the program starts here.
- function `handle_mcp` (src/mcp.rs) — Function handle_mcp takes stream: &mut TcpStream, reader: &mut BufReader<TcpStream>, method: &str, headers: &HashMap<Str
- const `QUIET_LINE` (src/render.rs) — The quiet suggestion — it lives in exactly two places: the editor's footer and the bundle index page's footer.
- const `SITE_CSS` (src/render.rs) — The shared stylesheet: quiet, editorial, paper and ink with one deep-indigo accent.
- const `THEME_SNIPPET` (src/render.rs) — Theme resolution, run before first paint on every published page — the same snippet the editor carries in its head.
- function `first_heading` (src/render.rs) — The first `# heading` of a markdown text, for page titles.
- function `index_page` (src/render.rs) — The bundle index: `/b/<slug>/`.
- function `not_found_page` (src/render.rs) — The 404 page, in the same quiet dress.
- function `reading_page` (src/render.rs) — One file's reading view: `/b/<slug>/<path>` (extension dropped).
- function `render_markdown` (src/render.rs) — Render markdown to an HTML fragment.
- function `version_line` (src/render.rs) — `Version 3 · (Your Version) · updated 15 November 2026`, plus the release's change line when one was given.
- function `feed_xml` (src/rss.rs) — The bundle's feed.
- function `long_date` (src/rss.rs) — Format a Unix timestamp as a long human date: `15 November 2026`.
- function `rfc822` (src/rss.rs) — Format a Unix timestamp as an RFC 822 date in GMT: `Sat, 15 Nov 2026 22:20:00 GMT`.
- struct `Bundle` (src/store.rs) — One published bundle, immutable once built.
- function `Bundle::stamps_of` (src/store.rs) — One file's (first published, last changed) stamps.
- struct `FileStamps` (src/store.rs) — When one file was first published, and when its content last changed.
- enum `PublishError` (src/store.rs) — A named choice named PublishError.
- enum `PublishOutcome` (src/store.rs) — A named choice named PublishOutcome.
- struct `Store` (src/store.rs) — A data shape named Store.
- function `Store::open` (src/store.rs) — Open the store at `dir`, loading every committed bundle.
- function `clean_changed_line` (src/store.rs) — Sanitize a "what changed?" line: one line, printable, capped.
- function `count` (src/store.rs) — Function count takes no inputs beyond the value it is called on, and gives back usize.
- function `get` (src/store.rs) — Function get takes &self, slug: &str and gives back Option<Arc<Bundle>>.
- function `has_previous` (src/store.rs) — Whether a previous published version survives for this slug.
- function `list` (src/store.rs) — Every published bundle, ordered by slug — the MCP `list_bundles` surface.
- function `publish` (src/store.rs) — Publish a bundle.
- function `publish_with_issued_key` (src/store.rs) — First publish with a caller-issued edit key.
- function `restore` (src/store.rs) — Restore the previous published version — the reversibility that replaces confirmation.
- struct `OpenedCopy` (src/work.rs) — The result of opening a working copy.
- struct `PublishedInfo` (src/work.rs) — A data shape named PublishedInfo.
- enum `WorkError` (src/work.rs) — A named choice named WorkError.
- struct `WorkStore` (src/work.rs) — A data shape named WorkStore.
- function `WorkStore::open` (src/work.rs) — Open the working-copy store at `dir`, loading every persisted draft.
- struct `WorkingCopy` (src/work.rs) — One slug's draft: a mutable file set behind a lock, with the version it was seeded from and the hash of its only credent
- function `WorkingCopy::file` (src/work.rs) — Function WorkingCopy::file takes &self, path: &str and gives back Option<&BundleFile>.
- struct `WriteInfo` (src/work.rs) — A data shape named WriteInfo.
- function `diverged` (src/work.rs) — Does this draft differ from what's published? The three-state word (Drafting / Current / Diverged) rests on this compari
- function `get` (src/work.rs) — Function get takes &self, slug: &str and gives back Option<Arc<Mutex<WorkingCopy>>>.
- function `open_or_seed` (src/work.rs) — Open the working copy for `slug`, seeding or minting it when none exists yet.
- function `publish` (src/work.rs) — Commit the slug's working copy through the store's own publish path — the same gate and the same atomic swap as a web release.
- function `total_bytes` (src/work.rs) — Function total_bytes takes no inputs beyond the value it is called on, and gives back u64.
- function `verify` (src/work.rs) — Function verify takes &self, supplied_key: Option<&str> and gives back bool.
- function `write_file` (src/work.rs) — Create or replace one file in the slug's working copy, opening the copy first when needed, and persist the draft before 

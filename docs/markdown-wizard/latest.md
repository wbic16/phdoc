# markdown-wizard — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 106). Project root: /source/markdown-wizard. Scanned at commit 0e9f81123654e8cce883200e1505263d426e8fc1. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 9.

- zip [dependencies] — 2
- pulldown-cmark [dependencies] — 0.12
- sha2 [dependencies] — 0.10
- serde_json [dependencies] — 1
- serde [dependencies] — 1
- ureq [dependencies] — 2
- tempfile [dev-dependencies] — 3
- serde_json [dev-dependencies] — 1
- zip [dev-dependencies] — 2

Total packages in the lock file, this one included: 107.

### JavaScript (npm) — tests/parity/package.json

Direct dependencies: 1.

- 
    "@playwright/test — 1.57.0"
  


---

## Authors

The git log holds 24 commits. The authors, by commit count:

- Theia <theia@mirrorborn.us> — 11 commits (45.8%)
- Emise <emise@phext.io> — 9 commits (37.5%)
- Emise <emise@localhost> — 2 commits (8.3%)
- Theia <theia@aletheia-core> — 1 commit (4.2%)
- Will Bickford <wbic16@gmail.com> — 1 commit (4.2%)

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- JavaScript: 4 files, 390 lines total, 31 blank, 108 comment, 251 code
- Markdown: 3 files, 778 lines total, 103 blank, 0 comment, 675 code
- Rust: 17 files, 15463 lines total, 804 blank, 2005 comment, 12654 code
- Shell: 3 files, 527 lines total, 42 blank, 96 comment, 389 code
- TOML: 1 files, 46 lines total, 4 blank, 16 comment, 26 code

All languages: 28 files, 17204 lines total, 13995 lines of code. Functions read: 509.

Largest files:

- src/fed.rs — 2465 lines
- src/cloud.rs — 1658 lines
- src/main.rs — 1079 lines
- src/wordpress.rs — 1061 lines
- tests/http_e2e.rs — 992 lines

Largest functions, by span in lines:

- stub_handle in tests/fed_e2e.rs — about 221 lines
- federation_against_a_stub_provider in tests/fed_e2e.rs — about 218 lines
- mcp_agent_session_end_to_end in tests/mcp_e2e.rs — about 200 lines
- mcp_keep_private_mark_holds_a_file_out_of_the_publication in tests/mcp_e2e.rs — about 198 lines
- strip_leading_h1 in src/render.rs — about 193 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 80 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 114 of 180 (63%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: git@github.com:wbic16/markdown-wizard.git
- Branch: (detached or unnamed)
- Head commit: 0e9f81123654e8cce883200e1505263d426e8fc1
- Working tree: 1 changed paths

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- const `MAX_ZIP_BYTES` (src/bundle.rs:17) — Upload cap: the whole zip, as sent.
- const `MAX_FILE_BYTES` (src/bundle.rs:19) — Per-file extracted cap.
- const `MAX_TOTAL_BYTES` (src/bundle.rs:21) — Whole-bundle extracted cap (zip-bomb guard).
- const `MAX_FILES` (src/bundle.rs:23) — Entry-count cap.
- struct `BundleFile` (src/bundle.rs:29) — One file inside a bundle, extracted and held in memory.
- function `BundleFile::text` (src/bundle.rs:38) — The file's text, when it is valid UTF-8 (markdown must be).
- struct `ParsedBundle` (src/bundle.rs:47) — A fully parsed bundle: the archive bytes as published, plus every file extracted.
- function `ParsedBundle::file` (src/bundle.rs:62) — Function ParsedBundle::file takes &self, path: &str and gives back Option<&BundleFile>.
- function `is_markdown_path` (src/bundle.rs:67) — Function is_markdown_path takes path: &str and gives back bool.
- function `markdown_stem` (src/bundle.rs:74) — Strip the markdown extension from a path (`docs/guide.md` → `docs/guide`), or return None when the path is not markdown.
- function `valid_bundle_path` (src/bundle.rs:88) — Path hygiene for one zip entry name.
- function `parse_bundle` (src/bundle.rs:108) — Parse and validate a bundle zip.
- function `parse_working` (src/bundle.rs:115) — Parse a working-copy archive (see work.rs): the same hygiene and caps as a published bundle, but a draft may legitimatel
- function `build_zip` (src/bundle.rs:207) — Serialize files back into a bundle archive — the inverse of parse_bundle, used for working copies (work.rs).
- function `incipit` (src/bundle.rs:235) — A file's incipit: its first heading if it opens with one, else its first non-empty line, lightly stripped of markdown furniture.
- function `is_vault_path` (src/bundle.rs:258) — The vault is the bundle's memory: `vault/` holds what was put away.
- function `is_root_index` (src/bundle.rs:263) — Is this the bundle's root self-description file?
- function `ordered_files` (src/bundle.rs:271) — Bundle order — the order the sidebar, the tracklist, and prev/next all walk: the root index first (index.md ahead of README.md when a bundle somehow holds both), then every other file by path.
- function `bundle_title` (src/bundle.rs:294) — The bundle's title, from the bundle alone: the root index's first line (its incipit), else the slug.
- function `valid_slug` (src/bundle.rs:308) — Slugs are the bundle's public name: `md.phext.io/b/<slug>/`.
- const `RESERVED_SLUGS` (src/bundle.rs:328) — Names that would collide with this service's own routes or files, now or plausibly later.
- function `sha256_hex` (src/bundle.rs:335) — SHA-256 hex of a byte string — edit keys are stored only as this.
- function `constant_time_eq` (src/bundle.rs:348) — Constant-time equality for key hashes, so a wrong-key probe learns nothing from timing.
- function `new_edit_key` (src/bundle.rs:362) — A fresh edit key: 32 bytes from the kernel's random source, hex.
- enum `Provider` (src/cloud.rs:64) — A named choice named Provider.
- function `Provider::parse` (src/cloud.rs:71) — Function Provider::parse takes s: &str and gives back Option<Provider>.
- function `as_str` (src/cloud.rs:79) — Function as_str takes no inputs beyond the value it is called on, and gives back &'static str.
- struct `CloudSettings` (src/cloud.rs:96) — One bundle's cloud connection, all three providers' fields in one flat record (the `cloud.txt` lines form); only the active provider's fields are read by the engine.
- function `CloudSettings::to_lines` (src/cloud.rs:119) — Function CloudSettings::to_lines takes no inputs beyond the value it is called on, and gives back String.
- function `from_lines` (src/cloud.rs:141) — Parse a `cloud.txt`.
- function `destination_id` (src/cloud.rs:200) — The identity of the destination the sync state belongs to.
- function `normalize_prefix` (src/cloud.rs:218) — A prefix is a folder: no leading slash, a trailing slash when it names anything.
- function `merge_settings` (src/cloud.rs:236) — Merge a settings request body (the same `key=value` lines as `cloud.txt`) over the settings already held, and validate the result.
- function `validate_settings` (src/cloud.rs:285) — Plain-language validation; the messages are user-facing and the settings API returns them verbatim.
- function `settings_json` (src/cloud.rs:376) — The settings as the API answers them: everything the editor needs to render the connection, and no secret — a key ID is 
- struct `CloudStore` (src/cloud.rs:433) — The per-bundle cloud files, beside the bundle in the data dir: `cloud.txt` (the connection — a credential file, written mode 0600 by staged rename) and `cloud-state.txt` (the sync state).
- function `CloudStore::new` (src/cloud.rs:438) — Function CloudStore::new takes dir: PathBuf and gives back CloudStore.
- function `load_settings` (src/cloud.rs:449) — Function load_settings takes &self, slug: &str and gives back Option<CloudSettings>.
- function `save_settings` (src/cloud.rs:454) — Function save_settings takes &self, slug: &str, settings: &CloudSettings and gives back Result<(), String>.
- function `clear` (src/cloud.rs:461) — Disconnect: both files go — the state belongs to the connection it describes.
- function `load_state` (src/cloud.rs:472) — Function load_state takes &self, slug: &str and gives back SyncState.
- function `record` (src/cloud.rs:479) — Function record takes &self, slug: &str, state: &SyncState and gives back Result<(), String>.
- struct `ConfirmedFile` (src/cloud.rs:530) — A data shape named ConfirmedFile.
- struct `SyncState` (src/cloud.rs:536) — A data shape named SyncState.
- function `SyncState::confirmed_for` (src/cloud.rs:547) — The confirmations that count for this destination — none, if the state was recorded against another one.
- function `to_lines` (src/cloud.rs:555) — Function to_lines takes no inputs beyond the value it is called on, and gives back String.
- function `from_lines` (src/cloud.rs:569) — Function from_lines takes text: &str and gives back SyncState.
- struct `FileOutcome` (src/cloud.rs:603) — One failed file of a sync: its tree path and the provider's error, in the provider's words.
- struct `CloudReport` (src/cloud.rs:609) — A data shape named CloudReport.
- function `report_json` (src/cloud.rs:618) — Function report_json takes r: &CloudReport and gives back String.
- struct `TreeFile` (src/cloud.rs:699) — One file of the deployable tree: its path inside the tree, the content type a static host serves it under, and its bytes
- function `site_tree` (src/cloud.rs:712) — The payload: the outputs build's static site for this bundle, read from the on-disk tree it generates at publish (outputs.rs is the one tree producer — this module only carries it).
- function `content_type_for` (src/cloud.rs:756) — Function content_type_for takes path: &str and gives back &'static str.
- function `base64_encode` (src/cloud.rs:930) — Function base64_encode takes data: &[u8] and gives back String.
- function `base64_decode` (src/cloud.rs:955) — Function base64_decode takes text: &str and gives back Option<Vec<u8>>.
- function `sync_if_configured` (src/cloud.rs:1369) — The publish hook, in the WordPress adapter's shape: when the bundle has a cloud connection, sync the just-committed bund
- function `sync_bundle` (src/cloud.rs:1399) — Sync on demand (the retry): the stored connection, the stored bundle, no re-publish.
- const `DEFAULT_ISSUER` (src/fed.rs:83) — A fixed value named DEFAULT_ISSUER, set to "https://edit.phext.io".
- const `VAULT_PREFIX` (src/fed.rs:84) — A fixed value named VAULT_PREFIX, set to "mdwiz-vault-".
- struct `FedConfig` (src/fed.rs:95) — A data shape named FedConfig.
- function `FedConfig::from_env` (src/fed.rs:101) — Function FedConfig::from_env takes no inputs beyond the value it is called on, and gives back FedConfig.
- struct `Fed` (src/fed.rs:148) — A data shape named Fed.
- function `Fed::open` (src/fed.rs:285) — Open the federation state under `<data>/fed/`.
- function `enabled` (src/fed.rs:312) — Function enabled takes no inputs beyond the value it is called on, and gives back bool.
- function `issuer` (src/fed.rs:316) — Function issuer takes no inputs beyond the value it is called on, and gives back &str.
- function `Fed::handle_login` (src/fed.rs:1326) — GET /auth/login — begin the flow.
- function `handle_callback` (src/fed.rs:1385) — GET /auth/callback — the provider returns the browser here with a code (or an error).
- function `handle_logout` (src/fed.rs:1496) — POST /auth/logout — forget the session here.
- function `handle_me` (src/fed.rs:1523) — GET /api/me — the editor's one boot question.
- function `handle_vault_status` (src/fed.rs:1593) — GET /api/vault — the vault dialog's whole picture.
- function `handle_vault_new` (src/fed.rs:1673) — POST /api/vault/new {label} — choose a fresh vault name for this session.
- function `handle_vault_choose` (src/fed.rs:1721) — POST /api/vault/choose {name, label?} — adopt a hosted vault as this session's.
- function `handle_vault_starter` (src/fed.rs:1787) — GET /api/vault/starter — the vault as a one-scroll phext file: the manifest itself, at the root coordinate, which is all a phext with one scroll at 1.1.1/1.1.1/1.1.1 is.
- function `handle_vault_sync` (src/fed.rs:1960) — POST /api/vault/sync — push one bundle's local state toward the vault.
- function `handle_vault_pull` (src/fed.rs:2130) — POST /api/vault/pull — bring the vault's copy of one bundle toward this browser.
- const `HTML` (src/http.rs:20) — A fixed value named HTML, set to "text/html; charset=utf-8".
- const `TEXT` (src/http.rs:21) — A fixed value named TEXT, set to "text/plain; charset=utf-8".
- const `JSON` (src/http.rs:22) — A fixed value named JSON, set to "application/json; charset=utf-8".
- enum `CacheClass` (src/http.rs:26) — How a response may be cached — the lattice's three classes.
- const `HSTS_VALUE` (src/http.rs:48) — One year, matching `deploy/nginx-md.phext.io.conf`.
- const `CONTENT_SECURITY_POLICY` (src/http.rs:51) — The lattice CSP, with the one documented `img-src` deviation.
- function `forwarded_https` (src/http.rs:67) — True when the proxy says the browser connection is HTTPS — the lattice's `forwarded_https`, verbatim in shape.
- function `status_text` (src/http.rs:95) — The lattice's status-text table, extended with the codes this service uses (409, 415, 422).
- function `strong_etag` (src/http.rs:120) — A strong ETag: the quoted SHA-256 hex of the body (the lattice's `strong_etag`, which hashes with SHA-256 the same way).
- function `parse_request` (src/http.rs:142) — Parse a request line and headers — the lattice's `parse_request` shape: method, raw target, and a lower-cased header map
- function `read_body` (src/http.rs:200) — Read a request body as bytes, up to `cap`.
- function `response_bytes` (src/http.rs:223) — Build a full response — the lattice's `response_bytes`: ETag and 304 handling for the cacheable classes, security header
- function `send_response` (src/http.rs:265) — Function send_response takes stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8], class: CacheClass, re
- function `send_response_extra` (src/http.rs:277) — Function send_response_extra takes stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8], class: CacheCla
- function `send_redirect` (src/http.rs:292) — Function send_redirect takes stream: &mut TcpStream, location: &str, req_headers: &HashMap<String, String> and gives bac
- function `json_escape` (src/http.rs:310) — Minimal JSON string escaping for the hand-formatted API responses (the lattice formats JSON with serde; this service's r
- function `html_escape` (src/http.rs:327) — HTML-escape text for interpolation into the page templates.
- function `xml_escape` (src/http.rs:350) — XML-escape text for the RSS feed (same five characters as HTML, without the apostrophe form XML parsers also accept).
- function `percent_decode` (src/http.rs:371) — Percent-decode a URL path segment string.
- entry point `main` (src/main.rs:105) — The entry point: the program starts here.
- command `--dir` (src/main.rs:73) — A command named --dir.
- command `--port` (src/main.rs:79) — A command named --port.
- command `--host` (src/main.rs:85) — A command named --host.
- command `--help` (src/main.rs:91) — A command named --help.
- command `provider` (src/main.rs:742) — A command named provider.
- command `bucket` (src/main.rs:747) — A command named bucket.
- command `container` (src/main.rs:750) — A command named container.
- function `handle_mcp` (src/mcp.rs:63) — Function handle_mcp takes stream: &mut TcpStream, reader: &mut BufReader<TcpStream>, method: &str, headers: &HashMap<Str
- const `CANONICAL_BASE` (src/outputs.rs:62) — The site's canonical public origin.
- function `outputs_incomplete` (src/outputs.rs:79) — Whether a bundle's on-disk outputs are missing or visibly incomplete: no site tree at all, or a Chromium is available now but the tree holds no PDF for a bundle that has markdown — the mark of a publish made while no Chromium existed.
- function `generate` (src/outputs.rs:127) — Build (or rebuild) the derived outputs for one committed bundle and swap them into `<data>/<slug>/site/`.
- function `render_markdown` (src/render.rs:68) — Render markdown to an HTML fragment.
- function `first_heading` (src/render.rs:175) — The first `# heading` of a markdown text, for page titles.
- const `THEME_SNIPPET` (src/render.rs:207) — Theme resolution, run before first paint on every published page — the same snippet the editor carries in its head.
- const `SITE_CSS` (src/render.rs:222) — The shared stylesheet: quiet, editorial, paper and ink with one deep-indigo accent.
- const `QUIET_LINE` (src/render.rs:379) — The quiet suggestion — it lives in exactly two places: the editor's footer and the bundle index page's footer.
- function `version_line` (src/render.rs:384) — `Version 3 · (Your Version) · updated 15 November 2026`, plus the release's change line when one was given.
- function `index_page` (src/render.rs:426) — The bundle index: `/b/<slug>/`.
- function `reading_page` (src/render.rs:500) — One file's reading view: `/b/<slug>/<path>` (extension dropped).
- function `integrate_page` (src/render.rs:587) — The ways-to-use-it page: `/integrate`.
- function `pdf_unavailable_page` (src/render.rs:659) — The PDF address, when this server has no Chromium to print with: a page that says so plainly, in the same quiet dress, a
- function `not_found_page` (src/render.rs:679) — The 404 page, in the same quiet dress.
- const `MAX_FEED_ITEMS` (src/rss.rs:69) — Items in one bundle's feed.
- const `MAX_SITE_FEED_ITEMS` (src/rss.rs:71) — Items in the site-wide feed.
- function `rfc822` (src/rss.rs:79) — Format a Unix timestamp as an RFC 822 date in GMT: `Sat, 15 Nov 2026 22:20:00 GMT`.
- function `long_date` (src/rss.rs:98) — Format a Unix timestamp as a long human date: `15 November 2026`.
- function `websub_hub` (src/rss.rs:125) — The configured WebSub hub, if any: `MDWIZ_WEBSUB_HUB`, read once at first use.
- function `feed_xml` (src/rss.rs:252) — The bundle's feed.
- function `site_feed_xml` (src/rss.rs:282) — The site-wide feed: the items of every published bundle on the instance, newest first.
- function `ping_hub` (src/rss.rs:308) — Announce a commit to the WebSub hub, if one is configured: the bundle's feed and the site-wide feed both changed.
- struct `FileStamps` (src/store.rs:65) — When one file was first published, and when its content last changed.
- struct `Bundle` (src/store.rs:72) — One published bundle, immutable once built.
- function `Bundle::stamps_of` (src/store.rs:96) — One file's (first published, last changed) stamps.
- struct `Store` (src/store.rs:104) — A data shape named Store.
- enum `PublishOutcome` (src/store.rs:117) — A named choice named PublishOutcome.
- enum `PublishError` (src/store.rs:125) — A named choice named PublishError.
- function `clean_changed_line` (src/store.rs:158) — Sanitize a "what changed?" line: one line, printable, capped.
- function `Store::open` (src/store.rs:177) — Open the store at `dir`, loading every committed bundle.
- function `get` (src/store.rs:217) — Function get takes &self, slug: &str and gives back Option<Arc<Bundle>>.
- function `list` (src/store.rs:223) — Every published bundle, ordered by slug — the MCP `list_bundles` surface.
- function `count` (src/store.rs:230) — Function count takes no inputs beyond the value it is called on, and gives back usize.
- function `has_previous` (src/store.rs:235) — Whether a previous published version survives for this slug.
- function `publish` (src/store.rs:255) — Publish a bundle.
- function `publish_from_draft` (src/store.rs:390) — Publish a working copy's archive, checked against the version the draft was based on — the check and the commit under the slug's one lock, so a web publish landing between the draft's own check and its commit cannot be silently overwritten (nor overwrite the draft's commit).
- function `restore` (src/store.rs:423) — Restore the previous published version — the reversibility that replaces confirmation.
- function `site_file` (src/store.rs:529) — One file of a bundle's derived outputs (see outputs.rs), read from `<slug>/site/`.
- struct `WpSettings` (src/wordpress.rs:53) — The per-bundle WordPress connection.
- enum `FileOutcome` (src/wordpress.rs:60) — A named choice named FileOutcome.
- struct `FileResult` (src/wordpress.rs:67) — A data shape named FileResult.
- struct `SyncReport` (src/wordpress.rs:77) — One sync run, recorded with the bundle.
- function `SyncReport::to_json` (src/wordpress.rs:87) — Function SyncReport::to_json takes no inputs beyond the value it is called on, and gives back String.
- struct `WpState` (src/wordpress.rs:133) — The bundle's sync bookkeeping: the path -> post ID mapping, and the last run's report.
- struct `WpStore` (src/wordpress.rs:141) — Persistence for the per-bundle WordPress files, under the same data directory the store owns.
- function `WpStore::new` (src/wordpress.rs:146) — Function WpStore::new takes dir: PathBuf and gives back WpStore.
- function `load_settings` (src/wordpress.rs:156) — Function load_settings takes &self, slug: &str and gives back Option<WpSettings>.
- function `save_settings` (src/wordpress.rs:187) — Function save_settings takes &self, slug: &str, settings: &WpSettings and gives back std::io::Result<()>.
- function `clear` (src/wordpress.rs:200) — Forget the connection and the bookkeeping both.
- function `load_state` (src/wordpress.rs:214) — Function load_state takes &self, slug: &str and gives back WpState.
- function `save_state` (src/wordpress.rs:276) — Function save_state takes &self, slug: &str, state: &WpState and gives back std::io::Result<()>.
- function `clean_site_url` (src/wordpress.rs:401) — Normalize and vet a site address the owner typed.
- function `clean_user` (src/wordpress.rs:496) — Function clean_user takes raw: &str and gives back Result<String, String>.
- function `publishable_markdown` (src/wordpress.rs:509) — The files a publish sends to WordPress: the bundle's markdown, in bundle order, the vault excluded (bundle order already excludes it).
- function `wp_slug_for` (src/wordpress.rs:527) — `essays/one.md` -> `essays-one`: the whole stem, dashes for slashes, so two files in different folders never collide on 
- function `sync_bundle` (src/wordpress.rs:547) — Sync a bundle to its WordPress site, if one is connected.
- function `sync_if_configured` (src/wordpress.rs:599) — The push that rides a publish — the web release, an MCP publish, and a restore all ride the same hook.
- function `top_level_string` (src/wordpress.rs:877) — A top-level string field, unescaped.
- function `top_level_u64` (src/wordpress.rs:883) — A top-level unsigned-integer field.
- function `json_string_field` (src/wordpress.rs:891) — One string field from a flat JSON object — the shape of the settings request body the editor sends.
- function `base64_encode` (src/wordpress.rs:897) — Function base64_encode takes bytes: &[u8] and gives back String.
- struct `WorkingCopy` (src/work.rs:59) — One slug's draft: a mutable file set behind a lock, with the version it was seeded from and the hash of its only credent
- function `WorkingCopy::file` (src/work.rs:73) — Function WorkingCopy::file takes &self, path: &str and gives back Option<&BundleFile>.
- function `is_private` (src/work.rs:77) — Function is_private takes &self, path: &str and gives back bool.
- function `verify` (src/work.rs:81) — Function verify takes &self, supplied_key: Option<&str> and gives back bool.
- function `total_bytes` (src/work.rs:87) — Function total_bytes takes no inputs beyond the value it is called on, and gives back u64.
- enum `WorkError` (src/work.rs:92) — A named choice named WorkError.
- struct `OpenedCopy` (src/work.rs:133) — The result of opening a working copy.
- struct `WriteInfo` (src/work.rs:140) — A data shape named WriteInfo.
- function `diverged` (src/work.rs:154) — Does this draft differ from what's published? The three-state word (Drafting / Current / Diverged) rests on this comparison.
- struct `PublishedInfo` (src/work.rs:175) — A data shape named PublishedInfo.
- struct `WorkStore` (src/work.rs:184) — A data shape named WorkStore.
- function `WorkStore::open` (src/work.rs:193) — Open the working-copy store at `dir`, loading every persisted draft.
- function `get` (src/work.rs:223) — Function get takes &self, slug: &str and gives back Option<Arc<Mutex<WorkingCopy>>>.
- function `open_or_seed` (src/work.rs:231) — Open the working copy for `slug`, seeding or minting it when none exists yet.
- function `write_file` (src/work.rs:304) — Create or replace one file in the slug's working copy, opening the copy first when needed, and persist the draft before returning — a write that answered is a write that survives a restart.
- function `publish` (src/work.rs:383) — Commit the slug's working copy through the store's own publish path — the same gate and the same atomic swap as a web release.

---

## const — MAX_ZIP_BYTES

Found in src/bundle.rs at line 17.

Signature: `pub const MAX_ZIP_BYTES: usize = 16 * 1024 * 1024;`

What it does: Upload cap: the whole zip, as sent. Matches the nginx `client_max_body_size 16M` in deploy/.

(Where these words came from: doc comment.)

---

## const — MAX_FILE_BYTES

Found in src/bundle.rs at line 19.

Signature: `pub const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;`

What it does: Per-file extracted cap.

(Where these words came from: doc comment.)

---

## const — MAX_TOTAL_BYTES

Found in src/bundle.rs at line 21.

Signature: `pub const MAX_TOTAL_BYTES: u64 = 64 * 1024 * 1024;`

What it does: Whole-bundle extracted cap (zip-bomb guard).

(Where these words came from: doc comment.)

---

## const — MAX_FILES

Found in src/bundle.rs at line 23.

Signature: `pub const MAX_FILES: usize = 2000;`

What it does: Entry-count cap.

(Where these words came from: doc comment.)

---

## struct — BundleFile

Found in src/bundle.rs at line 29.

Signature: `pub struct BundleFile {`

What it does: One file inside a bundle, extracted and held in memory. Readers hold the whole bundle behind an `Arc` (see store.rs), so a file is just bytes plus its address.

(Where these words came from: doc comment.)

---

## function — BundleFile::text

Found in src/bundle.rs at line 38.

Signature: `pub fn text(&self) -> Option<&str> {`

What it does: The file's text, when it is valid UTF-8 (markdown must be).

(Where these words came from: doc comment.)

---

## struct — ParsedBundle

Found in src/bundle.rs at line 47.

Signature: `pub struct ParsedBundle {`

What it does: A fully parsed bundle: the archive bytes as published, plus every file extracted. Parsing is the publish gate — holo-phext's rule that a transaction is applied iff every entry verifies, restated for zips: a bundle is stored iff every entry parses within caps.

(Where these words came from: doc comment.)

---

## function — ParsedBundle::file

Found in src/bundle.rs at line 62.

Signature: `pub fn file(&self, path: &str) -> Option<&BundleFile> {`

What it does: Function ParsedBundle::file takes &self, path: &str and gives back Option<&BundleFile>.

(Where these words came from: signature.)

---

## function — is_markdown_path

Found in src/bundle.rs at line 67.

Signature: `pub fn is_markdown_path(path: &str) -> bool {`

What it does: Function is_markdown_path takes path: &str and gives back bool.

(Where these words came from: signature.)

---

## function — markdown_stem

Found in src/bundle.rs at line 74.

Signature: `pub fn markdown_stem(path: &str) -> Option<String> {`

What it does: Strip the markdown extension from a path (`docs/guide.md` → `docs/guide`), or return None when the path is not markdown.

(Where these words came from: doc comment.)

---

## function — valid_bundle_path

Found in src/bundle.rs at line 88.

Signature: `pub fn valid_bundle_path(path: &str) -> bool {`

What it does: Path hygiene for one zip entry name. A path is an address, and an address must stay inside the bundle: no absolute paths, no `..`, no backslashes, no empty or `.` segments, no control characters.

(Where these words came from: doc comment.)

---

## function — parse_bundle

Found in src/bundle.rs at line 108.

Signature: `pub fn parse_bundle(zip_bytes: Vec<u8>) -> Result<ParsedBundle, String> {`

What it does: Parse and validate a bundle zip. Errors are plain-language strings the publish API returns verbatim.

(Where these words came from: doc comment.)

---

## function — parse_working

Found in src/bundle.rs at line 115.

Signature: `pub fn parse_working(zip_bytes: Vec<u8>) -> Result<ParsedBundle, String> {`

What it does: Parse a working-copy archive (see work.rs): the same hygiene and caps as a published bundle, but a draft may legitimately hold no markdown yet — the publish gate, not the draft, enforces that.

(Where these words came from: doc comment.)

---

## function — build_zip

Found in src/bundle.rs at line 207.

Signature: `pub fn build_zip(files: &[BundleFile]) -> Result<Vec<u8>, String> {`

What it does: Serialize files back into a bundle archive — the inverse of parse_bundle, used for working copies (work.rs). Entries are stored, never deflated, in path order, so the bytes are deterministic for a given file set: the same discipline the editor's hand-rolled writer keeps.

(Where these words came from: doc comment.)

---

## function — incipit

Found in src/bundle.rs at line 235.

Signature: `pub fn incipit(text: &str) -> Option<String> {`

What it does: A file's incipit: its first heading if it opens with one, else its first non-empty line, lightly stripped of markdown furniture. The sidebar, the tracklist, prev/next links, and feeds all name a file by its incipit — a file's display title derives from its own first line, never from a name it was required to have.

(Where these words came from: doc comment.)

---

## function — is_vault_path

Found in src/bundle.rs at line 258.

Signature: `pub fn is_vault_path(path: &str) -> bool {`

What it does: The vault is the bundle's memory: `vault/` holds what was put away. Vault paths are never served, listed, or fed — the vault is private by structure, like a draft.

(Where these words came from: doc comment.)

---

## function — is_root_index

Found in src/bundle.rs at line 263.

Signature: `pub fn is_root_index(path: &str) -> bool {`

What it does: Is this the bundle's root self-description file?

(Where these words came from: doc comment.)

---

## function — ordered_files

Found in src/bundle.rs at line 271.

Signature: `pub fn ordered_files(parsed: &ParsedBundle) -> Vec<&BundleFile> {`

What it does: Bundle order — the order the sidebar, the tracklist, and prev/next all walk: the root index first (index.md ahead of README.md when a bundle somehow holds both), then every other file by path. Vault files are excluded; they are not part of the published order.

(Where these words came from: doc comment.)

---

## function — bundle_title

Found in src/bundle.rs at line 294.

Signature: `pub fn bundle_title(parsed: &ParsedBundle, slug: &str) -> String {`

What it does: The bundle's title, from the bundle alone: the root index's first line (its incipit), else the slug. No app-side document structure exists to ask instead.

(Where these words came from: doc comment.)

---

## function — valid_slug

Found in src/bundle.rs at line 308.

Signature: `pub fn valid_slug(slug: &str) -> bool {`

What it does: Slugs are the bundle's public name: `md.phext.io/b/<slug>/`. Lowercase letters, digits, and hyphens; 1–64 chars; must start and end with a letter or digit.

(Where these words came from: doc comment.)

---

## const — RESERVED_SLUGS

Found in src/bundle.rs at line 328.

Signature: `pub const RESERVED_SLUGS: &[&str] = &[ "api", "b", "about", "new", "edit", "admin", "feed", "feed.xml", "rss", "rss.xml", "sitemap.xml", "robots.txt", "favicon.ico", "favicon.svg", "healthz", "static", "assets", "index", "index.html", ];`

What it does: Names that would collide with this service's own routes or files, now or plausibly later.

(Where these words came from: doc comment.)

---

## function — sha256_hex

Found in src/bundle.rs at line 335.

Signature: `pub fn sha256_hex(bytes: &[u8]) -> String {`

What it does: SHA-256 hex of a byte string — edit keys are stored only as this.

(Where these words came from: doc comment.)

---

## function — constant_time_eq

Found in src/bundle.rs at line 348.

Signature: `pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {`

What it does: Constant-time equality for key hashes, so a wrong-key probe learns nothing from timing.

(Where these words came from: doc comment.)

---

## function — new_edit_key

Found in src/bundle.rs at line 362.

Signature: `pub fn new_edit_key() -> std::io::Result<String> {`

What it does: A fresh edit key: 32 bytes from the kernel's random source, hex. (phext-lattice draws on `getrandom` for the same job; reading /dev/urandom is the std-only equivalent on the hosts this runs on.)

(Where these words came from: doc comment.)

---

## enum — Provider

Found in src/cloud.rs at line 64.

Signature: `pub enum Provider {`

What it does: A named choice named Provider. It is one of: S3; Azure; Gcs.

(Where these words came from: signature.)

---

## function — Provider::parse

Found in src/cloud.rs at line 71.

Signature: `pub fn parse(s: &str) -> Option<Provider> {`

What it does: Function Provider::parse takes s: &str and gives back Option<Provider>.

(Where these words came from: signature.)

---

## function — as_str

Found in src/cloud.rs at line 79.

Signature: `pub fn as_str(&self) -> &'static str {`

What it does: Function as_str takes no inputs beyond the value it is called on, and gives back &'static str.

(Where these words came from: signature.)

---

## struct — CloudSettings

Found in src/cloud.rs at line 96.

Signature: `pub struct CloudSettings {`

What it does: One bundle's cloud connection, all three providers' fields in one flat record (the `cloud.txt` lines form); only the active provider's fields are read by the engine. Credentials live in `access_key_id` / `secret_access_key` (S3, GCS) and `account_key` (Azure).

(Where these words came from: doc comment.)

---

## function — CloudSettings::to_lines

Found in src/cloud.rs at line 119.

Signature: `pub fn to_lines(&self) -> String {`

What it does: Function CloudSettings::to_lines takes no inputs beyond the value it is called on, and gives back String.

(Where these words came from: signature.)

---

## function — from_lines

Found in src/cloud.rs at line 141.

Signature: `pub fn from_lines(text: &str) -> Option<CloudSettings> {`

What it does: Parse a `cloud.txt`. Anything short of the active provider's complete connection is not a connection — the caller treats it as unconfigured.

(Where these words came from: doc comment.)

---

## function — destination_id

Found in src/cloud.rs at line 200.

Signature: `pub fn destination_id(&self) -> String {`

What it does: The identity of the destination the sync state belongs to. Confirmations recorded against any other destination are void: switching provider, bucket, container, prefix, region, or endpoint starts the new destination from unconfirmed. (Region and endpoint belong here as much as the bucket does: correcting a mistyped endpoint — same bucket, same prefix — once kept the old destination's confirmations, and the sync reported every file skipped while uploading nothing at all.)

(Where these words came from: doc comment.)

---

## function — normalize_prefix

Found in src/cloud.rs at line 218.

Signature: `pub fn normalize_prefix(p: &str) -> String {`

What it does: A prefix is a folder: no leading slash, a trailing slash when it names anything.

(Where these words came from: doc comment.)

---

## function — merge_settings

Found in src/cloud.rs at line 236.

Signature: `pub fn merge_settings( existing: Option<&CloudSettings>, body: &str, ) -> Result<CloudSettings, String> {`

What it does: Merge a settings request body (the same `key=value` lines as `cloud.txt`) over the settings already held, and validate the result. The plain fields are authoritative when their line is present (an empty endpoint line means "the provider's own"); the credential lines replace only when non-empty — the editor never receives a saved credential back, so a blank from it can only mean "keep what is saved". Switching the provider line switches which connection is active; the other providers' stored fields stay in the record, unused.

(Where these words came from: doc comment.)

---

## function — validate_settings

Found in src/cloud.rs at line 285.

Signature: `pub fn validate_settings(s: &CloudSettings) -> Result<(), String> {`

What it does: Plain-language validation; the messages are user-facing and the settings API returns them verbatim.

(Where these words came from: doc comment.)

---

## function — settings_json

Found in src/cloud.rs at line 376.

Signature: `pub fn settings_json(settings: Option<&CloudSettings>, state: &SyncState) -> String {`

What it does: The settings as the API answers them: everything the editor needs to render the connection, and no secret — a key ID is named by its last four characters only, and a secret's presence is all that is ever said about it.

(Where these words came from: doc comment.)

---

## struct — CloudStore

Found in src/cloud.rs at line 433.

Signature: `pub struct CloudStore {`

What it does: The per-bundle cloud files, beside the bundle in the data dir: `cloud.txt` (the connection — a credential file, written mode 0600 by staged rename) and `cloud-state.txt` (the sync state). The WordPress store's shape, for the same reasons: reads hit the disk per call, so a settings change made through any door is the settings the next publish uses.

(Where these words came from: doc comment.)

---

## function — CloudStore::new

Found in src/cloud.rs at line 438.

Signature: `pub fn new(dir: PathBuf) -> CloudStore {`

What it does: Function CloudStore::new takes dir: PathBuf and gives back CloudStore.

(Where these words came from: signature.)

---

## function — load_settings

Found in src/cloud.rs at line 449.

Signature: `pub fn load_settings(&self, slug: &str) -> Option<CloudSettings> {`

What it does: Function load_settings takes &self, slug: &str and gives back Option<CloudSettings>.

(Where these words came from: signature.)

---

## function — save_settings

Found in src/cloud.rs at line 454.

Signature: `pub fn save_settings(&self, slug: &str, settings: &CloudSettings) -> Result<(), String> {`

What it does: Function save_settings takes &self, slug: &str, settings: &CloudSettings and gives back Result<(), String>.

(Where these words came from: signature.)

---

## function — clear

Found in src/cloud.rs at line 461.

Signature: `pub fn clear(&self, slug: &str) -> Result<(), String> {`

What it does: Disconnect: both files go — the state belongs to the connection it describes. The destination keeps everything it has; only the road to it is removed.

(Where these words came from: doc comment.)

---

## function — load_state

Found in src/cloud.rs at line 472.

Signature: `pub fn load_state(&self, slug: &str) -> SyncState {`

What it does: Function load_state takes &self, slug: &str and gives back SyncState.

(Where these words came from: signature.)

---

## function — record

Found in src/cloud.rs at line 479.

Signature: `pub fn record(&self, slug: &str, state: &SyncState) -> Result<(), String> {`

What it does: Function record takes &self, slug: &str, state: &SyncState and gives back Result<(), String>.

(Where these words came from: signature.)

---

## struct — ConfirmedFile

Found in src/cloud.rs at line 530.

Signature: `pub struct ConfirmedFile {`

What it does: A data shape named ConfirmedFile. Its fields are: pub sha256: String; pub etag: String.

(Where these words came from: signature.)

---

## struct — SyncState

Found in src/cloud.rs at line 536.

Signature: `pub struct SyncState {`

What it does: A data shape named SyncState. Its fields are: pub dest: String; pub confirmed: HashMap<String, ConfirmedFile>; pub last_report: Option<CloudReport>.

(Where these words came from: signature.)

---

## function — SyncState::confirmed_for

Found in src/cloud.rs at line 547.

Signature: `pub fn confirmed_for(&self, dest: &str) -> HashMap<String, ConfirmedFile> {`

What it does: The confirmations that count for this destination — none, if the state was recorded against another one.

(Where these words came from: doc comment.)

---

## function — to_lines

Found in src/cloud.rs at line 555.

Signature: `pub fn to_lines(&self) -> String {`

What it does: Function to_lines takes no inputs beyond the value it is called on, and gives back String.

(Where these words came from: signature.)

---

## function — from_lines

Found in src/cloud.rs at line 569.

Signature: `pub fn from_lines(text: &str) -> SyncState {`

What it does: Function from_lines takes text: &str and gives back SyncState.

(Where these words came from: signature.)

---

## struct — FileOutcome

Found in src/cloud.rs at line 603.

Signature: `pub struct FileOutcome {`

What it does: One failed file of a sync: its tree path and the provider's error, in the provider's words. (Successes and skips are counts on the report; failures are the per-file detail.)

(Where these words came from: doc comment.)

---

## struct — CloudReport

Found in src/cloud.rs at line 609.

Signature: `pub struct CloudReport {`

What it does: A data shape named CloudReport. Its fields are: pub provider: Provider; pub at: u64; pub uploaded: usize; pub skipped: usize; pub failed: usize; pub failures: Vec<FileOutcome>.

(Where these words came from: signature.)

---

## function — report_json

Found in src/cloud.rs at line 618.

Signature: `pub fn report_json(r: &CloudReport) -> String {`

What it does: Function report_json takes r: &CloudReport and gives back String.

(Where these words came from: signature.)

---

## struct — TreeFile

Found in src/cloud.rs at line 699.

Signature: `pub struct TreeFile {`

What it does: One file of the deployable tree: its path inside the tree, the content type a static host serves it under, and its bytes.

(Where these words came from: doc comment.)

---

## function — site_tree

Found in src/cloud.rs at line 712.

Signature: `pub fn site_tree(dir: &Path, bundle: &Bundle) -> Result<Vec<TreeFile>, String> {`

What it does: The payload: the outputs build's static site for this bundle, read from the on-disk tree it generates at publish (outputs.rs is the one tree producer — this module only carries it). The tree's own `deploy.zip` is skipped: it is the tree, zipped, and would be itself twice. If the tree is missing or visibly incomplete (a bundle that predates a sync), it is generated first, the same regeneration Store::open performs.

(Where these words came from: doc comment.)

---

## function — content_type_for

Found in src/cloud.rs at line 756.

Signature: `pub fn content_type_for(path: &str) -> &'static str {`

What it does: Function content_type_for takes path: &str and gives back &'static str.

(Where these words came from: signature.)

---

## function — base64_encode

Found in src/cloud.rs at line 930.

Signature: `pub fn base64_encode(data: &[u8]) -> String {`

What it does: Function base64_encode takes data: &[u8] and gives back String.

(Where these words came from: signature.)

---

## function — base64_decode

Found in src/cloud.rs at line 955.

Signature: `pub fn base64_decode(text: &str) -> Option<Vec<u8>> {`

What it does: Function base64_decode takes text: &str and gives back Option<Vec<u8>>.

(Where these words came from: signature.)

---

## function — sync_if_configured

Found in src/cloud.rs at line 1369.

Signature: `pub fn sync_if_configured(store: &CloudStore, bundle: &Bundle) -> Option<CloudReport> {`

What it does: The publish hook, in the WordPress adapter's shape: when the bundle has a cloud connection, sync the just-committed bundle and return the report; the publish itself has already landed and is never touched by the outcome.

(Where these words came from: doc comment.)

---

## function — sync_bundle

Found in src/cloud.rs at line 1399.

Signature: `pub fn sync_bundle( store: &CloudStore, bundle: &Bundle, settings: &CloudSettings, ) -> Result<CloudReport, String> {`

What it does: Sync on demand (the retry): the stored connection, the stored bundle, no re-publish.

(Where these words came from: doc comment.)

---

## const — DEFAULT_ISSUER

Found in src/fed.rs at line 83.

Signature: `pub const DEFAULT_ISSUER: &str = "https://edit.phext.io";`

What it does: A fixed value named DEFAULT_ISSUER, set to "https://edit.phext.io".

(Where these words came from: signature.)

---

## const — VAULT_PREFIX

Found in src/fed.rs at line 84.

Signature: `pub const VAULT_PREFIX: &str = "mdwiz-vault-";`

What it does: A fixed value named VAULT_PREFIX, set to "mdwiz-vault-".

(Where these words came from: signature.)

---

## struct — FedConfig

Found in src/fed.rs at line 95.

Signature: `pub struct FedConfig {`

What it does: A data shape named FedConfig. Its fields are: pub enabled: bool; pub issuer: String.

(Where these words came from: signature.)

---

## function — FedConfig::from_env

Found in src/fed.rs at line 101.

Signature: `pub fn from_env() -> FedConfig {`

What it does: Function FedConfig::from_env takes no inputs beyond the value it is called on, and gives back FedConfig.

(Where these words came from: signature.)

---

## struct — Fed

Found in src/fed.rs at line 148.

Signature: `pub struct Fed {`

What it does: A data shape named Fed. The scanner did not read its fields.

(Where these words came from: signature.)

---

## function — Fed::open

Found in src/fed.rs at line 285.

Signature: `pub fn open(data_dir: &std::path::Path, cfg: FedConfig) -> std::io::Result<Fed> {`

What it does: Open the federation state under `<data>/fed/`. A torn or unreadable state file is not fatal: federation starts empty (everyone signs in again) and says so on stderr — the rest of the service never depends on this state.

(Where these words came from: doc comment.)

---

## function — enabled

Found in src/fed.rs at line 312.

Signature: `pub fn enabled(&self) -> bool {`

What it does: Function enabled takes no inputs beyond the value it is called on, and gives back bool.

(Where these words came from: signature.)

---

## function — issuer

Found in src/fed.rs at line 316.

Signature: `pub fn issuer(&self) -> &str {`

What it does: Function issuer takes no inputs beyond the value it is called on, and gives back &str.

(Where these words came from: signature.)

---

## function — Fed::handle_login

Found in src/fed.rs at line 1326.

Signature: `pub fn handle_login( &self, stream: &mut TcpStream, headers: &Map<String, String>, public_base: &str, ) {`

What it does: GET /auth/login — begin the flow. The browser is sent to edit.phext.io's authorization endpoint with a PKCE challenge; the verifier stays in this server's memory, keyed by the state nonce, for ten minutes.

(Where these words came from: doc comment.)

---

## function — handle_callback

Found in src/fed.rs at line 1385.

Signature: `pub fn handle_callback( &self, stream: &mut TcpStream, headers: &Map<String, String>, target: &str, ) {`

What it does: GET /auth/callback — the provider returns the browser here with a code (or an error). The exchange happens server-to-server; the browser leaves with only our own session cookie.

(Where these words came from: doc comment.)

---

## function — handle_logout

Found in src/fed.rs at line 1496.

Signature: `pub fn handle_logout(&self, stream: &mut TcpStream, headers: &Map<String, String>) {`

What it does: POST /auth/logout — forget the session here. (The provider exposes no revocation endpoint; the grant itself expires on its own schedule. Our tokens are gone from this server the moment this answers.)

(Where these words came from: doc comment.)

---

## function — handle_me

Found in src/fed.rs at line 1523.

Signature: `pub fn handle_me( &self, stream: &mut TcpStream, headers: &Map<String, String>, head_only: bool, ) {`

What it does: GET /api/me — the editor's one boot question. Deliberately makes no provider calls: it reports configuration and the session this server already holds.

(Where these words came from: doc comment.)

---

## function — handle_vault_status

Found in src/fed.rs at line 1593.

Signature: `pub fn handle_vault_status( &self, stream: &mut TcpStream, headers: &Map<String, String>, head_only: bool, ) {`

What it does: GET /api/vault — the vault dialog's whole picture.

(Where these words came from: doc comment.)

---

## function — handle_vault_new

Found in src/fed.rs at line 1673.

Signature: `pub fn handle_vault_new( &self, stream: &mut TcpStream, reader: &mut BufReader<TcpStream>, headers: &Map<String, String>, ) {`

What it does: POST /api/vault/new {label} — choose a fresh vault name for this session. The phext itself does not exist until it is hosted; the starter download carries its manifest.

(Where these words came from: doc comment.)

---

## function — handle_vault_choose

Found in src/fed.rs at line 1721.

Signature: `pub fn handle_vault_choose( &self, stream: &mut TcpStream, reader: &mut BufReader<TcpStream>, headers: &Map<String, String>, ) {`

What it does: POST /api/vault/choose {name, label?} — adopt a hosted vault as this session's. A root scroll that is present but not ours refuses; an absent one means an empty phext the first sync will initialize.

(Where these words came from: doc comment.)

---

## function — handle_vault_starter

Found in src/fed.rs at line 1787.

Signature: `pub fn handle_vault_starter( &self, stream: &mut TcpStream, headers: &Map<String, String>, head_only: bool, ) {`

What it does: GET /api/vault/starter — the vault as a one-scroll phext file: the manifest itself, at the root coordinate, which is all a phext with one scroll at 1.1.1/1.1.1/1.1.1 is. The user hosts this file (edit.phext.io/submit exists for exactly this) and the vault is live.

(Where these words came from: doc comment.)

---

## function — handle_vault_sync

Found in src/fed.rs at line 1960.

Signature: `pub fn handle_vault_sync( &self, stream: &mut TcpStream, reader: &mut BufReader<TcpStream>, headers: &Map<String, String>, ) {`

What it does: POST /api/vault/sync — push one bundle's local state toward the vault. See the module header for the model; the planning table is `plan_push`, and this function is its executor: file scrolls first, then the bundle manifest, then the vault manifest, so an interruption leaves at worst unlisted content a later sync re-plans.

(Where these words came from: doc comment.)

---

## function — handle_vault_pull

Found in src/fed.rs at line 2130.

Signature: `pub fn handle_vault_pull( &self, stream: &mut TcpStream, reader: &mut BufReader<TcpStream>, headers: &Map<String, String>, ) {`

What it does: POST /api/vault/pull — bring the vault's copy of one bundle toward this browser. Nothing here writes anywhere: the answer carries the texts to take, the conflicts to decide, and the honest lists (what local is ahead on, what local deleted and the vault did not resurrect).

(Where these words came from: doc comment.)

---

## const — HTML

Found in src/http.rs at line 20.

Signature: `pub const HTML: &str = "text/html; charset=utf-8";`

What it does: A fixed value named HTML, set to "text/html; charset=utf-8".

(Where these words came from: signature.)

---

## const — TEXT

Found in src/http.rs at line 21.

Signature: `pub const TEXT: &str = "text/plain; charset=utf-8";`

What it does: A fixed value named TEXT, set to "text/plain; charset=utf-8".

(Where these words came from: signature.)

---

## const — JSON

Found in src/http.rs at line 22.

Signature: `pub const JSON: &str = "application/json; charset=utf-8";`

What it does: A fixed value named JSON, set to "application/json; charset=utf-8".

(Where these words came from: signature.)

---

## enum — CacheClass

Found in src/http.rs at line 26.

Signature: `pub enum CacheClass {`

What it does: How a response may be cached — the lattice's three classes.

(Where these words came from: doc comment.)

---

## const — HSTS_VALUE

Found in src/http.rs at line 48.

Signature: `pub const HSTS_VALUE: &str = "max-age=31536000";`

What it does: One year, matching `deploy/nginx-md.phext.io.conf`. No `includeSubDomains`: this host is `md.phext.io`, and a future sibling name should not be pinned by accident. (Same reasoning as the lattice's HSTS note for edit.phext.io.)

(Where these words came from: doc comment.)

---

## const — CONTENT_SECURITY_POLICY

Found in src/http.rs at line 51.

Signature: `pub const CONTENT_SECURITY_POLICY: &str = concat!( "default-src 'self'; ",`

What it does: The lattice CSP, with the one documented `img-src` deviation.

(Where these words came from: doc comment.)

---

## function — forwarded_https

Found in src/http.rs at line 67.

Signature: `pub fn forwarded_https(headers: &HashMap<String, String>) -> bool {`

What it does: True when the proxy says the browser connection is HTTPS — the lattice's `forwarded_https`, verbatim in shape.

(Where these words came from: doc comment.)

---

## function — status_text

Found in src/http.rs at line 95.

Signature: `pub fn status_text(status: u16) -> &'static str {`

What it does: The lattice's status-text table, extended with the codes this service uses (409, 415, 422).

(Where these words came from: doc comment.)

---

## function — strong_etag

Found in src/http.rs at line 120.

Signature: `pub fn strong_etag(body: &[u8]) -> String {`

What it does: A strong ETag: the quoted SHA-256 hex of the body (the lattice's `strong_etag`, which hashes with SHA-256 the same way).

(Where these words came from: doc comment.)

---

## function — parse_request

Found in src/http.rs at line 142.

Signature: `pub fn parse_request( reader: &mut BufReader<TcpStream>, ) -> Option<(String, String, HashMap<String, String>)> {`

What it does: Parse a request line and headers — the lattice's `parse_request` shape: method, raw target, and a lower-cased header map.

(Where these words came from: doc comment.)

---

## function — read_body

Found in src/http.rs at line 200.

Signature: `pub fn read_body( reader: &mut BufReader<TcpStream>, headers: &HashMap<String, String>, cap: usize, ) -> Option<Vec<u8>> {`

What it does: Read a request body as bytes, up to `cap`. Returns `None` when the declared length exceeds the cap (the caller answers 413). The lattice reads bodies as lossy text; bundles are binary, so this service reads bytes — same Content-Length discipline.

(Where these words came from: doc comment.)

---

## function — response_bytes

Found in src/http.rs at line 223.

Signature: `pub fn response_bytes( status: u16, content_type: &str, body: &[u8], class: CacheClass, req_headers: &HashMap<String, String>, head_only: bool, extra_headers: &str, ) -> Vec<u8> {`

What it does: Build a full response — the lattice's `response_bytes`: ETag and 304 handling for the cacheable classes, security headers on everything, `Connection: close`.

(Where these words came from: doc comment.)

---

## function — send_response

Found in src/http.rs at line 265.

Signature: `pub fn send_response( stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8], class: CacheClass, req_headers: &HashMap<String, String>, head_only: bool, ) {`

What it does: Function send_response takes stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8], class: CacheClass, req_headers: &HashMap<String, String>, head_only: bool, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — send_response_extra

Found in src/http.rs at line 277.

Signature: `pub fn send_response_extra( stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8], class: CacheClass, req_headers: &HashMap<String, String>, head_only: bool, extra_headers: &str, ) {`

What it does: Function send_response_extra takes stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8], class: CacheClass, req_headers: &HashMap<String, String>, head_only: bool, extra_headers: &str, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — send_redirect

Found in src/http.rs at line 292.

Signature: `pub fn send_redirect(stream: &mut TcpStream, location: &str, req_headers: &HashMap<String, String>) {`

What it does: Function send_redirect takes stream: &mut TcpStream, location: &str, req_headers: &HashMap<String, String> and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — json_escape

Found in src/http.rs at line 310.

Signature: `pub fn json_escape(s: &str) -> String {`

What it does: Minimal JSON string escaping for the hand-formatted API responses (the lattice formats JSON with serde; this service's responses are small and fixed-shape, so they are formatted directly).

(Where these words came from: doc comment.)

---

## function — html_escape

Found in src/http.rs at line 327.

Signature: `pub fn html_escape(s: &str) -> String {`

What it does: HTML-escape text for interpolation into the page templates.

(Where these words came from: doc comment.)

---

## function — xml_escape

Found in src/http.rs at line 350.

Signature: `pub fn xml_escape(s: &str) -> String {`

What it does: XML-escape text for the RSS feed (same five characters as HTML, without the apostrophe form XML parsers also accept). Control characters XML 1.0 forbids outright — everything below U+0020 except tab, line feed, and carriage return — are replaced with U+FFFD: left raw, a single one (a stray bell or vertical tab in a markdown source, say) makes the whole feed not well-formed, and every subscriber's parser refuses it — the same silent death as one unescaped `&`, by another character.

(Where these words came from: doc comment.)

---

## function — percent_decode

Found in src/http.rs at line 371.

Signature: `pub fn percent_decode(s: &str) -> String {`

What it does: Percent-decode a URL path segment string. `+` is left alone (it is a literal plus in a path, not a space). Invalid sequences pass through unchanged; the caller's path rules reject anything odd.

(Where these words came from: doc comment.)

---

## entry point — main

Found in src/main.rs at line 105.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: signature.)

---

## command — --dir

Found in src/main.rs at line 73.

Signature: `command "--dir"`

What it does: A command named --dir. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — --port

Found in src/main.rs at line 79.

Signature: `command "--port"`

What it does: A command named --port. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — --host

Found in src/main.rs at line 85.

Signature: `command "--host"`

What it does: A command named --host. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — --help

Found in src/main.rs at line 91.

Signature: `command "--help"`

What it does: A command named --help. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — provider

Found in src/main.rs at line 742.

Signature: `command "provider"`

What it does: A command named provider. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — bucket

Found in src/main.rs at line 747.

Signature: `command "bucket"`

What it does: A command named bucket. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## command — container

Found in src/main.rs at line 750.

Signature: `command "container"`

What it does: A command named container. The program runs it when the user types this word on the command line.

(Where these words came from: signature.)

---

## function — handle_mcp

Found in src/mcp.rs at line 63.

Signature: `pub fn handle_mcp( stream: &mut TcpStream, reader: &mut BufReader<TcpStream>, method: &str, headers: &HashMap<String, String>, store: &Arc<Store>, work: &Arc<WorkStore>, wp: &Arc<WpStore>, cloudstore: &Arc<CloudStore>, ) {`

What it does: Function handle_mcp takes stream: &mut TcpStream, reader: &mut BufReader<TcpStream>, method: &str, headers: &HashMap<String, String>, store: &Arc<Store>, work: &Arc<WorkStore>, wp: &Arc<WpStore>, cloudstore: &Arc<CloudStore>, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## const — CANONICAL_BASE

Found in src/outputs.rs at line 62.

Signature: `pub const CANONICAL_BASE: &str = "https://md.phext.io";`

What it does: The site's canonical public origin. The exported feed is generated against it, so the feed inside a downloaded site stays a valid feed whose items point at the live bundle (a feed reader needs absolute addresses; a downloaded tree has none of its own to give).

(Where these words came from: doc comment.)

---

## function — outputs_incomplete

Found in src/outputs.rs at line 79.

Signature: `pub fn outputs_incomplete(data_dir: &Path, bundle: &Bundle) -> bool {`

What it does: Whether a bundle's on-disk outputs are missing or visibly incomplete: no site tree at all, or a Chromium is available now but the tree holds no PDF for a bundle that has markdown — the mark of a publish made while no Chromium existed. Store::open regenerates the outputs of any bundle this names.

(Where these words came from: doc comment.)

---

## function — generate

Found in src/outputs.rs at line 127.

Signature: `pub fn generate(data_dir: &Path, bundle: &Bundle) -> Result<(), String> {`

What it does: Build (or rebuild) the derived outputs for one committed bundle and swap them into `<data>/<slug>/site/`. Filesystem failures are the caller's to log; a missing Chromium is not a failure — it is the degraded state (the bundle simply has no PDFs).

(Where these words came from: doc comment.)

---

## function — render_markdown

Found in src/render.rs at line 68.

Signature: `pub fn render_markdown(md: &str) -> String {`

What it does: Render markdown to an HTML fragment.

(Where these words came from: doc comment.)

---

## function — first_heading

Found in src/render.rs at line 175.

Signature: `pub fn first_heading(md: &str) -> Option<String> {`

What it does: The first `# heading` of a markdown text, for page titles.

(Where these words came from: doc comment.)

---

## const — THEME_SNIPPET

Found in src/render.rs at line 207.

Signature: `pub const THEME_SNIPPET: &str = r#"(function () {`

What it does: Theme resolution, run before first paint on every published page — the same snippet the editor carries in its head. The reader's stored choice ("mdwiz.theme", set by the editor's toggle on this origin) wins; otherwise the OS preference; otherwise light. The stylesheet below also carries a no-JS fallback keyed on the media query alone.

(Where these words came from: doc comment.)

---

## const — SITE_CSS

Found in src/render.rs at line 222.

Signature: `pub const SITE_CSS: &str = r#" :root {`

What it does: The shared stylesheet: quiet, editorial, paper and ink with one deep-indigo accent. System serif for display and body, system sans for the small chrome, mono for addresses. No webfonts, no requests — the page is the whole of it. Dark is the same printed page at night: same serif, the one accent lifted, paper gone deep and warm, ink gone soft.

(Where these words came from: doc comment.)

---

## const — QUIET_LINE

Found in src/render.rs at line 379.

Signature: `pub const QUIET_LINE: &str = "Flat files are a fine start. When you want \ addresses in eleven dimensions, try <a href=\"https://edit.phext.io\">edit.phext.io</a>.";`

What it does: The quiet suggestion — it lives in exactly two places: the editor's footer and the bundle index page's footer.

(Where these words came from: doc comment.)

---

## function — version_line

Found in src/render.rs at line 384.

Signature: `pub fn version_line(bundle: &Bundle) -> String {`

What it does: `Version 3 · (Your Version) · updated 15 November 2026`, plus the release's change line when one was given.

(Where these words came from: doc comment.)

---

## function — index_page

Found in src/render.rs at line 426.

Signature: `pub fn index_page(bundle: &Bundle, base: &str) -> String {`

What it does: The bundle index: `/b/<slug>/`. A tracklist under the bundle's own self-description — the site is generated from the bundle alone.

(Where these words came from: doc comment.)

---

## function — reading_page

Found in src/render.rs at line 500.

Signature: `pub fn reading_page(bundle: &Bundle, file: &BundleFile, base: &str) -> String {`

What it does: One file's reading view: `/b/<slug>/<path>` (extension dropped). A printed page: title, rule, body, neighbours, colophon.

(Where these words came from: doc comment.)

---

## function — integrate_page

Found in src/render.rs at line 587.

Signature: `pub fn integrate_page() -> String {`

What it does: The ways-to-use-it page: `/integrate`. A new writer meets the editor first; this page answers the next question — where a bundle can go. Four doors, in order: the reader's own LLM over MCP, their WordPress, their own hosting, and the edit.phext.io vault for drafts that should travel with them. It closes with one sentence of kin for tluf.org. It is kept by the Pencil Puffball, worn once as a round badge in the opening — inside the hairline ring, the artwork's cream ground reads deliberately in both themes.

(Where these words came from: doc comment.)

---

## function — pdf_unavailable_page

Found in src/render.rs at line 659.

Signature: `pub fn pdf_unavailable_page(bundle: &Bundle, file: &BundleFile) -> String {`

What it does: The PDF address, when this server has no Chromium to print with: a page that says so plainly, in the same quiet dress, and points at the page itself — which is whole, and always was the primary form.

(Where these words came from: doc comment.)

---

## function — not_found_page

Found in src/render.rs at line 679.

Signature: `pub fn not_found_page(what: &str) -> String {`

What it does: The 404 page, in the same quiet dress.

(Where these words came from: doc comment.)

---

## const — MAX_FEED_ITEMS

Found in src/rss.rs at line 69.

Signature: `pub const MAX_FEED_ITEMS: usize = 25;`

What it does: Items in one bundle's feed.

(Where these words came from: doc comment.)

---

## const — MAX_SITE_FEED_ITEMS

Found in src/rss.rs at line 71.

Signature: `pub const MAX_SITE_FEED_ITEMS: usize = 50;`

What it does: Items in the site-wide feed.

(Where these words came from: doc comment.)

---

## function — rfc822

Found in src/rss.rs at line 79.

Signature: `pub fn rfc822(unix: u64) -> String {`

What it does: Format a Unix timestamp as an RFC 822 date in GMT: `Sat, 15 Nov 2026 22:20:00 GMT`.

(Where these words came from: doc comment.)

---

## function — long_date

Found in src/rss.rs at line 98.

Signature: `pub fn long_date(unix: u64) -> String {`

What it does: Format a Unix timestamp as a long human date: `15 November 2026`.

(Where these words came from: doc comment.)

---

## function — websub_hub

Found in src/rss.rs at line 125.

Signature: `pub fn websub_hub() -> Option<&'static str> {`

What it does: The configured WebSub hub, if any: `MDWIZ_WEBSUB_HUB`, read once at first use. Unset or empty means no hub — feeds carry no hub link and commits ping nothing.

(Where these words came from: doc comment.)

---

## function — feed_xml

Found in src/rss.rs at line 252.

Signature: `pub fn feed_xml(bundle: &Bundle, base: &str) -> String {`

What it does: The bundle's feed. `base` is the site's public origin with no trailing slash (e.g. `https://md.phext.io`); item links are built from it so feed readers land on real addresses.

(Where these words came from: doc comment.)

---

## function — site_feed_xml

Found in src/rss.rs at line 282.

Signature: `pub fn site_feed_xml(bundles: &[&Bundle], base: &str) -> String {`

What it does: The site-wide feed: the items of every published bundle on the instance, newest first. It lives only on the live instance — the static-site download is one bundle's artifact and carries that bundle's own feed, never this one.

(Where these words came from: doc comment.)

---

## function — ping_hub

Found in src/rss.rs at line 308.

Signature: `pub fn ping_hub(bundle: &Bundle) {`

What it does: Announce a commit to the WebSub hub, if one is configured: the bundle's feed and the site-wide feed both changed. Called from the store's commit point (publish, MCP publish, and restore alike), after the commit has landed and the outputs are built. A detached thread and a swallowed error: the hub's health is never the publisher's problem.

(Where these words came from: doc comment.)

---

## struct — FileStamps

Found in src/store.rs at line 65.

Signature: `pub struct FileStamps {`

What it does: When one file was first published, and when its content last changed. A bundle's feed dates each item by its own file's `last_changed`, so re-releasing a bundle for one file's sake does not re-date the rest.

(Where these words came from: doc comment.)

---

## struct — Bundle

Found in src/store.rs at line 72.

Signature: `pub struct Bundle {`

What it does: One published bundle, immutable once built. Readers clone the `Arc` and drop the lock immediately — the snapshot pattern.

(Where these words came from: doc comment.)

---

## function — Bundle::stamps_of

Found in src/store.rs at line 96.

Signature: `pub fn stamps_of(&self, path: &str) -> (u64, u64) {`

What it does: One file's (first published, last changed) stamps. Files with no stamps of their own — anything published before per-file stamps existed — fall back to the bundle's stamp: as far as the record knows, they were born and last changed when the bundle was.

(Where these words came from: doc comment.)

---

## struct — Store

Found in src/store.rs at line 104.

Signature: `pub struct Store {`

What it does: A data shape named Store. The scanner did not read its fields.

(Where these words came from: signature.)

---

## enum — PublishOutcome

Found in src/store.rs at line 117.

Signature: `pub enum PublishOutcome {`

What it does: A named choice named PublishOutcome. It is one of: Created; Replaced.

(Where these words came from: signature.)

---

## enum — PublishError

Found in src/store.rs at line 125.

Signature: `pub enum PublishError {`

What it does: A named choice named PublishError. It is one of: Invalid; WrongKey; NoPrevious; Stale; Io.

(Where these words came from: signature.)

---

## function — clean_changed_line

Found in src/store.rs at line 158.

Signature: `pub fn clean_changed_line(s: &str) -> Option<String> {`

What it does: Sanitize a "what changed?" line: one line, printable, capped.

(Where these words came from: doc comment.)

---

## function — Store::open

Found in src/store.rs at line 177.

Signature: `pub fn open(dir: PathBuf) -> std::io::Result<Store> {`

What it does: Open the store at `dir`, loading every committed bundle. Unparseable or torn directories are skipped with a note on stderr — an uncommitted state is not a state.

(Where these words came from: doc comment.)

---

## function — get

Found in src/store.rs at line 217.

Signature: `pub fn get(&self, slug: &str) -> Option<Arc<Bundle>> {`

What it does: Function get takes &self, slug: &str and gives back Option<Arc<Bundle>>.

(Where these words came from: signature.)

---

## function — list

Found in src/store.rs at line 223.

Signature: `pub fn list(&self) -> Vec<Arc<Bundle>> {`

What it does: Every published bundle, ordered by slug — the MCP `list_bundles` surface.

(Where these words came from: doc comment.)

---

## function — count

Found in src/store.rs at line 230.

Signature: `pub fn count(&self) -> usize {`

What it does: Function count takes no inputs beyond the value it is called on, and gives back usize.

(Where these words came from: signature.)

---

## function — has_previous

Found in src/store.rs at line 235.

Signature: `pub fn has_previous(&self, slug: &str) -> bool {`

What it does: Whether a previous published version survives for this slug.

(Where these words came from: doc comment.)

---

## function — publish

Found in src/store.rs at line 255.

Signature: `pub fn publish( &self, slug: &str, zip_bytes: Vec<u8>, supplied_key: Option<&str>, changed: Option<String>, ) -> Result<PublishOutcome, PublishError> {`

What it does: Publish a bundle. `supplied_key` is required iff the slug already exists. The whole operation is: verify in memory, check the key, preserve the current as previous, stage + sync + rename + sync, swap the Arc.

(Where these words came from: doc comment.)

---

## function — publish_from_draft

Found in src/store.rs at line 390.

Signature: `pub fn publish_from_draft( &self, slug: &str, zip_bytes: Vec<u8>, supplied_key: Option<&str>, key_sha256: String, base_version: u64, changed: Option<String>, ) -> Result<Arc<Bundle>, PublishError> {`

What it does: Publish a working copy's archive, checked against the version the draft was based on — the check and the commit under the slug's one lock, so a web publish landing between the draft's own check and its commit cannot be silently overwritten (nor overwrite the draft's commit). A first publish (base 0) goes through the issued-key path — the draft minted the slug's key when it was opened; a re-publish goes through the ordinary key-checked path.

(Where these words came from: doc comment.)

---

## function — restore

Found in src/store.rs at line 423.

Signature: `pub fn restore(&self, slug: &str, supplied_key: Option<&str>) -> Result<Arc<Bundle>, PublishError> {`

What it does: Restore the previous published version — the reversibility that replaces confirmation. Runs the same commit path with the previous bytes and the previous meta (its own version, date, and change line come back with it), after preserving the current state into `prev/`, so a restore is itself reversible.

(Where these words came from: doc comment.)

---

## function — site_file

Found in src/store.rs at line 529.

Signature: `pub fn site_file(&self, slug: &str, rel: &str) -> Option<Vec<u8>> {`

What it does: One file of a bundle's derived outputs (see outputs.rs), read from `<slug>/site/`. `rel` must be a path the caller has already validated as bundle-shaped (no `..`, no vault) — the outputs routes only ever pass the request path itself, after the same checks the bundle routes apply.

(Where these words came from: doc comment.)

---

## struct — WpSettings

Found in src/wordpress.rs at line 53.

Signature: `pub struct WpSettings {`

What it does: The per-bundle WordPress connection. `password` is the site user's application password; it exists in memory and in the settings file, and nowhere else.

(Where these words came from: doc comment.)

---

## enum — FileOutcome

Found in src/wordpress.rs at line 60.

Signature: `pub enum FileOutcome {`

What it does: A named choice named FileOutcome. It is one of: Created; Updated; Failed.

(Where these words came from: signature.)

---

## struct — FileResult

Found in src/wordpress.rs at line 67.

Signature: `pub struct FileResult {`

What it does: A data shape named FileResult. Its fields are: pub path: String; pub outcome: FileOutcome; pub post_id: Option<u64>; pub link: Option<String>.

(Where these words came from: signature.)

---

## struct — SyncReport

Found in src/wordpress.rs at line 77.

Signature: `pub struct SyncReport {`

What it does: One sync run, recorded with the bundle.

(Where these words came from: doc comment.)

---

## function — SyncReport::to_json

Found in src/wordpress.rs at line 87.

Signature: `pub fn to_json(&self) -> String {`

What it does: Function SyncReport::to_json takes no inputs beyond the value it is called on, and gives back String.

(Where these words came from: signature.)

---

## struct — WpState

Found in src/wordpress.rs at line 133.

Signature: `pub struct WpState {`

What it does: The bundle's sync bookkeeping: the path -> post ID mapping, and the last run's report.

(Where these words came from: doc comment.)

---

## struct — WpStore

Found in src/wordpress.rs at line 141.

Signature: `pub struct WpStore {`

What it does: Persistence for the per-bundle WordPress files, under the same data directory the store owns. Writes follow the store's durability shape: stage beside the home, sync, rename.

(Where these words came from: doc comment.)

---

## function — WpStore::new

Found in src/wordpress.rs at line 146.

Signature: `pub fn new(dir: PathBuf) -> WpStore {`

What it does: Function WpStore::new takes dir: PathBuf and gives back WpStore.

(Where these words came from: signature.)

---

## function — load_settings

Found in src/wordpress.rs at line 156.

Signature: `pub fn load_settings(&self, slug: &str) -> Option<WpSettings> {`

What it does: Function load_settings takes &self, slug: &str and gives back Option<WpSettings>.

(Where these words came from: signature.)

---

## function — save_settings

Found in src/wordpress.rs at line 187.

Signature: `pub fn save_settings(&self, slug: &str, settings: &WpSettings) -> std::io::Result<()> {`

What it does: Function save_settings takes &self, slug: &str, settings: &WpSettings and gives back std::io::Result<()>.

(Where these words came from: signature.)

---

## function — clear

Found in src/wordpress.rs at line 200.

Signature: `pub fn clear(&self, slug: &str) -> std::io::Result<()> {`

What it does: Forget the connection and the bookkeeping both. The posts on WordPress are untouched — disconnecting is a local act.

(Where these words came from: doc comment.)

---

## function — load_state

Found in src/wordpress.rs at line 214.

Signature: `pub fn load_state(&self, slug: &str) -> WpState {`

What it does: Function load_state takes &self, slug: &str and gives back WpState.

(Where these words came from: signature.)

---

## function — save_state

Found in src/wordpress.rs at line 276.

Signature: `pub fn save_state(&self, slug: &str, state: &WpState) -> std::io::Result<()> {`

What it does: Function save_state takes &self, slug: &str, state: &WpState and gives back std::io::Result<()>.

(Where these words came from: signature.)

---

## function — clean_site_url

Found in src/wordpress.rs at line 401.

Signature: `pub fn clean_site_url(raw: &str) -> Result<String, String> {`

What it does: Normalize and vet a site address the owner typed. Returns the address to store (no trailing slash) or the plain-language reason it cannot be used. Plain HTTP is accepted only for a site on this machine — an application password must not cross the network in the clear.

(Where these words came from: doc comment.)

---

## function — clean_user

Found in src/wordpress.rs at line 496.

Signature: `pub fn clean_user(raw: &str) -> Result<String, String> {`

What it does: Function clean_user takes raw: &str and gives back Result<String, String>.

(Where these words came from: signature.)

---

## function — publishable_markdown

Found in src/wordpress.rs at line 509.

Signature: `pub fn publishable_markdown(bundle: &Bundle) -> Vec<(String, String, String)> {`

What it does: The files a publish sends to WordPress: the bundle's markdown, in bundle order, the vault excluded (bundle order already excludes it). Carried files (images and the like) are not posts.

(Where these words came from: doc comment.)

---

## function — wp_slug_for

Found in src/wordpress.rs at line 527.

Signature: `pub fn wp_slug_for(path: &str) -> String {`

What it does: `essays/one.md` -> `essays-one`: the whole stem, dashes for slashes, so two files in different folders never collide on a WordPress slug.

(Where these words came from: doc comment.)

---

## function — sync_bundle

Found in src/wordpress.rs at line 547.

Signature: `pub fn sync_bundle(wp: &WpStore, bundle: &Bundle, settings: &WpSettings) -> SyncReport {`

What it does: Sync a bundle to its WordPress site, if one is connected. Returns the report, which is also persisted with the state. The mapping is saved after every file WordPress answers for, so a crash mid-sync cannot make the next run create a duplicate of a post this run already created.

(Where these words came from: doc comment.)

---

## function — sync_if_configured

Found in src/wordpress.rs at line 599.

Signature: `pub fn sync_if_configured(wp: &WpStore, bundle: &Bundle) -> Option<SyncReport> {`

What it does: The push that rides a publish — the web release, an MCP publish, and a restore all ride the same hook. A restore is a publish of the older state: it changes what is published, and WordPress follows the published state — the bundle is the source of truth either way. Runs only when a connection is saved for the bundle, and never fails the publish it rides: the sync records its own failures per file.

(Where these words came from: doc comment.)

---

## function — top_level_string

Found in src/wordpress.rs at line 877.

Signature: `pub fn top_level_string(json: &str, key: &str) -> Option<String> {`

What it does: A top-level string field, unescaped.

(Where these words came from: doc comment.)

---

## function — top_level_u64

Found in src/wordpress.rs at line 883.

Signature: `pub fn top_level_u64(json: &str, key: &str) -> Option<u64> {`

What it does: A top-level unsigned-integer field.

(Where these words came from: doc comment.)

---

## function — json_string_field

Found in src/wordpress.rs at line 891.

Signature: `pub fn json_string_field(body: &str, key: &str) -> Option<String> {`

What it does: One string field from a flat JSON object — the shape of the settings request body the editor sends. Same scanner; the body is an object, so its fields are top-level by construction.

(Where these words came from: doc comment.)

---

## function — base64_encode

Found in src/wordpress.rs at line 897.

Signature: `pub fn base64_encode(bytes: &[u8]) -> String {`

What it does: Function base64_encode takes bytes: &[u8] and gives back String.

(Where these words came from: signature.)

---

## struct — WorkingCopy

Found in src/work.rs at line 59.

Signature: `pub struct WorkingCopy {`

What it does: One slug's draft: a mutable file set behind a lock, with the version it was seeded from and the hash of its only credential.

(Where these words came from: doc comment.)

---

## function — WorkingCopy::file

Found in src/work.rs at line 73.

Signature: `pub fn file(&self, path: &str) -> Option<&BundleFile> {`

What it does: Function WorkingCopy::file takes &self, path: &str and gives back Option<&BundleFile>.

(Where these words came from: signature.)

---

## function — is_private

Found in src/work.rs at line 77.

Signature: `pub fn is_private(&self, path: &str) -> bool {`

What it does: Function is_private takes &self, path: &str and gives back bool.

(Where these words came from: signature.)

---

## function — verify

Found in src/work.rs at line 81.

Signature: `pub fn verify(&self, supplied_key: Option<&str>) -> bool {`

What it does: Function verify takes &self, supplied_key: Option<&str> and gives back bool.

(Where these words came from: signature.)

---

## function — total_bytes

Found in src/work.rs at line 87.

Signature: `pub fn total_bytes(&self) -> u64 {`

What it does: Function total_bytes takes no inputs beyond the value it is called on, and gives back u64.

(Where these words came from: signature.)

---

## enum — WorkError

Found in src/work.rs at line 92.

Signature: `pub enum WorkError {`

What it does: A named choice named WorkError. It is one of: WrongKey; NoWorkingCopy; Invalid; Stale; Io.

(Where these words came from: signature.)

---

## struct — OpenedCopy

Found in src/work.rs at line 133.

Signature: `pub struct OpenedCopy {`

What it does: The result of opening a working copy.

(Where these words came from: doc comment.)

---

## struct — WriteInfo

Found in src/work.rs at line 140.

Signature: `pub struct WriteInfo {`

What it does: A data shape named WriteInfo. Its fields are: pub files: usize; pub base_version: u64; pub diverged: bool; pub minted_key: Option<String>; pub keep_private: bool.

(Where these words came from: signature.)

---

## function — diverged

Found in src/work.rs at line 154.

Signature: `pub fn diverged(store: &Store, slug: &str, copy: &WorkingCopy) -> bool {`

What it does: Does this draft differ from what's published? The three-state word (Drafting / Current / Diverged) rests on this comparison. The comparison is against the publishable set: a file marked keep-private is not a difference from the published bundle — it is deliberately not in it.

(Where these words came from: doc comment.)

---

## struct — PublishedInfo

Found in src/work.rs at line 175.

Signature: `pub struct PublishedInfo {`

What it does: A data shape named PublishedInfo. Its fields are: pub version: u64; pub published_at: u64; pub files: Vec<String>; pub kept_private: Vec<String>.

(Where these words came from: signature.)

---

## struct — WorkStore

Found in src/work.rs at line 184.

Signature: `pub struct WorkStore {`

What it does: A data shape named WorkStore. The scanner did not read its fields.

(Where these words came from: signature.)

---

## function — WorkStore::open

Found in src/work.rs at line 193.

Signature: `pub fn open(dir: PathBuf) -> std::io::Result<WorkStore> {`

What it does: Open the working-copy store at `dir`, loading every persisted draft. Unparseable work directories are skipped with a note — an uncommitted draft is not a draft.

(Where these words came from: doc comment.)

---

## function — get

Found in src/work.rs at line 223.

Signature: `pub fn get(&self, slug: &str) -> Option<Arc<Mutex<WorkingCopy>>> {`

What it does: Function get takes &self, slug: &str and gives back Option<Arc<Mutex<WorkingCopy>>>.

(Where these words came from: signature.)

---

## function — open_or_seed

Found in src/work.rs at line 231.

Signature: `pub fn open_or_seed( &self, store: &Store, slug: &str, supplied_key: Option<&str>, ) -> Result<OpenedCopy, WorkError> {`

What it does: Open the working copy for `slug`, seeding or minting it when none exists yet. Seeding from a published bundle requires the bundle's edit key; minting for a fresh slug issues the key. An existing copy always requires its key.

(Where these words came from: doc comment.)

---

## function — write_file

Found in src/work.rs at line 304.

Signature: `pub fn write_file( &self, store: &Store, slug: &str, path: &str, content: &[u8], supplied_key: Option<&str>, keep_private: Option<bool>, ) -> Result<WriteInfo, WorkError> {`

What it does: Create or replace one file in the slug's working copy, opening the copy first when needed, and persist the draft before returning — a write that answered is a write that survives a restart. `keep_private`, when carried, sets or clears the file's keep-private mark with the same write; when absent, the mark stands as it was.

(Where these words came from: doc comment.)

---

## function — publish

Found in src/work.rs at line 383.

Signature: `pub fn publish( &self, store: &Store, slug: &str, supplied_key: Option<&str>, changed: Option<String>, ) -> Result<PublishedInfo, WorkError> {`

What it does: Commit the slug's working copy through the store's own publish path — the same gate and the same atomic swap as a web release. Refuses when the published bundle has moved past the draft's base version. The commit set is the publishable set: files marked keep-private are held back (they stay in the working copy), so the published bundle never contains them — and because a publish replaces the whole published tree, a file marked after it was live leaves the published surface with this commit.

(Where these words came from: doc comment.)

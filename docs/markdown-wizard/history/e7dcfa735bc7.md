# markdown-wizard — reference as of commit e7dcfa735bc7

Commit e7dcfa735bc718d9ee4b4f723ed4d8c0b02dc8f2. 19 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/markdown-wizard/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

## Dependencies

### Cargo.toml

- pulldown-cmark — 0.12
- sha2 — 0.10
- tempfile — 3
- zip — 2
- serde_json — 1
- ureq — 2
- serde — 1

## Authors

The author of record is **Will Bickford** — 19 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Theia — 11 commits
- Emise — 7 commits
- Will Bickford (his own identities) — 1 commit

## Complexity

- JavaScript: 4 files, 390 lines total, 0 blank, 0 comment, 390 code
- Markdown: 3 files, 758 lines total, 0 blank, 0 comment, 758 code
- Rust: 17 files, 14797 lines total, 0 blank, 0 comment, 14797 code
- Shell: 3 files, 523 lines total, 0 blank, 0 comment, 523 code
- TOML: 1 files, 46 lines total, 0 blank, 0 comment, 46 code

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
- struct `CloudReport` (src/cloud.rs) — A data shape named CloudReport.
- struct `CloudSettings` (src/cloud.rs) — One bundle's cloud connection, all three providers' fields in one flat record (the `cloud.txt` lines form); only the active provider's fields are read by the engine.
- function `CloudSettings::to_lines` (src/cloud.rs) — Function CloudSettings::to_lines takes no inputs beyond the value it is called on, and gives back String.
- struct `CloudStore` (src/cloud.rs) — The per-bundle cloud files, beside the bundle in the data dir: `cloud.txt` (the connection — a credential file, written mode 0600 by staged rename) and `cloud-state.txt` (the sync state).
- function `CloudStore::new` (src/cloud.rs) — Function CloudStore::new takes dir: PathBuf and gives back CloudStore.
- struct `ConfirmedFile` (src/cloud.rs) — A data shape named ConfirmedFile.
- struct `FileOutcome` (src/cloud.rs) — One failed file of a sync: its tree path and the provider's error, in the provider's words.
- enum `Provider` (src/cloud.rs) — A named choice named Provider.
- function `Provider::parse` (src/cloud.rs) — Function Provider::parse takes s: &str and gives back Option<Provider>.
- struct `SyncState` (src/cloud.rs) — A data shape named SyncState.
- function `SyncState::confirmed_for` (src/cloud.rs) — The confirmations that count for this destination — none, if the state was recorded against another one.
- struct `TreeFile` (src/cloud.rs) — One file of the deployable tree: its path inside the tree, the content type a static host serves it under, and its bytes
- function `as_str` (src/cloud.rs) — Function as_str takes no inputs beyond the value it is called on, and gives back &'static str.
- function `base64_decode` (src/cloud.rs) — Function base64_decode takes text: &str and gives back Option<Vec<u8>>.
- function `base64_encode` (src/cloud.rs) — Function base64_encode takes data: &[u8] and gives back String.
- function `clear` (src/cloud.rs) — Disconnect: both files go — the state belongs to the connection it describes.
- function `content_type_for` (src/cloud.rs) — Function content_type_for takes path: &str and gives back &'static str.
- function `destination_id` (src/cloud.rs) — The identity of the destination the sync state belongs to.
- function `from_lines` (src/cloud.rs) — Function from_lines takes text: &str and gives back SyncState.
- function `load_settings` (src/cloud.rs) — Function load_settings takes &self, slug: &str and gives back Option<CloudSettings>.
- function `load_state` (src/cloud.rs) — Function load_state takes &self, slug: &str and gives back SyncState.
- function `merge_settings` (src/cloud.rs) — Merge a settings request body (the same `key=value` lines as `cloud.txt`) over the settings already held, and validate the result.
- function `normalize_prefix` (src/cloud.rs) — A prefix is a folder: no leading slash, a trailing slash when it names anything.
- function `record` (src/cloud.rs) — Function record takes &self, slug: &str, state: &SyncState and gives back Result<(), String>.
- function `report_json` (src/cloud.rs) — Function report_json takes r: &CloudReport and gives back String.
- function `save_settings` (src/cloud.rs) — Function save_settings takes &self, slug: &str, settings: &CloudSettings and gives back Result<(), String>.
- function `settings_json` (src/cloud.rs) — The settings as the API answers them: everything the editor needs to render the connection, and no secret — a key ID is 
- function `site_tree` (src/cloud.rs) — The payload: the outputs build's static site for this bundle, read from the on-disk tree it generates at publish (outputs.rs is the one tree producer — this module only carries it).
- function `sync_bundle` (src/cloud.rs) — Sync on demand (the retry): the stored connection, the stored bundle, no re-publish.
- function `sync_if_configured` (src/cloud.rs) — The publish hook, in the WordPress adapter's shape: when the bundle has a cloud connection, sync the just-committed bund
- function `to_lines` (src/cloud.rs) — Function to_lines takes no inputs beyond the value it is called on, and gives back String.
- function `validate_settings` (src/cloud.rs) — Plain-language validation; the messages are user-facing and the settings API returns them verbatim.
- const `DEFAULT_ISSUER` (src/fed.rs) — A fixed value named DEFAULT_ISSUER, set to "https://edit.phext.io".
- struct `Fed` (src/fed.rs) — A data shape named Fed.
- function `Fed::handle_login` (src/fed.rs) — GET /auth/login — begin the flow.
- function `Fed::open` (src/fed.rs) — Open the federation state under `<data>/fed/`.
- struct `FedConfig` (src/fed.rs) — A data shape named FedConfig.
- function `FedConfig::from_env` (src/fed.rs) — Function FedConfig::from_env takes no inputs beyond the value it is called on, and gives back FedConfig.
- const `VAULT_PREFIX` (src/fed.rs) — A fixed value named VAULT_PREFIX, set to "mdwiz-vault-".
- function `enabled` (src/fed.rs) — Function enabled takes no inputs beyond the value it is called on, and gives back bool.
- function `handle_callback` (src/fed.rs) — GET /auth/callback — the provider returns the browser here with a code (or an error).
- function `handle_logout` (src/fed.rs) — POST /auth/logout — forget the session here.
- function `handle_me` (src/fed.rs) — GET /api/me — the editor's one boot question.
- function `handle_vault_choose` (src/fed.rs) — POST /api/vault/choose {name, label?} — adopt a hosted vault as this session's.
- function `handle_vault_new` (src/fed.rs) — POST /api/vault/new {label} — choose a fresh vault name for this session.
- function `handle_vault_pull` (src/fed.rs) — POST /api/vault/pull — bring the vault's copy of one bundle toward this browser.
- function `handle_vault_starter` (src/fed.rs) — GET /api/vault/starter — the vault as a one-scroll phext file: the manifest itself, at the root coordinate, which is all a phext with one scroll at 1.1.1/1.1.1/1.1.1 is.
- function `handle_vault_status` (src/fed.rs) — GET /api/vault — the vault dialog's whole picture.
- function `handle_vault_sync` (src/fed.rs) — POST /api/vault/sync — push one bundle's local state toward the vault.
- function `issuer` (src/fed.rs) — Function issuer takes no inputs beyond the value it is called on, and gives back &str.
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
- command `bucket` (src/main.rs) — A command named bucket.
- command `container` (src/main.rs) — A command named container.
- entry point `main` (src/main.rs) — The entry point: the program starts here.
- command `provider` (src/main.rs) — A command named provider.
- function `handle_mcp` (src/mcp.rs) — Function handle_mcp takes stream: &mut TcpStream, reader: &mut BufReader<TcpStream>, method: &str, headers: &HashMap<Str
- const `CANONICAL_BASE` (src/outputs.rs) — The site's canonical public origin.
- function `generate` (src/outputs.rs) — Build (or rebuild) the derived outputs for one committed bundle and swap them into `<data>/<slug>/site/`.
- function `outputs_incomplete` (src/outputs.rs) — Whether a bundle's on-disk outputs are missing or visibly incomplete: no site tree at all, or a Chromium is available now but the tree holds no PDF for a bundle that has markdown — the mark of a publish made while no Chromium existed.
- const `QUIET_LINE` (src/render.rs) — The quiet suggestion — it lives in exactly two places: the editor's footer and the bundle index page's footer.
- const `SITE_CSS` (src/render.rs) — The shared stylesheet: quiet, editorial, paper and ink with one deep-indigo accent.
- const `THEME_SNIPPET` (src/render.rs) — Theme resolution, run before first paint on every published page — the same snippet the editor carries in its head.
- function `first_heading` (src/render.rs) — The first `# heading` of a markdown text, for page titles.
- function `index_page` (src/render.rs) — The bundle index: `/b/<slug>/`.
- function `integrate_page` (src/render.rs) — The ways-to-use-it page: `/integrate`.
- function `not_found_page` (src/render.rs) — The 404 page, in the same quiet dress.
- function `pdf_unavailable_page` (src/render.rs) — The PDF address, when this server has no Chromium to print with: a page that says so plainly, in the same quiet dress, a
- function `reading_page` (src/render.rs) — One file's reading view: `/b/<slug>/<path>` (extension dropped).
- function `render_markdown` (src/render.rs) — Render markdown to an HTML fragment.
- function `version_line` (src/render.rs) — `Version 3 · (Your Version) · updated 15 November 2026`, plus the release's change line when one was given.
- const `MAX_FEED_ITEMS` (src/rss.rs) — Items in one bundle's feed.
- const `MAX_SITE_FEED_ITEMS` (src/rss.rs) — Items in the site-wide feed.
- function `feed_xml` (src/rss.rs) — The bundle's feed.
- function `long_date` (src/rss.rs) — Format a Unix timestamp as a long human date: `15 November 2026`.
- function `ping_hub` (src/rss.rs) — Announce a commit to the WebSub hub, if one is configured: the bundle's feed and the site-wide feed both changed.
- function `rfc822` (src/rss.rs) — Format a Unix timestamp as an RFC 822 date in GMT: `Sat, 15 Nov 2026 22:20:00 GMT`.
- function `site_feed_xml` (src/rss.rs) — The site-wide feed: the items of every published bundle on the instance, newest first.
- function `websub_hub` (src/rss.rs) — The configured WebSub hub, if any: `MDWIZ_WEBSUB_HUB`, read once at first use.
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
- function `publish_from_draft` (src/store.rs) — Publish a working copy's archive, checked against the version the draft was based on — the check and the commit under the slug's one lock, so a web publish landing between the draft's own check and its commit cannot be silently overwritten (nor overwrite the draft's commit).
- function `restore` (src/store.rs) — Restore the previous published version — the reversibility that replaces confirmation.
- function `site_file` (src/store.rs) — One file of a bundle's derived outputs (see outputs.rs), read from `<slug>/site/`.
- enum `FileOutcome` (src/wordpress.rs) — A named choice named FileOutcome.
- struct `FileResult` (src/wordpress.rs) — A data shape named FileResult.
- struct `SyncReport` (src/wordpress.rs) — One sync run, recorded with the bundle.
- function `SyncReport::to_json` (src/wordpress.rs) — Function SyncReport::to_json takes no inputs beyond the value it is called on, and gives back String.
- struct `WpSettings` (src/wordpress.rs) — The per-bundle WordPress connection.
- struct `WpState` (src/wordpress.rs) — The bundle's sync bookkeeping: the path -> post ID mapping, and the last run's report.
- struct `WpStore` (src/wordpress.rs) — Persistence for the per-bundle WordPress files, under the same data directory the store owns.
- function `WpStore::new` (src/wordpress.rs) — Function WpStore::new takes dir: PathBuf and gives back WpStore.
- function `base64_encode` (src/wordpress.rs) — Function base64_encode takes bytes: &[u8] and gives back String.
- function `clean_site_url` (src/wordpress.rs) — Normalize and vet a site address the owner typed.
- function `clean_user` (src/wordpress.rs) — Function clean_user takes raw: &str and gives back Result<String, String>.
- function `clear` (src/wordpress.rs) — Forget the connection and the bookkeeping both.
- function `json_string_field` (src/wordpress.rs) — One string field from a flat JSON object — the shape of the settings request body the editor sends.
- function `load_settings` (src/wordpress.rs) — Function load_settings takes &self, slug: &str and gives back Option<WpSettings>.
- function `load_state` (src/wordpress.rs) — Function load_state takes &self, slug: &str and gives back WpState.
- function `publishable_markdown` (src/wordpress.rs) — The files a publish sends to WordPress: the bundle's markdown, in bundle order, the vault excluded (bundle order already excludes it).
- function `save_settings` (src/wordpress.rs) — Function save_settings takes &self, slug: &str, settings: &WpSettings and gives back std::io::Result<()>.
- function `save_state` (src/wordpress.rs) — Function save_state takes &self, slug: &str, state: &WpState and gives back std::io::Result<()>.
- function `sync_bundle` (src/wordpress.rs) — Sync a bundle to its WordPress site, if one is connected.
- function `sync_if_configured` (src/wordpress.rs) — The push that rides a publish — the web release, an MCP publish, and a restore all ride the same hook.
- function `top_level_string` (src/wordpress.rs) — A top-level string field, unescaped.
- function `top_level_u64` (src/wordpress.rs) — A top-level unsigned-integer field.
- function `wp_slug_for` (src/wordpress.rs) — `essays/one.md` -> `essays-one`: the whole stem, dashes for slashes, so two files in different folders never collide on 
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

# phext-lanes — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 150). Scanned at commit c7aaa1b63455cafe94dc75f762274a72d54a784e. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

Description stated in the manifest: Predictive multi-chat stored as a single phext file

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 16.

- axum [dependencies] — =0.7.9
- chrono [dependencies] — =0.4.39
- clap [dependencies] — =4.5.23
- hyper [dependencies] — =1.5.2
- hyper-util [dependencies] — =0.1.10
- libphext [dependencies] — =0.3.3
- memmap2 [dependencies] — =0.9.5
- serde [dependencies] — =1.0.217
- serde_json [dependencies] — =1.0.134
- hmac [dependencies] — =0.12.1
- sha2 [dependencies] — =0.10.8
- tokio [dependencies] — =1.42.0
- ureq [dependencies] — =2.8.0
- url [dependencies] — =2.5.2
- http-body-util [dev-dependencies] — =0.1.5
- tower [dev-dependencies] — =0.5.3

Total packages in the lock file, this one included: 135.


---

## Authors

The git log holds 29 commits.

The author of record is **Will Bickford** — 14 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 13 commits
- Verse — 1 commit

Other authors, kept as themselves (not Exocortical cluster personas; never folded):

- Cursor Agent <cursoragent@cursor.com> — 14 commits
- Cursor <noreply@cursor.com> — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 1 file, 279 lines total, 94 blank, 2 comment, 183 code
- Rust: 20 files, 11369 lines total, 769 blank, 335 comment, 10265 code
- Shell: 1 file, 832 lines total, 51 blank, 49 comment, 732 code
- TOML: 1 file, 29 lines total, 2 blank, 2 comment, 25 code

All languages: 23 files, 12509 lines total, 11205 lines of code. Functions read: 528.

Largest files:

- src/server.rs — 3545 lines
- src/store.rs — 1276 lines
- src/oauth.rs — 933 lines
- src/rank.rs — 894 lines
- deploy.sh — 832 lines

Largest functions, by span in lines:

- main in deploy.sh — about 395 lines
- into_response in src/server.rs — about 190 lines
- rollback_nginx in deploy.sh — about 188 lines
- handle in src/server.rs — about 138 lines
- world_from_scrolls in src/store.rs — about 120 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 105 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 53 of 255 (21%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/phext-lanes
- Branch: exo
- Head commit: c7aaa1b63455cafe94dc75f762274a72d54a784e
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `usage` (deploy.sh:28) — Function usage takes no inputs beyond the value it is called on.
- function `expand_tilde` (deploy.sh:53) — Function expand_tilde takes no inputs beyond the value it is called on.
- function `die` (deploy.sh:67) — Function die takes no inputs beyond the value it is called on.
- function `port_is_foreign` (deploy.sh:73) — Function port_is_foreign takes no inputs beyond the value it is called on.
- function `ensure_rate_limit` (deploy.sh:92) — Function ensure_rate_limit takes no inputs beyond the value it is called on.
- function `site_matches` (deploy.sh:144) — Function site_matches takes no inputs beyond the value it is called on.
- function `ss_listen_dump` (deploy.sh:160) — Function ss_listen_dump takes no inputs beyond the value it is called on.
- function `public_ipv4` (deploy.sh:178) — Function public_ipv4 takes no inputs beyond the value it is called on.
- function `domain_v4s` (deploy.sh:200) — Function domain_v4s takes no inputs beyond the value it is called on.
- function `dns_points_at` (deploy.sh:211) — Function dns_points_at takes no inputs beyond the value it is called on.
- function `http_code` (deploy.sh:222) — Function http_code takes no inputs beyond the value it is called on.
- function `certbot_print` (deploy.sh:232) — Function certbot_print takes no inputs beyond the value it is called on.
- function `ensure_build_path` (deploy.sh:242) — Function ensure_build_path takes no inputs beyond the value it is called on.
- function `main` (deploy.sh:250) — Function main takes no inputs beyond the value it is called on.
- function `rollback_nginx` (deploy.sh:645) — Function rollback_nginx takes no inputs beyond the value it is called on.
- entry point `main` (examples/bench.rs:60) [example] — The entry point: the program starts here.
- trait `PlotHolderCheck` (src/auth.rs:12) — Proof that the caller holds a founding plot.
- struct `AdminTokenCheck` (src/auth.rs:17) — Deny everyone except a bearer of the configured admin token.
- function `AdminTokenCheck::from_env` (src/auth.rs:22) — Function AdminTokenCheck::from_env takes no inputs beyond the value it is called on.
- function `AdminTokenCheck::from_token` (src/auth.rs:30) — Function AdminTokenCheck::from_token takes token: impl Into<String>.
- type `Version` (src/cache.rs:15) — Store version and hub generation the body was built from.
- struct `Cache` (src/cache.rs:23) — awaiting a summary
- function `Cache::new` (src/cache.rs:30) — Function Cache::new takes ttl: Duration, capacity: usize.
- function `Cache::get` (src/cache.rs:38) — Function Cache::get takes &self, key: &str, version: Version.
- function `Cache::put` (src/cache.rs:48) — Function Cache::put takes &self, key: String, version: Version, body: Bytes.
- function `Cache::len` (src/cache.rs:71) — Function Cache::len takes no inputs beyond the value it is called on.
- function `Cache::is_empty` (src/cache.rs:78) — Function Cache::is_empty takes no inputs beyond the value it is called on.
- function `format_feed` (src/calm.rs:11) — Function format_feed takes feed: &Feed, details: bool.
- function `held_count` (src/calm.rs:56) — Function held_count takes feed: &Feed.
- function `format_thread` (src/calm.rs:72) — Function format_thread takes thread: &Thread, details: bool, limits: &rank::Limits.
- const `DEFAULT_USER` (src/config.rs:13) — A fixed value named DEFAULT_USER, set to "mira".
- const `DEFAULT_FILE` (src/config.rs:14) — A fixed value named DEFAULT_FILE, set to "data/chat.phext".
- const `DEFAULT_HOST` (src/config.rs:15) — A fixed value named DEFAULT_HOST, set to "127.0.0.1".
- const `DEFAULT_PORT` (src/config.rs:16) — A fixed value named DEFAULT_PORT, set to 7841.
- struct `Settings` (src/config.rs:19) — A data shape named Settings.
- struct `Listen` (src/config.rs:25) — A data shape named Listen.
- struct `Limits` (src/config.rs:40) — How much the feed shows and how fast people may write.
- function `Limits::is_flooded` (src/config.rs:83) — True when one author wrote enough of a large enough thread.
- function `Limits::check` (src/config.rs:88) — The first value out of range, as `(key, allowed range, value)`.
- struct `LimitOverrides` (src/config.rs:125) — Feed limits one lane overrides on top of the server's [`Limits`].
- const `KEYS` (src/config.rs:135) — A fixed value named KEYS, set to [.
- function `LimitOverrides::apply` (src/config.rs:144) — Function LimitOverrides::apply takes &self, base: Limits.
- function `LimitOverrides::is_empty` (src/config.rs:156) — Function LimitOverrides::is_empty takes no inputs beyond the value it is called on.
- function `LimitOverrides::lines` (src/config.rs:161) — `key: value` lines for the ones that are set.
- function `LimitOverrides::read` (src/config.rs:185) — Read one `key: value` line.
- function `load_limits` (src/config.rs:214) — Limits from the environment and `lanes.json`, checked for range.
- function `resolve_limits` (src/config.rs:224) — Function resolve_limits takes env: impl Fn.
- function `load` (src/config.rs:287) — Function load takes flag_user: Option<&str>, flag_file: Option<&Path>.
- function `resolve` (src/config.rs:305) — Function resolve takes flag_user: Option<&str>, flag_file: Option<&Path>, env_user: Option<&str>, env_file: Option<&str>, config_json: Option<&str>,.
- function `load_listen` (src/config.rs:341) — Function load_listen takes flag_host: Option<&str>, flag_port: Option<u16>.
- function `resolve_listen` (src/config.rs:352) — Function resolve_listen takes flag_host: Option<&str>, flag_port: Option<u16>, env_host: Option<&str>, env_port: Option<&str>,.
- const `ROSTER_LIBRARY` (src/coord.rs:12) — Library reserved for reader records.
- const `SETTINGS_COORD` (src/coord.rs:16) — The lane's settings scroll: owners and limit overrides.
- const `WEIGHTS` (src/coord.rs:21) — Library, shelf, series, collection, volume, book.
- function `parse_coord` (src/coord.rs:23) — Function parse_coord takes input: &str.
- enum `Target` (src/coord.rs:35) — A named choice named Target.
- function `parse_target` (src/coord.rs:43) — Function parse_target takes input: &str.
- function `format_coord` (src/coord.rs:56) — Function format_coord takes coord: Coordinate.
- function `topic_key` (src/coord.rs:60) — Function topic_key takes coord: Coordinate.
- function `thread_key` (src/coord.rs:64) — Function thread_key takes coord: Coordinate.
- function `section_key` (src/coord.rs:69) — Chapter and section stay.
- function `section_anchor` (src/coord.rs:78) — Function section_anchor takes coord: Coordinate.
- function `thread_anchor` (src/coord.rs:84) — Function thread_anchor takes coord: Coordinate.
- function `with_scroll` (src/coord.rs:92) — Function with_scroll takes thread: Coordinate, scroll: usize.
- function `is_roster` (src/coord.rs:98) — Function is_roster takes coord: Coordinate.
- function `is_card_slot` (src/coord.rs:102) — Function is_card_slot takes coord: Coordinate.
- function `arms` (src/coord.rs:106) — Function arms takes coord: Coordinate.
- function `weighted_distance` (src/coord.rs:117) — Function weighted_distance takes a: Coordinate, b: Coordinate.
- function `min_distance` (src/coord.rs:126) — Function min_distance takes thread: Coordinate, interests: &[Coordinate].
- function `next_message_scroll` (src/coord.rs:136) — Smallest free message scroll in chapter 1, section 1.
- function `sanitize` (src/coord.rs:151) — Strip dimension breaks so a message cannot split the phext.
- function `one_line` (src/coord.rs:171) — Function one_line takes text: &str.
- enum `Error` (src/error.rs:4) — A named choice named Error.
- const `MAX_FABRIC_BYTES` (src/fabric.rs:20) — A fixed value named MAX_FABRIC_BYTES, set to 5 * 1024 * 1024.
- const `MAX_FABRICS` (src/fabric.rs:21) — A fixed value named MAX_FABRICS, set to 100.
- enum `FabricError` (src/fabric.rs:26) — A named choice named FabricError.
- enum `Route` (src/fabric.rs:51) — A named choice named Route.
- struct `ParsedRoute` (src/fabric.rs:79) — A data shape named ParsedRoute.
- enum `RouteError` (src/fabric.rs:85) — A named choice named RouteError.
- function `parse_route` (src/fabric.rs:90) — Function parse_route takes path: &str.
- function `normalize_name` (src/fabric.rs:187) — Function normalize_name takes raw: &str.
- function `fabrics_dir` (src/fabric.rs:213) — Function fabrics_dir takes default_file: &Path.
- function `download_filename` (src/fabric.rs:220) — Function download_filename takes fabric: Option<&str>, lane_title: Option<&str>, day: &str.
- function `parse_upload` (src/fabric.rs:252) — Function parse_upload takes bytes: &[u8].
- function `roster_only` (src/fabric.rs:265) — Function roster_only takes no inputs beyond the value it is called on.
- function `lane_document` (src/fabric.rs:276) — Function lane_document takes doc: &str, thread: &Thread.
- struct `Hub` (src/fabric.rs:301) — awaiting a summary
- function `Hub::open` (src/fabric.rs:310) — Function Hub::open takes default_path: impl Into<PathBuf>.
- function `Hub::list` (src/fabric.rs:330) — Function Hub::list takes no inputs beyond the value it is called on.
- function `Hub::generation` (src/fabric.rs:356) — Changes whenever the list of fabrics does.
- function `Hub::with_store` (src/fabric.rs:360) — Function Hub::with_store takes &self, fabric: Option<&str>, body: impl FnOnce.
- function `Hub::create_empty` (src/fabric.rs:372) — Function Hub::create_empty takes &self, requested: Option<&str>, ip: &str.
- function `Hub::create_upload` (src/fabric.rs:376) — Function Hub::create_upload takes &self, requested: Option<&str>, bytes: &[u8], ip: &str,.
- function `day_stamp` (src/fabric.rs:556) — Function day_stamp takes now: chrono::DateTime<chrono::Utc>.
- const `JOIN_COOKIE` (src/invite.rs:18) — A fixed value named JOIN_COOKIE, set to "lanes_join".
- struct `Owners` (src/invite.rs:27) — awaiting a summary
- function `Owners::from_env` (src/invite.rs:32) — Function Owners::from_env takes no inputs beyond the value it is called on.
- function `Owners::parse` (src/invite.rs:39) — Function Owners::parse takes raw: &str.
- function `Owners::invite_only` (src/invite.rs:60) — Function Owners::invite_only takes no inputs beyond the value it is called on.
- function `Owners::is_empty` (src/invite.rs:64) — Function Owners::is_empty takes no inputs beyond the value it is called on.
- function `Owners::len` (src/invite.rs:68) — Function Owners::len takes no inputs beyond the value it is called on.
- function `Owners::matches` (src/invite.rs:72) — Function Owners::matches takes &self, who: &str, email: &str.
- function `beside` (src/invite.rs:96) — Default invite file: next to the phext, not inside the checkout when the phext lives in `/var/lib/phext-lanes`.
- function `resolve` (src/invite.rs:104) — `LANES_INVITES` wins.
- struct `Issued` (src/invite.rs:112) — A data shape named Issued.
- struct `Listed` (src/invite.rs:120) — A data shape named Listed.
- enum `RedeemFail` (src/invite.rs:130) — A named choice named RedeemFail.
- function `RedeemFail::message` (src/invite.rs:138) — Function RedeemFail::message takes no inputs beyond the value it is called on.
- function `create` (src/invite.rs:164) — Function create takes path: &Path, uses: u32, exp: Option<DateTime<Utc>>, now: DateTime<Utc>,.
- function `list` (src/invite.rs:197) — Function list takes path: &Path.
- function `revoke` (src/invite.rs:214) — Function revoke takes path: &Path, id: &str.
- function `redeem` (src/invite.rs:228) — Function redeem takes path: &Path, token: &str, now: DateTime<Utc>.
- function `token_shape` (src/invite.rs:261) — Function token_shape takes token: &str.
- re-export `error::Error` (src/lib.rs:27) — A re-export: error::Error is made reachable through this module, so other code can use it from here.
- enum `Bucket` (src/limit.rs:13) — A named choice named Bucket.
- struct `Limiter` (src/limit.rs:24) — awaiting a summary
- function `Limiter::standard` (src/limit.rs:32) — Function Limiter::standard takes no inputs beyond the value it is called on.
- function `Limiter::from_limits` (src/limit.rs:37) — The rate caps from the server's configured limits.
- function `Limiter::with_caps` (src/limit.rs:45) — Function Limiter::with_caps takes auth_per_ip: usize, writes_per_ip: usize, writes_per_account: usize.
- function `Limiter::hit` (src/limit.rs:55) — Function Limiter::hit takes &self, bucket: Bucket, id: &str.
- function `lane_id` (src/link.rs:23) — Function lane_id takes coord: Coordinate.
- function `post_id` (src/link.rs:27) — Function post_id takes coord: Coordinate.
- function `section_lane_id` (src/link.rs:32) — Address of one chapter and section: the coordinate of its first scroll.
- function `parse_lane_id` (src/link.rs:37) — A chat lane: the thread's card, `…/1.1.1`, or the six-part thread.
- function `parse_section_lane_id` (src/link.rs:51) — Function parse_section_lane_id takes id: &str.
- function `parse_post_id` (src/link.rs:66) — Function parse_post_id takes id: &str.
- function `canonical_lane` (src/link.rs:78) — The canonical nine-part address for a lane id in any accepted form.
- function `canonical_post` (src/link.rs:86) — The canonical address for a post id in any accepted form.
- struct `User` (src/model.rs:35) — A data shape named User.
- const `PLAIN_AUTHOR` (src/model.rs:43) — Author stamped on a scroll that is not a Lanes message.
- struct `Message` (src/model.rs:46) — A data shape named Message.
- function `Message::poster_key` (src/model.rs:59) — Share caps follow the OAuth identity when the scroll has one.
- struct `Thread` (src/model.rs:69) — A data shape named Thread.
- enum `Scroll` (src/model.rs:80) — A named choice named Scroll.
- struct `LaneSettings` (src/model.rs:99) — Who owns a lane and how its feed is limited.
- function `LaneSettings::to_scroll` (src/model.rs:106) — Function LaneSettings::to_scroll takes no inputs beyond the value it is called on.
- function `LaneSettings::is_owner` (src/model.rs:115) — Function LaneSettings::is_owner takes &self, who: &str.
- function `User::to_scroll` (src/model.rs:121) — Function User::to_scroll takes no inputs beyond the value it is called on.
- function `member_id` (src/model.rs:138) — Roster ids keep a single colon (`github:100`).
- function `format_card` (src/model.rs:151) — Function format_card takes topic: &str, title: &str, keywords: &str.
- function `format_message` (src/model.rs:160) — Function format_message takes author: &str, ts: DateTime<Utc>, text: &str.
- function `format_message_as` (src/model.rs:164) — Function format_message_as takes author: &str, who: &str, ts: DateTime<Utc>, text: &str.
- function `opening_line` (src/model.rs:180) — First non-empty line, clipped.
- function `rest_after_opening` (src/model.rs:195) — The scroll after its first non-empty line.
- function `title_from_text` (src/model.rs:217) — Function title_from_text takes text: &str.
- function `keyword_line` (src/model.rs:234) — Function keyword_line takes text: &str.
- function `parse_scroll` (src/model.rs:241) — Function parse_scroll takes raw: &str.
- function `content_tokens` (src/model.rs:356) — Lowercased content words.
- function `preview` (src/model.rs:377) — Function preview takes text: &str, limit: usize.
- const `TEST_CSRF` (src/oauth.rs:23) — A fixed value named TEST_CSRF, set to "csrf-test-token".
- struct `Identity` (src/oauth.rs:28) — A data shape named Identity.
- trait `IdentitySource` (src/oauth.rs:35) — awaiting a summary
- struct `Oauth` (src/oauth.rs:53) — awaiting a summary
- struct `Session` (src/oauth.rs:69) — A data shape named Session.
- struct `Pending` (src/oauth.rs:98) — A data shape named Pending.
- function `Oauth::from_env` (src/oauth.rs:163) — Function Oauth::from_env takes no inputs beyond the value it is called on.
- function `Oauth::test_ready` (src/oauth.rs:179) — Function Oauth::test_ready takes no inputs beyond the value it is called on.
- function `Oauth::closed` (src/oauth.rs:192) — Function Oauth::closed takes no inputs beyond the value it is called on.
- function `Oauth::ready` (src/oauth.rs:223) — Function Oauth::ready takes no inputs beyond the value it is called on.
- function `Oauth::providers` (src/oauth.rs:229) — Function Oauth::providers takes no inputs beyond the value it is called on.
- function `Oauth::cookie_secure` (src/oauth.rs:243) — Function Oauth::cookie_secure takes no inputs beyond the value it is called on.
- function `Oauth::authorize_url` (src/oauth.rs:247) — Function Oauth::authorize_url takes &self, provider_name: &str, next: &str,.
- function `Oauth::finish` (src/oauth.rs:291) — Function Oauth::finish takes &self, provider_name: &str, code: &str, state: &str, pending_cookie: &str,.
- function `Oauth::read_session` (src/oauth.rs:333) — Function Oauth::read_session takes &self, cookie_header: &str.
- function `Oauth::csrf_matches` (src/oauth.rs:355) — Function Oauth::csrf_matches takes &self, session: &Session, header: &str.
- function `Oauth::test_session_cookie` (src/oauth.rs:359) — Function Oauth::test_session_cookie takes no inputs beyond the value it is called on.
- function `session_cookie_name` (src/oauth.rs:418) — Function session_cookie_name takes no inputs beyond the value it is called on.
- function `pending_cookie_name` (src/oauth.rs:422) — Function pending_cookie_name takes no inputs beyond the value it is called on.
- function `cookie_header` (src/oauth.rs:426) — Function cookie_header takes name: &str, value: &str, max_age: i64, secure: bool.
- function `session_cookie` (src/oauth.rs:431) — Function session_cookie takes value: &str, secure: bool.
- function `pending_cookie` (src/oauth.rs:435) — Function pending_cookie takes value: &str, secure: bool.
- function `clear_cookie` (src/oauth.rs:439) — Function clear_cookie takes name: &str, secure: bool.
- function `safe_next` (src/oauth.rs:484) — Function safe_next takes next: &str.
- function `display_name` (src/oauth.rs:516) — Function display_name takes raw: &str, fallback: &str.
- function `clean_email` (src/oauth.rs:532) — Function clean_email takes raw: &str.
- function `clean_handle` (src/oauth.rs:553) — Function clean_handle takes raw: &str.
- function `pending_token` (src/oauth.rs:618) — Function pending_token takes cookie_header: &str.
- function `cookie_from_headers` (src/oauth.rs:622) — Function cookie_from_headers takes headers: &axum::http::HeaderMap.
- re-export `crate::config::Limits` (src/rank.rs:17) — A re-export: crate::config::Limits is made reachable through this module, so other code can use it from here.
- struct `Feed` (src/rank.rs:22) — A data shape named Feed.
- struct `FeedThread` (src/rank.rs:28) — A data shape named FeedThread.
- struct `Lane` (src/rank.rs:40) — A data shape named Lane.
- struct `Suggestion` (src/rank.rs:48) — A data shape named Suggestion.
- struct `MapEntry` (src/rank.rs:59) — A data shape named MapEntry.
- function `thread_score` (src/rank.rs:74) — Function thread_score takes distance: u64, age_secs: u64.
- function `nearness` (src/rank.rs:81) — Function nearness takes distance: u64.
- function `age_secs` (src/rank.rs:90) — Function age_secs takes ts: DateTime<Utc>, now: DateTime<Utc>.
- function `build_feed` (src/rank.rs:107) — Function build_feed takes threads: &[Thread], interests: &[Coordinate], now: DateTime<Utc>, limits: &Limits,.
- function `build_feed_for` (src/rank.rs:119) — [`build_feed`] for one reader.
- function `map_of` (src/rank.rs:239) — Function map_of takes threads: &[Thread], interests: &[Coordinate], limits: &Limits.
- function `majority` (src/rank.rs:267) — Function majority takes messages: &[Message].
- function `suggest_fresh` (src/rank.rs:285) — A new book on the reader's home shelf.
- function `suggest` (src/rank.rs:306) — Function suggest takes threads: &[Thread], interests: &[Coordinate], text: &str, now: DateTime<Utc>,.
- function `fresh_thread` (src/rank.rs:450) — Function fresh_thread takes threads: &[Thread], home: Coordinate.
- function `admitted_by` (src/rank.rs:505) — Function admitted_by takes feed: &Feed, author: &str.
- function `tokenize` (src/scrolls.rs:20) — Every non-empty scroll and where its text sits in `doc`, in document order.
- function `serialize` (src/scrolls.rs:45) — A phext document from scrolls in ascending coordinate order.
- function `textmap` (src/scrolls.rs:59) — One line per scroll, in the format of `phext::textmap`.
- const `MIRA` (src/seed.rs:12) — A fixed value named MIRA, set to "Mira Quill".
- const `JULES` (src/seed.rs:13) — A fixed value named JULES, set to "Jules Fen".
- const `ADA` (src/seed.rs:14) — A fixed value named ADA, set to "Ada North".
- const `REN` (src/seed.rs:15) — A fixed value named REN, set to "Ren Sato".
- const `CASEY` (src/seed.rs:16) — A fixed value named CASEY, set to "Casey Horn".
- const `HALL` (src/seed.rs:18) — A fixed value named HALL, set to "General".
- const `FERN` (src/seed.rs:19) — A fixed value named FERN, set to "Fern wall".
- const `SWAP` (src/seed.rs:20) — A fixed value named SWAP, set to "Seed swap".
- const `PLANES` (src/seed.rs:21) — A fixed value named PLANES, set to "Hand planes".
- const `RIVER` (src/seed.rs:22) — A fixed value named RIVER, set to "River wards".
- const `BREAD` (src/seed.rs:23) — A fixed value named BREAD, set to "Bread hour".
- const `HALL_THREAD` (src/seed.rs:25) — A fixed value named HALL_THREAD, set to "1.1.1/1.1.1".
- const `FERN_THREAD` (src/seed.rs:26) — A fixed value named FERN_THREAD, set to "1.2.1/1.1.1".
- const `SWAP_THREAD` (src/seed.rs:27) — A fixed value named SWAP_THREAD, set to "1.2.1/1.2.1".
- const `PLANES_THREAD` (src/seed.rs:28) — A fixed value named PLANES_THREAD, set to "2.1.1/1.1.1".
- const `RIVER_THREAD` (src/seed.rs:29) — A fixed value named RIVER_THREAD, set to "3.1.1/1.1.1".
- const `BREAD_THREAD` (src/seed.rs:30) — A fixed value named BREAD_THREAD, set to "4.1.1/1.1.1".
- function `demo_instant` (src/seed.rs:174) — Function demo_instant takes no inputs beyond the value it is called on.
- function `readers` (src/seed.rs:178) — Function readers takes no inputs beyond the value it is called on.
- const `PRIMER` (src/seed.rs:194) — A fixed value named PRIMER, set to "How phext works".
- const `PRIMER_THREAD` (src/seed.rs:195) — A fixed value named PRIMER_THREAD, set to "1.1.1/1.1.2".
- const `PRIMER_AUTHOR` (src/seed.rs:196) — A fixed value named PRIMER_AUTHOR, set to "Lanes".
- function `blank` (src/seed.rs:211) — A new chat: a General thread and the phext primer.
- function `document` (src/seed.rs:241) — Build the demo phext by inserting each scroll in coordinate order.
- struct `AppState` (src/server.rs:36) — awaiting a summary
- function `AppState::open` (src/server.rs:50) — Function AppState::open takes path: std::path::PathBuf, limits: Limits.
- function `AppState::with_auth` (src/server.rs:65) — Function AppState::with_auth takes path: std::path::PathBuf, auth: Arc<dyn PlotHolderCheck>,.
- function `listen_addr` (src/server.rs:172) — `host:port`, with brackets when `host` is an IPv6 address.
- function `serve` (src/server.rs:180) — Function serve takes path: std::path::PathBuf, host: &str, port: u16, limits: Limits,.
- function `router` (src/server.rs:224) — Function router takes state: AppState.
- struct `World` (src/store.rs:30) — A data shape named World.
- function `World::find_user` (src/store.rs:38) — Function World::find_user takes &self, query: &str.
- function `World::thread` (src/store.rs:54) — Function World::thread takes &self, key: &str.
- function `World::find_lane` (src/store.rs:65) — Function World::find_lane takes &self, id: &str.
- function `World::find_post` (src/store.rs:91) — The message scroll named by a post id.
- function `World::find_thread` (src/store.rs:103) — A thread title, or a thread or scroll coordinate.
- struct `Post` (src/store.rs:147) — A data shape named Post.
- struct `Placed` (src/store.rs:156) — A data shape named Placed.
- struct `Store` (src/store.rs:199) — awaiting a summary
- function `Store::open` (src/store.rs:215) — Function Store::open takes path: impl Into<PathBuf>.
- function `Store::path` (src/store.rs:230) — Function Store::path takes no inputs beyond the value it is called on.
- function `Store::doc` (src/store.rs:235) — The whole phext, straight from the mapping.
- function `Store::version` (src/store.rs:240) — Bumped on every save.
- function `Store::load_doc` (src/store.rs:245) — Load a fabric that already exists.
- function `Store::from_doc` (src/store.rs:257) — Function Store::from_doc takes path: impl Into<PathBuf>, doc: String.
- function `Store::text` (src/store.rs:300) — One scroll's text, borrowed from the mapping.
- function `Store::world` (src/store.rs:308) — The parsed fabric.
- function `Store::select` (src/store.rs:313) — One scroll, as `phext::fetch` returns it.
- function `Store::toc` (src/store.rs:318) — The contents listing, as `phext::textmap` writes it.
- function `Store::byte_index` (src/store.rs:323) — Where each scroll starts in the file, one coordinate per line.
- function `Store::reseed` (src/store.rs:331) — Function Store::reseed takes &mut self, now: DateTime<Utc>.
- function `Store::init_empty` (src/store.rs:339) — Create a chat with a General thread and no demo cast.
- function `Store::ensure_member` (src/store.rs:354) — Add a reader, or refresh their display name.
- function `Store::note_interest` (src/store.rs:394) — Remember a thread this member posted in or opened.
- function `Store::save_settings` (src/store.rs:418) — Write the settings scroll, replacing any earlier one.
- function `Store::place` (src/store.rs:422) — Function Store::place takes &mut self, post: Post.
- function `world_from` (src/store.rs:587) — Function world_from takes doc: &str.

---

## function — usage

Found in deploy.sh at line 28.

Surface: public API.

Signature: `usage()`

What it does: Function usage takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — expand_tilde

Found in deploy.sh at line 53.

Surface: public API.

Signature: `expand_tilde()`

What it does: Function expand_tilde takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — die

Found in deploy.sh at line 67.

Surface: public API.

Signature: `die()`

What it does: Function die takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — port_is_foreign

Found in deploy.sh at line 73.

Surface: public API.

Signature: `port_is_foreign()`

What it does: Function port_is_foreign takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — ensure_rate_limit

Found in deploy.sh at line 92.

Surface: public API.

Signature: `ensure_rate_limit()`

What it does: Function ensure_rate_limit takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — site_matches

Found in deploy.sh at line 144.

Surface: public API.

Signature: `site_matches()`

What it does: Function site_matches takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — ss_listen_dump

Found in deploy.sh at line 160.

Surface: public API.

Signature: `ss_listen_dump()`

What it does: Function ss_listen_dump takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — public_ipv4

Found in deploy.sh at line 178.

Surface: public API.

Signature: `public_ipv4()`

What it does: Function public_ipv4 takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — domain_v4s

Found in deploy.sh at line 200.

Surface: public API.

Signature: `domain_v4s()`

What it does: Function domain_v4s takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — dns_points_at

Found in deploy.sh at line 211.

Surface: public API.

Signature: `dns_points_at()`

What it does: Function dns_points_at takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — http_code

Found in deploy.sh at line 222.

Surface: public API.

Signature: `http_code()`

What it does: Function http_code takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — certbot_print

Found in deploy.sh at line 232.

Surface: public API.

Signature: `certbot_print()`

What it does: Function certbot_print takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — ensure_build_path

Found in deploy.sh at line 242.

Surface: public API.

Signature: `ensure_build_path()`

What it does: Function ensure_build_path takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — main

Found in deploy.sh at line 250.

Surface: public API.

Signature: `main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — rollback_nginx

Found in deploy.sh at line 645.

Surface: public API.

Signature: `rollback_nginx()`

What it does: Function rollback_nginx takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## entry point — main

Found in examples/bench.rs at line 60.

Surface: example.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## trait — PlotHolderCheck

Found in src/auth.rs at line 12.

Surface: public API.

Signature: `pub trait PlotHolderCheck: Send + Sync {`

What it does: Proof that the caller holds a founding plot. `proof` is the secret the request presented. An empty proof is never enough. A future Tessera check can replace the admin-token implementation without changing the call sites.

(Where these words came from: direct.)

---

## struct — AdminTokenCheck

Found in src/auth.rs at line 17.

Surface: public API.

Signature: `pub struct AdminTokenCheck {`

What it does: Deny everyone except a bearer of the configured admin token.

(Where these words came from: direct.)

---

## function — AdminTokenCheck::from_env

Found in src/auth.rs at line 22.

Surface: public API.

Signature: `pub fn from_env() -> Self {`

What it does: Function AdminTokenCheck::from_env takes no inputs beyond the value it is called on. It gives back Self.

(Where these words came from: inferred.)

---

## function — AdminTokenCheck::from_token

Found in src/auth.rs at line 30.

Surface: public API.

Signature: `pub fn from_token(token: impl Into<String>) -> Self {`

What it does: Function AdminTokenCheck::from_token takes token: impl Into<String>. It gives back Self.

(Where these words came from: inferred.)

---

## type — Version

Found in src/cache.rs at line 15.

Surface: public API.

Signature: `pub type Version = (u64, u64);`

What it does: Store version and hub generation the body was built from.

(Where these words came from: direct.)

---

## struct — Cache

Found in src/cache.rs at line 23.

Surface: public API.

Signature: `pub struct Cache {`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## function — Cache::new

Found in src/cache.rs at line 30.

Surface: public API.

Signature: `pub fn new(ttl: Duration, capacity: usize) -> Self {`

What it does: Function Cache::new takes ttl: Duration, capacity: usize. It gives back Self.

(Where these words came from: inferred.)

---

## function — Cache::get

Found in src/cache.rs at line 38.

Surface: public API.

Signature: `pub fn get(&self, key: &str, version: Version) -> Option<Bytes> {`

What it does: Function Cache::get takes &self, key: &str, version: Version. It gives back Option<Bytes>.

(Where these words came from: inferred.)

---

## function — Cache::put

Found in src/cache.rs at line 48.

Surface: public API.

Signature: `pub fn put(&self, key: String, version: Version, body: Bytes) {`

What it does: Function Cache::put takes &self, key: String, version: Version, body: Bytes. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — Cache::len

Found in src/cache.rs at line 71.

Surface: public API.

Signature: `pub fn len(&self) -> usize {`

What it does: Function Cache::len takes no inputs beyond the value it is called on. It gives back usize.

(Where these words came from: inferred.)

---

## function — Cache::is_empty

Found in src/cache.rs at line 78.

Surface: public API.

Signature: `pub fn is_empty(&self) -> bool {`

What it does: Function Cache::is_empty takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — format_feed

Found in src/calm.rs at line 11.

Surface: public API.

Signature: `pub fn format_feed(feed: &Feed, details: bool) -> String {`

What it does: Function format_feed takes feed: &Feed, details: bool. It gives back String.

(Where these words came from: inferred.)

---

## function — held_count

Found in src/calm.rs at line 56.

Surface: public API.

Signature: `pub fn held_count(feed: &Feed) -> usize {`

What it does: Function held_count takes feed: &Feed. It gives back usize.

(Where these words came from: inferred.)

---

## function — format_thread

Found in src/calm.rs at line 72.

Surface: public API.

Signature: `pub fn format_thread(thread: &Thread, details: bool, limits: &rank::Limits) -> String {`

What it does: Function format_thread takes thread: &Thread, details: bool, limits: &rank::Limits. It gives back String.

(Where these words came from: inferred.)

---

## const — DEFAULT_USER

Found in src/config.rs at line 13.

Surface: public API.

Signature: `pub const DEFAULT_USER: &str = "mira";`

What it does: A fixed value named DEFAULT_USER, set to "mira".

(Where these words came from: inferred.)

---

## const — DEFAULT_FILE

Found in src/config.rs at line 14.

Surface: public API.

Signature: `pub const DEFAULT_FILE: &str = "data/chat.phext";`

What it does: A fixed value named DEFAULT_FILE, set to "data/chat.phext".

(Where these words came from: inferred.)

---

## const — DEFAULT_HOST

Found in src/config.rs at line 15.

Surface: public API.

Signature: `pub const DEFAULT_HOST: &str = "127.0.0.1";`

What it does: A fixed value named DEFAULT_HOST, set to "127.0.0.1".

(Where these words came from: inferred.)

---

## const — DEFAULT_PORT

Found in src/config.rs at line 16.

Surface: public API.

Signature: `pub const DEFAULT_PORT: u16 = 7841;`

What it does: A fixed value named DEFAULT_PORT, set to 7841.

(Where these words came from: inferred.)

---

## struct — Settings

Found in src/config.rs at line 19.

Surface: public API.

Signature: `pub struct Settings {`

What it does: A data shape named Settings. Its fields are: pub user: String; pub file: PathBuf.

(Where these words came from: inferred.)

---

## struct — Listen

Found in src/config.rs at line 25.

Surface: public API.

Signature: `pub struct Listen {`

What it does: A data shape named Listen. Its fields are: pub host: String; pub port: u16.

(Where these words came from: inferred.)

---

## struct — Limits

Found in src/config.rs at line 40.

Surface: public API.

Signature: `pub struct Limits {`

What it does: How much the feed shows and how fast people may write.

(Where these words came from: direct.)

---

## function — Limits::is_flooded

Found in src/config.rs at line 83.

Surface: public API.

Signature: `pub fn is_flooded(&self, count: usize, share: f64) -> bool {`

What it does: True when one author wrote enough of a large enough thread.

(Where these words came from: direct.)

---

## function — Limits::check

Found in src/config.rs at line 88.

Surface: public API.

Signature: `pub fn check(&self) -> Result<(), (&'static str, &'static str, String)> {`

What it does: The first value out of range, as `(key, allowed range, value)`.

(Where these words came from: direct.)

---

## struct — LimitOverrides

Found in src/config.rs at line 125.

Surface: public API.

Signature: `pub struct LimitOverrides {`

What it does: Feed limits one lane overrides on top of the server's [`Limits`]. Rate limits are not here: they protect the whole server.

(Where these words came from: direct.)

---

## const — KEYS

Found in src/config.rs at line 135.

Surface: public API.

Signature: `pub const KEYS: [&'static str; 6] = [`

What it does: A fixed value named KEYS, set to [.

(Where these words came from: inferred.)

---

## function — LimitOverrides::apply

Found in src/config.rs at line 144.

Surface: public API.

Signature: `pub fn apply(&self, base: Limits) -> Limits {`

What it does: Function LimitOverrides::apply takes &self, base: Limits. It gives back Limits.

(Where these words came from: inferred.)

---

## function — LimitOverrides::is_empty

Found in src/config.rs at line 156.

Surface: public API.

Signature: `pub fn is_empty(&self) -> bool {`

What it does: Function LimitOverrides::is_empty takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — LimitOverrides::lines

Found in src/config.rs at line 161.

Surface: public API.

Signature: `pub fn lines(&self) -> Vec<String> {`

What it does: `key: value` lines for the ones that are set.

(Where these words came from: direct.)

---

## function — LimitOverrides::read

Found in src/config.rs at line 185.

Surface: public API.

Signature: `pub fn read(&mut self, key: &str, value: &str) {`

What it does: Read one `key: value` line. Unknown keys and bad numbers are skipped, so a hand-edited scroll never breaks the lane.

(Where these words came from: direct.)

---

## function — load_limits

Found in src/config.rs at line 214.

Surface: public API.

Signature: `pub fn load_limits() -> Result<Limits, Error> {`

What it does: Limits from the environment and `lanes.json`, checked for range.

(Where these words came from: direct.)

---

## function — resolve_limits

Found in src/config.rs at line 224.

Surface: public API.

Signature: `pub fn resolve_limits( env: impl Fn(&str) -> Option<String>, config_json: Option<&str>, ) -> Result<Limits, Error> {`

What it does: Function resolve_limits takes env: impl Fn. It gives back Option<String>, config_json: Option<&str>, ) -> Result<Limits, Error>.

(Where these words came from: inferred.)

---

## function — load

Found in src/config.rs at line 287.

Surface: public API.

Signature: `pub fn load(flag_user: Option<&str>, flag_file: Option<&Path>) -> Result<Settings, Error> {`

What it does: Function load takes flag_user: Option<&str>, flag_file: Option<&Path>. It gives back Result<Settings, Error>.

(Where these words came from: inferred.)

---

## function — resolve

Found in src/config.rs at line 305.

Surface: public API.

Signature: `pub fn resolve( flag_user: Option<&str>, flag_file: Option<&Path>, env_user: Option<&str>, env_file: Option<&str>, config_json: Option<&str>, ) -> Result<Settings, Error> {`

What it does: Function resolve takes flag_user: Option<&str>, flag_file: Option<&Path>, env_user: Option<&str>, env_file: Option<&str>, config_json: Option<&str>,. It gives back Result<Settings, Error>.

(Where these words came from: inferred.)

---

## function — load_listen

Found in src/config.rs at line 341.

Surface: public API.

Signature: `pub fn load_listen(flag_host: Option<&str>, flag_port: Option<u16>) -> Result<Listen, Error> {`

What it does: Function load_listen takes flag_host: Option<&str>, flag_port: Option<u16>. It gives back Result<Listen, Error>.

(Where these words came from: inferred.)

---

## function — resolve_listen

Found in src/config.rs at line 352.

Surface: public API.

Signature: `pub fn resolve_listen( flag_host: Option<&str>, flag_port: Option<u16>, env_host: Option<&str>, env_port: Option<&str>, ) -> Result<Listen, Error> {`

What it does: Function resolve_listen takes flag_host: Option<&str>, flag_port: Option<u16>, env_host: Option<&str>, env_port: Option<&str>,. It gives back Result<Listen, Error>.

(Where these words came from: inferred.)

---

## const — ROSTER_LIBRARY

Found in src/coord.rs at line 12.

Surface: public API.

Signature: `pub const ROSTER_LIBRARY: usize = 5;`

What it does: Library reserved for reader records. Chat topics use the other libraries.

(Where these words came from: direct.)

---

## const — SETTINGS_COORD

Found in src/coord.rs at line 16.

Surface: public API.

Signature: `pub const SETTINGS_COORD: &str = "5.1.1/1.1.2/1.1.1";`

What it does: The lane's settings scroll: owners and limit overrides. Book 2 of the roster library, so it never collides with a reader (book 1).

(Where these words came from: direct.)

---

## const — WEIGHTS

Found in src/coord.rs at line 21.

Surface: public API.

Signature: `pub const WEIGHTS: [u64; 6] = [16, 8, 4, 2, 2, 1];`

What it does: Library, shelf, series, collection, volume, book. Topic steps outweigh thread steps, so a different library cannot outrank a subscribed shelf just by being newer.

(Where these words came from: direct.)

---

## function — parse_coord

Found in src/coord.rs at line 23.

Surface: public API.

Signature: `pub fn parse_coord(input: &str) -> Result<Coordinate, String> {`

What it does: Function parse_coord takes input: &str. It gives back Result<Coordinate, String>.

(Where these words came from: inferred.)

---

## enum — Target

Found in src/coord.rs at line 35.

Surface: public API.

Signature: `pub enum Target {`

What it does: A named choice named Target. It is one of: Thread; Scroll.

(Where these words came from: inferred.)

---

## function — parse_target

Found in src/coord.rs at line 43.

Surface: public API.

Signature: `pub fn parse_target(input: &str) -> Result<Target, String> {`

What it does: Function parse_target takes input: &str. It gives back Result<Target, String>.

(Where these words came from: inferred.)

---

## function — format_coord

Found in src/coord.rs at line 56.

Surface: public API.

Signature: `pub fn format_coord(coord: Coordinate) -> String {`

What it does: Function format_coord takes coord: Coordinate. It gives back String.

(Where these words came from: inferred.)

---

## function — topic_key

Found in src/coord.rs at line 60.

Surface: public API.

Signature: `pub fn topic_key(coord: Coordinate) -> String {`

What it does: Function topic_key takes coord: Coordinate. It gives back String.

(Where these words came from: inferred.)

---

## function — thread_key

Found in src/coord.rs at line 64.

Surface: public API.

Signature: `pub fn thread_key(coord: Coordinate) -> String {`

What it does: Function thread_key takes coord: Coordinate. It gives back String.

(Where these words came from: inferred.)

---

## function — section_key

Found in src/coord.rs at line 69.

Surface: public API.

Signature: `pub fn section_key(coord: Coordinate) -> String {`

What it does: Chapter and section stay. Scrolls in one section share a lane.

(Where these words came from: direct.)

---

## function — section_anchor

Found in src/coord.rs at line 78.

Surface: public API.

Signature: `pub fn section_anchor(coord: Coordinate) -> Coordinate {`

What it does: Function section_anchor takes coord: Coordinate. It gives back Coordinate.

(Where these words came from: inferred.)

---

## function — thread_anchor

Found in src/coord.rs at line 84.

Surface: public API.

Signature: `pub fn thread_anchor(coord: Coordinate) -> Coordinate {`

What it does: Function thread_anchor takes coord: Coordinate. It gives back Coordinate.

(Where these words came from: inferred.)

---

## function — with_scroll

Found in src/coord.rs at line 92.

Surface: public API.

Signature: `pub fn with_scroll(thread: Coordinate, scroll: usize) -> Coordinate {`

What it does: Function with_scroll takes thread: Coordinate, scroll: usize. It gives back Coordinate.

(Where these words came from: inferred.)

---

## function — is_roster

Found in src/coord.rs at line 98.

Surface: public API.

Signature: `pub fn is_roster(coord: Coordinate) -> bool {`

What it does: Function is_roster takes coord: Coordinate. It gives back bool.

(Where these words came from: inferred.)

---

## function — is_card_slot

Found in src/coord.rs at line 102.

Surface: public API.

Signature: `pub fn is_card_slot(coord: Coordinate) -> bool {`

What it does: Function is_card_slot takes coord: Coordinate. It gives back bool.

(Where these words came from: inferred.)

---

## function — arms

Found in src/coord.rs at line 106.

Surface: public API.

Signature: `pub fn arms(coord: Coordinate) -> [usize; 6] {`

What it does: Function arms takes coord: Coordinate. It gives back [usize; 6].

(Where these words came from: inferred.)

---

## function — weighted_distance

Found in src/coord.rs at line 117.

Surface: public API.

Signature: `pub fn weighted_distance(a: Coordinate, b: Coordinate) -> u64 {`

What it does: Function weighted_distance takes a: Coordinate, b: Coordinate. It gives back u64.

(Where these words came from: inferred.)

---

## function — min_distance

Found in src/coord.rs at line 126.

Surface: public API.

Signature: `pub fn min_distance(thread: Coordinate, interests: &[Coordinate]) -> u64 {`

What it does: Function min_distance takes thread: Coordinate, interests: &[Coordinate]. It gives back u64.

(Where these words came from: inferred.)

---

## function — next_message_scroll

Found in src/coord.rs at line 136.

Surface: public API.

Signature: `pub fn next_message_scroll(used: impl IntoIterator<Item = usize>) -> usize {`

What it does: Smallest free message scroll in chapter 1, section 1. Scroll 1 is reserved for the thread card.

(Where these words came from: direct.)

---

## function — sanitize

Found in src/coord.rs at line 151.

Surface: public API.

Signature: `pub fn sanitize(text: &str) -> String {`

What it does: Strip dimension breaks so a message cannot split the phext. Newlines stay; they are inside the scroll.

(Where these words came from: direct.)

---

## function — one_line

Found in src/coord.rs at line 171.

Surface: public API.

Signature: `pub fn one_line(text: &str) -> String {`

What it does: Function one_line takes text: &str. It gives back String.

(Where these words came from: inferred.)

---

## enum — Error

Found in src/error.rs at line 4.

Surface: public API.

Signature: `pub enum Error {`

What it does: A named choice named Error. It is one of: Io; Bad.

(Where these words came from: inferred.)

---

## const — MAX_FABRIC_BYTES

Found in src/fabric.rs at line 20.

Surface: public API.

Signature: `pub const MAX_FABRIC_BYTES: usize = 5 * 1024 * 1024;`

What it does: A fixed value named MAX_FABRIC_BYTES, set to 5 * 1024 * 1024.

(Where these words came from: inferred.)

---

## const — MAX_FABRICS

Found in src/fabric.rs at line 21.

Surface: public API.

Signature: `pub const MAX_FABRICS: usize = 100;`

What it does: A fixed value named MAX_FABRICS, set to 100.

(Where these words came from: inferred.)

---

## enum — FabricError

Found in src/fabric.rs at line 26.

Surface: public API.

Signature: `pub enum FabricError {`

What it does: A named choice named FabricError. It is one of: Bad; Conflict; Limited; Forbidden; Missing; Io.

(Where these words came from: inferred.)

---

## enum — Route

Found in src/fabric.rs at line 51.

Surface: public API.

Signature: `pub enum Route {`

What it does: A named choice named Route. It is one of: Home; Lane; Post; Download; Health; State; ApiLane; ApiPost; Room; Suggest; Toc; Scroll; PostMessage; Reset; Limits; Access; Create; Upload; Session; AuthStart; AuthCallback; Logout; Join; JoinBack.

(Where these words came from: inferred.)

---

## struct — ParsedRoute

Found in src/fabric.rs at line 79.

Surface: public API.

Signature: `pub struct ParsedRoute {`

What it does: A data shape named ParsedRoute. Its fields are: pub fabric: Option<String>; pub route: Route.

(Where these words came from: inferred.)

---

## enum — RouteError

Found in src/fabric.rs at line 85.

Surface: public API.

Signature: `pub enum RouteError {`

What it does: A named choice named RouteError. It is one of: Unknown; BadFabric.

(Where these words came from: inferred.)

---

## function — parse_route

Found in src/fabric.rs at line 90.

Surface: public API.

Signature: `pub fn parse_route(path: &str) -> Result<ParsedRoute, RouteError> {`

What it does: Function parse_route takes path: &str. It gives back Result<ParsedRoute, RouteError>.

(Where these words came from: inferred.)

---

## function — normalize_name

Found in src/fabric.rs at line 187.

Surface: public API.

Signature: `pub fn normalize_name(raw: &str) -> Result<String, FabricError> {`

What it does: Function normalize_name takes raw: &str. It gives back Result<String, FabricError>.

(Where these words came from: inferred.)

---

## function — fabrics_dir

Found in src/fabric.rs at line 213.

Surface: public API.

Signature: `pub fn fabrics_dir(default_file: &Path) -> PathBuf {`

What it does: Function fabrics_dir takes default_file: &Path. It gives back PathBuf.

(Where these words came from: inferred.)

---

## function — download_filename

Found in src/fabric.rs at line 220.

Surface: public API.

Signature: `pub fn download_filename(fabric: Option<&str>, lane_title: Option<&str>, day: &str) -> String {`

What it does: Function download_filename takes fabric: Option<&str>, lane_title: Option<&str>, day: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — parse_upload

Found in src/fabric.rs at line 252.

Surface: public API.

Signature: `pub fn parse_upload(bytes: &[u8]) -> Result<String, FabricError> {`

What it does: Function parse_upload takes bytes: &[u8]. It gives back Result<String, FabricError>.

(Where these words came from: inferred.)

---

## function — roster_only

Found in src/fabric.rs at line 265.

Surface: public API.

Signature: `pub fn roster_only() -> String {`

What it does: Function roster_only takes no inputs beyond the value it is called on. It gives back String.

(Where these words came from: inferred.)

---

## function — lane_document

Found in src/fabric.rs at line 276.

Surface: public API.

Signature: `pub fn lane_document(doc: &str, thread: &Thread) -> String {`

What it does: Function lane_document takes doc: &str, thread: &Thread. It gives back String.

(Where these words came from: inferred.)

---

## struct — Hub

Found in src/fabric.rs at line 301.

Surface: public API.

Signature: `pub struct Hub {`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## function — Hub::open

Found in src/fabric.rs at line 310.

Surface: public API.

Signature: `pub fn open(default_path: impl Into<PathBuf>) -> Result<Self, Error> {`

What it does: Function Hub::open takes default_path: impl Into<PathBuf>. It gives back Result<Self, Error>.

(Where these words came from: inferred.)

---

## function — Hub::list

Found in src/fabric.rs at line 330.

Surface: public API.

Signature: `pub fn list(&self) -> Result<Vec<String>, FabricError> {`

What it does: Function Hub::list takes no inputs beyond the value it is called on. It gives back Result<Vec<String>, FabricError>.

(Where these words came from: inferred.)

---

## function — Hub::generation

Found in src/fabric.rs at line 356.

Surface: public API.

Signature: `pub fn generation(&self) -> u64 {`

What it does: Changes whenever the list of fabrics does.

(Where these words came from: direct.)

---

## function — Hub::with_store

Found in src/fabric.rs at line 360.

Surface: public API.

Signature: `pub fn with_store<T>( &self, fabric: Option<&str>, body: impl FnOnce(&mut Store) -> T, ) -> Result<T, FabricError> {`

What it does: Function Hub::with_store takes &self, fabric: Option<&str>, body: impl FnOnce. It gives back T, ) -> Result<T, FabricError>.

(Where these words came from: inferred.)

---

## function — Hub::create_empty

Found in src/fabric.rs at line 372.

Surface: public API.

Signature: `pub fn create_empty(&self, requested: Option<&str>, ip: &str) -> Result<String, FabricError> {`

What it does: Function Hub::create_empty takes &self, requested: Option<&str>, ip: &str. It gives back Result<String, FabricError>.

(Where these words came from: inferred.)

---

## function — Hub::create_upload

Found in src/fabric.rs at line 376.

Surface: public API.

Signature: `pub fn create_upload( &self, requested: Option<&str>, bytes: &[u8], ip: &str, ) -> Result<String, FabricError> {`

What it does: Function Hub::create_upload takes &self, requested: Option<&str>, bytes: &[u8], ip: &str,. It gives back Result<String, FabricError>.

(Where these words came from: inferred.)

---

## function — day_stamp

Found in src/fabric.rs at line 556.

Surface: public API.

Signature: `pub fn day_stamp(now: chrono::DateTime<chrono::Utc>) -> String {`

What it does: Function day_stamp takes now: chrono::DateTime<chrono::Utc>. It gives back String.

(Where these words came from: inferred.)

---

## const — JOIN_COOKIE

Found in src/invite.rs at line 18.

Surface: public API.

Signature: `pub const JOIN_COOKIE: &str = "lanes_join";`

What it does: A fixed value named JOIN_COOKIE, set to "lanes_join".

(Where these words came from: inferred.)

---

## struct — Owners

Found in src/invite.rs at line 27.

Surface: public API.

Signature: `pub struct Owners {`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## function — Owners::from_env

Found in src/invite.rs at line 32.

Surface: public API.

Signature: `pub fn from_env() -> Self {`

What it does: Function Owners::from_env takes no inputs beyond the value it is called on. It gives back Self.

(Where these words came from: inferred.)

---

## function — Owners::parse

Found in src/invite.rs at line 39.

Surface: public API.

Signature: `pub fn parse(raw: &str) -> Self {`

What it does: Function Owners::parse takes raw: &str. It gives back Self.

(Where these words came from: inferred.)

---

## function — Owners::invite_only

Found in src/invite.rs at line 60.

Surface: public API.

Signature: `pub fn invite_only(&self) -> bool {`

What it does: Function Owners::invite_only takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — Owners::is_empty

Found in src/invite.rs at line 64.

Surface: public API.

Signature: `pub fn is_empty(&self) -> bool {`

What it does: Function Owners::is_empty takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — Owners::len

Found in src/invite.rs at line 68.

Surface: public API.

Signature: `pub fn len(&self) -> usize {`

What it does: Function Owners::len takes no inputs beyond the value it is called on. It gives back usize.

(Where these words came from: inferred.)

---

## function — Owners::matches

Found in src/invite.rs at line 72.

Surface: public API.

Signature: `pub fn matches(&self, who: &str, email: &str) -> bool {`

What it does: Function Owners::matches takes &self, who: &str, email: &str. It gives back bool.

(Where these words came from: inferred.)

---

## function — beside

Found in src/invite.rs at line 96.

Surface: public API.

Signature: `pub fn beside(phext: &Path) -> PathBuf {`

What it does: Default invite file: next to the phext, not inside the checkout when the phext lives in `/var/lib/phext-lanes`.

(Where these words came from: direct.)

---

## function — resolve

Found in src/invite.rs at line 104.

Surface: public API.

Signature: `pub fn resolve(phext: &Path) -> PathBuf {`

What it does: `LANES_INVITES` wins. Otherwise [`beside`].

(Where these words came from: direct.)

---

## struct — Issued

Found in src/invite.rs at line 112.

Surface: public API.

Signature: `pub struct Issued {`

What it does: A data shape named Issued. Its fields are: pub id: String; pub token: String; pub uses: u32; pub exp: Option<DateTime<Utc>>.

(Where these words came from: inferred.)

---

## struct — Listed

Found in src/invite.rs at line 120.

Surface: public API.

Signature: `pub struct Listed {`

What it does: A data shape named Listed. Its fields are: pub id: String; pub uses: u32; pub remaining: u32; pub exp: Option<DateTime<Utc>>; pub revoked: bool; pub created: DateTime<Utc>.

(Where these words came from: inferred.)

---

## enum — RedeemFail

Found in src/invite.rs at line 130.

Surface: public API.

Signature: `pub enum RedeemFail {`

What it does: A named choice named RedeemFail. It is one of: Invalid; Expired; Revoked; Spent.

(Where these words came from: inferred.)

---

## function — RedeemFail::message

Found in src/invite.rs at line 138.

Surface: public API.

Signature: `pub fn message(self) -> &'static str {`

What it does: Function RedeemFail::message takes no inputs beyond the value it is called on. It gives back &'static str.

(Where these words came from: inferred.)

---

## function — create

Found in src/invite.rs at line 164.

Surface: public API.

Signature: `pub fn create( path: &Path, uses: u32, exp: Option<DateTime<Utc>>, now: DateTime<Utc>, ) -> Result<Issued, Error> {`

What it does: Function create takes path: &Path, uses: u32, exp: Option<DateTime<Utc>>, now: DateTime<Utc>,. It gives back Result<Issued, Error>.

(Where these words came from: inferred.)

---

## function — list

Found in src/invite.rs at line 197.

Surface: public API.

Signature: `pub fn list(path: &Path) -> Result<Vec<Listed>, Error> {`

What it does: Function list takes path: &Path. It gives back Result<Vec<Listed>, Error>.

(Where these words came from: inferred.)

---

## function — revoke

Found in src/invite.rs at line 214.

Surface: public API.

Signature: `pub fn revoke(path: &Path, id: &str) -> Result<(), Error> {`

What it does: Function revoke takes path: &Path, id: &str. It gives back Result<(), Error>.

(Where these words came from: inferred.)

---

## function — redeem

Found in src/invite.rs at line 228.

Surface: public API.

Signature: `pub fn redeem(path: &Path, token: &str, now: DateTime<Utc>) -> Result<(), RedeemFail> {`

What it does: Function redeem takes path: &Path, token: &str, now: DateTime<Utc>. It gives back Result<(), RedeemFail>.

(Where these words came from: inferred.)

---

## function — token_shape

Found in src/invite.rs at line 261.

Surface: public API.

Signature: `pub fn token_shape(token: &str) -> bool {`

What it does: Function token_shape takes token: &str. It gives back bool.

(Where these words came from: inferred.)

---

## re-export — error::Error

Found in src/lib.rs at line 27.

Surface: public API.

Signature: `pub use error::Error;`

What it does: A re-export: error::Error is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## enum — Bucket

Found in src/limit.rs at line 13.

Surface: public API.

Signature: `pub enum Bucket {`

What it does: A named choice named Bucket. It is one of: AuthIp; WriteIp; WriteAccount.

(Where these words came from: inferred.)

---

## struct — Limiter

Found in src/limit.rs at line 24.

Surface: public API.

Signature: `pub struct Limiter {`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## function — Limiter::standard

Found in src/limit.rs at line 32.

Surface: public API.

Signature: `pub fn standard() -> Self {`

What it does: Function Limiter::standard takes no inputs beyond the value it is called on. It gives back Self.

(Where these words came from: inferred.)

---

## function — Limiter::from_limits

Found in src/limit.rs at line 37.

Surface: public API.

Signature: `pub fn from_limits(limits: &crate::config::Limits) -> Self {`

What it does: The rate caps from the server's configured limits.

(Where these words came from: direct.)

---

## function — Limiter::with_caps

Found in src/limit.rs at line 45.

Surface: public API.

Signature: `pub fn with_caps(auth_per_ip: usize, writes_per_ip: usize, writes_per_account: usize) -> Self {`

What it does: Function Limiter::with_caps takes auth_per_ip: usize, writes_per_ip: usize, writes_per_account: usize. It gives back Self.

(Where these words came from: inferred.)

---

## function — Limiter::hit

Found in src/limit.rs at line 55.

Surface: public API.

Signature: `pub fn hit(&self, bucket: Bucket, id: &str) -> Result<(), ()> {`

What it does: Function Limiter::hit takes &self, bucket: Bucket, id: &str. It gives back Result<(), ()>.

(Where these words came from: inferred.)

---

## function — lane_id

Found in src/link.rs at line 23.

Surface: public API.

Signature: `pub fn lane_id(coord: Coordinate) -> String {`

What it does: Function lane_id takes coord: Coordinate. It gives back String.

(Where these words came from: inferred.)

---

## function — post_id

Found in src/link.rs at line 27.

Surface: public API.

Signature: `pub fn post_id(coord: Coordinate) -> String {`

What it does: Function post_id takes coord: Coordinate. It gives back String.

(Where these words came from: inferred.)

---

## function — section_lane_id

Found in src/link.rs at line 32.

Surface: public API.

Signature: `pub fn section_lane_id(coord: Coordinate) -> String {`

What it does: Address of one chapter and section: the coordinate of its first scroll.

(Where these words came from: direct.)

---

## function — parse_lane_id

Found in src/link.rs at line 37.

Surface: public API.

Signature: `pub fn parse_lane_id(id: &str) -> Option<Coordinate> {`

What it does: A chat lane: the thread's card, `…/1.1.1`, or the six-part thread.

(Where these words came from: direct.)

---

## function — parse_section_lane_id

Found in src/link.rs at line 51.

Surface: public API.

Signature: `pub fn parse_section_lane_id(id: &str) -> Option<Coordinate> {`

What it does: Function parse_section_lane_id takes id: &str. It gives back Option<Coordinate>.

(Where these words came from: inferred.)

---

## function — parse_post_id

Found in src/link.rs at line 66.

Surface: public API.

Signature: `pub fn parse_post_id(id: &str) -> Option<Coordinate> {`

What it does: Function parse_post_id takes id: &str. It gives back Option<Coordinate>.

(Where these words came from: inferred.)

---

## function — canonical_lane

Found in src/link.rs at line 78.

Surface: public API.

Signature: `pub fn canonical_lane(id: &str) -> Option<String> {`

What it does: The canonical nine-part address for a lane id in any accepted form.

(Where these words came from: direct.)

---

## function — canonical_post

Found in src/link.rs at line 86.

Surface: public API.

Signature: `pub fn canonical_post(id: &str) -> Option<String> {`

What it does: The canonical address for a post id in any accepted form.

(Where these words came from: direct.)

---

## struct — User

Found in src/model.rs at line 35.

Surface: public API.

Signature: `pub struct User {`

What it does: A data shape named User. Its fields are: pub id: String; pub name: String; pub blurb: String; pub interests: Vec<Coordinate>.

(Where these words came from: inferred.)

---

## const — PLAIN_AUTHOR

Found in src/model.rs at line 43.

Surface: public API.

Signature: `pub const PLAIN_AUTHOR: &str = "phext";`

What it does: Author stamped on a scroll that is not a Lanes message.

(Where these words came from: direct.)

---

## struct — Message

Found in src/model.rs at line 46.

Surface: public API.

Signature: `pub struct Message {`

What it does: A data shape named Message. Its fields are: pub coord: Coordinate; pub author: String; pub who: String; pub ts: DateTime<Utc>; pub text: String; pub plain: bool.

(Where these words came from: inferred.)

---

## function — Message::poster_key

Found in src/model.rs at line 59.

Surface: public API.

Signature: `pub fn poster_key(&self) -> &str {`

What it does: Share caps follow the OAuth identity when the scroll has one.

(Where these words came from: direct.)

---

## struct — Thread

Found in src/model.rs at line 69.

Surface: public API.

Signature: `pub struct Thread {`

What it does: A data shape named Thread. Its fields are: pub coord: Coordinate; pub topic: String; pub title: String; pub keywords: String; pub messages: Vec<Message>; pub plain: bool.

(Where these words came from: inferred.)

---

## enum — Scroll

Found in src/model.rs at line 80.

Surface: public API.

Signature: `pub enum Scroll {`

What it does: A named choice named Scroll. It is one of: User; Settings; Card.

(Where these words came from: inferred.)

---

## struct — LaneSettings

Found in src/model.rs at line 99.

Surface: public API.

Signature: `pub struct LaneSettings {`

What it does: Who owns a lane and how its feed is limited. One scroll at [`crate::coord::SETTINGS_COORD`].

(Where these words came from: direct.)

---

## function — LaneSettings::to_scroll

Found in src/model.rs at line 106.

Surface: public API.

Signature: `pub fn to_scroll(&self) -> String {`

What it does: Function LaneSettings::to_scroll takes no inputs beyond the value it is called on. It gives back String.

(Where these words came from: inferred.)

---

## function — LaneSettings::is_owner

Found in src/model.rs at line 115.

Surface: public API.

Signature: `pub fn is_owner(&self, who: &str) -> bool {`

What it does: Function LaneSettings::is_owner takes &self, who: &str. It gives back bool.

(Where these words came from: inferred.)

---

## function — User::to_scroll

Found in src/model.rs at line 121.

Surface: public API.

Signature: `pub fn to_scroll(&self) -> String {`

What it does: Function User::to_scroll takes no inputs beyond the value it is called on. It gives back String.

(Where these words came from: inferred.)

---

## function — member_id

Found in src/model.rs at line 138.

Surface: public API.

Signature: `pub fn member_id(id: &str) -> String {`

What it does: Roster ids keep a single colon (`github:100`). Other punctuation is dropped.

(Where these words came from: direct.)

---

## function — format_card

Found in src/model.rs at line 151.

Surface: public API.

Signature: `pub fn format_card(topic: &str, title: &str, keywords: &str) -> String {`

What it does: Function format_card takes topic: &str, title: &str, keywords: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — format_message

Found in src/model.rs at line 160.

Surface: public API.

Signature: `pub fn format_message(author: &str, ts: DateTime<Utc>, text: &str) -> String {`

What it does: Function format_message takes author: &str, ts: DateTime<Utc>, text: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — format_message_as

Found in src/model.rs at line 164.

Surface: public API.

Signature: `pub fn format_message_as(author: &str, who: &str, ts: DateTime<Utc>, text: &str) -> String {`

What it does: Function format_message_as takes author: &str, who: &str, ts: DateTime<Utc>, text: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — opening_line

Found in src/model.rs at line 180.

Surface: public API.

Signature: `pub fn opening_line(text: &str, limit: usize) -> String {`

What it does: First non-empty line, clipped. Empty text becomes "Scroll".

(Where these words came from: direct.)

---

## function — rest_after_opening

Found in src/model.rs at line 195.

Surface: public API.

Signature: `pub fn rest_after_opening(text: &str) -> String {`

What it does: The scroll after its first non-empty line.

(Where these words came from: direct.)

---

## function — title_from_text

Found in src/model.rs at line 217.

Surface: public API.

Signature: `pub fn title_from_text(text: &str) -> String {`

What it does: Function title_from_text takes text: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — keyword_line

Found in src/model.rs at line 234.

Surface: public API.

Signature: `pub fn keyword_line(text: &str) -> String {`

What it does: Function keyword_line takes text: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — parse_scroll

Found in src/model.rs at line 241.

Surface: public API.

Signature: `pub fn parse_scroll(raw: &str) -> Option<Scroll> {`

What it does: Function parse_scroll takes raw: &str. It gives back Option<Scroll>.

(Where these words came from: inferred.)

---

## function — content_tokens

Found in src/model.rs at line 356.

Surface: public API.

Signature: `pub fn content_tokens(text: &str) -> std::collections::BTreeSet<String> {`

What it does: Lowercased content words. A trailing "s" is stripped so "ferns" meets "fern".

(Where these words came from: direct.)

---

## function — preview

Found in src/model.rs at line 377.

Surface: public API.

Signature: `pub fn preview(text: &str, limit: usize) -> String {`

What it does: Function preview takes text: &str, limit: usize. It gives back String.

(Where these words came from: inferred.)

---

## const — TEST_CSRF

Found in src/oauth.rs at line 23.

Surface: public API.

Signature: `pub const TEST_CSRF: &str = "csrf-test-token";`

What it does: A fixed value named TEST_CSRF, set to "csrf-test-token".

(Where these words came from: inferred.)

---

## struct — Identity

Found in src/oauth.rs at line 28.

Surface: public API.

Signature: `pub struct Identity {`

What it does: A data shape named Identity. Its fields are: pub who: String; pub handle: String; pub name: String; pub email: String.

(Where these words came from: inferred.)

---

## trait — IdentitySource

Found in src/oauth.rs at line 35.

Surface: public API.

Signature: `pub trait IdentitySource: Send + Sync {`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## struct — Oauth

Found in src/oauth.rs at line 53.

Surface: public API.

Signature: `pub struct Oauth {`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## struct — Session

Found in src/oauth.rs at line 69.

Surface: public API.

Signature: `pub struct Session {`

What it does: A data shape named Session. Its fields are: pub who: String; pub handle: String; pub name: String; pub email: String; pub csrf: String.

(Where these words came from: inferred.)

---

## struct — Pending

Found in src/oauth.rs at line 98.

Surface: public API.

Signature: `pub struct Pending {`

What it does: A data shape named Pending. Its fields are: pub provider: String; pub state: String; pub verifier: String; pub next: String.

(Where these words came from: inferred.)

---

## function — Oauth::from_env

Found in src/oauth.rs at line 163.

Surface: public API.

Signature: `pub fn from_env() -> Self {`

What it does: Function Oauth::from_env takes no inputs beyond the value it is called on. It gives back Self.

(Where these words came from: inferred.)

---

## function — Oauth::test_ready

Found in src/oauth.rs at line 179.

Surface: public API.

Signature: `pub fn test_ready() -> Self {`

What it does: Function Oauth::test_ready takes no inputs beyond the value it is called on. It gives back Self.

(Where these words came from: inferred.)

---

## function — Oauth::closed

Found in src/oauth.rs at line 192.

Surface: public API.

Signature: `pub fn closed() -> Self {`

What it does: Function Oauth::closed takes no inputs beyond the value it is called on. It gives back Self.

(Where these words came from: inferred.)

---

## function — Oauth::ready

Found in src/oauth.rs at line 223.

Surface: public API.

Signature: `pub fn ready(&self) -> bool {`

What it does: Function Oauth::ready takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — Oauth::providers

Found in src/oauth.rs at line 229.

Surface: public API.

Signature: `pub fn providers(&self) -> Vec<&'static str> {`

What it does: Function Oauth::providers takes no inputs beyond the value it is called on. It gives back Vec<&'static str>.

(Where these words came from: inferred.)

---

## function — Oauth::cookie_secure

Found in src/oauth.rs at line 243.

Surface: public API.

Signature: `pub fn cookie_secure(&self) -> bool {`

What it does: Function Oauth::cookie_secure takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — Oauth::authorize_url

Found in src/oauth.rs at line 247.

Surface: public API.

Signature: `pub fn authorize_url( &self, provider_name: &str, next: &str, ) -> Result<(String, String), String> {`

What it does: Function Oauth::authorize_url takes &self, provider_name: &str, next: &str,. It gives back Result<(String, String), String>.

(Where these words came from: inferred.)

---

## function — Oauth::finish

Found in src/oauth.rs at line 291.

Surface: public API.

Signature: `pub fn finish( &self, provider_name: &str, code: &str, state: &str, pending_cookie: &str, ) -> Result<(Identity, String, String), String> {`

What it does: Function Oauth::finish takes &self, provider_name: &str, code: &str, state: &str, pending_cookie: &str,. It gives back Result<(Identity, String, String), String>.

(Where these words came from: inferred.)

---

## function — Oauth::read_session

Found in src/oauth.rs at line 333.

Surface: public API.

Signature: `pub fn read_session(&self, cookie_header: &str) -> Option<Session> {`

What it does: Function Oauth::read_session takes &self, cookie_header: &str. It gives back Option<Session>.

(Where these words came from: inferred.)

---

## function — Oauth::csrf_matches

Found in src/oauth.rs at line 355.

Surface: public API.

Signature: `pub fn csrf_matches(&self, session: &Session, header: &str) -> bool {`

What it does: Function Oauth::csrf_matches takes &self, session: &Session, header: &str. It gives back bool.

(Where these words came from: inferred.)

---

## function — Oauth::test_session_cookie

Found in src/oauth.rs at line 359.

Surface: public API.

Signature: `pub fn test_session_cookie() -> String {`

What it does: Function Oauth::test_session_cookie takes no inputs beyond the value it is called on. It gives back String.

(Where these words came from: inferred.)

---

## function — session_cookie_name

Found in src/oauth.rs at line 418.

Surface: public API.

Signature: `pub fn session_cookie_name() -> &'static str {`

What it does: Function session_cookie_name takes no inputs beyond the value it is called on. It gives back &'static str.

(Where these words came from: inferred.)

---

## function — pending_cookie_name

Found in src/oauth.rs at line 422.

Surface: public API.

Signature: `pub fn pending_cookie_name() -> &'static str {`

What it does: Function pending_cookie_name takes no inputs beyond the value it is called on. It gives back &'static str.

(Where these words came from: inferred.)

---

## function — cookie_header

Found in src/oauth.rs at line 426.

Surface: public API.

Signature: `pub fn cookie_header(name: &str, value: &str, max_age: i64, secure: bool) -> String {`

What it does: Function cookie_header takes name: &str, value: &str, max_age: i64, secure: bool. It gives back String.

(Where these words came from: inferred.)

---

## function — session_cookie

Found in src/oauth.rs at line 431.

Surface: public API.

Signature: `pub fn session_cookie(value: &str, secure: bool) -> String {`

What it does: Function session_cookie takes value: &str, secure: bool. It gives back String.

(Where these words came from: inferred.)

---

## function — pending_cookie

Found in src/oauth.rs at line 435.

Surface: public API.

Signature: `pub fn pending_cookie(value: &str, secure: bool) -> String {`

What it does: Function pending_cookie takes value: &str, secure: bool. It gives back String.

(Where these words came from: inferred.)

---

## function — clear_cookie

Found in src/oauth.rs at line 439.

Surface: public API.

Signature: `pub fn clear_cookie(name: &str, secure: bool) -> String {`

What it does: Function clear_cookie takes name: &str, secure: bool. It gives back String.

(Where these words came from: inferred.)

---

## function — safe_next

Found in src/oauth.rs at line 484.

Surface: public API.

Signature: `pub fn safe_next(next: &str) -> String {`

What it does: Function safe_next takes next: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — display_name

Found in src/oauth.rs at line 516.

Surface: public API.

Signature: `pub fn display_name(raw: &str, fallback: &str) -> String {`

What it does: Function display_name takes raw: &str, fallback: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — clean_email

Found in src/oauth.rs at line 532.

Surface: public API.

Signature: `pub fn clean_email(raw: &str) -> String {`

What it does: Function clean_email takes raw: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — clean_handle

Found in src/oauth.rs at line 553.

Surface: public API.

Signature: `pub fn clean_handle(raw: &str) -> String {`

What it does: Function clean_handle takes raw: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — pending_token

Found in src/oauth.rs at line 618.

Surface: public API.

Signature: `pub fn pending_token(cookie_header: &str) -> Option<String> {`

What it does: Function pending_token takes cookie_header: &str. It gives back Option<String>.

(Where these words came from: inferred.)

---

## function — cookie_from_headers

Found in src/oauth.rs at line 622.

Surface: public API.

Signature: `pub fn cookie_from_headers(headers: &axum::http::HeaderMap) -> String {`

What it does: Function cookie_from_headers takes headers: &axum::http::HeaderMap. It gives back String.

(Where these words came from: inferred.)

---

## re-export — crate::config::Limits

Found in src/rank.rs at line 17.

Surface: public API.

Signature: `pub use crate::config::Limits;`

What it does: A re-export: crate::config::Limits is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## struct — Feed

Found in src/rank.rs at line 22.

Surface: public API.

Signature: `pub struct Feed {`

What it does: A data shape named Feed. Its fields are: pub threads: Vec<FeedThread>; pub lanes: Vec<Lane>.

(Where these words came from: inferred.)

---

## struct — FeedThread

Found in src/rank.rs at line 28.

Surface: public API.

Signature: `pub struct FeedThread {`

What it does: A data shape named FeedThread. Its fields are: pub coord: Coordinate; pub topic: String; pub title: String; pub distance: u64; pub score: f64; pub nearness: &'static str; pub admitted: Vec<Message>; pub held: Vec<Message>.

(Where these words came from: inferred.)

---

## struct — Lane

Found in src/rank.rs at line 40.

Surface: public API.

Signature: `pub struct Lane {`

What it does: A data shape named Lane. Its fields are: pub author: String; pub held: usize; pub shown: usize; pub threads: Vec<String>.

(Where these words came from: inferred.)

---

## struct — Suggestion

Found in src/rank.rs at line 48.

Surface: public API.

Signature: `pub struct Suggestion {`

What it does: A data shape named Suggestion. Its fields are: pub coord: Coordinate; pub thread: Coordinate; pub title: String; pub topic: String; pub new_thread: bool; pub overlap: Vec<String>; pub reason: String.

(Where these words came from: inferred.)

---

## struct — MapEntry

Found in src/rank.rs at line 59.

Surface: public API.

Signature: `pub struct MapEntry {`

What it does: A data shape named MapEntry. Its fields are: pub coord: Coordinate; pub topic: String; pub title: String; pub distance: u64; pub message_count: usize; pub latest_author: Option<String>; pub latest_ts: Option<DateTime<Utc>>; pub preview: String; pub majority_author: Option<String>; pub majority_count: usize; pub majority_share: f64; pub flooded: bool.

(Where these words came from: inferred.)

---

## function — thread_score

Found in src/rank.rs at line 74.

Surface: public API.

Signature: `pub fn thread_score(distance: u64, age_secs: u64) -> f64 {`

What it does: Function thread_score takes distance: u64, age_secs: u64. It gives back f64.

(Where these words came from: inferred.)

---

## function — nearness

Found in src/rank.rs at line 81.

Surface: public API.

Signature: `pub fn nearness(distance: u64) -> &'static str {`

What it does: Function nearness takes distance: u64. It gives back &'static str.

(Where these words came from: inferred.)

---

## function — age_secs

Found in src/rank.rs at line 90.

Surface: public API.

Signature: `pub fn age_secs(ts: DateTime<Utc>, now: DateTime<Utc>) -> u64 {`

What it does: Function age_secs takes ts: DateTime<Utc>, now: DateTime<Utc>. It gives back u64.

(Where these words came from: inferred.)

---

## function — build_feed

Found in src/rank.rs at line 107.

Surface: public API.

Signature: `pub fn build_feed( threads: &[Thread], interests: &[Coordinate], now: DateTime<Utc>, limits: &Limits, ) -> Feed {`

What it does: Function build_feed takes threads: &[Thread], interests: &[Coordinate], now: DateTime<Utc>, limits: &Limits,. It gives back Feed.

(Where these words came from: inferred.)

---

## function — build_feed_for

Found in src/rank.rs at line 119.

Surface: public API.

Signature: `pub fn build_feed_for( threads: &[Thread], interests: &[Coordinate], now: DateTime<Utc>, limits: &Limits, reader: Option<&str>, ) -> Feed {`

What it does: [`build_feed`] for one reader. The reader's own posts are never held: the cap keeps other voices from flooding a feed, and nobody needs protecting from themselves. `reader` is a [`Message::poster_key`].

(Where these words came from: direct.)

---

## function — map_of

Found in src/rank.rs at line 239.

Surface: public API.

Signature: `pub fn map_of(threads: &[Thread], interests: &[Coordinate], limits: &Limits) -> Vec<MapEntry> {`

What it does: Function map_of takes threads: &[Thread], interests: &[Coordinate], limits: &Limits. It gives back Vec<MapEntry>.

(Where these words came from: inferred.)

---

## function — majority

Found in src/rank.rs at line 267.

Surface: public API.

Signature: `pub fn majority(messages: &[Message]) -> (Option<String>, usize, f64) {`

What it does: Function majority takes messages: &[Message]. It gives back (Option<String>, usize, f64).

(Where these words came from: inferred.)

---

## function — suggest_fresh

Found in src/rank.rs at line 285.

Surface: public API.

Signature: `pub fn suggest_fresh( threads: &[Thread], interests: &[Coordinate], text: &str, _now: DateTime<Utc>, ) -> Suggestion {`

What it does: A new book on the reader's home shelf. Same placement as a suggestion that shared no words. Used when someone explicitly starts a thread.

(Where these words came from: direct.)

---

## function — suggest

Found in src/rank.rs at line 306.

Surface: public API.

Signature: `pub fn suggest( threads: &[Thread], interests: &[Coordinate], text: &str, now: DateTime<Utc>, ) -> Suggestion {`

What it does: Function suggest takes threads: &[Thread], interests: &[Coordinate], text: &str, now: DateTime<Utc>,. It gives back Suggestion.

(Where these words came from: inferred.)

---

## function — fresh_thread

Found in src/rank.rs at line 450.

Surface: public API.

Signature: `pub fn fresh_thread(threads: &[Thread], home: Coordinate) -> Coordinate {`

What it does: Function fresh_thread takes threads: &[Thread], home: Coordinate. It gives back Coordinate.

(Where these words came from: inferred.)

---

## function — admitted_by

Found in src/rank.rs at line 505.

Surface: public API.

Signature: `pub fn admitted_by(feed: &Feed, author: &str) -> usize {`

What it does: Function admitted_by takes feed: &Feed, author: &str. It gives back usize.

(Where these words came from: inferred.)

---

## function — tokenize

Found in src/scrolls.rs at line 20.

Surface: public API.

Signature: `pub fn tokenize(doc: &str) -> Vec<(Coordinate, Range<usize>)> {`

What it does: Every non-empty scroll and where its text sits in `doc`, in document order. Coordinates only grow, so the output is sorted.

(Where these words came from: direct.)

---

## function — serialize

Found in src/scrolls.rs at line 45.

Surface: public API.

Signature: `pub fn serialize<'a>(scrolls: impl IntoIterator<Item = (Coordinate, &'a str)>) -> String {`

What it does: A phext document from scrolls in ascending coordinate order. Empty scrolls are skipped, as `phext::implode` skips them.

(Where these words came from: direct.)

---

## function — textmap

Found in src/scrolls.rs at line 59.

Surface: public API.

Signature: `pub fn textmap(doc: &str, scrolls: &[(Coordinate, Range<usize>)]) -> String {`

What it does: One line per scroll, in the format of `phext::textmap`.

(Where these words came from: direct.)

---

## const — MIRA

Found in src/seed.rs at line 12.

Surface: public API.

Signature: `pub const MIRA: &str = "Mira Quill";`

What it does: A fixed value named MIRA, set to "Mira Quill".

(Where these words came from: inferred.)

---

## const — JULES

Found in src/seed.rs at line 13.

Surface: public API.

Signature: `pub const JULES: &str = "Jules Fen";`

What it does: A fixed value named JULES, set to "Jules Fen".

(Where these words came from: inferred.)

---

## const — ADA

Found in src/seed.rs at line 14.

Surface: public API.

Signature: `pub const ADA: &str = "Ada North";`

What it does: A fixed value named ADA, set to "Ada North".

(Where these words came from: inferred.)

---

## const — REN

Found in src/seed.rs at line 15.

Surface: public API.

Signature: `pub const REN: &str = "Ren Sato";`

What it does: A fixed value named REN, set to "Ren Sato".

(Where these words came from: inferred.)

---

## const — CASEY

Found in src/seed.rs at line 16.

Surface: public API.

Signature: `pub const CASEY: &str = "Casey Horn";`

What it does: A fixed value named CASEY, set to "Casey Horn".

(Where these words came from: inferred.)

---

## const — HALL

Found in src/seed.rs at line 18.

Surface: public API.

Signature: `pub const HALL: &str = "General";`

What it does: A fixed value named HALL, set to "General".

(Where these words came from: inferred.)

---

## const — FERN

Found in src/seed.rs at line 19.

Surface: public API.

Signature: `pub const FERN: &str = "Fern wall";`

What it does: A fixed value named FERN, set to "Fern wall".

(Where these words came from: inferred.)

---

## const — SWAP

Found in src/seed.rs at line 20.

Surface: public API.

Signature: `pub const SWAP: &str = "Seed swap";`

What it does: A fixed value named SWAP, set to "Seed swap".

(Where these words came from: inferred.)

---

## const — PLANES

Found in src/seed.rs at line 21.

Surface: public API.

Signature: `pub const PLANES: &str = "Hand planes";`

What it does: A fixed value named PLANES, set to "Hand planes".

(Where these words came from: inferred.)

---

## const — RIVER

Found in src/seed.rs at line 22.

Surface: public API.

Signature: `pub const RIVER: &str = "River wards";`

What it does: A fixed value named RIVER, set to "River wards".

(Where these words came from: inferred.)

---

## const — BREAD

Found in src/seed.rs at line 23.

Surface: public API.

Signature: `pub const BREAD: &str = "Bread hour";`

What it does: A fixed value named BREAD, set to "Bread hour".

(Where these words came from: inferred.)

---

## const — HALL_THREAD

Found in src/seed.rs at line 25.

Surface: public API.

Signature: `pub const HALL_THREAD: &str = "1.1.1/1.1.1";`

What it does: A fixed value named HALL_THREAD, set to "1.1.1/1.1.1".

(Where these words came from: inferred.)

---

## const — FERN_THREAD

Found in src/seed.rs at line 26.

Surface: public API.

Signature: `pub const FERN_THREAD: &str = "1.2.1/1.1.1";`

What it does: A fixed value named FERN_THREAD, set to "1.2.1/1.1.1".

(Where these words came from: inferred.)

---

## const — SWAP_THREAD

Found in src/seed.rs at line 27.

Surface: public API.

Signature: `pub const SWAP_THREAD: &str = "1.2.1/1.2.1";`

What it does: A fixed value named SWAP_THREAD, set to "1.2.1/1.2.1".

(Where these words came from: inferred.)

---

## const — PLANES_THREAD

Found in src/seed.rs at line 28.

Surface: public API.

Signature: `pub const PLANES_THREAD: &str = "2.1.1/1.1.1";`

What it does: A fixed value named PLANES_THREAD, set to "2.1.1/1.1.1".

(Where these words came from: inferred.)

---

## const — RIVER_THREAD

Found in src/seed.rs at line 29.

Surface: public API.

Signature: `pub const RIVER_THREAD: &str = "3.1.1/1.1.1";`

What it does: A fixed value named RIVER_THREAD, set to "3.1.1/1.1.1".

(Where these words came from: inferred.)

---

## const — BREAD_THREAD

Found in src/seed.rs at line 30.

Surface: public API.

Signature: `pub const BREAD_THREAD: &str = "4.1.1/1.1.1";`

What it does: A fixed value named BREAD_THREAD, set to "4.1.1/1.1.1".

(Where these words came from: inferred.)

---

## function — demo_instant

Found in src/seed.rs at line 174.

Surface: public API.

Signature: `pub fn demo_instant() -> DateTime<Utc> {`

What it does: Function demo_instant takes no inputs beyond the value it is called on. It gives back DateTime<Utc>.

(Where these words came from: inferred.)

---

## function — readers

Found in src/seed.rs at line 178.

Surface: public API.

Signature: `pub fn readers() -> Vec<User> {`

What it does: Function readers takes no inputs beyond the value it is called on. It gives back Vec<User>.

(Where these words came from: inferred.)

---

## const — PRIMER

Found in src/seed.rs at line 194.

Surface: public API.

Signature: `pub const PRIMER: &str = "How phext works";`

What it does: A fixed value named PRIMER, set to "How phext works".

(Where these words came from: inferred.)

---

## const — PRIMER_THREAD

Found in src/seed.rs at line 195.

Surface: public API.

Signature: `pub const PRIMER_THREAD: &str = "1.1.1/1.1.2";`

What it does: A fixed value named PRIMER_THREAD, set to "1.1.1/1.1.2".

(Where these words came from: inferred.)

---

## const — PRIMER_AUTHOR

Found in src/seed.rs at line 196.

Surface: public API.

Signature: `pub const PRIMER_AUTHOR: &str = "Lanes";`

What it does: A fixed value named PRIMER_AUTHOR, set to "Lanes".

(Where these words came from: inferred.)

---

## function — blank

Found in src/seed.rs at line 211.

Surface: public API.

Signature: `pub fn blank(now: DateTime<Utc>) -> String {`

What it does: A new chat: a General thread and the phext primer. No readers. This does not include the fictional cast. [`document`] does.

(Where these words came from: direct.)

---

## function — document

Found in src/seed.rs at line 241.

Surface: public API.

Signature: `pub fn document(now: DateTime<Utc>) -> String {`

What it does: Build the demo phext by inserting each scroll in coordinate order.

(Where these words came from: direct.)

---

## struct — AppState

Found in src/server.rs at line 36.

Surface: public API.

Signature: `pub struct AppState {`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## function — AppState::open

Found in src/server.rs at line 50.

Surface: public API.

Signature: `pub fn open(path: std::path::PathBuf, limits: Limits) -> Result<Self, Error> {`

What it does: Function AppState::open takes path: std::path::PathBuf, limits: Limits. It gives back Result<Self, Error>.

(Where these words came from: inferred.)

---

## function — AppState::with_auth

Found in src/server.rs at line 65.

Surface: public API.

Signature: `pub fn with_auth( path: std::path::PathBuf, auth: Arc<dyn PlotHolderCheck>, ) -> Result<Self, Error> {`

What it does: Function AppState::with_auth takes path: std::path::PathBuf, auth: Arc<dyn PlotHolderCheck>,. It gives back Result<Self, Error>.

(Where these words came from: inferred.)

---

## function — listen_addr

Found in src/server.rs at line 172.

Surface: public API.

Signature: `pub fn listen_addr(host: &str, port: u16) -> String {`

What it does: `host:port`, with brackets when `host` is an IPv6 address.

(Where these words came from: direct.)

---

## function — serve

Found in src/server.rs at line 180.

Surface: public API.

Signature: `pub async fn serve( path: std::path::PathBuf, host: &str, port: u16, limits: Limits, ) -> Result<(), Error> {`

What it does: Function serve takes path: std::path::PathBuf, host: &str, port: u16, limits: Limits,. It gives back Result<(), Error>.

(Where these words came from: inferred.)

---

## function — router

Found in src/server.rs at line 224.

Surface: public API.

Signature: `pub fn router(state: AppState) -> Router {`

What it does: Function router takes state: AppState. It gives back Router.

(Where these words came from: inferred.)

---

## struct — World

Found in src/store.rs at line 30.

Surface: public API.

Signature: `pub struct World {`

What it does: A data shape named World. Its fields are: pub users: Vec<User>; pub threads: Vec<Thread>; pub settings: LaneSettings.

(Where these words came from: inferred.)

---

## function — World::find_user

Found in src/store.rs at line 38.

Surface: public API.

Signature: `pub fn find_user(&self, query: &str) -> Option<&User> {`

What it does: Function World::find_user takes &self, query: &str. It gives back Option<&User>.

(Where these words came from: inferred.)

---

## function — World::thread

Found in src/store.rs at line 54.

Surface: public API.

Signature: `pub fn thread(&self, key: &str) -> Option<&Thread> {`

What it does: Function World::thread takes &self, key: &str. It gives back Option<&Thread>.

(Where these words came from: inferred.)

---

## function — World::find_lane

Found in src/store.rs at line 65.

Surface: public API.

Signature: `pub fn find_lane(&self, id: &str) -> Option<&Thread> {`

What it does: Function World::find_lane takes &self, id: &str. It gives back Option<&Thread>.

(Where these words came from: inferred.)

---

## function — World::find_post

Found in src/store.rs at line 91.

Surface: public API.

Signature: `pub fn find_post(&self, id: &str) -> Option<(&Thread, &Message)> {`

What it does: The message scroll named by a post id. The thread card is not a post.

(Where these words came from: direct.)

---

## function — World::find_thread

Found in src/store.rs at line 103.

Surface: public API.

Signature: `pub fn find_thread(&self, query: &str) -> Result<&Thread, String> {`

What it does: A thread title, or a thread or scroll coordinate.

(Where these words came from: direct.)

---

## struct — Post

Found in src/store.rs at line 147.

Surface: public API.

Signature: `pub struct Post {`

What it does: A data shape named Post. Its fields are: pub author: String; pub who: String; pub text: String; pub target: Target; pub now: DateTime<Utc>.

(Where these words came from: inferred.)

---

## struct — Placed

Found in src/store.rs at line 156.

Surface: public API.

Signature: `pub struct Placed {`

What it does: A data shape named Placed. Its fields are: pub coord: Coordinate; pub thread: Coordinate; pub title: String; pub topic: String; pub new_thread: bool.

(Where these words came from: inferred.)

---

## struct — Store

Found in src/store.rs at line 199.

Surface: public API.

Signature: `pub struct Store {`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## function — Store::open

Found in src/store.rs at line 215.

Surface: public API.

Signature: `pub fn open(path: impl Into<PathBuf>) -> Result<Self, Error> {`

What it does: Function Store::open takes path: impl Into<PathBuf>. It gives back Result<Self, Error>.

(Where these words came from: inferred.)

---

## function — Store::path

Found in src/store.rs at line 230.

Surface: public API.

Signature: `pub fn path(&self) -> &Path {`

What it does: Function Store::path takes no inputs beyond the value it is called on. It gives back &Path.

(Where these words came from: inferred.)

---

## function — Store::doc

Found in src/store.rs at line 235.

Surface: public API.

Signature: `pub fn doc(&self) -> &str {`

What it does: The whole phext, straight from the mapping.

(Where these words came from: direct.)

---

## function — Store::version

Found in src/store.rs at line 240.

Surface: public API.

Signature: `pub fn version(&self) -> u64 {`

What it does: Bumped on every save. Equal versions mean equal contents.

(Where these words came from: direct.)

---

## function — Store::load_doc

Found in src/store.rs at line 245.

Surface: public API.

Signature: `pub fn load_doc(path: impl Into<PathBuf>) -> Result<Self, Error> {`

What it does: Load a fabric that already exists. Do not seed over it.

(Where these words came from: direct.)

---

## function — Store::from_doc

Found in src/store.rs at line 257.

Surface: public API.

Signature: `pub fn from_doc(path: impl Into<PathBuf>, doc: String) -> Self {`

What it does: Function Store::from_doc takes path: impl Into<PathBuf>, doc: String. It gives back Self.

(Where these words came from: inferred.)

---

## function — Store::text

Found in src/store.rs at line 300.

Surface: public API.

Signature: `pub fn text(&self, coord: Coordinate) -> &str {`

What it does: One scroll's text, borrowed from the mapping. Empty when nothing is there.

(Where these words came from: direct.)

---

## function — Store::world

Found in src/store.rs at line 308.

Surface: public API.

Signature: `pub fn world(&self) -> &World {`

What it does: The parsed fabric. Rebuilt only when the file changes.

(Where these words came from: direct.)

---

## function — Store::select

Found in src/store.rs at line 313.

Surface: public API.

Signature: `pub fn select(&self, coord: Coordinate) -> String {`

What it does: One scroll, as `phext::fetch` returns it.

(Where these words came from: direct.)

---

## function — Store::toc

Found in src/store.rs at line 318.

Surface: public API.

Signature: `pub fn toc(&self) -> String {`

What it does: The contents listing, as `phext::textmap` writes it.

(Where these words came from: direct.)

---

## function — Store::byte_index

Found in src/store.rs at line 323.

Surface: public API.

Signature: `pub fn byte_index(&self) -> String {`

What it does: Where each scroll starts in the file, one coordinate per line.

(Where these words came from: direct.)

---

## function — Store::reseed

Found in src/store.rs at line 331.

Surface: public API.

Signature: `pub fn reseed(&mut self, now: DateTime<Utc>) -> Result<(), Error> {`

What it does: Function Store::reseed takes &mut self, now: DateTime<Utc>. It gives back Result<(), Error>.

(Where these words came from: inferred.)

---

## function — Store::init_empty

Found in src/store.rs at line 339.

Surface: public API.

Signature: `pub fn init_empty(path: impl AsRef<Path>) -> Result<(), Error> {`

What it does: Create a chat with a General thread and no demo cast. A file that already has content is left untouched. `--seed` is the only path that replaces an existing chat, and it wipes that file.

(Where these words came from: direct.)

---

## function — Store::ensure_member

Found in src/store.rs at line 354.

Surface: public API.

Signature: `pub fn ensure_member(&mut self, id: &str, name: &str) -> Result<bool, Error> {`

What it does: Add a reader, or refresh their display name. Interests start empty. Returns true when the roster scroll was created.

(Where these words came from: direct.)

---

## function — Store::note_interest

Found in src/store.rs at line 394.

Surface: public API.

Signature: `pub fn note_interest(&mut self, id: &str, thread: Coordinate) -> Result<(), Error> {`

What it does: Remember a thread this member posted in or opened. The feed still uses the existing distance ranking. No interests means every thread is equally near, so a new member sees recency until this list grows. The first thread stays their home shelf.

(Where these words came from: direct.)

---

## function — Store::save_settings

Found in src/store.rs at line 418.

Surface: public API.

Signature: `pub fn save_settings(&mut self, settings: &LaneSettings) -> Result<(), Error> {`

What it does: Write the settings scroll, replacing any earlier one.

(Where these words came from: direct.)

---

## function — Store::place

Found in src/store.rs at line 422.

Surface: public API.

Signature: `pub fn place(&mut self, post: Post) -> Result<Placed, Error> {`

What it does: Function Store::place takes &mut self, post: Post. It gives back Result<Placed, Error>.

(Where these words came from: inferred.)

---

## function — world_from

Found in src/store.rs at line 587.

Surface: public API.

Signature: `pub fn world_from(doc: &str) -> World {`

What it does: Function world_from takes doc: &str. It gives back World.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/phext-lanes/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

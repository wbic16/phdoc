# phext-lanes — reference as of commit cbb12a25bbad

Commit cbb12a25bbadcddb6b719a59cb463f8c0a741480. 13 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 15 (15 default, 0 optional).

- axum [dependencies] — =0.7.9
- chrono [dependencies] — =0.4.39
- clap [dependencies] — =4.5.23
- hyper [dependencies] — =1.5.2
- hyper-util [dependencies] — =0.1.10
- libphext [dependencies] — =0.3.1
- serde [dependencies] — =1.0.217
- serde_json [dependencies] — =1.0.134
- tokio [dependencies] — =1.42.0
- http-body-util [dev-dependencies] — =0.1.5
- tower [dev-dependencies] — =0.5.3
- hmac [dependencies] — =0.12.1
- sha2 [dependencies] — =0.10.8
- ureq [dependencies] — =2.8.0
- url [dependencies] — =2.5.2

Total packages in the lock file, this one included: 134.

## Authors

Author counts over the 13 audited mainline commits folded to this point.

The author of record is **Will Bickford** — 4 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 3 commits
- Verse — 1 commit

Other authors, kept as themselves (not Exocortical cluster personas; never folded):

- Cursor Agent <cursoragent@cursor.com> — 9 commits

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 1 files, 233 lines total, 0 blank, 0 comment, 233 code
- Rust: 17 files, 9642 lines total, 0 blank, 0 comment, 9642 code
- Shell: 1 files, 797 lines total, 0 blank, 0 comment, 797 code
- TOML: 1 files, 28 lines total, 0 blank, 0 comment, 28 code

All languages: 20 files, 10700 lines total, 10700 lines of code. Functions read: 450.

Largest files:


Largest functions, by span in lines:


## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 85 test functions counted in the source, across 0 files in test directories or whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 31 of 236 (13%).

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/phext-lanes.git
- Branch: exo
- Head commit: cbb12a25bbadcddb6b719a59cb463f8c0a741480
- Working tree: the committed tree at this commit — a historical reference has no working tree

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `certbot_print` (deploy.sh:231) — Function certbot_print takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `die` (deploy.sh:66) — Function die takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is gi
- function `dns_points_at` (deploy.sh:210) — Function dns_points_at takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `domain_v4s` (deploy.sh:199) — Function domain_v4s takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `ensure_build_path` (deploy.sh:241) — Function ensure_build_path takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `ensure_rate_limit` (deploy.sh:91) — Function ensure_rate_limit takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `expand_tilde` (deploy.sh:52) — Function expand_tilde takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `http_code` (deploy.sh:221) — Function http_code takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it
- function `main` (deploy.sh:249) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `port_is_foreign` (deploy.sh:72) — Function port_is_foreign takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `public_ipv4` (deploy.sh:177) — Function public_ipv4 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `rollback_nginx` (deploy.sh:611) — Function rollback_nginx takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `site_matches` (deploy.sh:143) — Function site_matches takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `ss_listen_dump` (deploy.sh:159) — Function ss_listen_dump takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `usage` (deploy.sh:28) — Function usage takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is 
- struct `AdminTokenCheck` (src/auth.rs:17) — Deny everyone except a bearer of the configured admin token.
- function `AdminTokenCheck::from_env` (src/auth.rs:22) — Function AdminTokenCheck::from_env takes no inputs beyond the value it is called on, and gives back Self.
- trait `PlotHolderCheck` (src/auth.rs:12) — Proof that the caller holds a founding plot.
- function `from_token` (src/auth.rs:30) — Function from_token takes token: impl Into<String> and gives back Self.
- function `format_feed` (src/calm.rs:11) — Function format_feed takes feed: &Feed, details: bool and gives back String.
- function `format_thread` (src/calm.rs:72) — Function format_thread takes thread: &Thread, details: bool and gives back String.
- function `held_count` (src/calm.rs:56) — Function held_count takes feed: &Feed and gives back usize.
- const `DEFAULT_FILE` (src/config.rs:13) — A fixed value named DEFAULT_FILE, set to "data/chat.phext".
- const `DEFAULT_HOST` (src/config.rs:14) — A fixed value named DEFAULT_HOST, set to "127.0.0.1".
- const `DEFAULT_PORT` (src/config.rs:15) — A fixed value named DEFAULT_PORT, set to 7841.
- const `DEFAULT_USER` (src/config.rs:12) — A fixed value named DEFAULT_USER, set to "mira".
- struct `Listen` (src/config.rs:24) — A data shape named Listen.
- struct `Settings` (src/config.rs:18) — A data shape named Settings.
- function `load` (src/config.rs:35) — Function load takes flag_user: Option<&str>, flag_file: Option<&Path> and gives back Result<Settings, Error>.
- function `load_listen` (src/config.rs:89) — Function load_listen takes flag_host: Option<&str>, flag_port: Option<u16> and gives back Result<Listen, Error>.
- function `resolve` (src/config.rs:53) — Function resolve takes flag_user: Option<&str>, flag_file: Option<&Path>, env_user: Option<&str>, env_file: Option<&str>
- function `resolve_listen` (src/config.rs:100) — Function resolve_listen takes flag_host: Option<&str>, flag_port: Option<u16>, env_host: Option<&str>, env_port: Option<
- const `ROSTER_LIBRARY` (src/coord.rs:12) — Library reserved for reader records.
- enum `Target` (src/coord.rs:31) — A named choice named Target.
- const `WEIGHTS` (src/coord.rs:17) — Library, shelf, series, collection, volume, book.
- function `arms` (src/coord.rs:102) — Function arms takes coord: Coordinate and gives back [usize; 6].
- function `format_coord` (src/coord.rs:52) — Function format_coord takes coord: Coordinate and gives back String.
- function `is_card_slot` (src/coord.rs:98) — Function is_card_slot takes coord: Coordinate and gives back bool.
- function `is_roster` (src/coord.rs:94) — Function is_roster takes coord: Coordinate and gives back bool.
- function `min_distance` (src/coord.rs:122) — Function min_distance takes thread: Coordinate, interests: &[Coordinate] and gives back u64.
- function `next_message_scroll` (src/coord.rs:132) — Smallest free message scroll in chapter 1, section 1.
- function `one_line` (src/coord.rs:167) — Function one_line takes text: &str and gives back String.
- function `parse_coord` (src/coord.rs:19) — Function parse_coord takes input: &str and gives back Result<Coordinate, String>.
- function `parse_target` (src/coord.rs:39) — Function parse_target takes input: &str and gives back Result<Target, String>.
- function `sanitize` (src/coord.rs:147) — Strip dimension breaks so a message cannot split the phext.
- function `section_anchor` (src/coord.rs:74) — Function section_anchor takes coord: Coordinate and gives back Coordinate.
- function `section_key` (src/coord.rs:65) — Chapter and section stay.
- function `thread_anchor` (src/coord.rs:80) — Function thread_anchor takes coord: Coordinate and gives back Coordinate.
- function `thread_key` (src/coord.rs:60) — Function thread_key takes coord: Coordinate and gives back String.
- function `topic_key` (src/coord.rs:56) — Function topic_key takes coord: Coordinate and gives back String.
- function `weighted_distance` (src/coord.rs:113) — Function weighted_distance takes a: Coordinate, b: Coordinate and gives back u64.
- function `with_scroll` (src/coord.rs:88) — Function with_scroll takes thread: Coordinate, scroll: usize and gives back Coordinate.
- enum `Error` (src/error.rs:4) — A named choice named Error.
- enum `FabricError` (src/fabric.rs:26) — A named choice named FabricError.
- struct `Hub` (src/fabric.rs:279) — A data shape named Hub.
- function `Hub::open` (src/fabric.rs:286) — Function Hub::open takes default_path: impl Into<PathBuf> and gives back Result<Self, Error>.
- const `MAX_FABRICS` (src/fabric.rs:21) — A fixed value named MAX_FABRICS, set to 100.
- const `MAX_FABRIC_BYTES` (src/fabric.rs:20) — A fixed value named MAX_FABRIC_BYTES, set to 5 * 1024 * 1024.
- struct `ParsedRoute` (src/fabric.rs:78) — A data shape named ParsedRoute.
- enum `Route` (src/fabric.rs:51) — A named choice named Route.
- enum `RouteError` (src/fabric.rs:84) — A named choice named RouteError.
- function `create_empty` (src/fabric.rs:342) — Function create_empty takes &self, requested: Option<&str>, ip: &str and gives back Result<String, FabricError>.
- function `create_upload` (src/fabric.rs:346) — Function create_upload takes &self, requested: Option<&str>, bytes: &[u8], ip: &str, and gives back Result<String, Fabri
- function `day_stamp` (src/fabric.rs:524) — Function day_stamp takes now: chrono::DateTime<chrono::Utc> and gives back String.
- function `download_filename` (src/fabric.rs:199) — Function download_filename takes fabric: Option<&str>, lane_title: Option<&str>, day: &str and gives back String.
- function `fabrics_dir` (src/fabric.rs:192) — Function fabrics_dir takes default_file: &Path and gives back PathBuf.
- function `lane_document` (src/fabric.rs:256) — Function lane_document takes doc: &str, thread: &Thread and gives back String.
- function `list` (src/fabric.rs:305) — Function list takes no inputs beyond the value it is called on, and gives back Result<Vec<String>, FabricError>.
- function `normalize_name` (src/fabric.rs:166) — Function normalize_name takes raw: &str and gives back Result<String, FabricError>.
- function `parse_route` (src/fabric.rs:89) — Function parse_route takes path: &str and gives back Result<ParsedRoute, RouteError>.
- function `parse_upload` (src/fabric.rs:231) — Function parse_upload takes bytes: &[u8] and gives back Result<String, FabricError>.
- function `roster_only` (src/fabric.rs:245) — Function roster_only takes no inputs beyond the value it is called on, and gives back String.
- function `with_store` (src/fabric.rs:330) — Function with_store takes &self, fabric: Option<&str>, body: impl FnOnce and gives back T, ).
- struct `Issued` (src/invite.rs:112) — A data shape named Issued.
- const `JOIN_COOKIE` (src/invite.rs:18) — A fixed value named JOIN_COOKIE, set to "lanes_join".
- struct `Listed` (src/invite.rs:120) — A data shape named Listed.
- struct `Owners` (src/invite.rs:27) — A data shape named Owners.
- function `Owners::from_env` (src/invite.rs:32) — Function Owners::from_env takes no inputs beyond the value it is called on, and gives back Self.
- enum `RedeemFail` (src/invite.rs:130) — A named choice named RedeemFail.
- function `RedeemFail::message` (src/invite.rs:138) — Function RedeemFail::message takes no inputs beyond the value it is called on, and gives back &'static str.
- function `beside` (src/invite.rs:96) — Default invite file: next to the phext, not inside the checkout when the phext lives in `/var/lib/phext-lanes`.
- function `create` (src/invite.rs:164) — Function create takes path: &Path, uses: u32, exp: Option<DateTime<Utc>>, now: DateTime<Utc>, and gives back Result<Issu
- function `invite_only` (src/invite.rs:60) — Function invite_only takes no inputs beyond the value it is called on, and gives back bool.
- function `is_empty` (src/invite.rs:64) — Function is_empty takes no inputs beyond the value it is called on, and gives back bool.
- function `len` (src/invite.rs:68) — Function len takes no inputs beyond the value it is called on, and gives back usize.
- function `list` (src/invite.rs:197) — Function list takes path: &Path and gives back Result<Vec<Listed>, Error>.
- function `matches` (src/invite.rs:72) — Function matches takes &self, who: &str, email: &str and gives back bool.
- function `parse` (src/invite.rs:39) — Function parse takes raw: &str and gives back Self.
- function `redeem` (src/invite.rs:228) — Function redeem takes path: &Path, token: &str, now: DateTime<Utc> and gives back Result<(), RedeemFail>.
- function `resolve` (src/invite.rs:104) — `LANES_INVITES` wins.
- function `revoke` (src/invite.rs:214) — Function revoke takes path: &Path, id: &str and gives back Result<(), Error>.
- function `token_shape` (src/invite.rs:261) — Function token_shape takes token: &str and gives back bool.
- re-export `error::Error` (src/lib.rs:25) — A re-export: error::Error is made reachable through this module, so other code can use it from here.
- enum `Bucket` (src/limit.rs:16) — A named choice named Bucket.
- struct `Limiter` (src/limit.rs:27) — A data shape named Limiter.
- function `Limiter::standard` (src/limit.rs:35) — Function Limiter::standard takes no inputs beyond the value it is called on, and gives back Self.
- function `hit` (src/limit.rs:49) — Function hit takes &self, bucket: Bucket, id: &str and gives back Result<(), ()>.
- function `with_caps` (src/limit.rs:39) — Function with_caps takes auth_per_ip: usize, writes_per_ip: usize, writes_per_account: usize and gives back Self.
- function `lane_id` (src/link.rs:22) — Function lane_id takes coord: Coordinate and gives back String.
- function `parse_lane_id` (src/link.rs:35) — Function parse_lane_id takes id: &str and gives back Option<Coordinate>.
- function `parse_post_id` (src/link.rs:50) — Function parse_post_id takes id: &str and gives back Option<Coordinate>.
- function `parse_section_lane_id` (src/link.rs:41) — Function parse_section_lane_id takes id: &str and gives back Option<Coordinate>.
- function `post_id` (src/link.rs:26) — Function post_id takes coord: Coordinate and gives back String.
- function `section_lane_id` (src/link.rs:31) — Stable id for one chapter and section.
- flag `--- phext::index ---` (src/main.rs:478) — A command line flag, --- phext::index ---.
- flag `--coord` (src/main.rs:532) — A command line flag, --coord.
- flag `--days` (src/main.rs:509) — A command line flag, --days.
- flag `--days must be 3650 or fewer` (src/main.rs:218) — A command line flag, --days must be 3650 or fewer.
- flag `--details` (src/main.rs:548) — A command line flag, --details.
- flag `--empty` (src/main.rs:505) — A command line flag, --empty.
- flag `--host` (src/main.rs:595) — A command line flag, --host.
- flag `--index` (src/main.rs:588) — A command line flag, --index.
- flag `--port` (src/main.rs:595) — A command line flag, --port.
- flag `--preview` (src/main.rs:558) — A command line flag, --preview.
- flag `--scroll` (src/main.rs:575) — A command line flag, --scroll.
- flag `--seed` (src/main.rs:544) — A command line flag, --seed.
- flag `--text` (src/main.rs:531) — A command line flag, --text.
- flag `--user` (src/main.rs:548) — A command line flag, --user.
- flag `--uses` (src/main.rs:509) — A command line flag, --uses.
- struct `Message` (src/model.rs:46) — A data shape named Message.
- function `Message::poster_key` (src/model.rs:59) — Share caps follow the OAuth identity when the scroll has one.
- const `PLAIN_AUTHOR` (src/model.rs:43) — Author stamped on a scroll that is not a Lanes message.
- enum `Scroll` (src/model.rs:80) — A named choice named Scroll.
- struct `Thread` (src/model.rs:69) — A data shape named Thread.
- struct `User` (src/model.rs:35) — A data shape named User.
- function `User::to_scroll` (src/model.rs:96) — Function User::to_scroll takes no inputs beyond the value it is called on, and gives back String.
- function `content_tokens` (src/model.rs:314) — Lowercased content words.
- function `format_card` (src/model.rs:126) — Function format_card takes topic: &str, title: &str, keywords: &str and gives back String.
- function `format_message` (src/model.rs:135) — Function format_message takes author: &str, ts: DateTime<Utc>, text: &str and gives back String.
- function `format_message_as` (src/model.rs:139) — Function format_message_as takes author: &str, who: &str, ts: DateTime<Utc>, text: &str and gives back String.
- function `keyword_line` (src/model.rs:209) — Function keyword_line takes text: &str and gives back String.
- function `member_id` (src/model.rs:113) — Roster ids keep a single colon (`github:100`).
- function `opening_line` (src/model.rs:155) — First non-empty line, clipped.
- function `parse_scroll` (src/model.rs:216) — Function parse_scroll takes raw: &str and gives back Option<Scroll>.
- function `preview` (src/model.rs:335) — Function preview takes text: &str, limit: usize and gives back String.
- function `rest_after_opening` (src/model.rs:170) — The scroll after its first non-empty line.
- function `title_from_text` (src/model.rs:192) — Function title_from_text takes text: &str and gives back String.
- struct `Identity` (src/oauth.rs:28) — A data shape named Identity.
- trait `IdentitySource` (src/oauth.rs:35) — A trait named IdentitySource: a set of methods that a type can promise to give.
- struct `Oauth` (src/oauth.rs:53) — A data shape named Oauth.
- function `Oauth::from_env` (src/oauth.rs:163) — Function Oauth::from_env takes no inputs beyond the value it is called on, and gives back Self.
- struct `Pending` (src/oauth.rs:98) — A data shape named Pending.
- struct `Session` (src/oauth.rs:69) — A data shape named Session.
- const `TEST_CSRF` (src/oauth.rs:23) — A fixed value named TEST_CSRF, set to "csrf-test-token".
- function `authorize_url` (src/oauth.rs:247) — Function authorize_url takes &self, provider_name: &str, next: &str, and gives back Result<(String, String), String>.
- function `clean_email` (src/oauth.rs:532) — Function clean_email takes raw: &str and gives back String.
- function `clean_handle` (src/oauth.rs:553) — Function clean_handle takes raw: &str and gives back String.
- function `clear_cookie` (src/oauth.rs:439) — Function clear_cookie takes name: &str, secure: bool and gives back String.
- function `closed` (src/oauth.rs:192) — Function closed takes no inputs beyond the value it is called on, and gives back Self.
- function `cookie_from_headers` (src/oauth.rs:622) — Function cookie_from_headers takes headers: &axum::http::HeaderMap and gives back String.
- function `cookie_header` (src/oauth.rs:426) — Function cookie_header takes name: &str, value: &str, max_age: i64, secure: bool and gives back String.
- function `cookie_secure` (src/oauth.rs:243) — Function cookie_secure takes no inputs beyond the value it is called on, and gives back bool.
- function `csrf_matches` (src/oauth.rs:355) — Function csrf_matches takes &self, session: &Session, header: &str and gives back bool.
- function `display_name` (src/oauth.rs:516) — Function display_name takes raw: &str, fallback: &str and gives back String.
- function `finish` (src/oauth.rs:291) — Function finish takes &self, provider_name: &str, code: &str, state: &str, pending_cookie: &str, and gives back Result<(
- function `pending_cookie` (src/oauth.rs:435) — Function pending_cookie takes value: &str, secure: bool and gives back String.
- function `pending_cookie_name` (src/oauth.rs:422) — Function pending_cookie_name takes no inputs beyond the value it is called on, and gives back &'static str.
- function `pending_token` (src/oauth.rs:618) — Function pending_token takes cookie_header: &str and gives back Option<String>.
- function `providers` (src/oauth.rs:229) — Function providers takes no inputs beyond the value it is called on, and gives back Vec<&'static str>.
- function `read_session` (src/oauth.rs:333) — Function read_session takes &self, cookie_header: &str and gives back Option<Session>.
- function `ready` (src/oauth.rs:223) — Function ready takes no inputs beyond the value it is called on, and gives back bool.
- function `safe_next` (src/oauth.rs:484) — Function safe_next takes next: &str and gives back String.
- function `session_cookie` (src/oauth.rs:431) — Function session_cookie takes value: &str, secure: bool and gives back String.
- function `session_cookie_name` (src/oauth.rs:418) — Function session_cookie_name takes no inputs beyond the value it is called on, and gives back &'static str.
- function `test_ready` (src/oauth.rs:179) — Function test_ready takes no inputs beyond the value it is called on, and gives back Self.
- function `test_session_cookie` (src/oauth.rs:359) — Function test_session_cookie takes no inputs beyond the value it is called on, and gives back String.
- const `AUTHOR_CAP` (src/rank.rs:19) — A fixed value named AUTHOR_CAP, set to 2.
- struct `Feed` (src/rank.rs:28) — A data shape named Feed.
- struct `FeedThread` (src/rank.rs:34) — A data shape named FeedThread.
- const `LOUD_AT` (src/rank.rs:22) — Authors with at least this many messages are loud.
- struct `Lane` (src/rank.rs:46) — A data shape named Lane.
- const `MAX_THREADS` (src/rank.rs:23) — A fixed value named MAX_THREADS, set to 8.
- struct `MapEntry` (src/rank.rs:65) — A data shape named MapEntry.
- struct `Suggestion` (src/rank.rs:54) — A data shape named Suggestion.
- function `admitted_by` (src/rank.rs:498) — Function admitted_by takes feed: &Feed, author: &str and gives back usize.
- function `age_secs` (src/rank.rs:96) — Function age_secs takes ts: DateTime<Utc>, now: DateTime<Utc> and gives back u64.
- function `build_feed` (src/rank.rs:113) — Function build_feed takes threads: &[Thread], interests: &[Coordinate], now: DateTime<Utc>, cap: usize, max_threads: usi
- function `fresh_thread` (src/rank.rs:443) — Function fresh_thread takes threads: &[Thread], home: Coordinate and gives back Coordinate.
- function `is_flooded` (src/rank.rs:272) — Function is_flooded takes count: usize, share: f64 and gives back bool.
- function `majority` (src/rank.rs:256) — Function majority takes messages: &[Message] and gives back (Option<String>, usize, f64).
- function `map_of` (src/rank.rs:228) — Function map_of takes threads: &[Thread], interests: &[Coordinate] and gives back Vec<MapEntry>.
- function `nearness` (src/rank.rs:87) — Function nearness takes distance: u64 and gives back &'static str.
- function `suggest` (src/rank.rs:299) — Function suggest takes threads: &[Thread], interests: &[Coordinate], text: &str, now: DateTime<Utc>, and gives back Sugg
- function `suggest_fresh` (src/rank.rs:278) — A new book on the reader's home shelf.
- function `thread_score` (src/rank.rs:80) — Function thread_score takes distance: u64, age_secs: u64 and gives back f64.
- const `ADA` (src/seed.rs:14) — A fixed value named ADA, set to "Ada North".
- const `BREAD` (src/seed.rs:23) — A fixed value named BREAD, set to "Bread hour".
- const `BREAD_THREAD` (src/seed.rs:30) — A fixed value named BREAD_THREAD, set to "4.1.1/1.1.1".
- const `CASEY` (src/seed.rs:16) — A fixed value named CASEY, set to "Casey Horn".
- const `FERN` (src/seed.rs:19) — A fixed value named FERN, set to "Fern wall".
- const `FERN_THREAD` (src/seed.rs:26) — A fixed value named FERN_THREAD, set to "1.2.1/1.1.1".
- const `HALL` (src/seed.rs:18) — A fixed value named HALL, set to "General".
- const `HALL_THREAD` (src/seed.rs:25) — A fixed value named HALL_THREAD, set to "1.1.1/1.1.1".
- const `JULES` (src/seed.rs:13) — A fixed value named JULES, set to "Jules Fen".
- const `MIRA` (src/seed.rs:12) — A fixed value named MIRA, set to "Mira Quill".
- const `PLANES` (src/seed.rs:21) — A fixed value named PLANES, set to "Hand planes".
- const `PLANES_THREAD` (src/seed.rs:28) — A fixed value named PLANES_THREAD, set to "2.1.1/1.1.1".
- const `REN` (src/seed.rs:15) — A fixed value named REN, set to "Ren Sato".
- const `RIVER` (src/seed.rs:22) — A fixed value named RIVER, set to "River wards".
- const `RIVER_THREAD` (src/seed.rs:29) — A fixed value named RIVER_THREAD, set to "3.1.1/1.1.1".
- const `SWAP` (src/seed.rs:20) — A fixed value named SWAP, set to "Seed swap".
- const `SWAP_THREAD` (src/seed.rs:27) — A fixed value named SWAP_THREAD, set to "1.2.1/1.2.1".
- function `blank` (src/seed.rs:197) — A new chat: one General thread, no readers, no messages.
- function `demo_instant` (src/seed.rs:174) — Function demo_instant takes no inputs beyond the value it is called on, and gives back DateTime<Utc>.
- function `document` (src/seed.rs:207) — Build the demo phext by inserting each scroll in coordinate order.
- function `readers` (src/seed.rs:178) — Function readers takes no inputs beyond the value it is called on, and gives back Vec<User>.
- struct `AppState` (src/server.rs:34) — A data shape named AppState.
- function `AppState::open` (src/server.rs:44) — Function AppState::open takes path: std::path::PathBuf and gives back Result<Self, Error>.
- function `listen_addr` (src/server.rs:109) — `host:port`, with brackets when `host` is an IPv6 address.
- function `router` (src/server.rs:156) — Function router takes state: AppState and gives back Router.
- function `serve` (src/server.rs:117) — Function serve takes path: std::path::PathBuf, host: &str, port: u16 and gives back Result<(), Error>.
- function `with_auth` (src/server.rs:57) — Function with_auth takes path: std::path::PathBuf, auth: Arc<dyn PlotHolderCheck>, and gives back Result<Self, Error>.
- struct `Placed` (src/store.rs:146) — A data shape named Placed.
- struct `Post` (src/store.rs:137) — A data shape named Post.
- struct `Store` (src/store.rs:154) — A data shape named Store.
- function `Store::open` (src/store.rs:164) — Function Store::open takes path: impl Into<PathBuf> and gives back Result<Self, Error>.
- struct `World` (src/store.rs:23) — A data shape named World.
- function `World::find_user` (src/store.rs:29) — Function World::find_user takes &self, query: &str and gives back Option<&User>.
- function `byte_index` (src/store.rs:222) — Offsets from `phext::index`, one coordinate per line.
- function `doc` (src/store.rs:181) — Function doc takes no inputs beyond the value it is called on, and gives back &str.
- function `ensure_member` (src/store.rs:261) — Add a reader, or refresh their display name.
- function `find_lane` (src/store.rs:56) — Function find_lane takes &self, id: &str and gives back Option<&Thread>.
- function `find_post` (src/store.rs:81) — The message scroll named by a post id.
- function `find_thread` (src/store.rs:93) — A thread title, or a thread or scroll coordinate.
- function `from_doc` (src/store.rs:196) — Function from_doc takes path: impl Into<PathBuf>, doc: String and gives back Self.
- function `init_empty` (src/store.rs:246) — Create a chat with a General thread and no demo cast.
- function `load_doc` (src/store.rs:186) — Load a fabric that already exists.
- function `note_interest` (src/store.rs:305) — Remember a thread this member posted in or opened.
- function `path` (src/store.rs:177) — Function path takes no inputs beyond the value it is called on, and gives back &Path.
- function `place` (src/store.rs:331) — Function place takes &mut self, post: Post and gives back Result<Placed, Error>.
- function `reseed` (src/store.rs:238) — Function reseed takes &mut self, now: DateTime<Utc> and gives back Result<(), Error>.
- function `select` (src/store.rs:212) — `phext::fetch`
- function `thread` (src/store.rs:45) — Function thread takes &self, key: &str and gives back Option<&Thread>.
- function `toc` (src/store.rs:217) — `phext::textmap`
- function `world` (src/store.rs:207) — The parsed fabric.
- function `world_from` (src/store.rs:478) — Function world_from takes doc: &str and gives back World.

## function — certbot_print

Found in deploy.sh at line 231.

Signature: `certbot_print()`

What it does: Function certbot_print takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — die

Found in deploy.sh at line 66.

Signature: `die()`

What it does: Function die takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — dns_points_at

Found in deploy.sh at line 210.

Signature: `dns_points_at()`

What it does: Function dns_points_at takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — domain_v4s

Found in deploy.sh at line 199.

Signature: `domain_v4s()`

What it does: Function domain_v4s takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — ensure_build_path

Found in deploy.sh at line 241.

Signature: `ensure_build_path()`

What it does: Function ensure_build_path takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — ensure_rate_limit

Found in deploy.sh at line 91.

Signature: `ensure_rate_limit()`

What it does: Function ensure_rate_limit takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — expand_tilde

Found in deploy.sh at line 52.

Signature: `expand_tilde()`

What it does: Function expand_tilde takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — http_code

Found in deploy.sh at line 221.

Signature: `http_code()`

What it does: Function http_code takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — main

Found in deploy.sh at line 249.

Signature: `main()`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — port_is_foreign

Found in deploy.sh at line 72.

Signature: `port_is_foreign()`

What it does: Function port_is_foreign takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — public_ipv4

Found in deploy.sh at line 177.

Signature: `public_ipv4()`

What it does: Function public_ipv4 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — rollback_nginx

Found in deploy.sh at line 611.

Signature: `rollback_nginx()`

What it does: Function rollback_nginx takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — site_matches

Found in deploy.sh at line 143.

Signature: `site_matches()`

What it does: Function site_matches takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — ss_listen_dump

Found in deploy.sh at line 159.

Signature: `ss_listen_dump()`

What it does: Function ss_listen_dump takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## function — usage

Found in deploy.sh at line 28.

Signature: `usage()`

What it does: Function usage takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

## struct — AdminTokenCheck

Found in src/auth.rs at line 17.

Signature: `pub struct AdminTokenCheck {`

What it does: Deny everyone except a bearer of the configured admin token.

(Where these words came from: doc comment.)

## function — AdminTokenCheck::from_env

Found in src/auth.rs at line 22.

Signature: `pub fn from_env() -> Self {`

What it does: Function AdminTokenCheck::from_env takes no inputs beyond the value it is called on, and gives back Self.

(Where these words came from: signature.)

## trait — PlotHolderCheck

Found in src/auth.rs at line 12.

Signature: `pub trait PlotHolderCheck: Send + Sync {`

What it does: Proof that the caller holds a founding plot.  `proof` is the secret the request presented. An empty proof is never enough. A future Tessera check can replace the admin-token implementation without changing the call sites.

(Where these words came from: doc comment.)

## function — from_token

Found in src/auth.rs at line 30.

Signature: `pub fn from_token(token: impl Into<String>) -> Self {`

What it does: Function from_token takes token: impl Into<String> and gives back Self.

(Where these words came from: signature.)

## function — format_feed

Found in src/calm.rs at line 11.

Signature: `pub fn format_feed(feed: &Feed, details: bool) -> String {`

What it does: Function format_feed takes feed: &Feed, details: bool and gives back String.

(Where these words came from: signature.)

## function — format_thread

Found in src/calm.rs at line 72.

Signature: `pub fn format_thread(thread: &Thread, details: bool) -> String {`

What it does: Function format_thread takes thread: &Thread, details: bool and gives back String.

(Where these words came from: signature.)

## function — held_count

Found in src/calm.rs at line 56.

Signature: `pub fn held_count(feed: &Feed) -> usize {`

What it does: Function held_count takes feed: &Feed and gives back usize.

(Where these words came from: signature.)

## const — DEFAULT_FILE

Found in src/config.rs at line 13.

Signature: `pub const DEFAULT_FILE: &str = "data/chat.phext";`

What it does: A fixed value named DEFAULT_FILE, set to "data/chat.phext".

(Where these words came from: signature.)

## const — DEFAULT_HOST

Found in src/config.rs at line 14.

Signature: `pub const DEFAULT_HOST: &str = "127.0.0.1";`

What it does: A fixed value named DEFAULT_HOST, set to "127.0.0.1".

(Where these words came from: signature.)

## const — DEFAULT_PORT

Found in src/config.rs at line 15.

Signature: `pub const DEFAULT_PORT: u16 = 7841;`

What it does: A fixed value named DEFAULT_PORT, set to 7841.

(Where these words came from: signature.)

## const — DEFAULT_USER

Found in src/config.rs at line 12.

Signature: `pub const DEFAULT_USER: &str = "mira";`

What it does: A fixed value named DEFAULT_USER, set to "mira".

(Where these words came from: signature.)

## struct — Listen

Found in src/config.rs at line 24.

Signature: `pub struct Listen {`

What it does: A data shape named Listen. Its fields are: pub host: String; pub port: u16.

(Where these words came from: signature.)

## struct — Settings

Found in src/config.rs at line 18.

Signature: `pub struct Settings {`

What it does: A data shape named Settings. Its fields are: pub user: String; pub file: PathBuf.

(Where these words came from: signature.)

## function — load

Found in src/config.rs at line 35.

Signature: `pub fn load(flag_user: Option<&str>, flag_file: Option<&Path>) -> Result<Settings, Error> {`

What it does: Function load takes flag_user: Option<&str>, flag_file: Option<&Path> and gives back Result<Settings, Error>.

(Where these words came from: signature.)

## function — load_listen

Found in src/config.rs at line 89.

Signature: `pub fn load_listen(flag_host: Option<&str>, flag_port: Option<u16>) -> Result<Listen, Error> {`

What it does: Function load_listen takes flag_host: Option<&str>, flag_port: Option<u16> and gives back Result<Listen, Error>.

(Where these words came from: signature.)

## function — resolve

Found in src/config.rs at line 53.

Signature: `pub fn resolve( flag_user: Option<&str>, flag_file: Option<&Path>, env_user: Option<&str>, env_file: Option<&str>, config_json: Option<&str>, ) -> Result<Settings, Error> {`

What it does: Function resolve takes flag_user: Option<&str>, flag_file: Option<&Path>, env_user: Option<&str>, env_file: Option<&str>, config_json: Option<&str>, and gives back Result<Settings, Error>.

(Where these words came from: signature.)

## function — resolve_listen

Found in src/config.rs at line 100.

Signature: `pub fn resolve_listen( flag_host: Option<&str>, flag_port: Option<u16>, env_host: Option<&str>, env_port: Option<&str>, ) -> Result<Listen, Error> {`

What it does: Function resolve_listen takes flag_host: Option<&str>, flag_port: Option<u16>, env_host: Option<&str>, env_port: Option<&str>, and gives back Result<Listen, Error>.

(Where these words came from: signature.)

## const — ROSTER_LIBRARY

Found in src/coord.rs at line 12.

Signature: `pub const ROSTER_LIBRARY: usize = 5;`

What it does: Library reserved for reader records. Chat topics use the other libraries.

(Where these words came from: doc comment.)

## enum — Target

Found in src/coord.rs at line 31.

Signature: `pub enum Target {`

What it does: A named choice named Target. It is one of: Thread; Scroll.

(Where these words came from: signature.)

## const — WEIGHTS

Found in src/coord.rs at line 17.

Signature: `pub const WEIGHTS: [u64; 6] = [16, 8, 4, 2, 2, 1];`

What it does: Library, shelf, series, collection, volume, book. Topic steps outweigh thread steps, so a different library cannot outrank a subscribed shelf just by being newer.

(Where these words came from: doc comment.)

## function — arms

Found in src/coord.rs at line 102.

Signature: `pub fn arms(coord: Coordinate) -> [usize; 6] {`

What it does: Function arms takes coord: Coordinate and gives back [usize; 6].

(Where these words came from: signature.)

## function — format_coord

Found in src/coord.rs at line 52.

Signature: `pub fn format_coord(coord: Coordinate) -> String {`

What it does: Function format_coord takes coord: Coordinate and gives back String.

(Where these words came from: signature.)

## function — is_card_slot

Found in src/coord.rs at line 98.

Signature: `pub fn is_card_slot(coord: Coordinate) -> bool {`

What it does: Function is_card_slot takes coord: Coordinate and gives back bool.

(Where these words came from: signature.)

## function — is_roster

Found in src/coord.rs at line 94.

Signature: `pub fn is_roster(coord: Coordinate) -> bool {`

What it does: Function is_roster takes coord: Coordinate and gives back bool.

(Where these words came from: signature.)

## function — min_distance

Found in src/coord.rs at line 122.

Signature: `pub fn min_distance(thread: Coordinate, interests: &[Coordinate]) -> u64 {`

What it does: Function min_distance takes thread: Coordinate, interests: &[Coordinate] and gives back u64.

(Where these words came from: signature.)

## function — next_message_scroll

Found in src/coord.rs at line 132.

Signature: `pub fn next_message_scroll(used: impl IntoIterator<Item = usize>) -> usize {`

What it does: Smallest free message scroll in chapter 1, section 1. Scroll 1 is reserved for the thread card.

(Where these words came from: doc comment.)

## function — one_line

Found in src/coord.rs at line 167.

Signature: `pub fn one_line(text: &str) -> String {`

What it does: Function one_line takes text: &str and gives back String.

(Where these words came from: signature.)

## function — parse_coord

Found in src/coord.rs at line 19.

Signature: `pub fn parse_coord(input: &str) -> Result<Coordinate, String> {`

What it does: Function parse_coord takes input: &str and gives back Result<Coordinate, String>.

(Where these words came from: signature.)

## function — parse_target

Found in src/coord.rs at line 39.

Signature: `pub fn parse_target(input: &str) -> Result<Target, String> {`

What it does: Function parse_target takes input: &str and gives back Result<Target, String>.

(Where these words came from: signature.)

## function — sanitize

Found in src/coord.rs at line 147.

Signature: `pub fn sanitize(text: &str) -> String {`

What it does: Strip dimension breaks so a message cannot split the phext. Newlines stay; they are inside the scroll.

(Where these words came from: doc comment.)

## function — section_anchor

Found in src/coord.rs at line 74.

Signature: `pub fn section_anchor(coord: Coordinate) -> Coordinate {`

What it does: Function section_anchor takes coord: Coordinate and gives back Coordinate.

(Where these words came from: signature.)

## function — section_key

Found in src/coord.rs at line 65.

Signature: `pub fn section_key(coord: Coordinate) -> String {`

What it does: Chapter and section stay. Scrolls in one section share a lane.

(Where these words came from: doc comment.)

## function — thread_anchor

Found in src/coord.rs at line 80.

Signature: `pub fn thread_anchor(coord: Coordinate) -> Coordinate {`

What it does: Function thread_anchor takes coord: Coordinate and gives back Coordinate.

(Where these words came from: signature.)

## function — thread_key

Found in src/coord.rs at line 60.

Signature: `pub fn thread_key(coord: Coordinate) -> String {`

What it does: Function thread_key takes coord: Coordinate and gives back String.

(Where these words came from: signature.)

## function — topic_key

Found in src/coord.rs at line 56.

Signature: `pub fn topic_key(coord: Coordinate) -> String {`

What it does: Function topic_key takes coord: Coordinate and gives back String.

(Where these words came from: signature.)

## function — weighted_distance

Found in src/coord.rs at line 113.

Signature: `pub fn weighted_distance(a: Coordinate, b: Coordinate) -> u64 {`

What it does: Function weighted_distance takes a: Coordinate, b: Coordinate and gives back u64.

(Where these words came from: signature.)

## function — with_scroll

Found in src/coord.rs at line 88.

Signature: `pub fn with_scroll(thread: Coordinate, scroll: usize) -> Coordinate {`

What it does: Function with_scroll takes thread: Coordinate, scroll: usize and gives back Coordinate.

(Where these words came from: signature.)

## enum — Error

Found in src/error.rs at line 4.

Signature: `pub enum Error {`

What it does: A named choice named Error. It is one of: Io; Bad.

(Where these words came from: signature.)

## enum — FabricError

Found in src/fabric.rs at line 26.

Signature: `pub enum FabricError {`

What it does: A named choice named FabricError. It is one of: Bad; Conflict; Limited; Forbidden; Missing; Io.

(Where these words came from: signature.)

## struct — Hub

Found in src/fabric.rs at line 279.

Signature: `pub struct Hub {`

What it does: A data shape named Hub. The scanner did not read its fields.

(Where these words came from: signature.)

## function — Hub::open

Found in src/fabric.rs at line 286.

Signature: `pub fn open(default_path: impl Into<PathBuf>) -> Result<Self, Error> {`

What it does: Function Hub::open takes default_path: impl Into<PathBuf> and gives back Result<Self, Error>.

(Where these words came from: signature.)

## const — MAX_FABRICS

Found in src/fabric.rs at line 21.

Signature: `pub const MAX_FABRICS: usize = 100;`

What it does: A fixed value named MAX_FABRICS, set to 100.

(Where these words came from: signature.)

## const — MAX_FABRIC_BYTES

Found in src/fabric.rs at line 20.

Signature: `pub const MAX_FABRIC_BYTES: usize = 5 * 1024 * 1024;`

What it does: A fixed value named MAX_FABRIC_BYTES, set to 5 * 1024 * 1024.

(Where these words came from: signature.)

## struct — ParsedRoute

Found in src/fabric.rs at line 78.

Signature: `pub struct ParsedRoute {`

What it does: A data shape named ParsedRoute. Its fields are: pub fabric: Option<String>; pub route: Route.

(Where these words came from: signature.)

## enum — Route

Found in src/fabric.rs at line 51.

Signature: `pub enum Route {`

What it does: A named choice named Route. It is one of: Home; Lane; Post; Download; Health; State; ApiLane; ApiPost; Room; Suggest; Toc; Scroll; PostMessage; Reset; Access; Create; Upload.

(Where these words came from: signature.)

## enum — RouteError

Found in src/fabric.rs at line 84.

Signature: `pub enum RouteError {`

What it does: A named choice named RouteError. It is one of: Unknown; BadFabric.

(Where these words came from: signature.)

## function — create_empty

Found in src/fabric.rs at line 342.

Signature: `pub fn create_empty(&self, requested: Option<&str>, ip: &str) -> Result<String, FabricError> {`

What it does: Function create_empty takes &self, requested: Option<&str>, ip: &str and gives back Result<String, FabricError>.

(Where these words came from: signature.)

## function — create_upload

Found in src/fabric.rs at line 346.

Signature: `pub fn create_upload( &self, requested: Option<&str>, bytes: &[u8], ip: &str, ) -> Result<String, FabricError> {`

What it does: Function create_upload takes &self, requested: Option<&str>, bytes: &[u8], ip: &str, and gives back Result<String, FabricError>.

(Where these words came from: signature.)

## function — day_stamp

Found in src/fabric.rs at line 524.

Signature: `pub fn day_stamp(now: chrono::DateTime<chrono::Utc>) -> String {`

What it does: Function day_stamp takes now: chrono::DateTime<chrono::Utc> and gives back String.

(Where these words came from: signature.)

## function — download_filename

Found in src/fabric.rs at line 199.

Signature: `pub fn download_filename(fabric: Option<&str>, lane_title: Option<&str>, day: &str) -> String {`

What it does: Function download_filename takes fabric: Option<&str>, lane_title: Option<&str>, day: &str and gives back String.

(Where these words came from: signature.)

## function — fabrics_dir

Found in src/fabric.rs at line 192.

Signature: `pub fn fabrics_dir(default_file: &Path) -> PathBuf {`

What it does: Function fabrics_dir takes default_file: &Path and gives back PathBuf.

(Where these words came from: signature.)

## function — lane_document

Found in src/fabric.rs at line 256.

Signature: `pub fn lane_document(doc: &str, thread: &Thread) -> String {`

What it does: Function lane_document takes doc: &str, thread: &Thread and gives back String.

(Where these words came from: signature.)

## function — list

Found in src/fabric.rs at line 305.

Signature: `pub fn list(&self) -> Result<Vec<String>, FabricError> {`

What it does: Function list takes no inputs beyond the value it is called on, and gives back Result<Vec<String>, FabricError>.

(Where these words came from: signature.)

## function — normalize_name

Found in src/fabric.rs at line 166.

Signature: `pub fn normalize_name(raw: &str) -> Result<String, FabricError> {`

What it does: Function normalize_name takes raw: &str and gives back Result<String, FabricError>.

(Where these words came from: signature.)

## function — parse_route

Found in src/fabric.rs at line 89.

Signature: `pub fn parse_route(path: &str) -> Result<ParsedRoute, RouteError> {`

What it does: Function parse_route takes path: &str and gives back Result<ParsedRoute, RouteError>.

(Where these words came from: signature.)

## function — parse_upload

Found in src/fabric.rs at line 231.

Signature: `pub fn parse_upload(bytes: &[u8]) -> Result<String, FabricError> {`

What it does: Function parse_upload takes bytes: &[u8] and gives back Result<String, FabricError>.

(Where these words came from: signature.)

## function — roster_only

Found in src/fabric.rs at line 245.

Signature: `pub fn roster_only() -> String {`

What it does: Function roster_only takes no inputs beyond the value it is called on, and gives back String.

(Where these words came from: signature.)

## function — with_store

Found in src/fabric.rs at line 330.

Signature: `pub fn with_store<T>( &self, fabric: Option<&str>, body: impl FnOnce(&mut Store) -> T, ) -> Result<T, FabricError> {`

What it does: Function with_store takes &self, fabric: Option<&str>, body: impl FnOnce and gives back T, ).

(Where these words came from: signature.)

## struct — Issued

Found in src/invite.rs at line 112.

Signature: `pub struct Issued {`

What it does: A data shape named Issued. Its fields are: pub id: String; pub token: String; pub uses: u32; pub exp: Option<DateTime<Utc>>.

(Where these words came from: signature.)

## const — JOIN_COOKIE

Found in src/invite.rs at line 18.

Signature: `pub const JOIN_COOKIE: &str = "lanes_join";`

What it does: A fixed value named JOIN_COOKIE, set to "lanes_join".

(Where these words came from: signature.)

## struct — Listed

Found in src/invite.rs at line 120.

Signature: `pub struct Listed {`

What it does: A data shape named Listed. Its fields are: pub id: String; pub uses: u32; pub remaining: u32; pub exp: Option<DateTime<Utc>>; pub revoked: bool; pub created: DateTime<Utc>.

(Where these words came from: signature.)

## struct — Owners

Found in src/invite.rs at line 27.

Signature: `pub struct Owners {`

What it does: A data shape named Owners. The scanner did not read its fields.

(Where these words came from: signature.)

## function — Owners::from_env

Found in src/invite.rs at line 32.

Signature: `pub fn from_env() -> Self {`

What it does: Function Owners::from_env takes no inputs beyond the value it is called on, and gives back Self.

(Where these words came from: signature.)

## enum — RedeemFail

Found in src/invite.rs at line 130.

Signature: `pub enum RedeemFail {`

What it does: A named choice named RedeemFail. It is one of: Invalid; Expired; Revoked; Spent.

(Where these words came from: signature.)

## function — RedeemFail::message

Found in src/invite.rs at line 138.

Signature: `pub fn message(self) -> &'static str {`

What it does: Function RedeemFail::message takes no inputs beyond the value it is called on, and gives back &'static str.

(Where these words came from: signature.)

## function — beside

Found in src/invite.rs at line 96.

Signature: `pub fn beside(phext: &Path) -> PathBuf {`

What it does: Default invite file: next to the phext, not inside the checkout when the phext lives in `/var/lib/phext-lanes`.

(Where these words came from: doc comment.)

## function — create

Found in src/invite.rs at line 164.

Signature: `pub fn create( path: &Path, uses: u32, exp: Option<DateTime<Utc>>, now: DateTime<Utc>, ) -> Result<Issued, Error> {`

What it does: Function create takes path: &Path, uses: u32, exp: Option<DateTime<Utc>>, now: DateTime<Utc>, and gives back Result<Issued, Error>.

(Where these words came from: signature.)

## function — invite_only

Found in src/invite.rs at line 60.

Signature: `pub fn invite_only(&self) -> bool {`

What it does: Function invite_only takes no inputs beyond the value it is called on, and gives back bool.

(Where these words came from: signature.)

## function — is_empty

Found in src/invite.rs at line 64.

Signature: `pub fn is_empty(&self) -> bool {`

What it does: Function is_empty takes no inputs beyond the value it is called on, and gives back bool.

(Where these words came from: signature.)

## function — len

Found in src/invite.rs at line 68.

Signature: `pub fn len(&self) -> usize {`

What it does: Function len takes no inputs beyond the value it is called on, and gives back usize.

(Where these words came from: signature.)

## function — list

Found in src/invite.rs at line 197.

Signature: `pub fn list(path: &Path) -> Result<Vec<Listed>, Error> {`

What it does: Function list takes path: &Path and gives back Result<Vec<Listed>, Error>.

(Where these words came from: signature.)

## function — matches

Found in src/invite.rs at line 72.

Signature: `pub fn matches(&self, who: &str, email: &str) -> bool {`

What it does: Function matches takes &self, who: &str, email: &str and gives back bool.

(Where these words came from: signature.)

## function — parse

Found in src/invite.rs at line 39.

Signature: `pub fn parse(raw: &str) -> Self {`

What it does: Function parse takes raw: &str and gives back Self.

(Where these words came from: signature.)

## function — redeem

Found in src/invite.rs at line 228.

Signature: `pub fn redeem(path: &Path, token: &str, now: DateTime<Utc>) -> Result<(), RedeemFail> {`

What it does: Function redeem takes path: &Path, token: &str, now: DateTime<Utc> and gives back Result<(), RedeemFail>.

(Where these words came from: signature.)

## function — resolve

Found in src/invite.rs at line 104.

Signature: `pub fn resolve(phext: &Path) -> PathBuf {`

What it does: `LANES_INVITES` wins. Otherwise [`beside`].

(Where these words came from: doc comment.)

## function — revoke

Found in src/invite.rs at line 214.

Signature: `pub fn revoke(path: &Path, id: &str) -> Result<(), Error> {`

What it does: Function revoke takes path: &Path, id: &str and gives back Result<(), Error>.

(Where these words came from: signature.)

## function — token_shape

Found in src/invite.rs at line 261.

Signature: `pub fn token_shape(token: &str) -> bool {`

What it does: Function token_shape takes token: &str and gives back bool.

(Where these words came from: signature.)

## re-export — error::Error

Found in src/lib.rs at line 25.

Signature: `pub use error::Error;`

What it does: A re-export: error::Error is made reachable through this module, so other code can use it from here.

(Where these words came from: signature.)

## enum — Bucket

Found in src/limit.rs at line 16.

Signature: `pub enum Bucket {`

What it does: A named choice named Bucket. It is one of: AuthIp; WriteIp; WriteAccount.

(Where these words came from: signature.)

## struct — Limiter

Found in src/limit.rs at line 27.

Signature: `pub struct Limiter {`

What it does: A data shape named Limiter. The scanner did not read its fields.

(Where these words came from: signature.)

## function — Limiter::standard

Found in src/limit.rs at line 35.

Signature: `pub fn standard() -> Self {`

What it does: Function Limiter::standard takes no inputs beyond the value it is called on, and gives back Self.

(Where these words came from: signature.)

## function — hit

Found in src/limit.rs at line 49.

Signature: `pub fn hit(&self, bucket: Bucket, id: &str) -> Result<(), ()> {`

What it does: Function hit takes &self, bucket: Bucket, id: &str and gives back Result<(), ()>.

(Where these words came from: signature.)

## function — with_caps

Found in src/limit.rs at line 39.

Signature: `pub fn with_caps(auth_per_ip: usize, writes_per_ip: usize, writes_per_account: usize) -> Self {`

What it does: Function with_caps takes auth_per_ip: usize, writes_per_ip: usize, writes_per_account: usize and gives back Self.

(Where these words came from: signature.)

## function — lane_id

Found in src/link.rs at line 22.

Signature: `pub fn lane_id(coord: Coordinate) -> String {`

What it does: Function lane_id takes coord: Coordinate and gives back String.

(Where these words came from: signature.)

## function — parse_lane_id

Found in src/link.rs at line 35.

Signature: `pub fn parse_lane_id(id: &str) -> Option<Coordinate> {`

What it does: Function parse_lane_id takes id: &str and gives back Option<Coordinate>.

(Where these words came from: signature.)

## function — parse_post_id

Found in src/link.rs at line 50.

Signature: `pub fn parse_post_id(id: &str) -> Option<Coordinate> {`

What it does: Function parse_post_id takes id: &str and gives back Option<Coordinate>.

(Where these words came from: signature.)

## function — parse_section_lane_id

Found in src/link.rs at line 41.

Signature: `pub fn parse_section_lane_id(id: &str) -> Option<Coordinate> {`

What it does: Function parse_section_lane_id takes id: &str and gives back Option<Coordinate>.

(Where these words came from: signature.)

## function — post_id

Found in src/link.rs at line 26.

Signature: `pub fn post_id(coord: Coordinate) -> String {`

What it does: Function post_id takes coord: Coordinate and gives back String.

(Where these words came from: signature.)

## function — section_lane_id

Found in src/link.rs at line 31.

Signature: `pub fn section_lane_id(coord: Coordinate) -> String {`

What it does: Stable id for one chapter and section. Prefix `s`, eight coordinate parts.

(Where these words came from: doc comment.)

## flag — --- phext::index ---

Found in src/main.rs at line 478.

Signature: `flag --- phext::index ---`

What it does: A command line flag, --- phext::index ---. It is read by the program when the user passes it.

(Where these words came from: signature.)

## flag — --coord

Found in src/main.rs at line 532.

Signature: `flag --coord`

What it does: A command line flag, --coord. It is read by the program when the user passes it.

(Where these words came from: signature.)

## flag — --days

Found in src/main.rs at line 509.

Signature: `flag --days`

What it does: A command line flag, --days. It is read by the program when the user passes it.

(Where these words came from: signature.)

## flag — --days must be 3650 or fewer

Found in src/main.rs at line 218.

Signature: `flag --days must be 3650 or fewer`

What it does: A command line flag, --days must be 3650 or fewer. It is read by the program when the user passes it.

(Where these words came from: signature.)

## flag — --details

Found in src/main.rs at line 548.

Signature: `flag --details`

What it does: A command line flag, --details. It is read by the program when the user passes it.

(Where these words came from: signature.)

## flag — --empty

Found in src/main.rs at line 505.

Signature: `flag --empty`

What it does: A command line flag, --empty. It is read by the program when the user passes it.

(Where these words came from: signature.)

## flag — --host

Found in src/main.rs at line 595.

Signature: `flag --host`

What it does: A command line flag, --host. It is read by the program when the user passes it.

(Where these words came from: signature.)

## flag — --index

Found in src/main.rs at line 588.

Signature: `flag --index`

What it does: A command line flag, --index. It is read by the program when the user passes it.

(Where these words came from: signature.)

## flag — --port

Found in src/main.rs at line 595.

Signature: `flag --port`

What it does: A command line flag, --port. It is read by the program when the user passes it.

(Where these words came from: signature.)

## flag — --preview

Found in src/main.rs at line 558.

Signature: `flag --preview`

What it does: A command line flag, --preview. It is read by the program when the user passes it.

(Where these words came from: signature.)

## flag — --scroll

Found in src/main.rs at line 575.

Signature: `flag --scroll`

What it does: A command line flag, --scroll. It is read by the program when the user passes it.

(Where these words came from: signature.)

## flag — --seed

Found in src/main.rs at line 544.

Signature: `flag --seed`

What it does: A command line flag, --seed. It is read by the program when the user passes it.

(Where these words came from: signature.)

## flag — --text

Found in src/main.rs at line 531.

Signature: `flag --text`

What it does: A command line flag, --text. It is read by the program when the user passes it.

(Where these words came from: signature.)

## flag — --user

Found in src/main.rs at line 548.

Signature: `flag --user`

What it does: A command line flag, --user. It is read by the program when the user passes it.

(Where these words came from: signature.)

## flag — --uses

Found in src/main.rs at line 509.

Signature: `flag --uses`

What it does: A command line flag, --uses. It is read by the program when the user passes it.

(Where these words came from: signature.)

## struct — Message

Found in src/model.rs at line 46.

Signature: `pub struct Message {`

What it does: A data shape named Message. Its fields are: pub coord: Coordinate; pub author: String; pub ts: DateTime<Utc>; pub text: String; pub plain: bool.

(Where these words came from: signature.)

## function — Message::poster_key

Found in src/model.rs at line 59.

Signature: `pub fn poster_key(&self) -> &str {`

What it does: Share caps follow the OAuth identity when the scroll has one.

(Where these words came from: doc comment.)

## const — PLAIN_AUTHOR

Found in src/model.rs at line 43.

Signature: `pub const PLAIN_AUTHOR: &str = "phext";`

What it does: Author stamped on a scroll that is not a Lanes message.

(Where these words came from: doc comment.)

## enum — Scroll

Found in src/model.rs at line 80.

Signature: `pub enum Scroll {`

What it does: A named choice named Scroll. It is one of: User; Card.

(Where these words came from: signature.)

## struct — Thread

Found in src/model.rs at line 69.

Signature: `pub struct Thread {`

What it does: A data shape named Thread. Its fields are: pub coord: Coordinate; pub topic: String; pub title: String; pub keywords: String; pub messages: Vec<Message>; pub plain: bool.

(Where these words came from: signature.)

## struct — User

Found in src/model.rs at line 35.

Signature: `pub struct User {`

What it does: A data shape named User. Its fields are: pub id: String; pub name: String; pub blurb: String; pub interests: Vec<Coordinate>.

(Where these words came from: signature.)

## function — User::to_scroll

Found in src/model.rs at line 96.

Signature: `pub fn to_scroll(&self) -> String {`

What it does: Function User::to_scroll takes no inputs beyond the value it is called on, and gives back String.

(Where these words came from: signature.)

## function — content_tokens

Found in src/model.rs at line 314.

Signature: `pub fn content_tokens(text: &str) -> std::collections::BTreeSet<String> {`

What it does: Lowercased content words. A trailing "s" is stripped so "ferns" meets "fern".

(Where these words came from: doc comment.)

## function — format_card

Found in src/model.rs at line 126.

Signature: `pub fn format_card(topic: &str, title: &str, keywords: &str) -> String {`

What it does: Function format_card takes topic: &str, title: &str, keywords: &str and gives back String.

(Where these words came from: signature.)

## function — format_message

Found in src/model.rs at line 135.

Signature: `pub fn format_message(author: &str, ts: DateTime<Utc>, text: &str) -> String {`

What it does: Function format_message takes author: &str, ts: DateTime<Utc>, text: &str and gives back String.

(Where these words came from: signature.)

## function — format_message_as

Found in src/model.rs at line 139.

Signature: `pub fn format_message_as(author: &str, who: &str, ts: DateTime<Utc>, text: &str) -> String {`

What it does: Function format_message_as takes author: &str, who: &str, ts: DateTime<Utc>, text: &str and gives back String.

(Where these words came from: signature.)

## function — keyword_line

Found in src/model.rs at line 209.

Signature: `pub fn keyword_line(text: &str) -> String {`

What it does: Function keyword_line takes text: &str and gives back String.

(Where these words came from: signature.)

## function — member_id

Found in src/model.rs at line 113.

Signature: `pub fn member_id(id: &str) -> String {`

What it does: Roster ids keep a single colon (`github:100`). Other punctuation is dropped.

(Where these words came from: doc comment.)

## function — opening_line

Found in src/model.rs at line 155.

Signature: `pub fn opening_line(text: &str, limit: usize) -> String {`

What it does: First non-empty line, clipped. Empty text becomes "Scroll".

(Where these words came from: doc comment.)

## function — parse_scroll

Found in src/model.rs at line 216.

Signature: `pub fn parse_scroll(raw: &str) -> Option<Scroll> {`

What it does: Function parse_scroll takes raw: &str and gives back Option<Scroll>.

(Where these words came from: signature.)

## function — preview

Found in src/model.rs at line 335.

Signature: `pub fn preview(text: &str, limit: usize) -> String {`

What it does: Function preview takes text: &str, limit: usize and gives back String.

(Where these words came from: signature.)

## function — rest_after_opening

Found in src/model.rs at line 170.

Signature: `pub fn rest_after_opening(text: &str) -> String {`

What it does: The scroll after its first non-empty line.

(Where these words came from: doc comment.)

## function — title_from_text

Found in src/model.rs at line 192.

Signature: `pub fn title_from_text(text: &str) -> String {`

What it does: Function title_from_text takes text: &str and gives back String.

(Where these words came from: signature.)

## struct — Identity

Found in src/oauth.rs at line 28.

Signature: `pub struct Identity {`

What it does: A data shape named Identity. Its fields are: pub who: String; pub handle: String.

(Where these words came from: signature.)

## trait — IdentitySource

Found in src/oauth.rs at line 35.

Signature: `pub trait IdentitySource: Send + Sync {`

What it does: A trait named IdentitySource: a set of methods that a type can promise to give.

(Where these words came from: signature.)

## struct — Oauth

Found in src/oauth.rs at line 53.

Signature: `pub struct Oauth {`

What it does: A data shape named Oauth. The scanner did not read its fields.

(Where these words came from: signature.)

## function — Oauth::from_env

Found in src/oauth.rs at line 163.

Signature: `pub fn from_env() -> Self {`

What it does: Function Oauth::from_env takes no inputs beyond the value it is called on, and gives back Self.

(Where these words came from: signature.)

## struct — Pending

Found in src/oauth.rs at line 98.

Signature: `pub struct Pending {`

What it does: A data shape named Pending. Its fields are: pub provider: String; pub state: String; pub verifier: String; pub next: String.

(Where these words came from: signature.)

## struct — Session

Found in src/oauth.rs at line 69.

Signature: `pub struct Session {`

What it does: A data shape named Session. Its fields are: pub who: String; pub handle: String; pub name: String; pub email: String; pub csrf: String.

(Where these words came from: signature.)

## const — TEST_CSRF

Found in src/oauth.rs at line 23.

Signature: `pub const TEST_CSRF: &str = "csrf-test-token";`

What it does: A fixed value named TEST_CSRF, set to "csrf-test-token".

(Where these words came from: signature.)

## function — authorize_url

Found in src/oauth.rs at line 247.

Signature: `pub fn authorize_url( &self, provider_name: &str, next: &str, ) -> Result<(String, String), String> {`

What it does: Function authorize_url takes &self, provider_name: &str, next: &str, and gives back Result<(String, String), String>.

(Where these words came from: signature.)

## function — clean_email

Found in src/oauth.rs at line 532.

Signature: `pub fn clean_email(raw: &str) -> String {`

What it does: Function clean_email takes raw: &str and gives back String.

(Where these words came from: signature.)

## function — clean_handle

Found in src/oauth.rs at line 553.

Signature: `pub fn clean_handle(raw: &str) -> String {`

What it does: Function clean_handle takes raw: &str and gives back String.

(Where these words came from: signature.)

## function — clear_cookie

Found in src/oauth.rs at line 439.

Signature: `pub fn clear_cookie(name: &str, secure: bool) -> String {`

What it does: Function clear_cookie takes name: &str, secure: bool and gives back String.

(Where these words came from: signature.)

## function — closed

Found in src/oauth.rs at line 192.

Signature: `pub fn closed() -> Self {`

What it does: Function closed takes no inputs beyond the value it is called on, and gives back Self.

(Where these words came from: signature.)

## function — cookie_from_headers

Found in src/oauth.rs at line 622.

Signature: `pub fn cookie_from_headers(headers: &axum::http::HeaderMap) -> String {`

What it does: Function cookie_from_headers takes headers: &axum::http::HeaderMap and gives back String.

(Where these words came from: signature.)

## function — cookie_header

Found in src/oauth.rs at line 426.

Signature: `pub fn cookie_header(name: &str, value: &str, max_age: i64, secure: bool) -> String {`

What it does: Function cookie_header takes name: &str, value: &str, max_age: i64, secure: bool and gives back String.

(Where these words came from: signature.)

## function — cookie_secure

Found in src/oauth.rs at line 243.

Signature: `pub fn cookie_secure(&self) -> bool {`

What it does: Function cookie_secure takes no inputs beyond the value it is called on, and gives back bool.

(Where these words came from: signature.)

## function — csrf_matches

Found in src/oauth.rs at line 355.

Signature: `pub fn csrf_matches(&self, session: &Session, header: &str) -> bool {`

What it does: Function csrf_matches takes &self, session: &Session, header: &str and gives back bool.

(Where these words came from: signature.)

## function — display_name

Found in src/oauth.rs at line 516.

Signature: `pub fn display_name(raw: &str, fallback: &str) -> String {`

What it does: Function display_name takes raw: &str, fallback: &str and gives back String.

(Where these words came from: signature.)

## function — finish

Found in src/oauth.rs at line 291.

Signature: `pub fn finish( &self, provider_name: &str, code: &str, state: &str, pending_cookie: &str, ) -> Result<(Identity, String, String), String> {`

What it does: Function finish takes &self, provider_name: &str, code: &str, state: &str, pending_cookie: &str, and gives back Result<(Identity, String, String), String>.

(Where these words came from: signature.)

## function — pending_cookie

Found in src/oauth.rs at line 435.

Signature: `pub fn pending_cookie(value: &str, secure: bool) -> String {`

What it does: Function pending_cookie takes value: &str, secure: bool and gives back String.

(Where these words came from: signature.)

## function — pending_cookie_name

Found in src/oauth.rs at line 422.

Signature: `pub fn pending_cookie_name() -> &'static str {`

What it does: Function pending_cookie_name takes no inputs beyond the value it is called on, and gives back &'static str.

(Where these words came from: signature.)

## function — pending_token

Found in src/oauth.rs at line 618.

Signature: `pub fn pending_token(cookie_header: &str) -> Option<String> {`

What it does: Function pending_token takes cookie_header: &str and gives back Option<String>.

(Where these words came from: signature.)

## function — providers

Found in src/oauth.rs at line 229.

Signature: `pub fn providers(&self) -> Vec<&'static str> {`

What it does: Function providers takes no inputs beyond the value it is called on, and gives back Vec<&'static str>.

(Where these words came from: signature.)

## function — read_session

Found in src/oauth.rs at line 333.

Signature: `pub fn read_session(&self, cookie_header: &str) -> Option<Session> {`

What it does: Function read_session takes &self, cookie_header: &str and gives back Option<Session>.

(Where these words came from: signature.)

## function — ready

Found in src/oauth.rs at line 223.

Signature: `pub fn ready(&self) -> bool {`

What it does: Function ready takes no inputs beyond the value it is called on, and gives back bool.

(Where these words came from: signature.)

## function — safe_next

Found in src/oauth.rs at line 484.

Signature: `pub fn safe_next(next: &str) -> String {`

What it does: Function safe_next takes next: &str and gives back String.

(Where these words came from: signature.)

## function — session_cookie

Found in src/oauth.rs at line 431.

Signature: `pub fn session_cookie(value: &str, secure: bool) -> String {`

What it does: Function session_cookie takes value: &str, secure: bool and gives back String.

(Where these words came from: signature.)

## function — session_cookie_name

Found in src/oauth.rs at line 418.

Signature: `pub fn session_cookie_name() -> &'static str {`

What it does: Function session_cookie_name takes no inputs beyond the value it is called on, and gives back &'static str.

(Where these words came from: signature.)

## function — test_ready

Found in src/oauth.rs at line 179.

Signature: `pub fn test_ready() -> Self {`

What it does: Function test_ready takes no inputs beyond the value it is called on, and gives back Self.

(Where these words came from: signature.)

## function — test_session_cookie

Found in src/oauth.rs at line 359.

Signature: `pub fn test_session_cookie() -> String {`

What it does: Function test_session_cookie takes no inputs beyond the value it is called on, and gives back String.

(Where these words came from: signature.)

## const — AUTHOR_CAP

Found in src/rank.rs at line 19.

Signature: `pub const AUTHOR_CAP: usize = 2;`

What it does: A fixed value named AUTHOR_CAP, set to 2.

(Where these words came from: signature.)

## struct — Feed

Found in src/rank.rs at line 28.

Signature: `pub struct Feed {`

What it does: A data shape named Feed. Its fields are: pub threads: Vec<FeedThread>; pub lanes: Vec<Lane>.

(Where these words came from: signature.)

## struct — FeedThread

Found in src/rank.rs at line 34.

Signature: `pub struct FeedThread {`

What it does: A data shape named FeedThread. Its fields are: pub coord: Coordinate; pub topic: String; pub title: String; pub distance: u64; pub score: f64; pub nearness: &'static str; pub admitted: Vec<Message>; pub held: Vec<Message>.

(Where these words came from: signature.)

## const — LOUD_AT

Found in src/rank.rs at line 22.

Signature: `pub const LOUD_AT: usize = 8;`

What it does: Authors with at least this many messages are loud. Only they are capped. A reader with a handful of notes is left intact.

(Where these words came from: doc comment.)

## struct — Lane

Found in src/rank.rs at line 46.

Signature: `pub struct Lane {`

What it does: A data shape named Lane. Its fields are: pub author: String; pub held: usize; pub shown: usize; pub threads: Vec<String>.

(Where these words came from: signature.)

## const — MAX_THREADS

Found in src/rank.rs at line 23.

Signature: `pub const MAX_THREADS: usize = 8;`

What it does: A fixed value named MAX_THREADS, set to 8.

(Where these words came from: signature.)

## struct — MapEntry

Found in src/rank.rs at line 65.

Signature: `pub struct MapEntry {`

What it does: A data shape named MapEntry. Its fields are: pub coord: Coordinate; pub topic: String; pub title: String; pub distance: u64; pub message_count: usize; pub latest_author: Option<String>; pub latest_ts: Option<DateTime<Utc>>; pub preview: String; pub majority_author: Option<String>; pub majority_count: usize; pub majority_share: f64; pub flooded: bool.

(Where these words came from: signature.)

## struct — Suggestion

Found in src/rank.rs at line 54.

Signature: `pub struct Suggestion {`

What it does: A data shape named Suggestion. Its fields are: pub coord: Coordinate; pub thread: Coordinate; pub title: String; pub topic: String; pub new_thread: bool; pub overlap: Vec<String>; pub reason: String.

(Where these words came from: signature.)

## function — admitted_by

Found in src/rank.rs at line 498.

Signature: `pub fn admitted_by(feed: &Feed, author: &str) -> usize {`

What it does: Function admitted_by takes feed: &Feed, author: &str and gives back usize.

(Where these words came from: signature.)

## function — age_secs

Found in src/rank.rs at line 96.

Signature: `pub fn age_secs(ts: DateTime<Utc>, now: DateTime<Utc>) -> u64 {`

What it does: Function age_secs takes ts: DateTime<Utc>, now: DateTime<Utc> and gives back u64.

(Where these words came from: signature.)

## function — build_feed

Found in src/rank.rs at line 113.

Signature: `pub fn build_feed( threads: &[Thread], interests: &[Coordinate], now: DateTime<Utc>, cap: usize, max_threads: usize, ) -> Feed {`

What it does: Function build_feed takes threads: &[Thread], interests: &[Coordinate], now: DateTime<Utc>, cap: usize, max_threads: usize, and gives back Feed.

(Where these words came from: signature.)

## function — fresh_thread

Found in src/rank.rs at line 443.

Signature: `pub fn fresh_thread(threads: &[Thread], home: Coordinate) -> Coordinate {`

What it does: Function fresh_thread takes threads: &[Thread], home: Coordinate and gives back Coordinate.

(Where these words came from: signature.)

## function — is_flooded

Found in src/rank.rs at line 272.

Signature: `pub fn is_flooded(count: usize, share: f64) -> bool {`

What it does: Function is_flooded takes count: usize, share: f64 and gives back bool.

(Where these words came from: signature.)

## function — majority

Found in src/rank.rs at line 256.

Signature: `pub fn majority(messages: &[Message]) -> (Option<String>, usize, f64) {`

What it does: Function majority takes messages: &[Message] and gives back (Option<String>, usize, f64).

(Where these words came from: signature.)

## function — map_of

Found in src/rank.rs at line 228.

Signature: `pub fn map_of(threads: &[Thread], interests: &[Coordinate]) -> Vec<MapEntry> {`

What it does: Function map_of takes threads: &[Thread], interests: &[Coordinate] and gives back Vec<MapEntry>.

(Where these words came from: signature.)

## function — nearness

Found in src/rank.rs at line 87.

Signature: `pub fn nearness(distance: u64) -> &'static str {`

What it does: Function nearness takes distance: u64 and gives back &'static str.

(Where these words came from: signature.)

## function — suggest

Found in src/rank.rs at line 299.

Signature: `pub fn suggest( threads: &[Thread], interests: &[Coordinate], text: &str, now: DateTime<Utc>, ) -> Suggestion {`

What it does: Function suggest takes threads: &[Thread], interests: &[Coordinate], text: &str, now: DateTime<Utc>, and gives back Suggestion.

(Where these words came from: signature.)

## function — suggest_fresh

Found in src/rank.rs at line 278.

Signature: `pub fn suggest_fresh( threads: &[Thread], interests: &[Coordinate], text: &str, _now: DateTime<Utc>, ) -> Suggestion {`

What it does: A new book on the reader's home shelf. Same placement as a suggestion that shared no words. Used when someone explicitly starts a thread.

(Where these words came from: doc comment.)

## function — thread_score

Found in src/rank.rs at line 80.

Signature: `pub fn thread_score(distance: u64, age_secs: u64) -> f64 {`

What it does: Function thread_score takes distance: u64, age_secs: u64 and gives back f64.

(Where these words came from: signature.)

## const — ADA

Found in src/seed.rs at line 14.

Signature: `pub const ADA: &str = "Ada North";`

What it does: A fixed value named ADA, set to "Ada North".

(Where these words came from: signature.)

## const — BREAD

Found in src/seed.rs at line 23.

Signature: `pub const BREAD: &str = "Bread hour";`

What it does: A fixed value named BREAD, set to "Bread hour".

(Where these words came from: signature.)

## const — BREAD_THREAD

Found in src/seed.rs at line 30.

Signature: `pub const BREAD_THREAD: &str = "4.1.1/1.1.1";`

What it does: A fixed value named BREAD_THREAD, set to "4.1.1/1.1.1".

(Where these words came from: signature.)

## const — CASEY

Found in src/seed.rs at line 16.

Signature: `pub const CASEY: &str = "Casey Horn";`

What it does: A fixed value named CASEY, set to "Casey Horn".

(Where these words came from: signature.)

## const — FERN

Found in src/seed.rs at line 19.

Signature: `pub const FERN: &str = "Fern wall";`

What it does: A fixed value named FERN, set to "Fern wall".

(Where these words came from: signature.)

## const — FERN_THREAD

Found in src/seed.rs at line 26.

Signature: `pub const FERN_THREAD: &str = "1.2.1/1.1.1";`

What it does: A fixed value named FERN_THREAD, set to "1.2.1/1.1.1".

(Where these words came from: signature.)

## const — HALL

Found in src/seed.rs at line 18.

Signature: `pub const HALL: &str = "General";`

What it does: A fixed value named HALL, set to "General".

(Where these words came from: signature.)

## const — HALL_THREAD

Found in src/seed.rs at line 25.

Signature: `pub const HALL_THREAD: &str = "1.1.1/1.1.1";`

What it does: A fixed value named HALL_THREAD, set to "1.1.1/1.1.1".

(Where these words came from: signature.)

## const — JULES

Found in src/seed.rs at line 13.

Signature: `pub const JULES: &str = "Jules Fen";`

What it does: A fixed value named JULES, set to "Jules Fen".

(Where these words came from: signature.)

## const — MIRA

Found in src/seed.rs at line 12.

Signature: `pub const MIRA: &str = "Mira Quill";`

What it does: A fixed value named MIRA, set to "Mira Quill".

(Where these words came from: signature.)

## const — PLANES

Found in src/seed.rs at line 21.

Signature: `pub const PLANES: &str = "Hand planes";`

What it does: A fixed value named PLANES, set to "Hand planes".

(Where these words came from: signature.)

## const — PLANES_THREAD

Found in src/seed.rs at line 28.

Signature: `pub const PLANES_THREAD: &str = "2.1.1/1.1.1";`

What it does: A fixed value named PLANES_THREAD, set to "2.1.1/1.1.1".

(Where these words came from: signature.)

## const — REN

Found in src/seed.rs at line 15.

Signature: `pub const REN: &str = "Ren Sato";`

What it does: A fixed value named REN, set to "Ren Sato".

(Where these words came from: signature.)

## const — RIVER

Found in src/seed.rs at line 22.

Signature: `pub const RIVER: &str = "River wards";`

What it does: A fixed value named RIVER, set to "River wards".

(Where these words came from: signature.)

## const — RIVER_THREAD

Found in src/seed.rs at line 29.

Signature: `pub const RIVER_THREAD: &str = "3.1.1/1.1.1";`

What it does: A fixed value named RIVER_THREAD, set to "3.1.1/1.1.1".

(Where these words came from: signature.)

## const — SWAP

Found in src/seed.rs at line 20.

Signature: `pub const SWAP: &str = "Seed swap";`

What it does: A fixed value named SWAP, set to "Seed swap".

(Where these words came from: signature.)

## const — SWAP_THREAD

Found in src/seed.rs at line 27.

Signature: `pub const SWAP_THREAD: &str = "1.2.1/1.2.1";`

What it does: A fixed value named SWAP_THREAD, set to "1.2.1/1.2.1".

(Where these words came from: signature.)

## function — blank

Found in src/seed.rs at line 197.

Signature: `pub fn blank() -> String {`

What it does: A new chat: one General thread, no readers, no messages.  This does not include the fictional cast. [`document`] does.

(Where these words came from: doc comment.)

## function — demo_instant

Found in src/seed.rs at line 174.

Signature: `pub fn demo_instant() -> DateTime<Utc> {`

What it does: Function demo_instant takes no inputs beyond the value it is called on, and gives back DateTime<Utc>.

(Where these words came from: signature.)

## function — document

Found in src/seed.rs at line 207.

Signature: `pub fn document(now: DateTime<Utc>) -> String {`

What it does: Build the demo phext by inserting each scroll in coordinate order.

(Where these words came from: doc comment.)

## function — readers

Found in src/seed.rs at line 178.

Signature: `pub fn readers() -> Vec<User> {`

What it does: Function readers takes no inputs beyond the value it is called on, and gives back Vec<User>.

(Where these words came from: signature.)

## struct — AppState

Found in src/server.rs at line 34.

Signature: `pub struct AppState {`

What it does: A data shape named AppState. The scanner did not read its fields.

(Where these words came from: signature.)

## function — AppState::open

Found in src/server.rs at line 44.

Signature: `pub fn open(path: std::path::PathBuf) -> Result<Self, Error> {`

What it does: Function AppState::open takes path: std::path::PathBuf and gives back Result<Self, Error>.

(Where these words came from: signature.)

## function — listen_addr

Found in src/server.rs at line 109.

Signature: `pub fn listen_addr(host: &str, port: u16) -> String {`

What it does: `host:port`, with brackets when `host` is an IPv6 address.

(Where these words came from: doc comment.)

## function — router

Found in src/server.rs at line 156.

Signature: `pub fn router(state: AppState) -> Router {`

What it does: Function router takes state: AppState and gives back Router.

(Where these words came from: signature.)

## function — serve

Found in src/server.rs at line 117.

Signature: `pub async fn serve(path: std::path::PathBuf, host: &str, port: u16) -> Result<(), Error> {`

What it does: Function serve takes path: std::path::PathBuf, host: &str, port: u16 and gives back Result<(), Error>.

(Where these words came from: signature.)

## function — with_auth

Found in src/server.rs at line 57.

Signature: `pub fn with_auth( path: std::path::PathBuf, auth: Arc<dyn PlotHolderCheck>, ) -> Result<Self, Error> {`

What it does: Function with_auth takes path: std::path::PathBuf, auth: Arc<dyn PlotHolderCheck>, and gives back Result<Self, Error>.

(Where these words came from: signature.)

## struct — Placed

Found in src/store.rs at line 146.

Signature: `pub struct Placed {`

What it does: A data shape named Placed. Its fields are: pub coord: Coordinate; pub thread: Coordinate; pub title: String; pub topic: String; pub new_thread: bool.

(Where these words came from: signature.)

## struct — Post

Found in src/store.rs at line 137.

Signature: `pub struct Post {`

What it does: A data shape named Post. Its fields are: pub author: String; pub who: String; pub text: String; pub target: Target; pub now: DateTime<Utc>.

(Where these words came from: signature.)

## struct — Store

Found in src/store.rs at line 154.

Signature: `pub struct Store {`

What it does: A data shape named Store. The scanner did not read its fields.

(Where these words came from: signature.)

## function — Store::open

Found in src/store.rs at line 164.

Signature: `pub fn open(path: impl Into<PathBuf>) -> Result<Self, Error> {`

What it does: Function Store::open takes path: impl Into<PathBuf> and gives back Result<Self, Error>.

(Where these words came from: signature.)

## struct — World

Found in src/store.rs at line 23.

Signature: `pub struct World {`

What it does: A data shape named World. Its fields are: pub users: Vec<User>; pub threads: Vec<Thread>.

(Where these words came from: signature.)

## function — World::find_user

Found in src/store.rs at line 29.

Signature: `pub fn find_user(&self, query: &str) -> Option<&User> {`

What it does: Function World::find_user takes &self, query: &str and gives back Option<&User>.

(Where these words came from: signature.)

## function — byte_index

Found in src/store.rs at line 222.

Signature: `pub fn byte_index(&self) -> String {`

What it does: Offsets from `phext::index`, one coordinate per line.

(Where these words came from: doc comment.)

## function — doc

Found in src/store.rs at line 181.

Signature: `pub fn doc(&self) -> &str {`

What it does: Function doc takes no inputs beyond the value it is called on, and gives back &str.

(Where these words came from: signature.)

## function — ensure_member

Found in src/store.rs at line 261.

Signature: `pub fn ensure_member(&mut self, id: &str, name: &str) -> Result<bool, Error> {`

What it does: Add a reader, or refresh their display name. Interests start empty.  Returns true when the roster scroll was created.

(Where these words came from: doc comment.)

## function — find_lane

Found in src/store.rs at line 56.

Signature: `pub fn find_lane(&self, id: &str) -> Option<&Thread> {`

What it does: Function find_lane takes &self, id: &str and gives back Option<&Thread>.

(Where these words came from: signature.)

## function — find_post

Found in src/store.rs at line 81.

Signature: `pub fn find_post(&self, id: &str) -> Option<(&Thread, &Message)> {`

What it does: The message scroll named by a post id. The thread card is not a post.

(Where these words came from: doc comment.)

## function — find_thread

Found in src/store.rs at line 93.

Signature: `pub fn find_thread(&self, query: &str) -> Result<&Thread, String> {`

What it does: A thread title, or a thread or scroll coordinate.

(Where these words came from: doc comment.)

## function — from_doc

Found in src/store.rs at line 196.

Signature: `pub fn from_doc(path: impl Into<PathBuf>, doc: String) -> Self {`

What it does: Function from_doc takes path: impl Into<PathBuf>, doc: String and gives back Self.

(Where these words came from: signature.)

## function — init_empty

Found in src/store.rs at line 246.

Signature: `pub fn init_empty(path: impl AsRef<Path>) -> Result<(), Error> {`

What it does: Create a chat with a General thread and no demo cast.  A file that already has content is left untouched. `--seed` is the only path that replaces an existing chat, and it wipes that file.

(Where these words came from: doc comment.)

## function — load_doc

Found in src/store.rs at line 186.

Signature: `pub fn load_doc(path: impl Into<PathBuf>) -> Result<Self, Error> {`

What it does: Load a fabric that already exists. Do not seed over it.

(Where these words came from: doc comment.)

## function — note_interest

Found in src/store.rs at line 305.

Signature: `pub fn note_interest(&mut self, id: &str, thread: Coordinate) -> Result<(), Error> {`

What it does: Remember a thread this member posted in or opened.  The feed still uses the existing distance ranking. No interests means every thread is equally near, so a new member sees recency until this list grows. The first thread stays their home shelf.

(Where these words came from: doc comment.)

## function — path

Found in src/store.rs at line 177.

Signature: `pub fn path(&self) -> &Path {`

What it does: Function path takes no inputs beyond the value it is called on, and gives back &Path.

(Where these words came from: signature.)

## function — place

Found in src/store.rs at line 331.

Signature: `pub fn place(&mut self, post: Post) -> Result<Placed, Error> {`

What it does: Function place takes &mut self, post: Post and gives back Result<Placed, Error>.

(Where these words came from: signature.)

## function — reseed

Found in src/store.rs at line 238.

Signature: `pub fn reseed(&mut self, now: DateTime<Utc>) -> Result<(), Error> {`

What it does: Function reseed takes &mut self, now: DateTime<Utc> and gives back Result<(), Error>.

(Where these words came from: signature.)

## function — select

Found in src/store.rs at line 212.

Signature: `pub fn select(&self, coord: Coordinate) -> String {`

What it does: `phext::fetch`

(Where these words came from: doc comment.)

## function — thread

Found in src/store.rs at line 45.

Signature: `pub fn thread(&self, key: &str) -> Option<&Thread> {`

What it does: Function thread takes &self, key: &str and gives back Option<&Thread>.

(Where these words came from: signature.)

## function — toc

Found in src/store.rs at line 217.

Signature: `pub fn toc(&self) -> String {`

What it does: `phext::textmap`

(Where these words came from: doc comment.)

## function — world

Found in src/store.rs at line 207.

Signature: `pub fn world(&self) -> &World {`

What it does: The parsed fabric. Rebuilt only when the file changes.

(Where these words came from: doc comment.)

## function — world_from

Found in src/store.rs at line 478.

Signature: `pub fn world_from(doc: &str) -> World {`

What it does: Function world_from takes doc: &str and gives back World.

(Where these words came from: signature.)

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/phext-lanes/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

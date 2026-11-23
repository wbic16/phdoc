# phext-lanes — reference as of commit 820e5cb8ecfa

Commit 820e5cb8ecfa9693f71de081f000508fb5ece960. 2 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

### Cargo.toml

- axum — =0.7.9
- chrono — =0.4.39
- clap — =4.5.23
- hyper — =1.5.2
- hyper-util — =0.1.10
- libphext — =0.3.1
- serde — =1.0.217
- serde_json — =1.0.134
- tokio — =1.42.0

## Authors

- Cursor Agent <cursoragent@cursor.com> — 1 commits
- Will Bickford <wbic16@gmail.com> — 1 commits

## Complexity

- Markdown: 1 files, 175 lines total, 53 blank, 0 comment, 122 code
- Rust: 9 files, 2984 lines total, 216 blank, 95 comment, 2673 code
- TOML: 1 files, 19 lines total, 1 blank, 1 comment, 17 code

Declared RFCs at this point: none declared.

## Public methods and commands

- const `ROSTER_LIBRARY` (src/coord.rs) — Library reserved for reader records.
- enum `Target` (src/coord.rs) — A named choice named Target.
- const `WEIGHTS` (src/coord.rs) — Library, shelf, series, collection, volume, book.
- function `arms` (src/coord.rs) — Function arms takes coord: Coordinate.
- function `format_coord` (src/coord.rs) — Function format_coord takes coord: Coordinate.
- function `is_card_slot` (src/coord.rs) — Function is_card_slot takes coord: Coordinate.
- function `is_roster` (src/coord.rs) — Function is_roster takes coord: Coordinate.
- function `min_distance` (src/coord.rs) — Function min_distance takes thread: Coordinate, interests: &[Coordinate].
- function `next_message_scroll` (src/coord.rs) — Smallest free message scroll in chapter 1, section 1.
- function `one_line` (src/coord.rs) — Function one_line takes text: &str.
- function `parse_coord` (src/coord.rs) — Function parse_coord takes input: &str.
- function `parse_target` (src/coord.rs) — Function parse_target takes input: &str.
- function `sanitize` (src/coord.rs) — Strip dimension breaks so a message cannot split the phext.
- function `thread_anchor` (src/coord.rs) — Function thread_anchor takes coord: Coordinate.
- function `thread_key` (src/coord.rs) — Function thread_key takes coord: Coordinate.
- function `topic_key` (src/coord.rs) — Function topic_key takes coord: Coordinate.
- function `weighted_distance` (src/coord.rs) — Function weighted_distance takes a: Coordinate, b: Coordinate.
- function `with_scroll` (src/coord.rs) — Function with_scroll takes thread: Coordinate, scroll: usize.
- enum `Error` (src/error.rs) — A named choice named Error.
- re-export `error::Error` (src/lib.rs) — A re-export: error::Error is made reachable through this module, so other code can use it from here.
- struct `Message` (src/model.rs) — A data shape named Message.
- enum `Scroll` (src/model.rs) — A named choice named Scroll.
- struct `Thread` (src/model.rs) — A data shape named Thread.
- struct `User` (src/model.rs) — A data shape named User.
- function `User::to_scroll` (src/model.rs) — Function User::to_scroll takes no inputs beyond the value it is called on.
- function `content_tokens` (src/model.rs) — Lowercased content words.
- function `format_card` (src/model.rs) — Function format_card takes topic: &str, title: &str, keywords: &str.
- function `format_message` (src/model.rs) — Function format_message takes author: &str, ts: DateTime<Utc>, text: &str.
- function `keyword_line` (src/model.rs) — Function keyword_line takes text: &str.
- function `parse_scroll` (src/model.rs) — Function parse_scroll takes raw: &str.
- function `preview` (src/model.rs) — Function preview takes text: &str, limit: usize.
- function `title_from_text` (src/model.rs) — Function title_from_text takes text: &str.
- const `AUTHOR_CAP` (src/rank.rs) — A fixed value named AUTHOR_CAP, set to 2.
- struct `Feed` (src/rank.rs) — A data shape named Feed.
- struct `FeedThread` (src/rank.rs) — A data shape named FeedThread.
- const `LOUD_AT` (src/rank.rs) — Authors with at least this many messages are loud.
- struct `Lane` (src/rank.rs) — A data shape named Lane.
- const `MAX_THREADS` (src/rank.rs) — A fixed value named MAX_THREADS, set to 8.
- struct `MapEntry` (src/rank.rs) — A data shape named MapEntry.
- struct `Suggestion` (src/rank.rs) — A data shape named Suggestion.
- function `admitted_by` (src/rank.rs) — Function admitted_by takes feed: &Feed, author: &str.
- function `age_secs` (src/rank.rs) — Function age_secs takes ts: DateTime<Utc>, now: DateTime<Utc>.
- function `build_feed` (src/rank.rs) — Function build_feed takes threads: &[Thread], interests: &[Coordinate], now: DateTime<Utc>, cap: usize, max_threads: usize,.
- function `fresh_thread` (src/rank.rs) — Function fresh_thread takes threads: &[Thread], home: Coordinate.
- function `is_flooded` (src/rank.rs) — Function is_flooded takes count: usize, share: f64.
- function `majority` (src/rank.rs) — Function majority takes messages: &[Message].
- function `map_of` (src/rank.rs) — Function map_of takes threads: &[Thread], interests: &[Coordinate].
- function `nearness` (src/rank.rs) — Function nearness takes distance: u64.
- function `suggest` (src/rank.rs) — Function suggest takes threads: &[Thread], interests: &[Coordinate], text: &str, now: DateTime<Utc>,.
- function `thread_score` (src/rank.rs) — Function thread_score takes distance: u64, age_secs: u64.
- const `ADA` (src/seed.rs) — A fixed value named ADA, set to "Ada North".
- const `BREAD` (src/seed.rs) — A fixed value named BREAD, set to "Bread hour".
- const `BREAD_THREAD` (src/seed.rs) — A fixed value named BREAD_THREAD, set to "4.1.1/1.1.1".
- const `CASEY` (src/seed.rs) — A fixed value named CASEY, set to "Casey Horn".
- const `FERN` (src/seed.rs) — A fixed value named FERN, set to "Fern wall".
- const `FERN_THREAD` (src/seed.rs) — A fixed value named FERN_THREAD, set to "1.2.1/1.1.1".
- const `HALL` (src/seed.rs) — A fixed value named HALL, set to "General".
- const `HALL_THREAD` (src/seed.rs) — A fixed value named HALL_THREAD, set to "1.1.1/1.1.1".
- const `JULES` (src/seed.rs) — A fixed value named JULES, set to "Jules Fen".
- const `MIRA` (src/seed.rs) — A fixed value named MIRA, set to "Mira Quill".
- const `PLANES` (src/seed.rs) — A fixed value named PLANES, set to "Hand planes".
- const `PLANES_THREAD` (src/seed.rs) — A fixed value named PLANES_THREAD, set to "2.1.1/1.1.1".
- const `REN` (src/seed.rs) — A fixed value named REN, set to "Ren Sato".
- const `RIVER` (src/seed.rs) — A fixed value named RIVER, set to "River wards".
- const `RIVER_THREAD` (src/seed.rs) — A fixed value named RIVER_THREAD, set to "3.1.1/1.1.1".
- const `SWAP` (src/seed.rs) — A fixed value named SWAP, set to "Seed swap".
- const `SWAP_THREAD` (src/seed.rs) — A fixed value named SWAP_THREAD, set to "1.2.1/1.2.1".
- function `demo_instant` (src/seed.rs) — Function demo_instant takes no inputs beyond the value it is called on.
- function `document` (src/seed.rs) — Build the demo phext by inserting each scroll in coordinate order.
- function `readers` (src/seed.rs) — Function readers takes no inputs beyond the value it is called on.
- struct `AppState` (src/server.rs) — 
- function `router` (src/server.rs) — Function router takes state: AppState.
- function `serve` (src/server.rs) — Function serve takes path: std::path::PathBuf, port: u16.
- struct `Placed` (src/store.rs) — A data shape named Placed.
- struct `Post` (src/store.rs) — A data shape named Post.
- struct `Store` (src/store.rs) — 
- function `Store::byte_index` (src/store.rs) — Offsets from `phext::index`, one coordinate per line.
- function `Store::doc` (src/store.rs) — Function Store::doc takes no inputs beyond the value it is called on.
- function `Store::open` (src/store.rs) — Function Store::open takes path: impl Into<PathBuf>.
- function `Store::path` (src/store.rs) — Function Store::path takes no inputs beyond the value it is called on.
- function `Store::place` (src/store.rs) — Function Store::place takes &mut self, post: Post.
- function `Store::reseed` (src/store.rs) — Function Store::reseed takes &mut self, now: DateTime<Utc>.
- function `Store::select` (src/store.rs) — `phext::fetch`
- function `Store::toc` (src/store.rs) — `phext::textmap`
- function `Store::world` (src/store.rs) — Function Store::world takes no inputs beyond the value it is called on.
- struct `World` (src/store.rs) — A data shape named World.
- function `World::find_user` (src/store.rs) — Function World::find_user takes &self, query: &str.
- function `World::thread` (src/store.rs) — Function World::thread takes &self, key: &str.
- function `world_from` (src/store.rs) — Function world_from takes doc: &str.

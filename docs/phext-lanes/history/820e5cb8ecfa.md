# phext-lanes — reference as of commit 820e5cb8ecfa

Commit 820e5cb8ecfa9693f71de081f000508fb5ece960. 2 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 9 (9 default, 0 optional).

- axum [dependencies] — =0.7.9
- chrono [dependencies] — =0.4.39
- clap [dependencies] — =4.5.23
- hyper [dependencies] — =1.5.2
- hyper-util [dependencies] — =0.1.10
- libphext [dependencies] — =0.3.1
- serde [dependencies] — =1.0.217
- serde_json [dependencies] — =1.0.134
- tokio [dependencies] — =1.42.0

Total packages in the lock file, this one included: 110.

## Authors

Author counts over the 2 audited mainline commits folded to this point.

The author of record is **Will Bickford** — 1 commit folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 1 commit

Other authors, kept as themselves (not Exocortical cluster personas; never folded):

- Cursor Agent <cursoragent@cursor.com> — 1 commit

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 1 files, 175 lines total, 0 blank, 0 comment, 175 code
- Rust: 9 files, 2984 lines total, 0 blank, 0 comment, 2984 code
- TOML: 1 files, 19 lines total, 0 blank, 0 comment, 19 code

All languages: 11 files, 3178 lines total, 3178 lines of code. Functions read: 124.

Largest files:


Largest functions, by span in lines:


## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 21 test functions counted in the source, across 0 files in test directories or whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 10 of 90 (11%).

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/phext-lanes.git
- Branch: exo
- Head commit: 820e5cb8ecfa9693f71de081f000508fb5ece960
- Working tree: the committed tree at this commit — a historical reference has no working tree

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- const `ROSTER_LIBRARY` (src/coord.rs:12) — Library reserved for reader records.
- enum `Target` (src/coord.rs:31) — A named choice named Target.
- const `WEIGHTS` (src/coord.rs:17) — Library, shelf, series, collection, volume, book.
- function `arms` (src/coord.rs:86) — Function arms takes coord: Coordinate and gives back [usize; 6].
- function `format_coord` (src/coord.rs:52) — Function format_coord takes coord: Coordinate and gives back String.
- function `is_card_slot` (src/coord.rs:82) — Function is_card_slot takes coord: Coordinate and gives back bool.
- function `is_roster` (src/coord.rs:78) — Function is_roster takes coord: Coordinate and gives back bool.
- function `min_distance` (src/coord.rs:106) — Function min_distance takes thread: Coordinate, interests: &[Coordinate] and gives back u64.
- function `next_message_scroll` (src/coord.rs:116) — Smallest free message scroll in chapter 1, section 1.
- function `one_line` (src/coord.rs:151) — Function one_line takes text: &str and gives back String.
- function `parse_coord` (src/coord.rs:19) — Function parse_coord takes input: &str and gives back Result<Coordinate, String>.
- function `parse_target` (src/coord.rs:39) — Function parse_target takes input: &str and gives back Result<Target, String>.
- function `sanitize` (src/coord.rs:131) — Strip dimension breaks so a message cannot split the phext.
- function `thread_anchor` (src/coord.rs:64) — Function thread_anchor takes coord: Coordinate and gives back Coordinate.
- function `thread_key` (src/coord.rs:60) — Function thread_key takes coord: Coordinate and gives back String.
- function `topic_key` (src/coord.rs:56) — Function topic_key takes coord: Coordinate and gives back String.
- function `weighted_distance` (src/coord.rs:97) — Function weighted_distance takes a: Coordinate, b: Coordinate and gives back u64.
- function `with_scroll` (src/coord.rs:72) — Function with_scroll takes thread: Coordinate, scroll: usize and gives back Coordinate.
- enum `Error` (src/error.rs:4) — A named choice named Error.
- re-export `error::Error` (src/lib.rs:17) — A re-export: error::Error is made reachable through this module, so other code can use it from here.
- flag `--- phext::index ---` (src/main.rs:319) — A command line flag, --- phext::index ---.
- struct `Message` (src/model.rs:43) — A data shape named Message.
- enum `Scroll` (src/model.rs:60) — A named choice named Scroll.
- struct `Thread` (src/model.rs:51) — A data shape named Thread.
- struct `User` (src/model.rs:35) — A data shape named User.
- function `User::to_scroll` (src/model.rs:75) — Function User::to_scroll takes no inputs beyond the value it is called on, and gives back String.
- function `content_tokens` (src/model.rs:219) — Lowercased content words.
- function `format_card` (src/model.rs:91) — Function format_card takes topic: &str, title: &str, keywords: &str and gives back String.
- function `format_message` (src/model.rs:100) — Function format_message takes author: &str, ts: DateTime<Utc>, text: &str and gives back String.
- function `keyword_line` (src/model.rs:126) — Function keyword_line takes text: &str and gives back String.
- function `parse_scroll` (src/model.rs:133) — Function parse_scroll takes raw: &str and gives back Option<Scroll>.
- function `preview` (src/model.rs:240) — Function preview takes text: &str, limit: usize and gives back String.
- function `title_from_text` (src/model.rs:109) — Function title_from_text takes text: &str and gives back String.
- const `AUTHOR_CAP` (src/rank.rs:19) — A fixed value named AUTHOR_CAP, set to 2.
- struct `Feed` (src/rank.rs:28) — A data shape named Feed.
- struct `FeedThread` (src/rank.rs:34) — A data shape named FeedThread.
- const `LOUD_AT` (src/rank.rs:22) — Authors with at least this many messages are loud.
- struct `Lane` (src/rank.rs:46) — A data shape named Lane.
- const `MAX_THREADS` (src/rank.rs:23) — A fixed value named MAX_THREADS, set to 8.
- struct `MapEntry` (src/rank.rs:65) — A data shape named MapEntry.
- struct `Suggestion` (src/rank.rs:54) — A data shape named Suggestion.
- function `admitted_by` (src/rank.rs:460) — Function admitted_by takes feed: &Feed, author: &str and gives back usize.
- function `age_secs` (src/rank.rs:96) — Function age_secs takes ts: DateTime<Utc>, now: DateTime<Utc> and gives back u64.
- function `build_feed` (src/rank.rs:113) — Function build_feed takes threads: &[Thread], interests: &[Coordinate], now: DateTime<Utc>, cap: usize, max_threads: usi
- function `fresh_thread` (src/rank.rs:405) — Function fresh_thread takes threads: &[Thread], home: Coordinate and gives back Coordinate.
- function `is_flooded` (src/rank.rs:262) — Function is_flooded takes count: usize, share: f64 and gives back bool.
- function `majority` (src/rank.rs:246) — Function majority takes messages: &[Message] and gives back (Option<String>, usize, f64).
- function `map_of` (src/rank.rs:218) — Function map_of takes threads: &[Thread], interests: &[Coordinate] and gives back Vec<MapEntry>.
- function `nearness` (src/rank.rs:87) — Function nearness takes distance: u64 and gives back &'static str.
- function `suggest` (src/rank.rs:266) — Function suggest takes threads: &[Thread], interests: &[Coordinate], text: &str, now: DateTime<Utc>, and gives back Sugg
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
- function `demo_instant` (src/seed.rs:174) — Function demo_instant takes no inputs beyond the value it is called on, and gives back DateTime<Utc>.
- function `document` (src/seed.rs:195) — Build the demo phext by inserting each scroll in coordinate order.
- function `readers` (src/seed.rs:178) — Function readers takes no inputs beyond the value it is called on, and gives back Vec<User>.
- struct `AppState` (src/server.rs:22) — A data shape named AppState.
- function `router` (src/server.rs:45) — Function router takes state: AppState and gives back Router.
- function `serve` (src/server.rs:26) — Function serve takes path: std::path::PathBuf, port: u16 and gives back Result<(), Error>.
- struct `Placed` (src/store.rs:51) — A data shape named Placed.
- struct `Post` (src/store.rs:43) — A data shape named Post.
- struct `Store` (src/store.rs:59) — A data shape named Store.
- function `Store::open` (src/store.rs:65) — Function Store::open takes path: impl Into<PathBuf> and gives back Result<Self, Error>.
- struct `World` (src/store.rs:22) — A data shape named World.
- function `World::find_user` (src/store.rs:28) — Function World::find_user takes &self, query: &str and gives back Option<&User>.
- function `byte_index` (src/store.rs:104) — Offsets from `phext::index`, one coordinate per line.
- function `doc` (src/store.rs:85) — Function doc takes no inputs beyond the value it is called on, and gives back &str.
- function `path` (src/store.rs:81) — Function path takes no inputs beyond the value it is called on, and gives back &Path.
- function `place` (src/store.rs:124) — Function place takes &mut self, post: Post and gives back Result<Placed, Error>.
- function `reseed` (src/store.rs:120) — Function reseed takes &mut self, now: DateTime<Utc> and gives back Result<(), Error>.
- function `select` (src/store.rs:94) — `phext::fetch`
- function `thread` (src/store.rs:36) — Function thread takes &self, key: &str and gives back Option<&Thread>.
- function `toc` (src/store.rs:99) — `phext::textmap`
- function `world` (src/store.rs:89) — Function world takes no inputs beyond the value it is called on, and gives back World.
- function `world_from` (src/store.rs:217) — Function world_from takes doc: &str and gives back World.

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

Found in src/coord.rs at line 86.

Signature: `pub fn arms(coord: Coordinate) -> [usize; 6] {`

What it does: Function arms takes coord: Coordinate and gives back [usize; 6].

(Where these words came from: signature.)

## function — format_coord

Found in src/coord.rs at line 52.

Signature: `pub fn format_coord(coord: Coordinate) -> String {`

What it does: Function format_coord takes coord: Coordinate and gives back String.

(Where these words came from: signature.)

## function — is_card_slot

Found in src/coord.rs at line 82.

Signature: `pub fn is_card_slot(coord: Coordinate) -> bool {`

What it does: Function is_card_slot takes coord: Coordinate and gives back bool.

(Where these words came from: signature.)

## function — is_roster

Found in src/coord.rs at line 78.

Signature: `pub fn is_roster(coord: Coordinate) -> bool {`

What it does: Function is_roster takes coord: Coordinate and gives back bool.

(Where these words came from: signature.)

## function — min_distance

Found in src/coord.rs at line 106.

Signature: `pub fn min_distance(thread: Coordinate, interests: &[Coordinate]) -> u64 {`

What it does: Function min_distance takes thread: Coordinate, interests: &[Coordinate] and gives back u64.

(Where these words came from: signature.)

## function — next_message_scroll

Found in src/coord.rs at line 116.

Signature: `pub fn next_message_scroll(used: impl IntoIterator<Item = usize>) -> usize {`

What it does: Smallest free message scroll in chapter 1, section 1. Scroll 1 is reserved for the thread card.

(Where these words came from: doc comment.)

## function — one_line

Found in src/coord.rs at line 151.

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

Found in src/coord.rs at line 131.

Signature: `pub fn sanitize(text: &str) -> String {`

What it does: Strip dimension breaks so a message cannot split the phext. Newlines stay; they are inside the scroll.

(Where these words came from: doc comment.)

## function — thread_anchor

Found in src/coord.rs at line 64.

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

Found in src/coord.rs at line 97.

Signature: `pub fn weighted_distance(a: Coordinate, b: Coordinate) -> u64 {`

What it does: Function weighted_distance takes a: Coordinate, b: Coordinate and gives back u64.

(Where these words came from: signature.)

## function — with_scroll

Found in src/coord.rs at line 72.

Signature: `pub fn with_scroll(thread: Coordinate, scroll: usize) -> Coordinate {`

What it does: Function with_scroll takes thread: Coordinate, scroll: usize and gives back Coordinate.

(Where these words came from: signature.)

## enum — Error

Found in src/error.rs at line 4.

Signature: `pub enum Error {`

What it does: A named choice named Error. It is one of: Io; Bad.

(Where these words came from: signature.)

## re-export — error::Error

Found in src/lib.rs at line 17.

Signature: `pub use error::Error;`

What it does: A re-export: error::Error is made reachable through this module, so other code can use it from here.

(Where these words came from: signature.)

## flag — --- phext::index ---

Found in src/main.rs at line 319.

Signature: `flag --- phext::index ---`

What it does: A command line flag, --- phext::index ---. It is read by the program when the user passes it.

(Where these words came from: signature.)

## struct — Message

Found in src/model.rs at line 43.

Signature: `pub struct Message {`

What it does: A data shape named Message. Its fields are: pub coord: Coordinate; pub author: String; pub ts: DateTime<Utc>; pub text: String.

(Where these words came from: signature.)

## enum — Scroll

Found in src/model.rs at line 60.

Signature: `pub enum Scroll {`

What it does: A named choice named Scroll. It is one of: User; Card.

(Where these words came from: signature.)

## struct — Thread

Found in src/model.rs at line 51.

Signature: `pub struct Thread {`

What it does: A data shape named Thread. Its fields are: pub coord: Coordinate; pub topic: String; pub title: String; pub keywords: String; pub messages: Vec<Message>.

(Where these words came from: signature.)

## struct — User

Found in src/model.rs at line 35.

Signature: `pub struct User {`

What it does: A data shape named User. Its fields are: pub id: String; pub name: String; pub blurb: String; pub interests: Vec<Coordinate>.

(Where these words came from: signature.)

## function — User::to_scroll

Found in src/model.rs at line 75.

Signature: `pub fn to_scroll(&self) -> String {`

What it does: Function User::to_scroll takes no inputs beyond the value it is called on, and gives back String.

(Where these words came from: signature.)

## function — content_tokens

Found in src/model.rs at line 219.

Signature: `pub fn content_tokens(text: &str) -> std::collections::BTreeSet<String> {`

What it does: Lowercased content words. A trailing "s" is stripped so "ferns" meets "fern".

(Where these words came from: doc comment.)

## function — format_card

Found in src/model.rs at line 91.

Signature: `pub fn format_card(topic: &str, title: &str, keywords: &str) -> String {`

What it does: Function format_card takes topic: &str, title: &str, keywords: &str and gives back String.

(Where these words came from: signature.)

## function — format_message

Found in src/model.rs at line 100.

Signature: `pub fn format_message(author: &str, ts: DateTime<Utc>, text: &str) -> String {`

What it does: Function format_message takes author: &str, ts: DateTime<Utc>, text: &str and gives back String.

(Where these words came from: signature.)

## function — keyword_line

Found in src/model.rs at line 126.

Signature: `pub fn keyword_line(text: &str) -> String {`

What it does: Function keyword_line takes text: &str and gives back String.

(Where these words came from: signature.)

## function — parse_scroll

Found in src/model.rs at line 133.

Signature: `pub fn parse_scroll(raw: &str) -> Option<Scroll> {`

What it does: Function parse_scroll takes raw: &str and gives back Option<Scroll>.

(Where these words came from: signature.)

## function — preview

Found in src/model.rs at line 240.

Signature: `pub fn preview(text: &str, limit: usize) -> String {`

What it does: Function preview takes text: &str, limit: usize and gives back String.

(Where these words came from: signature.)

## function — title_from_text

Found in src/model.rs at line 109.

Signature: `pub fn title_from_text(text: &str) -> String {`

What it does: Function title_from_text takes text: &str and gives back String.

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

Found in src/rank.rs at line 460.

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

Found in src/rank.rs at line 405.

Signature: `pub fn fresh_thread(threads: &[Thread], home: Coordinate) -> Coordinate {`

What it does: Function fresh_thread takes threads: &[Thread], home: Coordinate and gives back Coordinate.

(Where these words came from: signature.)

## function — is_flooded

Found in src/rank.rs at line 262.

Signature: `pub fn is_flooded(count: usize, share: f64) -> bool {`

What it does: Function is_flooded takes count: usize, share: f64 and gives back bool.

(Where these words came from: signature.)

## function — majority

Found in src/rank.rs at line 246.

Signature: `pub fn majority(messages: &[Message]) -> (Option<String>, usize, f64) {`

What it does: Function majority takes messages: &[Message] and gives back (Option<String>, usize, f64).

(Where these words came from: signature.)

## function — map_of

Found in src/rank.rs at line 218.

Signature: `pub fn map_of(threads: &[Thread], interests: &[Coordinate]) -> Vec<MapEntry> {`

What it does: Function map_of takes threads: &[Thread], interests: &[Coordinate] and gives back Vec<MapEntry>.

(Where these words came from: signature.)

## function — nearness

Found in src/rank.rs at line 87.

Signature: `pub fn nearness(distance: u64) -> &'static str {`

What it does: Function nearness takes distance: u64 and gives back &'static str.

(Where these words came from: signature.)

## function — suggest

Found in src/rank.rs at line 266.

Signature: `pub fn suggest( threads: &[Thread], interests: &[Coordinate], text: &str, now: DateTime<Utc>, ) -> Suggestion {`

What it does: Function suggest takes threads: &[Thread], interests: &[Coordinate], text: &str, now: DateTime<Utc>, and gives back Suggestion.

(Where these words came from: signature.)

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

## function — demo_instant

Found in src/seed.rs at line 174.

Signature: `pub fn demo_instant() -> DateTime<Utc> {`

What it does: Function demo_instant takes no inputs beyond the value it is called on, and gives back DateTime<Utc>.

(Where these words came from: signature.)

## function — document

Found in src/seed.rs at line 195.

Signature: `pub fn document(now: DateTime<Utc>) -> String {`

What it does: Build the demo phext by inserting each scroll in coordinate order.

(Where these words came from: doc comment.)

## function — readers

Found in src/seed.rs at line 178.

Signature: `pub fn readers() -> Vec<User> {`

What it does: Function readers takes no inputs beyond the value it is called on, and gives back Vec<User>.

(Where these words came from: signature.)

## struct — AppState

Found in src/server.rs at line 22.

Signature: `pub struct AppState {`

What it does: A data shape named AppState. The scanner did not read its fields.

(Where these words came from: signature.)

## function — router

Found in src/server.rs at line 45.

Signature: `pub fn router(state: AppState) -> Router {`

What it does: Function router takes state: AppState and gives back Router.

(Where these words came from: signature.)

## function — serve

Found in src/server.rs at line 26.

Signature: `pub async fn serve(path: std::path::PathBuf, port: u16) -> Result<(), Error> {`

What it does: Function serve takes path: std::path::PathBuf, port: u16 and gives back Result<(), Error>.

(Where these words came from: signature.)

## struct — Placed

Found in src/store.rs at line 51.

Signature: `pub struct Placed {`

What it does: A data shape named Placed. Its fields are: pub coord: Coordinate; pub thread: Coordinate; pub title: String; pub topic: String; pub new_thread: bool.

(Where these words came from: signature.)

## struct — Post

Found in src/store.rs at line 43.

Signature: `pub struct Post {`

What it does: A data shape named Post. Its fields are: pub author: String; pub text: String; pub target: Target; pub now: DateTime<Utc>.

(Where these words came from: signature.)

## struct — Store

Found in src/store.rs at line 59.

Signature: `pub struct Store {`

What it does: A data shape named Store. The scanner did not read its fields.

(Where these words came from: signature.)

## function — Store::open

Found in src/store.rs at line 65.

Signature: `pub fn open(path: impl Into<PathBuf>) -> Result<Self, Error> {`

What it does: Function Store::open takes path: impl Into<PathBuf> and gives back Result<Self, Error>.

(Where these words came from: signature.)

## struct — World

Found in src/store.rs at line 22.

Signature: `pub struct World {`

What it does: A data shape named World. Its fields are: pub users: Vec<User>; pub threads: Vec<Thread>.

(Where these words came from: signature.)

## function — World::find_user

Found in src/store.rs at line 28.

Signature: `pub fn find_user(&self, query: &str) -> Option<&User> {`

What it does: Function World::find_user takes &self, query: &str and gives back Option<&User>.

(Where these words came from: signature.)

## function — byte_index

Found in src/store.rs at line 104.

Signature: `pub fn byte_index(&self) -> String {`

What it does: Offsets from `phext::index`, one coordinate per line.

(Where these words came from: doc comment.)

## function — doc

Found in src/store.rs at line 85.

Signature: `pub fn doc(&self) -> &str {`

What it does: Function doc takes no inputs beyond the value it is called on, and gives back &str.

(Where these words came from: signature.)

## function — path

Found in src/store.rs at line 81.

Signature: `pub fn path(&self) -> &Path {`

What it does: Function path takes no inputs beyond the value it is called on, and gives back &Path.

(Where these words came from: signature.)

## function — place

Found in src/store.rs at line 124.

Signature: `pub fn place(&mut self, post: Post) -> Result<Placed, Error> {`

What it does: Function place takes &mut self, post: Post and gives back Result<Placed, Error>.

(Where these words came from: signature.)

## function — reseed

Found in src/store.rs at line 120.

Signature: `pub fn reseed(&mut self, now: DateTime<Utc>) -> Result<(), Error> {`

What it does: Function reseed takes &mut self, now: DateTime<Utc> and gives back Result<(), Error>.

(Where these words came from: signature.)

## function — select

Found in src/store.rs at line 94.

Signature: `pub fn select(&self, coord: Coordinate) -> String {`

What it does: `phext::fetch`

(Where these words came from: doc comment.)

## function — thread

Found in src/store.rs at line 36.

Signature: `pub fn thread(&self, key: &str) -> Option<&Thread> {`

What it does: Function thread takes &self, key: &str and gives back Option<&Thread>.

(Where these words came from: signature.)

## function — toc

Found in src/store.rs at line 99.

Signature: `pub fn toc(&self) -> String {`

What it does: `phext::textmap`

(Where these words came from: doc comment.)

## function — world

Found in src/store.rs at line 89.

Signature: `pub fn world(&self) -> World {`

What it does: Function world takes no inputs beyond the value it is called on, and gives back World.

(Where these words came from: signature.)

## function — world_from

Found in src/store.rs at line 217.

Signature: `pub fn world_from(doc: &str) -> World {`

What it does: Function world_from takes doc: &str and gives back World.

(Where these words came from: signature.)

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/phext-lanes/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

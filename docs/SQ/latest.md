# SQ — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 150). Scanned at commit e4c492bc4f2d52bc070f05830f3f1d9c7d0dbfe7. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

Description stated in the manifest: A minimal client-server for phext hosting

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 6.

- libphext [dependencies] — 0.6.0
- percent-encoding [dependencies] — 2.3.1
- raw_sync [dependencies] — 0.1.5
- shared_memory [dependencies] — 0.12.4
- serde [dependencies] — 1.0
- serde_json [dependencies] — 1.0


---

## Authors

The git log holds 172 commits.

The author of record is **Will Bickford** — 172 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 132 commits
- Theia — 9 commits
- Chrys — 8 commits
- Lux — 7 commits
- Orin — 6 commits
- Cyon — 4 commits
- Emise — 3 commits
- Phex — 2 commits
- Claude — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 11 files, 1133 lines total, 277 blank, 15 comment, 841 code
- Python: 1 file, 123 lines total, 26 blank, 1 comment, 96 code
- Rust: 10 files, 5160 lines total, 563 blank, 702 comment, 3895 code
- Shell: 9 files, 332 lines total, 44 blank, 38 comment, 250 code
- TOML: 4 files, 3096 lines total, 513 blank, 9 comment, 2574 code
- TypeScript: 2 files, 930 lines total, 98 blank, 88 comment, 744 code

All languages: 37 files, 10774 lines total, 8400 lines of code. Functions read: 201.

Largest files:

- gateway/cloud-500.toml — 3007 lines
- src/main.rs — 1684 lines
- demos/http-content-streams/web-basic/phextio.ts — 922 lines
- src/tests.rs — 891 lines
- src/sq.rs — 615 lines

Largest functions, by span in lines:

- handle_multi_tenant_connection_with_reload in src/main.rs — about 266 lines
- main in src/main.rs — about 264 lines
- process in src/sq.rs — about 246 lines
- handle_tcp_connection_inner in src/main.rs — about 243 lines
- run_api in src/api.rs — about 174 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 78 test functions counted in the source, across 1 file whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 28 of 88 (32%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/SQ
- Branch: exo
- Head commit: e4c492bc4f2d52bc070f05830f3f1d9c7d0dbfe7
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `load_config` (gateway/sq-gateway.py:13) — Function load_config takes no inputs beyond the value it is called on.
- function `reload_config` (gateway/sq-gateway.py:22) — Function reload_config takes signum=None, frame=None.
- function `extract_token` (gateway/sq-gateway.py:39) — Function extract_token takes headers.
- function `GatewayHandler.send_cors` (gateway/sq-gateway.py:47) — Function GatewayHandler.send_cors takes no inputs beyond the value it is called on.
- function `GatewayHandler.respond` (gateway/sq-gateway.py:51) — Function GatewayHandler.respond takes self, code, body, content_type="application/json".
- function `GatewayHandler.respond_json` (gateway/sq-gateway.py:60) — Function GatewayHandler.respond_json takes self, code, obj.
- function `GatewayHandler.do_OPTIONS` (gateway/sq-gateway.py:63) — Function GatewayHandler.do_OPTIONS takes no inputs beyond the value it is called on.
- function `GatewayHandler.route` (gateway/sq-gateway.py:68) — Function GatewayHandler.route takes no inputs beyond the value it is called on.
- function `GatewayHandler.proxy_to` (gateway/sq-gateway.py:84) — Function GatewayHandler.proxy_to takes self, port.
- function `GatewayHandler.log_message` (gateway/sq-gateway.py:111) — Function GatewayHandler.log_message takes self, fmt, *args.
- function `next_port` (gateway/tenant-manager.sh:14) — Function next_port takes no inputs beyond the value it is called on.
- function `cmd_add` (gateway/tenant-manager.sh:22) — Function cmd_add takes no inputs beyond the value it is called on.
- function `cmd_remove` (gateway/tenant-manager.sh:45) — Function cmd_remove takes no inputs beyond the value it is called on.
- function `cmd_list` (gateway/tenant-manager.sh:67) — Function cmd_list takes no inputs beyond the value it is called on.
- struct `ApiConfig` (src/api.rs:26) — A data shape named ApiConfig.
- function `load_api_config` (src/api.rs:58) — Function load_api_config takes path: &str.
- function `run_api` (src/api.rs:344) — Function run_api takes config_path: &str, listen_port: u16.
- struct `PromptCache` (src/cache.rs:18) — SHA256-keyed prompt cache with LRU eviction and TTL expiry
- function `PromptCache::new` (src/cache.rs:28) — Function PromptCache::new takes max_entries: usize, ttl_secs: u64.
- function `PromptCache::get` (src/cache.rs:58) — Look up a cached response
- function `PromptCache::set` (src/cache.rs:83) — Store a prompt-response pair
- function `PromptCache::check_static` (src/cache.rs:105) — Check static patterns (greetings, status, ping)
- function `PromptCache::hit_rate` (src/cache.rs:115) — Function PromptCache::hit_rate takes no inputs beyond the value it is called on.
- function `PromptCache::stats` (src/cache.rs:120) — Function PromptCache::stats takes no inputs beyond the value it is called on.
- struct `TenantConfig` (src/config.rs:10) — A data shape named TenantConfig.
- struct `ServerConfig` (src/config.rs:16) — A data shape named ServerConfig.
- function `load_config` (src/config.rs:22) — Load multi-tenant configuration from JSON file Automatically creates tenant data directories if they don't exist
- const `MAX_BODY_SIZE` (src/main.rs:73) — A fixed value named MAX_BODY_SIZE, set to 64 * 1024 * 1024.
- enum `HashAlgorithm` (src/main.rs:167) — A named choice named HashAlgorithm.
- entry point `main` (src/main.rs:264) — The entry point: the program starts here.
- struct `HttpRequest` (src/main.rs:608) — A data shape named HttpRequest.
- function `read_http_request` (src/main.rs:613) — Function read_http_request takes stream: &mut TcpStream.
- flag `--config` (src/main.rs:368) — A command line flag, --config.
- flag `--data-dir` (src/main.rs:354) — A command line flag, --data-dir.
- flag `--key` (src/main.rs:348) — A command line flag, --key.
- flag `--mesh-config` (src/main.rs:362) — A command line flag, --mesh-config.
- struct `NodeConfig` (src/mesh.rs:13) — Node identity configuration
- struct `InboundConfig` (src/mesh.rs:22) — Inbound server configuration
- struct `PeerConfig` (src/mesh.rs:31) — Outbound peer connection details
- struct `OutboundConfig` (src/mesh.rs:43) — Outbound connections configuration
- struct `MeshSettings` (src/mesh.rs:50) — Mesh synchronization settings
- struct `MeshConfig` (src/mesh.rs:59) — Complete mesh configuration
- function `load_mesh_config` (src/mesh.rs:105) — Load mesh configuration from file # Arguments `path` - Path to mesh.json configuration file # Returns…
- function `save_mesh_config` (src/mesh.rs:150) — Save mesh configuration to file # Arguments `config` - Mesh configuration to save `path` - Path to write mesh.json #…
- function `generate_default_config` (src/mesh.rs:178) — Generate default mesh config for a node # Arguments `id` - Node identifier (e.g., "halycon-vector") `name` - Node…
- function `print_config_summary` (src/mesh.rs:207) — Print mesh configuration summary to stdout
- struct `TenantConfig` (src/router.rs:29) — A data shape named TenantConfig.
- struct `RouterConfig` (src/router.rs:36) — A data shape named RouterConfig.
- function `load_router_config` (src/router.rs:43) — Function load_router_config takes path: &str.
- function `run_router` (src/router.rs:215) — Function run_router takes config_path: &str, listen_port: u16.
- function `scroll_hash` (src/seed.rs:56) — Function scroll_hash takes content: &str.
- function `build_seed` (src/seed.rs:69) — Function build_seed takes phext_map: &HashMap<phext::Coordinate, String>.
- function `parse_seed` (src/seed.rs:92) — Function parse_seed takes seed_json: &str.
- struct `FetchEntry` (src/seed.rs:111) — A data shape named FetchEntry.
- struct `FetchRequest` (src/seed.rs:117) — A data shape named FetchRequest.
- function `parse_fetch_get` (src/seed.rs:123) — GET form: c=<coord> (strict), h=<hash> (may be empty/absent), has=<0|1|true|false>
- function `parse_fetch_post` (src/seed.rs:138) — POST form: body is a JSON array of {"coordinate", "hash", "has_content"}
- function `run_fetch` (src/seed.rs:186) — Runs a parsed fetch request against a loaded phext map
- struct `CacheEntry` (src/seed.rs:201) — A data shape named CacheEntry.
- struct `ScrollCache` (src/seed.rs:208) — Scrolls a client has already streamed in, keyed by coordinate string.
- function `ScrollCache::empty` (src/seed.rs:213) — Function ScrollCache::empty takes no inputs beyond the value it is called on.
- function `ScrollCache::load` (src/seed.rs:218) — Loads a cache file; a missing or unreadable file is simply an empty cache
- function `ScrollCache::save` (src/seed.rs:239) — Function ScrollCache::save takes &self, path: &str.
- function `ScrollCache::get` (src/seed.rs:250) — Function ScrollCache::get takes &self, coordinate: &str.
- function `ScrollCache::put` (src/seed.rs:254) — Function ScrollCache::put takes &mut self, coordinate: &str, hash: String, scroll: String.
- function `http_get` (src/seed.rs:299) — Function http_get takes base: &str, path: &str, key: &Option<String>.
- function `fetch_seed` (src/seed.rs:304) — Downloads the seed envelope for a phext and parses it into coordinate -> hash
- function `fetch_cached` (src/seed.rs:322) — Function fetch_cached takes base: &str, phext_name: &str, key: &Option<String>, coordinate: &str, cache: &mut ScrollCache.
- function `args_required` (src/sq.rs:10) — Function args_required takes command:&str.
- function `implode_ref` (src/sq.rs:101) — Function implode_ref takes map: &HashMap<phext::Coordinate, String>.
- function `process` (src/sq.rs:146) — Function process takes connection_id: u64, source: String, scroll: &mut String, command: String, phext_map: &mut…
- struct `QuerySpec` (src/sq.rs:385) — A data shape named QuerySpec.
- function `parse_coordinate_strict` (src/sq.rs:392) — Strict coordinate parser: exactly 9 positive integers as a.b.c/d.e.f/g.h.i
- function `parse_query` (src/sq.rs:409) — Function parse_query takes from: Option<&String>, to: Option<&String>, set: Option<&String>, limit: Option<&String>.
- function `run_query` (src/sq.rs:447) — Runs a query against a loaded phext map and returns a JSON array of {coordinate, scroll}
- enum `TocMode` (src/sq.rs:495) — awaiting a summary
- struct `TocSpec` (src/sq.rs:498) — A data shape named TocSpec.
- function `parse_toc_prefix` (src/sq.rs:505) — Function parse_toc_prefix takes text: &str.
- function `parse_toc_spec` (src/sq.rs:520) — Function parse_toc_spec takes mode: Option<&String>, at: Option<&String>, step: Option<&String>, scale: Option<&String>.
- function `run_toc` (src/sq.rs:558) — Function run_toc takes phext_map: &HashMap<phext::Coordinate, String>, spec: &TocSpec.
- enum `Tier` (src/triage.rs:12) — Which tier handles this prompt
- struct `RouteDecision` (src/triage.rs:20) — Result of evaluating a prompt
- function `evaluate` (src/triage.rs:42) — Score a prompt and decide routing tier
- struct `FeedbackLoop` (src/triage.rs:85) — Feedback loop — tracks local model quality
- function `FeedbackLoop::new` (src/triage.rs:91) — Function FeedbackLoop::new takes max_window: usize.
- function `FeedbackLoop::record` (src/triage.rs:98) — Function FeedbackLoop::record takes &mut self, success: bool.
- function `FeedbackLoop::should_escalate` (src/triage.rs:106) — Returns true if failure rate exceeds threshold (should escalate)
- function `FeedbackLoop::failure_rate` (src/triage.rs:114) — Function FeedbackLoop::failure_rate takes no inputs beyond the value it is called on.

---

## function — load_config

Found in gateway/sq-gateway.py at line 13.

Surface: public API.

Signature: `def load_config()`

What it does: Function load_config takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning gw, tenants.

(Where these words came from: inferred.)

---

## function — reload_config

Found in gateway/sq-gateway.py at line 22.

Surface: public API.

Signature: `def reload_config(signum=None, frame=None)`

What it does: Function reload_config takes signum=None, frame=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — extract_token

Found in gateway/sq-gateway.py at line 39.

Surface: public API.

Signature: `def extract_token(headers)`

What it does: Function extract_token takes headers. Its return type is unannotated; the code shows it returning auth[7:].strip(); "".

(Where these words came from: inferred.)

---

## function — GatewayHandler.send_cors

Found in gateway/sq-gateway.py at line 47.

Surface: public API.

Signature: `def send_cors(self)`

What it does: Function GatewayHandler.send_cors takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — GatewayHandler.respond

Found in gateway/sq-gateway.py at line 51.

Surface: public API.

Signature: `def respond(self, code, body, content_type="application/json")`

What it does: Function GatewayHandler.respond takes self, code, body, content_type="application/json". Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — GatewayHandler.respond_json

Found in gateway/sq-gateway.py at line 60.

Surface: public API.

Signature: `def respond_json(self, code, obj)`

What it does: Function GatewayHandler.respond_json takes self, code, obj. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — GatewayHandler.do_OPTIONS

Found in gateway/sq-gateway.py at line 63.

Surface: public API.

Signature: `def do_OPTIONS(self)`

What it does: Function GatewayHandler.do_OPTIONS takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — GatewayHandler.route

Found in gateway/sq-gateway.py at line 68.

Surface: public API.

Signature: `def route(self)`

What it does: Function GatewayHandler.route takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning self.respond_json(200, {"status": "ok", "tenants; self.respond_json(401, {"error": "unauthorized"}; self.respond_json(200, {"tenants": [t["name"] fo; self.respond_json(401, {"error": "invalid or mis.

(Where these words came from: inferred.)

---

## function — GatewayHandler.proxy_to

Found in gateway/sq-gateway.py at line 84.

Surface: public API.

Signature: `def proxy_to(self, port)`

What it does: Function GatewayHandler.proxy_to takes self, port. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — GatewayHandler.log_message

Found in gateway/sq-gateway.py at line 111.

Surface: public API.

Signature: `def log_message(self, fmt, *args)`

What it does: Function GatewayHandler.log_message takes self, fmt, *args. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — next_port

Found in gateway/tenant-manager.sh at line 14.

Surface: public API.

Signature: `next_port()`

What it does: Function next_port takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — cmd_add

Found in gateway/tenant-manager.sh at line 22.

Surface: public API.

Signature: `cmd_add()`

What it does: Function cmd_add takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — cmd_remove

Found in gateway/tenant-manager.sh at line 45.

Surface: public API.

Signature: `cmd_remove()`

What it does: Function cmd_remove takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — cmd_list

Found in gateway/tenant-manager.sh at line 67.

Surface: public API.

Signature: `cmd_list()`

What it does: Function cmd_list takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## struct — ApiConfig

Found in src/api.rs at line 26.

Surface: public API.

Signature: `pub struct ApiConfig {`

What it does: A data shape named ApiConfig. Its fields are: pub local_url: String,; pub local_model: String,; pub local_timeout_secs: u64; pub upstream_url: String,; pub upstream_api_key: String; pub upstream_model: String,; pub cache_max_entries: usize; pub cache_ttl_secs: u64; pub signal_threshold: usize; pub escalation_threshold: f64; pub feedback_window: usize.

(Where these words came from: inferred.)

---

## function — load_api_config

Found in src/api.rs at line 58.

Surface: public API.

Signature: `pub fn load_api_config(path: &str) -> Result<ApiConfig, Box<dyn std::error::Error>> {`

What it does: Function load_api_config takes path: &str. It gives back Result<ApiConfig, Box<dyn std::error::Error>>.

(Where these words came from: inferred.)

---

## function — run_api

Found in src/api.rs at line 344.

Surface: public API.

Signature: `pub fn run_api(config_path: &str, listen_port: u16) -> Result<(), Box<dyn std::error::Error>> {`

What it does: Function run_api takes config_path: &str, listen_port: u16. It gives back Result<(), Box<dyn std::error::Error>>.

(Where these words came from: inferred.)

---

## struct — PromptCache

Found in src/cache.rs at line 18.

Surface: public API.

Signature: `pub struct PromptCache {`

What it does: SHA256-keyed prompt cache with LRU eviction and TTL expiry

(Where these words came from: direct.)

---

## function — PromptCache::new

Found in src/cache.rs at line 28.

Surface: public API.

Signature: `pub fn new(max_entries: usize, ttl_secs: u64) -> Self {`

What it does: Function PromptCache::new takes max_entries: usize, ttl_secs: u64. It gives back Self.

(Where these words came from: inferred.)

---

## function — PromptCache::get

Found in src/cache.rs at line 58.

Surface: public API.

Signature: `pub fn get(&mut self, prompt: &str) -> Option<String> {`

What it does: Look up a cached response

(Where these words came from: direct.)

---

## function — PromptCache::set

Found in src/cache.rs at line 83.

Surface: public API.

Signature: `pub fn set(&mut self, prompt: &str, response: &str) {`

What it does: Store a prompt-response pair

(Where these words came from: direct.)

---

## function — PromptCache::check_static

Found in src/cache.rs at line 105.

Surface: public API.

Signature: `pub fn check_static(prompt: &str) -> Option<&'static str> {`

What it does: Check static patterns (greetings, status, ping)

(Where these words came from: direct.)

---

## function — PromptCache::hit_rate

Found in src/cache.rs at line 115.

Surface: public API.

Signature: `pub fn hit_rate(&self) -> f64 {`

What it does: Function PromptCache::hit_rate takes no inputs beyond the value it is called on. It gives back f64.

(Where these words came from: inferred.)

---

## function — PromptCache::stats

Found in src/cache.rs at line 120.

Surface: public API.

Signature: `pub fn stats(&self) -> (u64, u64, usize) {`

What it does: Function PromptCache::stats takes no inputs beyond the value it is called on. It gives back (u64, u64, usize).

(Where these words came from: inferred.)

---

## struct — TenantConfig

Found in src/config.rs at line 10.

Surface: public API.

Signature: `pub struct TenantConfig {`

What it does: A data shape named TenantConfig. Its fields are: pub name: String; pub data_dir: String.

(Where these words came from: inferred.)

---

## struct — ServerConfig

Found in src/config.rs at line 16.

Surface: public API.

Signature: `pub struct ServerConfig {`

What it does: A data shape named ServerConfig. Its fields are: pub tenants: HashMap<String, TenantConfig>.

(Where these words came from: inferred.)

---

## function — load_config

Found in src/config.rs at line 22.

Surface: public API.

Signature: `pub fn load_config(path: &str) -> Result<ServerConfig, Box<dyn std::error::Error>> {`

What it does: Load multi-tenant configuration from JSON file Automatically creates tenant data directories if they don't exist

(Where these words came from: direct.)

---

## const — MAX_BODY_SIZE

Found in src/main.rs at line 73.

Surface: public API.

Signature: `pub const MAX_BODY_SIZE: usize = 64 * 1024 * 1024; // 64 MB per request body`

What it does: A fixed value named MAX_BODY_SIZE, set to 64 * 1024 * 1024.

(Where these words came from: inferred.)

---

## enum — HashAlgorithm

Found in src/main.rs at line 167.

Surface: public API.

Signature: `pub enum HashAlgorithm {`

What it does: A named choice named HashAlgorithm. It is one of: Xor; Checksum.

(Where these words came from: inferred.)

---

## entry point — main

Found in src/main.rs at line 264.

Surface: public API.

Signature: `fn main() -> Result<(), Box<dyn std::error::Error>> {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## struct — HttpRequest

Found in src/main.rs at line 608.

Surface: public API.

Signature: `pub struct HttpRequest {`

What it does: A data shape named HttpRequest. Its fields are: pub header: String; pub content: Vec<u8>.

(Where these words came from: inferred.)

---

## function — read_http_request

Found in src/main.rs at line 613.

Surface: public API.

Signature: `pub fn read_http_request(stream: &mut TcpStream) -> std::io::Result<HttpRequest> {`

What it does: Function read_http_request takes stream: &mut TcpStream. It gives back std::io::Result<HttpRequest>.

(Where these words came from: inferred.)

---

## flag — --config

Found in src/main.rs at line 368.

Surface: public API.

Signature: `flag --config`

What it does: A command line flag, --config. It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## flag — --data-dir

Found in src/main.rs at line 354.

Surface: public API.

Signature: `flag --data-dir`

What it does: A command line flag, --data-dir. It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## flag — --key

Found in src/main.rs at line 348.

Surface: public API.

Signature: `flag --key`

What it does: A command line flag, --key. It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## flag — --mesh-config

Found in src/main.rs at line 362.

Surface: public API.

Signature: `flag --mesh-config`

What it does: A command line flag, --mesh-config. It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## struct — NodeConfig

Found in src/mesh.rs at line 13.

Surface: public API.

Signature: `pub struct NodeConfig {`

What it does: Node identity configuration

(Where these words came from: direct.)

---

## struct — InboundConfig

Found in src/mesh.rs at line 22.

Surface: public API.

Signature: `pub struct InboundConfig {`

What it does: Inbound server configuration

(Where these words came from: direct.)

---

## struct — PeerConfig

Found in src/mesh.rs at line 31.

Surface: public API.

Signature: `pub struct PeerConfig {`

What it does: Outbound peer connection details

(Where these words came from: direct.)

---

## struct — OutboundConfig

Found in src/mesh.rs at line 43.

Surface: public API.

Signature: `pub struct OutboundConfig {`

What it does: Outbound connections configuration

(Where these words came from: direct.)

---

## struct — MeshSettings

Found in src/mesh.rs at line 50.

Surface: public API.

Signature: `pub struct MeshSettings {`

What it does: Mesh synchronization settings

(Where these words came from: direct.)

---

## struct — MeshConfig

Found in src/mesh.rs at line 59.

Surface: public API.

Signature: `pub struct MeshConfig {`

What it does: Complete mesh configuration

(Where these words came from: direct.)

---

## function — load_mesh_config

Found in src/mesh.rs at line 105.

Surface: public API.

Signature: `pub fn load_mesh_config<P: AsRef<Path>>(path: P) -> Result<MeshConfig, String> {`

What it does: Load mesh configuration from file # Arguments `path` - Path to mesh.json configuration file # Returns `Ok(MeshConfig)` if config loaded successfully `Err(String)` with error message if loading failed

(Where these words came from: direct.)

---

## function — save_mesh_config

Found in src/mesh.rs at line 150.

Surface: public API.

Signature: `pub fn save_mesh_config<P: AsRef<Path>>(config: &MeshConfig, path: P) -> Result<(), String> {`

What it does: Save mesh configuration to file # Arguments `config` - Mesh configuration to save `path` - Path to write mesh.json # Returns `Ok(())` if config saved successfully `Err(String)` with error message if save failed

(Where these words came from: direct.)

---

## function — generate_default_config

Found in src/mesh.rs at line 178.

Surface: public API.

Signature: `pub fn generate_default_config(id: &str, name: &str, emoji: &str, coordinate: &str) -> MeshConfig {`

What it does: Generate default mesh config for a node # Arguments `id` - Node identifier (e.g., "halycon-vector") `name` - Node name (e.g., "Cyon") `emoji` - Node emoji (e.g., "🪶") `coordinate` - Phext coordinate (e.g., "2.7.1/8.2.8/3.1.4") # Returns `MeshConfig` with sensible defaults for the node

(Where these words came from: direct.)

---

## function — print_config_summary

Found in src/mesh.rs at line 207.

Surface: public API.

Signature: `pub fn print_config_summary(config: &MeshConfig) {`

What it does: Print mesh configuration summary to stdout

(Where these words came from: direct.)

---

## struct — TenantConfig

Found in src/router.rs at line 29.

Surface: public API.

Signature: `pub struct TenantConfig {`

What it does: A data shape named TenantConfig. Its fields are: pub token: String,; pub port: u16,; pub data_dir: String,.

(Where these words came from: inferred.)

---

## struct — RouterConfig

Found in src/router.rs at line 36.

Surface: public API.

Signature: `pub struct RouterConfig {`

What it does: A data shape named RouterConfig. Its fields are: pub tenants: Vec<TenantConfig>.

(Where these words came from: inferred.)

---

## function — load_router_config

Found in src/router.rs at line 43.

Surface: public API.

Signature: `pub fn load_router_config(path: &str) -> Result<RouterConfig, Box<dyn std::error::Error>> {`

What it does: Function load_router_config takes path: &str. It gives back Result<RouterConfig, Box<dyn std::error::Error>>.

(Where these words came from: inferred.)

---

## function — run_router

Found in src/router.rs at line 215.

Surface: public API.

Signature: `pub fn run_router(config_path: &str, listen_port: u16) -> Result<(), Box<dyn std::error::Error>> {`

What it does: Function run_router takes config_path: &str, listen_port: u16. It gives back Result<(), Box<dyn std::error::Error>>.

(Where these words came from: inferred.)

---

## function — scroll_hash

Found in src/seed.rs at line 56.

Surface: public API.

Signature: `pub fn scroll_hash(content: &str) -> String {`

What it does: Function scroll_hash takes content: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — build_seed

Found in src/seed.rs at line 69.

Surface: public API.

Signature: `pub fn build_seed(phext_map: &HashMap<phext::Coordinate, String>) -> String {`

What it does: Function build_seed takes phext_map: &HashMap<phext::Coordinate, String>. It gives back String.

(Where these words came from: inferred.)

---

## function — parse_seed

Found in src/seed.rs at line 92.

Surface: public API.

Signature: `pub fn parse_seed(seed_json: &str) -> Result<HashMap<String, String>, String> {`

What it does: Function parse_seed takes seed_json: &str. It gives back Result<HashMap<String, String>, String>.

(Where these words came from: inferred.)

---

## struct — FetchEntry

Found in src/seed.rs at line 111.

Surface: public API.

Signature: `pub struct FetchEntry {`

What it does: A data shape named FetchEntry. Its fields are: pub coordinate: phext::Coordinate; pub client_hash: Option<String>; pub has_content: bool.

(Where these words came from: inferred.)

---

## struct — FetchRequest

Found in src/seed.rs at line 117.

Surface: public API.

Signature: `pub struct FetchRequest {`

What it does: A data shape named FetchRequest. Its fields are: pub entries: Vec<FetchEntry>; pub single: bool.

(Where these words came from: inferred.)

---

## function — parse_fetch_get

Found in src/seed.rs at line 123.

Surface: public API.

Signature: `pub fn parse_fetch_get(parsed: &HashMap<String, String>) -> Result<FetchRequest, String> {`

What it does: GET form: c=<coord> (strict), h=<hash> (may be empty/absent), has=<0|1|true|false>

(Where these words came from: direct.)

---

## function — parse_fetch_post

Found in src/seed.rs at line 138.

Surface: public API.

Signature: `pub fn parse_fetch_post(body: &str) -> Result<FetchRequest, String> {`

What it does: POST form: body is a JSON array of {"coordinate", "hash", "has_content"}

(Where these words came from: direct.)

---

## function — run_fetch

Found in src/seed.rs at line 186.

Surface: public API.

Signature: `pub fn run_fetch(phext_map: &HashMap<phext::Coordinate, String>, spec: &FetchRequest) -> String {`

What it does: Runs a parsed fetch request against a loaded phext map

(Where these words came from: direct.)

---

## struct — CacheEntry

Found in src/seed.rs at line 201.

Surface: public API.

Signature: `pub struct CacheEntry {`

What it does: A data shape named CacheEntry. Its fields are: pub hash: String; pub scroll: String.

(Where these words came from: inferred.)

---

## struct — ScrollCache

Found in src/seed.rs at line 208.

Surface: public API.

Signature: `pub struct ScrollCache {`

What it does: Scrolls a client has already streamed in, keyed by coordinate string. Persisted as one JSON file: {"<coord>": {"hash": "...", "scroll": "..."}}

(Where these words came from: direct.)

---

## function — ScrollCache::empty

Found in src/seed.rs at line 213.

Surface: public API.

Signature: `pub fn empty() -> ScrollCache {`

What it does: Function ScrollCache::empty takes no inputs beyond the value it is called on. It gives back ScrollCache.

(Where these words came from: inferred.)

---

## function — ScrollCache::load

Found in src/seed.rs at line 218.

Surface: public API.

Signature: `pub fn load(path: &str) -> ScrollCache {`

What it does: Loads a cache file; a missing or unreadable file is simply an empty cache

(Where these words came from: direct.)

---

## function — ScrollCache::save

Found in src/seed.rs at line 239.

Surface: public API.

Signature: `pub fn save(&self, path: &str) -> Result<(), String> {`

What it does: Function ScrollCache::save takes &self, path: &str. It gives back Result<(), String>.

(Where these words came from: inferred.)

---

## function — ScrollCache::get

Found in src/seed.rs at line 250.

Surface: public API.

Signature: `pub fn get(&self, coordinate: &str) -> Option<&CacheEntry> {`

What it does: Function ScrollCache::get takes &self, coordinate: &str. It gives back Option<&CacheEntry>.

(Where these words came from: inferred.)

---

## function — ScrollCache::put

Found in src/seed.rs at line 254.

Surface: public API.

Signature: `pub fn put(&mut self, coordinate: &str, hash: String, scroll: String) {`

What it does: Function ScrollCache::put takes &mut self, coordinate: &str, hash: String, scroll: String. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — http_get

Found in src/seed.rs at line 299.

Surface: public API.

Signature: `pub fn http_get(base: &str, path: &str, key: &Option<String>) -> Result<String, String> {`

What it does: Function http_get takes base: &str, path: &str, key: &Option<String>. It gives back Result<String, String>.

(Where these words came from: inferred.)

---

## function — fetch_seed

Found in src/seed.rs at line 304.

Surface: public API.

Signature: `pub fn fetch_seed(base: &str, phext_name: &str, key: &Option<String>) -> Result<HashMap<String, String>, String> {`

What it does: Downloads the seed envelope for a phext and parses it into coordinate -> hash

(Where these words came from: direct.)

---

## function — fetch_cached

Found in src/seed.rs at line 322.

Surface: public API.

Signature: `pub fn fetch_cached(base: &str, phext_name: &str, key: &Option<String>, coordinate: &str, cache: &mut ScrollCache) -> Result<(String, bool), String> {`

What it does: Function fetch_cached takes base: &str, phext_name: &str, key: &Option<String>, coordinate: &str, cache: &mut ScrollCache. It gives back Result<(String, bool), String>.

(Where these words came from: inferred.)

---

## function — args_required

Found in src/sq.rs at line 10.

Surface: public API.

Signature: `pub fn args_required(command:&str) -> usize {`

What it does: Function args_required takes command:&str. It gives back usize.

(Where these words came from: inferred.)

---

## function — implode_ref

Found in src/sq.rs at line 101.

Surface: public API.

Signature: `pub fn implode_ref(map: &HashMap<phext::Coordinate, String>) -> String {`

What it does: Function implode_ref takes map: &HashMap<phext::Coordinate, String>. It gives back String.

(Where these words came from: inferred.)

---

## function — process

Found in src/sq.rs at line 146.

Surface: public API.

Signature: `pub fn process(connection_id: u64, source: String, scroll: &mut String, command: String, phext_map: &mut HashMap::<phext::Coordinate, String>, coordinate: phext::Coordinate, update: String, filename: String, algorithm: crate::HashAlgorithm, limit: usize) -> bool {`

What it does: Function process takes connection_id: u64, source: String, scroll: &mut String, command: String, phext_map: &mut HashMap::<phext::Coordinate, String>, coordinate: phext::Coordinate, update: String, filename: String, algorithm: crate::HashAlgorithm, limit: usize. It gives back bool.

(Where these words came from: inferred.)

---

## struct — QuerySpec

Found in src/sq.rs at line 385.

Surface: public API.

Signature: `pub struct QuerySpec {`

What it does: A data shape named QuerySpec. Its fields are: pub range: Option<(phext::Coordinate, phext::Coordinate)>; pub set: Vec<phext::Coordinate>; pub limit: Option<usize>.

(Where these words came from: inferred.)

---

## function — parse_coordinate_strict

Found in src/sq.rs at line 392.

Surface: public API.

Signature: `pub fn parse_coordinate_strict(text: &str) -> Option<phext::Coordinate> {`

What it does: Strict coordinate parser: exactly 9 positive integers as a.b.c/d.e.f/g.h.i

(Where these words came from: direct.)

---

## function — parse_query

Found in src/sq.rs at line 409.

Surface: public API.

Signature: `pub fn parse_query(from: Option<&String>, to: Option<&String>, set: Option<&String>, limit: Option<&String>) -> Result<QuerySpec, String> {`

What it does: Function parse_query takes from: Option<&String>, to: Option<&String>, set: Option<&String>, limit: Option<&String>. It gives back Result<QuerySpec, String>.

(Where these words came from: inferred.)

---

## function — run_query

Found in src/sq.rs at line 447.

Surface: public API.

Signature: `pub fn run_query(phext_map: &HashMap<phext::Coordinate, String>, spec: &QuerySpec) -> String {`

What it does: Runs a query against a loaded phext map and returns a JSON array of {coordinate, scroll}

(Where these words came from: direct.)

---

## enum — TocMode

Found in src/sq.rs at line 495.

Surface: public API.

Signature: `pub enum TocMode { Full, Children, Summary, Scrolls, Matches }`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## struct — TocSpec

Found in src/sq.rs at line 498.

Surface: public API.

Signature: `pub struct TocSpec {`

What it does: A data shape named TocSpec. Its fields are: pub mode: TocMode; pub prefix: Vec<usize>; pub step: usize; pub scale: Option<usize>.

(Where these words came from: inferred.)

---

## function — parse_toc_prefix

Found in src/sq.rs at line 505.

Surface: public API.

Signature: `pub fn parse_toc_prefix(text: &str) -> Result<Vec<usize>, String> {`

What it does: Function parse_toc_prefix takes text: &str. It gives back Result<Vec<usize>, String>.

(Where these words came from: inferred.)

---

## function — parse_toc_spec

Found in src/sq.rs at line 520.

Surface: public API.

Signature: `pub fn parse_toc_spec(mode: Option<&String>, at: Option<&String>, step: Option<&String>, scale: Option<&String>) -> Result<Option<TocSpec>, String> {`

What it does: Function parse_toc_spec takes mode: Option<&String>, at: Option<&String>, step: Option<&String>, scale: Option<&String>. It gives back Result<Option<TocSpec>, String>.

(Where these words came from: inferred.)

---

## function — run_toc

Found in src/sq.rs at line 558.

Surface: public API.

Signature: `pub fn run_toc(phext_map: &HashMap<phext::Coordinate, String>, spec: &TocSpec) -> String {`

What it does: Function run_toc takes phext_map: &HashMap<phext::Coordinate, String>, spec: &TocSpec. It gives back String.

(Where these words came from: inferred.)

---

## enum — Tier

Found in src/triage.rs at line 12.

Surface: public API.

Signature: `pub enum Tier {`

What it does: Which tier handles this prompt

(Where these words came from: direct.)

---

## struct — RouteDecision

Found in src/triage.rs at line 20.

Surface: public API.

Signature: `pub struct RouteDecision {`

What it does: Result of evaluating a prompt

(Where these words came from: direct.)

---

## function — evaluate

Found in src/triage.rs at line 42.

Surface: public API.

Signature: `pub fn evaluate(prompt: &str, upstream_threshold: usize) -> RouteDecision {`

What it does: Score a prompt and decide routing tier

(Where these words came from: direct.)

---

## struct — FeedbackLoop

Found in src/triage.rs at line 85.

Surface: public API.

Signature: `pub struct FeedbackLoop {`

What it does: Feedback loop — tracks local model quality

(Where these words came from: direct.)

---

## function — FeedbackLoop::new

Found in src/triage.rs at line 91.

Surface: public API.

Signature: `pub fn new(max_window: usize) -> Self {`

What it does: Function FeedbackLoop::new takes max_window: usize. It gives back Self.

(Where these words came from: inferred.)

---

## function — FeedbackLoop::record

Found in src/triage.rs at line 98.

Surface: public API.

Signature: `pub fn record(&mut self, success: bool) {`

What it does: Function FeedbackLoop::record takes &mut self, success: bool. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — FeedbackLoop::should_escalate

Found in src/triage.rs at line 106.

Surface: public API.

Signature: `pub fn should_escalate(&self, threshold: f64) -> bool {`

What it does: Returns true if failure rate exceeds threshold (should escalate)

(Where these words came from: direct.)

---

## function — FeedbackLoop::failure_rate

Found in src/triage.rs at line 114.

Surface: public API.

Signature: `pub fn failure_rate(&self) -> f64 {`

What it does: Function FeedbackLoop::failure_rate takes no inputs beyond the value it is called on. It gives back f64.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/SQ/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

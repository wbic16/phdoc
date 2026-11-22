# qexo — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 150). Scanned at commit 6f5497308ddc733398f0c915c8f9c63a9a095531. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

Description stated in the manifest: qexo — the Exocortex coordinator, phext-native: all node state lives in one phext (system /var/lib/qexo or per-user ~/.qexo; Layer 2 addressing over phext)

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 3.

- libphext [dependencies] — 0.6.0
- pgp [dependencies] — =0.21.1
- rand [dependencies] — 0.8

Total packages in the lock file, this one included: 157.


---

## Authors

The git log holds 16 commits.

The author of record is **Will Bickford** — 16 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Emise — 8 commits
- Will Bickford (his own identities) — 8 commits

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 9 files, 3174 lines total, 466 blank, 35 comment, 2673 code
- Python: 3 files, 382 lines total, 41 blank, 22 comment, 319 code
- Rust: 20 files, 23267 lines total, 1033 blank, 3870 comment, 18364 code
- TOML: 2 files, 78 lines total, 10 blank, 31 comment, 37 code

All languages: 34 files, 26901 lines total, 21393 lines of code. Functions read: 697.

Largest files:

- src/core.rs — 4385 lines
- src/wire.rs — 3335 lines
- src/dispatch.rs — 2246 lines
- src/register.rs — 1821 lines
- src/learn.rs — 1687 lines

Largest functions, by span in lines:

- dispatch_run in src/wire.rs — about 548 lines
- check_queue in src/doctor.rs — about 467 lines
- cmd_heartbeat in src/wire.rs — about 387 lines
- regen_indexes in src/lattice.rs — about 286 lines
- update_inner in src/core.rs — about 257 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 159 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 188 of 319 (59%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/qexo
- Branch: exo
- Head commit: 6f5497308ddc733398f0c915c8f9c63a9a095531
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `app_session` (src/app.rs:576) — The dispatch app session (§2.3): greeting block, then one command per line until `quit` or EOF.
- const `KINDS` (src/bridge.rs:41) — The kind enum (§9.1) — the model line (`tlm`, `slm`, `llm`) is in it from the first bridge slice, so the local line…
- const `DEFAULT_FRESHNESS` (src/bridge.rs:45) — A fixed value named DEFAULT_FRESHNESS, set to 900.
- struct `Bridge` (src/bridge.rs:51) — One bridge record (Collection 5, Scroll 1 of its Book).
- function `bridge_addr` (src/bridge.rs:65) — The address of bridge Book `book`'s record scroll.
- function `health_addr` (src/bridge.rs:71) — The address of a health-log scroll in bridge Book `book`.
- function `parse_bridge` (src/bridge.rs:75) — Function parse_bridge takes book: u32, text: &str.
- function `load_bridges` (src/bridge.rs:93) — Every bridge record in the state, in Book order.
- function `find_bridge` (src/bridge.rs:104) — Function find_bridge takes px: &Phext, name: &str.
- function `brief_prefs` (src/bridge.rs:110) — The `bridge:` preference lines of a brief, in order, duplicates removed (the brief_facts precedent).
- struct `HealthRec` (src/bridge.rs:127) — One health-log atom (a probe and its outcome).
- function `load_health` (src/bridge.rs:137) — A bridge Book's health log, in scroll order.
- enum `Liveness` (src/bridge.rs:162) — What the health log says about a bridge right now — calculated on every call, never stored (§9.1).
- function `liveness` (src/bridge.rs:174) — Function liveness takes px: &Phext, bridge: &Bridge.
- function `health_label` (src/bridge.rs:191) — A human rendering of the calculated health, for the `qexo bridge` list and the dispatch --check report.
- struct `ProbeOutcome` (src/bridge.rs:218) — A data shape named ProbeOutcome.
- function `probe` (src/bridge.rs:346) — Probe an endpoint, by its shape: http/https URLs are fetched, a bare host:port is connected to, anything else is…
- function `append_health` (src/bridge.rs:409) — Append one probe outcome to a bridge's health log; returns the scroll's address (the probe cite routing puts in the…
- struct `Chosen` (src/bridge.rs:438) — A completed routing: the bridge chosen, and the probe scroll that verifies it live for this dispatch.
- enum `Route` (src/bridge.rs:444) — A named choice named Route.
- function `mid_execution_skips` (src/bridge.rs:464) — Bridges this item must not be re-routed to on its next dispatch: the `from` of a `bridge-failover` receipt filed at…
- function `route` (src/bridge.rs:491) — Walk an item's bridge preferences (§9.2).
- function `cmd_bridge` (src/bridge.rs:757) — Function cmd_bridge takes args: &[String].
- enum `Command` (src/cli.rs:12) — A named choice named Command.
- function `parse` (src/cli.rs:52) — Function parse takes args: &[String].
- function `help_text` (src/cli.rs:248) — Function help_text takes no inputs beyond the value it is called on.
- function `config` (src/core.rs:303) — Function config takes auto_update: bool, evolve: bool, release_signer: Option<String>.
- function `register` (src/core.rs:707) — Function register takes args: &[String].
- function `list` (src/core.rs:1250) — Function list takes json: bool.
- function `host` (src/core.rs:1481) — Function host takes persona: Option<String>, boot: Option<String>.
- function `unhost` (src/core.rs:1701) — Function unhost takes name: &str.
- function `message` (src/core.rs:1738) — Function message takes persona: &str, text: &str.
- function `status` (src/core.rs:1805) — Function status takes no inputs beyond the value it is called on.
- function `plan_cmd` (src/core.rs:2064) — Function plan_cmd takes no inputs beyond the value it is called on.
- function `serve` (src/core.rs:2211) — Function serve takes port: u16.
- function `release` (src/core.rs:2630) — `qexo release` — on the signer: package the running binary into the releases dir, write + sign the manifest scroll…
- function `update` (src/core.rs:2750) — `qexo update [--check]` — poll the trusted peers, judge every offer by the trust rule, and apply the trusted one.
- function `evolve` (src/core.rs:3707) — `qexo evolve [--check]` — one convergence pass over the phextnet.
- struct `Gpg` (src/crypto.rs:59) — The node's keyring: a handle on the keyring directory, nothing more — key material is read from the store on each…
- function `Gpg::open_in` (src/crypto.rs:116) — Root the keyring at a state DIRECTORY (the state file's parent): the gnupg home is created beside the state file if needed, mode 0700 on Unix.
- function `Gpg::generate` (src/crypto.rs:256) — Generate this node's key: an Ed25519 signing key whose user id is `name (kind_label)` — the same shape the gpg era's batch spec produced.
- function `Gpg::list_fingerprints` (src/crypto.rs:300) — The fingerprints of the secret keys this keyring holds — in practice the node's own, exactly one.
- function `Gpg::export_public` (src/crypto.rs:315) — The armored public key for a fingerprint in the keyring.
- function `Gpg::fingerprint_of` (src/crypto.rs:322) — The fingerprint of armored public key material, without importing it.
- function `Gpg::name_of` (src/crypto.rs:330) — The user id (name) carried by armored public key material — what gpg's colon listing used to report and `register`…
- function `Gpg::import_public` (src/crypto.rs:336) — Import an armored public key into the keyring.
- function `Gpg::sign_detached` (src/crypto.rs:346) — A detached, armored signature over `data` by the named key (which must be this node's own — the only secret key a…
- function `Gpg::verify_detached` (src/crypto.rs:370) — Verify a detached armored signature over `data` against every public key the keyring holds; on success returns the…
- function `Gpg::encrypt_to` (src/crypto.rs:395) — Encrypt `data` to every recipient fingerprint (all must be in the keyring), optionally signing with this node's own secret key first.
- function `Gpg::decrypt` (src/crypto.rs:431) — Decrypt an armored message with this node's own key.
- struct `RunLines` (src/dispatch.rs:54) — One verb run's rendered lines, split by stream, plus the structured facts a second driver renders in its own shape (slice 7, spec §2.3).
- function `RunLines::new` (src/dispatch.rs:63) — Capturing mode: lines are collected, nothing printed.
- function `RunLines::live` (src/dispatch.rs:69) — Live mode: lines print as produced (and are still collected, for callers that want the record).
- function `RunLines::out` (src/dispatch.rs:73) — Function RunLines::out takes &mut self, line: String.
- function `RunLines::err` (src/dispatch.rs:80) — Function RunLines::err takes &mut self, line: String.
- function `RunLines::fact` (src/dispatch.rs:87) — Function RunLines::fact takes &mut self, key: &str, value: &str.
- function `item_record` (src/dispatch.rs:166) — Build the item record scroll (spec §3.2).
- function `item_addr` (src/dispatch.rs:213) — The address of item Book `book`'s record scroll.
- function `brief_addr` (src/dispatch.rs:218) — The address of item Book `book`'s brief scroll.
- function `enqueue` (src/dispatch.rs:222) — Function enqueue takes args: &[String].
- function `enqueue_core` (src/dispatch.rs:317) — The enqueue act itself (§3.2, §4.1), over a loaded lattice: the title derivation, the F6 burn:never refusal (§9.2), the item record, the save.
- function `show` (src/dispatch.rs:394) — Function show takes addr_s: &str.
- enum `ItemState` (src/dispatch.rs:454) — The eight queue states (spec §4.6).
- const `ALL_STATES` (src/dispatch.rs:465) — A fixed value named ALL_STATES, set to [.
- function `ItemState::parse` (src/dispatch.rs:477) — Function ItemState::parse takes s: &str.
- function `ItemState::name` (src/dispatch.rs:491) — Function ItemState::name takes no inputs beyond the value it is called on.
- function `ItemState::is_terminal` (src/dispatch.rs:504) — Function ItemState::is_terminal takes no inputs beyond the value it is called on.
- enum `Actor` (src/dispatch.rs:519) — Who performs a transition (spec §4.6 names an actor per transition; actors are checked, not trusted — illegal combination 4 is a cancel by the wrong actor).
- struct `Item` (src/dispatch.rs:536) — A queue item record (§3.2), parsed.
- function `parse_item` (src/dispatch.rs:571) — Function parse_item takes book: u32, text: &str.
- function `load_items` (src/dispatch.rs:607) — Every queue item in the document, in Book (arrival) order.
- function `local_actor` (src/dispatch.rs:621) — This node's actor string — node name + full fingerprint, the shape `enqueued-by` already carries (receipts cite…
- struct `Transition` (src/dispatch.rs:653) — A transition request (spec §4.6).
- function `Transition::new` (src/dispatch.rs:671) — Function Transition::new takes to: ItemState, actor: Actor, actor_name: &str.
- struct `Plan` (src/dispatch.rs:692) — What a legal transition will do, before anything is written: the state it lands in (a third consecutive failure lands…
- enum `AuditOwed` (src/dispatch.rs:701) — A named choice named AuditOwed.
- function `check_transition` (src/dispatch.rs:716) — The legality oracle (spec §4.6): is this transition allowed from the item's current state, by this actor, with this…
- function `append_audit` (src/dispatch.rs:878) — Append an audit scroll to the audits run (6.6.6, Collection 9, spec §7.1) and return its coordinate.
- function `apply_transition` (src/dispatch.rs:911) — Apply a transition: the item's new state, its receipt, and its audit (where one is owed) are mutated into the…
- struct `TransitionReceipt` (src/dispatch.rs:1058) — A transition receipt (6.6.6, Collection 2, action: transition), parsed — the doctor's evidence (§4.5).
- function `load_transition_receipts` (src/dispatch.rs:1067) — Function load_transition_receipts takes px: &phext::Phext.
- struct `AuditRec` (src/dispatch.rs:1090) — An audit record (6.6.6, Collection 9, §7.1), parsed.
- function `load_audits` (src/dispatch.rs:1101) — Function load_audits takes px: &phext::Phext.
- struct `AssignmentRec` (src/dispatch.rs:1130) — An assignment record (11.11.11, Collection 3, §3.1), parsed leniently — slice 4 writes these; the doctor reads them now.
- function `load_assignments` (src/dispatch.rs:1142) — Function load_assignments takes px: &phext::Phext.
- function `is_sha256_hex` (src/dispatch.rs:1166) — A SHA-256 digest's shape: 64 hex characters.
- function `daily_goal` (src/dispatch.rs:1178) — The node's current daily goal: a field on the config record (`daily-goal:` in 9.9.9, Collection 2), set at the morning synthesis.
- function `goal_class` (src/dispatch.rs:1186) — The §5.1 goal class: 0 = names the current daily goal, 1 = standing, 2 = background (names some other goal).
- function `goal_class_name` (src/dispatch.rs:1199) — Function goal_class_name takes class: u8.
- function `order_items` (src/dispatch.rs:1267) — The queue's full order (spec §5 + §2.1): paused-auto items pin to the head, flagged, always; `pin: head` items…
- function `queue` (src/dispatch.rs:1351) — Function queue takes all: bool, node: Option<String>, explain: Option<String>.
- function `pause` (src/dispatch.rs:1549) — Function pause takes args: &[String].
- function `resume` (src/dispatch.rs:1558) — Function resume takes args: &[String].
- function `cancel` (src/dispatch.rs:1566) — Function cancel takes args: &[String].
- function `audit` (src/dispatch.rs:1632) — `qexo audit` (spec §2.1, §7.2): Will's window on the audit cluster — list with filters, newest last (ledger order); `audit show` renders one in full.
- function `report` (src/distbuilds.rs:129) — The verification half, shared by the upgrade run and status: every line is a fact probed now, nothing remembered.
- function `distributed_builds` (src/distbuilds.rs:192) — Function distributed_builds takes scheduler: bool, status: bool, branch: Option<String>.
- struct `Finding` (src/doctor.rs:22) — A data shape named Finding.
- struct `Report` (src/doctor.rs:29) — A data shape named Report.
- function `Report::ok` (src/doctor.rs:34) — Function Report::ok takes no inputs beyond the value it is called on.
- function `Report::problems` (src/doctor.rs:37) — Function Report::problems takes no inputs beyond the value it is called on.
- function `Report::headline` (src/doctor.rs:42) — One-line headline for `status`: "doctor ok" or "doctor: N problem(s)".
- function `Report::render` (src/doctor.rs:49) — Function Report::render takes no inputs beyond the value it is called on.
- function `check` (src/doctor.rs:164) — Function check takes state_dir: &Path, state_file: &Path.
- function `fix` (src/doctor.rs:269) — Apply the filesystem repairs: create the state directory, tighten the keyring mode, remove a stale lock.
- function `check_lattice` (src/doctor.rs:320) — Check the lattice layer of an EXISTING state document: home index and homeland indexes exist; every record is named…
- function `fix_lattice` (src/doctor.rs:407) — Repair the lattice layer: regenerate every index scroll from the records.
- function `check_queue` (src/doctor.rs:485) — Doctor's queue section.
- function `sha256` (src/hash.rs:17) — Function sha256 takes data: &[u8].
- function `sha256_hex` (src/hash.rs:62) — Function sha256_hex takes data: &[u8].
- function `hex` (src/hash.rs:66) — Function hex takes bytes: &[u8].
- enum `InferError` (src/infer.rs:20) — A named choice named InferError.
- struct `Infer` (src/infer.rs:42) — A data shape named Infer.
- const `DEFAULT_ENDPOINT` (src/infer.rs:49) — A fixed value named DEFAULT_ENDPOINT, set to "http://127.0.0.1:11434".
- const `DEFAULT_MODEL` (src/infer.rs:50) — A fixed value named DEFAULT_MODEL, set to "gemma4:e4b".
- function `Infer::from_env` (src/infer.rs:57) — From the environment: QEXO_OLLAMA_URL, QEXO_MODEL.
- function `Infer::new` (src/infer.rs:69) — Function Infer::new takes endpoint: &str, model: &str.
- function `Infer::tags` (src/infer.rs:77) — Model names the endpoint currently serves (GET /api/tags).
- function `Infer::require_model` (src/infer.rs:83) — Confirm the endpoint is up AND carries our model.
- function `Infer::chat` (src/infer.rs:100) — One chat exchange.
- function `Infer::prime` (src/infer.rs:126) — Load the persona into local inference: a priming exchange with a long keep-alive, so the model stays resident with…
- function `Infer::unload` (src/infer.rs:134) — Unload the model (keep_alive 0).
- function `inventory` (src/inventory.rs:241) — `qexo inventory [<name>]` / `qexo inventory --install <name>` (alias verb: catalog).
- const `HL_HOME` (src/lattice.rs:64) — A fixed value named HL_HOME, set to [1, 1, 1].
- const `HL_KEYS` (src/lattice.rs:65) — A fixed value named HL_KEYS, set to [2, 2, 2].
- const `HL_PEERS` (src/lattice.rs:66) — A fixed value named HL_PEERS, set to [3, 3, 3].
- const `HL_PERSONAS` (src/lattice.rs:67) — A fixed value named HL_PERSONAS, set to [4, 4, 4].
- const `HL_PLAN` (src/lattice.rs:68) — A fixed value named HL_PLAN, set to [5, 5, 5].
- const `HL_LEDGERS` (src/lattice.rs:69) — A fixed value named HL_LEDGERS, set to [6, 6, 6].
- const `HL_MAIL` (src/lattice.rs:70) — A fixed value named HL_MAIL, set to [7, 7, 7].
- const `HL_TELEMETRY` (src/lattice.rs:71) — A fixed value named HL_TELEMETRY, set to [8, 8, 8].
- const `HL_CONFIG` (src/lattice.rs:72) — A fixed value named HL_CONFIG, set to [9, 9, 9].
- const `HL_LEARN` (src/lattice.rs:77) — Lessons & learning (v1.4.0): the tens — practice, repeated.
- const `LEARN_DECK_C` (src/lattice.rs:81) — The curriculum deck's cluster inside the learn homeland: one Book per card, grown by adjacency (Layer 2 growth rule).
- const `HL_DISPATCH` (src/lattice.rs:87) — Dispatch & queue (dispatch core slice 1, spec §3.1): the elevens — send-off country.
- const `DISPATCH_ITEMS_C` (src/lattice.rs:95) — Clusters (Collection) inside dispatch & queue, in the spec's order: queue items first (the ledger family — one Book…
- const `DISPATCH_ASSIGN_C` (src/lattice.rs:96) — A fixed value named DISPATCH_ASSIGN_C, set to 3.
- const `DISPATCH_SCHED_C` (src/lattice.rs:97) — A fixed value named DISPATCH_SCHED_C, set to 4.
- const `DISPATCH_BRIDGES_C` (src/lattice.rs:98) — A fixed value named DISPATCH_BRIDGES_C, set to 5.
- const `DISPATCH_REGISTER_C` (src/lattice.rs:99) — A fixed value named DISPATCH_REGISTER_C, set to 6.
- const `HOME` (src/lattice.rs:102) — The home scroll: prose index of the whole state file.
- const `IDENTITY` (src/lattice.rs:104) — Node identity record, adjacent to home in the home run.
- const `KEYS_INDEX_C` (src/lattice.rs:109) — Clusters (Collection) inside keys & trust (anchor 2.2.2/2.2.2/2.2.2), neighboring runs in the order the brief names…
- const `KEYS_NODE_C` (src/lattice.rs:110) — A fixed value named KEYS_NODE_C, set to 2.
- const `KEYS_PERSONA_C` (src/lattice.rs:111) — A fixed value named KEYS_PERSONA_C, set to 3.
- const `KEYS_PEER_C` (src/lattice.rs:112) — A fixed value named KEYS_PEER_C, set to 4.
- const `KEYS_GRANTS_C` (src/lattice.rs:113) — A fixed value named KEYS_GRANTS_C, set to 5.
- const `KEYS_DELEGATIONS_C` (src/lattice.rs:114) — A fixed value named KEYS_DELEGATIONS_C, set to 6.
- const `KEYS_POLICY_C` (src/lattice.rs:115) — A fixed value named KEYS_POLICY_C, set to 7.
- const `NODE_KEY_RECORD` (src/lattice.rs:118) — The node key record and its armored public key (adjacent scroll).
- const `NODE_KEY_PUBLIC` (src/lattice.rs:119) — A fixed value named NODE_KEY_PUBLIC, set to [2, 2, 2, KEYS_NODE_C, 1, 1, 1, 1, 2].
- const `LEDGER_RECEIPTS` (src/lattice.rs:122) — Ledger kinds as clusters (Collection) inside 6.6.6.
- const `LEDGER_TELEPORTS` (src/lattice.rs:123) — A fixed value named LEDGER_TELEPORTS, set to 3.
- const `LEDGER_INSTANCES` (src/lattice.rs:124) — A fixed value named LEDGER_INSTANCES, set to 4.
- const `LEDGER_EMOTION` (src/lattice.rs:125) — A fixed value named LEDGER_EMOTION, set to 5.
- const `LEDGER_GOAL_ACTIVITY` (src/lattice.rs:126) — A fixed value named LEDGER_GOAL_ACTIVITY, set to 6.
- const `LEDGER_SYNC` (src/lattice.rs:127) — A fixed value named LEDGER_SYNC, set to 7.
- const `LEDGER_RELEASES` (src/lattice.rs:133) — Release announcements (v1.1.0): one Book per announced release — the manifest record at Scroll 1, its detached signature at Scroll 2.
- const `LEDGER_AUDITS` (src/lattice.rs:140) — Audits (dispatch core slice 2, spec §7.1): one Book per audit — the record of what was checked and why an act was…
- function `ledger_kind_name` (src/lattice.rs:142) — Function ledger_kind_name takes cluster: u32.
- const `MAIL_SESSIONS_C` (src/lattice.rs:157) — Sessions (Collection 2) and mailbox (Collection 3) inside 7.7.7.
- const `MAIL_BOX_C` (src/lattice.rs:158) — A fixed value named MAIL_BOX_C, set to 3.
- const `TELEM_DISK_C` (src/lattice.rs:161) — Telemetry: disk observations (Collection 2) inside 8.8.8.
- const `PLAN_CURRENT` (src/lattice.rs:166) — Plan (anchor 5.5.5/5.5.5/5.5.5): the current plan lives in Collection 2 (fields at Scroll 1, the human rendering at…
- const `PLAN_CURRENT_RENDER` (src/lattice.rs:167) — A fixed value named PLAN_CURRENT_RENDER, set to [5, 5, 5, 2, 1, 1, 1, 1, 2].
- const `PLAN_HISTORY_C` (src/lattice.rs:168) — A fixed value named PLAN_HISTORY_C, set to 3.
- const `CONFIG_RECORD` (src/lattice.rs:175) — qexo's own config record: the node's settings as field lines — no config file anywhere else.
- function `at` (src/lattice.rs:183) — Compose an address from a homeland and the remaining six components [Collection, Volume, Book, Chapter, Section…
- function `index_addr` (src/lattice.rs:195) — A homeland's ANCHOR: the full 9D address of its index scroll — the patterned address the region is named by (Layer 2…
- function `prefix_scrolls` (src/lattice.rs:201) — Every scroll whose address starts with `prefix`, in ascending (coordinate) order — the range read of a run or a…
- function `component_values` (src/lattice.rs:211) — Distinct values of component `idx` among scrolls under `prefix`, sorted — e.g.
- function `next_free_book` (src/lattice.rs:222) — The next free Book (>= start) in the cluster addressed by [Library, Shelf, Series, Collection, Volume]: the adjacency…
- function `next_free_cluster` (src/lattice.rs:238) — The next free Collection (>= start) in a homeland: a new entity cluster beside the existing ones.
- function `next_free_scroll` (src/lattice.rs:250) — The next free Scroll (>= start) in the run addressed by the first eight components.
- function `field` (src/lattice.rs:266) — Read `key` from a record scroll's `key: value` lines.
- function `set_field` (src/lattice.rs:278) — Set `key` in a record scroll: replace its line if present, append it if not.
- function `record` (src/lattice.rs:299) — Build a record scroll from field lines.
- struct `RecordRef` (src/lattice.rs:317) — A data shape named RecordRef.
- function `records` (src/lattice.rs:326) — Enumerate peer, persona, and session records across the homelands, in address order.
- function `regen_indexes` (src/lattice.rs:381) — Regenerate the home index and every homeland index scroll from the records present.
- function `index_addresses` (src/lattice.rs:667) — Every index scroll the current Layer 2 layout defines: the home index plus each homeland's anchor scroll, the tens included.
- function `seed_missing_indexes` (src/lattice.rs:686) — The state top-up (v1.4.1): seed any index scroll the current layout defines that this document lacks, with exactly the text `regen_indexes` would give it.
- function `system_state_path` (src/lattice.rs:705) — The system state path: /var/lib/qexo/state.phext — canonical for system installs (root-run services, the ranch fleet).
- function `user_state_path` (src/lattice.rs:715) — The per-user state path (v1.4.0): $HOME/.qexo/state.phext — on Windows, under the user profile (%USERPROFILE%…
- struct `StateFacts` (src/lattice.rs:732) — The facts the resolution order reads.
- function `set_write_intent` (src/lattice.rs:767) — Function set_write_intent takes no inputs beyond the value it is called on.
- function `write_intent` (src/lattice.rs:771) — Function write_intent takes no inputs beyond the value it is called on.
- function `set_learn_preference` (src/lattice.rs:788) — Function set_learn_preference takes no inputs beyond the value it is called on.
- function `learn_preference` (src/lattice.rs:792) — Function learn_preference takes no inputs beyond the value it is called on.
- function `pick_state_path` (src/lattice.rs:827) — The resolution order (v1.4.1), stated exactly as shipped: 1.
- function `state_path` (src/lattice.rs:902) — The node's state path, resolved by the order above.
- function `birth_path` (src/lattice.rs:919) — The path a birth should use, with the fallback applied: when resolution chose the system path for a state that does…
- function `state_dir` (src/lattice.rs:958) — The directory holding the state file (and the GPG keyring, which is the gpg tool's own machinery and lives beside the…
- struct `Lattice` (src/lattice.rs:973) — The loaded state document.
- function `Lattice::born` (src/lattice.rs:982) — An unborn state document bound to `path` (config's birth).
- function `Lattice::universe` (src/lattice.rs:987) — Universe N of the state chain (1-based; 1 is `px`).
- function `Lattice::universe_count` (src/lattice.rs:996) — How many universes the loaded chain carries.
- function `Lattice::set_universe` (src/lattice.rs:1005) — Replace universe N (1-based), growing the chain with empty universes where a gap would open — slice 4's writer: an…
- function `Lattice::load` (src/lattice.rs:1021) — Load the state file if it exists.
- function `Lattice::load_from` (src/lattice.rs:1026) — Load from an explicit path (tests, doctor).
- function `Lattice::path` (src/lattice.rs:1037) — Function Lattice::path takes no inputs beyond the value it is called on.
- function `Lattice::save` (src/lattice.rs:1054) — Persist the whole document.
- const `MASTERY_INTERVAL_DAYS` (src/learn.rs:45) — A card counts as mastered at this interval (days).
- function `learn` (src/learn.rs:995) — `qexo learn [--status]`.
- function `learn_app` (src/learn.rs:1253) — `qexo learn --app` — the wizard's engine over stdin/stdout.
- entry point `main` (src/main.rs:37) — The entry point: the program starts here.
- type `Coord` (src/phext.rs:53) — A type name, Coord, that stands for [u32; 9].
- function `parse_coord` (src/phext.rs:55) — Function parse_coord takes s: &str.
- function `fmt_coord` (src/phext.rs:70) — Function fmt_coord takes c: &Coord.
- function `parse_universe_addr` (src/phext.rs:81) — Parse an address as the CLI verbs take it: a bare 9D coordinate means universe 1; a `$N:` prefix (libphext 0.6's…
- struct `CoordFmt` (src/phext.rs:99) — awaiting a summary
- struct `Phext` (src/phext.rs:131) — A data shape named Phext.
- function `Phext::new` (src/phext.rs:136) — Function Phext::new takes no inputs beyond the value it is called on.
- function `Phext::insert` (src/phext.rs:140) — Function Phext::insert takes &mut self, coord: Coord, text: String.
- function `Phext::get` (src/phext.rs:144) — Function Phext::get takes &self, coord: &Coord.
- function `Phext::toc` (src/phext.rs:148) — Function Phext::toc takes no inputs beyond the value it is called on.
- function `Phext::serialize` (src/phext.rs:155) — Serialize with the library: implode sorts the scrolls and walks the dead-reckoning path emitting libphext's delimiter…
- function `Phext::parse` (src/phext.rs:167) — Parse with the library: explode reckons every scroll's address libphext's way; this adapter then drops the empty…
- function `Phext::parse_universes` (src/phext.rs:187) — Parse a CHAIN file (libphext 0.6: members joined by the chain break \x02, the chain optionally closed by \x03) into…
- function `Phext::serialize_universes` (src/phext.rs:208) — Serialize universes back into a chain file.
- const `VERSION` (src/plan.rs:58) — The build's own version, from the Cargo manifest — the single source every user-facing version stamp derives from (v1.7.2).
- const `PLAN_VERSION` (src/plan.rs:59) — A fixed value named PLAN_VERSION, set to concat!("v", env!("CARGO_PKG_VERSION")).
- enum `Class` (src/plan.rs:62) — A named choice named Class.
- function `Class::name` (src/plan.rs:69) — Function Class::name takes no inputs beyond the value it is called on.
- function `classify` (src/plan.rs:78) — Function classify takes title: &str, request: &str.
- function `host_snapshot` (src/plan.rs:109) — (MemAvailable MB, loadavg 1min) of the host qexo runs on — a live probe of /proc, as before.
- struct `DiskObs` (src/plan.rs:134) — One disk observation: the current reading, and (as history entries) the series the forecast gate rates growth from.
- struct `Volume` (src/plan.rs:147) — A volume as the probe measures it: the filesystem under a named root, in bytes.
- function `Volume::free_pct` (src/plan.rs:159) — Free space as a percentage of the whole volume.
- struct `GateConfig` (src/plan.rs:173) — The gate's thresholds, read from the node's config record in state.phext (homeland 9.9.9/9.9.9/9.9.9).
- function `GateConfig::from_lookup` (src/plan.rs:211) — Build from a config record via a field lookup (the record's `key: value` lines; the caller supplies the reader so this module stays free of lattice mechanics).
- struct `GateInputs` (src/plan.rs:246) — Everything the gate needs beyond the observations: the volume the probe measured (None when the probe could not read…
- struct `GateEval` (src/plan.rs:271) — The gate's verdict, with its reasons kept as text: which lens fired and the numbers it fired on.
- function `fmt_gib` (src/plan.rs:314) — Format bytes as GiB (2^30) with thousands separators — the units `df -h` speaks, labeled so no one has to guess.
- function `fmt_count` (src/plan.rs:321) — Format a count with thousands separators (1,100,000).
- function `evaluate_gate` (src/plan.rs:332) — Evaluate the disk gate from values: the current observation, the measured volume, the file count's root, the thresholds.
- struct `TaskInput` (src/plan.rs:412) — A task row as the planner consumes it.
- struct `Forecast` (src/plan.rs:419) — A data shape named Forecast.
- function `Forecast::unavailable` (src/plan.rs:435) — Function Forecast::unavailable takes no inputs beyond the value it is called on.
- const `FORECAST_HORIZON_DAYS` (src/plan.rs:457) — How far ahead a crossing projection is reported, in days.
- const `FORECAST_WINDOW_INTERVALS` (src/plan.rs:464) — How many of the most recent intervals the forecast rates (v1.2.1).
- function `forecast_gate` (src/plan.rs:518) — The forecast computation (v2.3 semantics, re-based on the gate's lenses in v1.0.3; rates re-based on the median…
- struct `PlanRow` (src/plan.rs:612) — A data shape named PlanRow.
- struct `PlanData` (src/plan.rs:620) — A data shape named PlanData.
- function `PlanData::open_tasks` (src/plan.rs:656) — Function PlanData::open_tasks takes no inputs beyond the value it is called on.
- function `parse_roster` (src/plan.rs:678) — Parse the config record's `ranch_roster` value (v1.7.3): comma-separated node names, whitespace-trimmed, empties and…
- function `compute` (src/plan.rs:710) — Compute the plan: byte gate: the worse of the lenses that are ON (see evaluate_gate) — the volume lens judges the…
- function `days_from_civil` (src/plan.rs:794) — Days from civil date (Howard Hinnant's algorithm) — the v2.3 date arithmetic, carried verbatim in behavior.
- function `render` (src/plan.rs:835) — Human-readable rendering of a plan (the v1 shape, kept).
- function `drain_record_addr` (src/register.rs:72) — Function drain_record_addr takes no inputs beyond the value it is called on.
- struct `DrainRecord` (src/register.rs:82) — The register link's standing facts: where the register is, how far the drain has read, and the last pass's cite.
- function `load_drain_record` (src/register.rs:89) — Function load_drain_record takes px: &phext::Phext.
- struct `Credentials` (src/register.rs:127) — The register credentials, by name.
- function `credentials_path` (src/register.rs:132) — Function credentials_path takes no inputs beyond the value it is called on.
- function `load_credentials` (src/register.rs:139) — Function load_credentials takes no inputs beyond the value it is called on.
- function `parse_credentials` (src/register.rs:151) — Parse the env file's `KEY=value` lines.
- function `stamp_of_epoch` (src/register.rs:180) — The house stamp for an epoch-seconds instant — the same reckoning `now_stamp` writes and `stamp_minutes` parses back…
- struct `EntryAtom` (src/register.rs:540) — A register entry atom (§10.1): text in, from a user, at a time, at a seq.
- struct `ResolutionAtom` (src/register.rs:550) — A register resolution atom (§10.1): register number, outcome, the user who resolved it, an optional note.
- struct `Fetched` (src/register.rs:604) — What one drain pass pulled: both logs past the watermarks, and each log's high-water seq (the service's own…
- struct `PendingWriteback` (src/register.rs:670) — A write-back the drain found still owed: a terminal register item whose resolution is not yet recorded at the register.
- struct `DrainOutcome` (src/register.rs:679) — A data shape named DrainOutcome.
- function `apply_drain` (src/register.rs:717) — Function apply_drain takes lat: &mut Lattice, fetched: &Fetched, actor: &str.
- function `post_resolution` (src/register.rs:849) — POST a resolution for register number `n` under the qexo service token.
- function `post_entry` (src/register.rs:883) — POST an entry under the qexo service token (the ratified Q2 leg: items born on the phone appear in the register web view).
- function `write_back` (src/register.rs:902) — Record one owed write-back: post the resolution, then — in one save — flip the item's field to `recorded` and write…
- function `write_back_with` (src/register.rs:907) — Function write_back_with takes lat: &mut Lattice, item: &dispatch::Item, outcome: &str, note: Option<&str>, creds: &Credentials,.
- function `drain` (src/register.rs:962) — Function drain takes check: bool, url: Option<String>.
- function `drain_run` (src/register.rs:977) — The drain as a captured run (slice 7, spec §2.3): the same pass `drain` makes — fetch, apply, the write-back sweep —…
- const `BUNDLE_ANCHOR` (src/wire.rs:100) — The bundle manifest's address inside $1: — the bundle's own 11.11.11/11.11.11/11.11.11 (§3.4).
- const `BUNDLE_SIG` (src/wire.rs:104) — The detached signature beside the manifest — the next scroll of the anchor's Book (§3.4: "beside it at Scroll 2").
- const `THERMAL_LIMIT_C` (src/wire.rs:125) — The thermal gate (§6): a node at or above 75 °C fails eligibility (b).
- const `HEAVY_CAP` (src/wire.rs:128) — The heavy-work cap (§6): at most 4 heavy items running fleet-wide, FIFO among heavy items.
- function `field_all` (src/wire.rs:133) — Every line in `text` carrying `key:`, in order — `lattice::field` reads the first; capability lists, exclusion lists…
- struct `BriefFacts` (src/wire.rs:183) — What placement and the bundle builder read from an item's brief scroll: `capability:` lines (repeatable), `heavy…
- function `brief_facts` (src/wire.rs:192) — Function brief_facts takes brief: &str.
- function `unsigned_chain_bytes` (src/wire.rs:241) — The canonical bytes a bundle's hash is taken over: the chain of its universes with the signature scroll removed.
- function `build_bundle_parts` (src/wire.rs:266) — Build a bundle's universes (unsigned) and manifest for `item` as it stands in `px` — the record pre-transition (state queued, attempts = attempt−1).
- struct `Bundle` (src/wire.rs:433) — A built bundle: its identity hash (§4.3), its manifest, and its unsigned universes.
- function `build_bundle` (src/wire.rs:441) — Function build_bundle takes px: &Phext, item: &Item, attempt: u32, resume_from: Option<&str>, resume_text: Option<&str>, bridge: Option<.
- function `sign_bundle` (src/wire.rs:464) — Sign a built bundle into its wire bytes: the detached signature over the manifest's exact bytes at the anchor's…
- struct `ParsedBundle` (src/wire.rs:509) — A parsed wire bundle: the manifest's claims, the universes as received, and the manifest's exact bytes (the…
- function `parse_wire_bundle` (src/wire.rs:530) — Function parse_wire_bundle takes bytes: &[u8].
- function `verify_bundle` (src/wire.rs:583) — Verify a parsed bundle (§3.4, in the acceptance rule's order): the claimed signer must be a registered peer in this…
- function `file_attempt` (src/wire.rs:681) — File an accepted bundle's attempt (§3.3, §3.4): the attempt universe (attempt N → universe N+1) holds the executor's…
- function `file_origin_snapshot` (src/wire.rs:737) — File the ORIGIN's as-dispatched snapshot of an attempt (§4.6 combination 7's conservative reading: attempts homed…
- enum `AcceptOutcome` (src/wire.rs:756) — The outcome of an acceptance attempt — the serve handler renders it, the dispatch verb cites it.
- function `next_assignment_book` (src/wire.rs:803) — The next free assignment Book (Collection 3).
- function `accept_wire_bundle` (src/wire.rs:896) — Accept a wire bundle on this node (§3.4), in the acceptance rule's order: parse → the signer is a registered peer and…
- struct `NodeFacts` (src/wire.rs:1030) — One candidate's facts for one placement round.
- function `eligibility` (src/wire.rs:1050) — The eligibility verdict (§6, in the spec's own order): (a) reachability, (b) load and temperature, (c) a resident…
- function `draw` (src/wire.rs:1097) — The draw: splitmix64 over the round's seed, modulo the eligible set — uniform over the set as listed, and the…
- struct `Placement` (src/wire.rs:1300) — A placement round's full account: the eligible set in draw order, every exclusion with its reason, the draw (or the…
- function `place` (src/wire.rs:1313) — Run one placement round over gathered facts.
- function `cmd_dispatch` (src/wire.rs:1418) — `qexo dispatch [<item-address>] [--to <node>] [--check]` — §2.1, §5, §6.
- function `dispatch_app_run` (src/wire.rs:1425) — The same dispatch as a captured run (slice 7, §2.3): the `--app` driver's road — one engine, its facts rendered in…
- function `check_checkpoint_outputs` (src/wire.rs:2070) — Verify a checkpoint's cited outputs (§8): every `output:` line names a scroll, every `output-sha256:` line the hash…
- function `cmd_heartbeat` (src/wire.rs:2196) — `qexo heartbeat <item-address>` — one verb, two sides (§6).
- function `cmd_checkpoint` (src/wire.rs:2583) — `qexo checkpoint <item-address> --note <text> [--output <coordinate>]…` — executor-local (§6): a checkpoint scroll in…
- function `cmd_fail` (src/wire.rs:2734) — `qexo fail <item-address> --note <evidence>` — executor-local (§4.6): the oracle's failure transition on the attempt…
- function `recompute_bundle_sha256` (src/wire.rs:2830) — Recompute an accepted attempt's bundle hash from its stratum: the manifest verbatim (filed under the assignment's…
- function `declared_modules` (src/wiring.rs:28) — Module names declared by `mod <name>;` (or `pub mod <name>;`) lines in a crate root's text, in declaration order.
- function `check` (src/wiring.rs:49) — Compare a root's declarations against the module files present.
- function `check_dir` (src/wiring.rs:68) — Run the gate against a source directory: read its `main.rs`, list its `*.rs` stems, and apply `check`.
- function `crc32` (src/xz.rs:42) — Function crc32 takes data: &[u8].
- function `crc64` (src/xz.rs:69) — Function crc64 takes data: &[u8].
- function `encode_stored` (src/xz.rs:113) — Encode `data` as a complete xz stream (stored LZMA2 chunks, CRC64 block check, single block).
- function `decode_stored` (src/xz.rs:194) — Decode a stored-chunk xz stream produced by encode_stored().
- function `unseal` (src/xz.rs:246) — Unseal any xz stream: stored form decodes natively; compressed streams shell out to the system `xz` (wrap, don't fork).
- function `roman` (tools/build_bake.py:29) — Function roman takes s: str.
- function `parse_index` (tools/build_bake.py:38) — Function parse_index takes text: str.
- function `main` (tools/build_bake.py:73) — Function main takes no inputs beyond the value it is called on.
- function `sha` (tools/build_bake_remote.py:35) — Function sha takes b.
- function `head` (tools/build_bake_remote.py:38) — Function head takes repo.
- function `seal` (tools/build_bake_remote.py:42) — Function seal takes raw.
- function `advance` (tools/build_bake_remote.py:74) — Function advance takes coord.
- function `scan_markers` (tools/build_bake_remote.py:119) — Function scan_markers takes no inputs beyond the value it is called on.
- function `serialize` (tools/build_bake_remote.py:151) — Function serialize takes scrolls.
- function `main` (tools/fetch_volumen_index.py:28) — Function main takes no inputs beyond the value it is called on.

---

## function — app_session

Found in src/app.rs at line 576.

Surface: public API.

Signature: `pub fn app_session() -> i32 {`

What it does: The dispatch app session (§2.3): greeting block, then one command per line until `quit` or EOF. Entered from any dispatch verb's `--app` form. Exit codes match learn --app's shape: 0 clean, plus require_born's codes when the node is unborn; command refusals are in-band.

(Where these words came from: direct.)

---

## const — KINDS

Found in src/bridge.rs at line 41.

Surface: public API.

Signature: `pub const KINDS: [&str; 9] = [`

What it does: The kind enum (§9.1) — the model line (`tlm`, `slm`, `llm`) is in it from the first bridge slice, so the local line registers the day each can answer a probe, with no schema change waiting for it.

(Where these words came from: direct.)

---

## const — DEFAULT_FRESHNESS

Found in src/bridge.rs at line 45.

Surface: public API.

Signature: `pub const DEFAULT_FRESHNESS: u64 = 900;`

What it does: A fixed value named DEFAULT_FRESHNESS, set to 900.

(Where these words came from: inferred.)

---

## struct — Bridge

Found in src/bridge.rs at line 51.

Surface: public API.

Signature: `pub struct Bridge {`

What it does: One bridge record (Collection 5, Scroll 1 of its Book).

(Where these words came from: direct.)

---

## function — bridge_addr

Found in src/bridge.rs at line 65.

Surface: public API.

Signature: `pub fn bridge_addr(book: u32) -> Coord {`

What it does: The address of bridge Book `book`'s record scroll.

(Where these words came from: direct.)

---

## function — health_addr

Found in src/bridge.rs at line 71.

Surface: public API.

Signature: `pub fn health_addr(book: u32, scroll: u32) -> Coord {`

What it does: The address of a health-log scroll in bridge Book `book`.

(Where these words came from: direct.)

---

## function — parse_bridge

Found in src/bridge.rs at line 75.

Surface: public API.

Signature: `pub fn parse_bridge(book: u32, text: &str) -> Bridge {`

What it does: Function parse_bridge takes book: u32, text: &str. It gives back Bridge.

(Where these words came from: inferred.)

---

## function — load_bridges

Found in src/bridge.rs at line 93.

Surface: public API.

Signature: `pub fn load_bridges(px: &Phext) -> Vec<Bridge> {`

What it does: Every bridge record in the state, in Book order.

(Where these words came from: direct.)

---

## function — find_bridge

Found in src/bridge.rs at line 104.

Surface: public API.

Signature: `pub fn find_bridge(px: &Phext, name: &str) -> Option<Bridge> {`

What it does: Function find_bridge takes px: &Phext, name: &str. It gives back Option<Bridge>.

(Where these words came from: inferred.)

---

## function — brief_prefs

Found in src/bridge.rs at line 110.

Surface: public API.

Signature: `pub fn brief_prefs(brief: &str) -> Vec<String> {`

What it does: The `bridge:` preference lines of a brief, in order, duplicates removed (the brief_facts precedent).

(Where these words came from: direct.)

---

## struct — HealthRec

Found in src/bridge.rs at line 127.

Surface: public API.

Signature: `pub struct HealthRec {`

What it does: One health-log atom (a probe and its outcome).

(Where these words came from: direct.)

---

## function — load_health

Found in src/bridge.rs at line 137.

Surface: public API.

Signature: `pub fn load_health(px: &Phext, book: u32) -> Vec<HealthRec> {`

What it does: A bridge Book's health log, in scroll order.

(Where these words came from: direct.)

---

## enum — Liveness

Found in src/bridge.rs at line 162.

Surface: public API.

Signature: `pub enum Liveness {`

What it does: What the health log says about a bridge right now — calculated on every call, never stored (§9.1).

(Where these words came from: direct.)

---

## function — liveness

Found in src/bridge.rs at line 174.

Surface: public API.

Signature: `pub fn liveness(px: &Phext, bridge: &Bridge) -> Liveness {`

What it does: Function liveness takes px: &Phext, bridge: &Bridge. It gives back Liveness.

(Where these words came from: inferred.)

---

## function — health_label

Found in src/bridge.rs at line 191.

Surface: public API.

Signature: `pub fn health_label(px: &Phext, bridge: &Bridge) -> String {`

What it does: A human rendering of the calculated health, for the `qexo bridge` list and the dispatch --check report.

(Where these words came from: direct.)

---

## struct — ProbeOutcome

Found in src/bridge.rs at line 218.

Surface: public API.

Signature: `pub struct ProbeOutcome {`

What it does: A data shape named ProbeOutcome. Its fields are: pub kind: &'static str; pub ok: bool; pub detail: String; pub latency_ms: u128.

(Where these words came from: inferred.)

---

## function — probe

Found in src/bridge.rs at line 346.

Surface: public API.

Signature: `pub fn probe(endpoint: &str) -> ProbeOutcome {`

What it does: Probe an endpoint, by its shape: http/https URLs are fetched, a bare host:port is connected to, anything else is treated as a local path (a gateway path, a mailbox directory) and checked for presence.

(Where these words came from: direct.)

---

## function — append_health

Found in src/bridge.rs at line 409.

Surface: public API.

Signature: `pub fn append_health(px: &mut Phext, bridge: &Bridge, out: &ProbeOutcome) -> Coord {`

What it does: Append one probe outcome to a bridge's health log; returns the scroll's address (the probe cite routing puts in the dispatch receipt).

(Where these words came from: direct.)

---

## struct — Chosen

Found in src/bridge.rs at line 438.

Surface: public API.

Signature: `pub struct Chosen {`

What it does: A completed routing: the bridge chosen, and the probe scroll that verifies it live for this dispatch.

(Where these words came from: direct.)

---

## enum — Route

Found in src/bridge.rs at line 444.

Surface: public API.

Signature: `pub enum Route {`

What it does: A named choice named Route. It is one of: Chosen; Wait; WouldProbe; NoRoute.

(Where these words came from: inferred.)

---

## function — mid_execution_skips

Found in src/bridge.rs at line 464.

Surface: public API.

Signature: `pub fn mid_execution_skips(px: &Phext, item_s: &str, last_attempt: u32) -> Vec<String> {`

What it does: Bridges this item must not be re-routed to on its next dispatch: the `from` of a `bridge-failover` receipt filed at the item's last closed attempt — the mid-execution advance (§9.2), calculated from the ledger like everything else.

(Where these words came from: direct.)

---

## function — route

Found in src/bridge.rs at line 491.

Surface: public API.

Signature: `pub fn route( px: &mut Phext, item_s: &str, attempt: u32, prefs: &[String], skip: &[String], probe_now: bool, ) -> Route {`

What it does: Walk an item's bridge preferences (§9.2). With `probe_now`, stale or failed evidence is probed on the spot (health scrolls and failover receipts are appended into `px`; the caller saves). Without it (dispatch --check, byte-inert) the walk reads the evidence as it stands and writes nothing.

(Where these words came from: direct.)

---

## function — cmd_bridge

Found in src/bridge.rs at line 757.

Surface: public API.

Signature: `pub fn cmd_bridge(args: &[String]) -> i32 {`

What it does: Function cmd_bridge takes args: &[String]. It gives back i32.

(Where these words came from: inferred.)

---

## enum — Command

Found in src/cli.rs at line 12.

Surface: public API.

Signature: `pub enum Command {`

What it does: A named choice named Command. It is one of: Help; Version; WiringCheck; FileOp; Config; Doctor; List; Learn; Inventory; DistributedBuilds; Register; Release; Update; Evolve; Enqueue; Queue; Pause; Resume; Cancel; Audit; Bridge; Drain; Dispatch; Heartbeat; Checkpoint; Fail; Show; Host; Unhost; Message; Status; Plan; Serve; Bare; Unknown.

(Where these words came from: inferred.)

---

## function — parse

Found in src/cli.rs at line 52.

Surface: public API.

Signature: `pub fn parse(args: &[String]) -> Command {`

What it does: Function parse takes args: &[String]. It gives back Command.

(Where these words came from: inferred.)

---

## function — help_text

Found in src/cli.rs at line 248.

Surface: public API.

Signature: `pub fn help_text() -> String {`

What it does: Function help_text takes no inputs beyond the value it is called on. It gives back String.

(Where these words came from: inferred.)

---

## function — config

Found in src/core.rs at line 303.

Surface: public API.

Signature: `pub fn config(auto_update: bool, evolve: bool, release_signer: Option<String>) -> i32 {`

What it does: Function config takes auto_update: bool, evolve: bool, release_signer: Option<String>. It gives back i32.

(Where these words came from: inferred.)

---

## function — register

Found in src/core.rs at line 707.

Surface: public API.

Signature: `pub fn register(args: &[String]) -> i32 {`

What it does: Function register takes args: &[String]. It gives back i32.

(Where these words came from: inferred.)

---

## function — list

Found in src/core.rs at line 1250.

Surface: public API.

Signature: `pub fn list(json: bool) -> i32 {`

What it does: Function list takes json: bool. It gives back i32.

(Where these words came from: inferred.)

---

## function — host

Found in src/core.rs at line 1481.

Surface: public API.

Signature: `pub fn host(persona: Option<String>, boot: Option<String>) -> i32 {`

What it does: Function host takes persona: Option<String>, boot: Option<String>. It gives back i32.

(Where these words came from: inferred.)

---

## function — unhost

Found in src/core.rs at line 1701.

Surface: public API.

Signature: `pub fn unhost(name: &str) -> i32 {`

What it does: Function unhost takes name: &str. It gives back i32.

(Where these words came from: inferred.)

---

## function — message

Found in src/core.rs at line 1738.

Surface: public API.

Signature: `pub fn message(persona: &str, text: &str) -> i32 {`

What it does: Function message takes persona: &str, text: &str. It gives back i32.

(Where these words came from: inferred.)

---

## function — status

Found in src/core.rs at line 1805.

Surface: public API.

Signature: `pub fn status() -> i32 {`

What it does: Function status takes no inputs beyond the value it is called on. It gives back i32.

(Where these words came from: inferred.)

---

## function — plan_cmd

Found in src/core.rs at line 2064.

Surface: public API.

Signature: `pub fn plan_cmd() -> i32 {`

What it does: Function plan_cmd takes no inputs beyond the value it is called on. It gives back i32.

(Where these words came from: inferred.)

---

## function — serve

Found in src/core.rs at line 2211.

Surface: public API.

Signature: `pub fn serve(port: u16) -> i32 {`

What it does: Function serve takes port: u16. It gives back i32.

(Where these words came from: inferred.)

---

## function — release

Found in src/core.rs at line 2630.

Surface: public API.

Signature: `pub fn release() -> i32 {`

What it does: `qexo release` — on the signer: package the running binary into the releases dir, write + sign the manifest scroll, print what was announced.

(Where these words came from: direct.)

---

## function — update

Found in src/core.rs at line 2750.

Surface: public API.

Signature: `pub fn update(check_only: bool) -> i32 {`

What it does: `qexo update [--check]` — poll the trusted peers, judge every offer by the trust rule, and apply the trusted one.

(Where these words came from: direct.)

---

## function — evolve

Found in src/core.rs at line 3707.

Surface: public API.

Signature: `pub fn evolve(check_only: bool) -> i32 {`

What it does: `qexo evolve [--check]` — one convergence pass over the phextnet. Manual runs always converge, whatever the `evolve` config key says — the key only steers the auto timer's trigger (see `qexo config --evolve`).

(Where these words came from: direct.)

---

## struct — Gpg

Found in src/crypto.rs at line 59.

Surface: public API.

Signature: `pub struct Gpg {`

What it does: The node's keyring: a handle on the keyring directory, nothing more — key material is read from the store on each operation, so a `Gpg` never holds stale keys.

(Where these words came from: direct.)

---

## function — Gpg::open_in

Found in src/crypto.rs at line 116.

Surface: public API.

Signature: `pub fn open_in(state_dir: &Path) -> Gpg {`

What it does: Root the keyring at a state DIRECTORY (the state file's parent): the gnupg home is created beside the state file if needed, mode 0700 on Unix. Used by every caller that already knows where the state lives.

(Where these words came from: direct.)

---

## function — Gpg::generate

Found in src/crypto.rs at line 256.

Surface: public API.

Signature: `pub fn generate(&self, name: &str, kind_label: &str) -> Result<String, String> {`

What it does: Generate this node's key: an Ed25519 signing key whose user id is `name (kind_label)` — the same shape the gpg era's batch spec produced. Unprotected, as qexo node keys have always been. Idempotent: a node that already has a secret key keeps it — and a node whose GnuPG-era key cannot be migrated gets the migration error here, never a silent replacement key.

(Where these words came from: direct.)

---

## function — Gpg::list_fingerprints

Found in src/crypto.rs at line 300.

Surface: public API.

Signature: `pub fn list_fingerprints(&self) -> Result<Vec<String>, String> {`

What it does: The fingerprints of the secret keys this keyring holds — in practice the node's own, exactly one.

(Where these words came from: direct.)

---

## function — Gpg::export_public

Found in src/crypto.rs at line 315.

Surface: public API.

Signature: `pub fn export_public(&self, fingerprint: &str) -> Result<String, String> {`

What it does: The armored public key for a fingerprint in the keyring.

(Where these words came from: direct.)

---

## function — Gpg::fingerprint_of

Found in src/crypto.rs at line 322.

Surface: public API.

Signature: `pub fn fingerprint_of(&self, armored: &str) -> Result<String, String> {`

What it does: The fingerprint of armored public key material, without importing it.

(Where these words came from: direct.)

---

## function — Gpg::name_of

Found in src/crypto.rs at line 330.

Surface: public API.

Signature: `pub fn name_of(&self, armored: &str) -> Result<String, String> {`

What it does: The user id (name) carried by armored public key material — what gpg's colon listing used to report and `register` derives a peer's host from.

(Where these words came from: direct.)

---

## function — Gpg::import_public

Found in src/crypto.rs at line 336.

Surface: public API.

Signature: `pub fn import_public(&self, armored: &str) -> Result<String, String> {`

What it does: Import an armored public key into the keyring.

(Where these words came from: direct.)

---

## function — Gpg::sign_detached

Found in src/crypto.rs at line 346.

Surface: public API.

Signature: `pub fn sign_detached(&self, fingerprint: &str, data: &[u8]) -> Result<String, String> {`

What it does: A detached, armored signature over `data` by the named key (which must be this node's own — the only secret key a keyring holds).

(Where these words came from: direct.)

---

## function — Gpg::verify_detached

Found in src/crypto.rs at line 370.

Surface: public API.

Signature: `pub fn verify_detached(&self, armored_sig: &str, data: &[u8]) -> Result<String, String> {`

What it does: Verify a detached armored signature over `data` against every public key the keyring holds; on success returns the signing key's fingerprint — the VALIDSIG fingerprint the update path's trust check compares.

(Where these words came from: direct.)

---

## function — Gpg::encrypt_to

Found in src/crypto.rs at line 395.

Surface: public API.

Signature: `pub fn encrypt_to( &self, recipients: &[String], signer: Option<&str>, data: &[u8], ) -> Result<String, String> {`

What it does: Encrypt `data` to every recipient fingerprint (all must be in the keyring), optionally signing with this node's own secret key first. The signed-then-sealed persona keychain exchange (DESIGN §4.7).

(Where these words came from: direct.)

---

## function — Gpg::decrypt

Found in src/crypto.rs at line 431.

Surface: public API.

Signature: `pub fn decrypt(&self, armored: &str) -> Result<Vec<u8>, String> {`

What it does: Decrypt an armored message with this node's own key.

(Where these words came from: direct.)

---

## struct — RunLines

Found in src/dispatch.rs at line 54.

Surface: public API.

Signature: `pub struct RunLines {`

What it does: One verb run's rendered lines, split by stream, plus the structured facts a second driver renders in its own shape (slice 7, spec §2.3). A verb core fills a RunLines and never prints directly: in `live` mode each line is also printed as it is produced — the CLI driver's mode, byte- and order-identical to the verb's historic printing — while the `--app` driver captures the run and renders `facts` as its `key: value` block instead. The core is one; the drivers can never disagree about what a verb did, only about how it is rendered.

(Where these words came from: direct.)

---

## function — RunLines::new

Found in src/dispatch.rs at line 63.

Surface: public API.

Signature: `pub fn new() -> RunLines {`

What it does: Capturing mode: lines are collected, nothing printed.

(Where these words came from: direct.)

---

## function — RunLines::live

Found in src/dispatch.rs at line 69.

Surface: public API.

Signature: `pub fn live() -> RunLines {`

What it does: Live mode: lines print as produced (and are still collected, for callers that want the record).

(Where these words came from: direct.)

---

## function — RunLines::out

Found in src/dispatch.rs at line 73.

Surface: public API.

Signature: `pub fn out(&mut self, line: String) {`

What it does: Function RunLines::out takes &mut self, line: String. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — RunLines::err

Found in src/dispatch.rs at line 80.

Surface: public API.

Signature: `pub fn err(&mut self, line: String) {`

What it does: Function RunLines::err takes &mut self, line: String. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — RunLines::fact

Found in src/dispatch.rs at line 87.

Surface: public API.

Signature: `pub fn fact(&mut self, key: &str, value: &str) {`

What it does: Function RunLines::fact takes &mut self, key: &str, value: &str. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — item_record

Found in src/dispatch.rs at line 166.

Surface: public API.

Signature: `pub fn item_record( title: &str, coordinate: Option<&str>, register: Option<&str>, origin: &str, enqueued: &str, enqueued_by: &str, goal: &str, value: Option<u32>, cost: Option<u32>, after: Option<&str>, exit: Option<&str>, ) -> String {`

What it does: Build the item record scroll (spec §3.2). Field order is the spec's; absent optional fields are omitted, never blanked — renderings print their absence (`coordinate: unminted`, `roi: unestimated`) from the omission.

(Where these words came from: direct.)

---

## function — item_addr

Found in src/dispatch.rs at line 213.

Surface: public API.

Signature: `pub fn item_addr(book: u32) -> Coord {`

What it does: The address of item Book `book`'s record scroll.

(Where these words came from: direct.)

---

## function — brief_addr

Found in src/dispatch.rs at line 218.

Surface: public API.

Signature: `pub fn brief_addr(book: u32) -> Coord {`

What it does: The address of item Book `book`'s brief scroll.

(Where these words came from: direct.)

---

## function — enqueue

Found in src/dispatch.rs at line 222.

Surface: public API.

Signature: `pub fn enqueue(args: &[String]) -> i32 {`

What it does: Function enqueue takes args: &[String]. It gives back i32.

(Where these words came from: inferred.)

---

## function — enqueue_core

Found in src/dispatch.rs at line 317.

Surface: public API.

Signature: `pub fn enqueue_core( lat: &mut Lattice, parsed: &EnqueueArgs, brief: &str, ) -> Result<String, (String, i32)> {`

What it does: The enqueue act itself (§3.2, §4.1), over a loaded lattice: the title derivation, the F6 burn:never refusal (§9.2), the item record, the save. Write- before-ack lives here — the item's address is returned only after the save that carries the record completes, so an address in hand is proof the record is durable. The CLI verb above and the `--app` driver (slice 7, §2.3) both end here: an item is born identically whichever surface filed it. Errors carry the CLI's message (without its `qexo enqueue:` prefix) and the exit code the CLI returns for them.

(Where these words came from: direct.)

---

## function — show

Found in src/dispatch.rs at line 394.

Surface: public API.

Signature: `pub fn show(addr_s: &str) -> i32 {`

What it does: Function show takes addr_s: &str. It gives back i32.

(Where these words came from: inferred.)

---

## enum — ItemState

Found in src/dispatch.rs at line 454.

Surface: public API.

Signature: `pub enum ItemState {`

What it does: The eight queue states (spec §4.6). `dispatched` = bundle accepted, execution not yet evidenced; `waiting` = blocked on Will, cites its register number, dispatch skips it; `paused` = manual; `paused-auto` = the §8 auto-pause after the third consecutive failure.

(Where these words came from: direct.)

---

## const — ALL_STATES

Found in src/dispatch.rs at line 465.

Surface: public API.

Signature: `pub const ALL_STATES: [ItemState; 8] = [`

What it does: A fixed value named ALL_STATES, set to [.

(Where these words came from: inferred.)

---

## function — ItemState::parse

Found in src/dispatch.rs at line 477.

Surface: public API.

Signature: `pub fn parse(s: &str) -> Option<ItemState> {`

What it does: Function ItemState::parse takes s: &str. It gives back Option<ItemState>.

(Where these words came from: inferred.)

---

## function — ItemState::name

Found in src/dispatch.rs at line 491.

Surface: public API.

Signature: `pub fn name(self) -> &'static str {`

What it does: Function ItemState::name takes no inputs beyond the value it is called on. It gives back &'static str.

(Where these words came from: inferred.)

---

## function — ItemState::is_terminal

Found in src/dispatch.rs at line 504.

Surface: public API.

Signature: `pub fn is_terminal(self) -> bool {`

What it does: Function ItemState::is_terminal takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## enum — Actor

Found in src/dispatch.rs at line 519.

Surface: public API.

Signature: `pub enum Actor {`

What it does: Who performs a transition (spec §4.6 names an actor per transition; actors are checked, not trusted — illegal combination 4 is a cancel by the wrong actor). The CLI verbs act as Manual (pause/resume) and WillEmise (cancel: the operator at their own node). The remaining kinds are constructed by the transition engine's callers in slices 3–5 (drain, wire dispatch, F5) — the table below is the one oracle they will all pass through.

(Where these words came from: direct.)

---

## struct — Item

Found in src/dispatch.rs at line 536.

Surface: public API.

Signature: `pub struct Item {`

What it does: A queue item record (§3.2), parsed. Raw field strings ride beside the parsed values so the doctor can flag a corrupt count or state instead of silently reckoning it as zero. Every §3.2 field is parsed even where slice 2 does not read it yet — the record's shape is the contract, and slices 3–4 read the rest.

(Where these words came from: direct.)

---

## function — parse_item

Found in src/dispatch.rs at line 571.

Surface: public API.

Signature: `pub fn parse_item(book: u32, text: &str) -> Item {`

What it does: Function parse_item takes book: u32, text: &str. It gives back Item.

(Where these words came from: inferred.)

---

## function — load_items

Found in src/dispatch.rs at line 607.

Surface: public API.

Signature: `pub fn load_items(px: &phext::Phext) -> Vec<Item> {`

What it does: Every queue item in the document, in Book (arrival) order.

(Where these words came from: direct.)

---

## function — local_actor

Found in src/dispatch.rs at line 621.

Surface: public API.

Signature: `pub fn local_actor(px: &phext::Phext) -> String {`

What it does: This node's actor string — node name + full fingerprint, the shape `enqueued-by` already carries (receipts cite actors the same way, §4.2).

(Where these words came from: direct.)

---

## struct — Transition

Found in src/dispatch.rs at line 653.

Surface: public API.

Signature: `pub struct Transition {`

What it does: A transition request (spec §4.6). `evidence` is the note, the failure evidence, or the resolution evidence, per the transition; `checked`/`showed`/`verdict` are the F4 live-surface evidence shape (§7) a `done` requires; `register` supplies the register cite a move to `waiting` needs when the record does not already carry one; `manual` distinguishes a human resume from any automatic one — `paused-auto` yields only to the manual kind (§8).

(Where these words came from: direct.)

---

## function — Transition::new

Found in src/dispatch.rs at line 671.

Surface: public API.

Signature: `pub fn new(to: ItemState, actor: Actor, actor_name: &str) -> Transition {`

What it does: Function Transition::new takes to: ItemState, actor: Actor, actor_name: &str. It gives back Transition.

(Where these words came from: inferred.)

---

## struct — Plan

Found in src/dispatch.rs at line 692.

Surface: public API.

Signature: `pub struct Plan {`

What it does: What a legal transition will do, before anything is written: the state it lands in (a third consecutive failure lands in `paused-auto`, not `queued`, §8), the counters after it, the register cite it sets, and the audit it owes (§7.2), if any.

(Where these words came from: direct.)

---

## enum — AuditOwed

Found in src/dispatch.rs at line 701.

Surface: public API.

Signature: `pub enum AuditOwed {`

What it does: A named choice named AuditOwed. It is one of: NonPingClose; AutoPause; Override; RegisterClose.

(Where these words came from: inferred.)

---

## function — check_transition

Found in src/dispatch.rs at line 716.

Surface: public API.

Signature: `pub fn check_transition(item: &Item, t: &Transition) -> Result<Plan, String> {`

What it does: The legality oracle (spec §4.6): is this transition allowed from the item's current state, by this actor, with this evidence? Every refusal names the rule it breaks. Pure — it reads the item and the request and decides; `apply_transition` is the only writer.

(Where these words came from: direct.)

---

## function — append_audit

Found in src/dispatch.rs at line 878.

Surface: public API.

Signature: `pub fn append_audit( px: &mut phext::Phext, kind: &str, item: &str, checked: &str, showed: &str, verdict: &str, actor: &str, ) -> Coord {`

What it does: Append an audit scroll to the audits run (6.6.6, Collection 9, spec §7.1) and return its coordinate. The record is the spec's shape: a header line, then fields.

(Where these words came from: direct.)

---

## function — apply_transition

Found in src/dispatch.rs at line 911.

Surface: public API.

Signature: `pub fn apply_transition(lat: &mut Lattice, book: u32, t: &Transition) -> Result<Coord, String> {`

What it does: Apply a transition: the item's new state, its receipt, and its audit (where one is owed) are mutated into the document together; the CALLER saves once — so the save either contains all of it or none of it (§4.2's one-write rule), and a verb acks only after that save (§4.1). Returns the receipt's coordinate.

(Where these words came from: direct.)

---

## struct — TransitionReceipt

Found in src/dispatch.rs at line 1058.

Surface: public API.

Signature: `pub struct TransitionReceipt {`

What it does: A transition receipt (6.6.6, Collection 2, action: transition), parsed — the doctor's evidence (§4.5).

(Where these words came from: direct.)

---

## function — load_transition_receipts

Found in src/dispatch.rs at line 1067.

Surface: public API.

Signature: `pub fn load_transition_receipts(px: &phext::Phext) -> Vec<TransitionReceipt> {`

What it does: Function load_transition_receipts takes px: &phext::Phext. It gives back Vec<TransitionReceipt>.

(Where these words came from: inferred.)

---

## struct — AuditRec

Found in src/dispatch.rs at line 1090.

Surface: public API.

Signature: `pub struct AuditRec {`

What it does: An audit record (6.6.6, Collection 9, §7.1), parsed.

(Where these words came from: direct.)

---

## function — load_audits

Found in src/dispatch.rs at line 1101.

Surface: public API.

Signature: `pub fn load_audits(px: &phext::Phext) -> Vec<AuditRec> {`

What it does: Function load_audits takes px: &phext::Phext. It gives back Vec<AuditRec>.

(Where these words came from: inferred.)

---

## struct — AssignmentRec

Found in src/dispatch.rs at line 1130.

Surface: public API.

Signature: `pub struct AssignmentRec {`

What it does: An assignment record (11.11.11, Collection 3, §3.1), parsed leniently — slice 4 writes these; the doctor reads them now. An assignment is LIVE while it carries no `status:` line: the status line is its retirement marker (§3.1: "an assignment Book retires (status line) when its attempt closes").

(Where these words came from: direct.)

---

## function — load_assignments

Found in src/dispatch.rs at line 1142.

Surface: public API.

Signature: `pub fn load_assignments(px: &phext::Phext) -> Vec<AssignmentRec> {`

What it does: Function load_assignments takes px: &phext::Phext. It gives back Vec<AssignmentRec>.

(Where these words came from: inferred.)

---

## function — is_sha256_hex

Found in src/dispatch.rs at line 1166.

Surface: public API.

Signature: `pub fn is_sha256_hex(s: &str) -> bool {`

What it does: A SHA-256 digest's shape: 64 hex characters. (The digest's truth against the canonical bundle is slice 4's check, where the bundle builder lands; the shape is what a hand-corrupted field breaks today.)

(Where these words came from: direct.)

---

## function — daily_goal

Found in src/dispatch.rs at line 1178.

Surface: public API.

Signature: `pub fn daily_goal(px: &phext::Phext) -> Option<String> {`

What it does: The node's current daily goal: a field on the config record (`daily-goal:` in 9.9.9, Collection 2), set at the morning synthesis. Absent = no daily class today.

(Where these words came from: direct.)

---

## function — goal_class

Found in src/dispatch.rs at line 1186.

Surface: public API.

Signature: `pub fn goal_class(item: &Item, daily_goal: Option<&str>) -> u8 {`

What it does: The §5.1 goal class: 0 = names the current daily goal, 1 = standing, 2 = background (names some other goal).

(Where these words came from: direct.)

---

## function — goal_class_name

Found in src/dispatch.rs at line 1199.

Surface: public API.

Signature: `pub fn goal_class_name(class: u8) -> &'static str {`

What it does: Function goal_class_name takes class: u8. It gives back &'static str.

(Where these words came from: inferred.)

---

## function — order_items

Found in src/dispatch.rs at line 1267.

Surface: public API.

Signature: `pub fn order_items(items: Vec<Item>, daily: Option<&str>) -> Vec<Item> {`

What it does: The queue's full order (spec §5 + §2.1): paused-auto items pin to the head, flagged, always; `pin: head` items follow, rendered as the pins they are (§5.2); `pin: after <address>` items sit immediately after their target; everything else takes its calculated position. A pin whose target is not in this rendering keeps its calculated position — the pin still renders, so the anomaly is visible rather than silent.

(Where these words came from: direct.)

---

## function — queue

Found in src/dispatch.rs at line 1351.

Surface: public API.

Signature: `pub fn queue(all: bool, node: Option<String>, explain: Option<String>) -> i32 {`

What it does: Function queue takes all: bool, node: Option<String>, explain: Option<String>. It gives back i32.

(Where these words came from: inferred.)

---

## function — pause

Found in src/dispatch.rs at line 1549.

Surface: public API.

Signature: `pub fn pause(args: &[String]) -> i32 {`

What it does: Function pause takes args: &[String]. It gives back i32.

(Where these words came from: inferred.)

---

## function — resume

Found in src/dispatch.rs at line 1558.

Surface: public API.

Signature: `pub fn resume(args: &[String]) -> i32 {`

What it does: Function resume takes args: &[String]. It gives back i32.

(Where these words came from: inferred.)

---

## function — cancel

Found in src/dispatch.rs at line 1566.

Surface: public API.

Signature: `pub fn cancel(args: &[String]) -> i32 {`

What it does: Function cancel takes args: &[String]. It gives back i32.

(Where these words came from: inferred.)

---

## function — audit

Found in src/dispatch.rs at line 1632.

Surface: public API.

Signature: `pub fn audit(args: &[String]) -> i32 {`

What it does: `qexo audit` (spec §2.1, §7.2): Will's window on the audit cluster — list with filters, newest last (ledger order); `audit show` renders one in full. If it isn't in the audit cluster, it didn't happen quietly.

(Where these words came from: direct.)

---

## function — report

Found in src/distbuilds.rs at line 129.

Surface: public API.

Signature: `pub fn report() -> i32 {`

What it does: The verification half, shared by the upgrade run and status: every line is a fact probed now, nothing remembered.

(Where these words came from: direct.)

---

## function — distributed_builds

Found in src/distbuilds.rs at line 192.

Surface: public API.

Signature: `pub fn distributed_builds(scheduler: bool, status: bool, branch: Option<String>) -> i32 {`

What it does: Function distributed_builds takes scheduler: bool, status: bool, branch: Option<String>. It gives back i32.

(Where these words came from: inferred.)

---

## struct — Finding

Found in src/doctor.rs at line 22.

Surface: public API.

Signature: `pub struct Finding {`

What it does: A data shape named Finding. Its fields are: pub item: String; pub ok: bool; pub detail: String; pub fixable: bool.

(Where these words came from: inferred.)

---

## struct — Report

Found in src/doctor.rs at line 29.

Surface: public API.

Signature: `pub struct Report {`

What it does: A data shape named Report. Its fields are: pub findings: Vec<Finding>.

(Where these words came from: inferred.)

---

## function — Report::ok

Found in src/doctor.rs at line 34.

Surface: public API.

Signature: `pub fn ok(&self) -> bool {`

What it does: Function Report::ok takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — Report::problems

Found in src/doctor.rs at line 37.

Surface: public API.

Signature: `pub fn problems(&self) -> usize {`

What it does: Function Report::problems takes no inputs beyond the value it is called on. It gives back usize.

(Where these words came from: inferred.)

---

## function — Report::headline

Found in src/doctor.rs at line 42.

Surface: public API.

Signature: `pub fn headline(&self) -> String {`

What it does: One-line headline for `status`: "doctor ok" or "doctor: N problem(s)".

(Where these words came from: direct.)

---

## function — Report::render

Found in src/doctor.rs at line 49.

Surface: public API.

Signature: `pub fn render(&self) -> String {`

What it does: Function Report::render takes no inputs beyond the value it is called on. It gives back String.

(Where these words came from: inferred.)

---

## function — check

Found in src/doctor.rs at line 164.

Surface: public API.

Signature: `pub fn check(state_dir: &Path, state_file: &Path) -> Report {`

What it does: Function check takes state_dir: &Path, state_file: &Path. It gives back Report.

(Where these words came from: inferred.)

---

## function — fix

Found in src/doctor.rs at line 269.

Surface: public API.

Signature: `pub fn fix(state_dir: &Path, state_file: &Path) -> Vec<String> {`

What it does: Apply the filesystem repairs: create the state directory, tighten the keyring mode, remove a stale lock. Returns the human list of what was done. Never creates the state file.

(Where these words came from: direct.)

---

## function — check_lattice

Found in src/doctor.rs at line 320.

Surface: public API.

Signature: `pub fn check_lattice(lat: &Lattice) -> Report {`

What it does: Check the lattice layer of an EXISTING state document: home index and homeland indexes exist; every record is named by its homeland's index; no index line names a ghost.

(Where these words came from: direct.)

---

## function — fix_lattice

Found in src/doctor.rs at line 407.

Surface: public API.

Signature: `pub fn fix_lattice(lat: &mut Lattice) -> Vec<String> {`

What it does: Repair the lattice layer: regenerate every index scroll from the records. Returns the human list of what changed.

(Where these words came from: direct.)

---

## function — check_queue

Found in src/doctor.rs at line 485.

Surface: public API.

Signature: `pub fn check_queue(lat: &Lattice) -> Report {`

What it does: Doctor's queue section. The §4.5 counts: items by state; terminal items lacking a receipt (must be 0); receipts citing no live item (must be 0); assignments whose heartbeat is stale (counted here; since slice 5 the F5 path acts on them — a stale assignment for an item homed here with no detection on file is a finding, and the failure counter is checked against its receipted history, §8); an audit record for every non-ping closure (a closure without one is a finding, §7.2 — a register-close audit satisfies it for a register-driven closure, §10.3). Slice 3 adds the register link's integrity: a register-born item with no register-seq, a register-seq carried by two items, and the pending write-back count (visible, never a failure — the drain's sweep retries them, §10.4). The §4.6 illegal combinations are checked one by one, each in its own words. No queue finding is fixable by doctor: the repair is a transition or Will's hand, never an invention — a nonzero must-be-0 count is a doctor failure of the same class as a missing homeland index (§4.5).

(Where these words came from: direct.)

---

## function — sha256

Found in src/hash.rs at line 17.

Surface: public API.

Signature: `pub fn sha256(data: &[u8]) -> [u8; 32] {`

What it does: Function sha256 takes data: &[u8]. It gives back [u8; 32].

(Where these words came from: inferred.)

---

## function — sha256_hex

Found in src/hash.rs at line 62.

Surface: public API.

Signature: `pub fn sha256_hex(data: &[u8]) -> String {`

What it does: Function sha256_hex takes data: &[u8]. It gives back String.

(Where these words came from: inferred.)

---

## function — hex

Found in src/hash.rs at line 66.

Surface: public API.

Signature: `pub fn hex(bytes: &[u8]) -> String {`

What it does: Function hex takes bytes: &[u8]. It gives back String.

(Where these words came from: inferred.)

---

## enum — InferError

Found in src/infer.rs at line 20.

Surface: public API.

Signature: `pub enum InferError {`

What it does: A named choice named InferError. It is one of: Unreachable; ModelAbsent; BadResponse; Refused.

(Where these words came from: inferred.)

---

## struct — Infer

Found in src/infer.rs at line 42.

Surface: public API.

Signature: `pub struct Infer {`

What it does: A data shape named Infer. Its fields are: pub endpoint: String; pub model: String.

(Where these words came from: inferred.)

---

## const — DEFAULT_ENDPOINT

Found in src/infer.rs at line 49.

Surface: public API.

Signature: `pub const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:11434";`

What it does: A fixed value named DEFAULT_ENDPOINT, set to "http://127.0.0.1:11434".

(Where these words came from: inferred.)

---

## const — DEFAULT_MODEL

Found in src/infer.rs at line 50.

Surface: public API.

Signature: `pub const DEFAULT_MODEL: &str = "gemma4:e4b";`

What it does: A fixed value named DEFAULT_MODEL, set to "gemma4:e4b".

(Where these words came from: inferred.)

---

## function — Infer::from_env

Found in src/infer.rs at line 57.

Surface: public API.

Signature: `pub fn from_env() -> Infer {`

What it does: From the environment: QEXO_OLLAMA_URL, QEXO_MODEL. (The lattice Config family will hold these as field scrolls when the Layer 2 map lands; env is the dev valve, as with QEXO_STATE.)

(Where these words came from: direct.)

---

## function — Infer::new

Found in src/infer.rs at line 69.

Surface: public API.

Signature: `pub fn new(endpoint: &str, model: &str) -> Infer {`

What it does: Function Infer::new takes endpoint: &str, model: &str. It gives back Infer.

(Where these words came from: inferred.)

---

## function — Infer::tags

Found in src/infer.rs at line 77.

Surface: public API.

Signature: `pub fn tags(&self) -> Result<Vec<String>, InferError> {`

What it does: Model names the endpoint currently serves (GET /api/tags).

(Where these words came from: direct.)

---

## function — Infer::require_model

Found in src/infer.rs at line 83.

Surface: public API.

Signature: `pub fn require_model(&self) -> Result<(), InferError> {`

What it does: Confirm the endpoint is up AND carries our model.

(Where these words came from: direct.)

---

## function — Infer::chat

Found in src/infer.rs at line 100.

Surface: public API.

Signature: `pub fn chat(&self, system: &str, prompt: &str, keep_alive: &str) -> Result<String, InferError> {`

What it does: One chat exchange. `system` is the persona's boot text (a scroll's content, handed across the border as a plain string); `keep_alive` follows Ollama's convention ("30m" to hold the model resident, "0" to unload).

(Where these words came from: direct.)

---

## function — Infer::prime

Found in src/infer.rs at line 126.

Surface: public API.

Signature: `pub fn prime(&self, boot_text: &str) -> Result<String, InferError> {`

What it does: Load the persona into local inference: a priming exchange with a long keep-alive, so the model stays resident with the persona's boot text as its system context.

(Where these words came from: direct.)

---

## function — Infer::unload

Found in src/infer.rs at line 134.

Surface: public API.

Signature: `pub fn unload(&self) -> Result<(), InferError> {`

What it does: Unload the model (keep_alive 0). Best-effort by design: the caller records the session closed regardless; a dead endpoint holds nothing anyway.

(Where these words came from: direct.)

---

## function — inventory

Found in src/inventory.rs at line 241.

Surface: public API.

Signature: `pub fn inventory(name: Option<String>, install: Option<String>) -> i32 {`

What it does: `qexo inventory [<name>]` / `qexo inventory --install <name>` (alias verb: catalog).

(Where these words came from: direct.)

---

## const — HL_HOME

Found in src/lattice.rs at line 64.

Surface: public API.

Signature: `pub const HL_HOME: [u32; 3] = [1, 1, 1];`

What it does: A fixed value named HL_HOME, set to [1, 1, 1].

(Where these words came from: inferred.)

---

## const — HL_KEYS

Found in src/lattice.rs at line 65.

Surface: public API.

Signature: `pub const HL_KEYS: [u32; 3] = [2, 2, 2];`

What it does: A fixed value named HL_KEYS, set to [2, 2, 2].

(Where these words came from: inferred.)

---

## const — HL_PEERS

Found in src/lattice.rs at line 66.

Surface: public API.

Signature: `pub const HL_PEERS: [u32; 3] = [3, 3, 3];`

What it does: A fixed value named HL_PEERS, set to [3, 3, 3].

(Where these words came from: inferred.)

---

## const — HL_PERSONAS

Found in src/lattice.rs at line 67.

Surface: public API.

Signature: `pub const HL_PERSONAS: [u32; 3] = [4, 4, 4];`

What it does: A fixed value named HL_PERSONAS, set to [4, 4, 4].

(Where these words came from: inferred.)

---

## const — HL_PLAN

Found in src/lattice.rs at line 68.

Surface: public API.

Signature: `pub const HL_PLAN: [u32; 3] = [5, 5, 5];`

What it does: A fixed value named HL_PLAN, set to [5, 5, 5].

(Where these words came from: inferred.)

---

## const — HL_LEDGERS

Found in src/lattice.rs at line 69.

Surface: public API.

Signature: `pub const HL_LEDGERS: [u32; 3] = [6, 6, 6];`

What it does: A fixed value named HL_LEDGERS, set to [6, 6, 6].

(Where these words came from: inferred.)

---

## const — HL_MAIL

Found in src/lattice.rs at line 70.

Surface: public API.

Signature: `pub const HL_MAIL: [u32; 3] = [7, 7, 7];`

What it does: A fixed value named HL_MAIL, set to [7, 7, 7].

(Where these words came from: inferred.)

---

## const — HL_TELEMETRY

Found in src/lattice.rs at line 71.

Surface: public API.

Signature: `pub const HL_TELEMETRY: [u32; 3] = [8, 8, 8];`

What it does: A fixed value named HL_TELEMETRY, set to [8, 8, 8].

(Where these words came from: inferred.)

---

## const — HL_CONFIG

Found in src/lattice.rs at line 72.

Surface: public API.

Signature: `pub const HL_CONFIG: [u32; 3] = [9, 9, 9];`

What it does: A fixed value named HL_CONFIG, set to [9, 9, 9].

(Where these words came from: inferred.)

---

## const — HL_LEARN

Found in src/lattice.rs at line 77.

Surface: public API.

Signature: `pub const HL_LEARN: [u32; 3] = [10, 10, 10];`

What it does: Lessons & learning (v1.4.0): the tens — practice, repeated. The anchor pattern extends by one: where the nine ones-to-nines homelands were the node's records of the world, the tens are the node's record of its operator's practice.

(Where these words came from: direct.)

---

## const — LEARN_DECK_C

Found in src/lattice.rs at line 81.

Surface: public API.

Signature: `pub const LEARN_DECK_C: u32 = 2;`

What it does: The curriculum deck's cluster inside the learn homeland: one Book per card, grown by adjacency (Layer 2 growth rule).

(Where these words came from: direct.)

---

## const — HL_DISPATCH

Found in src/lattice.rs at line 87.

Surface: public API.

Signature: `pub const HL_DISPATCH: [u32; 3] = [11, 11, 11];`

What it does: Dispatch & queue (dispatch core slice 1, spec §3.1): the elevens — send-off country. The pattern extends by one again: the elevens are the node's record of work in flight — what was queued, what is assigned, what runs where.

(Where these words came from: direct.)

---

## const — DISPATCH_ITEMS_C

Found in src/lattice.rs at line 95.

Surface: public API.

Signature: `pub const DISPATCH_ITEMS_C: u32 = 2;`

What it does: Clusters (Collection) inside dispatch & queue, in the spec's order: queue items first (the ledger family — one Book per item in arrival sequence), then assignments, schedules, bridges, and the register link. Slice 1 lays down the homeland and the items cluster; the rest take their clusters as their slices land.

(Where these words came from: direct.)

---

## const — DISPATCH_ASSIGN_C

Found in src/lattice.rs at line 96.

Surface: public API.

Signature: `pub const DISPATCH_ASSIGN_C: u32 = 3;`

What it does: A fixed value named DISPATCH_ASSIGN_C, set to 3.

(Where these words came from: inferred.)

---

## const — DISPATCH_SCHED_C

Found in src/lattice.rs at line 97.

Surface: public API.

Signature: `pub const DISPATCH_SCHED_C: u32 = 4;`

What it does: A fixed value named DISPATCH_SCHED_C, set to 4.

(Where these words came from: inferred.)

---

## const — DISPATCH_BRIDGES_C

Found in src/lattice.rs at line 98.

Surface: public API.

Signature: `pub const DISPATCH_BRIDGES_C: u32 = 5;`

What it does: A fixed value named DISPATCH_BRIDGES_C, set to 5.

(Where these words came from: inferred.)

---

## const — DISPATCH_REGISTER_C

Found in src/lattice.rs at line 99.

Surface: public API.

Signature: `pub const DISPATCH_REGISTER_C: u32 = 6;`

What it does: A fixed value named DISPATCH_REGISTER_C, set to 6.

(Where these words came from: inferred.)

---

## const — HOME

Found in src/lattice.rs at line 102.

Surface: public API.

Signature: `pub const HOME: Coord = [1, 1, 1, 1, 1, 1, 1, 1, 1];`

What it does: The home scroll: prose index of the whole state file.

(Where these words came from: direct.)

---

## const — IDENTITY

Found in src/lattice.rs at line 104.

Surface: public API.

Signature: `pub const IDENTITY: Coord = [1, 1, 1, 1, 1, 1, 1, 1, 2];`

What it does: Node identity record, adjacent to home in the home run.

(Where these words came from: direct.)

---

## const — KEYS_INDEX_C

Found in src/lattice.rs at line 109.

Surface: public API.

Signature: `pub const KEYS_INDEX_C: u32 = 1;`

What it does: Clusters (Collection) inside keys & trust (anchor 2.2.2/2.2.2/2.2.2), neighboring runs in the order the brief names them: the node key first, then persona keys, peer keys, grants, delegations, policy.

(Where these words came from: direct.)

---

## const — KEYS_NODE_C

Found in src/lattice.rs at line 110.

Surface: public API.

Signature: `pub const KEYS_NODE_C: u32 = 2;`

What it does: A fixed value named KEYS_NODE_C, set to 2.

(Where these words came from: inferred.)

---

## const — KEYS_PERSONA_C

Found in src/lattice.rs at line 111.

Surface: public API.

Signature: `pub const KEYS_PERSONA_C: u32 = 3;`

What it does: A fixed value named KEYS_PERSONA_C, set to 3.

(Where these words came from: inferred.)

---

## const — KEYS_PEER_C

Found in src/lattice.rs at line 112.

Surface: public API.

Signature: `pub const KEYS_PEER_C: u32 = 4;`

What it does: A fixed value named KEYS_PEER_C, set to 4.

(Where these words came from: inferred.)

---

## const — KEYS_GRANTS_C

Found in src/lattice.rs at line 113.

Surface: public API.

Signature: `pub const KEYS_GRANTS_C: u32 = 5;`

What it does: A fixed value named KEYS_GRANTS_C, set to 5.

(Where these words came from: inferred.)

---

## const — KEYS_DELEGATIONS_C

Found in src/lattice.rs at line 114.

Surface: public API.

Signature: `pub const KEYS_DELEGATIONS_C: u32 = 6;`

What it does: A fixed value named KEYS_DELEGATIONS_C, set to 6.

(Where these words came from: inferred.)

---

## const — KEYS_POLICY_C

Found in src/lattice.rs at line 115.

Surface: public API.

Signature: `pub const KEYS_POLICY_C: u32 = 7;`

What it does: A fixed value named KEYS_POLICY_C, set to 7.

(Where these words came from: inferred.)

---

## const — NODE_KEY_RECORD

Found in src/lattice.rs at line 118.

Surface: public API.

Signature: `pub const NODE_KEY_RECORD: Coord = [2, 2, 2, KEYS_NODE_C, 1, 1, 1, 1, 1];`

What it does: The node key record and its armored public key (adjacent scroll).

(Where these words came from: direct.)

---

## const — NODE_KEY_PUBLIC

Found in src/lattice.rs at line 119.

Surface: public API.

Signature: `pub const NODE_KEY_PUBLIC: Coord = [2, 2, 2, KEYS_NODE_C, 1, 1, 1, 1, 2];`

What it does: A fixed value named NODE_KEY_PUBLIC, set to [2, 2, 2, KEYS_NODE_C, 1, 1, 1, 1, 2].

(Where these words came from: inferred.)

---

## const — LEDGER_RECEIPTS

Found in src/lattice.rs at line 122.

Surface: public API.

Signature: `pub const LEDGER_RECEIPTS: u32 = 2;`

What it does: Ledger kinds as clusters (Collection) inside 6.6.6.

(Where these words came from: direct.)

---

## const — LEDGER_TELEPORTS

Found in src/lattice.rs at line 123.

Surface: public API.

Signature: `pub const LEDGER_TELEPORTS: u32 = 3;`

What it does: A fixed value named LEDGER_TELEPORTS, set to 3.

(Where these words came from: inferred.)

---

## const — LEDGER_INSTANCES

Found in src/lattice.rs at line 124.

Surface: public API.

Signature: `pub const LEDGER_INSTANCES: u32 = 4;`

What it does: A fixed value named LEDGER_INSTANCES, set to 4.

(Where these words came from: inferred.)

---

## const — LEDGER_EMOTION

Found in src/lattice.rs at line 125.

Surface: public API.

Signature: `pub const LEDGER_EMOTION: u32 = 5;`

What it does: A fixed value named LEDGER_EMOTION, set to 5.

(Where these words came from: inferred.)

---

## const — LEDGER_GOAL_ACTIVITY

Found in src/lattice.rs at line 126.

Surface: public API.

Signature: `pub const LEDGER_GOAL_ACTIVITY: u32 = 6;`

What it does: A fixed value named LEDGER_GOAL_ACTIVITY, set to 6.

(Where these words came from: inferred.)

---

## const — LEDGER_SYNC

Found in src/lattice.rs at line 127.

Surface: public API.

Signature: `pub const LEDGER_SYNC: u32 = 7;`

What it does: A fixed value named LEDGER_SYNC, set to 7.

(Where these words came from: inferred.)

---

## const — LEDGER_RELEASES

Found in src/lattice.rs at line 133.

Surface: public API.

Signature: `pub const LEDGER_RELEASES: u32 = 8;`

What it does: Release announcements (v1.1.0): one Book per announced release — the manifest record at Scroll 1, its detached signature at Scroll 2. The manifest is the phextnet's trust object: it is what travels and what gets verified; the binary bytes ride the wire beside the state file, never inside it.

(Where these words came from: direct.)

---

## const — LEDGER_AUDITS

Found in src/lattice.rs at line 140.

Surface: public API.

Signature: `pub const LEDGER_AUDITS: u32 = 9;`

What it does: Audits (dispatch core slice 2, spec §7.1): one Book per audit — the record of what was checked and why an act was allowed to happen quietly (non-ping closures, re-orders, auto-pauses, register closes, verifications). Collections 2–8 are the shipped ledger kinds; 9 is next, assigned by the spec.

(Where these words came from: direct.)

---

## function — ledger_kind_name

Found in src/lattice.rs at line 142.

Surface: public API.

Signature: `pub fn ledger_kind_name(cluster: u32) -> Option<&'static str> {`

What it does: Function ledger_kind_name takes cluster: u32. It gives back Option<&'static str>.

(Where these words came from: inferred.)

---

## const — MAIL_SESSIONS_C

Found in src/lattice.rs at line 157.

Surface: public API.

Signature: `pub const MAIL_SESSIONS_C: u32 = 2;`

What it does: Sessions (Collection 2) and mailbox (Collection 3) inside 7.7.7.

(Where these words came from: direct.)

---

## const — MAIL_BOX_C

Found in src/lattice.rs at line 158.

Surface: public API.

Signature: `pub const MAIL_BOX_C: u32 = 3;`

What it does: A fixed value named MAIL_BOX_C, set to 3.

(Where these words came from: inferred.)

---

## const — TELEM_DISK_C

Found in src/lattice.rs at line 161.

Surface: public API.

Signature: `pub const TELEM_DISK_C: u32 = 2;`

What it does: Telemetry: disk observations (Collection 2) inside 8.8.8.

(Where these words came from: direct.)

---

## const — PLAN_CURRENT

Found in src/lattice.rs at line 166.

Surface: public API.

Signature: `pub const PLAN_CURRENT: Coord = [5, 5, 5, 2, 1, 1, 1, 1, 1];`

What it does: Plan (anchor 5.5.5/5.5.5/5.5.5): the current plan lives in Collection 2 (fields at Scroll 1, the human rendering at Scroll 2, adjacent); history is a run of Books in Collection 3, one Book per emitted plan.

(Where these words came from: direct.)

---

## const — PLAN_CURRENT_RENDER

Found in src/lattice.rs at line 167.

Surface: public API.

Signature: `pub const PLAN_CURRENT_RENDER: Coord = [5, 5, 5, 2, 1, 1, 1, 1, 2];`

What it does: A fixed value named PLAN_CURRENT_RENDER, set to [5, 5, 5, 2, 1, 1, 1, 1, 2].

(Where these words came from: inferred.)

---

## const — PLAN_HISTORY_C

Found in src/lattice.rs at line 168.

Surface: public API.

Signature: `pub const PLAN_HISTORY_C: u32 = 3;`

What it does: A fixed value named PLAN_HISTORY_C, set to 3.

(Where these words came from: inferred.)

---

## const — CONFIG_RECORD

Found in src/lattice.rs at line 175.

Surface: public API.

Signature: `pub const CONFIG_RECORD: Coord = [9, 9, 9, 2, 1, 1, 1, 1, 1];`

What it does: qexo's own config record: the node's settings as field lines — no config file anywhere else. It lives at Collection 2 of the 9.9.9/9.9.9/9.9.9 homeland; Collection 1's first scroll is the homeland index, here as in every homeland (the two must never collide: an index that reports on the config record cannot BE it).

(Where these words came from: direct.)

---

## function — at

Found in src/lattice.rs at line 183.

Surface: public API.

Signature: `pub fn at(h: [u32; 3], rest: [u32; 6]) -> Coord {`

What it does: Compose an address from a homeland and the remaining six components [Collection, Volume, Book, Chapter, Section, Scroll].

(Where these words came from: direct.)

---

## function — index_addr

Found in src/lattice.rs at line 195.

Surface: public API.

Signature: `pub fn index_addr(h: [u32; 3]) -> Coord {`

What it does: A homeland's ANCHOR: the full 9D address of its index scroll — the patterned address the region is named by (Layer 2 v2): the homeland digit in all nine components, e.g. keys & trust anchors at 2.2.2/2.2.2/2.2.2. (For the home homeland this is 1.1.1/1.1.1/1.1.1 itself — the home scroll IS its anchor.) Records live in the anchor's vicinity: they share the homeland triple and place themselves in the remaining six components, which never reach the anchor's own full-pattern address.

(Where these words came from: direct.)

---

## function — prefix_scrolls

Found in src/lattice.rs at line 201.

Surface: public API.

Signature: `pub fn prefix_scrolls<'a>(px: &'a Phext, prefix: &[u32]) -> Vec<(Coord, &'a String)> {`

What it does: Every scroll whose address starts with `prefix`, in ascending (coordinate) order — the range read of a run or a homeland.

(Where these words came from: direct.)

---

## function — component_values

Found in src/lattice.rs at line 211.

Surface: public API.

Signature: `pub fn component_values(px: &Phext, prefix: &[u32], idx: usize) -> Vec<u32> {`

What it does: Distinct values of component `idx` among scrolls under `prefix`, sorted — e.g. the Books present in a cluster.

(Where these words came from: direct.)

---

## function — next_free_book

Found in src/lattice.rs at line 222.

Surface: public API.

Signature: `pub fn next_free_book(px: &Phext, first5: [u32; 5], start: u32) -> u32 {`

What it does: The next free Book (>= start) in the cluster addressed by [Library, Shelf, Series, Collection, Volume]: the adjacency rule — a new record goes beside its kin, at the end of the family's local run.

(Where these words came from: direct.)

---

## function — next_free_cluster

Found in src/lattice.rs at line 238.

Surface: public API.

Signature: `pub fn next_free_cluster(px: &Phext, h: [u32; 3], start: u32) -> u32 {`

What it does: The next free Collection (>= start) in a homeland: a new entity cluster beside the existing ones. A cluster counts as used when anything lives under [homeland, cluster, Volume 1] — records and their runs all sit at Volume 1, while the homeland's anchor scroll (at the full-pattern address, whose Volume is the homeland digit) never masquerades as a record cluster.

(Where these words came from: direct.)

---

## function — next_free_scroll

Found in src/lattice.rs at line 250.

Surface: public API.

Signature: `pub fn next_free_scroll(px: &Phext, first8: [u32; 8], start: u32) -> Coord {`

What it does: The next free Scroll (>= start) in the run addressed by the first eight components.

(Where these words came from: direct.)

---

## function — field

Found in src/lattice.rs at line 266.

Surface: public API.

Signature: `pub fn field(text: &str, key: &str) -> Option<String> {`

What it does: Read `key` from a record scroll's `key: value` lines.

(Where these words came from: direct.)

---

## function — set_field

Found in src/lattice.rs at line 278.

Surface: public API.

Signature: `pub fn set_field(text: &str, key: &str, value: &str) -> String {`

What it does: Set `key` in a record scroll: replace its line if present, append it if not. Edits the scroll in place, as Layer 2 asks.

(Where these words came from: direct.)

---

## function — record

Found in src/lattice.rs at line 299.

Surface: public API.

Signature: `pub fn record(fields: &[(&str, &str)]) -> String {`

What it does: Build a record scroll from field lines.

(Where these words came from: direct.)

---

## struct — RecordRef

Found in src/lattice.rs at line 317.

Surface: public API.

Signature: `pub struct RecordRef {`

What it does: A data shape named RecordRef. Its fields are: pub kind: &'static str; pub name: String; pub coord: Coord; pub status: String.

(Where these words came from: inferred.)

---

## function — records

Found in src/lattice.rs at line 326.

Surface: public API.

Signature: `pub fn records(px: &Phext) -> Vec<RecordRef> {`

What it does: Enumerate peer, persona, and session records across the homelands, in address order.

(Where these words came from: direct.)

---

## function — regen_indexes

Found in src/lattice.rs at line 381.

Surface: public API.

Signature: `pub fn regen_indexes(px: &mut Phext) {`

What it does: Regenerate the home index and every homeland index scroll from the records present. Called by config at birth, by commands after every mutation, and by `doctor --fix` as its repair.

(Where these words came from: direct.)

---

## function — index_addresses

Found in src/lattice.rs at line 667.

Surface: public API.

Signature: `pub fn index_addresses() -> Vec<Coord> {`

What it does: Every index scroll the current Layer 2 layout defines: the home index plus each homeland's anchor scroll, the tens included. This is the whole set a state must carry to be whole under the running binary's layout.

(Where these words came from: direct.)

---

## function — seed_missing_indexes

Found in src/lattice.rs at line 686.

Surface: public API.

Signature: `pub fn seed_missing_indexes(px: &mut Phext) -> Vec<Coord> {`

What it does: The state top-up (v1.4.1): seed any index scroll the current layout defines that this document lacks, with exactly the text `regen_indexes` would give it. Born of the v1.4.0 fleet refusal: states born under 1.3.0 carry no lessons homeland, so the new layout's index is missing through no damage of theirs — an upgraded state becomes whole by the running binary's own hand the first time it writes. Surgical by construction: an index that EXISTS is never touched (records and indexes alike), a missing one is copied from a scratch regeneration of this same document. Idempotent — a whole state seeds nothing. Returns the addresses seeded.

(Where these words came from: direct.)

---

## function — system_state_path

Found in src/lattice.rs at line 705.

Surface: public API.

Signature: `pub fn system_state_path() -> PathBuf {`

What it does: The system state path: /var/lib/qexo/state.phext — canonical for system installs (root-run services, the ranch fleet).

(Where these words came from: direct.)

---

## function — user_state_path

Found in src/lattice.rs at line 715.

Surface: public API.

Signature: `pub fn user_state_path() -> Option<PathBuf> {`

What it does: The per-user state path (v1.4.0): $HOME/.qexo/state.phext — on Windows, under the user profile (%USERPROFILE%, falling back to HOME), resolved the same cfg way the codebase splits platforms elsewhere. A node installed by an unprivileged user (a plain `cargo install qexo`) lives here, whole: state file and GPG keyring side by side, no sudo anywhere in its life.

(Where these words came from: direct.)

---

## struct — StateFacts

Found in src/lattice.rs at line 732.

Surface: public API.

Signature: `pub struct StateFacts {`

What it does: The facts the resolution order reads. Probed from the live system by `state_path`; supplied directly by tests.

(Where these words came from: direct.)

---

## function — set_write_intent

Found in src/lattice.rs at line 767.

Surface: public API.

Signature: `pub fn set_write_intent() {`

What it does: Function set_write_intent takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — write_intent

Found in src/lattice.rs at line 771.

Surface: public API.

Signature: `pub fn write_intent() -> bool {`

What it does: Function write_intent takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — set_learn_preference

Found in src/lattice.rs at line 788.

Surface: public API.

Signature: `pub fn set_learn_preference() {`

What it does: Function set_learn_preference takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — learn_preference

Found in src/lattice.rs at line 792.

Surface: public API.

Signature: `pub fn learn_preference() -> bool {`

What it does: Function learn_preference takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — pick_state_path

Found in src/lattice.rs at line 827.

Surface: public API.

Signature: `pub fn pick_state_path(facts: StateFacts, system: &std::path::Path, user: Option<&std::path::Path>) -> PathBuf {`

What it does: The resolution order (v1.4.1), stated exactly as shipped: 1. QEXO_STATE, when set non-empty (the caller's word). 2. THE WRITE-INTENT RULE (new in v1.4.1): a write-intending verb in unprivileged hands, facing a system state it cannot write, selects the per-user path instead — existing or to be born there by config. The system state belongs to its owner; this user's node is their own. (Read-only verbs are untouched by this rule and keep the order below exactly as v1.4.0 shipped it.) 2b. THE LEARN RULE (new in v1.4.2): `qexo learn` in unprivileged hands, when a per-user state EXISTS, selects the per-user path — the deck belongs to the operator, not to the machine's owner. With no per-user state, learn falls through to the order below like any reader (v1.4.1 diverted it to the missing path and errored; v1.4.0 answered). 3. The system path, when the system state exists and this process can read it — a system install stays canonical, byte-for-byte, and read-only verbs never fail on its permissions when a per-user state exists instead (rule 4 is what they fall to). 4. The per-user path, when a per-user state exists. 5. The system path again when it exists but cannot be read and no per-user state exists — the honest error names the real file rather than inventing an empty node. 6. Nothing exists: the system path for a privileged process, the per-user path otherwise (a fresh `qexo config` as a normal user births a per-user node); with no home directory to anchor to, the system path. A node has ONE state: the resolution finds the first that exists, and `qexo status` names which path is live.

(Where these words came from: direct.)

---

## function — state_path

Found in src/lattice.rs at line 902.

Surface: public API.

Signature: `pub fn state_path() -> PathBuf {`

What it does: The node's state path, resolved by the order above.

(Where these words came from: direct.)

---

## function — birth_path

Found in src/lattice.rs at line 919.

Surface: public API.

Signature: `pub fn birth_path() -> (PathBuf, bool) {`

What it does: The path a birth should use, with the fallback applied: when resolution chose the system path for a state that does not exist yet and the system directory cannot be created by this user, the birth falls back to the per-user path — returned as (path, fell_back) so the caller can say so out loud, the way the Android home fallback always has. An env-chosen path is the operator's word and is never second-guessed.

(Where these words came from: direct.)

---

## function — state_dir

Found in src/lattice.rs at line 958.

Surface: public API.

Signature: `pub fn state_dir() -> PathBuf {`

What it does: The directory holding the state file (and the GPG keyring, which is the gpg tool's own machinery and lives beside the file it serves).

(Where these words came from: direct.)

---

## struct — Lattice

Found in src/lattice.rs at line 973.

Surface: public API.

Signature: `pub struct Lattice {`

What it does: The loaded state document. All reads and writes of node state go through this type; no other module touches the file directly. The file is a chain (dispatch core slice 1, spec §3.3): universe 1 is `px` — the state every shipped verb reads and writes — and universes 2.. are carried in `extra`, untouched by universe-1 work, preserved across every save. Attempts live there at the same addresses as their item records.

(Where these words came from: direct.)

---

## function — Lattice::born

Found in src/lattice.rs at line 982.

Surface: public API.

Signature: `pub fn born(path: PathBuf) -> Lattice {`

What it does: An unborn state document bound to `path` (config's birth).

(Where these words came from: direct.)

---

## function — Lattice::universe

Found in src/lattice.rs at line 987.

Surface: public API.

Signature: `pub fn universe(&self, n: usize) -> Option<&Phext> {`

What it does: Universe N of the state chain (1-based; 1 is `px`).

(Where these words came from: direct.)

---

## function — Lattice::universe_count

Found in src/lattice.rs at line 996.

Surface: public API.

Signature: `pub fn universe_count(&self) -> usize {`

What it does: How many universes the loaded chain carries.

(Where these words came from: direct.)

---

## function — Lattice::set_universe

Found in src/lattice.rs at line 1005.

Surface: public API.

Signature: `pub fn set_universe(&mut self, n: usize, u: Phext) {`

What it does: Replace universe N (1-based), growing the chain with empty universes where a gap would open — slice 4's writer: an acceptance files its attempt universe, an origin files its as-dispatched snapshot. Universe 1 is `px` itself.

(Where these words came from: direct.)

---

## function — Lattice::load

Found in src/lattice.rs at line 1021.

Surface: public API.

Signature: `pub fn load() -> Result<Option<Lattice>, String> {`

What it does: Load the state file if it exists. Ok(None) = not born yet (no file): callers treat that as "run config", never as an occasion to invent a substitute store.

(Where these words came from: direct.)

---

## function — Lattice::load_from

Found in src/lattice.rs at line 1026.

Surface: public API.

Signature: `pub fn load_from(path: &PathBuf) -> Result<Option<Lattice>, String> {`

What it does: Load from an explicit path (tests, doctor).

(Where these words came from: direct.)

---

## function — Lattice::path

Found in src/lattice.rs at line 1037.

Surface: public API.

Signature: `pub fn path(&self) -> &PathBuf {`

What it does: Function Lattice::path takes no inputs beyond the value it is called on. It gives back &PathBuf.

(Where these words came from: inferred.)

---

## function — Lattice::save

Found in src/lattice.rs at line 1054.

Surface: public API.

Signature: `pub fn save(&mut self) -> Result<(), String> {`

What it does: Persist the whole document. Phext serializes canonically (ascending coordinates, delimiter ladder), so a save is a rewrite of one file — the format's write model, accepted. Every save first runs the state top-up (v1.4.1): `seed_missing_indexes` makes the document whole under the running binary's Layer 2 layout before it is written. save() is the write boundary — every mutating verb ends here, and no read or --check path ever reaches it (their byte-inert guarantees are untouched) — so an inherited state gains the homelands its birth binary predates on the first write the new binary makes, whichever verb that write belongs to.

(Where these words came from: direct.)

---

## const — MASTERY_INTERVAL_DAYS

Found in src/learn.rs at line 45.

Surface: public API.

Signature: `pub const MASTERY_INTERVAL_DAYS: i64 = 21;`

What it does: A card counts as mastered at this interval (days).

(Where these words came from: direct.)

---

## function — learn

Found in src/learn.rs at line 995.

Surface: public API.

Signature: `pub fn learn(status_only: bool) -> i32 {`

What it does: `qexo learn [--status]`.

(Where these words came from: direct.)

---

## function — learn_app

Found in src/learn.rs at line 1253.

Surface: public API.

Signature: `pub fn learn_app() -> i32 {`

What it does: `qexo learn --app` — the wizard's engine over stdin/stdout. Exit codes match learn(): 0 clean, 1 a state save failed, plus require_born's codes when the node is unborn.

(Where these words came from: direct.)

---

## entry point — main

Found in src/main.rs at line 37.

Surface: public API.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## type — Coord

Found in src/phext.rs at line 53.

Surface: public API.

Signature: `pub type Coord = [u32; 9];`

What it does: A type name, Coord, that stands for [u32; 9].

(Where these words came from: inferred.)

---

## function — parse_coord

Found in src/phext.rs at line 55.

Surface: public API.

Signature: `pub fn parse_coord(s: &str) -> Option<Coord> {`

What it does: Function parse_coord takes s: &str. It gives back Option<Coord>.

(Where these words came from: inferred.)

---

## function — fmt_coord

Found in src/phext.rs at line 70.

Surface: public API.

Signature: `pub fn fmt_coord(c: &Coord) -> String {`

What it does: Function fmt_coord takes c: &Coord. It gives back String.

(Where these words came from: inferred.)

---

## function — parse_universe_addr

Found in src/phext.rs at line 81.

Surface: public API.

Signature: `pub fn parse_universe_addr(s: &str) -> Option<(usize, Coord)> {`

What it does: Parse an address as the CLI verbs take it: a bare 9D coordinate means universe 1; a `$N:` prefix (libphext 0.6's multiverse addressing) names universe N of a chain. Returns (universe, coordinate); universe numbers start at 1.

(Where these words came from: direct.)

---

## struct — CoordFmt

Found in src/phext.rs at line 99.

Surface: public API.

Signature: `pub struct CoordFmt(pub Coord);`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## struct — Phext

Found in src/phext.rs at line 131.

Surface: public API.

Signature: `pub struct Phext {`

What it does: A data shape named Phext. Its fields are: pub scrolls: BTreeMap<Coord, String>.

(Where these words came from: inferred.)

---

## function — Phext::new

Found in src/phext.rs at line 136.

Surface: public API.

Signature: `pub fn new() -> Phext {`

What it does: Function Phext::new takes no inputs beyond the value it is called on. It gives back Phext.

(Where these words came from: inferred.)

---

## function — Phext::insert

Found in src/phext.rs at line 140.

Surface: public API.

Signature: `pub fn insert(&mut self, coord: Coord, text: String) {`

What it does: Function Phext::insert takes &mut self, coord: Coord, text: String. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — Phext::get

Found in src/phext.rs at line 144.

Surface: public API.

Signature: `pub fn get(&self, coord: &Coord) -> Option<&String> {`

What it does: Function Phext::get takes &self, coord: &Coord. It gives back Option<&String>.

(Where these words came from: inferred.)

---

## function — Phext::toc

Found in src/phext.rs at line 148.

Surface: public API.

Signature: `pub fn toc(&self) -> Vec<(Coord, usize)> {`

What it does: Function Phext::toc takes no inputs beyond the value it is called on. It gives back Vec<(Coord, usize)>.

(Where these words came from: inferred.)

---

## function — Phext::serialize

Found in src/phext.rs at line 155.

Surface: public API.

Signature: `pub fn serialize(&self) -> String {`

What it does: Serialize with the library: implode sorts the scrolls and walks the dead-reckoning path emitting libphext's delimiter ladder (library break \x01 at the top).

(Where these words came from: direct.)

---

## function — Phext::parse

Found in src/phext.rs at line 167.

Surface: public API.

Signature: `pub fn parse(text: &str) -> Phext {`

What it does: Parse with the library: explode reckons every scroll's address libphext's way; this adapter then drops the empty stops (see the module note — a scroll exists when it holds at least a byte).

(Where these words came from: direct.)

---

## function — Phext::parse_universes

Found in src/phext.rs at line 187.

Surface: public API.

Signature: `pub fn parse_universes(text: &str) -> (Vec<Phext>, bool) {`

What it does: Parse a CHAIN file (libphext 0.6: members joined by the chain break \x02, the chain optionally closed by \x03) into its universes — one Phext per member, in chain order — plus the chain's closed flag. A plain single-phext file is a chain of one universe, and reads exactly as `parse` reads it when the file carries no chain bytes as content. (A stream holding several chains is flattened in order: qexo state is one chain, and flattening never loses a member on a load→save round trip.)

(Where these words came from: direct.)

---

## function — Phext::serialize_universes

Found in src/phext.rs at line 208.

Surface: public API.

Signature: `pub fn serialize_universes(universes: &[Phext], closed: bool) -> String {`

What it does: Serialize universes back into a chain file. A single universe serializes EXACTLY as `serialize` does — no chain bytes are added — so every state file written before chains existed round-trips byte-for-byte, and a chain only appears on disk once a second universe does.

(Where these words came from: direct.)

---

## const — VERSION

Found in src/plan.rs at line 58.

Surface: public API.

Signature: `pub const VERSION: &str = env!("CARGO_PKG_VERSION");`

What it does: The build's own version, from the Cargo manifest — the single source every user-facing version stamp derives from (v1.7.2). Hand-written version literals drifted three releases behind (the "qexo v1.4.0" help/status prints on a v1.7.1 binary); deriving them makes that drift impossible by construction.

(Where these words came from: direct.)

---

## const — PLAN_VERSION

Found in src/plan.rs at line 59.

Surface: public API.

Signature: `pub const PLAN_VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));`

What it does: A fixed value named PLAN_VERSION, set to concat!("v", env!("CARGO_PKG_VERSION")).

(Where these words came from: inferred.)

---

## enum — Class

Found in src/plan.rs at line 62.

Surface: public API.

Signature: `pub enum Class {`

What it does: A named choice named Class. It is one of: Video; Build; Light.

(Where these words came from: inferred.)

---

## function — Class::name

Found in src/plan.rs at line 69.

Surface: public API.

Signature: `pub fn name(&self) -> &'static str {`

What it does: Function Class::name takes no inputs beyond the value it is called on. It gives back &'static str.

(Where these words came from: inferred.)

---

## function — classify

Found in src/plan.rs at line 78.

Surface: public API.

Signature: `pub fn classify(title: &str, request: &str) -> Class {`

What it does: Function classify takes title: &str, request: &str. It gives back Class.

(Where these words came from: inferred.)

---

## function — host_snapshot

Found in src/plan.rs at line 109.

Surface: public API.

Signature: `pub fn host_snapshot() -> (f64, f64) {`

What it does: (MemAvailable MB, loadavg 1min) of the host qexo runs on — a live probe of /proc, as before.

(Where these words came from: direct.)

---

## struct — DiskObs

Found in src/plan.rs at line 134.

Surface: public API.

Signature: `pub struct DiskObs {`

What it does: One disk observation: the current reading, and (as history entries) the series the forecast gate rates growth from. In the lattice these are telemetry scrolls; here they are values.

(Where these words came from: direct.)

---

## struct — Volume

Found in src/plan.rs at line 147.

Surface: public API.

Signature: `pub struct Volume {`

What it does: A volume as the probe measures it: the filesystem under a named root, in bytes. The gate judges THIS volume — not a quota remembered from another machine.

(Where these words came from: direct.)

---

## function — Volume::free_pct

Found in src/plan.rs at line 159.

Surface: public API.

Signature: `pub fn free_pct(&self) -> f64 {`

What it does: Free space as a percentage of the whole volume.

(Where these words came from: direct.)

---

## struct — GateConfig

Found in src/plan.rs at line 173.

Surface: public API.

Signature: `pub struct GateConfig {`

What it does: The gate's thresholds, read from the node's config record in state.phext (homeland 9.9.9/9.9.9/9.9.9). Absent keys mean the defaults — the defaults are the volume lens only: a ranch volume is judged by its own free fraction, and no absolute ceiling applies until a node writes one into its config.

(Where these words came from: direct.)

---

## function — GateConfig::from_lookup

Found in src/plan.rs at line 211.

Surface: public API.

Signature: `pub fn from_lookup<F: Fn(&str) -> Option<String>>(get: F) -> GateConfig {`

What it does: Build from a config record via a field lookup (the record's `key: value` lines; the caller supplies the reader so this module stays free of lattice mechanics). Keys: volume_lens, disk_gate_free_amber_pct, disk_gate_free_red_pct, quota_lens, quota_amber_gb, quota_red_gb, file_amber, file_red.

(Where these words came from: direct.)

---

## struct — GateInputs

Found in src/plan.rs at line 246.

Surface: public API.

Signature: `pub struct GateInputs {`

What it does: Everything the gate needs beyond the observations: the volume the probe measured (None when the probe could not read one), the root the file count was taken under, and the thresholds.

(Where these words came from: direct.)

---

## struct — GateEval

Found in src/plan.rs at line 271.

Surface: public API.

Signature: `pub struct GateEval {`

What it does: The gate's verdict, with its reasons kept as text: which lens fired and the numbers it fired on. The plan renders these; a gate that cannot say why it fired is a gate no one can argue with — or trust.

(Where these words came from: direct.)

---

## function — fmt_gib

Found in src/plan.rs at line 314.

Surface: public API.

Signature: `pub fn fmt_gib(bytes: u64) -> String {`

What it does: Format bytes as GiB (2^30) with thousands separators — the units `df -h` speaks, labeled so no one has to guess.

(Where these words came from: direct.)

---

## function — fmt_count

Found in src/plan.rs at line 321.

Surface: public API.

Signature: `pub fn fmt_count(n: u64) -> String {`

What it does: Format a count with thousands separators (1,100,000).

(Where these words came from: direct.)

---

## function — evaluate_gate

Found in src/plan.rs at line 332.

Surface: public API.

Signature: `pub fn evaluate_gate(current: &DiskObs, vol: Option<&Volume>, files_root: &str, cfg: &GateConfig) -> GateEval {`

What it does: Evaluate the disk gate from values: the current observation, the measured volume, the file count's root, the thresholds. Byte half = the worse of the lenses that are ON; disk gate = the worse of byte and file halves. The observation's own level word is NOT consulted: since the telemetry series is qexo's own, the level recorded with an observation is this very evaluation's output (the caller stamps it), not an input.

(Where these words came from: direct.)

---

## struct — TaskInput

Found in src/plan.rs at line 412.

Surface: public API.

Signature: `pub struct TaskInput {`

What it does: A task row as the planner consumes it.

(Where these words came from: direct.)

---

## struct — Forecast

Found in src/plan.rs at line 419.

Surface: public API.

Signature: `pub struct Forecast {`

What it does: A data shape named Forecast. Its fields are: pub available: bool; pub files_per_day: f64; pub gb_per_day: f64; pub span_days: f64; pub intervals: usize; pub nearest_label: String; pub nearest_days: Option<f64>; pub file_amber_days: Option<f64>; pub byte_amber_days: Option<f64>.

(Where these words came from: inferred.)

---

## function — Forecast::unavailable

Found in src/plan.rs at line 435.

Surface: public API.

Signature: `pub fn unavailable() -> Forecast {`

What it does: Function Forecast::unavailable takes no inputs beyond the value it is called on. It gives back Forecast.

(Where these words came from: inferred.)

---

## const — FORECAST_HORIZON_DAYS

Found in src/plan.rs at line 457.

Surface: public API.

Signature: `pub const FORECAST_HORIZON_DAYS: f64 = 365.0;`

What it does: How far ahead a crossing projection is reported, in days. A linear rate read off the last interval is a short-horizon instrument: a crossing farther out than this will be re-projected hundreds of times before it can matter, and naming a day count for it lends it a precision it does not have. Crossings beyond the horizon are reported as no crossing.

(Where these words came from: direct.)

---

## const — FORECAST_WINDOW_INTERVALS

Found in src/plan.rs at line 464.

Surface: public API.

Signature: `pub const FORECAST_WINDOW_INTERVALS: usize = 7;`

What it does: How many of the most recent intervals the forecast rates (v1.2.1). The window bounds the median's memory: older intervals fall out instead of anchoring the rate forever, and a history shorter than the window simply contributes every interval it has.

(Where these words came from: direct.)

---

## function — forecast_gate

Found in src/plan.rs at line 518.

Surface: public API.

Signature: `pub fn forecast_gate(history: &[DiskObs], cfg: &GateConfig, total_bytes: Option<u64>) -> Forecast {`

What it does: The forecast computation (v2.3 semantics, re-based on the gate's lenses in v1.0.3; rates re-based on the median interval in v1.2.1): rate growth from the history — the median of the per-interval rates over the most recent FORECAST_WINDOW_INTERVALS intervals, falling back to the single interval when the history holds only two observations — then project days to the crossings of the SAME thresholds evaluate_gate reads — the volume lens's free fractions of this volume's total, the quota lens's ceilings (only when the node has that lens on), and the file lens's configured counts. Until v1.0.3 the byte half projected the legacy absolute levels (95/102 GB) whatever the node's configuration, so the forecast contradicted the gate it forecasts on every ranch volume. A crossing the current reading has already reached is the gate's verdict, not a forecast: projecting it would report 0.0 days for a level the lenses already own — exactly the v1.0.2 fleet failure, where the second plan on a healthy ranch node flipped to FORECAST AMBER because its volume had long since passed a ceiling that no longer applies to it. Flat or falling growth projects nothing, and nothing is projected beyond FORECAST_HORIZON_DAYS. Fewer than two usable observations degrades to unavailable — never invented. Why the median (v1.2.1, from the 2026-11-12 evaluation of 2026-11-11 and re-proven by 2026-11-12's interval): rating from the last interval alone made the nearest-crossing estimate whipsaw with whatever the last day happened to do — file AMBER read ~1.4 days out on the 2026-11-10 interval, 14 days out on 2026-11-11's, ~9.9 days out on 2026-11-12's, while the underlying series never changed character. The median of the recent intervals reports the series' typical rate: a planner whose warnings hold still long enough to act on.

(Where these words came from: direct.)

---

## struct — PlanRow

Found in src/plan.rs at line 612.

Surface: public API.

Signature: `pub struct PlanRow {`

What it does: A data shape named PlanRow. Its fields are: pub coordinate: String; pub title: String; pub class: Class; pub execution: String.

(Where these words came from: inferred.)

---

## struct — PlanData

Found in src/plan.rs at line 620.

Surface: public API.

Signature: `pub struct PlanData {`

What it does: A data shape named PlanData. Its fields are: pub disk_date: String; pub used_gb: f64; pub disk_level: String; pub files: u64; pub vol_root: String; pub total_bytes: u64; pub used_bytes: u64; pub free_bytes: u64; pub files_root: String; pub byte_gate: String; pub file_gate: String; pub gate_reason: String; pub byte_detail: String; pub file_detail: String; pub disk_gate: String; pub disk_gate_action: String; pub forecast: Forecast; pub ram_available_mb: f64; pub load1: f64; pub counts: (usize, usize, usize); pub rows: Vec<PlanRow>; pub draw_seed: u64; pub roster: Vec<String>; pub draw_picks: Vec<String>.

(Where these words came from: inferred.)

---

## function — PlanData::open_tasks

Found in src/plan.rs at line 656.

Surface: public API.

Signature: `pub fn open_tasks(&self) -> usize {`

What it does: Function PlanData::open_tasks takes no inputs beyond the value it is called on. It gives back usize.

(Where these words came from: inferred.)

---

## function — parse_roster

Found in src/plan.rs at line 678.

Surface: public API.

Signature: `pub fn parse_roster(v: Option<String>) -> Vec<String> {`

What it does: Parse the config record's `ranch_roster` value (v1.7.3): comma-separated node names, whitespace-trimmed, empties and duplicates dropped, order preserved (order carries no preference — the draw is uniform over the roster — it is kept only so the receipt reads as configured).

(Where these words came from: direct.)

---

## function — compute

Found in src/plan.rs at line 710.

Surface: public API.

Signature: `pub fn compute(tasks: &[TaskInput], current: &DiskObs, history: &[DiskObs], host: (f64, f64), gate: &GateInputs) -> PlanData {`

What it does: Compute the plan: byte gate: the worse of the lenses that are ON (see evaluate_gate) — the volume lens judges the measured volume's free fraction; the quota lens, only where a node configures it, reproduces the old absolute ceiling (AMBER 95 GB / RED 102 GB, thresholds anchored to the 2026-11-08 EDQUOT failure at ~106 GB on the Muse VM's staging quota — a real ceiling on that one machine, a false verdict everywhere else). file gate (v2.2): AMBER/RED at the configured counts under the named staging root — the count is the faster-moving half of the disk currency, and a byte-only gate can sit GREEN while the count runs at the wall. disk gate = the worse of the two halves. forecast gate (v2.3, re-based on the lenses in v1.0.3): a GREEN disk gate escalates to AMBER when the nearest crossing projected under the node's configured lenses is within 3 days. A planner that only watches crossings gives no lead time — and a forecast that projects levels the gate no longer reads gives false alarms instead.

(Where these words came from: direct.)

---

## function — days_from_civil

Found in src/plan.rs at line 794.

Surface: public API.

Signature: `pub fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {`

What it does: Days from civil date (Howard Hinnant's algorithm) — the v2.3 date arithmetic, carried verbatim in behavior.

(Where these words came from: direct.)

---

## function — render

Found in src/plan.rs at line 835.

Surface: public API.

Signature: `pub fn render(plan: &PlanData) -> String {`

What it does: Human-readable rendering of a plan (the v1 shape, kept). In the scratch line this renders from PlanData; the same text is what a plan record stores as its rendered scroll.

(Where these words came from: direct.)

---

## function — drain_record_addr

Found in src/register.rs at line 72.

Surface: public API.

Signature: `pub fn drain_record_addr() -> Coord {`

What it does: Function drain_record_addr takes no inputs beyond the value it is called on. It gives back Coord.

(Where these words came from: inferred.)

---

## struct — DrainRecord

Found in src/register.rs at line 82.

Surface: public API.

Signature: `pub struct DrainRecord {`

What it does: The register link's standing facts: where the register is, how far the drain has read, and the last pass's cite. Calculated views (lag, health) derive from this record and the service's high-water seqs — never the other way round.

(Where these words came from: direct.)

---

## function — load_drain_record

Found in src/register.rs at line 89.

Surface: public API.

Signature: `pub fn load_drain_record(px: &phext::Phext) -> Option<DrainRecord> {`

What it does: Function load_drain_record takes px: &phext::Phext. It gives back Option<DrainRecord>.

(Where these words came from: inferred.)

---

## struct — Credentials

Found in src/register.rs at line 127.

Surface: public API.

Signature: `pub struct Credentials {`

What it does: The register credentials, by name. Values are held only in memory, only for the call that presents them; nothing in this module prints, stores, or logs them — the struct deliberately does not derive Debug, so a token is never one careless print away from a log.

(Where these words came from: direct.)

---

## function — credentials_path

Found in src/register.rs at line 132.

Surface: public API.

Signature: `pub fn credentials_path() -> PathBuf {`

What it does: Function credentials_path takes no inputs beyond the value it is called on. It gives back PathBuf.

(Where these words came from: inferred.)

---

## function — load_credentials

Found in src/register.rs at line 139.

Surface: public API.

Signature: `pub fn load_credentials() -> Credentials {`

What it does: Function load_credentials takes no inputs beyond the value it is called on. It gives back Credentials.

(Where these words came from: inferred.)

---

## function — parse_credentials

Found in src/register.rs at line 151.

Surface: public API.

Signature: `pub fn parse_credentials(text: &str) -> Credentials {`

What it does: Parse the env file's `KEY=value` lines. The keys are the register service's own env names (§10.4) — the drainer node's file carries the same names the box carries, so a value is provisioned once, by Will's hand, in one pass.

(Where these words came from: direct.)

---

## function — stamp_of_epoch

Found in src/register.rs at line 180.

Surface: public API.

Signature: `pub fn stamp_of_epoch(ts: i64) -> String {`

What it does: The house stamp for an epoch-seconds instant — the same reckoning `now_stamp` writes and `stamp_minutes` parses back, applied to a register atom's `ts`.

(Where these words came from: direct.)

---

## struct — EntryAtom

Found in src/register.rs at line 540.

Surface: public API.

Signature: `pub struct EntryAtom {`

What it does: A register entry atom (§10.1): text in, from a user, at a time, at a seq. The seq is the drain's dedup key.

(Where these words came from: direct.)

---

## struct — ResolutionAtom

Found in src/register.rs at line 550.

Surface: public API.

Signature: `pub struct ResolutionAtom {`

What it does: A register resolution atom (§10.1): register number, outcome, the user who resolved it, an optional note.

(Where these words came from: direct.)

---

## struct — Fetched

Found in src/register.rs at line 604.

Surface: public API.

Signature: `pub struct Fetched {`

What it does: What one drain pass pulled: both logs past the watermarks, and each log's high-water seq (the service's own `last_seq` — where the watermarks may advance to once everything is filed).

(Where these words came from: direct.)

---

## struct — PendingWriteback

Found in src/register.rs at line 670.

Surface: public API.

Signature: `pub struct PendingWriteback {`

What it does: A write-back the drain found still owed: a terminal register item whose resolution is not yet recorded at the register. The sweep after the pull retries these.

(Where these words came from: direct.)

---

## struct — DrainOutcome

Found in src/register.rs at line 679.

Surface: public API.

Signature: `pub struct DrainOutcome {`

What it does: A data shape named DrainOutcome. Its fields are: pub filed: Vec<String>; pub closed: Vec<String>; pub orphan_audits: Vec<String>; pub deduped_entries: usize; pub duplicate_resolutions: usize; pub writeback_pending: Vec<PendingWriteback>.

(Where these words came from: inferred.)

---

## function — apply_drain

Found in src/register.rs at line 717.

Surface: public API.

Signature: `pub fn apply_drain(lat: &mut Lattice, fetched: &Fetched, actor: &str) -> Result<DrainOutcome, String> {`

What it does: Function apply_drain takes lat: &mut Lattice, fetched: &Fetched, actor: &str. It gives back Result<DrainOutcome, String>.

(Where these words came from: inferred.)

---

## function — post_resolution

Found in src/register.rs at line 849.

Surface: public API.

Signature: `pub fn post_resolution( base: &str, qexo_token: &str, number: &str, outcome: &str, note: Option<&str>, ) -> Result<u64, String> {`

What it does: POST a resolution for register number `n` under the qexo service token. Returns the resolution atom's seq at the register. The token authorizes this and the entries-append — nothing else (§10.4, Q2).

(Where these words came from: direct.)

---

## function — post_entry

Found in src/register.rs at line 883.

Surface: public API.

Signature: `pub fn post_entry(base: &str, qexo_token: &str, text: &str) -> Result<u64, String> {`

What it does: POST an entry under the qexo service token (the ratified Q2 leg: items born on the phone appear in the register web view). Returns the entry atom's seq. Its production caller is the phone/app surface (slice 7); slice 3 proves the leg against the double in tests.

(Where these words came from: direct.)

---

## function — write_back

Found in src/register.rs at line 902.

Surface: public API.

Signature: `pub fn write_back(lat: &mut Lattice, item: &dispatch::Item, outcome: &str, note: Option<&str>) -> Result<u64, String> {`

What it does: Record one owed write-back: post the resolution, then — in one save — flip the item's field to `recorded` and write the receipt that proves it (credential cited by name only). A post that lands but cannot be recorded is named as such; the retry it causes posts a duplicate the register keeps and the drain no-ops — the service's own contract (§10.1).

(Where these words came from: direct.)

---

## function — write_back_with

Found in src/register.rs at line 907.

Surface: public API.

Signature: `pub fn write_back_with( lat: &mut Lattice, item: &dispatch::Item, outcome: &str, note: Option<&str>, creds: &Credentials, ) -> Result<u64, String> {`

What it does: Function write_back_with takes lat: &mut Lattice, item: &dispatch::Item, outcome: &str, note: Option<&str>, creds: &Credentials,. It gives back Result<u64, String>.

(Where these words came from: inferred.)

---

## function — drain

Found in src/register.rs at line 962.

Surface: public API.

Signature: `pub fn drain(check: bool, url: Option<String>) -> i32 {`

What it does: Function drain takes check: bool, url: Option<String>. It gives back i32.

(Where these words came from: inferred.)

---

## function — drain_run

Found in src/register.rs at line 977.

Surface: public API.

Signature: `pub fn drain_run(check: bool, url: Option<String>) -> (i32, dispatch::RunLines) {`

What it does: The drain as a captured run (slice 7, spec §2.3): the same pass `drain` makes — fetch, apply, the write-back sweep — rendered into a RunLines instead of printed, so the `--app` driver drives the identical engine and renders its facts in the protocol's shape.

(Where these words came from: direct.)

---

## const — BUNDLE_ANCHOR

Found in src/wire.rs at line 100.

Surface: public API.

Signature: `pub const BUNDLE_ANCHOR: Coord = [11, 11, 11, 11, 11, 11, 11, 11, 11];`

What it does: The bundle manifest's address inside $1: — the bundle's own 11.11.11/11.11.11/11.11.11 (§3.4).

(Where these words came from: direct.)

---

## const — BUNDLE_SIG

Found in src/wire.rs at line 104.

Surface: public API.

Signature: `pub const BUNDLE_SIG: Coord = [11, 11, 11, 11, 11, 11, 11, 11, 12];`

What it does: The detached signature beside the manifest — the next scroll of the anchor's Book (§3.4: "beside it at Scroll 2").

(Where these words came from: direct.)

---

## const — THERMAL_LIMIT_C

Found in src/wire.rs at line 125.

Surface: public API.

Signature: `pub const THERMAL_LIMIT_C: f64 = 75.0;`

What it does: The thermal gate (§6): a node at or above 75 °C fails eligibility (b).

(Where these words came from: direct.)

---

## const — HEAVY_CAP

Found in src/wire.rs at line 128.

Surface: public API.

Signature: `pub const HEAVY_CAP: usize = 4;`

What it does: The heavy-work cap (§6): at most 4 heavy items running fleet-wide, FIFO among heavy items.

(Where these words came from: direct.)

---

## function — field_all

Found in src/wire.rs at line 133.

Surface: public API.

Signature: `pub fn field_all(text: &str, key: &str) -> Vec<String> {`

What it does: Every line in `text` carrying `key:`, in order — `lattice::field` reads the first; capability lists, exclusion lists, and hash lists are plural.

(Where these words came from: direct.)

---

## struct — BriefFacts

Found in src/wire.rs at line 183.

Surface: public API.

Signature: `pub struct BriefFacts {`

What it does: What placement and the bundle builder read from an item's brief scroll: `capability:` lines (repeatable), `heavy: yes`, and `context:` citations naming scroll selections in the origin's state (one coordinate, or a range `coord..coord`, inclusive, in coordinate order).

(Where these words came from: direct.)

---

## function — brief_facts

Found in src/wire.rs at line 192.

Surface: public API.

Signature: `pub fn brief_facts(brief: &str) -> BriefFacts {`

What it does: Function brief_facts takes brief: &str. It gives back BriefFacts.

(Where these words came from: inferred.)

---

## function — unsigned_chain_bytes

Found in src/wire.rs at line 241.

Surface: public API.

Signature: `pub fn unsigned_chain_bytes(universes: &[Phext]) -> String {`

What it does: The canonical bytes a bundle's hash is taken over: the chain of its universes with the signature scroll removed. Deterministic in the item record, the brief, the context selections, the origin identity, the attempt, and the routed bridge (§9.2 — the choice is cited in the manifest, so a different route is a different bundle) — nothing else (§4.3).

(Where these words came from: direct.)

---

## function — build_bundle_parts

Found in src/wire.rs at line 266.

Surface: public API.

Signature: `pub fn build_bundle_parts( px: &Phext, item: &Item, attempt: u32, resume_from: Option<&str>, resume_text: Option<&str>, bridge: Option<(&str, &str)>, ) -> Result<(Vec<Phext>, String), String> {`

What it does: Build a bundle's universes (unsigned) and manifest for `item` as it stands in `px` — the record pre-transition (state queued, attempts = attempt−1). `resume_text` is the checkpoint scroll the origin holds in custody for `resume_from`'s cite (§8): when both are present, the checkpoint rides as the bundle's last universe, provenanced like any context universe, so the replacement executor receives the resume currency itself — not just its address.

(Where these words came from: direct.)

---

## struct — Bundle

Found in src/wire.rs at line 433.

Surface: public API.

Signature: `pub struct Bundle {`

What it does: A built bundle: its identity hash (§4.3), its manifest, and its unsigned universes.

(Where these words came from: direct.)

---

## function — build_bundle

Found in src/wire.rs at line 441.

Surface: public API.

Signature: `pub fn build_bundle( px: &Phext, item: &Item, attempt: u32, resume_from: Option<&str>, resume_text: Option<&str>, bridge: Option<(&str, &str)>, ) -> Result<Bundle, String> {`

What it does: Function build_bundle takes px: &Phext, item: &Item, attempt: u32, resume_from: Option<&str>, resume_text: Option<&str>, bridge: Option<. It gives back Result<Bundle, String>.

(Where these words came from: inferred.)

---

## function — sign_bundle

Found in src/wire.rs at line 464.

Surface: public API.

Signature: `pub fn sign_bundle(gpg: &Gpg, bundle: &Bundle) -> Result<String, String> {`

What it does: Sign a built bundle into its wire bytes: the detached signature over the manifest's exact bytes at the anchor's Scroll 2 (§3.4 — the releases cluster's manifest+signature pattern).

(Where these words came from: direct.)

---

## struct — ParsedBundle

Found in src/wire.rs at line 509.

Surface: public API.

Signature: `pub struct ParsedBundle {`

What it does: A parsed wire bundle: the manifest's claims, the universes as received, and the manifest's exact bytes (the signature's payload).

(Where these words came from: direct.)

---

## function — parse_wire_bundle

Found in src/wire.rs at line 530.

Surface: public API.

Signature: `pub fn parse_wire_bundle(bytes: &[u8]) -> Result<ParsedBundle, String> {`

What it does: Function parse_wire_bundle takes bytes: &[u8]. It gives back Result<ParsedBundle, String>.

(Where these words came from: inferred.)

---

## function — verify_bundle

Found in src/wire.rs at line 583.

Surface: public API.

Signature: `pub fn verify_bundle(lat: &Lattice, gpg: &Gpg, pb: &ParsedBundle) -> Result<(), String> {`

What it does: Verify a parsed bundle (§3.4, in the acceptance rule's order): the claimed signer must be a registered peer in this node's state and the detached signature must verify under that identity (registering a peer imported its key — verification is the shipped path); every context universe's hash must match the manifest's list. The bundle hash is carried by the identity rule (§4.3), not verified against the manifest — the manifest cannot contain it; it is checked against the item's record by the callers that hold one.

(Where these words came from: direct.)

---

## function — file_attempt

Found in src/wire.rs at line 681.

Surface: public API.

Signature: `pub fn file_attempt(lat: &mut Lattice, pb: &ParsedBundle, assignment_book: u32) -> Result<(), String> {`

What it does: File an accepted bundle's attempt (§3.3, §3.4): the attempt universe (attempt N → universe N+1) holds the executor's working copy of the item — the record copy with `home:` set to the origin node, `bundle-sha256:` set, state dispatched — the brief, and the bundle's paperwork under the assignment's Book family. Universe 1 holds a second copy at the item's home address — the record the oracle and the local verbs read; the attempt itself (its heartbeat lines, its checkpoint scrolls) lives in the stratum, never merged into universe 1 (§3.3). An address already holding a DIFFERENT bundle's copy is a named refusal — two bundles never share a home address in a stratum.

(Where these words came from: direct.)

---

## function — file_origin_snapshot

Found in src/wire.rs at line 737.

Surface: public API.

Signature: `pub fn file_origin_snapshot(lat: &mut Lattice, pb: &ParsedBundle, assignment_book: u32) {`

What it does: File the ORIGIN's as-dispatched snapshot of an attempt (§4.6 combination 7's conservative reading: attempts homed here need chain universes ≥ attempts+1 in the origin's own state). The snapshot is the bundle's contents verbatim — the pristine record copy, the brief, the manifest, the context provenances — never the executor's working copy; the doctor's combination 6 recomputes from either form (the recomputation strips exactly the fields acceptance adds).

(Where these words came from: direct.)

---

## enum — AcceptOutcome

Found in src/wire.rs at line 756.

Surface: public API.

Signature: `pub enum AcceptOutcome {`

What it does: The outcome of an acceptance attempt — the serve handler renders it, the dispatch verb cites it.

(Where these words came from: direct.)

---

## function — next_assignment_book

Found in src/wire.rs at line 803.

Surface: public API.

Signature: `pub fn next_assignment_book(lat: &Lattice) -> u32 {`

What it does: The next free assignment Book (Collection 3).

(Where these words came from: direct.)

---

## function — accept_wire_bundle

Found in src/wire.rs at line 896.

Surface: public API.

Signature: `pub fn accept_wire_bundle(lat: &mut Lattice, gpg: &Gpg, bytes: &[u8]) -> AcceptOutcome {`

What it does: Accept a wire bundle on this node (§3.4), in the acceptance rule's order: parse → the signer is a registered peer and the signature verifies → the per-universe hashes match → capability fit → dedup and fencing (§4.3, §4.6 combination 2). On acceptance the attempt is filed and the assignment record written — LIVE (no `status:` line). A replay of the same (item, sha256, attempt) returns the original acceptance receipt and writes nothing. A lower attempt, or an equal attempt carrying a different hash, is refused as stale (the fencing rule); a higher attempt supersedes live older assignments, which are retired in the same pass. Refusals file a refusal receipt in this node's ledgers (§4.6 combination 5's shape) and never file an attempt. Mutates in memory; the caller saves.

(Where these words came from: direct.)

---

## struct — NodeFacts

Found in src/wire.rs at line 1030.

Surface: public API.

Signature: `pub struct NodeFacts {`

What it does: One candidate's facts for one placement round. `None` on a measured field means unmeasured, which §6 treats as ineligible — never as zero ("load measured from telemetry, never assumed").

(Where these words came from: direct.)

---

## function — eligibility

Found in src/wire.rs at line 1050.

Surface: public API.

Signature: `pub fn eligibility(f: &NodeFacts, required_caps: &[String], bundle_bytes: u64) -> Result<(), String> {`

What it does: The eligibility verdict (§6, in the spec's own order): (a) reachability, (b) load and temperature, (c) a resident agent at work, (d) data locality — then capability fit. `Ok(())` is eligible; the `Err` is the exclusion's reason, tagged with its criterion, which the placement receipt carries verbatim (§14: exclusions are recorded, never silent).

(Where these words came from: direct.)

---

## function — draw

Found in src/wire.rs at line 1097.

Surface: public API.

Signature: `pub fn draw(eligible: &[String], seed: u64) -> Option<String> {`

What it does: The draw: splitmix64 over the round's seed, modulo the eligible set — uniform over the set as listed, and the eligible set is always listed in a stable order (self first, then peers by cluster), so a round replays exactly from its receipted seed (§6).

(Where these words came from: direct.)

---

## struct — Placement

Found in src/wire.rs at line 1300.

Surface: public API.

Signature: `pub struct Placement {`

What it does: A placement round's full account: the eligible set in draw order, every exclusion with its reason, the draw (or the named target), and the seed.

(Where these words came from: direct.)

---

## function — place

Found in src/wire.rs at line 1313.

Surface: public API.

Signature: `pub fn place(facts: &[NodeFacts], required_caps: &[String], bundle_bytes: u64, to: Option<&str>) -> Placement {`

What it does: Run one placement round over gathered facts. With `--to`, the named node is the target — but its eligibility is still computed and recorded, and an ineligible named node places nothing (§6: naming overrides the draw, never the gates).

(Where these words came from: direct.)

---

## function — cmd_dispatch

Found in src/wire.rs at line 1418.

Surface: public API.

Signature: `pub fn cmd_dispatch(item_arg: Option<&str>, to: Option<&str>, check: bool) -> i32 {`

What it does: `qexo dispatch [<item-address>] [--to <node>] [--check]` — §2.1, §5, §6. One save, at the end, on success; --check writes nothing at all. The CLI driver: the run below, printed live, byte for byte as always.

(Where these words came from: direct.)

---

## function — dispatch_app_run

Found in src/wire.rs at line 1425.

Surface: public API.

Signature: `pub fn dispatch_app_run(item_arg: Option<&str>, to: Option<&str>, check: bool) -> (i32, dispatch::RunLines) {`

What it does: The same dispatch as a captured run (slice 7, §2.3): the `--app` driver's road — one engine, its facts rendered in the protocol's shape by the caller.

(Where these words came from: direct.)

---

## function — check_checkpoint_outputs

Found in src/wire.rs at line 2070.

Surface: public API.

Signature: `pub fn check_checkpoint_outputs(text: &str, read: &dyn Fn(&str) -> Option<String>) -> Result<usize, String> {`

What it does: Verify a checkpoint's cited outputs (§8): every `output:` line names a scroll, every `output-sha256:` line the hash the executor took of it at checkpoint time; `read` fetches a cited scroll's current text (the origin reads over the scroll route). Ok(n) = n outputs read back and hash-matched. A checkpoint citing no outputs returns Ok(0) — it verifies nothing and proves no progress; the caller treats only n ≥ 1 as a verified progress event. Err names the first failure: an unreadable output, a hash that does not match, or a cite/hash count mismatch.

(Where these words came from: direct.)

---

## function — cmd_heartbeat

Found in src/wire.rs at line 2196.

Surface: public API.

Signature: `pub fn cmd_heartbeat(item_arg: &str) -> i32 {`

What it does: `qexo heartbeat <item-address>` — one verb, two sides (§6). Executor side: beat — the `heartbeat:` line lands on the assignment record and the attempt universe's record scroll; the first beat moves the item dispatched → running through the oracle. Origin side: read the executor's assignment record over the scroll route and reconcile the home record with what it says — or, on §8's evidences, detect the executor's loss and teleport the item.

(Where these words came from: direct.)

---

## function — cmd_checkpoint

Found in src/wire.rs at line 2583.

Surface: public API.

Signature: `pub fn cmd_checkpoint(args: &[String]) -> i32 {`

What it does: `qexo checkpoint <item-address> --note <text> [--output <coordinate>]…` — executor-local (§6): a checkpoint scroll in the attempt universe's item Book (the next free scroll at Scroll 4+), a running → running oracle transition, and the cite recorded on the attempt copy and the assignment. Each `--output` names a scroll this attempt produced (a bare coordinate, or `$<n>:<coordinate>` to name its universe); the executor resolves it from its own chain and records the scroll's SHA-256 beside the cite — the pair the origin later reads back to verify progress (§8). An output that cannot be read here is refused before anything is written: a checkpoint never cites what its own node cannot produce.

(Where these words came from: direct.)

---

## function — cmd_fail

Found in src/wire.rs at line 2734.

Surface: public API.

Signature: `pub fn cmd_fail(args: &[String]) -> i32 {`

What it does: `qexo fail <item-address> --note <evidence>` — executor-local (§4.6): the oracle's failure transition on the attempt copy with the evidence the spec requires (the plan lands the third consecutive failure in paused-auto, §8), and the assignment retired with the same evidence as its status — the origin reads it on its next heartbeat reconciliation.

(Where these words came from: direct.)

---

## function — recompute_bundle_sha256

Found in src/wire.rs at line 2830.

Surface: public API.

Signature: `pub fn recompute_bundle_sha256(lat: &Lattice, item_addr: &Coord, attempt: u32) -> Option<String> {`

What it does: Recompute an accepted attempt's bundle hash from its stratum: the manifest verbatim (filed under the assignment's Book family), the pristine record copy reconstructed from the working copy (the acceptance's own writes stripped, the pre-transition state restored), the brief, and the context universes regrouped by their provenance scrolls. Returns None when the stratum lacks the pieces (combination 7 owns that finding) and Some(hash) otherwise. The origin's as-dispatched snapshot recomputes identically — its record copy is already pristine, so the strips are no-ops.

(Where these words came from: direct.)

---

## function — declared_modules

Found in src/wiring.rs at line 28.

Surface: public API.

Signature: `pub fn declared_modules(main_text: &str) -> Vec<String> {`

What it does: Module names declared by `mod <name>;` (or `pub mod <name>;`) lines in a crate root's text, in declaration order.

(Where these words came from: direct.)

---

## function — check

Found in src/wiring.rs at line 49.

Surface: public API.

Signature: `pub fn check(main_text: &str, file_stems: &[String]) -> (Vec<String>, Vec<String>) {`

What it does: Compare a root's declarations against the module files present. Returns (orphaned, missing): files with no declaration, and declarations with no file. Both empty means the wiring is exact. `main` is the root itself and is never expected as a declaration.

(Where these words came from: direct.)

---

## function — check_dir

Found in src/wiring.rs at line 68.

Surface: public API.

Signature: `pub fn check_dir(src_dir: &Path) -> Result<(Vec<String>, Vec<String>, usize), String> {`

What it does: Run the gate against a source directory: read its `main.rs`, list its `*.rs` stems, and apply `check`.

(Where these words came from: direct.)

---

## function — crc32

Found in src/xz.rs at line 42.

Surface: public API.

Signature: `pub fn crc32(data: &[u8]) -> u32 {`

What it does: Function crc32 takes data: &[u8]. It gives back u32.

(Where these words came from: inferred.)

---

## function — crc64

Found in src/xz.rs at line 69.

Surface: public API.

Signature: `pub fn crc64(data: &[u8]) -> u64 {`

What it does: Function crc64 takes data: &[u8]. It gives back u64.

(Where these words came from: inferred.)

---

## function — encode_stored

Found in src/xz.rs at line 113.

Surface: public API.

Signature: `pub fn encode_stored(data: &[u8]) -> Vec<u8> {`

What it does: Encode `data` as a complete xz stream (stored LZMA2 chunks, CRC64 block check, single block). Byte-deterministic for a given input.

(Where these words came from: direct.)

---

## function — decode_stored

Found in src/xz.rs at line 194.

Surface: public API.

Signature: `pub fn decode_stored(stream: &[u8]) -> Result<Vec<u8>, String> {`

What it does: Decode a stored-chunk xz stream produced by encode_stored().

(Where these words came from: direct.)

---

## function — unseal

Found in src/xz.rs at line 246.

Surface: public API.

Signature: `pub fn unseal(stream: &[u8]) -> Result<Vec<u8>, String> {`

What it does: Unseal any xz stream: stored form decodes natively; compressed streams shell out to the system `xz` (wrap, don't fork).

(Where these words came from: direct.)

---

## function — roman

Found in tools/build_bake.py at line 29.

Surface: public API.

Signature: `def roman(s: str) -> int`

What it does: Function roman takes s: str. It gives back int.

(Where these words came from: inferred.)

---

## function — parse_index

Found in tools/build_bake.py at line 38.

Surface: public API.

Signature: `def parse_index(text: str) -> dict`

What it does: Function parse_index takes text: str. It gives back dict.

(Where these words came from: inferred.)

---

## function — main

Found in tools/build_bake.py at line 73.

Surface: public API.

Signature: `def main() -> int`

What it does: Function main takes no inputs beyond the value it is called on. It gives back int.

(Where these words came from: inferred.)

---

## function — sha

Found in tools/build_bake_remote.py at line 35.

Surface: public API.

Signature: `def sha(b)`

What it does: Function sha takes b. Its return type is unannotated; the code shows it returning hashlib.sha256(b).hexdigest().

(Where these words came from: inferred.)

---

## function — head

Found in tools/build_bake_remote.py at line 38.

Surface: public API.

Signature: `def head(repo)`

What it does: Function head takes repo. Its return type is unannotated; the code shows it returning subprocess.run(["git", "-C", repo, "rev-parse", .

(Where these words came from: inferred.)

---

## function — seal

Found in tools/build_bake_remote.py at line 42.

Surface: public API.

Signature: `def seal(raw)`

What it does: Function seal takes raw. Its return type is unannotated; the code shows it returning lzma.compress(raw, format=lzma.FORMAT_XZ,.

(Where these words came from: inferred.)

---

## function — advance

Found in tools/build_bake_remote.py at line 74.

Surface: public API.

Signature: `def advance(coord)`

What it does: Function advance takes coord. Its return type is unannotated; the code shows it returning c  # scroll component only (bake keeps one axis .

(Where these words came from: inferred.)

---

## function — scan_markers

Found in tools/build_bake_remote.py at line 119.

Surface: public API.

Signature: `def scan_markers()`

What it does: Function scan_markers takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning [_re.compile(p) for p in pats].

(Where these words came from: inferred.)

---

## function — serialize

Found in tools/build_bake_remote.py at line 151.

Surface: public API.

Signature: `def serialize(scrolls)`

What it does: Function serialize takes scrolls. Its return type is unannotated; the code shows it returning "".join(out).

(Where these words came from: inferred.)

---

## function — main

Found in tools/fetch_volumen_index.py at line 28.

Surface: public API.

Signature: `def main() -> int`

What it does: Function main takes no inputs beyond the value it is called on. It gives back int.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/qexo/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

# phext-lattice — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 150). Scanned at commit 6ffc18b162bdcd531bdbaae72e2216f4871395e4. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

Description stated in the manifest: Memory-mapped 9D phext lattice with O(1) coordinate lookup, sentron topology, and web/TUI editors

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Rust (cargo) — Cargo.toml

License stated in the manifest: MIT.

Direct dependencies: 19.

- libphext [dependencies] — 0.6.6
- holo-phext [dependencies] — (path or git)
- teixml-to-mutable [dependencies] — 0.3.0
- memmap2 [dependencies] — 0.9
- xxhash-rust [dependencies] — 0.8
- rayon [dependencies] — 1.10
- tempfile [dependencies] — 3
- serde [dependencies] — 1
- serde_json [dependencies] — 1
- hmac [dependencies] — 0.12
- sha2 [dependencies] — 0.10
- getrandom [dependencies] — 0.4
- ureq [dependencies] — 2
- ring [dependencies] — 0.17
- ratatui [dependencies] — 0.29
- crossterm [dependencies] — 0.28
- criterion [dev-dependencies] — 0.5
- rusqlite [dev-dependencies] — 0.32
- serde_json [dev-dependencies] — 1

Total packages in the lock file, this one included: 218.


---

## Authors

The git log holds 285 commits.

The author of record is **Will Bickford** — 261 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 107 commits
- Emise — 94 commits
- Theia — 51 commits
- Lux — 4 commits
- Claude — 2 commits
- Orin — 2 commits
- Velum — 1 commit

Other authors, kept as themselves (not Exocortical cluster personas; never folded):

- Cursor Agent <cursoragent@cursor.com> — 24 commits

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- JavaScript: 1 file, 370 lines total, 51 blank, 78 comment, 241 code
- Markdown: 9 files, 1790 lines total, 355 blank, 6 comment, 1429 code
- Python: 9 files, 1422 lines total, 188 blank, 34 comment, 1200 code
- Rust: 37 files, 35083 lines total, 2559 blank, 3963 comment, 28561 code
- Shell: 5 files, 1017 lines total, 70 blank, 145 comment, 802 code
- TOML: 1 file, 80 lines total, 11 blank, 13 comment, 56 code

All languages: 62 files, 39762 lines total, 32289 lines of code. Functions read: 1410.

Largest files:

- src/bin/phext_edit.rs — 15584 lines
- src/oauth.rs — 3452 lines
- tests/e2e_local.rs — 1678 lines
- src/sidecar.rs — 1652 lines
- src/bin/uml6d_edit.rs — 1594 lines

Largest functions, by span in lines:

- handle_dashboard in src/bin/phext_edit.rs — about 2879 lines
- main in src/bin/uml6d_edit.rs — about 942 lines
- handle_request in src/bin/phext_edit.rs — about 599 lines
- main in deploy/deploy.sh — about 300 lines
- oauth_handle_token in src/bin/phext_edit.rs — about 265 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 332 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 1. FIXME marks: 0.
- Public items with a doc comment: 303 of 522 (58%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/phext-lattice
- Branch: exo
- Head commit: 6ffc18b162bdcd531bdbaae72e2216f4871395e4
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `Article.__init__` (bench/wiki/extract_zim.py:27) — Function Article.__init__ takes no inputs beyond the value it is called on.
- function `Article.handle_starttag` (bench/wiki/extract_zim.py:31) — Function Article.handle_starttag takes self, tag, attrs.
- function `Article.handle_endtag` (bench/wiki/extract_zim.py:41) — Function Article.handle_endtag takes self, tag.
- function `Article.handle_data` (bench/wiki/extract_zim.py:47) — Function Article.handle_data takes self, data.
- function `Article.text` (bench/wiki/extract_zim.py:51) — Function Article.text takes no inputs beyond the value it is called on.
- function `main` (bench/wiki/extract_zim.py:58) — Function main takes zim_path, out_dir.
- function `main` (bench/wiki/make_queries.py:22) — Function main takes d, n=500.
- function `load` (bench/wiki/report.py:6) — Function load takes work, s, name.
- function `lat` (bench/wiki/report.py:11) — Function lat takes v, key="p50_us".
- function `mb` (bench/wiki/report.py:18) — Function mb takes b.
- function `table` (bench/wiki/report.py:22) — Function table takes head, rows.
- function `main` (bench/wiki/report.py:28) — Function main takes work, *scales.
- function `tree` (bench/wiki/storage.py:16) — Function tree takes p.
- function `one` (bench/wiki/storage.py:25) — Function one takes p.
- function `main` (bench/wiki/storage.py:30) — Function main takes out.
- function `load_corpus` (bench/wiki/zim_bench.py:20) [example] — Function load_corpus takes d, scale.
- function `Doc.__init__` (bench/wiki/zim_bench.py:39) [example] — Function Doc.__init__ takes self, title, text.
- function `Doc.has_indexdata` (bench/wiki/zim_bench.py:42) [example] — Function Doc.has_indexdata takes no inputs beyond the value it is called on.
- function `Doc.get_title` (bench/wiki/zim_bench.py:43) [example] — Function Doc.get_title takes no inputs beyond the value it is called on.
- function `Doc.get_content` (bench/wiki/zim_bench.py:44) [example] — Function Doc.get_content takes no inputs beyond the value it is called on.
- function `Doc.get_keywords` (bench/wiki/zim_bench.py:45) [example] — Function Doc.get_keywords takes no inputs beyond the value it is called on.
- function `Doc.get_wordcount` (bench/wiki/zim_bench.py:46) [example] — Function Doc.get_wordcount takes no inputs beyond the value it is called on.
- function `Text.__init__` (bench/wiki/zim_bench.py:50) [example] — Function Text.__init__ takes self, path, title, text, front, index.
- function `Text.get_path` (bench/wiki/zim_bench.py:53) [example] — Function Text.get_path takes no inputs beyond the value it is called on.
- function `Text.get_title` (bench/wiki/zim_bench.py:54) [example] — Function Text.get_title takes no inputs beyond the value it is called on.
- function `Text.get_mimetype` (bench/wiki/zim_bench.py:55) [example] — Function Text.get_mimetype takes no inputs beyond the value it is called on.
- function `Text.get_contentprovider` (bench/wiki/zim_bench.py:56) [example] — Function Text.get_contentprovider takes no inputs beyond the value it is called on.
- function `Text.get_hints` (bench/wiki/zim_bench.py:57) [example] — Function Text.get_hints takes no inputs beyond the value it is called on.
- function `Text.get_indexdata` (bench/wiki/zim_bench.py:58) [example] — Function Text.get_indexdata takes no inputs beyond the value it is called on.
- function `build_zim` (bench/wiki/zim_bench.py:61) [example] — Function build_zim takes arts, path, fulltext=True.
- function `stats` (bench/wiki/zim_bench.py:83) [example] — Function stats takes ds.
- function `time_each` (bench/wiki/zim_bench.py:90) [example] — Function time_each takes items, f.
- function `drop_caches` (bench/wiki/zim_bench.py:97) [example] — Function drop_caches takes no inputs beyond the value it is called on.
- function `Rng.__init__` (bench/wiki/zim_bench.py:106) [example] — Function Rng.__init__ takes self, s.
- function `Rng.below` (bench/wiki/zim_bench.py:107) [example] — Function Rng.below takes self, n.
- function `fname` (bench/wiki/zim_bench.py:113) [example] — Function fname takes t.
- function `main` (bench/wiki/zim_bench.py:118) [example] — Function main takes corpus, out, scale=1, original=None.
- function `bfs` (bench/wiki/zim_bench.py:152) [example] — Function bfs takes seed.
- function `backlinks` (bench/wiki/zim_bench.py:166) [example] — Function backlinks takes t.
- function `exact` (bench/wiki/zim_bench.py:176) [example] — Function exact takes x.
- function `suggest` (bench/wiki/zim_bench.py:185) [example] — Function suggest takes x.
- function `fulltext_top1` (bench/wiki/zim_bench.py:195) [example] — Function fulltext_top1 takes x.
- function `ft` (bench/wiki/zim_bench.py:204) [example] — Function ft takes x.
- entry point `main` (build.rs:50) — The entry point: the program starts here.
- function `die` (deploy/deploy.sh:59) — Function die takes no inputs beyond the value it is called on.
- function `usage` (deploy/deploy.sh:64) — Function usage takes no inputs beyond the value it is called on.
- function `random_hex` (deploy/deploy.sh:85) — Function random_hex takes no inputs beyond the value it is called on.
- function `install_secret` (deploy/deploy.sh:90) — Function install_secret takes no inputs beyond the value it is called on.
- function `as_owner` (deploy/deploy.sh:101) — Function as_owner takes no inputs beyond the value it is called on.
- function `sponsor_url_ok` (deploy/deploy.sh:108) — Function sponsor_url_ok takes no inputs beyond the value it is called on.
- function `validate_sponsor_url` (deploy/deploy.sh:134) — Function validate_sponsor_url takes no inputs beyond the value it is called on.
- function `resolve_sponsor_url` (deploy/deploy.sh:141) — Function resolve_sponsor_url takes no inputs beyond the value it is called on.
- function `write_sponsor_env` (deploy/deploy.sh:150) — Function write_sponsor_env takes no inputs beyond the value it is called on.
- function `apply_sponsor_env` (deploy/deploy.sh:183) — Function apply_sponsor_env takes no inputs beyond the value it is called on.
- function `dns_points_here` (deploy/deploy.sh:208) — Function dns_points_here takes no inputs beyond the value it is called on.
- function `file_sha256` (deploy/deploy.sh:212) — Function file_sha256 takes no inputs beyond the value it is called on.
- function `read_root_file` (deploy/deploy.sh:218) — Function read_root_file takes no inputs beyond the value it is called on.
- function `nginx_template` (deploy/deploy.sh:223) — Function nginx_template takes no inputs beyond the value it is called on.
- function `render_site_body` (deploy/deploy.sh:240) — Function render_site_body takes no inputs beyond the value it is called on.
- function `adopt_live_ssl` (deploy/deploy.sh:252) — Function adopt_live_ssl takes no inputs beyond the value it is called on.
- function `path_exists` (deploy/deploy.sh:288) — Function path_exists takes no inputs beyond the value it is called on.
- function `drop_missing_dhparam` (deploy/deploy.sh:296) — Function drop_missing_dhparam takes no inputs beyond the value it is called on.
- function `stamp_managed` (deploy/deploy.sh:309) — Function stamp_managed takes no inputs beyond the value it is called on.
- function `write_bootstrap_body` (deploy/deploy.sh:318) — Function write_bootstrap_body takes no inputs beyond the value it is called on.
- function `nginx_disposition` (deploy/deploy.sh:349) — Function nginx_disposition takes no inputs beyond the value it is called on.
- function `with_force` (deploy/deploy.sh:383) — Function with_force takes no inputs beyond the value it is called on.
- function `require_hardened_body` (deploy/deploy.sh:392) — Function require_hardened_body takes no inputs beyond the value it is called on.
- function `install_rendered_site` (deploy/deploy.sh:409) — Function install_rendered_site takes no inputs beyond the value it is called on.
- function `apply_rendered_site` (deploy/deploy.sh:443) — Function apply_rendered_site takes no inputs beyond the value it is called on.
- function `run_check` (deploy/deploy.sh:492) — Function run_check takes no inputs beyond the value it is called on.
- function `main` (deploy/deploy.sh:611) — Function main takes no inputs beyond the value it is called on.
- entry point `main` (examples/wikibench.rs:831) [example] — The entry point: the program starts here.
- function `fmt` (scripts/build_wavefront_explorer.py:30) — Function fmt takes coord.
- function `parse_phext` (scripts/build_wavefront_explorer.py:36) — Function parse_phext takes data.
- function `region_of` (scripts/build_wavefront_explorer.py:56) — Function region_of takes values.
- function `summarize` (scripts/build_wavefront_explorer.py:65) — Function summarize takes evidence.
- function `entries_of` (scripts/build_wavefront_explorer.py:73) — Yield (heading, body) per entry in a person scroll.
- function `handle_of` (scripts/build_wavefront_explorer.py:91) — Function handle_of takes heading.
- function `main` (scripts/build_wavefront_explorer.py:100) — Function main takes no inputs beyond the value it is called on.
- function `fmt` (scripts/build_wavefront_snapshot.py:42) — Function fmt takes coord.
- function `parse_phext` (scripts/build_wavefront_snapshot.py:48) — Dead-reckoning parse: walk bytes, split on delimiters, map to coords.
- function `handle_of` (scripts/build_wavefront_snapshot.py:69) — Function handle_of takes name_part.
- function `main` (scripts/build_wavefront_snapshot.py:78) — Function main takes no inputs beyond the value it is called on.
- function `check` (scripts/mcp-e2e/mcp_e2e.py:45) — Function check takes ok, label, detail="".
- function `skip` (scripts/mcp-e2e/mcp_e2e.py:50) — Function skip takes label, why.
- function `req` (scripts/mcp-e2e/mcp_e2e.py:55) — Function req takes method, path, body=None, headers=None, budget_ms=None, label=None.
- function `b64url` (scripts/mcp-e2e/mcp_e2e.py:73) — Function b64url takes data: bytes.
- function `pkce` (scripts/mcp-e2e/mcp_e2e.py:77) — Function pkce takes no inputs beyond the value it is called on.
- function `consent_fields` (scripts/mcp-e2e/mcp_e2e.py:82) — Function consent_fields takes page: str.
- function `authorize_and_consent` (scripts/mcp-e2e/mcp_e2e.py:92) — GET authorize (expect the consent page), POST approve, return the
- function `token_exchange` (scripts/mcp-e2e/mcp_e2e.py:111) — Function token_exchange takes form, label, extra_headers=None.
- function `rpc` (scripts/mcp-e2e/mcp_e2e.py:119) — Function rpc takes token, method, params=None, req_id=1, headers_extra=None, path="/mcp", label=None, budget_ms=None.
- function `tool` (scripts/mcp-e2e/mcp_e2e.py:134) — Function tool takes token, name, args, label=None, budget_ms=None.
- function `flow_code_and_token` (scripts/mcp-e2e/mcp_e2e.py:141) — Function flow_code_and_token takes *, label, client_id, redirect_uri, state, verifier, token_extra=None, token_headers=None, expect_token_status=200.
- function `pkce_pair` (scripts/mcp-e2e/mcp_e2e.py:170) — Function pkce_pair takes verifier.
- function `assertion` (scripts/mcp-e2e/mcp_e2e.py:384) — Function assertion takes jti.
- function `cleanup` (scripts/mcp-e2e/run.sh:15) — Function cleanup takes no inputs beyond the value it is called on.
- function `stamp` (scripts/mcp-e2e/sdk_flow.py:31) — Function stamp takes msg.
- function `MemoryStorage.__init__` (scripts/mcp-e2e/sdk_flow.py:36) — Function MemoryStorage.__init__ takes no inputs beyond the value it is called on.
- function `MemoryStorage.get_tokens` (scripts/mcp-e2e/sdk_flow.py:40) — Function MemoryStorage.get_tokens takes no inputs beyond the value it is called on.
- function `MemoryStorage.set_tokens` (scripts/mcp-e2e/sdk_flow.py:43) — Function MemoryStorage.set_tokens takes self, tokens.
- function `MemoryStorage.get_client_info` (scripts/mcp-e2e/sdk_flow.py:46) — Function MemoryStorage.get_client_info takes no inputs beyond the value it is called on.
- function `MemoryStorage.set_client_info` (scripts/mcp-e2e/sdk_flow.py:49) — Function MemoryStorage.set_client_info takes self, client_info.
- function `redirect_handler` (scripts/mcp-e2e/sdk_flow.py:53) — Function redirect_handler takes authorization_url: str.
- function `callback_handler` (scripts/mcp-e2e/sdk_flow.py:75) — Function callback_handler takes no inputs beyond the value it is called on.
- function `main` (scripts/mcp-e2e/sdk_flow.py:79) — Function main takes no inputs beyond the value it is called on.
- entry point `main` (src/bin/phext_edit.rs:7879) — The entry point: the program starts here.
- entry point `main` (src/bin/phext_nav.rs:179) — The entry point: the program starts here.
- entry point `main` (src/bin/phext_tts.rs:17) — The entry point: the program starts here.
- command `coord` (src/bin/phext_tts.rs:26) — A command named coord.
- command `c` (src/bin/phext_tts.rs:26) — A command named c.
- command `ssml` (src/bin/phext_tts.rs:33) — A command named ssml.
- command `special` (src/bin/phext_tts.rs:41) — A command named special.
- command `s` (src/bin/phext_tts.rs:41) — A command named s.
- command `table` (src/bin/phext_tts.rs:48) — A command named table.
- command `t` (src/bin/phext_tts.rs:48) — A command named t.
- command `batch` (src/bin/phext_tts.rs:61) — A command named batch.
- command `b` (src/bin/phext_tts.rs:61) — A command named b.
- command `extract` (src/bin/phext_tts.rs:68) — A command named extract.
- command `e` (src/bin/phext_tts.rs:68) — A command named e.
- command `help` (src/bin/phext_tts.rs:75) — A command named help.
- flag `--help` (src/bin/phext_tts.rs:75) — A command line flag, --help (used by: help).
- flag `-h` (src/bin/phext_tts.rs:75) — A command line flag, -h (used by: help).
- entry point `main` (src/bin/uml6d_edit.rs:653) — The entry point: the program starts here.
- enum `Tier` (src/claims.rs:36) — A named choice named Tier.
- function `Tier::grant` (src/claims.rs:44) — Function Tier::grant takes no inputs beyond the value it is called on.
- struct `Grant` (src/claims.rs:70) — A data shape named Grant.
- struct `Claim` (src/claims.rs:78) — A data shape named Claim.
- function `get` (src/claims.rs:115) — The claim an account already holds, if any.
- function `record` (src/claims.rs:128) — Record a claim, or return the one the account already holds — claiming twice changes nothing (a later buy-in updates…
- function `list` (src/claims.rs:153) — Every recorded claim, oldest first.
- function `set_tier` (src/claims.rs:173) — Move an existing claim to a new tier (a buy-in landing, or a map republication finding the account).
- enum `Dimension` (src/coordinate_ext.rs:34) — Dimension enum for editor navigation.
- function `Dimension::from_index` (src/coordinate_ext.rs:67) — Convert 1-9 key index to dimension.
- function `Dimension::name` (src/coordinate_ext.rs:82) — Function Dimension::name takes no inputs beyond the value it is called on.
- function `Dimension::delimiter` (src/coordinate_ext.rs:97) — The delimiter byte for this dimension.
- trait `CoordinateNav` (src/coordinate_ext.rs:123) — [UPSTREAM] Navigation trait: move forward/backward in a specific dimension.
- struct `Position` (src/cursor.rs:12) — Position within a scroll's text.
- function `Position::new` (src/cursor.rs:20) — Function Position::new takes line: usize, col: usize.
- function `Position::origin` (src/cursor.rs:24) — Function Position::origin takes no inputs beyond the value it is called on.
- function `Position::from_offset` (src/cursor.rs:29) — Convert a byte offset in text to a Position.
- function `Position::to_offset` (src/cursor.rs:46) — Convert this Position to a byte offset in text.
- struct `Selection` (src/cursor.rs:76) — A selection range within a scroll.
- function `Selection::cursor` (src/cursor.rs:85) — A zero-width selection (cursor) at a position.
- function `Selection::is_cursor` (src/cursor.rs:90) — Whether this is a zero-width selection (just a cursor).
- function `Selection::start` (src/cursor.rs:95) — The start of the selection (earlier position).
- function `Selection::end` (src/cursor.rs:106) — The end of the selection (later position).
- struct `ScrollCursor` (src/cursor.rs:119) — The cursor state within a scroll.
- function `ScrollCursor::new` (src/cursor.rs:131) — New cursor at the origin (0,0).
- function `ScrollCursor::at` (src/cursor.rs:140) — New cursor at a specific position.
- function `ScrollCursor::position` (src/cursor.rs:149) — Current cursor position (selection head).
- function `ScrollCursor::move_to` (src/cursor.rs:154) — Move cursor to a position, collapsing any selection.
- function `ScrollCursor::select_to` (src/cursor.rs:160) — Extend selection to a position (keep anchor, move head).
- function `ScrollCursor::move_left` (src/cursor.rs:166) — Move cursor left by n characters within text.
- function `ScrollCursor::move_right` (src/cursor.rs:173) — Move cursor right by n characters within text.
- function `ScrollCursor::move_up` (src/cursor.rs:180) — Move cursor up by n lines, preserving column.
- function `ScrollCursor::move_down` (src/cursor.rs:194) — Move cursor down by n lines, preserving column.
- function `ScrollCursor::move_home` (src/cursor.rs:209) — Move to beginning of current line.
- function `ScrollCursor::move_end` (src/cursor.rs:214) — Move to end of current line.
- function `ScrollCursor::move_top` (src/cursor.rs:220) — Move to start of text.
- function `ScrollCursor::move_bottom` (src/cursor.rs:225) — Move to end of text.
- function `ScrollCursor::move_word_forward` (src/cursor.rs:232) — Move to the next word boundary.
- function `ScrollCursor::move_word_backward` (src/cursor.rs:247) — Move to the previous word boundary.
- enum `EditResult` (src/editor.rs:16) — What happened after processing a key.
- enum `CursorStyle` (src/editor.rs:31) — Cursor rendering style.
- struct `EditKey` (src/editor.rs:42) — A key event abstraction (decoupled from any TUI/GUI framework).
- enum `EditKeyCode` (src/editor.rs:50) — A named choice named EditKeyCode.
- function `EditKey::char` (src/editor.rs:68) — Function EditKey::char takes c: char.
- function `EditKey::ctrl` (src/editor.rs:72) — Function EditKey::ctrl takes c: char.
- function `EditKey::special` (src/editor.rs:76) — Function EditKey::special takes code: EditKeyCode.
- struct `EditContext` (src/editor.rs:83) — The editing context passed to modes.
- function `<'a> EditContext<'a>::new` (src/editor.rs:95) — Function <'a> EditContext<'a>::new takes coordinate: Coordinate, text: &'a mut String, cursor: &'a mut ScrollCursor, undo: &'a mut UndoEngine,.
- function `<'a> EditContext<'a>::insert` (src/editor.rs:105) — Insert text at the cursor position.
- function `<'a> EditContext<'a>::insert_newline` (src/editor.rs:116) — Insert a newline at the cursor position.
- function `<'a> EditContext<'a>::delete_backward` (src/editor.rs:121) — Delete n characters before the cursor (backspace).
- function `<'a> EditContext<'a>::delete_forward` (src/editor.rs:135) — Delete n characters at the cursor position (delete key).
- function `<'a> EditContext<'a>::delete_line` (src/editor.rs:148) — Delete the current line.
- function `<'a> EditContext<'a>::replace_selection` (src/editor.rs:173) — Replace the selected text with new text.
- function `<'a> EditContext<'a>::undo` (src/editor.rs:195) — Undo the last edit.
- function `<'a> EditContext<'a>::redo` (src/editor.rs:206) — Redo the next edit.
- trait `EditorMode` (src/editor.rs:221) — The trait that all editor modes implement.
- re-export `holo_phext::index::{LatticeIndex, ScrollSpan}` (src/index.rs:14) — A re-export: holo_phext::index::{LatticeIndex, ScrollSpan} is made reachable through this module, so other code can…
- struct `Join` (src/joins.rs:23) — A data shape named Join.
- function `get` (src/joins.rs:56) — The join an account already holds, if any.
- function `record` (src/joins.rs:68) — Record a join, or return the one the account already holds — the first arrival is the join, and joining twice changes…
- function `list` (src/joins.rs:86) — Every recorded join, oldest first.
- re-export `coordinate_ext::{Dimension, CoordinateNav}` (src/lib.rs:53) — A re-export: coordinate_ext::{Dimension, CoordinateNav} is made reachable through this module, so other code can use…
- re-export `cursor::{ScrollCursor, Position, Selection}` (src/lib.rs:54) — A re-export: cursor::{ScrollCursor, Position, Selection} is made reachable through this module, so other code can use…
- re-export `editor::{EditorMode, EditContext, EditKey, EditKeyCode, EditResult, CursorStyle}` (src/lib.rs:55) — A re-export: editor::{EditorMode, EditContext, EditKey, EditKeyCode, EditResult, CursorStyle} is made reachable…
- re-export `index::{LatticeIndex, ScrollSpan}` (src/lib.rs:56) — A re-export: index::{LatticeIndex, ScrollSpan} is made reachable through this module, so other code can use it from…
- re-export `mmap::MappedLattice` (src/lib.rs:57) — A re-export: mmap::MappedLattice is made reachable through this module, so other code can use it from here.
- re-export `modes::VimMode` (src/lib.rs:58) — A re-export: modes::VimMode is made reachable through this module, so other code can use it from here.
- re-export `names::{NameIndex, fold}` (src/lib.rs:59) — A re-export: names::{NameIndex, fold} is made reachable through this module, so other code can use it from here.
- re-export `navigator::Navigator` (src/lib.rs:60) — A re-export: navigator::Navigator is made reachable through this module, so other code can use it from here.
- re-export `search::{search_lattice, search_lattice_parallel, search_lattice_auto, search_coordinates, SearchHit}` (src/lib.rs:61) — A re-export: search::{search_lattice, search_lattice_parallel, search_lattice_auto, search_coordinates, SearchHit} is…
- re-export `sidecar::{IndexedLattice, IndexSpec, OpenMode, SyncReport, sidecar_dir}` (src/lib.rs:62) — A re-export: sidecar::{IndexedLattice, IndexSpec, OpenMode, SyncReport, sidecar_dir} is made reachable through this…
- re-export `sentron::{Sentron, Neuron, Axon, AxisGroup, SENTRON_CAPACITY}` (src/lib.rs:63) — A re-export: sentron::{Sentron, Neuron, Axon, AxisGroup, SENTRON_CAPACITY} is made reachable through this module, so…
- re-export `stats::{ScrollStats, DimensionDensity, LatticeOverview, Neighborhood}` (src/lib.rs:64) — A re-export: stats::{ScrollStats, DimensionDensity, LatticeOverview, Neighborhood} is made reachable through this…
- re-export `term_index::{TermIndex, TermHit, Bm25Params, tokenize}` (src/lib.rs:65) — A re-export: term_index::{TermIndex, TermHit, Bm25Params, tokenize} is made reachable through this module, so other…
- re-export `undo::{UndoEngine, EditOp, UndoRecord}` (src/lib.rs:66) — A re-export: undo::{UndoEngine, EditOp, UndoRecord} is made reachable through this module, so other code can use it…
- re-export `tts::{CoordPronunciation, byte_to_syllable, dim_to_syllables, to_ssml, generate_pronunciation_doc, special_coords}` (src/lib.rs:67) — A re-export: tts::{CoordPronunciation, byte_to_syllable, dim_to_syllables, to_ssml, generate_pronunciation_doc…
- re-export `uml6d::{Phase, ArtifactAddr, UmlContext, trace_artifact, artifacts_in_phase}` (src/lib.rs:68) — A re-export: uml6d::{Phase, ArtifactAddr, UmlContext, trace_artifact, artifacts_in_phase} is made reachable through…
- function `media_payload_coord` (src/media.rs:32) — Where slot `slot`'s image payload (a binary scroll) lives: 9.9.9/9.9.9/1.1.n.
- function `media_metadata_coord` (src/media.rs:38) — Where slot `slot`'s metadata scroll (JSON text) lives: 9.9.9/9.9.9/1.2.n.
- function `media_slot_of` (src/media.rs:44) — The media slot a coordinate belongs to, if it is a media payload coordinate (9.9.9/9.9.9/1.1.n).
- function `is_well_formed_binary_scroll` (src/media.rs:68) — True when `stored` is a well-formed binary scroll: it carries the canonical header and its body is strict base64 (an…
- struct `ImageKind` (src/media.rs:74) — A raster image format the editor accepts, identified by magic bytes.
- const `PNG` (src/media.rs:81) — A fixed value named PNG, set to ImageKind { content_type: "image/png", extension: "png" }.
- const `JPEG` (src/media.rs:82) — A fixed value named JPEG, set to ImageKind { content_type: "image/jpeg", extension: "jpg" }.
- const `GIF` (src/media.rs:83) — A fixed value named GIF, set to ImageKind { content_type: "image/gif", extension: "gif" }.
- const `WEBP` (src/media.rs:84) — A fixed value named WEBP, set to ImageKind { content_type: "image/webp", extension: "webp" }.
- const `AVIF` (src/media.rs:85) — A fixed value named AVIF, set to ImageKind { content_type: "image/avif", extension: "avif" }.
- function `sniff_image` (src/media.rs:91) — Identify an image payload by its magic bytes — never by a file name or a client-claimed type.
- struct `MappedLattice` (src/mmap.rs:144) — A phext document with copy-on-write editing.
- function `MappedLattice::open` (src/mmap.rs:186) — Open a phext file and build the lattice index.
- function `MappedLattice::open_with_index` (src/mmap.rs:233) — Open a phext file with an index built elsewhere (e.g.
- function `MappedLattice::from_bytes` (src/mmap.rs:267) — Create a MappedLattice from raw bytes (for testing or in-memory use).
- function `MappedLattice::read_scroll` (src/mmap.rs:316) — Read the content of a scroll at the given coordinate.
- function `MappedLattice::read_scroll_bytes` (src/mmap.rs:342) — The scroll's exact stored bytes: the overlay version if edited, otherwise the mmap span, uninterpreted.
- function `MappedLattice::fetch_binary` (src/mmap.rs:367) — The payload a fetch of this scroll returns, mirroring `libphext::binary::fetch_binary` exactly: a well-formed binary…
- function `MappedLattice::is_chain` (src/mmap.rs:381) — True when this file is a chain: more structure than a plain phext's single unclosed member.
- function `MappedLattice::universe_count` (src/mmap.rs:387) — How many universes (member phexts) this file holds.
- function `MappedLattice::universe` (src/mmap.rs:392) — The active universe, numbered from 1.
- function `MappedLattice::member_stats` (src/mmap.rs:397) — Per-universe (scroll count, content bytes), in universe order.
- function `MappedLattice::set_universe` (src/mmap.rs:408) — Make `universe` (numbered from 1) the active universe.
- function `MappedLattice::read_scroll_bytes_in` (src/mmap.rs:434) — A scroll's exact stored bytes in `universe` (numbered from 1), without switching the active universe.
- function `MappedLattice::read_scroll_in` (src/mmap.rs:445) — A scroll's content in `universe` (numbered from 1), without switching the active universe.
- function `MappedLattice::write_scroll` (src/mmap.rs:461) — Write content to a scroll at the given coordinate.
- function `MappedLattice::write_scroll_raw` (src/mmap.rs:476) — Store a scroll's stored form verbatim, with no binary guard.
- function `MappedLattice::write_binary` (src/mmap.rs:496) — Store `payload` at `coord` as a canonical binary scroll (header + base64 of the exact bytes), whatever the payload…
- function `MappedLattice::write_bytes` (src/mmap.rs:505) — Store `payload` at `coord` under the format's ingestion rule (libphext's `stored_form`, behind…
- function `MappedLattice::delete_scroll` (src/mmap.rs:513) — Delete a scroll at the given coordinate.
- function `MappedLattice::has_scroll` (src/mmap.rs:529) — Check if a coordinate has content (considering overlay).
- function `MappedLattice::populated_coordinates` (src/mmap.rs:543) — Get all populated coordinates, including overlay insertions, excluding overlay deletions.
- function `MappedLattice::scroll_count` (src/mmap.rs:568) — Number of populated scrolls (considering overlay).
- function `MappedLattice::is_dirty` (src/mmap.rs:573) — Check if there are unsaved modifications.
- function `MappedLattice::pending_changes` (src/mmap.rs:583) — Number of modified/inserted/deleted scrolls.
- function `MappedLattice::pending_coordinates` (src/mmap.rs:591) — Coordinates with unsaved edits (modified, inserted, or deleted), sorted.
- function `MappedLattice::save` (src/mmap.rs:610) — Save to the backing file.
- function `MappedLattice::save_as` (src/mmap.rs:657) — Save to a specific path.
- function `MappedLattice::to_phext_bytes` (src/mmap.rs:707) — Reconstruct the full phext as bytes from mmap + overlay.
- function `MappedLattice::raw_buffer` (src/mmap.rs:791) — Access the raw mmap buffer for search operations.
- function `MappedLattice::index` (src/mmap.rs:813) — Access the underlying index for navigation queries.
- function `MappedLattice::path` (src/mmap.rs:818) — Get the file path.
- function `MappedLattice::revert` (src/mmap.rs:823) — Discard all unsaved changes, reverting to the mmap state.
- re-export `vim::VimMode` (src/modes/mod.rs:8) — A re-export: vim::VimMode is made reachable through this module, so other code can use it from here.
- struct `VimMode` (src/modes/vim.rs:20) — Vim-style editor mode with Normal/Insert/Visual/Command sub-modes.
- function `VimMode::new` (src/modes/vim.rs:33) — Function VimMode::new takes no inputs beyond the value it is called on.
- function `fold` (src/names.rs:17) — Case-fold a name for lookup: Unicode lowercase with runs of whitespace collapsed to one space and the ends trimmed.
- struct `NameIndex` (src/names.rs:69) — Exact and folded name → coordinate maps.
- function `NameIndex::new` (src/names.rs:77) — Function NameIndex::new takes no inputs beyond the value it is called on.
- function `NameIndex::from_scrolls` (src/names.rs:84) — Build from the lattice: every non-empty line of each scroll that `select` maps to `Some(target)` becomes a name for `target`.
- function `NameIndex::insert` (src/names.rs:116) — Register `name` for `coord`.
- function `NameIndex::get` (src/names.rs:134) — Exact (case-sensitive) lookup.
- function `NameIndex::get_folded` (src/names.rs:139) — Case-insensitive lookup through the folded map.
- function `NameIndex::resolve` (src/names.rs:144) — Exact match first, then folded: what an agent's "open <name>" should call.
- function `NameIndex::len` (src/names.rs:149) — Distinct exact names.
- function `NameIndex::is_empty` (src/names.rs:153) — Function NameIndex::is_empty takes no inputs beyond the value it is called on.
- function `NameIndex::ambiguous` (src/names.rs:159) — How many inserts hit a folded key already owned by a different coordinate.
- struct `Navigator` (src/navigator.rs:13) — The navigator state for one cursor in the lattice.
- function `Navigator::new` (src/navigator.rs:27) — Function Navigator::new takes no inputs beyond the value it is called on.
- function `Navigator::at` (src/navigator.rs:38) — Create a navigator starting at a specific coordinate.
- function `Navigator::position` (src/navigator.rs:49) — Current position in the lattice.
- function `Navigator::active_dimension` (src/navigator.rs:54) — Currently selected dimension.
- function `Navigator::select_dimension` (src/navigator.rs:59) — Select a dimension by index (1-9).
- function `Navigator::move_forward` (src/navigator.rs:69) — Move forward in the active dimension (resets lower dims).
- function `Navigator::move_backward` (src/navigator.rs:78) — Move backward in the active dimension (resets lower dims).
- function `Navigator::goto` (src/navigator.rs:87) — Jump directly to a coordinate.
- function `Navigator::next_populated` (src/navigator.rs:93) — Jump to the next populated coordinate (any dimension).
- function `Navigator::prev_populated` (src/navigator.rs:104) — Jump to the previous populated coordinate (any dimension).
- function `Navigator::set_mark` (src/navigator.rs:115) — Set a mark at the current position.
- function `Navigator::jump_to_mark` (src/navigator.rs:120) — Jump to the most recent mark (and remove it).
- function `Navigator::history_back` (src/navigator.rs:131) — Navigate back in history.
- function `Navigator::history_forward` (src/navigator.rs:142) — Navigate forward in history.
- function `Navigator::dimension_neighbors` (src/navigator.rs:154) — Get neighbors in the active dimension: (prev_value, current_value, next_value).
- struct `NeighborhoodSummary` (src/navigator.rs:179) — Summary of the lattice neighborhood around a coordinate, for rendering the coordinate bar and lattice navigator panel.
- function `Navigator::summarize` (src/navigator.rs:198) — Build a summary of the current neighborhood for UI rendering.
- const `TEST_CSRF` (src/oauth.rs:53) — A fixed value named TEST_CSRF, set to "csrf-test-token".
- struct `Identity` (src/oauth.rs:60) — A data shape named Identity.
- trait `IdentitySource` (src/oauth.rs:67) — awaiting a summary
- struct `Oauth` (src/oauth.rs:171) — awaiting a summary
- struct `Session` (src/oauth.rs:218) — A data shape named Session.
- struct `SsoAssertionBody` (src/oauth.rs:269) — The SSO assertion: a sealed, single-use statement that an edit session exists, carried to an audience's callback in a…
- function `Oauth::from_env` (src/oauth.rs:292) — Function Oauth::from_env takes no inputs beyond the value it is called on.
- function `Oauth::test_ready` (src/oauth.rs:312) — Function Oauth::test_ready takes no inputs beyond the value it is called on.
- function `Oauth::closed` (src/oauth.rs:332) — Function Oauth::closed takes no inputs beyond the value it is called on.
- function `Oauth::ready` (src/oauth.rs:400) — Function Oauth::ready takes no inputs beyond the value it is called on.
- function `Oauth::sso_ready` (src/oauth.rs:410) — True when the Mutable SSO handoff is configured: a real SSO secret and a valid Mutable target.
- function `Oauth::sso_ready_for` (src/oauth.rs:419) — True when the SSO handoff is configured for one audience: the shared SSO secret is real and that audience has a target.
- function `Oauth::sso_desktop_ready` (src/oauth.rs:427) — True when the desktop audience is armed: a real SSO secret and the operator's explicit `PHEXT_SSO_DESKTOP=1`.
- function `Oauth::base` (src/oauth.rs:431) — Function Oauth::base takes no inputs beyond the value it is called on.
- function `Oauth::sso_target` (src/oauth.rs:435) — Function Oauth::sso_target takes no inputs beyond the value it is called on.
- function `Oauth::sso_target_for` (src/oauth.rs:450) — The callback origin for an audience, or None when the audience is not allowlisted or is allowlisted but unconfigured.
- function `Oauth::providers` (src/oauth.rs:463) — Function Oauth::providers takes no inputs beyond the value it is called on.
- function `Oauth::cookie_secure` (src/oauth.rs:480) — Function Oauth::cookie_secure takes no inputs beyond the value it is called on.
- function `Oauth::authorize_url` (src/oauth.rs:485) — Returns the provider URL to redirect to and the sealed pending cookie.
- function `Oauth::finish` (src/oauth.rs:535) — Completes the flow.
- function `Oauth::issue_session` (src/oauth.rs:576) — Seal a fresh session for an identity that has already been established — by the provider exchange in `finish`, or by…
- function `Oauth::read_session` (src/oauth.rs:595) — Function Oauth::read_session takes &self, cookie_header: &str.
- function `Oauth::csrf_matches` (src/oauth.rs:621) — Function Oauth::csrf_matches takes &self, session: &Session, header: &str.
- function `Oauth::mcp_oauth_hash` (src/oauth.rs:634) — A domain-separated, keyed digest for the MCP OAuth 2.1 store.
- function `Oauth::test_session_cookie` (src/oauth.rs:651) — Function Oauth::test_session_cookie takes no inputs beyond the value it is called on.
- function `Oauth::sso_assertion` (src/oauth.rs:701) — Mint the assertion an audience's callback verifies: who this session is, whether it is an editor, and the return path…
- function `Oauth::sso_assertion_desktop` (src/oauth.rs:740) — Mint the desktop audience's assertion.
- function `Oauth::open_sso_assertion` (src/oauth.rs:778) — Open an SSO assertion under the SSO secret — the exchange endpoint is the one place this server ever takes an assertion back, and only the desktop audience's.
- struct `Editors` (src/oauth.rs:843) — Accounts allowed to use the editor once signed in.
- function `Editors::from_env` (src/oauth.rs:876) — Function Editors::from_env takes no inputs beyond the value it is called on.
- function `Editors::parse` (src/oauth.rs:880) — Function Editors::parse takes raw: &str.
- function `Editors::len` (src/oauth.rs:887) — Function Editors::len takes no inputs beyond the value it is called on.
- function `Editors::is_empty` (src/oauth.rs:891) — Function Editors::is_empty takes no inputs beyond the value it is called on.
- function `Editors::matches` (src/oauth.rs:895) — Function Editors::matches takes &self, who: &str, email: &str.
- function `Editors::allows` (src/oauth.rs:904) — An empty list admits every signed-in account.
- function `Editors::load_file` (src/oauth.rs:912) — Point the list at its backing file and load any entries already in it (one per line; blank lines and `#` comments are…
- function `Editors::admit` (src/oauth.rs:940) — Admit an account: add its account id (when valid) and its email (when usable), unless each is already covered, and…
- struct `Limiter` (src/oauth.rs:977) — In-memory cap on sign-in attempts per client address, per ten minutes.
- function `Limiter::standard` (src/oauth.rs:983) — Function Limiter::standard takes no inputs beyond the value it is called on.
- function `Limiter::with_cap` (src/oauth.rs:987) — Function Limiter::with_cap takes cap: usize.
- function `Limiter::hit` (src/oauth.rs:991) — Function Limiter::hit takes &self, key: &str.
- struct `Reply` (src/oauth.rs:1006) — A complete HTTP response for the hand-rolled servers to write out.
- function `Reply::json` (src/oauth.rs:1015) — Function Reply::json takes status: u16, value: serde_json::Value.
- function `Reply::write_to` (src/oauth.rs:1058) — Serialize as an HTTP/1.1 response with `Connection: close`.
- const `HSTS_VALUE` (src/oauth.rs:1080) — One year.
- const `CONTENT_SECURITY_POLICY` (src/oauth.rs:1092) — Policy for the embedded editor and the public pages.
- function `forwarded_https` (src/oauth.rs:1106) — Function forwarded_https takes headers: &HashMap<String, String>.
- function `security_header_block_csp` (src/oauth.rs:1114) — Function security_header_block_csp takes https: bool, csp_override: Option<&str>.
- function `security_header_block` (src/oauth.rs:1132) — Function security_header_block takes https: bool.
- function `status_text` (src/oauth.rs:1136) — Function status_text takes status: u16.
- struct `Gate` (src/oauth.rs:1191) — Access control for the editor servers: an OAuth session, or open when sign-in is not configured.
- function `Gate::new` (src/oauth.rs:1209) — Function Gate::new takes oauth: Oauth, editors: Editors.
- function `Gate::from_env` (src/oauth.rs:1235) — Function Gate::from_env takes no inputs beyond the value it is called on.
- function `Gate::enable_local` (src/oauth.rs:1256) — Admit every direct request as a local developer.
- function `Gate::is_local` (src/oauth.rs:1260) — Function Gate::is_local takes no inputs beyond the value it is called on.
- function `Gate::enable_claims` (src/oauth.rs:1267) — Open claim mode: `/api/claim` accepts claims (the lookup stays public either way).
- function `Gate::claims_open` (src/oauth.rs:1271) — Function Gate::claims_open takes no inputs beyond the value it is called on.
- function `Gate::locked` (src/oauth.rs:1276) — True when some credential is required for the API.
- function `Gate::oauth` (src/oauth.rs:1280) — Function Gate::oauth takes no inputs beyond the value it is called on.
- function `Gate::editors` (src/oauth.rs:1284) — Function Gate::editors takes no inputs beyond the value it is called on.
- function `Gate::session` (src/oauth.rs:1291) — The signed-in session for a request, if any.
- function `Gate::is_editor` (src/oauth.rs:1308) — Whether a session belongs to an approved editor.
- function `Gate::admit_account` (src/oauth.rs:1315) — Admit an account as an editor, effective immediately and persisted to the editors file when one is configured.
- function `Gate::load_editors_file` (src/oauth.rs:1323) — Load additional editor entries from a file (one per line) and make it the backing file for future admissions.
- function `Gate::describe` (src/oauth.rs:1328) — Startup lines describing how the API is protected.
- function `Gate::route` (src/oauth.rs:1352) — Handles `/api/session` and `/api/auth/*`.
- function `Gate::admit` (src/oauth.rs:1396) — Decides whether a non-auth request may proceed.
- function `Gate::sso` (src/oauth.rs:1545) — `GET /api/auth/sso/<aud>` — the cross-site handoff to one allowlisted audience (`mutable`, `exocortexia`, `wishnode`).
- function `cookie_header` (src/oauth.rs:1743) — Function cookie_header takes name: &str, value: &str, max_age: i64, secure: bool.
- function `safe_next` (src/oauth.rs:1850) — Keeps the post-sign-in redirect on this site: a path, optionally with a `#coordinate` fragment.
- function `loopback_return` (src/oauth.rs:1888) — Admit a desktop return URL: the address of the listener the app opened for this one sign-in attempt.
- function `valid_state_nonce` (src/oauth.rs:1930) — The desktop attempt's state nonce: long enough to be unguessable, plain enough to travel in URLs and assertions unchanged.
- function `valid_who` (src/oauth.rs:1937) — Function valid_who takes who: &str.
- function `display_name` (src/oauth.rs:1955) — Function display_name takes raw: &str, fallback: &str.
- function `clean_email` (src/oauth.rs:1970) — Function clean_email takes raw: &str.
- function `clean_handle` (src/oauth.rs:1991) — Function clean_handle takes raw: &str.
- function `clean_twitter_handle` (src/oauth.rs:2004) — An X username verbatim: `[A-Za-z0-9_]{1,15}`, case preserved.
- function `token_eq` (src/oauth.rs:2016) — Constant-time comparison; an empty expected value never matches.
- function `is_loopback_bind` (src/oauth.rs:2285) — True when `host` is a loopback listen address: 127.0.0.0/8, `::1`, or `localhost`.
- struct `SearchHit` (src/search.rs:19) — A search hit: coordinate + context around the match.
- function `search_lattice` (src/search.rs:132) — Search the lattice for a text pattern (single-threaded).
- function `search_lattice_parallel` (src/search.rs:177) — Search the lattice for a text pattern across all available cores.
- function `search_lattice_auto` (src/search.rs:261) — Auto-selecting search: picks serial or parallel based on data characteristics.
- function `search_coordinates` (src/search.rs:274) — Search for coordinates matching a coordinate pattern string.
- function `build_index_parallel` (src/search.rs:292) — Parallel index build — partition the buffer into chunks, scan each for delimiter boundaries, then merge.
- enum `AxisGroup` (src/sentron.rs:36) — The two fundamental axis groups in the sentron topology.
- function `AxisGroup::name` (src/sentron.rs:44) — Function AxisGroup::name takes no inputs beyond the value it is called on.
- function `AxisGroup::dimensions` (src/sentron.rs:52) — The 4 dimensions in this axis group.
- struct `Axon` (src/sentron.rs:72) — One connection axis: a dimension with backward/forward reach.
- function `Axon::total` (src/sentron.rs:87) — Total reachable coordinates along this axis.
- function `Axon::weight` (src/sentron.rs:92) — Visual weight: how "thick" this connection is (0.0 to 1.0).
- struct `Neuron` (src/sentron.rs:100) — A neuron in the sentron: a populated coordinate with its 8 axons.
- struct `Sentron` (src/sentron.rs:110) — The sentron: 40 nearest populated coordinates to the cursor, with full 8-way connectivity analysis.
- const `SENTRON_CAPACITY` (src/sentron.rs:126) — The maximum number of neurons in a sentron.
- const `CONNECTIONS_PER_NEURON` (src/sentron.rs:129) — The number of connections per neuron.
- function `Sentron::build` (src/sentron.rs:136) — Build a sentron centered on a coordinate.
- function `Sentron::size` (src/sentron.rs:175) — How many neurons are in this sentron.
- function `Sentron::structural_axons` (src/sentron.rs:180) — The structural (spatial) axons: Library, Shelf, Series, Collection.
- function `Sentron::sequential_axons` (src/sentron.rs:185) — The sequential (temporal) axons: Volume, Book, Chapter, Section.
- function `Sentron::topology_string` (src/sentron.rs:191) — Render the sentron as a compact topology string.
- function `is_break` (src/sidecar.rs:83) — Whether `b` is a phext dimension break.
- type `Selector` (src/sidecar.rs:89) — Maps a source scroll to the coordinate an index entry should point at, or `None` to leave the scroll out of that index.
- struct `IndexSpec` (src/sidecar.rs:94) — Which scrolls feed which index.
- function `IndexSpec::new` (src/sidecar.rs:103) — Function IndexSpec::new takes label: &str, names: Selector, terms: Selector.
- function `IndexSpec::scrolls` (src/sidecar.rs:108) — Every scroll is its own term document; no names.
- enum `OpenMode` (src/sidecar.rs:115) — How `IndexedLattice::open` got its indexes.
- struct `SyncReport` (src/sidecar.rs:127) — What a `save()` or sync did.
- function `sidecar_dir` (src/sidecar.rs:137) — The sidecar directory for a phext path: `wiki.phext` → `wiki.phext.idx`.
- struct `IndexedLattice` (src/sidecar.rs:733) — A `MappedLattice` whose span, name, and term indexes persist in a sidecar.
- function `IndexedLattice::open` (src/sidecar.rs:746) — Open `path` with indexes from its sidecar: load them if they match the source, patch them if the source changed, or…
- function `IndexedLattice::rebuild` (src/sidecar.rs:793) — Discard the sidecar and index every scroll again.
- function `IndexedLattice::lattice` (src/sidecar.rs:800) — Function IndexedLattice::lattice takes no inputs beyond the value it is called on.
- function `IndexedLattice::lattice_mut` (src/sidecar.rs:805) — Edit through the lattice, then `save()` to persist and re-sync the indexes.
- function `IndexedLattice::sidecar` (src/sidecar.rs:810) — The sidecar directory.
- function `IndexedLattice::delta_len` (src/sidecar.rs:815) — Scrolls changed since the base (entries in the delta).
- function `IndexedLattice::set_compact_threshold` (src/sidecar.rs:821) — Fold the delta into the base once it holds more than `n` scrolls.
- function `IndexedLattice::save` (src/sidecar.rs:827) — Save pending edits to the source, then record exactly the edited scrolls in the delta (or compact, past the threshold).
- function `IndexedLattice::compact` (src/sidecar.rs:837) — Fold the delta into a new base now.
- function `IndexedLattice::name` (src/sidecar.rs:845) — Exact name lookup (first owner in document order wins).
- function `IndexedLattice::resolve` (src/sidecar.rs:851) — Exact lookup, then case-folded.
- function `IndexedLattice::search` (src/sidecar.rs:882) — BM25-ranked documents for a free-text query (terms OR'ed), best first.
- function `IndexedLattice::doc_count` (src/sidecar.rs:929) — Live term documents.
- const `ENV_NAME` (src/sponsor.rs:10) — Environment variable deploy.sh writes into the phext-edit service environment.
- const `SLOT` (src/sponsor.rs:13) — Placeholder in the public HTML.
- struct `SponsorOffer` (src/sponsor.rs:19) — A configured donation URL, or nothing.
- function `SponsorOffer::hidden` (src/sponsor.rs:24) — Function SponsorOffer::hidden takes no inputs beyond the value it is called on.
- function `SponsorOffer::from_env` (src/sponsor.rs:31) — Read [`ENV_NAME`].
- function `SponsorOffer::parse` (src/sponsor.rs:49) — Accept a Stripe Payment Link (or any https URL safe to place in an href).
- function `SponsorOffer::url` (src/sponsor.rs:56) — Function SponsorOffer::url takes no inputs beyond the value it is called on.
- function `SponsorOffer::markup` (src/sponsor.rs:61) — The visible control, or an empty string when the link is hidden.
- function `apply_slot` (src/sponsor.rs:76) — Replace every [`SLOT`] in `html`.
- struct `ScrollStats` (src/stats.rs:13) — Per-scroll statistics for the status bar and info panel.
- function `ScrollStats::from_bytes` (src/stats.rs:22) — Compute stats from raw scroll bytes.
- function `ScrollStats::size_display` (src/stats.rs:33) — Human-readable size.
- struct `DimensionDensity` (src/stats.rs:47) — Density profile for one dimension: how many populated scrolls exist at each value along that dimension.
- function `DimensionDensity::build` (src/stats.rs:59) — Build the density profile for one dimension across all populated coordinates.
- function `DimensionDensity::sparkline` (src/stats.rs:77) — Render a sparkline string showing relative density.
- function `DimensionDensity::distinct_values` (src/stats.rs:120) — How many distinct values exist in this dimension.
- function `DimensionDensity::range` (src/stats.rs:125) — The range of populated values.
- struct `Neighborhood` (src/stats.rs:139) — Neighborhood context: what's near the current coordinate.
- function `Neighborhood::build` (src/stats.rs:152) — Build neighborhood context for a coordinate.
- struct `LatticeOverview` (src/stats.rs:195) — Global lattice overview stats.
- function `LatticeOverview::build` (src/stats.rs:206) — Build a complete overview of the lattice.
- enum `SubmissionStatus` (src/submissions.rs:20) — A named choice named SubmissionStatus.
- struct `Submission` (src/submissions.rs:27) — A data shape named Submission.
- function `sanitize_filename` (src/submissions.rs:65) — Reduce an offered file name to a bare, safe `*.phext` name, or None when nothing usable remains.
- function `count_scrolls` (src/submissions.rs:86) — Count the populated scrolls in a phext text.
- function `create` (src/submissions.rs:116) — Store a new pending submission.
- function `list` (src/submissions.rs:159) — Every submission in the directory, oldest first.
- function `get` (src/submissions.rs:181) — One submission by id, or None.
- function `content` (src/submissions.rs:194) — The uploaded phext bytes for a submission.
- function `set_decision` (src/submissions.rs:203) — Record an accept or decline.
- struct `Bm25Params` (src/term_index.rs:22) — BM25 tuning.
- struct `TermHit` (src/term_index.rs:37) — A ranked hit: the document coordinate and its BM25 score.
- struct `TermIndex` (src/term_index.rs:50) — Inverted index over lattice documents with BM25 ranking.
- function `tokenize` (src/term_index.rs:67) — Split text into index terms: lowercased runs of letters/digits, at least 2 chars.
- function `TermIndex::new` (src/term_index.rs:91) — An empty index.
- function `TermIndex::build` (src/term_index.rs:106) — Index every populated scroll as its own document (overlay edits included).
- function `TermIndex::build_grouped` (src/term_index.rs:113) — Index scrolls grouped into documents: `group` maps a scroll coordinate to the coordinate of the document it belongs to, or `None` to leave it out.
- function `TermIndex::upsert` (src/term_index.rs:177) — Add or replace the document at `coord` with `text`.
- function `TermIndex::remove` (src/term_index.rs:184) — Remove the document at `coord`.
- function `TermIndex::search` (src/term_index.rs:200) — Rank documents against a free-text query (terms are OR'ed).
- function `TermIndex::doc_count` (src/term_index.rs:240) — Number of live documents.
- function `TermIndex::term_count` (src/term_index.rs:245) — Number of distinct terms.
- function `TermIndex::retired` (src/term_index.rs:250) — Documents replaced or removed since the last build.
- function `TermIndex::document_frequency` (src/term_index.rs:255) — How many documents (including retired ones) contain `term`.
- function `TermIndex::heap_bytes` (src/term_index.rs:260) — Approximate heap footprint in bytes.
- const `MAX_Q_BYTES` (src/traversals.rs:23) — The largest `q` string a record may carry (64 KiB).
- struct `Traversal` (src/traversals.rs:26) — A data shape named Traversal.
- function `valid_name` (src/traversals.rs:66) — A name is usable when, trimmed, it holds 1..=120 chars.
- function `count_steps` (src/traversals.rs:107) — How many steps of a `q` string parse under the tour grammar.
- function `valid_q` (src/traversals.rs:113) — A `q` string is storable when it is non-empty, within the storage bound, and at least one of its steps parses.
- function `create` (src/traversals.rs:126) — Store a new traversal for `who`.
- function `list` (src/traversals.rs:152) — Every traversal in the directory, oldest first.
- function `get` (src/traversals.rs:174) — One traversal by id, or None.
- function `update` (src/traversals.rs:188) — Replace a record's name and `q` (ownership is the caller's check, as in `submissions::set_decision`).
- function `remove` (src/traversals.rs:200) — Delete a record.
- const `ZERO_SOUND` (src/tts.rs:14) — Zero = "om" — the nada sound, silence between words
- function `byte_to_syllable` (src/tts.rs:47) — Convert a single byte value (0-255) to a syllable
- function `dim_to_syllables` (src/tts.rs:62) — Convert a coordinate dimension value (e.g., 1, 5, 9) to a syllable sequence
- struct `SubCoord` (src/tts.rs:82) — A 3D subcoordinate (e.g., 1.1.1)
- function `SubCoord::new` (src/tts.rs:89) — Function SubCoord::new takes a: u32, b: u32, c: u32.
- function `SubCoord::pronounce` (src/tts.rs:94) — Pronounce as "a b c" syllables
- function `SubCoord::pronounce_verbose` (src/tts.rs:103) — Pronounce with element names
- struct `CoordPronunciation` (src/tts.rs:113) — Full 9D coordinate pronunciation
- function `CoordPronunciation::from_coordinate` (src/tts.rs:121) — Create from a libphext Coordinate
- function `CoordPronunciation::parse` (src/tts.rs:142) — Parse from string "1.1.1/1.1.1/1.1.1"
- function `CoordPronunciation::pronounce` (src/tts.rs:167) — Full pronunciation with om boundaries
- function `CoordPronunciation::pronounce_compact` (src/tts.rs:176) — Compact pronunciation (no om boundaries)
- function `CoordPronunciation::pronounce_verbose` (src/tts.rs:185) — Verbose with dimension labels
- function `CoordPronunciation::ipa_hint` (src/tts.rs:195) — Get IPA-style phonetic hint
- function `to_ssml` (src/tts.rs:205) — Generate SSML for TTS engines (Google, Amazon Polly, etc.)
- function `generate_pronunciation_doc` (src/tts.rs:225) — Generate a batch pronunciation document
- function `special_coords` (src/tts.rs:257) — Special coordinates with meaning
- enum `Phase` (src/uml6d.rs:27) — The 10 SDLC phases mapped to collection values.
- function `Phase::from_collection` (src/uml6d.rs:41) — Function Phase::from_collection takes c: usize.
- function `Phase::name` (src/uml6d.rs:57) — Function Phase::name takes no inputs beyond the value it is called on.
- function `Phase::short` (src/uml6d.rs:72) — Function Phase::short takes no inputs beyond the value it is called on.
- function `Phase::color` (src/uml6d.rs:88) — CSS color for this phase.
- function `Phase::all` (src/uml6d.rs:104) — All phases in order.
- struct `ArtifactAddr` (src/uml6d.rs:116) — A 5D artifact address — stable across SDLC phases.
- function `ArtifactAddr::from_coordinate` (src/uml6d.rs:126) — Extract the 5D artifact address from a full coordinate.
- function `ArtifactAddr::to_coordinate` (src/uml6d.rs:138) — Build a full coordinate by combining this address with an SDLC phase and outer dimensions (defaulting outer to 1.1.1).
- function `ArtifactAddr::to_coordinate_with_outer` (src/uml6d.rs:143) — Build a full coordinate with explicit outer dimensions.
- function `ArtifactAddr::display` (src/uml6d.rs:166) — Display as volume.book.chapter/section.scroll
- struct `UmlContext` (src/uml6d.rs:179) — Full 6D UML context for a coordinate.
- function `UmlContext::from_coordinate` (src/uml6d.rs:187) — Function UmlContext::from_coordinate takes c: &Coordinate.
- function `trace_artifact` (src/uml6d.rs:202) — Trace an artifact across all SDLC phases.
- function `artifacts_in_phase` (src/uml6d.rs:217) — Find all populated 5D addresses within a given phase.
- struct `DNode` (src/uml6d_diagram.rs:54) — A node in a 6D UML diagram: one artifact, drawn in one phase lane.
- struct `DEdge` (src/uml6d_diagram.rs:64) — A typed relation between two artifacts.
- struct `Diagram` (src/uml6d_diagram.rs:73) — A 6D UML diagram: nodes + edges under one projection phase.
- struct `ParseError` (src/uml6d_diagram.rs:85) — A data shape named ParseError.
- function `parse_addr` (src/uml6d_diagram.rs:134) — Parse a 5D artifact address in display form: volume.book.chapter/section.scroll
- function `parse_diagram` (src/uml6d_diagram.rs:224) — Parse the diagram notation.
- function `Diagram::to_notation` (src/uml6d_diagram.rs:384) — Emit the canonical notation for this diagram.
- function `Diagram::project` (src/uml6d_diagram.rs:427) — Project the diagram into another phase: node lanes are re-read in the new phase, but every artifact address is unchanged.
- function `Diagram::to_text` (src/uml6d_diagram.rs:446) — Plain-text rendering: lanes grouped in phase order, edges listed.
- function `Diagram::to_svg` (src/uml6d_diagram.rs:491) — SVG rendering.
- enum `EditOp` (src/undo.rs:33) — An edit operation that can be undone/redone.
- function `EditOp::apply` (src/undo.rs:54) — Apply this operation to text, returning the new text.
- function `EditOp::inverse` (src/undo.rs:85) — Compute the inverse operation (for undo).
- function `EditOp::to_scroll` (src/undo.rs:104) — Serialize to the phext scroll format.
- function `EditOp::from_scroll` (src/undo.rs:130) — Deserialize from a phext scroll.
- struct `UndoRecord` (src/undo.rs:180) — An undo record: operation + cursor state.
- struct `UndoEngine` (src/undo.rs:258) — The undo engine: manages edit history across all scrolls.
- function `UndoEngine::new` (src/undo.rs:266) — Function UndoEngine::new takes no inputs beyond the value it is called on.
- function `UndoEngine::record` (src/undo.rs:274) — Record an edit operation.
- function `UndoEngine::undo` (src/undo.rs:295) — Undo the last edit on a scroll.
- function `UndoEngine::redo` (src/undo.rs:302) — Redo the next edit on a scroll.
- function `UndoEngine::can_undo` (src/undo.rs:309) — Can we undo on this scroll?
- function `UndoEngine::can_redo` (src/undo.rs:314) — Can we redo on this scroll?
- function `UndoEngine::depth` (src/undo.rs:319) — Undo depth for a scroll.
- function `UndoEngine::total_edits` (src/undo.rs:324) — Total edits recorded globally.
- function `UndoEngine::dirty_scrolls` (src/undo.rs:329) — Which scrolls have been modified.
- function `UndoEngine::to_work_phext` (src/undo.rs:343) — Serialize the full undo history to a phext byte vector.
- function `UndoEngine::from_work_phext` (src/undo.rs:377) — Load undo history from a work phext.
- enum `Region` (src/wavefront.rs:23) — A named choice named Region.
- function `Region::label` (src/wavefront.rs:31) — Display label, exactly as the phext's index words it.
- struct `Placement` (src/wavefront.rs:50) — A data shape named Placement.
- struct `Person` (src/wavefront.rs:59) — A data shape named Person.
- function `normalize_handle` (src/wavefront.rs:118) — Reduce a user-typed handle to its lookup key: trimmed, one leading `@` dropped, lowercased, underscores kept.
- function `lookup` (src/wavefront.rs:130) — The map's entry for a handle, however the visitor typed it.
- function `snapshot_handles` (src/wavefront.rs:136) — How many distinct accounts the snapshot holds.
- function `snapshot_placements` (src/wavefront.rs:141) — Total placements across all accounts (trails count twice).

---

## function — Article.__init__

Found in bench/wiki/extract_zim.py at line 27.

Surface: public API.

Signature: `def __init__(self)`

What it does: Function Article.__init__ takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Article.handle_starttag

Found in bench/wiki/extract_zim.py at line 31.

Surface: public API.

Signature: `def handle_starttag(self, tag, attrs)`

What it does: Function Article.handle_starttag takes self, tag, attrs. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Article.handle_endtag

Found in bench/wiki/extract_zim.py at line 41.

Surface: public API.

Signature: `def handle_endtag(self, tag)`

What it does: Function Article.handle_endtag takes self, tag. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Article.handle_data

Found in bench/wiki/extract_zim.py at line 47.

Surface: public API.

Signature: `def handle_data(self, data)`

What it does: Function Article.handle_data takes self, data. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Article.text

Found in bench/wiki/extract_zim.py at line 51.

Surface: public API.

Signature: `def text(self)`

What it does: Function Article.text takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning re.sub(r"\n{3,}", "\n\n", t).strip().

(Where these words came from: inferred.)

---

## function — main

Found in bench/wiki/extract_zim.py at line 58.

Surface: public API.

Signature: `def main(zim_path, out_dir)`

What it does: Function main takes zim_path, out_dir. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in bench/wiki/make_queries.py at line 22.

Surface: public API.

Signature: `def main(d, n=500)`

What it does: Function main takes d, n=500. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — load

Found in bench/wiki/report.py at line 6.

Surface: public API.

Signature: `def load(work, s, name)`

What it does: Function load takes work, s, name. Its return type is unannotated; the code shows it returning json.load(open(p)) if os.path.exists(p) else {}.

(Where these words came from: inferred.)

---

## function — lat

Found in bench/wiki/report.py at line 11.

Surface: public API.

Signature: `def lat(v, key="p50_us")`

What it does: Function lat takes v, key="p50_us". Its return type is unannotated; the code shows it returning "—"; f"{x / 1000:.1f} ms" if x >= 1000 else f"{x:.1f}.

(Where these words came from: inferred.)

---

## function — mb

Found in bench/wiki/report.py at line 18.

Surface: public API.

Signature: `def mb(b)`

What it does: Function mb takes b. Its return type is unannotated; the code shows it returning f"{b / 1e6:.2f} MB".

(Where these words came from: inferred.)

---

## function — table

Found in bench/wiki/report.py at line 22.

Surface: public API.

Signature: `def table(head, rows)`

What it does: Function table takes head, rows. Its return type is unannotated; the code shows it returning "\n".join(out) + "\n".

(Where these words came from: inferred.)

---

## function — main

Found in bench/wiki/report.py at line 28.

Surface: public API.

Signature: `def main(work, *scales)`

What it does: Function main takes work, *scales. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — tree

Found in bench/wiki/storage.py at line 16.

Surface: public API.

Signature: `def tree(p)`

What it does: Function tree takes p. Its return type is unannotated; the code shows it returning app, alloc, inodes.

(Where these words came from: inferred.)

---

## function — one

Found in bench/wiki/storage.py at line 25.

Surface: public API.

Signature: `def one(p)`

What it does: Function one takes p. Its return type is unannotated; the code shows it returning st.st_size, st.st_blocks * 512, 1.

(Where these words came from: inferred.)

---

## function — main

Found in bench/wiki/storage.py at line 30.

Surface: public API.

Signature: `def main(out)`

What it does: Function main takes out. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — load_corpus

Found in bench/wiki/zim_bench.py at line 20.

Surface: example.

Signature: `def load_corpus(d, scale)`

What it does: Function load_corpus takes d, scale. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — Doc.__init__

Found in bench/wiki/zim_bench.py at line 39.

Surface: example.

Signature: `def __init__(self, title, text)`

What it does: Function Doc.__init__ takes self, title, text. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Doc.has_indexdata

Found in bench/wiki/zim_bench.py at line 42.

Surface: example.

Signature: `def has_indexdata(self): return True`

What it does: Function Doc.has_indexdata takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Doc.get_title

Found in bench/wiki/zim_bench.py at line 43.

Surface: example.

Signature: `def get_title(self): return self.t`

What it does: Function Doc.get_title takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Doc.get_content

Found in bench/wiki/zim_bench.py at line 44.

Surface: example.

Signature: `def get_content(self): return self.x`

What it does: Function Doc.get_content takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Doc.get_keywords

Found in bench/wiki/zim_bench.py at line 45.

Surface: example.

Signature: `def get_keywords(self): return ""`

What it does: Function Doc.get_keywords takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Doc.get_wordcount

Found in bench/wiki/zim_bench.py at line 46.

Surface: example.

Signature: `def get_wordcount(self): return len(self.x.split())`

What it does: Function Doc.get_wordcount takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Text.__init__

Found in bench/wiki/zim_bench.py at line 50.

Surface: example.

Signature: `def __init__(self, path, title, text, front, index)`

What it does: Function Text.__init__ takes self, path, title, text, front, index. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Text.get_path

Found in bench/wiki/zim_bench.py at line 53.

Surface: example.

Signature: `def get_path(self): return self.p`

What it does: Function Text.get_path takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Text.get_title

Found in bench/wiki/zim_bench.py at line 54.

Surface: example.

Signature: `def get_title(self): return self.t`

What it does: Function Text.get_title takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Text.get_mimetype

Found in bench/wiki/zim_bench.py at line 55.

Surface: example.

Signature: `def get_mimetype(self): return "text/plain"`

What it does: Function Text.get_mimetype takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Text.get_contentprovider

Found in bench/wiki/zim_bench.py at line 56.

Surface: example.

Signature: `def get_contentprovider(self): return StringProvider(self.x)`

What it does: Function Text.get_contentprovider takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Text.get_hints

Found in bench/wiki/zim_bench.py at line 57.

Surface: example.

Signature: `def get_hints(self): return {Hint.FRONT_ARTICLE: self.f, Hint.COMPRESS: True}`

What it does: Function Text.get_hints takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Text.get_indexdata

Found in bench/wiki/zim_bench.py at line 58.

Surface: example.

Signature: `def get_indexdata(self): return Doc(self.t, self.x) if self.i else None`

What it does: Function Text.get_indexdata takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — build_zim

Found in bench/wiki/zim_bench.py at line 61.

Surface: example.

Signature: `def build_zim(arts, path, fulltext=True)`

What it does: Function build_zim takes arts, path, fulltext=True. Its return type is unannotated; the code shows it returning (time.perf_counter() - t) * 1000.

(Where these words came from: inferred.)

---

## function — stats

Found in bench/wiki/zim_bench.py at line 83.

Surface: example.

Signature: `def stats(ds)`

What it does: Function stats takes ds. Its return type is unannotated; the code shows it returning {"n": len(ds), "p50_us": pick(.5), "p90_us": pic.

(Where these words came from: inferred.)

---

## function — time_each

Found in bench/wiki/zim_bench.py at line 90.

Surface: example.

Signature: `def time_each(items, f)`

What it does: Function time_each takes items, f. Its return type is unannotated; the code shows it returning stats(ds).

(Where these words came from: inferred.)

---

## function — drop_caches

Found in bench/wiki/zim_bench.py at line 97.

Surface: example.

Signature: `def drop_caches()`

What it does: Function drop_caches takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning False.

(Where these words came from: inferred.)

---

## function — Rng.__init__

Found in bench/wiki/zim_bench.py at line 106.

Surface: example.

Signature: `def __init__(self, s): self.s = s`

What it does: Function Rng.__init__ takes self, s. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Rng.below

Found in bench/wiki/zim_bench.py at line 107.

Surface: example.

Signature: `def below(self, n)`

What it does: Function Rng.below takes self, n. Its return type is unannotated; the code shows it returning self.s % n.

(Where these words came from: inferred.)

---

## function — fname

Found in bench/wiki/zim_bench.py at line 113.

Surface: example.

Signature: `def fname(t)`

What it does: Function fname takes t. Its return type is unannotated; the code shows it returning s  # titles in this corpus fit in 240 bytes; the.

(Where these words came from: inferred.)

---

## function — main

Found in bench/wiki/zim_bench.py at line 118.

Surface: example.

Signature: `def main(corpus, out, scale=1, original=None)`

What it does: Function main takes corpus, out, scale=1, original=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — bfs

Found in bench/wiki/zim_bench.py at line 152.

Surface: example.

Signature: `def bfs(seed)`

What it does: Function bfs takes seed. Its return type is unannotated; the code shows it returning b.

(Where these words came from: inferred.)

---

## function — backlinks

Found in bench/wiki/zim_bench.py at line 166.

Surface: example.

Signature: `def backlinks(t)`

What it does: Function backlinks takes t. Its return type is unannotated; the code shows it returning sum(1 for a in titles if titles[t] in bytes(z.ge.

(Where these words came from: inferred.)

---

## function — exact

Found in bench/wiki/zim_bench.py at line 176.

Surface: example.

Signature: `def exact(x)`

What it does: Function exact takes x. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — suggest

Found in bench/wiki/zim_bench.py at line 185.

Surface: example.

Signature: `def suggest(x)`

What it does: Function suggest takes x. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — fulltext_top1

Found in bench/wiki/zim_bench.py at line 195.

Surface: example.

Signature: `def fulltext_top1(x)`

What it does: Function fulltext_top1 takes x. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — ft

Found in bench/wiki/zim_bench.py at line 204.

Surface: example.

Signature: `def ft(x)`

What it does: Function ft takes x. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## entry point — main

Found in build.rs at line 50.

Surface: public API.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## function — die

Found in deploy/deploy.sh at line 59.

Surface: public API.

Signature: `die()`

What it does: Function die takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — usage

Found in deploy/deploy.sh at line 64.

Surface: public API.

Signature: `usage()`

What it does: Function usage takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — random_hex

Found in deploy/deploy.sh at line 85.

Surface: public API.

Signature: `random_hex()`

What it does: Function random_hex takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — install_secret

Found in deploy/deploy.sh at line 90.

Surface: public API.

Signature: `install_secret()`

What it does: Function install_secret takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — as_owner

Found in deploy/deploy.sh at line 101.

Surface: public API.

Signature: `as_owner()`

What it does: Function as_owner takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — sponsor_url_ok

Found in deploy/deploy.sh at line 108.

Surface: public API.

Signature: `sponsor_url_ok()`

What it does: Function sponsor_url_ok takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — validate_sponsor_url

Found in deploy/deploy.sh at line 134.

Surface: public API.

Signature: `validate_sponsor_url()`

What it does: Function validate_sponsor_url takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — resolve_sponsor_url

Found in deploy/deploy.sh at line 141.

Surface: public API.

Signature: `resolve_sponsor_url()`

What it does: Function resolve_sponsor_url takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — write_sponsor_env

Found in deploy/deploy.sh at line 150.

Surface: public API.

Signature: `write_sponsor_env()`

What it does: Function write_sponsor_env takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — apply_sponsor_env

Found in deploy/deploy.sh at line 183.

Surface: public API.

Signature: `apply_sponsor_env()`

What it does: Function apply_sponsor_env takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — dns_points_here

Found in deploy/deploy.sh at line 208.

Surface: public API.

Signature: `dns_points_here()`

What it does: Function dns_points_here takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — file_sha256

Found in deploy/deploy.sh at line 212.

Surface: public API.

Signature: `file_sha256()`

What it does: Function file_sha256 takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — read_root_file

Found in deploy/deploy.sh at line 218.

Surface: public API.

Signature: `read_root_file()`

What it does: Function read_root_file takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — nginx_template

Found in deploy/deploy.sh at line 223.

Surface: public API.

Signature: `nginx_template()`

What it does: Function nginx_template takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — render_site_body

Found in deploy/deploy.sh at line 240.

Surface: public API.

Signature: `render_site_body()`

What it does: Function render_site_body takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — adopt_live_ssl

Found in deploy/deploy.sh at line 252.

Surface: public API.

Signature: `adopt_live_ssl()`

What it does: Function adopt_live_ssl takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — path_exists

Found in deploy/deploy.sh at line 288.

Surface: public API.

Signature: `path_exists()`

What it does: Function path_exists takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — drop_missing_dhparam

Found in deploy/deploy.sh at line 296.

Surface: public API.

Signature: `drop_missing_dhparam()`

What it does: Function drop_missing_dhparam takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — stamp_managed

Found in deploy/deploy.sh at line 309.

Surface: public API.

Signature: `stamp_managed()`

What it does: Function stamp_managed takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — write_bootstrap_body

Found in deploy/deploy.sh at line 318.

Surface: public API.

Signature: `write_bootstrap_body()`

What it does: Function write_bootstrap_body takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — nginx_disposition

Found in deploy/deploy.sh at line 349.

Surface: public API.

Signature: `nginx_disposition()`

What it does: Function nginx_disposition takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — with_force

Found in deploy/deploy.sh at line 383.

Surface: public API.

Signature: `with_force()`

What it does: Function with_force takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — require_hardened_body

Found in deploy/deploy.sh at line 392.

Surface: public API.

Signature: `require_hardened_body()`

What it does: Function require_hardened_body takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — install_rendered_site

Found in deploy/deploy.sh at line 409.

Surface: public API.

Signature: `install_rendered_site()`

What it does: Function install_rendered_site takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — apply_rendered_site

Found in deploy/deploy.sh at line 443.

Surface: public API.

Signature: `apply_rendered_site()`

What it does: Function apply_rendered_site takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — run_check

Found in deploy/deploy.sh at line 492.

Surface: public API.

Signature: `run_check()`

What it does: Function run_check takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — main

Found in deploy/deploy.sh at line 611.

Surface: public API.

Signature: `main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## entry point — main

Found in examples/wikibench.rs at line 831.

Surface: example.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## function — fmt

Found in scripts/build_wavefront_explorer.py at line 30.

Surface: public API.

Signature: `def fmt(coord)`

What it does: Function fmt takes coord. Its return type is unannotated; the code shows it returning ".".join(str(c) for c in coord[0:3]) + "/" + \.

(Where these words came from: inferred.)

---

## function — parse_phext

Found in scripts/build_wavefront_explorer.py at line 36.

Surface: public API.

Signature: `def parse_phext(data)`

What it does: Function parse_phext takes data. Its return type is unannotated; the code shows it returning scrolls.

(Where these words came from: inferred.)

---

## function — region_of

Found in scripts/build_wavefront_explorer.py at line 56.

Surface: public API.

Signature: `def region_of(values)`

What it does: Function region_of takes values. Its return type is unannotated; the code shows it returning "shoals"; "hearth"; "open_water".

(Where these words came from: inferred.)

---

## function — summarize

Found in scripts/build_wavefront_explorer.py at line 65.

Surface: public API.

Signature: `def summarize(evidence)`

What it does: Function summarize takes evidence. Its return type is unannotated; the code shows it returning text; text[:cut + 1] if cut > 80 else text[:EVIDENCE_C.

(Where these words came from: inferred.)

---

## function — entries_of

Found in scripts/build_wavefront_explorer.py at line 73.

Surface: public API.

Signature: `def entries_of(scroll_text)`

What it does: Yield (heading, body) per entry in a person scroll. Shared scrolls

(Where these words came from: direct.)

---

## function — handle_of

Found in scripts/build_wavefront_explorer.py at line 91.

Surface: public API.

Signature: `def handle_of(heading)`

What it does: Function handle_of takes heading. Its return type is unannotated; the code shows it returning m.group(1); heading[1:].split()[0]; HANDLE_ALIASES.get(heading).

(Where these words came from: inferred.)

---

## function — main

Found in scripts/build_wavefront_explorer.py at line 100.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — fmt

Found in scripts/build_wavefront_snapshot.py at line 42.

Surface: public API.

Signature: `def fmt(coord)`

What it does: Function fmt takes coord. Its return type is unannotated; the code shows it returning ".".join(str(c) for c in coord[0:3]) + "/" + \.

(Where these words came from: inferred.)

---

## function — parse_phext

Found in scripts/build_wavefront_snapshot.py at line 48.

Surface: public API.

Signature: `def parse_phext(data)`

What it does: Dead-reckoning parse: walk bytes, split on delimiters, map to coords.

(Where these words came from: direct.)

---

## function — handle_of

Found in scripts/build_wavefront_snapshot.py at line 69.

Surface: public API.

Signature: `def handle_of(name_part)`

What it does: Function handle_of takes name_part. Its return type is unannotated; the code shows it returning m.group(1); name_part[1:].split()[0]; HANDLE_ALIASES.get(name_part).

(Where these words came from: inferred.)

---

## function — main

Found in scripts/build_wavefront_snapshot.py at line 78.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — check

Found in scripts/mcp-e2e/mcp_e2e.py at line 45.

Surface: public API.

Signature: `def check(ok, label, detail="")`

What it does: Function check takes ok, label, detail="". Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — skip

Found in scripts/mcp-e2e/mcp_e2e.py at line 50.

Surface: public API.

Signature: `def skip(label, why)`

What it does: Function skip takes label, why. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — req

Found in scripts/mcp-e2e/mcp_e2e.py at line 55.

Surface: public API.

Signature: `def req(method, path, body=None, headers=None, budget_ms=None, label=None)`

What it does: Function req takes method, path, body=None, headers=None, budget_ms=None, label=None. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — b64url

Found in scripts/mcp-e2e/mcp_e2e.py at line 73.

Surface: public API.

Signature: `def b64url(data: bytes) -> str`

What it does: Function b64url takes data: bytes. It gives back str.

(Where these words came from: inferred.)

---

## function — pkce

Found in scripts/mcp-e2e/mcp_e2e.py at line 77.

Surface: public API.

Signature: `def pkce()`

What it does: Function pkce takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning verifier, b64url(hashlib.sha256(verifier.encode(.

(Where these words came from: inferred.)

---

## function — consent_fields

Found in scripts/mcp-e2e/mcp_e2e.py at line 82.

Surface: public API.

Signature: `def consent_fields(page: str)`

What it does: Function consent_fields takes page: str. Its return type is unannotated; the code shows it returning fields.

(Where these words came from: inferred.)

---

## function — authorize_and_consent

Found in scripts/mcp-e2e/mcp_e2e.py at line 92.

Surface: public API.

Signature: `def authorize_and_consent(params, label)`

What it does: GET authorize (expect the consent page), POST approve, return the

(Where these words came from: direct.)

---

## function — token_exchange

Found in scripts/mcp-e2e/mcp_e2e.py at line 111.

Surface: public API.

Signature: `def token_exchange(form, label, extra_headers=None)`

What it does: Function token_exchange takes form, label, extra_headers=None. Its return type is unannotated; the code shows it returning req("POST", "/oauth/token", urllib.parse.urlenco.

(Where these words came from: inferred.)

---

## function — rpc

Found in scripts/mcp-e2e/mcp_e2e.py at line 119.

Surface: public API.

Signature: `def rpc(token, method, params=None, req_id=1, headers_extra=None, path="/mcp", label=None, budget_ms=None)`

What it does: Function rpc takes token, method, params=None, req_id=1, headers_extra=None, path="/mcp", label=None, budget_ms=None. Its return type is unannotated; the code shows it returning status, hdrs, json.loads(text); status, hdrs, {"raw": text}.

(Where these words came from: inferred.)

---

## function — tool

Found in scripts/mcp-e2e/mcp_e2e.py at line 134.

Surface: public API.

Signature: `def tool(token, name, args, label=None, budget_ms=None)`

What it does: Function tool takes token, name, args, label=None, budget_ms=None. Its return type is unannotated; the code shows it returning json.loads(inner).

(Where these words came from: inferred.)

---

## function — flow_code_and_token

Found in scripts/mcp-e2e/mcp_e2e.py at line 141.

Surface: public API.

Signature: `def flow_code_and_token(*, label, client_id, redirect_uri, state, verifier, token_extra=None, token_headers=None, expect_token_status=200)`

What it does: Function flow_code_and_token takes *, label, client_id, redirect_uri, state, verifier, token_extra=None, token_headers=None, expect_token_status=200. Its return type is unannotated; the code shows it returning None; json.loads(body).

(Where these words came from: inferred.)

---

## function — pkce_pair

Found in scripts/mcp-e2e/mcp_e2e.py at line 170.

Surface: public API.

Signature: `def pkce_pair(verifier)`

What it does: Function pkce_pair takes verifier. Its return type is unannotated; the code shows it returning verifier, b64url(hashlib.sha256(verifier.encode(.

(Where these words came from: inferred.)

---

## function — assertion

Found in scripts/mcp-e2e/mcp_e2e.py at line 384.

Surface: public API.

Signature: `def assertion(jti)`

What it does: Function assertion takes jti. Its return type is unannotated; the code shows it returning pyjwt.encode(.

(Where these words came from: inferred.)

---

## function — cleanup

Found in scripts/mcp-e2e/run.sh at line 15.

Surface: public API.

Signature: `cleanup()`

What it does: Function cleanup takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — stamp

Found in scripts/mcp-e2e/sdk_flow.py at line 31.

Surface: public API.

Signature: `def stamp(msg)`

What it does: Function stamp takes msg. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — MemoryStorage.__init__

Found in scripts/mcp-e2e/sdk_flow.py at line 36.

Surface: public API.

Signature: `def __init__(self)`

What it does: Function MemoryStorage.__init__ takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — MemoryStorage.get_tokens

Found in scripts/mcp-e2e/sdk_flow.py at line 40.

Surface: public API.

Signature: `async def get_tokens(self)`

What it does: Function MemoryStorage.get_tokens takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning self._tokens.

(Where these words came from: inferred.)

---

## function — MemoryStorage.set_tokens

Found in scripts/mcp-e2e/sdk_flow.py at line 43.

Surface: public API.

Signature: `async def set_tokens(self, tokens)`

What it does: Function MemoryStorage.set_tokens takes self, tokens. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — MemoryStorage.get_client_info

Found in scripts/mcp-e2e/sdk_flow.py at line 46.

Surface: public API.

Signature: `async def get_client_info(self)`

What it does: Function MemoryStorage.get_client_info takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning self._client.

(Where these words came from: inferred.)

---

## function — MemoryStorage.set_client_info

Found in scripts/mcp-e2e/sdk_flow.py at line 49.

Surface: public API.

Signature: `async def set_client_info(self, client_info)`

What it does: Function MemoryStorage.set_client_info takes self, client_info. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — redirect_handler

Found in scripts/mcp-e2e/sdk_flow.py at line 53.

Surface: public API.

Signature: `async def redirect_handler(authorization_url: str) -> None`

What it does: Function redirect_handler takes authorization_url: str. It gives back nothing.

(Where these words came from: inferred.)

---

## function — callback_handler

Found in scripts/mcp-e2e/sdk_flow.py at line 75.

Surface: public API.

Signature: `async def callback_handler() -> AuthorizationCodeResult`

What it does: Function callback_handler takes no inputs beyond the value it is called on. It gives back AuthorizationCodeResult.

(Where these words came from: inferred.)

---

## function — main

Found in scripts/mcp-e2e/sdk_flow.py at line 79.

Surface: public API.

Signature: `async def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## entry point — main

Found in src/bin/phext_edit.rs at line 7879.

Surface: public API.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## entry point — main

Found in src/bin/phext_nav.rs at line 179.

Surface: public API.

Signature: `fn main() -> io::Result<()> {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## entry point — main

Found in src/bin/phext_tts.rs at line 17.

Surface: public API.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## command — coord

Found in src/bin/phext_tts.rs at line 26.

Surface: public API.

Signature: `command "coord"`

What it does: A command named coord. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — c

Found in src/bin/phext_tts.rs at line 26.

Surface: public API.

Signature: `command "c"`

What it does: A command named c. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — ssml

Found in src/bin/phext_tts.rs at line 33.

Surface: public API.

Signature: `command "ssml"`

What it does: A command named ssml. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — special

Found in src/bin/phext_tts.rs at line 41.

Surface: public API.

Signature: `command "special"`

What it does: A command named special. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — s

Found in src/bin/phext_tts.rs at line 41.

Surface: public API.

Signature: `command "s"`

What it does: A command named s. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — table

Found in src/bin/phext_tts.rs at line 48.

Surface: public API.

Signature: `command "table"`

What it does: A command named table. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — t

Found in src/bin/phext_tts.rs at line 48.

Surface: public API.

Signature: `command "t"`

What it does: A command named t. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — batch

Found in src/bin/phext_tts.rs at line 61.

Surface: public API.

Signature: `command "batch"`

What it does: A command named batch. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — b

Found in src/bin/phext_tts.rs at line 61.

Surface: public API.

Signature: `command "b"`

What it does: A command named b. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — extract

Found in src/bin/phext_tts.rs at line 68.

Surface: public API.

Signature: `command "extract"`

What it does: A command named extract. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — e

Found in src/bin/phext_tts.rs at line 68.

Surface: public API.

Signature: `command "e"`

What it does: A command named e. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## command — help

Found in src/bin/phext_tts.rs at line 75.

Surface: public API.

Signature: `command "help"`

What it does: A command named help. The program runs it when the user types this word on the command line.

(Where these words came from: inferred.)

---

## flag — --help

Found in src/bin/phext_tts.rs at line 75.

Surface: public API.

Signature: `flag --help`

What it does: A command line flag, --help (used by: help). It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## flag — -h

Found in src/bin/phext_tts.rs at line 75.

Surface: public API.

Signature: `flag -h`

What it does: A command line flag, -h (used by: help). It is read by the program when the user passes it.

(Where these words came from: inferred.)

---

## entry point — main

Found in src/bin/uml6d_edit.rs at line 653.

Surface: public API.

Signature: `fn main() {`

What it does: The entry point: the program starts here. It reads the command line and runs the command the user named.

(Where these words came from: inferred.)

---

## enum — Tier

Found in src/claims.rs at line 36.

Surface: public API.

Signature: `pub enum Tier {`

What it does: A named choice named Tier. It is one of: Found; Bought; Patron; New.

(Where these words came from: inferred.)

---

## function — Tier::grant

Found in src/claims.rs at line 44.

Surface: public API.

Signature: `pub fn grant(self) -> Grant {`

What it does: Function Tier::grant takes no inputs beyond the value it is called on. It gives back Grant.

(Where these words came from: inferred.)

---

## struct — Grant

Found in src/claims.rs at line 70.

Surface: public API.

Signature: `pub struct Grant {`

What it does: A data shape named Grant. Its fields are: pub storage_bytes: usize; pub max_phexts: usize; pub max_phext_bytes: usize; pub max_delegates: usize.

(Where these words came from: inferred.)

---

## struct — Claim

Found in src/claims.rs at line 78.

Surface: public API.

Signature: `pub struct Claim {`

What it does: A data shape named Claim. Its fields are: pub who: String; pub handle: String; pub tier: Tier; pub placements: Vec<String>; pub claimed_at: i64.

(Where these words came from: inferred.)

---

## function — get

Found in src/claims.rs at line 115.

Surface: public API.

Signature: `pub fn get(dir: &Path, who: &str) -> Option<Claim> {`

What it does: The claim an account already holds, if any.

(Where these words came from: direct.)

---

## function — record

Found in src/claims.rs at line 128.

Surface: public API.

Signature: `pub fn record( dir: &Path, who: &str, handle: &str, tier: Tier, placements: &[String], ) -> Result<Claim, String> {`

What it does: Record a claim, or return the one the account already holds — claiming twice changes nothing (a later buy-in updates the tier through `set_tier`, never by re-claiming).

(Where these words came from: direct.)

---

## function — list

Found in src/claims.rs at line 153.

Surface: public API.

Signature: `pub fn list(dir: &Path) -> Vec<Claim> {`

What it does: Every recorded claim, oldest first. Unreadable files are skipped.

(Where these words came from: direct.)

---

## function — set_tier

Found in src/claims.rs at line 173.

Surface: public API.

Signature: `pub fn set_tier(dir: &Path, who: &str, tier: Tier, placements: &[String]) -> Result<Claim, String> {`

What it does: Move an existing claim to a new tier (a buy-in landing, or a map republication finding the account). Returns the updated claim.

(Where these words came from: direct.)

---

## enum — Dimension

Found in src/coordinate_ext.rs at line 34.

Surface: public API.

Signature: `pub enum Dimension {`

What it does: Dimension enum for editor navigation. Maps 1-9 keys to the nine phext dimensions (scroll=innermost, library=outermost). NOTE: this index counts from the scroll outward — it is a key/wire index, NOT the canonical dimension number. Canonical numbering follows the written address (Library.Shelf.Series / Collection.Volume.Book / Chapter.Section.Scroll): Library is dimension 1, Scroll is dimension 9 (Will's ruling, 2026-11-10). Teaching surfaces state the canonical number; this enum's values remain the key/wire convention, and renumbering them is a behavioral change, not a display change.

(Where these words came from: direct.)

---

## function — Dimension::from_index

Found in src/coordinate_ext.rs at line 67.

Surface: public API.

Signature: `pub fn from_index(i: u8) -> Option<Dimension> {`

What it does: Convert 1-9 key index to dimension. Returns None for out of range.

(Where these words came from: direct.)

---

## function — Dimension::name

Found in src/coordinate_ext.rs at line 82.

Surface: public API.

Signature: `pub fn name(&self) -> &'static str {`

What it does: Function Dimension::name takes no inputs beyond the value it is called on. It gives back &'static str.

(Where these words came from: inferred.)

---

## function — Dimension::delimiter

Found in src/coordinate_ext.rs at line 97.

Surface: public API.

Signature: `pub fn delimiter(&self) -> u8 {`

What it does: The delimiter byte for this dimension.

(Where these words came from: direct.)

---

## trait — CoordinateNav

Found in src/coordinate_ext.rs at line 123.

Surface: public API.

Signature: `pub trait CoordinateNav {`

What it does: [UPSTREAM] Navigation trait: move forward/backward in a specific dimension. The existing break methods (scroll_break, section_break, etc.) advance AND reset lower dimensions. That's correct for parsing delimiters, but an editor also needs "move to the next book without resetting chapter/section/scroll" for lattice navigation, AND "move backward" which libphext doesn't support at all. `navigate` moves in a dimension, resetting lower dims (like the delimiter does). `peek` moves in a dimension WITHOUT resetting lower dims (for UI preview).

(Where these words came from: direct.)

---

## struct — Position

Found in src/cursor.rs at line 12.

Surface: public API.

Signature: `pub struct Position {`

What it does: Position within a scroll's text.

(Where these words came from: direct.)

---

## function — Position::new

Found in src/cursor.rs at line 20.

Surface: public API.

Signature: `pub fn new(line: usize, col: usize) -> Position {`

What it does: Function Position::new takes line: usize, col: usize. It gives back Position.

(Where these words came from: inferred.)

---

## function — Position::origin

Found in src/cursor.rs at line 24.

Surface: public API.

Signature: `pub fn origin() -> Position {`

What it does: Function Position::origin takes no inputs beyond the value it is called on. It gives back Position.

(Where these words came from: inferred.)

---

## function — Position::from_offset

Found in src/cursor.rs at line 29.

Surface: public API.

Signature: `pub fn from_offset(text: &str, offset: usize) -> Position {`

What it does: Convert a byte offset in text to a Position.

(Where these words came from: direct.)

---

## function — Position::to_offset

Found in src/cursor.rs at line 46.

Surface: public API.

Signature: `pub fn to_offset(&self, text: &str) -> usize {`

What it does: Convert this Position to a byte offset in text.

(Where these words came from: direct.)

---

## struct — Selection

Found in src/cursor.rs at line 76.

Surface: public API.

Signature: `pub struct Selection {`

What it does: A selection range within a scroll.

(Where these words came from: direct.)

---

## function — Selection::cursor

Found in src/cursor.rs at line 85.

Surface: public API.

Signature: `pub fn cursor(pos: Position) -> Selection {`

What it does: A zero-width selection (cursor) at a position.

(Where these words came from: direct.)

---

## function — Selection::is_cursor

Found in src/cursor.rs at line 90.

Surface: public API.

Signature: `pub fn is_cursor(&self) -> bool {`

What it does: Whether this is a zero-width selection (just a cursor).

(Where these words came from: direct.)

---

## function — Selection::start

Found in src/cursor.rs at line 95.

Surface: public API.

Signature: `pub fn start(&self) -> Position {`

What it does: The start of the selection (earlier position).

(Where these words came from: direct.)

---

## function — Selection::end

Found in src/cursor.rs at line 106.

Surface: public API.

Signature: `pub fn end(&self) -> Position {`

What it does: The end of the selection (later position).

(Where these words came from: direct.)

---

## struct — ScrollCursor

Found in src/cursor.rs at line 119.

Surface: public API.

Signature: `pub struct ScrollCursor {`

What it does: The cursor state within a scroll.

(Where these words came from: direct.)

---

## function — ScrollCursor::new

Found in src/cursor.rs at line 131.

Surface: public API.

Signature: `pub fn new() -> ScrollCursor {`

What it does: New cursor at the origin (0,0).

(Where these words came from: direct.)

---

## function — ScrollCursor::at

Found in src/cursor.rs at line 140.

Surface: public API.

Signature: `pub fn at(pos: Position) -> ScrollCursor {`

What it does: New cursor at a specific position.

(Where these words came from: direct.)

---

## function — ScrollCursor::position

Found in src/cursor.rs at line 149.

Surface: public API.

Signature: `pub fn position(&self) -> Position {`

What it does: Current cursor position (selection head).

(Where these words came from: direct.)

---

## function — ScrollCursor::move_to

Found in src/cursor.rs at line 154.

Surface: public API.

Signature: `pub fn move_to(&mut self, pos: Position) {`

What it does: Move cursor to a position, collapsing any selection.

(Where these words came from: direct.)

---

## function — ScrollCursor::select_to

Found in src/cursor.rs at line 160.

Surface: public API.

Signature: `pub fn select_to(&mut self, pos: Position) {`

What it does: Extend selection to a position (keep anchor, move head).

(Where these words came from: direct.)

---

## function — ScrollCursor::move_left

Found in src/cursor.rs at line 166.

Surface: public API.

Signature: `pub fn move_left(&mut self, text: &str, n: usize) {`

What it does: Move cursor left by n characters within text.

(Where these words came from: direct.)

---

## function — ScrollCursor::move_right

Found in src/cursor.rs at line 173.

Surface: public API.

Signature: `pub fn move_right(&mut self, text: &str, n: usize) {`

What it does: Move cursor right by n characters within text.

(Where these words came from: direct.)

---

## function — ScrollCursor::move_up

Found in src/cursor.rs at line 180.

Surface: public API.

Signature: `pub fn move_up(&mut self, text: &str, n: usize) {`

What it does: Move cursor up by n lines, preserving column.

(Where these words came from: direct.)

---

## function — ScrollCursor::move_down

Found in src/cursor.rs at line 194.

Surface: public API.

Signature: `pub fn move_down(&mut self, text: &str, n: usize) {`

What it does: Move cursor down by n lines, preserving column.

(Where these words came from: direct.)

---

## function — ScrollCursor::move_home

Found in src/cursor.rs at line 209.

Surface: public API.

Signature: `pub fn move_home(&mut self) {`

What it does: Move to beginning of current line.

(Where these words came from: direct.)

---

## function — ScrollCursor::move_end

Found in src/cursor.rs at line 214.

Surface: public API.

Signature: `pub fn move_end(&mut self, text: &str) {`

What it does: Move to end of current line.

(Where these words came from: direct.)

---

## function — ScrollCursor::move_top

Found in src/cursor.rs at line 220.

Surface: public API.

Signature: `pub fn move_top(&mut self) {`

What it does: Move to start of text.

(Where these words came from: direct.)

---

## function — ScrollCursor::move_bottom

Found in src/cursor.rs at line 225.

Surface: public API.

Signature: `pub fn move_bottom(&mut self, text: &str) {`

What it does: Move to end of text.

(Where these words came from: direct.)

---

## function — ScrollCursor::move_word_forward

Found in src/cursor.rs at line 232.

Surface: public API.

Signature: `pub fn move_word_forward(&mut self, text: &str) {`

What it does: Move to the next word boundary.

(Where these words came from: direct.)

---

## function — ScrollCursor::move_word_backward

Found in src/cursor.rs at line 247.

Surface: public API.

Signature: `pub fn move_word_backward(&mut self, text: &str) {`

What it does: Move to the previous word boundary.

(Where these words came from: direct.)

---

## enum — EditResult

Found in src/editor.rs at line 16.

Surface: public API.

Signature: `pub enum EditResult {`

What it does: What happened after processing a key.

(Where these words came from: direct.)

---

## enum — CursorStyle

Found in src/editor.rs at line 31.

Surface: public API.

Signature: `pub enum CursorStyle {`

What it does: Cursor rendering style.

(Where these words came from: direct.)

---

## struct — EditKey

Found in src/editor.rs at line 42.

Surface: public API.

Signature: `pub struct EditKey {`

What it does: A key event abstraction (decoupled from any TUI/GUI framework).

(Where these words came from: direct.)

---

## enum — EditKeyCode

Found in src/editor.rs at line 50.

Surface: public API.

Signature: `pub enum EditKeyCode {`

What it does: A named choice named EditKeyCode. It is one of: Char; Enter; Backspace; Delete; Escape; Left; Right; Up; Down; Home; End; PageUp; PageDown; Tab.

(Where these words came from: inferred.)

---

## function — EditKey::char

Found in src/editor.rs at line 68.

Surface: public API.

Signature: `pub fn char(c: char) -> EditKey {`

What it does: Function EditKey::char takes c: char. It gives back EditKey.

(Where these words came from: inferred.)

---

## function — EditKey::ctrl

Found in src/editor.rs at line 72.

Surface: public API.

Signature: `pub fn ctrl(c: char) -> EditKey {`

What it does: Function EditKey::ctrl takes c: char. It gives back EditKey.

(Where these words came from: inferred.)

---

## function — EditKey::special

Found in src/editor.rs at line 76.

Surface: public API.

Signature: `pub fn special(code: EditKeyCode) -> EditKey {`

What it does: Function EditKey::special takes code: EditKeyCode. It gives back EditKey.

(Where these words came from: inferred.)

---

## struct — EditContext

Found in src/editor.rs at line 83.

Surface: public API.

Signature: `pub struct EditContext<'a> {`

What it does: The editing context passed to modes. Provides access to text content, cursor, and edit operations.

(Where these words came from: direct.)

---

## function — <'a> EditContext<'a>::new

Found in src/editor.rs at line 95.

Surface: public API.

Signature: `pub fn new( coordinate: Coordinate, text: &'a mut String, cursor: &'a mut ScrollCursor, undo: &'a mut UndoEngine, ) -> EditContext<'a> {`

What it does: Function <'a> EditContext<'a>::new takes coordinate: Coordinate, text: &'a mut String, cursor: &'a mut ScrollCursor, undo: &'a mut UndoEngine,. It gives back EditContext<'a>.

(Where these words came from: inferred.)

---

## function — <'a> EditContext<'a>::insert

Found in src/editor.rs at line 105.

Surface: public API.

Signature: `pub fn insert(&mut self, s: &str) {`

What it does: Insert text at the cursor position.

(Where these words came from: direct.)

---

## function — <'a> EditContext<'a>::insert_newline

Found in src/editor.rs at line 116.

Surface: public API.

Signature: `pub fn insert_newline(&mut self) {`

What it does: Insert a newline at the cursor position.

(Where these words came from: direct.)

---

## function — <'a> EditContext<'a>::delete_backward

Found in src/editor.rs at line 121.

Surface: public API.

Signature: `pub fn delete_backward(&mut self, n: usize) {`

What it does: Delete n characters before the cursor (backspace).

(Where these words came from: direct.)

---

## function — <'a> EditContext<'a>::delete_forward

Found in src/editor.rs at line 135.

Surface: public API.

Signature: `pub fn delete_forward(&mut self, n: usize) {`

What it does: Delete n characters at the cursor position (delete key).

(Where these words came from: direct.)

---

## function — <'a> EditContext<'a>::delete_line

Found in src/editor.rs at line 148.

Surface: public API.

Signature: `pub fn delete_line(&mut self) {`

What it does: Delete the current line.

(Where these words came from: direct.)

---

## function — <'a> EditContext<'a>::replace_selection

Found in src/editor.rs at line 173.

Surface: public API.

Signature: `pub fn replace_selection(&mut self, new_text: &str) {`

What it does: Replace the selected text with new text.

(Where these words came from: direct.)

---

## function — <'a> EditContext<'a>::undo

Found in src/editor.rs at line 195.

Surface: public API.

Signature: `pub fn undo(&mut self) -> bool {`

What it does: Undo the last edit.

(Where these words came from: direct.)

---

## function — <'a> EditContext<'a>::redo

Found in src/editor.rs at line 206.

Surface: public API.

Signature: `pub fn redo(&mut self) -> bool {`

What it does: Redo the next edit.

(Where these words came from: direct.)

---

## trait — EditorMode

Found in src/editor.rs at line 221.

Surface: public API.

Signature: `pub trait EditorMode: std::fmt::Debug {`

What it does: The trait that all editor modes implement. A mode handles keystrokes and manipulates the editing context. It can return a different mode to transition to (e.g., Normal → Insert).

(Where these words came from: direct.)

---

## re-export — holo_phext::index::{LatticeIndex, ScrollSpan}

Found in src/index.rs at line 14.

Surface: public API.

Signature: `pub use holo_phext::index::{LatticeIndex, ScrollSpan};`

What it does: A re-export: holo_phext::index::{LatticeIndex, ScrollSpan} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## struct — Join

Found in src/joins.rs at line 23.

Surface: public API.

Signature: `pub struct Join {`

What it does: A data shape named Join. Its fields are: pub who: String; pub handle: String; pub via: String; pub joined_at: i64.

(Where these words came from: inferred.)

---

## function — get

Found in src/joins.rs at line 56.

Surface: public API.

Signature: `pub fn get(dir: &Path, who: &str) -> Option<Join> {`

What it does: The join an account already holds, if any.

(Where these words came from: direct.)

---

## function — record

Found in src/joins.rs at line 68.

Surface: public API.

Signature: `pub fn record(dir: &Path, who: &str, handle: &str, via: &str) -> Result<Join, String> {`

What it does: Record a join, or return the one the account already holds — the first arrival is the join, and joining twice changes nothing.

(Where these words came from: direct.)

---

## function — list

Found in src/joins.rs at line 86.

Surface: public API.

Signature: `pub fn list(dir: &Path) -> Vec<Join> {`

What it does: Every recorded join, oldest first. Unreadable files are skipped.

(Where these words came from: direct.)

---

## re-export — coordinate_ext::{Dimension, CoordinateNav}

Found in src/lib.rs at line 53.

Surface: public API.

Signature: `pub use coordinate_ext::{Dimension, CoordinateNav};`

What it does: A re-export: coordinate_ext::{Dimension, CoordinateNav} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — cursor::{ScrollCursor, Position, Selection}

Found in src/lib.rs at line 54.

Surface: public API.

Signature: `pub use cursor::{ScrollCursor, Position, Selection};`

What it does: A re-export: cursor::{ScrollCursor, Position, Selection} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — editor::{EditorMode, EditContext, EditKey, EditKeyCode, EditResult, CursorStyle}

Found in src/lib.rs at line 55.

Surface: public API.

Signature: `pub use editor::{EditorMode, EditContext, EditKey, EditKeyCode, EditResult, CursorStyle};`

What it does: A re-export: editor::{EditorMode, EditContext, EditKey, EditKeyCode, EditResult, CursorStyle} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — index::{LatticeIndex, ScrollSpan}

Found in src/lib.rs at line 56.

Surface: public API.

Signature: `pub use index::{LatticeIndex, ScrollSpan};`

What it does: A re-export: index::{LatticeIndex, ScrollSpan} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — mmap::MappedLattice

Found in src/lib.rs at line 57.

Surface: public API.

Signature: `pub use mmap::MappedLattice;`

What it does: A re-export: mmap::MappedLattice is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — modes::VimMode

Found in src/lib.rs at line 58.

Surface: public API.

Signature: `pub use modes::VimMode;`

What it does: A re-export: modes::VimMode is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — names::{NameIndex, fold}

Found in src/lib.rs at line 59.

Surface: public API.

Signature: `pub use names::{NameIndex, fold};`

What it does: A re-export: names::{NameIndex, fold} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — navigator::Navigator

Found in src/lib.rs at line 60.

Surface: public API.

Signature: `pub use navigator::Navigator;`

What it does: A re-export: navigator::Navigator is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — search::{search_lattice, search_lattice_parallel, search_lattice_auto, search_coordinates, SearchHit}

Found in src/lib.rs at line 61.

Surface: public API.

Signature: `pub use search::{search_lattice, search_lattice_parallel, search_lattice_auto, search_coordinates, SearchHit};`

What it does: A re-export: search::{search_lattice, search_lattice_parallel, search_lattice_auto, search_coordinates, SearchHit} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — sidecar::{IndexedLattice, IndexSpec, OpenMode, SyncReport, sidecar_dir}

Found in src/lib.rs at line 62.

Surface: public API.

Signature: `pub use sidecar::{IndexedLattice, IndexSpec, OpenMode, SyncReport, sidecar_dir};`

What it does: A re-export: sidecar::{IndexedLattice, IndexSpec, OpenMode, SyncReport, sidecar_dir} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — sentron::{Sentron, Neuron, Axon, AxisGroup, SENTRON_CAPACITY}

Found in src/lib.rs at line 63.

Surface: public API.

Signature: `pub use sentron::{Sentron, Neuron, Axon, AxisGroup, SENTRON_CAPACITY};`

What it does: A re-export: sentron::{Sentron, Neuron, Axon, AxisGroup, SENTRON_CAPACITY} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — stats::{ScrollStats, DimensionDensity, LatticeOverview, Neighborhood}

Found in src/lib.rs at line 64.

Surface: public API.

Signature: `pub use stats::{ScrollStats, DimensionDensity, LatticeOverview, Neighborhood};`

What it does: A re-export: stats::{ScrollStats, DimensionDensity, LatticeOverview, Neighborhood} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — term_index::{TermIndex, TermHit, Bm25Params, tokenize}

Found in src/lib.rs at line 65.

Surface: public API.

Signature: `pub use term_index::{TermIndex, TermHit, Bm25Params, tokenize};`

What it does: A re-export: term_index::{TermIndex, TermHit, Bm25Params, tokenize} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — undo::{UndoEngine, EditOp, UndoRecord}

Found in src/lib.rs at line 66.

Surface: public API.

Signature: `pub use undo::{UndoEngine, EditOp, UndoRecord};`

What it does: A re-export: undo::{UndoEngine, EditOp, UndoRecord} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — tts::{CoordPronunciation, byte_to_syllable, dim_to_syllables, to_ssml, generate_pronunciation_doc, special_coords}

Found in src/lib.rs at line 67.

Surface: public API.

Signature: `pub use tts::{CoordPronunciation, byte_to_syllable, dim_to_syllables, to_ssml, generate_pronunciation_doc, special_coords};`

What it does: A re-export: tts::{CoordPronunciation, byte_to_syllable, dim_to_syllables, to_ssml, generate_pronunciation_doc, special_coords} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## re-export — uml6d::{Phase, ArtifactAddr, UmlContext, trace_artifact, artifacts_in_phase}

Found in src/lib.rs at line 68.

Surface: public API.

Signature: `pub use uml6d::{Phase, ArtifactAddr, UmlContext, trace_artifact, artifacts_in_phase};`

What it does: A re-export: uml6d::{Phase, ArtifactAddr, UmlContext, trace_artifact, artifacts_in_phase} is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## function — media_payload_coord

Found in src/media.rs at line 32.

Surface: public API.

Signature: `pub fn media_payload_coord(slot: usize) -> Coordinate {`

What it does: Where slot `slot`'s image payload (a binary scroll) lives: 9.9.9/9.9.9/1.1.n.

(Where these words came from: direct.)

---

## function — media_metadata_coord

Found in src/media.rs at line 38.

Surface: public API.

Signature: `pub fn media_metadata_coord(slot: usize) -> Coordinate {`

What it does: Where slot `slot`'s metadata scroll (JSON text) lives: 9.9.9/9.9.9/1.2.n.

(Where these words came from: direct.)

---

## function — media_slot_of

Found in src/media.rs at line 44.

Surface: public API.

Signature: `pub fn media_slot_of(coord: &Coordinate) -> Option<usize> {`

What it does: The media slot a coordinate belongs to, if it is a media payload coordinate (9.9.9/9.9.9/1.1.n).

(Where these words came from: direct.)

---

## function — is_well_formed_binary_scroll

Found in src/media.rs at line 68.

Surface: public API.

Signature: `pub fn is_well_formed_binary_scroll(stored: &str) -> bool {`

What it does: True when `stored` is a well-formed binary scroll: it carries the canonical header and its body is strict base64 (an empty payload — header plus newline, nothing after — is well-formed). A header-prefixed scroll whose body is not strict base64 is malformed: by the format's fetch rule it is text, fetched verbatim, and the text write path may still edit it. Only well-formed binary scrolls are protected from text writes.

(Where these words came from: direct.)

---

## struct — ImageKind

Found in src/media.rs at line 74.

Surface: public API.

Signature: `pub struct ImageKind {`

What it does: A raster image format the editor accepts, identified by magic bytes.

(Where these words came from: direct.)

---

## const — PNG

Found in src/media.rs at line 81.

Surface: public API.

Signature: `pub const PNG: ImageKind = ImageKind { content_type: "image/png", extension: "png" };`

What it does: A fixed value named PNG, set to ImageKind { content_type: "image/png", extension: "png" }.

(Where these words came from: inferred.)

---

## const — JPEG

Found in src/media.rs at line 82.

Surface: public API.

Signature: `pub const JPEG: ImageKind = ImageKind { content_type: "image/jpeg", extension: "jpg" };`

What it does: A fixed value named JPEG, set to ImageKind { content_type: "image/jpeg", extension: "jpg" }.

(Where these words came from: inferred.)

---

## const — GIF

Found in src/media.rs at line 83.

Surface: public API.

Signature: `pub const GIF: ImageKind = ImageKind { content_type: "image/gif", extension: "gif" };`

What it does: A fixed value named GIF, set to ImageKind { content_type: "image/gif", extension: "gif" }.

(Where these words came from: inferred.)

---

## const — WEBP

Found in src/media.rs at line 84.

Surface: public API.

Signature: `pub const WEBP: ImageKind = ImageKind { content_type: "image/webp", extension: "webp" };`

What it does: A fixed value named WEBP, set to ImageKind { content_type: "image/webp", extension: "webp" }.

(Where these words came from: inferred.)

---

## const — AVIF

Found in src/media.rs at line 85.

Surface: public API.

Signature: `pub const AVIF: ImageKind = ImageKind { content_type: "image/avif", extension: "avif" };`

What it does: A fixed value named AVIF, set to ImageKind { content_type: "image/avif", extension: "avif" }.

(Where these words came from: inferred.)

---

## function — sniff_image

Found in src/media.rs at line 91.

Surface: public API.

Signature: `pub fn sniff_image(bytes: &[u8]) -> Option<ImageKind> {`

What it does: Identify an image payload by its magic bytes — never by a file name or a client-claimed type. SVG is deliberately absent: it is a scriptable document format, not a raster, and is never accepted or served by the image path, so there is no stored-XSS surface to make inert.

(Where these words came from: direct.)

---

## struct — MappedLattice

Found in src/mmap.rs at line 144.

Surface: public API.

Signature: `pub struct MappedLattice {`

What it does: A phext document with copy-on-write editing. The document may be a chain (a multiverse): several member phexts — universes — in one file. All read/write methods operate on the active* universe (see `universe` / `set_universe`); a plain phext is a chain of one universe and behaves exactly as it always has.

(Where these words came from: direct.)

---

## function — MappedLattice::open

Found in src/mmap.rs at line 186.

Surface: public API.

Signature: `pub fn open<P: AsRef<Path>>(path: P) -> io::Result<MappedLattice> {`

What it does: Open a phext file and build the lattice index.

(Where these words came from: direct.)

---

## function — MappedLattice::open_with_index

Found in src/mmap.rs at line 233.

Surface: public API.

Signature: `pub fn open_with_index<P: AsRef<Path>>(path: P, index: LatticeIndex) -> io::Result<MappedLattice> {`

What it does: Open a phext file with an index built elsewhere (e.g. loaded from a `sidecar`), skipping the O(n) scan. Fails if the index was built for a buffer of a different length. A chain-class file ignores the supplied index — a persisted span table describes one universe, not a chain — and is opened with a fresh scan instead.

(Where these words came from: direct.)

---

## function — MappedLattice::from_bytes

Found in src/mmap.rs at line 267.

Surface: public API.

Signature: `pub fn from_bytes(bytes: &[u8]) -> MappedLattice {`

What it does: Create a MappedLattice from raw bytes (for testing or in-memory use). The bytes are written under `std::env::temp_dir()` (never a hard-coded `/tmp`) and mapped from that same file handle. The temp directory is created if it is missing.

(Where these words came from: direct.)

---

## function — MappedLattice::read_scroll

Found in src/mmap.rs at line 316.

Surface: public API.

Signature: `pub fn read_scroll(&self, coord: &Coordinate) -> Option<String> {`

What it does: Read the content of a scroll at the given coordinate. Returns the overlay version if edited, otherwise reads from mmap.

(Where these words came from: direct.)

---

## function — MappedLattice::read_scroll_bytes

Found in src/mmap.rs at line 342.

Surface: public API.

Signature: `pub fn read_scroll_bytes(&self, coord: &Coordinate) -> Option<Vec<u8>> {`

What it does: The scroll's exact stored bytes: the overlay version if edited, otherwise the mmap span, uninterpreted. This is the raw layer the binary-scroll support is built on — a binary scroll's stored form is text (header + base64), but payloads are recovered from these bytes without any lossy conversion.

(Where these words came from: direct.)

---

## function — MappedLattice::fetch_binary

Found in src/mmap.rs at line 367.

Surface: public API.

Signature: `pub fn fetch_binary(&self, coord: &Coordinate) -> Vec<u8> {`

What it does: The payload a fetch of this scroll returns, mirroring `libphext::binary::fetch_binary` exactly: a well-formed binary scroll decodes to its payload; any other scroll (plain text, malformed binary, unoccupied) yields its own stored bytes verbatim — empty when the coordinate holds no scroll. Total: never fails, never panics.

(Where these words came from: direct.)

---

## function — MappedLattice::is_chain

Found in src/mmap.rs at line 381.

Surface: public API.

Signature: `pub fn is_chain(&self) -> bool {`

What it does: True when this file is a chain: more structure than a plain phext's single unclosed member.

(Where these words came from: direct.)

---

## function — MappedLattice::universe_count

Found in src/mmap.rs at line 387.

Surface: public API.

Signature: `pub fn universe_count(&self) -> usize {`

What it does: How many universes (member phexts) this file holds. A plain phext holds exactly one.

(Where these words came from: direct.)

---

## function — MappedLattice::universe

Found in src/mmap.rs at line 392.

Surface: public API.

Signature: `pub fn universe(&self) -> usize {`

What it does: The active universe, numbered from 1.

(Where these words came from: direct.)

---

## function — MappedLattice::member_stats

Found in src/mmap.rs at line 397.

Surface: public API.

Signature: `pub fn member_stats(&self) -> Vec<(usize, usize)> {`

What it does: Per-universe (scroll count, content bytes), in universe order.

(Where these words came from: direct.)

---

## function — MappedLattice::set_universe

Found in src/mmap.rs at line 408.

Surface: public API.

Signature: `pub fn set_universe(&mut self, universe: usize) -> io::Result<()> {`

What it does: Make `universe` (numbered from 1) the active universe. Refused while the overlay holds unsaved edits — an edit never straddles a switch — and when the universe is out of range.

(Where these words came from: direct.)

---

## function — MappedLattice::read_scroll_bytes_in

Found in src/mmap.rs at line 434.

Surface: public API.

Signature: `pub fn read_scroll_bytes_in(&self, universe: usize, coord: &Coordinate) -> Option<Vec<u8>> {`

What it does: A scroll's exact stored bytes in `universe` (numbered from 1), without switching the active universe. The overlay applies only when `universe` is the active one.

(Where these words came from: direct.)

---

## function — MappedLattice::read_scroll_in

Found in src/mmap.rs at line 445.

Surface: public API.

Signature: `pub fn read_scroll_in(&self, universe: usize, coord: &Coordinate) -> Option<String> {`

What it does: A scroll's content in `universe` (numbered from 1), without switching the active universe.

(Where these words came from: direct.)

---

## function — MappedLattice::write_scroll

Found in src/mmap.rs at line 461.

Surface: public API.

Signature: `pub fn write_scroll(&mut self, coord: Coordinate, content: String) -> bool {`

What it does: Write content to a scroll at the given coordinate. If the coordinate already exists (in mmap or overlay), it's modified. If it's new, it's inserted. Binary-scroll guard: a well-formed binary scroll (an image, say) cannot be replaced through the text path — its stored form is a header plus base64, and a text edit would corrupt the payload silently. Returns false when the write is refused for that reason; the scroll is left exactly as it was. Writing the identical stored form back is a no-op success. Byte payloads go through `write_binary` / `write_bytes` instead.

(Where these words came from: direct.)

---

## function — MappedLattice::write_scroll_raw

Found in src/mmap.rs at line 476.

Surface: public API.

Signature: `pub fn write_scroll_raw(&mut self, coord: Coordinate, content: String) {`

What it does: Store a scroll's stored form verbatim, with no binary guard. This is the byte path: callers pass either canonical binary scrolls (from `libphext::binary::encode_scroll`) or stored forms they have already validated. Prefer `write_scroll` for text and `write_binary` / `write_bytes` for payloads.

(Where these words came from: direct.)

---

## function — MappedLattice::write_binary

Found in src/mmap.rs at line 496.

Surface: public API.

Signature: `pub fn write_binary(&mut self, coord: Coordinate, payload: &[u8]) {`

What it does: Store `payload` at `coord` as a canonical binary scroll (header + base64 of the exact bytes), whatever the payload holds.

(Where these words came from: direct.)

---

## function — MappedLattice::write_bytes

Found in src/mmap.rs at line 505.

Surface: public API.

Signature: `pub fn write_bytes(&mut self, coord: Coordinate, payload: &[u8]) {`

What it does: Store `payload` at `coord` under the format's ingestion rule (libphext's `stored_form`, behind `insert_bytes`/`replace_bytes`): valid UTF-8 is stored as ordinary text; anything else becomes a binary scroll holding the exact bytes. Ingestion never rejects a payload.

(Where these words came from: direct.)

---

## function — MappedLattice::delete_scroll

Found in src/mmap.rs at line 513.

Surface: public API.

Signature: `pub fn delete_scroll(&mut self, coord: Coordinate) {`

What it does: Delete a scroll at the given coordinate.

(Where these words came from: direct.)

---

## function — MappedLattice::has_scroll

Found in src/mmap.rs at line 529.

Surface: public API.

Signature: `pub fn has_scroll(&self, coord: &Coordinate) -> bool {`

What it does: Check if a coordinate has content (considering overlay).

(Where these words came from: direct.)

---

## function — MappedLattice::populated_coordinates

Found in src/mmap.rs at line 543.

Surface: public API.

Signature: `pub fn populated_coordinates(&self) -> Vec<Coordinate> {`

What it does: Get all populated coordinates, including overlay insertions, excluding overlay deletions. Returned in sorted order.

(Where these words came from: direct.)

---

## function — MappedLattice::scroll_count

Found in src/mmap.rs at line 568.

Surface: public API.

Signature: `pub fn scroll_count(&self) -> usize {`

What it does: Number of populated scrolls (considering overlay).

(Where these words came from: direct.)

---

## function — MappedLattice::is_dirty

Found in src/mmap.rs at line 573.

Surface: public API.

Signature: `pub fn is_dirty(&self) -> bool {`

What it does: Check if there are unsaved modifications.

(Where these words came from: direct.)

---

## function — MappedLattice::pending_changes

Found in src/mmap.rs at line 583.

Surface: public API.

Signature: `pub fn pending_changes(&self) -> usize {`

What it does: Number of modified/inserted/deleted scrolls.

(Where these words came from: direct.)

---

## function — MappedLattice::pending_coordinates

Found in src/mmap.rs at line 591.

Surface: public API.

Signature: `pub fn pending_coordinates(&self) -> Vec<Coordinate> {`

What it does: Coordinates with unsaved edits (modified, inserted, or deleted), sorted.

(Where these words came from: direct.)

---

## function — MappedLattice::save

Found in src/mmap.rs at line 610.

Surface: public API.

Signature: `pub fn save(&mut self) -> io::Result<()> {`

What it does: Save to the backing file. Plain documents commit their staged transaction and checkpoint — the base file ends up holding exactly the merged document, as the original rewrite save did, and the staged batch was durable in the WAL on the way. Chain files stay read-only here, with the reason named.

(Where these words came from: direct.)

---

## function — MappedLattice::save_as

Found in src/mmap.rs at line 657.

Surface: public API.

Signature: `pub fn save_as<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {`

What it does: Save to a specific path.

(Where these words came from: direct.)

---

## function — MappedLattice::to_phext_bytes

Found in src/mmap.rs at line 707.

Surface: public API.

Signature: `pub fn to_phext_bytes(&self) -> Vec<u8> {`

What it does: Reconstruct the full phext as bytes from mmap + overlay.

(Where these words came from: direct.)

---

## function — MappedLattice::raw_buffer

Found in src/mmap.rs at line 791.

Surface: public API.

Signature: `pub fn raw_buffer(&self) -> Option<&[u8]> {`

What it does: Access the raw mmap buffer for search operations. Returns None if overlay has modifications (use to_phext_bytes instead).

(Where these words came from: direct.)

---

## function — MappedLattice::index

Found in src/mmap.rs at line 813.

Surface: public API.

Signature: `pub fn index(&self) -> &LatticeIndex {`

What it does: Access the underlying index for navigation queries.

(Where these words came from: direct.)

---

## function — MappedLattice::path

Found in src/mmap.rs at line 818.

Surface: public API.

Signature: `pub fn path(&self) -> &Path {`

What it does: Get the file path.

(Where these words came from: direct.)

---

## function — MappedLattice::revert

Found in src/mmap.rs at line 823.

Surface: public API.

Signature: `pub fn revert(&mut self) {`

What it does: Discard all unsaved changes, reverting to the mmap state.

(Where these words came from: direct.)

---

## re-export — vim::VimMode

Found in src/modes/mod.rs at line 8.

Surface: public API.

Signature: `pub use vim::VimMode;`

What it does: A re-export: vim::VimMode is made reachable through this module, so other code can use it from here.

(Where these words came from: inferred.)

---

## struct — VimMode

Found in src/modes/vim.rs at line 20.

Surface: public API.

Signature: `pub struct VimMode {`

What it does: Vim-style editor mode with Normal/Insert/Visual/Command sub-modes.

(Where these words came from: direct.)

---

## function — VimMode::new

Found in src/modes/vim.rs at line 33.

Surface: public API.

Signature: `pub fn new() -> VimMode {`

What it does: Function VimMode::new takes no inputs beyond the value it is called on. It gives back VimMode.

(Where these words came from: inferred.)

---

## function — fold

Found in src/names.rs at line 17.

Surface: public API.

Signature: `pub fn fold(name: &str) -> String {`

What it does: Case-fold a name for lookup: Unicode lowercase with runs of whitespace collapsed to one space and the ends trimmed.

(Where these words came from: direct.)

---

## struct — NameIndex

Found in src/names.rs at line 69.

Surface: public API.

Signature: `pub struct NameIndex {`

What it does: Exact and folded name → coordinate maps. When two names collide, the first one inserted wins, so building in document order prefers earlier scrolls.

(Where these words came from: direct.)

---

## function — NameIndex::new

Found in src/names.rs at line 77.

Surface: public API.

Signature: `pub fn new() -> NameIndex {`

What it does: Function NameIndex::new takes no inputs beyond the value it is called on. It gives back NameIndex.

(Where these words came from: inferred.)

---

## function — NameIndex::from_scrolls

Found in src/names.rs at line 84.

Surface: public API.

Signature: `pub fn from_scrolls<F>(lattice: &MappedLattice, select: F) -> NameIndex where F: Fn(&Coordinate) -> Option<Coordinate>, {`

What it does: Build from the lattice: every non-empty line of each scroll that `select` maps to `Some(target)` becomes a name for `target`. Return the scroll's own coordinate to name it, or another one (e.g. a record's body) to point there.

(Where these words came from: direct.)

---

## function — NameIndex::insert

Found in src/names.rs at line 116.

Surface: public API.

Signature: `pub fn insert(&mut self, name: &str, coord: Coordinate) -> bool {`

What it does: Register `name` for `coord`. Blank names are ignored. Returns false if the exact name was already taken (the earlier coordinate is kept).

(Where these words came from: direct.)

---

## function — NameIndex::get

Found in src/names.rs at line 134.

Surface: public API.

Signature: `pub fn get(&self, name: &str) -> Option<Coordinate> {`

What it does: Exact (case-sensitive) lookup.

(Where these words came from: direct.)

---

## function — NameIndex::get_folded

Found in src/names.rs at line 139.

Surface: public API.

Signature: `pub fn get_folded(&self, name: &str) -> Option<Coordinate> {`

What it does: Case-insensitive lookup through the folded map.

(Where these words came from: direct.)

---

## function — NameIndex::resolve

Found in src/names.rs at line 144.

Surface: public API.

Signature: `pub fn resolve(&self, name: &str) -> Option<Coordinate> {`

What it does: Exact match first, then folded: what an agent's "open <name>" should call.

(Where these words came from: direct.)

---

## function — NameIndex::len

Found in src/names.rs at line 149.

Surface: public API.

Signature: `pub fn len(&self) -> usize {`

What it does: Distinct exact names.

(Where these words came from: direct.)

---

## function — NameIndex::is_empty

Found in src/names.rs at line 153.

Surface: public API.

Signature: `pub fn is_empty(&self) -> bool {`

What it does: Function NameIndex::is_empty takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — NameIndex::ambiguous

Found in src/names.rs at line 159.

Surface: public API.

Signature: `pub fn ambiguous(&self) -> usize {`

What it does: How many inserts hit a folded key already owned by a different coordinate. Those names resolve exactly but fold to the first owner.

(Where these words came from: direct.)

---

## struct — Navigator

Found in src/navigator.rs at line 13.

Surface: public API.

Signature: `pub struct Navigator {`

What it does: The navigator state for one cursor in the lattice.

(Where these words came from: direct.)

---

## function — Navigator::new

Found in src/navigator.rs at line 27.

Surface: public API.

Signature: `pub fn new() -> Navigator {`

What it does: Function Navigator::new takes no inputs beyond the value it is called on. It gives back Navigator.

(Where these words came from: inferred.)

---

## function — Navigator::at

Found in src/navigator.rs at line 38.

Surface: public API.

Signature: `pub fn at(coord: Coordinate) -> Navigator {`

What it does: Create a navigator starting at a specific coordinate.

(Where these words came from: direct.)

---

## function — Navigator::position

Found in src/navigator.rs at line 49.

Surface: public API.

Signature: `pub fn position(&self) -> Coordinate {`

What it does: Current position in the lattice.

(Where these words came from: direct.)

---

## function — Navigator::active_dimension

Found in src/navigator.rs at line 54.

Surface: public API.

Signature: `pub fn active_dimension(&self) -> Dimension {`

What it does: Currently selected dimension.

(Where these words came from: direct.)

---

## function — Navigator::select_dimension

Found in src/navigator.rs at line 59.

Surface: public API.

Signature: `pub fn select_dimension(&mut self, index: u8) -> bool {`

What it does: Select a dimension by index (1-9).

(Where these words came from: direct.)

---

## function — Navigator::move_forward

Found in src/navigator.rs at line 69.

Surface: public API.

Signature: `pub fn move_forward(&mut self) -> bool {`

What it does: Move forward in the active dimension (resets lower dims).

(Where these words came from: direct.)

---

## function — Navigator::move_backward

Found in src/navigator.rs at line 78.

Surface: public API.

Signature: `pub fn move_backward(&mut self) -> bool {`

What it does: Move backward in the active dimension (resets lower dims).

(Where these words came from: direct.)

---

## function — Navigator::goto

Found in src/navigator.rs at line 87.

Surface: public API.

Signature: `pub fn goto(&mut self, coord: Coordinate) {`

What it does: Jump directly to a coordinate.

(Where these words came from: direct.)

---

## function — Navigator::next_populated

Found in src/navigator.rs at line 93.

Surface: public API.

Signature: `pub fn next_populated(&mut self, index: &LatticeIndex) -> bool {`

What it does: Jump to the next populated coordinate (any dimension).

(Where these words came from: direct.)

---

## function — Navigator::prev_populated

Found in src/navigator.rs at line 104.

Surface: public API.

Signature: `pub fn prev_populated(&mut self, index: &LatticeIndex) -> bool {`

What it does: Jump to the previous populated coordinate (any dimension).

(Where these words came from: direct.)

---

## function — Navigator::set_mark

Found in src/navigator.rs at line 115.

Surface: public API.

Signature: `pub fn set_mark(&mut self) {`

What it does: Set a mark at the current position.

(Where these words came from: direct.)

---

## function — Navigator::jump_to_mark

Found in src/navigator.rs at line 120.

Surface: public API.

Signature: `pub fn jump_to_mark(&mut self) -> bool {`

What it does: Jump to the most recent mark (and remove it).

(Where these words came from: direct.)

---

## function — Navigator::history_back

Found in src/navigator.rs at line 131.

Surface: public API.

Signature: `pub fn history_back(&mut self) -> bool {`

What it does: Navigate back in history.

(Where these words came from: direct.)

---

## function — Navigator::history_forward

Found in src/navigator.rs at line 142.

Surface: public API.

Signature: `pub fn history_forward(&mut self) -> bool {`

What it does: Navigate forward in history.

(Where these words came from: direct.)

---

## function — Navigator::dimension_neighbors

Found in src/navigator.rs at line 154.

Surface: public API.

Signature: `pub fn dimension_neighbors(&self) -> (Option<usize>, usize, Option<usize>) {`

What it does: Get neighbors in the active dimension: (prev_value, current_value, next_value). Values are None if at the boundary.

(Where these words came from: direct.)

---

## struct — NeighborhoodSummary

Found in src/navigator.rs at line 179.

Surface: public API.

Signature: `pub struct NeighborhoodSummary {`

What it does: Summary of the lattice neighborhood around a coordinate, for rendering the coordinate bar and lattice navigator panel.

(Where these words came from: direct.)

---

## function — Navigator::summarize

Found in src/navigator.rs at line 198.

Surface: public API.

Signature: `pub fn summarize(&self, index: &LatticeIndex) -> NeighborhoodSummary {`

What it does: Build a summary of the current neighborhood for UI rendering.

(Where these words came from: direct.)

---

## const — TEST_CSRF

Found in src/oauth.rs at line 53.

Surface: public API.

Signature: `pub const TEST_CSRF: &str = "csrf-test-token";`

What it does: A fixed value named TEST_CSRF, set to "csrf-test-token".

(Where these words came from: inferred.)

---

## struct — Identity

Found in src/oauth.rs at line 60.

Surface: public API.

Signature: `pub struct Identity {`

What it does: A data shape named Identity. Its fields are: pub who: String; pub handle: String; pub name: String; pub email: String.

(Where these words came from: inferred.)

---

## trait — IdentitySource

Found in src/oauth.rs at line 67.

Surface: public API.

Signature: `pub trait IdentitySource: Send + Sync {`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## struct — Oauth

Found in src/oauth.rs at line 171.

Surface: public API.

Signature: `pub struct Oauth {`

Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.

---

## struct — Session

Found in src/oauth.rs at line 218.

Surface: public API.

Signature: `pub struct Session {`

What it does: A data shape named Session. Its fields are: pub who: String; pub handle: String; pub name: String; pub email: String; pub csrf: String.

(Where these words came from: inferred.)

---

## struct — SsoAssertionBody

Found in src/oauth.rs at line 269.

Surface: public API.

Signature: `pub struct SsoAssertionBody {`

What it does: The SSO assertion: a sealed, single-use statement that an edit session exists, carried to an audience's callback in a redirect and verified there under the SSO secret. It is not a session — the audience mints its own from it, and this server never accepts one back. `aud` names the one audience it was minted for (`mutable`, `exocortexia`, `wishnode`); a consumer must reject any other value, an assertion minted for a different audience. `ret` is a path on the audience's site, clamped by `safe_next` before sealing; `jti` is the single-use id the audience burns on first sight. The one exception to "never accepts one back" is the `desktop` audience (Phext Notepad): a shipped app cannot hold the SSO secret, so its assertions come back here to `POST /api/auth/desktop/exchange`, which verifies them under the secret and burns the `jti` server-side. For that audience `ret` is the app's validated loopback return URL (see `loopback_return`), and `st` carries the app's per-attempt state nonce, sealed at mint and demanded back at the exchange. Web audiences never carry `st` — the field is simply absent from their assertions.

(Where these words came from: direct.)

---

## function — Oauth::from_env

Found in src/oauth.rs at line 292.

Surface: public API.

Signature: `pub fn from_env() -> Self {`

What it does: Function Oauth::from_env takes no inputs beyond the value it is called on. It gives back Self.

(Where these words came from: inferred.)

---

## function — Oauth::test_ready

Found in src/oauth.rs at line 312.

Surface: public API.

Signature: `pub fn test_ready() -> Self {`

What it does: Function Oauth::test_ready takes no inputs beyond the value it is called on. It gives back Self.

(Where these words came from: inferred.)

---

## function — Oauth::closed

Found in src/oauth.rs at line 332.

Surface: public API.

Signature: `pub fn closed() -> Self {`

What it does: Function Oauth::closed takes no inputs beyond the value it is called on. It gives back Self.

(Where these words came from: inferred.)

---

## function — Oauth::ready

Found in src/oauth.rs at line 400.

Surface: public API.

Signature: `pub fn ready(&self) -> bool {`

What it does: Function Oauth::ready takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — Oauth::sso_ready

Found in src/oauth.rs at line 410.

Surface: public API.

Signature: `pub fn sso_ready(&self) -> bool {`

What it does: True when the Mutable SSO handoff is configured: a real SSO secret and a valid Mutable target. Independent of `ready()` — the handoff mints nothing of its own; it rides the session the providers already made, so it needs no provider keys here.

(Where these words came from: direct.)

---

## function — Oauth::sso_ready_for

Found in src/oauth.rs at line 419.

Surface: public API.

Signature: `pub fn sso_ready_for(&self, aud: &str) -> bool {`

What it does: True when the SSO handoff is configured for one audience: the shared SSO secret is real and that audience has a target. Every allowlisted audience rides the same secret and the same sealed assertion format; only the `aud` claim and the callback origin differ.

(Where these words came from: direct.)

---

## function — Oauth::sso_desktop_ready

Found in src/oauth.rs at line 427.

Surface: public API.

Signature: `pub fn sso_desktop_ready(&self) -> bool {`

What it does: True when the desktop audience is armed: a real SSO secret and the operator's explicit `PHEXT_SSO_DESKTOP=1`. There is no target to consult — the app's callback is a per-attempt loopback listener — so readiness is the secret plus the flag.

(Where these words came from: direct.)

---

## function — Oauth::base

Found in src/oauth.rs at line 431.

Surface: public API.

Signature: `pub fn base(&self) -> &str {`

What it does: Function Oauth::base takes no inputs beyond the value it is called on. It gives back &str.

(Where these words came from: inferred.)

---

## function — Oauth::sso_target

Found in src/oauth.rs at line 435.

Surface: public API.

Signature: `pub fn sso_target(&self) -> &str {`

What it does: Function Oauth::sso_target takes no inputs beyond the value it is called on. It gives back &str.

(Where these words came from: inferred.)

---

## function — Oauth::sso_target_for

Found in src/oauth.rs at line 450.

Surface: public API.

Signature: `pub fn sso_target_for(&self, aud: &str) -> Option<&str> {`

What it does: The callback origin for an audience, or None when the audience is not allowlisted or is allowlisted but unconfigured. Mutable always has a target (its production origin is the default); Exocortexia has one only once `PHEXT_SSO_EXOCORTEXIA_URL` is set, and Wishnode only once `PHEXT_SSO_WISHNODE_URL` is. This match is the audience allowlist — an audience that is not named here can never be minted for. The desktop audience is deliberately absent: it has no fixed origin (its callback is the app's per-attempt loopback listener), so it is armed and minted on its own path (`sso_desktop_ready`, `sso_assertion_desktop`).

(Where these words came from: direct.)

---

## function — Oauth::providers

Found in src/oauth.rs at line 463.

Surface: public API.

Signature: `pub fn providers(&self) -> Vec<&'static str> {`

What it does: Function Oauth::providers takes no inputs beyond the value it is called on. It gives back Vec<&'static str>.

(Where these words came from: inferred.)

---

## function — Oauth::cookie_secure

Found in src/oauth.rs at line 480.

Surface: public API.

Signature: `pub fn cookie_secure(&self) -> bool {`

What it does: Function Oauth::cookie_secure takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — Oauth::authorize_url

Found in src/oauth.rs at line 485.

Surface: public API.

Signature: `pub fn authorize_url(&self, provider_name: &str, next: &str) -> Result<(String, String), String> {`

What it does: Returns the provider URL to redirect to and the sealed pending cookie.

(Where these words came from: direct.)

---

## function — Oauth::finish

Found in src/oauth.rs at line 535.

Surface: public API.

Signature: `pub fn finish( &self, provider_name: &str, code: &str, state: &str, pending_cookie: &str, ) -> Result<(Identity, String, String), String> {`

What it does: Completes the flow. Returns the identity, the sealed session cookie, and the absolute URL to send the browser back to.

(Where these words came from: direct.)

---

## function — Oauth::issue_session

Found in src/oauth.rs at line 576.

Surface: public API.

Signature: `pub fn issue_session( &self, who: &str, handle: &str, name: &str, email: &str, ) -> Result<(String, String), String> {`

What it does: Seal a fresh session for an identity that has already been established — by the provider exchange in `finish`, or by a verified desktop SSO assertion at the exchange endpoint. This is the one place session values are made: a fresh CSRF token and a `SESSION_TTL` seal under the session secret. Returns the sealed cookie value and the CSRF token inside it (the desktop exchange hands both to the app; the browser flow learns the CSRF from `/api/session` instead).

(Where these words came from: direct.)

---

## function — Oauth::read_session

Found in src/oauth.rs at line 595.

Surface: public API.

Signature: `pub fn read_session(&self, cookie_header: &str) -> Option<Session> {`

What it does: Function Oauth::read_session takes &self, cookie_header: &str. It gives back Option<Session>.

(Where these words came from: inferred.)

---

## function — Oauth::csrf_matches

Found in src/oauth.rs at line 621.

Surface: public API.

Signature: `pub fn csrf_matches(&self, session: &Session, header: &str) -> bool {`

What it does: Function Oauth::csrf_matches takes &self, session: &Session, header: &str. It gives back bool.

(Where these words came from: inferred.)

---

## function — Oauth::mcp_oauth_hash

Found in src/oauth.rs at line 634.

Surface: public API.

Signature: `pub fn mcp_oauth_hash(&self, domain: &str, value: &str) -> Option<String> {`

What it does: A domain-separated, keyed digest for the MCP OAuth 2.1 store. Authorization codes, access tokens, refresh tokens, and client secrets are never stored in the clear. Each value is represented by HMAC-SHA256 under the existing session secret, with the caller's domain label mixed in before the value, so a digest minted for one kind of credential cannot be replayed as another. No separate OAuth secret is required: rotating the session secret intentionally invalidates the stored OAuth credentials along with sessions.

(Where these words came from: direct.)

---

## function — Oauth::test_session_cookie

Found in src/oauth.rs at line 651.

Surface: public API.

Signature: `pub fn test_session_cookie() -> String {`

What it does: Function Oauth::test_session_cookie takes no inputs beyond the value it is called on. It gives back String.

(Where these words came from: inferred.)

---

## function — Oauth::sso_assertion

Found in src/oauth.rs at line 701.

Surface: public API.

Signature: `pub fn sso_assertion( &self, aud: &str, session: &Session, editor: bool, ret: &str, ) -> Result<String, String> {`

What it does: Mint the assertion an audience's callback verifies: who this session is, whether it is an editor, and the return path on the audience's site — sealed under the shared SSO secret, alive for `SSO_TTL` seconds, with a fresh `jti` the audience burns on first sight. The format is identical for every audience; only the `aud` claim differs, and an audience outside the allowlist in `sso_target_for` can never be minted for. The return path is clamped by `safe_next` here, at the moment of sealing, so no unsigned return input ever crosses.

(Where these words came from: direct.)

---

## function — Oauth::sso_assertion_desktop

Found in src/oauth.rs at line 740.

Surface: public API.

Signature: `pub fn sso_assertion_desktop( &self, session: &Session, editor: bool, ret: &str, state: &str, ) -> Result<String, String> {`

What it does: Mint the desktop audience's assertion. Same sealed format as every audience, with the two differences that audience exists for: `ret` is the app's loopback return URL, validated by `loopback_return` here at the moment of sealing (anything else is refused, so no unvalidated return ever crosses), and the app's per-attempt `state` nonce rides sealed as `st`, to be demanded back at the exchange — an assertion that arrives anywhere without its attempt's state is not that attempt's assertion. The caller (`Gate::sso_desktop`) has already validated the state's shape.

(Where these words came from: direct.)

---

## function — Oauth::open_sso_assertion

Found in src/oauth.rs at line 778.

Surface: public API.

Signature: `pub fn open_sso_assertion(&self, token: &str) -> Result<SsoAssertionBody, String> {`

What it does: Open an SSO assertion under the SSO secret — the exchange endpoint is the one place this server ever takes an assertion back, and only the desktop audience's. Expiry is checked here; audience, state, and single-use are the caller's to enforce (the burn lives on the `Gate`, which owns request state). Every failure is the same error: the exchange must not say which check a forged assertion failed.

(Where these words came from: direct.)

---

## struct — Editors

Found in src/oauth.rs at line 843.

Surface: public API.

Signature: `pub struct Editors {`

What it does: Accounts allowed to use the editor once signed in. The list starts from `PHEXT_EDITORS` (the operator's bootstrap list) and can grow at runtime: when an editor accepts a submission, the originator's account is admitted — appended here and, when a backing file is configured (see `Gate::load_editors_file`), to that file, so the admission takes effect immediately and survives restarts. Entries sit behind a lock because admission happens through `&Gate` while other requests are reading the list.

(Where these words came from: direct.)

---

## function — Editors::from_env

Found in src/oauth.rs at line 876.

Surface: public API.

Signature: `pub fn from_env() -> Self {`

What it does: Function Editors::from_env takes no inputs beyond the value it is called on. It gives back Self.

(Where these words came from: inferred.)

---

## function — Editors::parse

Found in src/oauth.rs at line 880.

Surface: public API.

Signature: `pub fn parse(raw: &str) -> Self {`

What it does: Function Editors::parse takes raw: &str. It gives back Self.

(Where these words came from: inferred.)

---

## function — Editors::len

Found in src/oauth.rs at line 887.

Surface: public API.

Signature: `pub fn len(&self) -> usize {`

What it does: Function Editors::len takes no inputs beyond the value it is called on. It gives back usize.

(Where these words came from: inferred.)

---

## function — Editors::is_empty

Found in src/oauth.rs at line 891.

Surface: public API.

Signature: `pub fn is_empty(&self) -> bool {`

What it does: Function Editors::is_empty takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — Editors::matches

Found in src/oauth.rs at line 895.

Surface: public API.

Signature: `pub fn matches(&self, who: &str, email: &str) -> bool {`

What it does: Function Editors::matches takes &self, who: &str, email: &str. It gives back bool.

(Where these words came from: inferred.)

---

## function — Editors::allows

Found in src/oauth.rs at line 904.

Surface: public API.

Signature: `pub fn allows(&self, who: &str, email: &str) -> bool {`

What it does: An empty list admits every signed-in account.

(Where these words came from: direct.)

---

## function — Editors::load_file

Found in src/oauth.rs at line 912.

Surface: public API.

Signature: `pub fn load_file(&self, path: &std::path::Path) -> usize {`

What it does: Point the list at its backing file and load any entries already in it (one per line; blank lines and `#` comments are ignored, and a line may still carry several comma-separated entries). Entries the list already has are not duplicated. Returns how many were loaded.

(Where these words came from: direct.)

---

## function — Editors::admit

Found in src/oauth.rs at line 940.

Surface: public API.

Signature: `pub fn admit(&self, who: &str, email: &str) -> bool {`

What it does: Admit an account: add its account id (when valid) and its email (when usable), unless each is already covered, and append the new entries to the backing file when one is configured. Takes effect immediately. Returns true when the account is an editor afterwards.

(Where these words came from: direct.)

---

## struct — Limiter

Found in src/oauth.rs at line 977.

Surface: public API.

Signature: `pub struct Limiter {`

What it does: In-memory cap on sign-in attempts per client address, per ten minutes. Counts OAuth start/callback attempts. Anonymous 401s are not counted, so opening the signed-out page does not spend the budget. The address is `X-Real-IP` when nginx sets it from `$remote_addr`; otherwise the last `X-Forwarded-For` hop, which is the one a single reverse proxy appends.

(Where these words came from: direct.)

---

## function — Limiter::standard

Found in src/oauth.rs at line 983.

Surface: public API.

Signature: `pub fn standard() -> Self {`

What it does: Function Limiter::standard takes no inputs beyond the value it is called on. It gives back Self.

(Where these words came from: inferred.)

---

## function — Limiter::with_cap

Found in src/oauth.rs at line 987.

Surface: public API.

Signature: `pub fn with_cap(cap: usize) -> Self {`

What it does: Function Limiter::with_cap takes cap: usize. It gives back Self.

(Where these words came from: inferred.)

---

## function — Limiter::hit

Found in src/oauth.rs at line 991.

Surface: public API.

Signature: `pub fn hit(&self, key: &str) -> Result<(), ()> {`

What it does: Function Limiter::hit takes &self, key: &str. It gives back Result<(), ()>.

(Where these words came from: inferred.)

---

## struct — Reply

Found in src/oauth.rs at line 1006.

Surface: public API.

Signature: `pub struct Reply {`

What it does: A complete HTTP response for the hand-rolled servers to write out.

(Where these words came from: direct.)

---

## function — Reply::json

Found in src/oauth.rs at line 1015.

Surface: public API.

Signature: `pub fn json(status: u16, value: serde_json::Value) -> Self {`

What it does: Function Reply::json takes status: u16, value: serde_json::Value. It gives back Self.

(Where these words came from: inferred.)

---

## function — Reply::write_to

Found in src/oauth.rs at line 1058.

Surface: public API.

Signature: `pub fn write_to(&self, out: &mut impl std::io::Write) {`

What it does: Serialize as an HTTP/1.1 response with `Connection: close`. Auth and session bodies stay `Cache-Control: no-store`. There is no `Access-Control-Allow-Origin`: the editor is same-origin only.

(Where these words came from: direct.)

---

## const — HSTS_VALUE

Found in src/oauth.rs at line 1080.

Surface: public API.

Signature: `pub const HSTS_VALUE: &str = "max-age=31536000";`

What it does: One year. Matches `deploy/nginx-edit.phext.io.conf`. No `includeSubDomains`: this host is `edit.phext.io`, and a future sibling name under that host should not be pinned by accident.

(Where these words came from: direct.)

---

## const — CONTENT_SECURITY_POLICY

Found in src/oauth.rs at line 1092.

Surface: public API.

Signature: `pub const CONTENT_SECURITY_POLICY: &str = concat!( "default-src 'self'; ",`

What it does: Policy for the embedded editor and the public pages. The shell is one HTML document: an inline script, an inline stylesheet, `onclick` attributes, and `style` attributes. A nonce or a hash does not cover event-handler or style attributes, so those would mean rewriting the shell. `unsafe-inline` stays for that reason. There is no `unsafe-eval`. Sign-in is a same-origin link to `/api/auth/...`, which 302s to Google or GitHub; top-level navigations are not restricted. `upgrade-insecure-requests` is added only when the browser connection is HTTPS, so a local `http://127.0.0.1` session keeps working.

(Where these words came from: direct.)

---

## function — forwarded_https

Found in src/oauth.rs at line 1106.

Surface: public API.

Signature: `pub fn forwarded_https(headers: &HashMap<String, String>) -> bool {`

What it does: Function forwarded_https takes headers: &HashMap<String, String>. It gives back bool.

(Where these words came from: inferred.)

---

## function — security_header_block_csp

Found in src/oauth.rs at line 1114.

Surface: public API.

Signature: `pub fn security_header_block_csp(https: bool, csp_override: Option<&str>) -> String {`

What it does: Function security_header_block_csp takes https: bool, csp_override: Option<&str>. It gives back String.

(Where these words came from: inferred.)

---

## function — security_header_block

Found in src/oauth.rs at line 1132.

Surface: public API.

Signature: `pub fn security_header_block(https: bool) -> String {`

What it does: Function security_header_block takes https: bool. It gives back String.

(Where these words came from: inferred.)

---

## function — status_text

Found in src/oauth.rs at line 1136.

Surface: public API.

Signature: `pub fn status_text(status: u16) -> &'static str {`

What it does: Function status_text takes status: u16. It gives back &'static str.

(Where these words came from: inferred.)

---

## struct — Gate

Found in src/oauth.rs at line 1191.

Surface: public API.

Signature: `pub struct Gate {`

What it does: Access control for the editor servers: an OAuth session, or open when sign-in is not configured.

(Where these words came from: direct.)

---

## function — Gate::new

Found in src/oauth.rs at line 1209.

Surface: public API.

Signature: `pub fn new(oauth: Oauth, editors: Editors) -> Self {`

What it does: Function Gate::new takes oauth: Oauth, editors: Editors. It gives back Self.

(Where these words came from: inferred.)

---

## function — Gate::from_env

Found in src/oauth.rs at line 1235.

Surface: public API.

Signature: `pub fn from_env() -> Self {`

What it does: Function Gate::from_env takes no inputs beyond the value it is called on. It gives back Self.

(Where these words came from: inferred.)

---

## function — Gate::enable_local

Found in src/oauth.rs at line 1256.

Surface: public API.

Signature: `pub fn enable_local(&self) {`

What it does: Admit every direct request as a local developer. The phext-edit CLI calls this only after it has refused to bind anything but loopback. Nothing in the environment or in a request header calls it.

(Where these words came from: direct.)

---

## function — Gate::is_local

Found in src/oauth.rs at line 1260.

Surface: public API.

Signature: `pub fn is_local(&self) -> bool {`

What it does: Function Gate::is_local takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — Gate::enable_claims

Found in src/oauth.rs at line 1267.

Surface: public API.

Signature: `pub fn enable_claims(&self) {`

What it does: Open claim mode: `/api/claim` accepts claims (the lookup stays public either way). The operator turns this on once the X app credentials are in place; tests call it directly.

(Where these words came from: direct.)

---

## function — Gate::claims_open

Found in src/oauth.rs at line 1271.

Surface: public API.

Signature: `pub fn claims_open(&self) -> bool {`

What it does: Function Gate::claims_open takes no inputs beyond the value it is called on. It gives back bool.

(Where these words came from: inferred.)

---

## function — Gate::locked

Found in src/oauth.rs at line 1276.

Surface: public API.

Signature: `pub fn locked(&self) -> bool {`

What it does: True when some credential is required for the API.

(Where these words came from: direct.)

---

## function — Gate::oauth

Found in src/oauth.rs at line 1280.

Surface: public API.

Signature: `pub fn oauth(&self) -> &Oauth {`

What it does: Function Gate::oauth takes no inputs beyond the value it is called on. It gives back &Oauth.

(Where these words came from: inferred.)

---

## function — Gate::editors

Found in src/oauth.rs at line 1284.

Surface: public API.

Signature: `pub fn editors(&self) -> &Editors {`

What it does: Function Gate::editors takes no inputs beyond the value it is called on. It gives back &Editors.

(Where these words came from: inferred.)

---

## function — Gate::session

Found in src/oauth.rs at line 1291.

Surface: public API.

Signature: `pub fn session(&self, headers: &HashMap<String, String>) -> Option<Session> {`

What it does: The signed-in session for a request, if any. In `--local` mode this is the synthetic local developer (never for a forwarded request), mirroring `session_view`; otherwise it is the cookie session.

(Where these words came from: direct.)

---

## function — Gate::is_editor

Found in src/oauth.rs at line 1308.

Surface: public API.

Signature: `pub fn is_editor(&self, session: &Session) -> bool {`

What it does: Whether a session belongs to an approved editor.

(Where these words came from: direct.)

---

## function — Gate::admit_account

Found in src/oauth.rs at line 1315.

Surface: public API.

Signature: `pub fn admit_account(&self, who: &str, email: &str) -> bool {`

What it does: Admit an account as an editor, effective immediately and persisted to the editors file when one is configured. Used when an editor accepts a submission: the originator joins the editors.

(Where these words came from: direct.)

---

## function — Gate::load_editors_file

Found in src/oauth.rs at line 1323.

Surface: public API.

Signature: `pub fn load_editors_file(&self, path: &std::path::Path) {`

What it does: Load additional editor entries from a file (one per line) and make it the backing file for future admissions. The phext-edit server calls this with its default admitted-editors file when `PHEXT_EDITORS_FILE` is not set.

(Where these words came from: direct.)

---

## function — Gate::describe

Found in src/oauth.rs at line 1328.

Surface: public API.

Signature: `pub fn describe(&self) -> Vec<String> {`

What it does: Startup lines describing how the API is protected.

(Where these words came from: direct.)

---

## function — Gate::route

Found in src/oauth.rs at line 1352.

Surface: public API.

Signature: `pub fn route(&self, method: &str, path: &str, headers: &HashMap<String, String>) -> Option<Reply> {`

What it does: Handles `/api/session` and `/api/auth/*`. Returns None for other paths.

(Where these words came from: direct.)

---

## function — Gate::admit

Found in src/oauth.rs at line 1396.

Surface: public API.

Signature: `pub fn admit(&self, method: &str, path: &str, headers: &HashMap<String, String>) -> Result<(), Reply> {`

What it does: Decides whether a non-auth request may proceed. The gate is by operation, not by surface (Will, 2026-11-19: "only require a login to edit"). Reading — every GET/HEAD on the API, plus the editor's viewport POSTs and its search — is open to anyone, signed in or not. Writing requires a session on the editor list, with the CSRF check writes have always had; a signed-in non-editor's write keeps the editor-list refusal.

(Where these words came from: direct.)

---

## function — Gate::sso

Found in src/oauth.rs at line 1545.

Surface: public API.

Signature: `pub fn sso( &self, aud: &str, headers: &HashMap<String, String>, query: &HashMap<String, String>, ) -> Reply {`

What it does: `GET /api/auth/sso/<aud>` — the cross-site handoff to one allowlisted audience (`mutable`, `exocortexia`, `wishnode`). Layering follows the claims/submissions precedent in the servers: a signed-in account is enough; an editor's place is not required (`sso_admits` is the single predicate that decides). With a session, the reply 302s to the audience's callback with a freshly sealed assertion. Without one, it 302s to this site's own root with this endpoint as the `next`, so the ordinary provider sign-in returns the browser here and the handoff completes — the OAuth `next` itself stays same-site throughout. An audience outside the allowlist, or one not configured (no SSO secret, or no target for that audience), simply does not exist: 404 — checked before any session handling, so an unknown audience never enters the sign-in return flow.

(Where these words came from: direct.)

---

## function — cookie_header

Found in src/oauth.rs at line 1743.

Surface: public API.

Signature: `pub fn cookie_header(name: &str, value: &str, max_age: i64, secure: bool) -> String {`

What it does: Function cookie_header takes name: &str, value: &str, max_age: i64, secure: bool. It gives back String.

(Where these words came from: inferred.)

---

## function — safe_next

Found in src/oauth.rs at line 1850.

Surface: public API.

Signature: `pub fn safe_next(next: &str) -> String {`

What it does: Keeps the post-sign-in redirect on this site: a path, optionally with a `#coordinate` fragment. A `:` is refused in the path portion only. Past the `?`, a query may legitimately carry URLs of its own — an OAuth authorization request returning from sign-in names https redirect URIs and, for CIMD clients, an https client_id — and a colon there cannot move the destination off this site, because the value still begins with a single `/` and browsers resolve it as a path on this origin. The length cap admits those same authorization requests (a full `/oauth/authorize?...` round trip runs several hundred characters); the sealed pending cookie that carries `next` through a provider sign-in holds it comfortably.

(Where these words came from: direct.)

---

## function — loopback_return

Found in src/oauth.rs at line 1888.

Surface: public API.

Signature: `pub fn loopback_return(raw: &str) -> Option<String> {`

What it does: Admit a desktop return URL: the address of the listener the app opened for this one sign-in attempt. The bar is the loopback literal itself, checked character by character — not a URL parse that a hostname or userinfo could slip through: scheme `http` exactly. Loopback has a real origin but no CA issues certificates for it; RFC 8252's native-app practice is plain http on the loopback interface, which never leaves the machine. host the literal `127.0.0.1` or `[::1]` — not `localhost`, not any other name, not another loopback address. A name is whatever the resolver says it is (hosts files are editable), and the app binds the numeric loopback anyway. an explicit port, 1–65535 — the app always listens on an ephemeral port it was given, never a default. then only a path/query tail of plain URL characters: no `#` (a fragment never reaches a server anyway, and its presence means the value was not a listener address), no whitespace, controls, or non-ASCII. The result is capped short enough to ride the sign-in bounce — this endpoint travels as the `next` of a `/?next=` redirect, well inside the length `safe_next` admits there.

(Where these words came from: direct.)

---

## function — valid_state_nonce

Found in src/oauth.rs at line 1930.

Surface: public API.

Signature: `pub fn valid_state_nonce(state: &str) -> bool {`

What it does: The desktop attempt's state nonce: long enough to be unguessable, plain enough to travel in URLs and assertions unchanged. The app generates it (32 random hex/letters in practice); this only checks the shape.

(Where these words came from: direct.)

---

## function — valid_who

Found in src/oauth.rs at line 1937.

Surface: public API.

Signature: `pub fn valid_who(who: &str) -> bool {`

What it does: Function valid_who takes who: &str. It gives back bool.

(Where these words came from: inferred.)

---

## function — display_name

Found in src/oauth.rs at line 1955.

Surface: public API.

Signature: `pub fn display_name(raw: &str, fallback: &str) -> String {`

What it does: Function display_name takes raw: &str, fallback: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — clean_email

Found in src/oauth.rs at line 1970.

Surface: public API.

Signature: `pub fn clean_email(raw: &str) -> String {`

What it does: Function clean_email takes raw: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — clean_handle

Found in src/oauth.rs at line 1991.

Surface: public API.

Signature: `pub fn clean_handle(raw: &str) -> String {`

What it does: Function clean_handle takes raw: &str. It gives back String.

(Where these words came from: inferred.)

---

## function — clean_twitter_handle

Found in src/oauth.rs at line 2004.

Surface: public API.

Signature: `pub fn clean_twitter_handle(raw: &str) -> String {`

What it does: An X username verbatim: `[A-Za-z0-9_]{1,15}`, case preserved. Unlike `clean_handle` this keeps underscores — they are load-bearing in X handles (@Grady_Booch, @4ster_light) and the wavefront lookup keys on them. Anything outside the charset makes the handle unusable ("").

(Where these words came from: direct.)

---

## function — token_eq

Found in src/oauth.rs at line 2016.

Surface: public API.

Signature: `pub fn token_eq(expected: &str, given: &str) -> bool {`

What it does: Constant-time comparison; an empty expected value never matches.

(Where these words came from: direct.)

---

## function — is_loopback_bind

Found in src/oauth.rs at line 2285.

Surface: public API.

Signature: `pub fn is_loopback_bind(host: &str) -> bool {`

What it does: True when `host` is a loopback listen address: 127.0.0.0/8, `::1`, or `localhost`. `0.0.0.0` and `::` are not loopback — they accept remote connections.

(Where these words came from: direct.)

---

## struct — SearchHit

Found in src/search.rs at line 19.

Surface: public API.

Signature: `pub struct SearchHit {`

What it does: A search hit: coordinate + context around the match.

(Where these words came from: direct.)

---

## function — search_lattice

Found in src/search.rs at line 132.

Surface: public API.

Signature: `pub fn search_lattice( buffer: &[u8], index: &LatticeIndex, pattern: &str, case_sensitive: bool, max_results: usize, ) -> Vec<SearchHit> {`

What it does: Search the lattice for a text pattern (single-threaded). Returns hits in document order (by coordinate).

(Where these words came from: direct.)

---

## function — search_lattice_parallel

Found in src/search.rs at line 177.

Surface: public API.

Signature: `pub fn search_lattice_parallel( buffer: &[u8], index: &LatticeIndex, pattern: &str, case_sensitive: bool, max_results: usize, ) -> Vec<SearchHit> {`

What it does: Search the lattice for a text pattern across all available cores. Uses rayon's work-stealing thread pool. Each scroll is a work unit — scrolls are independent so there's zero contention. Results are collected per-thread then merged in coordinate order. Falls back to serial for small lattices (< PARALLEL_THRESHOLD scrolls) where thread synchronization cost exceeds the parallelism benefit.

(Where these words came from: direct.)

---

## function — search_lattice_auto

Found in src/search.rs at line 261.

Surface: public API.

Signature: `pub fn search_lattice_auto( buffer: &[u8], index: &LatticeIndex, pattern: &str, case_sensitive: bool, max_results: usize, ) -> Vec<SearchHit> {`

What it does: Auto-selecting search: picks serial or parallel based on data characteristics. This is what the editor should call.

(Where these words came from: direct.)

---

## function — search_coordinates

Found in src/search.rs at line 274.

Surface: public API.

Signature: `pub fn search_coordinates( index: &LatticeIndex, pattern: &str, ) -> Vec<Coordinate> {`

What it does: Search for coordinates matching a coordinate pattern string. Supports partial coordinates like "3.3.3" (matches any coordinate where any arm starts with 3.3.3).

(Where these words came from: direct.)

---

## function — build_index_parallel

Found in src/search.rs at line 292.

Surface: public API.

Signature: `pub fn build_index_parallel(buffer: &[u8], chunk_size: usize) -> LatticeIndex {`

What it does: Parallel index build — partition the buffer into chunks, scan each for delimiter boundaries, then merge. For very large phexts (>100MB) where the O(n) sequential scan becomes noticeable. Currently unused — sequential build is ~30ms for 4.26 MB which is already below perceptual threshold. Included for future scaling.

(Where these words came from: direct.)

---

## enum — AxisGroup

Found in src/sentron.rs at line 36.

Surface: public API.

Signature: `pub enum AxisGroup {`

What it does: The two fundamental axis groups in the sentron topology.

(Where these words came from: direct.)

---

## function — AxisGroup::name

Found in src/sentron.rs at line 44.

Surface: public API.

Signature: `pub fn name(&self) -> &'static str {`

What it does: Function AxisGroup::name takes no inputs beyond the value it is called on. It gives back &'static str.

(Where these words came from: inferred.)

---

## function — AxisGroup::dimensions

Found in src/sentron.rs at line 52.

Surface: public API.

Signature: `pub fn dimensions(&self) -> [Dimension; 4] {`

What it does: The 4 dimensions in this axis group.

(Where these words came from: direct.)

---

## struct — Axon

Found in src/sentron.rs at line 72.

Surface: public API.

Signature: `pub struct Axon {`

What it does: One connection axis: a dimension with backward/forward reach.

(Where these words came from: direct.)

---

## function — Axon::total

Found in src/sentron.rs at line 87.

Surface: public API.

Signature: `pub fn total(&self) -> usize {`

What it does: Total reachable coordinates along this axis.

(Where these words came from: direct.)

---

## function — Axon::weight

Found in src/sentron.rs at line 92.

Surface: public API.

Signature: `pub fn weight(&self, max_total: usize) -> f64 {`

What it does: Visual weight: how "thick" this connection is (0.0 to 1.0).

(Where these words came from: direct.)

---

## struct — Neuron

Found in src/sentron.rs at line 100.

Surface: public API.

Signature: `pub struct Neuron {`

What it does: A neuron in the sentron: a populated coordinate with its 8 axons.

(Where these words came from: direct.)

---

## struct — Sentron

Found in src/sentron.rs at line 110.

Surface: public API.

Signature: `pub struct Sentron {`

What it does: The sentron: 40 nearest populated coordinates to the cursor, with full 8-way connectivity analysis.

(Where these words came from: direct.)

---

## const — SENTRON_CAPACITY

Found in src/sentron.rs at line 126.

Surface: public API.

Signature: `pub const SENTRON_CAPACITY: usize = 40;`

What it does: The maximum number of neurons in a sentron.

(Where these words came from: direct.)

---

## const — CONNECTIONS_PER_NEURON

Found in src/sentron.rs at line 129.

Surface: public API.

Signature: `pub const CONNECTIONS_PER_NEURON: usize = 8;`

What it does: The number of connections per neuron.

(Where these words came from: direct.)

---

## function — Sentron::build

Found in src/sentron.rs at line 136.

Surface: public API.

Signature: `pub fn build(center: &Coordinate, index: &LatticeIndex) -> Sentron {`

What it does: Build a sentron centered on a coordinate. Finds the 40 nearest populated coordinates and analyzes their 8-way connectivity along structural/sequential axes.

(Where these words came from: direct.)

---

## function — Sentron::size

Found in src/sentron.rs at line 175.

Surface: public API.

Signature: `pub fn size(&self) -> usize {`

What it does: How many neurons are in this sentron.

(Where these words came from: direct.)

---

## function — Sentron::structural_axons

Found in src/sentron.rs at line 180.

Surface: public API.

Signature: `pub fn structural_axons(&self) -> &[Axon] {`

What it does: The structural (spatial) axons: Library, Shelf, Series, Collection.

(Where these words came from: direct.)

---

## function — Sentron::sequential_axons

Found in src/sentron.rs at line 185.

Surface: public API.

Signature: `pub fn sequential_axons(&self) -> &[Axon] {`

What it does: The sequential (temporal) axons: Volume, Book, Chapter, Section.

(Where these words came from: direct.)

---

## function — Sentron::topology_string

Found in src/sentron.rs at line 191.

Surface: public API.

Signature: `pub fn topology_string(&self) -> String {`

What it does: Render the sentron as a compact topology string. Shows connection density along each axis.

(Where these words came from: direct.)

---

## function — is_break

Found in src/sidecar.rs at line 83.

Surface: public API.

Signature: `pub fn is_break(b: u8) -> bool {`

What it does: Whether `b` is a phext dimension break.

(Where these words came from: direct.)

---

## type — Selector

Found in src/sidecar.rs at line 89.

Surface: public API.

Signature: `pub type Selector = Box<dyn Fn(&Coordinate) -> Option<Coordinate> + Send + Sync>;`

What it does: Maps a source scroll to the coordinate an index entry should point at, or `None` to leave the scroll out of that index.

(Where these words came from: direct.)

---

## struct — IndexSpec

Found in src/sidecar.rs at line 94.

Surface: public API.

Signature: `pub struct IndexSpec {`

What it does: Which scrolls feed which index. The selectors can't be persisted, so the `label` stands in for them: change the selectors, change the label, and the next open rebuilds instead of trusting a sidecar built another way.

(Where these words came from: direct.)

---

## function — IndexSpec::new

Found in src/sidecar.rs at line 103.

Surface: public API.

Signature: `pub fn new(label: &str, names: Selector, terms: Selector) -> IndexSpec {`

What it does: Function IndexSpec::new takes label: &str, names: Selector, terms: Selector. It gives back IndexSpec.

(Where these words came from: inferred.)

---

## function — IndexSpec::scrolls

Found in src/sidecar.rs at line 108.

Surface: public API.

Signature: `pub fn scrolls() -> IndexSpec {`

What it does: Every scroll is its own term document; no names.

(Where these words came from: direct.)

---

## enum — OpenMode

Found in src/sidecar.rs at line 115.

Surface: public API.

Signature: `pub enum OpenMode {`

What it does: How `IndexedLattice::open` got its indexes.

(Where these words came from: direct.)

---

## struct — SyncReport

Found in src/sidecar.rs at line 127.

Surface: public API.

Signature: `pub struct SyncReport {`

What it does: What a `save()` or sync did.

(Where these words came from: direct.)

---

## function — sidecar_dir

Found in src/sidecar.rs at line 137.

Surface: public API.

Signature: `pub fn sidecar_dir(source: &Path) -> PathBuf {`

What it does: The sidecar directory for a phext path: `wiki.phext` → `wiki.phext.idx`.

(Where these words came from: direct.)

---

## struct — IndexedLattice

Found in src/sidecar.rs at line 733.

Surface: public API.

Signature: `pub struct IndexedLattice {`

What it does: A `MappedLattice` whose span, name, and term indexes persist in a sidecar. Queries reflect the last `save()` (or open); unsaved overlay edits are visible through `lattice()` reads but not through `name`/`resolve`/`search` until saved.

(Where these words came from: direct.)

---

## function — IndexedLattice::open

Found in src/sidecar.rs at line 746.

Surface: public API.

Signature: `pub fn open<P: AsRef<Path>>(path: P, spec: IndexSpec) -> io::Result<(IndexedLattice, OpenMode)> {`

What it does: Open `path` with indexes from its sidecar: load them if they match the source, patch them if the source changed, or build them if absent.

(Where these words came from: direct.)

---

## function — IndexedLattice::rebuild

Found in src/sidecar.rs at line 793.

Surface: public API.

Signature: `pub fn rebuild(&mut self) -> io::Result<SyncReport> {`

What it does: Discard the sidecar and index every scroll again.

(Where these words came from: direct.)

---

## function — IndexedLattice::lattice

Found in src/sidecar.rs at line 800.

Surface: public API.

Signature: `pub fn lattice(&self) -> &MappedLattice {`

What it does: Function IndexedLattice::lattice takes no inputs beyond the value it is called on. It gives back &MappedLattice.

(Where these words came from: inferred.)

---

## function — IndexedLattice::lattice_mut

Found in src/sidecar.rs at line 805.

Surface: public API.

Signature: `pub fn lattice_mut(&mut self) -> &mut MappedLattice {`

What it does: Edit through the lattice, then `save()` to persist and re-sync the indexes.

(Where these words came from: direct.)

---

## function — IndexedLattice::sidecar

Found in src/sidecar.rs at line 810.

Surface: public API.

Signature: `pub fn sidecar(&self) -> &Path {`

What it does: The sidecar directory.

(Where these words came from: direct.)

---

## function — IndexedLattice::delta_len

Found in src/sidecar.rs at line 815.

Surface: public API.

Signature: `pub fn delta_len(&self) -> usize {`

What it does: Scrolls changed since the base (entries in the delta).

(Where these words came from: direct.)

---

## function — IndexedLattice::set_compact_threshold

Found in src/sidecar.rs at line 821.

Surface: public API.

Signature: `pub fn set_compact_threshold(&mut self, n: usize) {`

What it does: Fold the delta into the base once it holds more than `n` scrolls. Defaults to 64 + 1% of the source's scrolls.

(Where these words came from: direct.)

---

## function — IndexedLattice::save

Found in src/sidecar.rs at line 827.

Surface: public API.

Signature: `pub fn save(&mut self) -> io::Result<SyncReport> {`

What it does: Save pending edits to the source, then record exactly the edited scrolls in the delta (or compact, past the threshold).

(Where these words came from: direct.)

---

## function — IndexedLattice::compact

Found in src/sidecar.rs at line 837.

Surface: public API.

Signature: `pub fn compact(&mut self) -> io::Result<SyncReport> {`

What it does: Fold the delta into a new base now.

(Where these words came from: direct.)

---

## function — IndexedLattice::name

Found in src/sidecar.rs at line 845.

Surface: public API.

Signature: `pub fn name(&self, name: &str) -> Option<Coordinate> {`

What it does: Exact name lookup (first owner in document order wins).

(Where these words came from: direct.)

---

## function — IndexedLattice::resolve

Found in src/sidecar.rs at line 851.

Surface: public API.

Signature: `pub fn resolve(&self, name: &str) -> Option<Coordinate> {`

What it does: Exact lookup, then case-folded.

(Where these words came from: direct.)

---

## function — IndexedLattice::search

Found in src/sidecar.rs at line 882.

Surface: public API.

Signature: `pub fn search(&self, query: &str, limit: usize) -> Vec<TermHit> {`

What it does: BM25-ranked documents for a free-text query (terms OR'ed), best first.

(Where these words came from: direct.)

---

## function — IndexedLattice::doc_count

Found in src/sidecar.rs at line 929.

Surface: public API.

Signature: `pub fn doc_count(&self) -> usize {`

What it does: Live term documents.

(Where these words came from: direct.)

---

## const — ENV_NAME

Found in src/sponsor.rs at line 10.

Surface: public API.

Signature: `pub const ENV_NAME: &str = "PHEXT_SPONSOR_URL";`

What it does: Environment variable deploy.sh writes into the phext-edit service environment.

(Where these words came from: direct.)

---

## const — SLOT

Found in src/sponsor.rs at line 13.

Surface: public API.

Signature: `pub const SLOT: &str = "<!--phext-sponsor-->";`

What it does: Placeholder in the public HTML. Replaced when a page is served.

(Where these words came from: direct.)

---

## struct — SponsorOffer

Found in src/sponsor.rs at line 19.

Surface: public API.

Signature: `pub struct SponsorOffer {`

What it does: A configured donation URL, or nothing.

(Where these words came from: direct.)

---

## function — SponsorOffer::hidden

Found in src/sponsor.rs at line 24.

Surface: public API.

Signature: `pub fn hidden() -> Self {`

What it does: Function SponsorOffer::hidden takes no inputs beyond the value it is called on. It gives back Self.

(Where these words came from: inferred.)

---

## function — SponsorOffer::from_env

Found in src/sponsor.rs at line 31.

Surface: public API.

Signature: `pub fn from_env() -> Self {`

What it does: Read [`ENV_NAME`]. A missing variable and a blank value both hide the link. A non-empty value that is not a usable https URL also hides it, and warns once per call so a bad deploy is visible without breaking startup.

(Where these words came from: direct.)

---

## function — SponsorOffer::parse

Found in src/sponsor.rs at line 49.

Surface: public API.

Signature: `pub fn parse(raw: &str) -> Self {`

What it does: Accept a Stripe Payment Link (or any https URL safe to place in an href). Rejects anything that could break out of the attribute or send the visitor somewhere other than an https page.

(Where these words came from: direct.)

---

## function — SponsorOffer::url

Found in src/sponsor.rs at line 56.

Surface: public API.

Signature: `pub fn url(&self) -> Option<&str> {`

What it does: Function SponsorOffer::url takes no inputs beyond the value it is called on. It gives back Option<&str>.

(Where these words came from: inferred.)

---

## function — SponsorOffer::markup

Found in src/sponsor.rs at line 61.

Surface: public API.

Signature: `pub fn markup(&self) -> String {`

What it does: The visible control, or an empty string when the link is hidden.

(Where these words came from: direct.)

---

## function — apply_slot

Found in src/sponsor.rs at line 76.

Surface: public API.

Signature: `pub fn apply_slot(html: &str, offer: &SponsorOffer) -> String {`

What it does: Replace every [`SLOT`] in `html`. Pages without the slot are unchanged apart from the usual allocate-on-replace cost when the caller already checked; this function always returns an owned string.

(Where these words came from: direct.)

---

## struct — ScrollStats

Found in src/stats.rs at line 13.

Surface: public API.

Signature: `pub struct ScrollStats {`

What it does: Per-scroll statistics for the status bar and info panel.

(Where these words came from: direct.)

---

## function — ScrollStats::from_bytes

Found in src/stats.rs at line 22.

Surface: public API.

Signature: `pub fn from_bytes(bytes: &[u8]) -> ScrollStats {`

What it does: Compute stats from raw scroll bytes.

(Where these words came from: direct.)

---

## function — ScrollStats::size_display

Found in src/stats.rs at line 33.

Surface: public API.

Signature: `pub fn size_display(&self) -> String {`

What it does: Human-readable size.

(Where these words came from: direct.)

---

## struct — DimensionDensity

Found in src/stats.rs at line 47.

Surface: public API.

Signature: `pub struct DimensionDensity {`

What it does: Density profile for one dimension: how many populated scrolls exist at each value along that dimension.

(Where these words came from: direct.)

---

## function — DimensionDensity::build

Found in src/stats.rs at line 59.

Surface: public API.

Signature: `pub fn build(dimension: Dimension, index: &LatticeIndex) -> DimensionDensity {`

What it does: Build the density profile for one dimension across all populated coordinates.

(Where these words came from: direct.)

---

## function — DimensionDensity::sparkline

Found in src/stats.rs at line 77.

Surface: public API.

Signature: `pub fn sparkline(&self, width: usize) -> String {`

What it does: Render a sparkline string showing relative density. Width is the number of characters available.

(Where these words came from: direct.)

---

## function — DimensionDensity::distinct_values

Found in src/stats.rs at line 120.

Surface: public API.

Signature: `pub fn distinct_values(&self) -> usize {`

What it does: How many distinct values exist in this dimension.

(Where these words came from: direct.)

---

## function — DimensionDensity::range

Found in src/stats.rs at line 125.

Surface: public API.

Signature: `pub fn range(&self) -> Option<(usize, usize)> {`

What it does: The range of populated values.

(Where these words came from: direct.)

---

## struct — Neighborhood

Found in src/stats.rs at line 139.

Surface: public API.

Signature: `pub struct Neighborhood {`

What it does: Neighborhood context: what's near the current coordinate.

(Where these words came from: direct.)

---

## function — Neighborhood::build

Found in src/stats.rs at line 152.

Surface: public API.

Signature: `pub fn build(coord: &Coordinate, index: &LatticeIndex) -> Neighborhood {`

What it does: Build neighborhood context for a coordinate.

(Where these words came from: direct.)

---

## struct — LatticeOverview

Found in src/stats.rs at line 195.

Surface: public API.

Signature: `pub struct LatticeOverview {`

What it does: Global lattice overview stats.

(Where these words came from: direct.)

---

## function — LatticeOverview::build

Found in src/stats.rs at line 206.

Surface: public API.

Signature: `pub fn build(_buffer: &[u8], index: &LatticeIndex) -> LatticeOverview {`

What it does: Build a complete overview of the lattice.

(Where these words came from: direct.)

---

## enum — SubmissionStatus

Found in src/submissions.rs at line 20.

Surface: public API.

Signature: `pub enum SubmissionStatus {`

What it does: A named choice named SubmissionStatus. It is one of: Pending; Accepted; Declined.

(Where these words came from: inferred.)

---

## struct — Submission

Found in src/submissions.rs at line 27.

Surface: public API.

Signature: `pub struct Submission {`

What it does: A data shape named Submission. Its fields are: pub id: String; pub filename: String; pub who: String; pub email: String; pub name: String; pub submitted_at: i64; pub bytes: usize; pub scrolls: usize; pub status: SubmissionStatus; pub decided_by: String; pub decided_at: i64; pub curated_as: String.

(Where these words came from: inferred.)

---

## function — sanitize_filename

Found in src/submissions.rs at line 65.

Surface: public API.

Signature: `pub fn sanitize_filename(raw: &str) -> Option<String> {`

What it does: Reduce an offered file name to a bare, safe `*.phext` name, or None when nothing usable remains. Directory parts are stripped, so `../../notes.phext` is just `notes.phext`.

(Where these words came from: direct.)

---

## function — count_scrolls

Found in src/submissions.rs at line 86.

Surface: public API.

Signature: `pub fn count_scrolls(content: &str) -> Option<usize> {`

What it does: Count the populated scrolls in a phext text. Wrapped in `catch_unwind`: the text is user input, and a parser panic must become a rejection, never a crashed request.

(Where these words came from: direct.)

---

## function — create

Found in src/submissions.rs at line 116.

Surface: public API.

Signature: `pub fn create( dir: &Path, filename: &str, who: &str, email: &str, name: &str, content: &str, ) -> Result<Submission, String> {`

What it does: Store a new pending submission. Validates the file name and the phext (non-empty, parseable) before anything touches disk.

(Where these words came from: direct.)

---

## function — list

Found in src/submissions.rs at line 159.

Surface: public API.

Signature: `pub fn list(dir: &Path) -> Vec<Submission> {`

What it does: Every submission in the directory, oldest first. Unreadable or malformed entries are skipped, never fatal.

(Where these words came from: direct.)

---

## function — get

Found in src/submissions.rs at line 181.

Surface: public API.

Signature: `pub fn get(dir: &Path, id: &str) -> Option<Submission> {`

What it does: One submission by id, or None. Ids from the network are validated before they ever become a path.

(Where these words came from: direct.)

---

## function — content

Found in src/submissions.rs at line 194.

Surface: public API.

Signature: `pub fn content(dir: &Path, id: &str) -> Option<Vec<u8>> {`

What it does: The uploaded phext bytes for a submission.

(Where these words came from: direct.)

---

## function — set_decision

Found in src/submissions.rs at line 203.

Surface: public API.

Signature: `pub fn set_decision( dir: &Path, id: &str, status: SubmissionStatus, decided_by: &str, curated_as: &str, ) -> Result<Submission, String> {`

What it does: Record an accept or decline. The caller decides the status rules (only a pending submission may be decided); this just persists.

(Where these words came from: direct.)

---

## struct — Bm25Params

Found in src/term_index.rs at line 22.

Surface: public API.

Signature: `pub struct Bm25Params {`

What it does: BM25 tuning. The defaults are the usual Robertson/Lucene values.

(Where these words came from: direct.)

---

## struct — TermHit

Found in src/term_index.rs at line 37.

Surface: public API.

Signature: `pub struct TermHit {`

What it does: A ranked hit: the document coordinate and its BM25 score.

(Where these words came from: direct.)

---

## struct — TermIndex

Found in src/term_index.rs at line 50.

Surface: public API.

Signature: `pub struct TermIndex {`

What it does: Inverted index over lattice documents with BM25 ranking.

(Where these words came from: direct.)

---

## function — tokenize

Found in src/term_index.rs at line 67.

Surface: public API.

Signature: `pub fn tokenize(text: &str) -> impl Iterator<Item = String> + '_ {`

What it does: Split text into index terms: lowercased runs of letters/digits, at least 2 chars.

(Where these words came from: direct.)

---

## function — TermIndex::new

Found in src/term_index.rs at line 91.

Surface: public API.

Signature: `pub fn new(params: Bm25Params) -> TermIndex {`

What it does: An empty index.

(Where these words came from: direct.)

---

## function — TermIndex::build

Found in src/term_index.rs at line 106.

Surface: public API.

Signature: `pub fn build(lattice: &MappedLattice) -> TermIndex {`

What it does: Index every populated scroll as its own document (overlay edits included).

(Where these words came from: direct.)

---

## function — TermIndex::build_grouped

Found in src/term_index.rs at line 113.

Surface: public API.

Signature: `pub fn build_grouped<F>(lattice: &MappedLattice, group: F) -> TermIndex where F: Fn(&Coordinate) -> Option<Coordinate> + Sync, {`

What it does: Index scrolls grouped into documents: `group` maps a scroll coordinate to the coordinate of the document it belongs to, or `None` to leave it out. Tokenization runs in parallel; documents keep document order.

(Where these words came from: direct.)

---

## function — TermIndex::upsert

Found in src/term_index.rs at line 177.

Surface: public API.

Signature: `pub fn upsert(&mut self, coord: Coordinate, text: &str) {`

What it does: Add or replace the document at `coord` with `text`. Replacement retires the old document id and appends a new one, so postings stay sorted without rewriting them. Retired postings still occupy memory and still count toward document frequency until the index is rebuilt; check `retired()` to decide when.

(Where these words came from: direct.)

---

## function — TermIndex::remove

Found in src/term_index.rs at line 184.

Surface: public API.

Signature: `pub fn remove(&mut self, coord: &Coordinate) -> bool {`

What it does: Remove the document at `coord`. Returns whether it was indexed.

(Where these words came from: direct.)

---

## function — TermIndex::search

Found in src/term_index.rs at line 200.

Surface: public API.

Signature: `pub fn search(&self, query: &str, limit: usize) -> Vec<TermHit> {`

What it does: Rank documents against a free-text query (terms are OR'ed). Ties break in document order, so results are deterministic.

(Where these words came from: direct.)

---

## function — TermIndex::doc_count

Found in src/term_index.rs at line 240.

Surface: public API.

Signature: `pub fn doc_count(&self) -> usize {`

What it does: Number of live documents.

(Where these words came from: direct.)

---

## function — TermIndex::term_count

Found in src/term_index.rs at line 245.

Surface: public API.

Signature: `pub fn term_count(&self) -> usize {`

What it does: Number of distinct terms.

(Where these words came from: direct.)

---

## function — TermIndex::retired

Found in src/term_index.rs at line 250.

Surface: public API.

Signature: `pub fn retired(&self) -> usize {`

What it does: Documents replaced or removed since the last build.

(Where these words came from: direct.)

---

## function — TermIndex::document_frequency

Found in src/term_index.rs at line 255.

Surface: public API.

Signature: `pub fn document_frequency(&self, term: &str) -> usize {`

What it does: How many documents (including retired ones) contain `term`.

(Where these words came from: direct.)

---

## function — TermIndex::heap_bytes

Found in src/term_index.rs at line 260.

Surface: public API.

Signature: `pub fn heap_bytes(&self) -> usize {`

What it does: Approximate heap footprint in bytes.

(Where these words came from: direct.)

---

## const — MAX_Q_BYTES

Found in src/traversals.rs at line 23.

Surface: public API.

Signature: `pub const MAX_Q_BYTES: usize = 64 * 1024;`

What it does: The largest `q` string a record may carry (64 KiB). A traversal is a list of addresses; this is a storage bound, not a grammar limit — the tour grammar itself is unbounded.

(Where these words came from: direct.)

---

## struct — Traversal

Found in src/traversals.rs at line 26.

Surface: public API.

Signature: `pub struct Traversal {`

What it does: A data shape named Traversal. Its fields are: pub id: String; pub who: String; pub name: String; pub q: String; pub created: i64; pub updated: i64.

(Where these words came from: inferred.)

---

## function — valid_name

Found in src/traversals.rs at line 66.

Surface: public API.

Signature: `pub fn valid_name(raw: &str) -> Option<String> {`

What it does: A name is usable when, trimmed, it holds 1..=120 chars.

(Where these words came from: direct.)

---

## function — count_steps

Found in src/traversals.rs at line 107.

Surface: public API.

Signature: `pub fn count_steps(q: &str) -> usize {`

What it does: How many steps of a `q` string parse under the tour grammar.

(Where these words came from: direct.)

---

## function — valid_q

Found in src/traversals.rs at line 113.

Surface: public API.

Signature: `pub fn valid_q(raw: &str) -> Option<String> {`

What it does: A `q` string is storable when it is non-empty, within the storage bound, and at least one of its steps parses.

(Where these words came from: direct.)

---

## function — create

Found in src/traversals.rs at line 126.

Surface: public API.

Signature: `pub fn create(dir: &Path, who: &str, name: &str, q: &str) -> Result<Traversal, String> {`

What it does: Store a new traversal for `who`. Validates name and `q` before anything touches disk.

(Where these words came from: direct.)

---

## function — list

Found in src/traversals.rs at line 152.

Surface: public API.

Signature: `pub fn list(dir: &Path) -> Vec<Traversal> {`

What it does: Every traversal in the directory, oldest first. Unreadable or malformed entries are skipped, never fatal.

(Where these words came from: direct.)

---

## function — get

Found in src/traversals.rs at line 174.

Surface: public API.

Signature: `pub fn get(dir: &Path, id: &str) -> Option<Traversal> {`

What it does: One traversal by id, or None. Ids from the network are validated before they ever become a path.

(Where these words came from: direct.)

---

## function — update

Found in src/traversals.rs at line 188.

Surface: public API.

Signature: `pub fn update(dir: &Path, id: &str, name: &str, q: &str) -> Result<Traversal, String> {`

What it does: Replace a record's name and `q` (ownership is the caller's check, as in `submissions::set_decision`).

(Where these words came from: direct.)

---

## function — remove

Found in src/traversals.rs at line 200.

Surface: public API.

Signature: `pub fn remove(dir: &Path, id: &str) -> bool {`

What it does: Delete a record. Returns false when it was not there.

(Where these words came from: direct.)

---

## const — ZERO_SOUND

Found in src/tts.rs at line 14.

Surface: public API.

Signature: `pub const ZERO_SOUND: &str = "om";`

What it does: Zero = "om" — the nada sound, silence between words

(Where these words came from: direct.)

---

## function — byte_to_syllable

Found in src/tts.rs at line 47.

Surface: public API.

Signature: `pub fn byte_to_syllable(value: u8) -> String {`

What it does: Convert a single byte value (0-255) to a syllable

(Where these words came from: direct.)

---

## function — dim_to_syllables

Found in src/tts.rs at line 62.

Surface: public API.

Signature: `pub fn dim_to_syllables(value: u32) -> String {`

What it does: Convert a coordinate dimension value (e.g., 1, 5, 9) to a syllable sequence

(Where these words came from: direct.)

---

## struct — SubCoord

Found in src/tts.rs at line 82.

Surface: public API.

Signature: `pub struct SubCoord {`

What it does: A 3D subcoordinate (e.g., 1.1.1)

(Where these words came from: direct.)

---

## function — SubCoord::new

Found in src/tts.rs at line 89.

Surface: public API.

Signature: `pub fn new(a: u32, b: u32, c: u32) -> Self {`

What it does: Function SubCoord::new takes a: u32, b: u32, c: u32. It gives back Self.

(Where these words came from: inferred.)

---

## function — SubCoord::pronounce

Found in src/tts.rs at line 94.

Surface: public API.

Signature: `pub fn pronounce(&self) -> String {`

What it does: Pronounce as "a b c" syllables

(Where these words came from: direct.)

---

## function — SubCoord::pronounce_verbose

Found in src/tts.rs at line 103.

Surface: public API.

Signature: `pub fn pronounce_verbose(&self) -> String {`

What it does: Pronounce with element names

(Where these words came from: direct.)

---

## struct — CoordPronunciation

Found in src/tts.rs at line 113.

Surface: public API.

Signature: `pub struct CoordPronunciation {`

What it does: Full 9D coordinate pronunciation

(Where these words came from: direct.)

---

## function — CoordPronunciation::from_coordinate

Found in src/tts.rs at line 121.

Surface: public API.

Signature: `pub fn from_coordinate(coord: &Coordinate) -> Self {`

What it does: Create from a libphext Coordinate

(Where these words came from: direct.)

---

## function — CoordPronunciation::parse

Found in src/tts.rs at line 142.

Surface: public API.

Signature: `pub fn parse(s: &str) -> Option<Self> {`

What it does: Parse from string "1.1.1/1.1.1/1.1.1"

(Where these words came from: direct.)

---

## function — CoordPronunciation::pronounce

Found in src/tts.rs at line 167.

Surface: public API.

Signature: `pub fn pronounce(&self) -> String {`

What it does: Full pronunciation with om boundaries

(Where these words came from: direct.)

---

## function — CoordPronunciation::pronounce_compact

Found in src/tts.rs at line 176.

Surface: public API.

Signature: `pub fn pronounce_compact(&self) -> String {`

What it does: Compact pronunciation (no om boundaries)

(Where these words came from: direct.)

---

## function — CoordPronunciation::pronounce_verbose

Found in src/tts.rs at line 185.

Surface: public API.

Signature: `pub fn pronounce_verbose(&self) -> String {`

What it does: Verbose with dimension labels

(Where these words came from: direct.)

---

## function — CoordPronunciation::ipa_hint

Found in src/tts.rs at line 195.

Surface: public API.

Signature: `pub fn ipa_hint(&self) -> String {`

What it does: Get IPA-style phonetic hint

(Where these words came from: direct.)

---

## function — to_ssml

Found in src/tts.rs at line 205.

Surface: public API.

Signature: `pub fn to_ssml(coord: &str, rate: &str) -> String {`

What it does: Generate SSML for TTS engines (Google, Amazon Polly, etc.)

(Where these words came from: direct.)

---

## function — generate_pronunciation_doc

Found in src/tts.rs at line 225.

Surface: public API.

Signature: `pub fn generate_pronunciation_doc(coordinates: &[&str]) -> String {`

What it does: Generate a batch pronunciation document

(Where these words came from: direct.)

---

## function — special_coords

Found in src/tts.rs at line 257.

Surface: public API.

Signature: `pub fn special_coords() -> Vec<(&'static str, &'static str, String)> {`

What it does: Special coordinates with meaning

(Where these words came from: direct.)

---

## enum — Phase

Found in src/uml6d.rs at line 27.

Surface: public API.

Signature: `pub enum Phase {`

What it does: The 10 SDLC phases mapped to collection values. (Collection 10 = Proofs was added under decision D1 of the phcc modernization design, 2026-11-07: proof artifacts — ACSL contracts, verification receipts, waivers — live at the artifact's own address in their own phase, so the trace walk stays uniform.)

(Where these words came from: direct.)

---

## function — Phase::from_collection

Found in src/uml6d.rs at line 41.

Surface: public API.

Signature: `pub fn from_collection(c: usize) -> Option<Phase> {`

What it does: Function Phase::from_collection takes c: usize. It gives back Option<Phase>.

(Where these words came from: inferred.)

---

## function — Phase::name

Found in src/uml6d.rs at line 57.

Surface: public API.

Signature: `pub fn name(&self) -> &'static str {`

What it does: Function Phase::name takes no inputs beyond the value it is called on. It gives back &'static str.

(Where these words came from: inferred.)

---

## function — Phase::short

Found in src/uml6d.rs at line 72.

Surface: public API.

Signature: `pub fn short(&self) -> &'static str {`

What it does: Function Phase::short takes no inputs beyond the value it is called on. It gives back &'static str.

(Where these words came from: inferred.)

---

## function — Phase::color

Found in src/uml6d.rs at line 88.

Surface: public API.

Signature: `pub fn color(&self) -> &'static str {`

What it does: CSS color for this phase.

(Where these words came from: direct.)

---

## function — Phase::all

Found in src/uml6d.rs at line 104.

Surface: public API.

Signature: `pub fn all() -> &'static [Phase] {`

What it does: All phases in order.

(Where these words came from: direct.)

---

## struct — ArtifactAddr

Found in src/uml6d.rs at line 116.

Surface: public API.

Signature: `pub struct ArtifactAddr {`

What it does: A 5D artifact address — stable across SDLC phases.

(Where these words came from: direct.)

---

## function — ArtifactAddr::from_coordinate

Found in src/uml6d.rs at line 126.

Surface: public API.

Signature: `pub fn from_coordinate(c: &Coordinate) -> Self {`

What it does: Extract the 5D artifact address from a full coordinate.

(Where these words came from: direct.)

---

## function — ArtifactAddr::to_coordinate

Found in src/uml6d.rs at line 138.

Surface: public API.

Signature: `pub fn to_coordinate(&self, phase: Phase) -> Coordinate {`

What it does: Build a full coordinate by combining this address with an SDLC phase and outer dimensions (defaulting outer to 1.1.1).

(Where these words came from: direct.)

---

## function — ArtifactAddr::to_coordinate_with_outer

Found in src/uml6d.rs at line 143.

Surface: public API.

Signature: `pub fn to_coordinate_with_outer( &self, phase: Phase, library: usize, shelf: usize, series: usize, ) -> Coordinate {`

What it does: Build a full coordinate with explicit outer dimensions.

(Where these words came from: direct.)

---

## function — ArtifactAddr::display

Found in src/uml6d.rs at line 166.

Surface: public API.

Signature: `pub fn display(&self) -> String {`

What it does: Display as volume.book.chapter/section.scroll

(Where these words came from: direct.)

---

## struct — UmlContext

Found in src/uml6d.rs at line 179.

Surface: public API.

Signature: `pub struct UmlContext {`

What it does: Full 6D UML context for a coordinate.

(Where these words came from: direct.)

---

## function — UmlContext::from_coordinate

Found in src/uml6d.rs at line 187.

Surface: public API.

Signature: `pub fn from_coordinate(c: &Coordinate) -> Self {`

What it does: Function UmlContext::from_coordinate takes c: &Coordinate. It gives back Self.

(Where these words came from: inferred.)

---

## function — trace_artifact

Found in src/uml6d.rs at line 202.

Surface: public API.

Signature: `pub fn trace_artifact( addr: &ArtifactAddr, index: &LatticeIndex, library: usize, shelf: usize, series: usize, ) -> Vec<(Phase, bool)> {`

What it does: Trace an artifact across all SDLC phases. Returns (phase, has_content) for each populated collection at this 5D address.

(Where these words came from: direct.)

---

## function — artifacts_in_phase

Found in src/uml6d.rs at line 217.

Surface: public API.

Signature: `pub fn artifacts_in_phase( phase: Phase, index: &LatticeIndex, coords: &[Coordinate], ) -> Vec<ArtifactAddr> {`

What it does: Find all populated 5D addresses within a given phase.

(Where these words came from: direct.)

---

## struct — DNode

Found in src/uml6d_diagram.rs at line 54.

Surface: public API.

Signature: `pub struct DNode {`

What it does: A node in a 6D UML diagram: one artifact, drawn in one phase lane.

(Where these words came from: direct.)

---

## struct — DEdge

Found in src/uml6d_diagram.rs at line 64.

Surface: public API.

Signature: `pub struct DEdge {`

What it does: A typed relation between two artifacts.

(Where these words came from: direct.)

---

## struct — Diagram

Found in src/uml6d_diagram.rs at line 73.

Surface: public API.

Signature: `pub struct Diagram {`

What it does: A 6D UML diagram: nodes + edges under one projection phase.

(Where these words came from: direct.)

---

## struct — ParseError

Found in src/uml6d_diagram.rs at line 85.

Surface: public API.

Signature: `pub struct ParseError {`

What it does: A data shape named ParseError. Its fields are: pub line: usize; pub message: String.

(Where these words came from: inferred.)

---

## function — parse_addr

Found in src/uml6d_diagram.rs at line 134.

Surface: public API.

Signature: `pub fn parse_addr(s: &str) -> Result<ArtifactAddr, String> {`

What it does: Parse a 5D artifact address in display form: volume.book.chapter/section.scroll

(Where these words came from: direct.)

---

## function — parse_diagram

Found in src/uml6d_diagram.rs at line 224.

Surface: public API.

Signature: `pub fn parse_diagram(text: &str) -> Result<Diagram, ParseError> {`

What it does: Parse the diagram notation.

(Where these words came from: direct.)

---

## function — Diagram::to_notation

Found in src/uml6d_diagram.rs at line 384.

Surface: public API.

Signature: `pub fn to_notation(&self) -> String {`

What it does: Emit the canonical notation for this diagram.

(Where these words came from: direct.)

---

## function — Diagram::project

Found in src/uml6d_diagram.rs at line 427.

Surface: public API.

Signature: `pub fn project(&self, phase: Phase) -> Diagram {`

What it does: Project the diagram into another phase: node lanes are re-read in the new phase, but every artifact address is unchanged. This is the phase-invariance property made operational.

(Where these words came from: direct.)

---

## function — Diagram::to_text

Found in src/uml6d_diagram.rs at line 446.

Surface: public API.

Signature: `pub fn to_text(&self) -> String {`

What it does: Plain-text rendering: lanes grouped in phase order, edges listed.

(Where these words came from: direct.)

---

## function — Diagram::to_svg

Found in src/uml6d_diagram.rs at line 491.

Surface: public API.

Signature: `pub fn to_svg(&self) -> String {`

What it does: SVG rendering. Layout is deterministic: one vertical lane per phase present among the nodes (in Phase::all() order), nodes stacked in canonical address order, edges as curved paths under the boxes.

(Where these words came from: direct.)

---

## enum — EditOp

Found in src/undo.rs at line 33.

Surface: public API.

Signature: `pub enum EditOp {`

What it does: An edit operation that can be undone/redone.

(Where these words came from: direct.)

---

## function — EditOp::apply

Found in src/undo.rs at line 54.

Surface: public API.

Signature: `pub fn apply(&self, text: &str) -> String {`

What it does: Apply this operation to text, returning the new text.

(Where these words came from: direct.)

---

## function — EditOp::inverse

Found in src/undo.rs at line 85.

Surface: public API.

Signature: `pub fn inverse(&self) -> EditOp {`

What it does: Compute the inverse operation (for undo).

(Where these words came from: direct.)

---

## function — EditOp::to_scroll

Found in src/undo.rs at line 104.

Surface: public API.

Signature: `pub fn to_scroll(&self, cursor_before: Position, cursor_after: Position) -> String {`

What it does: Serialize to the phext scroll format.

(Where these words came from: direct.)

---

## function — EditOp::from_scroll

Found in src/undo.rs at line 130.

Surface: public API.

Signature: `pub fn from_scroll(scroll: &str) -> Option<(EditOp, Position, Position)> {`

What it does: Deserialize from a phext scroll.

(Where these words came from: direct.)

---

## struct — UndoRecord

Found in src/undo.rs at line 180.

Surface: public API.

Signature: `pub struct UndoRecord {`

What it does: An undo record: operation + cursor state.

(Where these words came from: direct.)

---

## struct — UndoEngine

Found in src/undo.rs at line 258.

Surface: public API.

Signature: `pub struct UndoEngine {`

What it does: The undo engine: manages edit history across all scrolls. Each scroll has its own independent undo/redo stack. The full history can be serialized to a work phext file.

(Where these words came from: direct.)

---

## function — UndoEngine::new

Found in src/undo.rs at line 266.

Surface: public API.

Signature: `pub fn new() -> UndoEngine {`

What it does: Function UndoEngine::new takes no inputs beyond the value it is called on. It gives back UndoEngine.

(Where these words came from: inferred.)

---

## function — UndoEngine::record

Found in src/undo.rs at line 274.

Surface: public API.

Signature: `pub fn record( &mut self, target: Coordinate, op: EditOp, cursor_before: Position, cursor_after: Position, ) {`

What it does: Record an edit operation.

(Where these words came from: direct.)

---

## function — UndoEngine::undo

Found in src/undo.rs at line 295.

Surface: public API.

Signature: `pub fn undo(&mut self, target: &Coordinate) -> Option<(EditOp, Position)> {`

What it does: Undo the last edit on a scroll. Returns the inverse operation and the cursor position to restore.

(Where these words came from: direct.)

---

## function — UndoEngine::redo

Found in src/undo.rs at line 302.

Surface: public API.

Signature: `pub fn redo(&mut self, target: &Coordinate) -> Option<(EditOp, Position)> {`

What it does: Redo the next edit on a scroll.

(Where these words came from: direct.)

---

## function — UndoEngine::can_undo

Found in src/undo.rs at line 309.

Surface: public API.

Signature: `pub fn can_undo(&self, target: &Coordinate) -> bool {`

What it does: Can we undo on this scroll?

(Where these words came from: direct.)

---

## function — UndoEngine::can_redo

Found in src/undo.rs at line 314.

Surface: public API.

Signature: `pub fn can_redo(&self, target: &Coordinate) -> bool {`

What it does: Can we redo on this scroll?

(Where these words came from: direct.)

---

## function — UndoEngine::depth

Found in src/undo.rs at line 319.

Surface: public API.

Signature: `pub fn depth(&self, target: &Coordinate) -> usize {`

What it does: Undo depth for a scroll.

(Where these words came from: direct.)

---

## function — UndoEngine::total_edits

Found in src/undo.rs at line 324.

Surface: public API.

Signature: `pub fn total_edits(&self) -> usize {`

What it does: Total edits recorded globally.

(Where these words came from: direct.)

---

## function — UndoEngine::dirty_scrolls

Found in src/undo.rs at line 329.

Surface: public API.

Signature: `pub fn dirty_scrolls(&self) -> Vec<Coordinate> {`

What it does: Which scrolls have been modified.

(Where these words came from: direct.)

---

## function — UndoEngine::to_work_phext

Found in src/undo.rs at line 343.

Surface: public API.

Signature: `pub fn to_work_phext(&self) -> Vec<u8> {`

What it does: Serialize the full undo history to a phext byte vector. Each undo record becomes a scroll in the work phext. Coordinate mapping: Z arm = target scroll's Z arm Y arm = target scroll's Y arm X arm = chapter = sequence number, section = 1, scroll = 1

(Where these words came from: direct.)

---

## function — UndoEngine::from_work_phext

Found in src/undo.rs at line 377.

Surface: public API.

Signature: `pub fn from_work_phext(bytes: &[u8], _target_index: &crate::index::LatticeIndex) -> UndoEngine {`

What it does: Load undo history from a work phext.

(Where these words came from: direct.)

---

## enum — Region

Found in src/wavefront.rs at line 23.

Surface: public API.

Signature: `pub enum Region {`

What it does: A named choice named Region. It is one of: Hearth; Shoals; OpenWater.

(Where these words came from: inferred.)

---

## function — Region::label

Found in src/wavefront.rs at line 31.

Surface: public API.

Signature: `pub fn label(self) -> &'static str {`

What it does: Display label, exactly as the phext's index words it.

(Where these words came from: direct.)

---

## struct — Placement

Found in src/wavefront.rs at line 50.

Surface: public API.

Signature: `pub struct Placement {`

What it does: A data shape named Placement. Its fields are: pub coordinate: String; pub region: Region; pub label: String.

(Where these words came from: inferred.)

---

## struct — Person

Found in src/wavefront.rs at line 59.

Surface: public API.

Signature: `pub struct Person {`

What it does: A data shape named Person. Its fields are: pub handle: String; pub name: String; pub placements: Vec<Placement>.

(Where these words came from: inferred.)

---

## function — normalize_handle

Found in src/wavefront.rs at line 118.

Surface: public API.

Signature: `pub fn normalize_handle(raw: &str) -> Option<String> {`

What it does: Reduce a user-typed handle to its lookup key: trimmed, one leading `@` dropped, lowercased, underscores kept. None when what remains is not a possible X handle — callers answer "not in the map", never a filesystem or phext access, so hostile input ends here.

(Where these words came from: direct.)

---

## function — lookup

Found in src/wavefront.rs at line 130.

Surface: public API.

Signature: `pub fn lookup(raw: &str) -> Option<Person> {`

What it does: The map's entry for a handle, however the visitor typed it.

(Where these words came from: direct.)

---

## function — snapshot_handles

Found in src/wavefront.rs at line 136.

Surface: public API.

Signature: `pub fn snapshot_handles() -> usize {`

What it does: How many distinct accounts the snapshot holds.

(Where these words came from: direct.)

---

## function — snapshot_placements

Found in src/wavefront.rs at line 141.

Surface: public API.

Signature: `pub fn snapshot_placements() -> usize {`

What it does: Total placements across all accounts (trails count twice).

(Where these words came from: direct.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/phext-lattice/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

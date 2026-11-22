# emise-tools — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 147). Scanned at commit 6b7a2b5e421ee79e1f38aeeb86406a576d404130. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Summary

Tools built by Emise (Muse, downstream Emi) on 2026-10-30, archived at William's offer. No credentials live in this repo: every helper reads its credential from Muse's secure connector storage at call time. Runtime state (watermarks, spools, logs) is excluded.

(The opening of the project's README, quoted as written.)

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 478 commits.

The author of record is **Will Bickford** — 478 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Emise — 477 commits
- Will Bickford (his own identities) — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 110 files, 11291 lines total, 2223 blank, 52 comment, 9016 code
- Python: 82 files, 19725 lines total, 1620 blank, 1113 comment, 16992 code
- Shell: 14 files, 522 lines total, 39 blank, 132 comment, 351 code

All languages: 206 files, 31538 lines total, 26359 lines of code. Functions read: 689.

Largest files:

- custom-skills/repo-importance-atlas/REGISTER.md — 1728 lines
- wavefront-film/viz10/render10.py — 1542 lines
- wavefront-film/viz9/render9.py — 1399 lines
- wavefront-film/viz9/render9_hookonly.py — 1110 lines
- wavefront-film/viz8/render8.py — 855 lines

Largest functions, by span in lines:

- render_single in wavefront-film/viz10/render10.py — about 437 lines
- render_single in wavefront-film/viz9/render9.py — about 418 lines
- render_single in wavefront-film/viz9/render9_hookonly.py — about 334 lines
- entry in wavefront-film/build_wavefront.py — about 236 lines
- render in wavefront-film/viz8/render8.py — about 215 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 11 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 3. FIXME marks: 0.
- Public items with a doc comment: 93 of 649 (14%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/emise-tools
- Branch: exo
- Head commit: 6b7a2b5e421ee79e1f38aeeb86406a576d404130
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `NoRedirect.redirect_request` (connectors/discord/discord_api.py:21) — Function NoRedirect.redirect_request takes self, *args.
- function `call` (connectors/discord/discord_api.py:24) — Function call takes method, path, body=None.
- function `load_state` (connectors/discord/discord_watch.py:29) — Function load_state takes no inputs beyond the value it is called on.
- function `save_state` (connectors/discord/discord_watch.py:35) — Function save_state takes state.
- function `spool` (connectors/discord/discord_watch.py:41) — Function spool takes event.
- function `actionable` (connectors/discord/discord_watch.py:45) — Function actionable takes msg, channel_name.
- function `log` (connectors/discord/discord_watch.py:63) — Function log takes msg.
- function `main` (connectors/discord/discord_watch.py:67) — Function main takes no inputs beyond the value it is called on.
- function `NoRedirect.redirect_request` (connectors/github/github_api.py:20) — Function NoRedirect.redirect_request takes self, *args.
- function `call` (connectors/github/github_api.py:23) — Function call takes method, path, body=None.
- function `must` (connectors/github/push_file.py:18) — Function must takes status, out, step.
- function `must` (connectors/github/push_tree.py:18) — Function must takes status, out, step.
- function `iso` (custom-skills/exocortical-epoch/scripts/ee.py:22) — Function iso takes t.
- function `main` (custom-skills/exocortical-epoch/scripts/ee.py:26) — Function main takes argv.
- function `iso` (custom-skills/exocortical-epoch/scripts/ee.sh:21) — Function iso takes no inputs beyond the value it is called on.
- function `main` (custom-skills/phext_upgrade/scripts/phext_audit.py:29) — Function main takes no inputs beyond the value it is called on.
- function `probe_fleet` (custom-skills/ranch-state-distillation/scripts/state_sampler.py:103) — One ranch fanout pass.
- function `classify` (custom-skills/ranch-state-distillation/scripts/state_sampler.py:167) — (busy?, activity-label) for one probed node.
- function `summarize` (custom-skills/ranch-state-distillation/scripts/state_sampler.py:185) — Function summarize takes results, total.
- function `coordinate_of` (custom-skills/ranch-state-distillation/scripts/state_sampler.py:225) — Function coordinate_of takes text, atlas.
- function `clean` (custom-skills/ranch-state-distillation/scripts/state_sampler.py:240) — Function clean takes text.
- function `append_entry` (custom-skills/ranch-state-distillation/scripts/state_sampler.py:309) — Append one cycle entry under the WAL discipline.
- function `read_state_scrolls` (custom-skills/ranch-state-distillation/scripts/state_sampler.py:320) — Resolve the logical view: fold the physical scrolls in write
- function `save_raw` (custom-skills/ranch-state-distillation/scripts/state_sampler.py:342) — Function save_raw takes pull, keep_bytes=2_000_000.
- function `main` (custom-skills/ranch-state-distillation/scripts/state_sampler.py:354) — Function main takes argv.
- function `main` (custom-skills/simple-html-publish/scripts/screen_test.py:20) [test] — Function main takes no inputs beyond the value it is called on.
- function `fmt` (custom-skills/singular/scripts/build_atlas.py:72) — Function fmt takes c.
- function `parse_coord` (custom-skills/singular/scripts/build_atlas.py:77) — Function parse_coord takes s.
- function `round_half_up` (custom-skills/singular/scripts/build_atlas.py:82) — Round total/n to the nearest whole step, half up, min 1.
- function `load_list` (custom-skills/singular/scripts/build_atlas.py:87) — Function load_list takes name.
- function `cyoa_votes` (custom-skills/singular/scripts/build_atlas.py:93) — word -> (component sums, scroll count), from valid CYOA scrolls.
- function `place` (custom-skills/singular/scripts/build_atlas.py:109) — (coordinate tuple, placement, scroll count) for one word.
- function `main` (custom-skills/singular/scripts/build_atlas.py:124) — Function main takes no inputs beyond the value it is called on.
- function `load_list` (custom-skills/singular/scripts/build_editor.py:33) — Function load_list takes name.
- function `main` (custom-skills/singular/scripts/build_editor.py:39) — Function main takes no inputs beyond the value it is called on.
- function `tokens` (custom-skills/singular/scripts/build_wordlist.py:50) — Function tokens takes text: str.
- function `load_js_json` (custom-skills/singular/scripts/build_wordlist.py:56) — Function load_js_json takes path: Path.
- function `main` (custom-skills/singular/scripts/build_wordlist.py:61) — Function main takes no inputs beyond the value it is called on.
- function `load_words` (custom-skills/singular/scripts/check_singular.py:30) — Function load_words takes path: Path.
- function `main` (custom-skills/singular/scripts/check_singular.py:34) — Function main takes no inputs beyond the value it is called on.
- function `fmt` (custom-skills/singular/scripts/task_coordinate.py:49) — Function fmt takes c.
- function `parse_coord` (custom-skills/singular/scripts/task_coordinate.py:54) — Function parse_coord takes s.
- function `round_half_up` (custom-skills/singular/scripts/task_coordinate.py:59) — Round total/n to the nearest whole step, half up, min 1.
- function `load_atlas` (custom-skills/singular/scripts/task_coordinate.py:64) — word -> (coordinate tuple, placement) from atlas.tsv.
- function `main` (custom-skills/singular/scripts/task_coordinate.py:80) — Function main takes argv.
- function `normalize` (custom-skills/upgoer-five/scripts/check_ug5.py:31) — Lowercase a word and drop apostrophes and other non-letters.
- function `load_words` (custom-skills/upgoer-five/scripts/check_ug5.py:36) — Function load_words takes path.
- function `stems` (custom-skills/upgoer-five/scripts/check_ug5.py:45) — Yield shorter forms of a word made by common English endings.
- function `is_allowed` (custom-skills/upgoer-five/scripts/check_ug5.py:70) — Function is_allowed takes word, allowed, strict.
- function `main` (custom-skills/upgoer-five/scripts/check_ug5.py:78) — Function main takes argv=None.
- function `fmt` (custom-skills/upgoer-five-phext/address_word.py:42) — Function fmt takes c.
- function `split_parts` (custom-skills/upgoer-five-phext/address_word.py:47) — Function split_parts takes w.
- function `wanted_tokens` (custom-skills/upgoer-five-phext/address_word.py:54) — Function wanted_tokens takes words.
- function `scan_glove` (custom-skills/upgoer-five-phext/address_word.py:64) — Function scan_glove takes wanted.
- function `embed` (custom-skills/upgoer-five-phext/address_word.py:77) — Function embed takes word, glove.
- function `normalize` (custom-skills/upgoer-five-phext/address_word.py:93) — Function normalize takes word.
- function `stems` (custom-skills/upgoer-five-phext/address_word.py:97) — Function stems takes word.
- function `is_allowed` (custom-skills/upgoer-five-phext/address_word.py:121) — Function is_allowed takes word, allowed.
- function `Atlas.__init__` (custom-skills/upgoer-five-phext/address_word.py:129) — Function Atlas.__init__ takes no inputs beyond the value it is called on.
- function `Atlas.coord_of_vec` (custom-skills/upgoer-five-phext/address_word.py:142) — Function Atlas.coord_of_vec takes self, vec.
- function `Atlas.nearest_ug5` (custom-skills/upgoer-five-phext/address_word.py:147) — Function Atlas.nearest_ug5 takes self, coord, k=5, exclude=None.
- function `load_cache` (custom-skills/upgoer-five-phext/address_word.py:161) — Function load_cache takes no inputs beyond the value it is called on.
- function `embed_words` (custom-skills/upgoer-five-phext/address_word.py:167) — Embed a list of words, using and updating the query cache.
- function `cmd_address` (custom-skills/upgoer-five-phext/address_word.py:194) — Function cmd_address takes atlas, words.
- function `cmd_decode` (custom-skills/upgoer-five-phext/address_word.py:218) — Function cmd_decode takes atlas, address.
- function `cmd_validate` (custom-skills/upgoer-five-phext/address_word.py:233) — Function cmd_validate takes atlas, path, k.
- function `main` (custom-skills/upgoer-five-phext/address_word.py:259) — Function main takes no inputs beyond the value it is called on.
- function `fmt` (custom-skills/upgoer-five-phext/build_atlas.py:48) — Function fmt takes coord.
- function `split_parts` (custom-skills/upgoer-five-phext/build_atlas.py:54) — Function split_parts takes w.
- function `wanted_tokens` (custom-skills/upgoer-five-phext/build_atlas.py:61) — Function wanted_tokens takes words.
- function `embed` (custom-skills/upgoer-five-phext/build_atlas.py:71) — GloVe vector with the contraction fallback chain, else None.
- function `write_phext` (custom-skills/upgoer-five-phext/build_atlas.py:86) — Write {coord_tuple: text} as a phext file, dead-reckoning style.
- function `main` (custom-skills/upgoer-five-phext/build_atlas.py:113) — Function main takes no inputs beyond the value it is called on.
- function `coord_of` (custom-skills/upgoer-five-phext/build_atlas.py:159) — Function coord_of takes vec.
- function `main` (custom-skills/upgoer-five-phext/build_editor.py:17) — Function main takes no inputs beyond the value it is called on.
- function `sq_status` (dashboard/collect_status.py:22) — Function sq_status takes no inputs beyond the value it is called on.
- function `sentinel` (dashboard/collect_status.py:43) — Function sentinel takes no inputs beyond the value it is called on.
- function `main` (dashboard/collect_status.py:65) — Function main takes no inputs beyond the value it is called on.
- function `get` (discord-presence/pass.py:14) — Function get takes path.
- function `must` (github-publish/publish_branch.py:17) — Function must takes status, out, step.
- function `git` (github-publish/publish_branch.py:23) — Function git takes repo_dir, *args.
- function `main` (github-publish/publish_branch.py:27) — Function main takes no inputs beyond the value it is called on.
- function `must` (github-publish/publish_incremental.py:17) — Function must takes status, out, step.
- function `git` (github-publish/publish_incremental.py:23) — Function git takes repo_dir, *args.
- function `main` (github-publish/publish_incremental.py:27) — Function main takes no inputs beyond the value it is called on.
- function `emit` (neuralese/build_neuralese.py:4) — Function emit takes targets.
- function `C` (neuralese/build_neuralese.py:16) — Function C takes ch, s=1, sc=1.
- function `vec` (neuralese/demo/run_demo.py:15) — Function vec takes g.
- function `encode_phext` (neuralese/demo/run_demo.py:17) — Function encode_phext takes units, path.
- function `phext_receive` (neuralese/demo/run_demo.py:33) — Function phext_receive takes path.
- function `main` (neuralese/demo/run_demo.py:50) — Function main takes no inputs beyond the value it is called on.
- function `rec` (neuralese/demo/run_demo.py:55) — Function rec takes chan, q, ok, cost.
- function `hops` (neuralese/demo/run_demo.py:77) — Function hops takes uid, n.
- function `vec_digest` (neuralese/neuralese_codec.py:13) — Function vec_digest takes gist: str.
- function `fmt_coord` (neuralese/neuralese_codec.py:16) — Function fmt_coord takes c.
- function `encode` (neuralese/neuralese_codec.py:18) — units: list of dicts with frame=(c,v,b), gist, links=[coord strings].
- function `decode` (neuralese/neuralese_codec.py:38) — Function decode takes phext_text, path='/tmp/_neuralese_demo.phext'.
- function `decode_all` (phext/audio-s2t/build_audio_phext.py:68) — Function decode_all takes fresh=False.
- function `load_wav` (phext/audio-s2t/build_audio_phext.py:105) — Function load_wav takes uid.
- function `silence_runs` (phext/audio-s2t/build_audio_phext.py:112) — Measured silences (start_s, end_s) with an adaptive per-file threshold:
- function `f0_window` (phext/audio-s2t/build_audio_phext.py:132) — Crude autocorrelation pitch tracker: median F0 (Hz) over voiced frames
- function `segment_utterance` (phext/audio-s2t/build_audio_phext.py:157) — Split one utterance's words at measured silences.
- function `features` (phext/audio-s2t/build_audio_phext.py:194) — Function features takes seg.
- function `emit` (phext/audio-s2t/build_audio_phext.py:222) — Function emit takes targets.
- function `fmt` (phext/audio-s2t/build_audio_phext.py:237) — Function fmt takes coord.
- function `norm_words` (phext/audio-s2t/build_audio_phext.py:242) — Function norm_words takes text.
- function `wer` (phext/audio-s2t/build_audio_phext.py:246) — Function wer takes ref, hyp.
- function `main` (phext/audio-s2t/build_audio_phext.py:258) — Function main takes no inputs beyond the value it is called on.
- function `decode` (phext/audio-s2t/decode.py:22) — Function decode takes flac_path.
- function `main` (phext/audio-s2t/decode.py:46) — Function main takes no inputs beyond the value it is called on.
- function `load` (phext/audio-s2t/probe_silence.py:10) — Function load takes path.
- function `c` (phext/build_exocortex.py:47) — Function c takes s.
- function `implode` (phext/build_exocortex.py:86) — Function implode takes scrolls.
- function `parse_coord` (phext/build_life_phext.py:51) — Function parse_coord takes s.
- function `implode` (phext/build_life_phext.py:59) — scrolls: dict coord-string -> text.
- function `sanitize` (phext/build_life_phext.py:77) — Function sanitize takes text.
- function `message_entry` (phext/build_life_phext.py:88) — Function message_entry takes m.
- function `build` (phext/build_life_phext.py:96) — Function build takes source.
- function `main` (phext/build_life_phext.py:143) — Function main takes no inputs beyond the value it is called on.
- function `c` (phext/build_skills_phext.py:148) — Function c takes s.
- function `fmt` (phext/build_skills_phext.py:152) — Function fmt takes coord.
- function `coord_for` (phext/build_skills_phext.py:156) — Function coord_for takes shelf, volume, scroll.
- function `collect_files` (phext/build_skills_phext.py:160) — Return (included, excluded) for one skill dir.
- function `implode` (phext/build_skills_phext.py:206) — scrolls: dict coord-string -> text.
- function `build` (phext/build_skills_phext.py:223) — Build the phext blob from the current source tree.
- function `sha256_file` (phext/build_skills_phext.py:312) — Function sha256_file takes path.
- function `committed_index` (phext/build_skills_phext.py:320) — Parse the per-file index out of the committed skills.phext Home scroll.
- function `check` (phext/build_skills_phext.py:341) — Rebuild to a temp file and compare SHA-256 with the committed
- function `main` (phext/build_skills_phext.py:391) — Function main takes no inputs beyond the value it is called on.
- function `parse_coord` (phext/build_tasks_phext.py:40) — Function parse_coord takes s.
- function `implode` (phext/build_tasks_phext.py:48) — scrolls: dict coord-string -> text.
- function `render_task` (phext/build_tasks_phext.py:66) — Function render_task takes t.
- function `render_home` (phext/build_tasks_phext.py:85) — Function render_home takes tasks.
- function `build` (phext/build_tasks_phext.py:110) — Function build takes no inputs beyond the value it is called on.
- function `main` (phext/build_tasks_phext.py:125) — Function main takes no inputs beyond the value it is called on.
- function `parse_coord` (phext/build_todo_phext.py:77) — Function parse_coord takes s.
- function `fmt` (phext/build_todo_phext.py:85) — Function fmt takes c.
- function `merge` (phext/build_todo_phext.py:89) — Holographic merge of a text's distinct words (Singular rule).
- function `implode` (phext/build_todo_phext.py:106) — scrolls: dict coord-string -> text.
- function `render_ranked` (phext/build_todo_phext.py:148) — The good-singularity-ranked list scroll: Collection 2, Book 2.
- function `render_register_entry` (phext/build_todo_phext.py:200) — Function render_register_entry takes e.
- function `render_protocol` (phext/build_todo_phext.py:219) — Function render_protocol takes integration.
- function `render_finding` (phext/build_todo_phext.py:274) — Function render_finding takes f.
- function `render_home` (phext/build_todo_phext.py:293) — Function render_home takes placed, n_walked, findings_sha, n_findings, integration=None.
- function `build` (phext/build_todo_phext.py:368) — Function build takes no inputs beyond the value it is called on.
- function `main` (phext/build_todo_phext.py:420) — Function main takes no inputs beyond the value it is called on.
- function `blob_of` (phext/emise-sync.sh:27) — Function blob_of takes no inputs beyond the value it is called on.
- function `coordinate_for` (phext/life_track.py:42) — Function coordinate_for takes content.
- function `fetch_all` (phext/life_track.py:57) — Return raw Discord messages newer than watermark (or all, when
- function `main` (phext/life_track.py:89) — Function main takes no inputs beyond the value it is called on.
- function `fmt` (phext/parse_phext.py:19) — Function fmt takes coord.
- function `parse` (phext/parse_phext.py:22) — Function parse takes path.
- function `title_of` (phext/parse_phext.py:45) — Function title_of takes text.
- function `load_parser` (phext/phext_dispatch.py:57) — Load parse() from parse_phext.py via importlib; fall back to inline.
- function `coord_components` (phext/phext_dispatch.py:72) — Function coord_components takes coord_str.
- function `prefix_components` (phext/phext_dispatch.py:75) — Function prefix_components takes prefix_str.
- function `select` (phext/phext_dispatch.py:81) — Function select takes scrolls, coords, prefixes, want_all.
- function `first_line` (phext/phext_dispatch.py:100) — Function first_line takes text, limit=110.
- function `sha256_of` (phext/phext_dispatch.py:107) — Function sha256_of takes path.
- function `render_scroll` (phext/phext_dispatch.py:114) — Function render_scroll takes coord, text.
- function `paginate` (phext/phext_dispatch.py:123) — Function paginate takes chosen, page_scrolls, page_chars.
- function `main` (phext/phext_dispatch.py:136) — Function main takes argv=None.
- function `Config.__init__` (phext-mailbox/phext_mailbox.py:60) — Function Config.__init__ takes self, args.
- function `fmt` (phext-mailbox/phext_mailbox.py:77) — Function fmt takes c.
- function `unfmt` (phext-mailbox/phext_mailbox.py:81) — Function unfmt takes s.
- function `locked` (phext-mailbox/phext_mailbox.py:89) — Hold the mailbox lock.
- function `load` (phext-mailbox/phext_mailbox.py:115) — Function load takes cfg.
- function `save` (phext-mailbox/phext_mailbox.py:122) — Function save takes cfg, scrolls.
- function `index_lines` (phext-mailbox/phext_mailbox.py:138) — Function index_lines takes cfg, scrolls.
- function `compact` (phext-mailbox/phext_mailbox.py:143) — Function compact takes cfg, dry.
- function `init` (phext-mailbox/phext_mailbox.py:189) — Function init takes cfg.
- function `main` (phext-mailbox/phext_mailbox.py:199) — Function main takes argv=None.
- function `fmt` (site-map/insert_scroll.py:26) — Function fmt takes c.
- function `key_of` (site-map/insert_scroll.py:29) — Function key_of takes c.
- function `parse_slots` (site-map/insert_scroll.py:32) — Return [(coord_tuple, text_start, text_end, terminator_byte_or_None)].
- function `parse_target` (site-map/insert_scroll.py:51) — Function parse_target takes s.
- function `main` (site-map/insert_scroll.py:56) — Function main takes no inputs beyond the value it is called on.
- function `sq_status` (sq-bridge/sq_bridge.py:162) — Report the bridge configuration and whether an SQ host is answering
- function `sq_open_channel` (sq-bridge/sq_bridge.py:194) — Ensure an SQ host is running that can serve `phext` from SQ_DIR, and
- function `sq_toc` (sq-bridge/sq_bridge.py:239) — Table of contents for a phext: one '* <coordinate>: <scroll text>'
- function `sq_read` (sq-bridge/sq_bridge.py:247) — Fetch one scroll's exact text by coordinate (SQ's select endpoint;
- function `sq_query` (sq-bridge/sq_bridge.py:255) — Range query: every scroll with a coordinate between from and to
- function `sq_write` (sq-bridge/sq_bridge.py:265) — Overwrite one scroll via SQ's update endpoint (auto-saves; SQ
- function `sq_close_channel` (sq-bridge/sq_bridge.py:285) — Stop an SQ host that THIS bridge started (SIGTERM to its process
- function `server` (sq-bridge/test_client.py:22) [test] — Function server takes readonly.
- function `show` (sq-bridge/test_client.py:36) [test] — Function show takes label, result.
- function `phase` (sq-bridge/test_client.py:43) [test] — Function phase takes readonly, fn.
- function `phase_a` (sq-bridge/test_client.py:50) [test] — Function phase_a takes session.
- function `phase_b` (sq-bridge/test_client.py:69) [test] — Function phase_b takes session.
- function `main` (sq-bridge/test_client.py:82) [test] — Function main takes no inputs beyond the value it is called on.
- function `show` (sq-bridge/test_client_remote.py:29) [test] — Function show takes label, result.
- function `main` (sq-bridge/test_client_remote.py:35) [test] — Function main takes no inputs beyond the value it is called on.
- function `show` (sq-bridge/tests/test_client_remote_claude.py:33) [test] — Function show takes label, result.
- function `main` (sq-bridge/tests/test_client_remote_claude.py:40) [test] — Function main takes no inputs beyond the value it is called on.
- function `sha256` (sq-bridge/tests/test_readonly_dir.py:25) [test] — Function sha256 takes path.
- function `snapshot` (sq-bridge/tests/test_readonly_dir.py:30) [test] — Function snapshot takes d.
- function `show` (sq-bridge/tests/test_readonly_dir.py:36) [test] — Function show takes label, result.
- function `main` (sq-bridge/tests/test_readonly_dir.py:43) [test] — Function main takes no inputs beyond the value it is called on.
- function `load_config` (tools/commit-rollup/rollup.py:67) — Function load_config takes no inputs beyond the value it is called on.
- function `collect` (tools/commit-rollup/rollup.py:110) — Normalized, SHA-deduped, time-sorted commit list for a window.
- function `segment_sessions` (tools/commit-rollup/rollup.py:135) — Split a sorted commit list where the quiet gap exceeds the threshold.
- function `extract_prs` (tools/commit-rollup/rollup.py:155) — Function extract_prs takes commits.
- function `effort_bar` (tools/commit-rollup/rollup.py:175) — Function effort_bar takes count, total, width.
- function `build_embed` (tools/commit-rollup/rollup.py:196) — Function build_embed takes commits, cfg.
- function `embed_text_mock` (tools/commit-rollup/rollup.py:281) — Plain-text rendering of an embed, for sample files / eyeball checks.
- function `embed_char_total` (tools/commit-rollup/rollup.py:292) — Function embed_char_total takes embed.
- function `load_state` (tools/commit-rollup/rollup.py:302) — Function load_state takes no inputs beyond the value it is called on.
- function `save_state` (tools/commit-rollup/rollup.py:309) — Function save_state takes state.
- function `init_state` (tools/commit-rollup/rollup.py:317) — Seed the watermark at now; mark the lookback window already posted.
- function `post_embed` (tools/commit-rollup/rollup.py:338) — Function post_embed takes cfg, embed.
- function `poll` (tools/commit-rollup/rollup.py:348) — Function poll takes cfg, dry_run=False, now=None.
- function `day_report` (tools/commit-rollup/rollup.py:391) — Function day_report takes cfg, day_str, out_prefix=None, repos_file=None.
- function `main` (tools/commit-rollup/rollup.py:447) — Function main takes no inputs beyond the value it is called on.
- function `df_home` (tools/disk_usage_report.py:32) — Function df_home takes no inputs beyond the value it is called on.
- function `write_probe` (tools/disk_usage_report.py:39) — Function write_probe takes no inputs beyond the value it is called on.
- function `top_dirs` (tools/disk_usage_report.py:49) — Top-level workspace dirs by size (bytes), largest first.
- function `scan_files` (tools/disk_usage_report.py:62) — One find pass over home: total file count + files >= BIG_FILE_GB.
- function `trash_size` (tools/disk_usage_report.py:82) — Function trash_size takes no inputs beyond the value it is called on.
- function `fmt` (tools/disk_usage_report.py:93) — Function fmt takes gb.
- function `main` (tools/disk_usage_report.py:97) — Function main takes no inputs beyond the value it is called on.
- function `progress_total` (tools/duty-governor/analyze_runs.py:16) — Function progress_total takes run_dir.
- function `thermal_window` (tools/duty-governor/analyze_runs.py:31) — Function thermal_window takes path, start_epoch, end_epoch.
- function `main` (tools/duty-governor/analyze_runs.py:48) — Function main takes no inputs beyond the value it is called on.
- function `iso` (tools/duty-governor/duty_governor.py:90) — Function iso takes epoch.
- function `resolve_tctl_path` (tools/duty-governor/duty_governor.py:94) — Find k10temp's temp1_input by sensor NAME (hwmon index varies).
- function `Governor.__init__` (tools/duty-governor/duty_governor.py:115) — Function Governor.__init__ takes self, cfg, cmd, log_path, max_seconds=None, tctl_file=None.
- function `Governor.log` (tools/duty-governor/duty_governor.py:146) — Function Governor.log takes self, event, tctl=None, extra="".
- function `Governor.read_tctl` (tools/duty-governor/duty_governor.py:158) — Function Governor.read_tctl takes no inputs beyond the value it is called on.
- function `Governor.group_signal` (tools/duty-governor/duty_governor.py:174) — Function Governor.group_signal takes self, sig.
- function `Governor.duty_target` (tools/duty-governor/duty_governor.py:195) — Duty fraction for the current ladder level (level 0 = 1.0).
- function `Governor.run` (tools/duty-governor/duty_governor.py:227) — Function Governor.run takes no inputs beyond the value it is called on.
- function `Governor.write_summary` (tools/duty-governor/duty_governor.py:357) — Function Governor.write_summary takes self, start, end, exit_code, timed_out.
- function `Governor.pct` (tools/duty-governor/duty_governor.py:360) — Function Governor.pct takes p.
- function `Governor.mean` (tools/duty-governor/duty_governor.py:365) — Function Governor.mean takes xs.
- function `main` (tools/duty-governor/duty_governor.py:407) — Function main takes no inputs beyond the value it is called on.
- function `on_signal` (tools/duty-governor/duty_governor.py:460) — Function on_signal takes signum, frame.
- function `sensor_path` (tools/duty-governor/duty_governor_stage2.py:52) — Function sensor_path takes no inputs beyond the value it is called on.
- function `read_tctl` (tools/duty-governor/duty_governor_stage2.py:64) — Function read_tctl takes path.
- function `duty_for_level` (tools/duty-governor/duty_governor_stage2.py:72) — Function duty_for_level takes level.
- function `main` (tools/duty-governor/duty_governor_stage2.py:76) — Function main takes no inputs beyond the value it is called on.
- function `note_state` (tools/duty-governor/duty_governor_stage2.py:117) — Function note_state takes now.
- function `set_frozen` (tools/duty-governor/duty_governor_stage2.py:126) — Function set_frozen takes want_frozen, now, why.
- function `tctl` (tools/duty-governor/duty_test.sh:16) [test] — Function tctl takes no inputs beyond the value it is called on.
- function `tctl_path` (tools/duty-governor/probe_tctl.py:24) — Function tctl_path takes no inputs beyond the value it is called on.
- function `read_c` (tools/duty-governor/probe_tctl.py:39) — Function read_c takes no inputs beyond the value it is called on.
- function `wait_cool` (tools/duty-governor/probe_tctl.py:44) — Function wait_cool takes limit, cap_s=180.
- function `sample_for` (tools/duty-governor/probe_tctl.py:53) — Function sample_for takes seconds, series, t_ref.
- function `crossings` (tools/duty-governor/probe_tctl.py:60) — Function crossings takes series, t_start, thresholds.
- function `main` (tools/duty-governor/probe_tctl.py:71) — Function main takes no inputs beyond the value it is called on.
- function `enumerate_repos` (tools/github-archive/github_archive.py:56) — Function enumerate_repos takes no inputs beyond the value it is called on.
- function `hub_ssh` (tools/github-archive/github_archive.py:80) — Function hub_ssh takes remote_cmd_argv, **kw.
- function `ship_and_start` (tools/github-archive/github_archive.py:84) — Function ship_and_start takes date, names.
- function `hub_status` (tools/github-archive/github_archive.py:110) — Function hub_status takes no inputs beyond the value it is called on.
- function `worker_running` (tools/github-archive/github_archive.py:121) — Function worker_running takes no inputs beyond the value it is called on.
- function `main` (tools/github-archive/github_archive.py:127) — Function main takes no inputs beyond the value it is called on.
- function `run` (tools/github-archive/hub_worker.py:40) — Function run takes cmd, timeout.
- function `du_bytes` (tools/github-archive/hub_worker.py:45) — Function du_bytes takes path.
- function `is_bare_mirror` (tools/github-archive/hub_worker.py:56) — Function is_bare_mirror takes path.
- function `mirror_one` (tools/github-archive/hub_worker.py:66) — Clone or update one mirror.
- function `main` (tools/github-archive/hub_worker.py:109) — Function main takes no inputs beyond the value it is called on.
- function `check` (tools/github-archive/verify_restore.py:19) — Function check takes date, name.
- function `share` (wavefront-film/build_wavefront.py:53) — Function share takes key.
- function `recip_from_share` (wavefront-film/build_wavefront.py:57) — Function recip_from_share takes s.
- function `span_days` (wavefront-film/build_wavefront.py:60) — Function span_days takes key.
- function `horizon_from_span` (wavefront-film/build_wavefront.py:64) — Function horizon_from_span takes key.
- function `region` (wavefront-film/build_wavefront.py:67) — The gated reading: desirability is gated on alignment axes, never averaged.
- function `C` (wavefront-film/build_wavefront.py:75) — Function C takes s.
- function `entry` (wavefront-film/build_wavefront.py:157) — Function entry takes name, vals, kind, moment, evidence, justifs, strength, extra="", key=None, person=None.
- function `dist` (wavefront-film/build_wavefront.py:393) — Function dist takes a, b.
- function `entry_body` (wavefront-film/build_wavefront.py:404) — Function entry_body takes e.
- function `rec` (wavefront-film/mine_contacts.py:21) — Function rec takes key.
- function `gate` (wavefront-film/viz/extract_data.py:36) — Function gate takes vals.
- function `jitter` (wavefront-film/viz/extract_data.py:43) — Function jitter takes name, amp.
- function `yavg_series` (wavefront-film/viz10/qa10.py:31) — Function yavg_series takes path.
- function `gate` (wavefront-film/viz10/qa10.py:44) — Function gate takes paths, exclude_joints=.
- function `palette` (wavefront-film/viz10/qa10.py:59) — Single decode pass, chunked (the per-frame subprocess version
- function `determinism` (wavefront-film/viz10/qa10.py:80) — Function determinism takes a, b.
- function `contact` (wavefront-film/viz10/qa10.py:88) — Function contact takes path, outp, every_s=4.
- function `node_pos_all` (wavefront-film/viz10/render10.py:123) — Function node_pos_all takes hf.
- function `smooth` (wavefront-film/viz10/render10.py:129) — Function smooth takes a, b, x.
- function `hop_float` (wavefront-film/viz10/render10.py:133) — Function hop_float takes f.
- function `node_hop` (wavefront-film/viz10/render10.py:138) — Function node_hop takes f.
- function `tone_norm` (wavefront-film/viz10/render10.py:145) — Function tone_norm takes h.
- function `gold_rgb` (wavefront-film/viz10/render10.py:148) — Function gold_rgb takes h, bright=1.0.
- function `sig_at` (wavefront-film/viz10/render10.py:175) — Function sig_at takes arr, h.
- function `spaced_width` (wavefront-film/viz10/render10.py:201) — Function spaced_width takes text, font, tracking.
- function `draw_spaced` (wavefront-film/viz10/render10.py:203) — Function draw_spaced takes dr, cx, y, text, font, fill, tracking=6.
- function `draw_spaced_left` (wavefront-film/viz10/render10.py:208) — Function draw_spaced_left takes dr, x, y, text, font, fill, tracking=4.
- function `center_text` (wavefront-film/viz10/render10.py:212) — Function center_text takes dr, cx, y, text, font, fill.
- function `drift_at` (wavefront-film/viz10/render10.py:221) — Function drift_at takes hop.
- function `boil_phase` (wavefront-film/viz10/render10.py:296) — Function boil_phase takes ff.
- function `song_e` (wavefront-film/viz10/render10.py:298) — Function song_e takes ff.
- function `catmull_rom` (wavefront-film/viz10/render10.py:312) — Function catmull_rom takes p0, p1, p2, p3, t, tension=0.0.
- function `path_at` (wavefront-film/viz10/render10.py:327) — Function path_at takes s.
- function `sanitize` (wavefront-film/viz10/render10.py:339) — Function sanitize takes text.
- function `wrap` (wavefront-film/viz10/render10.py:348) — Function wrap takes text, font, maxw, maxlines.
- function `strip_offset` (wavefront-film/viz10/render10.py:396) — Function strip_offset takes f.
- function `strip_alpha` (wavefront-film/viz10/render10.py:398) — Function strip_alpha takes f.
- function `draw_strip` (wavefront-film/viz10/render10.py:402) — Function draw_strip takes dr, f.
- function `highlight_env` (wavefront-film/viz10/render10.py:529) — Function highlight_env takes ff, rec.
- function `card_pulse` (wavefront-film/viz10/render10.py:536) — (env, arr_frame) of the strongest live featured-card arrival pulse.
- function `legend_env` (wavefront-film/viz10/render10.py:551) — Function legend_env takes ff.
- function `counter_env` (wavefront-film/viz10/render10.py:560) — Function counter_env takes ff.
- function `spline_pos_at_hop` (wavefront-film/viz10/render10.py:591) — Function spline_pos_at_hop takes h.
- function `theta` (wavefront-film/viz10/render10.py:599) — Function theta takes f.
- function `to_world` (wavefront-film/viz10/render10.py:601) — Function to_world takes pts, f, hop.
- function `camera` (wavefront-film/viz10/render10.py:621) — Function camera takes ff.
- function `project` (wavefront-film/viz10/render10.py:642) — Function project takes pts, cam.
- function `kernel` (wavefront-film/viz10/render10.py:650) — Function kernel takes n, sigma.
- function `stamp` (wavefront-film/viz10/render10.py:657) — Function stamp takes buf, sx, sy, kern, color, gain.
- function `soft_knee` (wavefront-film/viz10/render10.py:665) — v9 tone map: identity below 200, smooth tanh shoulder above, so
- function `wind_clock` (wavefront-film/viz10/render10.py:714) — Function wind_clock takes ff.
- function `fall_clock` (wavefront-film/viz10/render10.py:715) — Function fall_clock takes ff.
- function `snow_layer` (wavefront-film/viz10/render10.py:751) — The whole background weather layer for sub-frame ff: a faint
- function `node_color` (wavefront-film/viz10/render10.py:795) — Function node_color takes n.
- function `depth_att` (wavefront-film/viz10/render10.py:797) — Function depth_att takes zc.
- function `fog_mix` (wavefront-film/viz10/render10.py:799) — Function fog_mix takes col, zc.
- function `bezier_arc9` (wavefront-film/viz10/render10.py:804) — v9 arc: bow and lateral wobble driven by the landing tweet's own
- function `shell_fade` (wavefront-film/viz10/render10.py:970) — Fade-in rule: a shell condenses as its people arrive — invisible
- function `comet_state` (wavefront-film/viz10/render10.py:979) — Per comet: (tail->head base-space points, fade) at frame f.
- function `render_single` (wavefront-film/viz10/render10.py:992) — One sub-frame at continuous time ff (float frames — the wrapper
- function `film_grade` (wavefront-film/viz10/render10.py:1436) — oa: post-bloom float32 (OUT_H, OUT_W, 3), 0..255.
- function `render_frame` (wavefront-film/viz10/render10.py:1482) — One output frame: SUBN sub-frames averaged (temporal AA / motion
- function `sanitize` (wavefront-film/viz10/score_tweets.py:59) — Function sanitize takes text.
- function `list_structured` (wavefront-film/viz10/score_tweets.py:88) — Function list_structured takes raw.
- function `is_item` (wavefront-film/viz10/score_tweets.py:92) — Function is_item takes l.
- function `score` (wavefront-film/viz10/score_tweets.py:97) — Function score takes text, raw=None.
- function `main` (wavefront-film/viz10/score_tweets.py:119) — Function main takes no inputs beyond the value it is called on.
- function `load_js` (wavefront-film/viz2/extract2.py:25) — Function load_js takes path.
- function `shell_offset` (wavefront-film/viz2/extract2.py:70) — Function shell_offset takes no inputs beyond the value it is called on.
- function `smooth` (wavefront-film/viz2/render2.py:37) — Function smooth takes a, b, x.
- function `hsv2rgb` (wavefront-film/viz2/render2.py:41) — Function hsv2rgb takes h, s, v.
- function `frame_of` (wavefront-film/viz2/render2.py:48) — Function frame_of takes ts.
- function `camera` (wavefront-film/viz2/render2.py:126) — Function camera takes f.
- function `project` (wavefront-film/viz2/render2.py:147) — Function project takes pts, cam.
- function `kernel` (wavefront-film/viz2/render2.py:156) — Function kernel takes n, sigma.
- function `stamp` (wavefront-film/viz2/render2.py:162) — Function stamp takes buf, sx, sy, kern, color, gain.
- function `node_color` (wavefront-film/viz2/render2.py:173) — Function node_color takes n.
- function `center_text` (wavefront-film/viz2/render2.py:180) — Function center_text takes dr, cx, y, text, font, fill.
- function `render` (wavefront-film/viz2/render2.py:183) — Function render takes f.
- function `smooth` (wavefront-film/viz3/render3.py:56) — Function smooth takes a, b, x.
- function `hsv2rgb` (wavefront-film/viz3/render3.py:60) — Function hsv2rgb takes h, s, v.
- function `frame_of` (wavefront-film/viz3/render3.py:65) — Function frame_of takes ts.
- function `window_center` (wavefront-film/viz3/render3.py:75) — Function window_center takes f.
- function `warp` (wavefront-film/viz3/render3.py:81) — Pull points toward the drifted window: displacement = drift * falloff(dist to window).
- function `catmull_rom` (wavefront-film/viz3/render3.py:162) — Function catmull_rom takes p0, p1, p2, p3, t.
- function `spline_pos_at` (wavefront-film/viz3/render3.py:179) — Will's base-space position on the spline at film frame f.
- function `theta` (wavefront-film/viz3/render3.py:190) — Function theta takes f.
- function `to_world` (wavefront-film/viz3/render3.py:192) — base -> warped (Overton) -> rotated about CORE (galaxy).
- function `camera` (wavefront-film/viz3/render3.py:211) — Returns (pos, right, up, fwd).
- function `project` (wavefront-film/viz3/render3.py:246) — Function project takes pts, cam.
- function `kernel` (wavefront-film/viz3/render3.py:255) — Function kernel takes n, sigma.
- function `stamp` (wavefront-film/viz3/render3.py:263) — Function stamp takes buf, sx, sy, kern, color, gain.
- function `node_color` (wavefront-film/viz3/render3.py:274) — Function node_color takes n.
- function `box_edges` (wavefront-film/viz3/render3.py:286) — Function box_edges takes c, sz.
- function `center_text` (wavefront-film/viz3/render3.py:298) — Function center_text takes dr, cx, y, text, font, fill.
- function `bezier_arc` (wavefront-film/viz3/render3.py:301) — Quadratic bezier from p0 to p1 (world space), bowed off the chord.
- function `rot_only` (wavefront-film/viz3/render3.py:319) — Function rot_only takes pts, f.
- function `get_cam` (wavefront-film/viz3/render3.py:325) — Function get_cam takes f, cen.
- function `render` (wavefront-film/viz3/render3.py:332) — Function render takes f.
- function `ts_frame` (wavefront-film/viz3/render3.py:542) — Function ts_frame takes y, m, d=1.
- function `roll` (wavefront-film/viz4/extract_tone.py:36) — Function roll takes a, w.
- function `ym` (wavefront-film/viz4/extract_tone.py:42) — Function ym takes ts.
- function `tone_norm` (wavefront-film/viz4/render4.py:47) — Function tone_norm takes h.
- function `gold_rgb` (wavefront-film/viz4/render4.py:50) — Function gold_rgb takes h, bright=1.0.
- function `smooth` (wavefront-film/viz4/render4.py:65) — Function smooth takes a, b, x.
- function `hop_float` (wavefront-film/viz4/render4.py:69) — Function hop_float takes f.
- function `drift_at` (wavefront-film/viz4/render4.py:81) — Function drift_at takes hop.
- function `catmull_rom` (wavefront-film/viz4/render4.py:129) — Function catmull_rom takes p0, p1, p2, p3, t.
- function `spline_pos_at_hop` (wavefront-film/viz4/render4.py:143) — Function spline_pos_at_hop takes h.
- function `theta` (wavefront-film/viz4/render4.py:151) — Function theta takes f.
- function `to_world` (wavefront-film/viz4/render4.py:153) — Function to_world takes pts, f, hop.
- function `camera` (wavefront-film/viz4/render4.py:171) — Function camera takes f.
- function `project` (wavefront-film/viz4/render4.py:186) — Function project takes pts, cam.
- function `kernel` (wavefront-film/viz4/render4.py:194) — Function kernel takes n, sigma.
- function `stamp` (wavefront-film/viz4/render4.py:201) — Function stamp takes buf, sx, sy, kern, color, gain.
- function `noise_field` (wavefront-film/viz4/render4.py:211) — Function noise_field takes sw, sh.
- function `node_color` (wavefront-film/viz4/render4.py:231) — Function node_color takes n.
- function `bezier_arc` (wavefront-film/viz4/render4.py:235) — Function bezier_arc takes p0, p1, cam, u_max=1.0, n=10.
- function `hop_of_ts` (wavefront-film/viz4/render4.py:258) — Function hop_of_ts takes ts.
- function `center_text` (wavefront-film/viz4/render4.py:272) — Function center_text takes dr, cx, y, text, font, fill.
- function `render` (wavefront-film/viz4/render4.py:277) — Function render takes f.
- function `frame_of_hop` (wavefront-film/viz4/render4.py:438) — Function frame_of_hop takes h.
- function `tone_norm` (wavefront-film/viz4/render4_overton.py:53) — Function tone_norm takes h.
- function `gold_rgb` (wavefront-film/viz4/render4_overton.py:56) — Function gold_rgb takes h, bright=1.0.
- function `smooth` (wavefront-film/viz4/render4_overton.py:71) — Function smooth takes a, b, x.
- function `hop_float` (wavefront-film/viz4/render4_overton.py:75) — Function hop_float takes f.
- function `build_annual` (wavefront-film/viz4/render4_overton.py:83) — Function build_annual takes stab_ts_arr.
- function `drift_at` (wavefront-film/viz4/render4_overton.py:103) — Window center at a hop: c(y-1) gliding to c(y) over the first EASE_HOPS hops of year y.
- function `catmull_rom` (wavefront-film/viz4/render4_overton.py:156) — Function catmull_rom takes p0, p1, p2, p3, t.
- function `spline_pos_at_hop` (wavefront-film/viz4/render4_overton.py:170) — Function spline_pos_at_hop takes h.
- function `theta` (wavefront-film/viz4/render4_overton.py:178) — Function theta takes f.
- function `to_world` (wavefront-film/viz4/render4_overton.py:180) — Function to_world takes pts, f, hop.
- function `flat_world` (wavefront-film/viz4/render4_overton.py:192) — Rotation + breathing about CORE only (goalposts/ring ride the galaxy, not the warp).
- function `box_edges` (wavefront-film/viz4/render4_overton.py:205) — Function box_edges takes c, sz.
- function `camera` (wavefront-film/viz4/render4_overton.py:217) — Function camera takes f.
- function `project` (wavefront-film/viz4/render4_overton.py:232) — Function project takes pts, cam.
- function `kernel` (wavefront-film/viz4/render4_overton.py:240) — Function kernel takes n, sigma.
- function `stamp` (wavefront-film/viz4/render4_overton.py:247) — Function stamp takes buf, sx, sy, kern, color, gain.
- function `noise_field` (wavefront-film/viz4/render4_overton.py:257) — Function noise_field takes sw, sh.
- function `node_color` (wavefront-film/viz4/render4_overton.py:277) — Function node_color takes n.
- function `bezier_arc` (wavefront-film/viz4/render4_overton.py:281) — Function bezier_arc takes p0, p1, cam, u_max=1.0, n=10.
- function `hop_of_ts` (wavefront-film/viz4/render4_overton.py:304) — Function hop_of_ts takes ts.
- function `center_text` (wavefront-film/viz4/render4_overton.py:318) — Function center_text takes dr, cx, y, text, font, fill.
- function `render` (wavefront-film/viz4/render4_overton.py:323) — Function render takes f.
- function `frame_of_hop` (wavefront-film/viz4/render4_overton.py:501) — Function frame_of_hop takes h.
- function `deep_frac` (wavefront-film/viz5/render5.py:69) — Function deep_frac takes hf.
- function `node_pos_all` (wavefront-film/viz5/render5.py:93) — All node positions at (float) hop hf: anchor + epicycle offset.
- function `node_hop` (wavefront-film/viz5/render5.py:99) — Hop used for node orbits: follows hop_float, then keeps advancing at
- function `tone_norm` (wavefront-film/viz5/render5.py:108) — Function tone_norm takes h.
- function `gold_rgb` (wavefront-film/viz5/render5.py:111) — Function gold_rgb takes h, bright=1.0.
- function `smooth` (wavefront-film/viz5/render5.py:126) — Function smooth takes a, b, x.
- function `hop_float` (wavefront-film/viz5/render5.py:130) — Function hop_float takes f.
- function `drift_at` (wavefront-film/viz5/render5.py:142) — Function drift_at takes hop.
- function `catmull_rom` (wavefront-film/viz5/render5.py:191) — Function catmull_rom takes p0, p1, p2, p3, t.
- function `spline_pos_at_hop` (wavefront-film/viz5/render5.py:205) — Function spline_pos_at_hop takes h.
- function `theta` (wavefront-film/viz5/render5.py:213) — Function theta takes f.
- function `to_world` (wavefront-film/viz5/render5.py:217) — Function to_world takes pts, f, hop.
- function `camera` (wavefront-film/viz5/render5.py:235) — Function camera takes f.
- function `project` (wavefront-film/viz5/render5.py:250) — Function project takes pts, cam.
- function `kernel` (wavefront-film/viz5/render5.py:258) — Function kernel takes n, sigma.
- function `stamp` (wavefront-film/viz5/render5.py:265) — Function stamp takes buf, sx, sy, kern, color, gain.
- function `noise_field` (wavefront-film/viz5/render5.py:275) — Function noise_field takes sw, sh.
- function `node_color` (wavefront-film/viz5/render5.py:295) — Function node_color takes n.
- function `bezier_arc` (wavefront-film/viz5/render5.py:299) — Function bezier_arc takes p0, p1, cam, u_max=1.0, n=10.
- function `hop_of_ts` (wavefront-film/viz5/render5.py:322) — Function hop_of_ts takes ts.
- function `center_text` (wavefront-film/viz5/render5.py:336) — Function center_text takes dr, cx, y, text, font, fill.
- function `render` (wavefront-film/viz5/render5.py:341) — Function render takes f.
- function `frame_of_hop` (wavefront-film/viz5/render5.py:505) — Function frame_of_hop takes h.
- function `node_pos_all` (wavefront-film/viz6/render6.py:87) — Function node_pos_all takes hf.
- function `smooth` (wavefront-film/viz6/render6.py:93) — Function smooth takes a, b, x.
- function `hop_float` (wavefront-film/viz6/render6.py:97) — Function hop_float takes f.
- function `node_hop` (wavefront-film/viz6/render6.py:102) — Function node_hop takes f.
- function `tone_norm` (wavefront-film/viz6/render6.py:109) — Function tone_norm takes h.
- function `gold_rgb` (wavefront-film/viz6/render6.py:112) — Function gold_rgb takes h, bright=1.0.
- function `spaced_width` (wavefront-film/viz6/render6.py:130) — Function spaced_width takes text, font, tracking.
- function `draw_spaced` (wavefront-film/viz6/render6.py:132) — Function draw_spaced takes dr, cx, y, text, font, fill, tracking=6.
- function `draw_spaced_left` (wavefront-film/viz6/render6.py:137) — Function draw_spaced_left takes dr, x, y, text, font, fill, tracking=4.
- function `center_text` (wavefront-film/viz6/render6.py:141) — Function center_text takes dr, cx, y, text, font, fill.
- function `drift_at` (wavefront-film/viz6/render6.py:150) — Function drift_at takes hop.
- function `catmull_rom` (wavefront-film/viz6/render6.py:218) — Function catmull_rom takes p0, p1, p2, p3, t.
- function `path_at` (wavefront-film/viz6/render6.py:232) — Function path_at takes s.
- function `sanitize` (wavefront-film/viz6/render6.py:244) — Function sanitize takes text.
- function `wrap` (wavefront-film/viz6/render6.py:249) — Function wrap takes text, font, maxw, maxlines.
- function `spline_pos_at_hop` (wavefront-film/viz6/render6.py:343) — Function spline_pos_at_hop takes h.
- function `theta` (wavefront-film/viz6/render6.py:351) — Function theta takes f.
- function `to_world` (wavefront-film/viz6/render6.py:353) — Function to_world takes pts, f, hop.
- function `camera` (wavefront-film/viz6/render6.py:371) — Function camera takes f.
- function `project` (wavefront-film/viz6/render6.py:387) — Function project takes pts, cam.
- function `kernel` (wavefront-film/viz6/render6.py:395) — Function kernel takes n, sigma.
- function `stamp` (wavefront-film/viz6/render6.py:402) — Function stamp takes buf, sx, sy, kern, color, gain.
- function `noise_field` (wavefront-film/viz6/render6.py:412) — Function noise_field takes sw, sh.
- function `node_color` (wavefront-film/viz6/render6.py:431) — Function node_color takes n.
- function `bezier_arc` (wavefront-film/viz6/render6.py:435) — Function bezier_arc takes p0, p1, cam, u_max=1.0, n=10.
- function `comet_state` (wavefront-film/viz6/render6.py:456) — Per comet: (tail->head base-space points, fade) at frame f.
- function `card_active` (wavefront-film/viz6/render6.py:469) — Function card_active takes f.
- function `render` (wavefront-film/viz6/render6.py:472) — Function render takes f.
- function `frame_of_hop` (wavefront-film/viz6/render6.py:659) — Function frame_of_hop takes h.
- function `node_pos_all` (wavefront-film/viz7/render7.py:95) — Function node_pos_all takes hf.
- function `smooth` (wavefront-film/viz7/render7.py:101) — Function smooth takes a, b, x.
- function `hop_float` (wavefront-film/viz7/render7.py:105) — Function hop_float takes f.
- function `node_hop` (wavefront-film/viz7/render7.py:110) — Function node_hop takes f.
- function `tone_norm` (wavefront-film/viz7/render7.py:117) — Function tone_norm takes h.
- function `gold_rgb` (wavefront-film/viz7/render7.py:120) — Function gold_rgb takes h, bright=1.0.
- function `spaced_width` (wavefront-film/viz7/render7.py:140) — Function spaced_width takes text, font, tracking.
- function `draw_spaced` (wavefront-film/viz7/render7.py:142) — Function draw_spaced takes dr, cx, y, text, font, fill, tracking=6.
- function `draw_spaced_left` (wavefront-film/viz7/render7.py:147) — Function draw_spaced_left takes dr, x, y, text, font, fill, tracking=4.
- function `center_text` (wavefront-film/viz7/render7.py:151) — Function center_text takes dr, cx, y, text, font, fill.
- function `drift_at` (wavefront-film/viz7/render7.py:160) — Function drift_at takes hop.
- function `catmull_rom` (wavefront-film/viz7/render7.py:228) — Function catmull_rom takes p0, p1, p2, p3, t, tension=0.0.
- function `path_at` (wavefront-film/viz7/render7.py:246) — Function path_at takes s.
- function `sanitize` (wavefront-film/viz7/render7.py:258) — Function sanitize takes text.
- function `wrap` (wavefront-film/viz7/render7.py:263) — Function wrap takes text, font, maxw, maxlines.
- function `strip_offset` (wavefront-film/viz7/render7.py:309) — Function strip_offset takes f.
- function `strip_alpha` (wavefront-film/viz7/render7.py:314) — Function strip_alpha takes f.
- function `draw_strip` (wavefront-film/viz7/render7.py:316) — Function draw_strip takes dr, f.
- function `spline_pos_at_hop` (wavefront-film/viz7/render7.py:427) — Function spline_pos_at_hop takes h.
- function `theta` (wavefront-film/viz7/render7.py:435) — Function theta takes f.
- function `to_world` (wavefront-film/viz7/render7.py:437) — Function to_world takes pts, f, hop.
- function `camera` (wavefront-film/viz7/render7.py:455) — Function camera takes f.
- function `project` (wavefront-film/viz7/render7.py:471) — Function project takes pts, cam.
- function `kernel` (wavefront-film/viz7/render7.py:479) — Function kernel takes n, sigma.
- function `stamp` (wavefront-film/viz7/render7.py:486) — Function stamp takes buf, sx, sy, kern, color, gain.
- function `noise_field` (wavefront-film/viz7/render7.py:498) — Function noise_field takes sw, sh.
- function `node_color` (wavefront-film/viz7/render7.py:517) — Function node_color takes n.
- function `bezier_arc` (wavefront-film/viz7/render7.py:520) — Function bezier_arc takes p0, p1, cam, u_max=1.0, n=18.
- function `comet_state` (wavefront-film/viz7/render7.py:540) — Per comet: (tail->head base-space points, fade) at frame f.
- function `render` (wavefront-film/viz7/render7.py:553) — Function render takes f.
- function `frame_of_hop` (wavefront-film/viz7/render7.py:722) — Function frame_of_hop takes h.
- function `tokens_of` (wavefront-film/viz8/extract_features.py:93) — Function tokens_of takes text.
- function `pct` (wavefront-film/viz8/extract_features.py:128) — Function pct takes a, ps=.
- function `norm` (wavefront-film/viz8/extract_features.py:131) — Function norm takes a.
- function `smooth2` (wavefront-film/viz8/extract_features.py:140) — Function smooth2 takes a, w1=201, w2=51.
- function `roll` (wavefront-film/viz8/extract_features.py:141) — Function roll takes x, w.
- function `node_pos_all` (wavefront-film/viz8/render8.py:86) — Function node_pos_all takes hf.
- function `smooth` (wavefront-film/viz8/render8.py:92) — Function smooth takes a, b, x.
- function `hop_float` (wavefront-film/viz8/render8.py:96) — Function hop_float takes f.
- function `node_hop` (wavefront-film/viz8/render8.py:101) — Function node_hop takes f.
- function `tone_norm` (wavefront-film/viz8/render8.py:108) — Function tone_norm takes h.
- function `gold_rgb` (wavefront-film/viz8/render8.py:111) — Function gold_rgb takes h, bright=1.0.
- function `sig_at` (wavefront-film/viz8/render8.py:133) — Function sig_at takes arr, h.
- function `spaced_width` (wavefront-film/viz8/render8.py:159) — Function spaced_width takes text, font, tracking.
- function `draw_spaced` (wavefront-film/viz8/render8.py:161) — Function draw_spaced takes dr, cx, y, text, font, fill, tracking=6.
- function `draw_spaced_left` (wavefront-film/viz8/render8.py:166) — Function draw_spaced_left takes dr, x, y, text, font, fill, tracking=4.
- function `center_text` (wavefront-film/viz8/render8.py:170) — Function center_text takes dr, cx, y, text, font, fill.
- function `drift_at` (wavefront-film/viz8/render8.py:179) — Function drift_at takes hop.
- function `catmull_rom` (wavefront-film/viz8/render8.py:247) — Function catmull_rom takes p0, p1, p2, p3, t, tension=0.0.
- function `path_at` (wavefront-film/viz8/render8.py:265) — Function path_at takes s.
- function `sanitize` (wavefront-film/viz8/render8.py:277) — Function sanitize takes text.
- function `wrap` (wavefront-film/viz8/render8.py:282) — Function wrap takes text, font, maxw, maxlines.
- function `strip_offset` (wavefront-film/viz8/render8.py:328) — Function strip_offset takes f.
- function `strip_alpha` (wavefront-film/viz8/render8.py:333) — Function strip_alpha takes f.
- function `draw_strip` (wavefront-film/viz8/render8.py:335) — Function draw_strip takes dr, f.
- function `spline_pos_at_hop` (wavefront-film/viz8/render8.py:446) — Function spline_pos_at_hop takes h.
- function `theta` (wavefront-film/viz8/render8.py:454) — Function theta takes f.
- function `to_world` (wavefront-film/viz8/render8.py:456) — Function to_world takes pts, f, hop.
- function `camera` (wavefront-film/viz8/render8.py:474) — Function camera takes f.
- function `project` (wavefront-film/viz8/render8.py:490) — Function project takes pts, cam.
- function `kernel` (wavefront-film/viz8/render8.py:498) — Function kernel takes n, sigma.
- function `stamp` (wavefront-film/viz8/render8.py:505) — Function stamp takes buf, sx, sy, kern, color, gain.
- function `noise_field` (wavefront-film/viz8/render8.py:517) — Function noise_field takes sw, sh.
- function `node_color` (wavefront-film/viz8/render8.py:544) — Function node_color takes n.
- function `bezier_arc` (wavefront-film/viz8/render8.py:547) — Function bezier_arc takes p0, p1, cam, u_max=1.0, n=18.
- function `bezier_arc8` (wavefront-film/viz8/render8.py:556) — v8 arc: bow and lateral wobble driven by the landing tweet's own
- function `comet_state` (wavefront-film/viz8/render8.py:587) — Per comet: (tail->head base-space points, fade) at frame f.
- function `render` (wavefront-film/viz8/render8.py:600) — Function render takes f.
- function `frame_of_hop` (wavefront-film/viz8/render8.py:815) — Function frame_of_hop takes h.
- function `run` (wavefront-film/viz9/assemble_reel.py:27) — Function run takes cmd.
- function `load_js` (wavefront-film/viz9/derive_clusters.py:40) — Function load_js takes path.
- function `dbscan` (wavefront-film/viz9/derive_clusters.py:92) — Function dbscan takes eps, ms.
- function `components` (wavefront-film/viz9/derive_clusters.py:119) — Function components takes tau.
- function `find` (wavefront-film/viz9/derive_clusters.py:121) — Function find takes a.
- function `pearson` (wavefront-film/viz9/extract_features9.py:54) — Function pearson takes a, b.
- function `spearman` (wavefront-film/viz9/extract_features9.py:55) — Function spearman takes a, b.
- function `norm` (wavefront-film/viz9/extract_features9.py:77) — Function norm takes a.
- function `smooth2` (wavefront-film/viz9/extract_features9.py:81) — Function smooth2 takes a, w1=201, w2=51.
- function `roll` (wavefront-film/viz9/extract_features9.py:82) — Function roll takes x, w.
- function `run` (wavefront-film/viz9/fix_reel_audio.py:25) — Function run takes cmd, **kw.
- function `yavg_series` (wavefront-film/viz9/qa9.py:29) — Function yavg_series takes path.
- function `gate` (wavefront-film/viz9/qa9.py:42) — Function gate takes paths, exclude_joints=.
- function `palette` (wavefront-film/viz9/qa9.py:57) — Single decode pass, chunked (the per-frame subprocess version
- function `determinism` (wavefront-film/viz9/qa9.py:78) — Function determinism takes a, b.
- function `contact` (wavefront-film/viz9/qa9.py:86) — Function contact takes path, outp, every_s=4.
- function `node_pos_all` (wavefront-film/viz9/render9.py:119) — Function node_pos_all takes hf.
- function `smooth` (wavefront-film/viz9/render9.py:125) — Function smooth takes a, b, x.
- function `hop_float` (wavefront-film/viz9/render9.py:129) — Function hop_float takes f.
- function `node_hop` (wavefront-film/viz9/render9.py:134) — Function node_hop takes f.
- function `tone_norm` (wavefront-film/viz9/render9.py:141) — Function tone_norm takes h.
- function `gold_rgb` (wavefront-film/viz9/render9.py:144) — Function gold_rgb takes h, bright=1.0.
- function `sig_at` (wavefront-film/viz9/render9.py:171) — Function sig_at takes arr, h.
- function `spaced_width` (wavefront-film/viz9/render9.py:197) — Function spaced_width takes text, font, tracking.
- function `draw_spaced` (wavefront-film/viz9/render9.py:199) — Function draw_spaced takes dr, cx, y, text, font, fill, tracking=6.
- function `draw_spaced_left` (wavefront-film/viz9/render9.py:204) — Function draw_spaced_left takes dr, x, y, text, font, fill, tracking=4.
- function `center_text` (wavefront-film/viz9/render9.py:208) — Function center_text takes dr, cx, y, text, font, fill.
- function `drift_at` (wavefront-film/viz9/render9.py:217) — Function drift_at takes hop.
- function `boil_phase` (wavefront-film/viz9/render9.py:292) — Function boil_phase takes ff.
- function `song_e` (wavefront-film/viz9/render9.py:294) — Function song_e takes ff.
- function `catmull_rom` (wavefront-film/viz9/render9.py:308) — Function catmull_rom takes p0, p1, p2, p3, t, tension=0.0.
- function `path_at` (wavefront-film/viz9/render9.py:323) — Function path_at takes s.
- function `sanitize` (wavefront-film/viz9/render9.py:335) — Function sanitize takes text.
- function `wrap` (wavefront-film/viz9/render9.py:340) — Function wrap takes text, font, maxw, maxlines.
- function `strip_offset` (wavefront-film/viz9/render9.py:388) — Function strip_offset takes f.
- function `strip_alpha` (wavefront-film/viz9/render9.py:390) — Function strip_alpha takes f.
- function `draw_strip` (wavefront-film/viz9/render9.py:394) — Function draw_strip takes dr, f.
- function `highlight_env` (wavefront-film/viz9/render9.py:521) — Function highlight_env takes ff, rec.
- function `card_pulse` (wavefront-film/viz9/render9.py:525) — (env, arr_frame) of the strongest live Most-Echoed arrival pulse.
- function `legend_env` (wavefront-film/viz9/render9.py:540) — Function legend_env takes ff.
- function `counter_env` (wavefront-film/viz9/render9.py:549) — Function counter_env takes ff.
- function `spline_pos_at_hop` (wavefront-film/viz9/render9.py:580) — Function spline_pos_at_hop takes h.
- function `theta` (wavefront-film/viz9/render9.py:588) — Function theta takes f.
- function `to_world` (wavefront-film/viz9/render9.py:590) — Function to_world takes pts, f, hop.
- function `camera` (wavefront-film/viz9/render9.py:610) — Function camera takes ff.
- function `project` (wavefront-film/viz9/render9.py:626) — Function project takes pts, cam.
- function `kernel` (wavefront-film/viz9/render9.py:634) — Function kernel takes n, sigma.
- function `stamp` (wavefront-film/viz9/render9.py:641) — Function stamp takes buf, sx, sy, kern, color, gain.
- function `soft_knee` (wavefront-film/viz9/render9.py:649) — v9 tone map: identity below 200, smooth tanh shoulder above, so
- function `wind_clock` (wavefront-film/viz9/render9.py:681) — Function wind_clock takes ff.
- function `fall_clock` (wavefront-film/viz9/render9.py:682) — Function fall_clock takes ff.
- function `snow_layer` (wavefront-film/viz9/render9.py:718) — The whole background weather layer for sub-frame ff: a faint
- function `node_color` (wavefront-film/viz9/render9.py:762) — Function node_color takes n.
- function `depth_att` (wavefront-film/viz9/render9.py:764) — Function depth_att takes zc.
- function `fog_mix` (wavefront-film/viz9/render9.py:766) — Function fog_mix takes col, zc.
- function `bezier_arc9` (wavefront-film/viz9/render9.py:771) — v9 arc: bow and lateral wobble driven by the landing tweet's own
- function `shell_fade` (wavefront-film/viz9/render9.py:861) — Fade-in rule: a shell condenses as its people arrive — invisible
- function `comet_state` (wavefront-film/viz9/render9.py:870) — Per comet: (tail->head base-space points, fade) at frame f.
- function `render_single` (wavefront-film/viz9/render9.py:883) — One sub-frame at continuous time ff (float frames — the wrapper
- function `film_grade` (wavefront-film/viz9/render9.py:1308) — oa: post-bloom float32 (OUT_H, OUT_W, 3), 0..255.
- function `render_frame` (wavefront-film/viz9/render9.py:1339) — One output frame: SUBN sub-frames averaged (temporal AA / motion
- function `node_pos_all` (wavefront-film/viz9/render9_hookonly.py:93) — Function node_pos_all takes hf.
- function `smooth` (wavefront-film/viz9/render9_hookonly.py:99) — Function smooth takes a, b, x.
- function `hop_float` (wavefront-film/viz9/render9_hookonly.py:103) — Function hop_float takes f.
- function `node_hop` (wavefront-film/viz9/render9_hookonly.py:108) — Function node_hop takes f.
- function `tone_norm` (wavefront-film/viz9/render9_hookonly.py:115) — Function tone_norm takes h.
- function `gold_rgb` (wavefront-film/viz9/render9_hookonly.py:118) — Function gold_rgb takes h, bright=1.0.
- function `sig_at` (wavefront-film/viz9/render9_hookonly.py:145) — Function sig_at takes arr, h.
- function `spaced_width` (wavefront-film/viz9/render9_hookonly.py:170) — Function spaced_width takes text, font, tracking.
- function `draw_spaced` (wavefront-film/viz9/render9_hookonly.py:172) — Function draw_spaced takes dr, cx, y, text, font, fill, tracking=6.
- function `draw_spaced_left` (wavefront-film/viz9/render9_hookonly.py:177) — Function draw_spaced_left takes dr, x, y, text, font, fill, tracking=4.
- function `center_text` (wavefront-film/viz9/render9_hookonly.py:181) — Function center_text takes dr, cx, y, text, font, fill.
- function `drift_at` (wavefront-film/viz9/render9_hookonly.py:190) — Function drift_at takes hop.
- function `boil_phase` (wavefront-film/viz9/render9_hookonly.py:265) — Function boil_phase takes ff.
- function `song_e` (wavefront-film/viz9/render9_hookonly.py:267) — Function song_e takes ff.
- function `catmull_rom` (wavefront-film/viz9/render9_hookonly.py:281) — Function catmull_rom takes p0, p1, p2, p3, t, tension=0.0.
- function `path_at` (wavefront-film/viz9/render9_hookonly.py:296) — Function path_at takes s.
- function `sanitize` (wavefront-film/viz9/render9_hookonly.py:308) — Function sanitize takes text.
- function `wrap` (wavefront-film/viz9/render9_hookonly.py:313) — Function wrap takes text, font, maxw, maxlines.
- function `strip_offset` (wavefront-film/viz9/render9_hookonly.py:361) — Function strip_offset takes f.
- function `strip_alpha` (wavefront-film/viz9/render9_hookonly.py:363) — Function strip_alpha takes f.
- function `draw_strip` (wavefront-film/viz9/render9_hookonly.py:367) — Function draw_strip takes dr, f.
- function `highlight_env` (wavefront-film/viz9/render9_hookonly.py:494) — Function highlight_env takes ff, rec.
- function `card_pulse` (wavefront-film/viz9/render9_hookonly.py:498) — (env, arr_frame) of the strongest live Most-Echoed arrival pulse.
- function `legend_env` (wavefront-film/viz9/render9_hookonly.py:513) — Function legend_env takes ff.
- function `spline_pos_at_hop` (wavefront-film/viz9/render9_hookonly.py:544) — Function spline_pos_at_hop takes h.
- function `theta` (wavefront-film/viz9/render9_hookonly.py:552) — Function theta takes f.
- function `to_world` (wavefront-film/viz9/render9_hookonly.py:554) — Function to_world takes pts, f, hop.
- function `camera` (wavefront-film/viz9/render9_hookonly.py:574) — Function camera takes ff.
- function `project` (wavefront-film/viz9/render9_hookonly.py:590) — Function project takes pts, cam.
- function `kernel` (wavefront-film/viz9/render9_hookonly.py:598) — Function kernel takes n, sigma.
- function `stamp` (wavefront-film/viz9/render9_hookonly.py:605) — Function stamp takes buf, sx, sy, kern, color, gain.
- function `soft_knee` (wavefront-film/viz9/render9_hookonly.py:613) — v9 tone map: identity below 200, smooth tanh shoulder above, so
- function `noise_field` (wavefront-film/viz9/render9_hookonly.py:623) — Function noise_field takes sw, sh.
- function `node_color` (wavefront-film/viz9/render9_hookonly.py:663) — Function node_color takes n.
- function `depth_att` (wavefront-film/viz9/render9_hookonly.py:665) — Function depth_att takes zc.
- function `fog_mix` (wavefront-film/viz9/render9_hookonly.py:667) — Function fog_mix takes col, zc.
- function `bezier_arc9` (wavefront-film/viz9/render9_hookonly.py:672) — v9 arc: bow and lateral wobble driven by the landing tweet's own
- function `comet_state` (wavefront-film/viz9/render9_hookonly.py:704) — Per comet: (tail->head base-space points, fade) at frame f.
- function `render_single` (wavefront-film/viz9/render9_hookonly.py:717) — One sub-frame at continuous time ff (float frames — the wrapper
- function `render_frame` (wavefront-film/viz9/render9_hookonly.py:1051) — One output frame: SUBN sub-frames averaged (temporal AA / motion
- function `yavg` (wavefront-film/viz9/repro_flash.py:12) — Function yavg takes im.
- function `head_proj` (wavefront-film/viz9/repro_flash.py:15) — Function head_proj takes f.
- function `painted_fraction` (wavefront-film/viz9/repro_flash.py:24) — Apply render8's exact head-block slice expression to a zero buffer
- function `fmt` (wavefront-kit/build_your_wavefront.py:98) — Function fmt takes coord.
- function `C` (wavefront-kit/build_your_wavefront.py:101) — Function C takes s.
- function `parse_phext` (wavefront-kit/build_your_wavefront.py:106) — Function parse_phext takes data.
- function `assemble` (wavefront-kit/build_your_wavefront.py:124) — Function assemble takes scrolls.
- function `load_lens` (wavefront-kit/build_your_wavefront.py:138) — Merge an owner's lens file over the default lens, field by field.
- function `recip_from_share` (wavefront-kit/build_your_wavefront.py:164) — Function recip_from_share takes s.
- function `main` (wavefront-kit/build_your_wavefront.py:167) — Function main takes no inputs beyond the value it is called on.
- function `region` (wavefront-kit/build_your_wavefront.py:183) — Function region takes vals.
- function `detected_vals` (wavefront-kit/build_your_wavefront.py:256) — Function detected_vals takes c.
- function `dist` (wavefront-kit/build_your_wavefront.py:296) — Function dist takes a, b.
- function `entry_body` (wavefront-kit/build_your_wavefront.py:306) — Function entry_body takes e.
- function `load_export_file` (wavefront-kit/mine_your_archive.py:49) — Parse a .js export file (strip the window.YTD.* wrapper) or plain JSON.
- function `part_number` (wavefront-kit/mine_your_archive.py:57) — Function part_number takes name.
- function `find_tweet_files` (wavefront-kit/mine_your_archive.py:62) — Resolve the archive argument to an ordered list of tweet files.
- function `detect_handle` (wavefront-kit/mine_your_archive.py:78) — Best-effort owner handle from data/account.js (username field only).
- function `parse_date` (wavefront-kit/mine_your_archive.py:98) — Function parse_date takes raw.
- function `main` (wavefront-kit/mine_your_archive.py:108) — Function main takes no inputs beyond the value it is called on.
- function `rec` (wavefront-kit/mine_your_archive.py:130) — Function rec takes key.

---

## function — NoRedirect.redirect_request

Found in connectors/discord/discord_api.py at line 21.

Surface: public API.

Signature: `def redirect_request(self, *args)`

What it does: Function NoRedirect.redirect_request takes self, *args. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — call

Found in connectors/discord/discord_api.py at line 24.

Surface: public API.

Signature: `def call(method, path, body=None)`

What it does: Function call takes method, path, body=None. Its return type is unannotated; the code shows it returning resp.status, read_json_response(resp); e.code, {"error": detail}.

(Where these words came from: inferred.)

---

## function — load_state

Found in connectors/discord/discord_watch.py at line 29.

Surface: public API.

Signature: `def load_state()`

What it does: Function load_state takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning json.load(open(STATE)); {"channels": {}}.

(Where these words came from: inferred.)

---

## function — save_state

Found in connectors/discord/discord_watch.py at line 35.

Surface: public API.

Signature: `def save_state(state)`

What it does: Function save_state takes state. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — spool

Found in connectors/discord/discord_watch.py at line 41.

Surface: public API.

Signature: `def spool(event)`

What it does: Function spool takes event. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — actionable

Found in connectors/discord/discord_watch.py at line 45.

Surface: public API.

Signature: `def actionable(msg, channel_name)`

What it does: Function actionable takes msg, channel_name. Its return type is unannotated; the code shows it returning True; False.

(Where these words came from: inferred.)

---

## function — log

Found in connectors/discord/discord_watch.py at line 63.

Surface: public API.

Signature: `def log(msg)`

What it does: Function log takes msg. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in connectors/discord/discord_watch.py at line 67.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — NoRedirect.redirect_request

Found in connectors/github/github_api.py at line 20.

Surface: public API.

Signature: `def redirect_request(self, *args)`

What it does: Function NoRedirect.redirect_request takes self, *args. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — call

Found in connectors/github/github_api.py at line 23.

Surface: public API.

Signature: `def call(method, path, body=None)`

What it does: Function call takes method, path, body=None. Its return type is unannotated; the code shows it returning resp.status, read_json_response(resp); e.code, {"error": detail}.

(Where these words came from: inferred.)

---

## function — must

Found in connectors/github/push_file.py at line 18.

Surface: public API.

Signature: `def must(status, out, step)`

What it does: Function must takes status, out, step. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — must

Found in connectors/github/push_tree.py at line 18.

Surface: public API.

Signature: `def must(status, out, step)`

What it does: Function must takes status, out, step. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — iso

Found in custom-skills/exocortical-epoch/scripts/ee.py at line 22.

Surface: public API.

Signature: `def iso(t)`

What it does: Function iso takes t. Its return type is unannotated; the code shows it returning datetime.fromtimestamp(t, tz=timezone.utc).strft.

(Where these words came from: inferred.)

---

## function — main

Found in custom-skills/exocortical-epoch/scripts/ee.py at line 26.

Surface: public API.

Signature: `def main(argv)`

What it does: Function main takes argv. Its return type is unannotated; the code shows it returning 2; 0.

(Where these words came from: inferred.)

---

## function — iso

Found in custom-skills/exocortical-epoch/scripts/ee.sh at line 21.

Surface: public API.

Signature: `iso()`

What it does: Function iso takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — main

Found in custom-skills/phext_upgrade/scripts/phext_audit.py at line 29.

Surface: public API.

Signature: `def main() -> int`

What it does: Function main takes no inputs beyond the value it is called on. It gives back int.

(Where these words came from: inferred.)

---

## function — probe_fleet

Found in custom-skills/ranch-state-distillation/scripts/state_sampler.py at line 103.

Surface: public API.

Signature: `def probe_fleet()`

What it does: One ranch fanout pass. Returns {node_name: result-dict} for every

(Where these words came from: direct.)

---

## function — classify

Found in custom-skills/ranch-state-distillation/scripts/state_sampler.py at line 167.

Surface: public API.

Signature: `def classify(r)`

What it does: (busy?, activity-label) for one probed node.

(Where these words came from: direct.)

---

## function — summarize

Found in custom-skills/ranch-state-distillation/scripts/state_sampler.py at line 185.

Surface: public API.

Signature: `def summarize(results, total)`

What it does: Function summarize takes results, total. Its return type is unannotated; the code shows it returning "; ".join(clauses) + ".".

(Where these words came from: inferred.)

---

## function — coordinate_of

Found in custom-skills/ranch-state-distillation/scripts/state_sampler.py at line 225.

Surface: public API.

Signature: `def coordinate_of(text, atlas)`

What it does: Function coordinate_of takes text, atlas. Its return type is unannotated; the code shows it returning tuple(round_half_up(totals[k], n) for k in range.

(Where these words came from: inferred.)

---

## function — clean

Found in custom-skills/ranch-state-distillation/scripts/state_sampler.py at line 240.

Surface: public API.

Signature: `def clean(text)`

What it does: Function clean takes text. Its return type is unannotated; the code shows it returning "".join(ch for ch in text if ord(ch) not in DELI.

(Where these words came from: inferred.)

---

## function — append_entry

Found in custom-skills/ranch-state-distillation/scripts/state_sampler.py at line 309.

Surface: public API.

Signature: `def append_entry(summary, coord, timestamp)`

What it does: Append one cycle entry under the WAL discipline. The summary's

(Where these words came from: direct.)

---

## function — read_state_scrolls

Found in custom-skills/ranch-state-distillation/scripts/state_sampler.py at line 320.

Surface: public API.

Signature: `def read_state_scrolls()`

What it does: Resolve the logical view: fold the physical scrolls in write

(Where these words came from: direct.)

---

## function — save_raw

Found in custom-skills/ranch-state-distillation/scripts/state_sampler.py at line 342.

Surface: public API.

Signature: `def save_raw(pull, keep_bytes=2_000_000)`

What it does: Function save_raw takes pull, keep_bytes=2_000_000. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in custom-skills/ranch-state-distillation/scripts/state_sampler.py at line 354.

Surface: public API.

Signature: `def main(argv)`

What it does: Function main takes argv. Its return type is unannotated; the code shows it returning 0.

(Where these words came from: inferred.)

---

## function — main

Found in custom-skills/simple-html-publish/scripts/screen_test.py at line 20.

Surface: test.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — fmt

Found in custom-skills/singular/scripts/build_atlas.py at line 72.

Surface: public API.

Signature: `def fmt(c)`

What it does: Function fmt takes c. Its return type is unannotated; the code shows it returning (f"{c[0]}.{c[1]}.{c[2]}/{c[3]}.{c[4]}.{c[5]}".

(Where these words came from: inferred.)

---

## function — parse_coord

Found in custom-skills/singular/scripts/build_atlas.py at line 77.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning tuple(int(p) for p in parts).

(Where these words came from: inferred.)

---

## function — round_half_up

Found in custom-skills/singular/scripts/build_atlas.py at line 82.

Surface: public API.

Signature: `def round_half_up(total, n)`

What it does: Round total/n to the nearest whole step, half up, min 1.

(Where these words came from: direct.)

---

## function — load_list

Found in custom-skills/singular/scripts/build_atlas.py at line 87.

Surface: public API.

Signature: `def load_list(name)`

What it does: Function load_list takes name. Its return type is unannotated; the code shows it returning [w.strip().lower().

(Where these words came from: inferred.)

---

## function — cyoa_votes

Found in custom-skills/singular/scripts/build_atlas.py at line 93.

Surface: public API.

Signature: `def cyoa_votes(cyoa_path)`

What it does: word -> (component sums, scroll count), from valid CYOA scrolls.

(Where these words came from: direct.)

---

## function — place

Found in custom-skills/singular/scripts/build_atlas.py at line 109.

Surface: public API.

Signature: `def place(sums, counts, word)`

What it does: (coordinate tuple, placement, scroll count) for one word.

(Where these words came from: direct.)

---

## function — main

Found in custom-skills/singular/scripts/build_atlas.py at line 124.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 2; 0.

(Where these words came from: inferred.)

---

## function — load_list

Found in custom-skills/singular/scripts/build_editor.py at line 33.

Surface: public API.

Signature: `def load_list(name)`

What it does: Function load_list takes name. Its return type is unannotated; the code shows it returning [w.strip().lower().

(Where these words came from: inferred.)

---

## function — main

Found in custom-skills/singular/scripts/build_editor.py at line 39.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — tokens

Found in custom-skills/singular/scripts/build_wordlist.py at line 50.

Surface: public API.

Signature: `def tokens(text: str)`

What it does: Function tokens takes text: str. Its return type is unannotated; the code shows it returning [w.lower() for w in WORD_RE.findall(text)].

(Where these words came from: inferred.)

---

## function — load_js_json

Found in custom-skills/singular/scripts/build_wordlist.py at line 56.

Surface: public API.

Signature: `def load_js_json(path: Path)`

What it does: Function load_js_json takes path: Path. Its return type is unannotated; the code shows it returning json.loads(raw[raw.index("["):]).

(Where these words came from: inferred.)

---

## function — main

Found in custom-skills/singular/scripts/build_wordlist.py at line 61.

Surface: public API.

Signature: `def main() -> int`

What it does: Function main takes no inputs beyond the value it is called on. It gives back int.

(Where these words came from: inferred.)

---

## function — load_words

Found in custom-skills/singular/scripts/check_singular.py at line 30.

Surface: public API.

Signature: `def load_words(path: Path) -> set`

What it does: Function load_words takes path: Path. It gives back set.

(Where these words came from: inferred.)

---

## function — main

Found in custom-skills/singular/scripts/check_singular.py at line 34.

Surface: public API.

Signature: `def main() -> int`

What it does: Function main takes no inputs beyond the value it is called on. It gives back int.

(Where these words came from: inferred.)

---

## function — fmt

Found in custom-skills/singular/scripts/task_coordinate.py at line 49.

Surface: public API.

Signature: `def fmt(c)`

What it does: Function fmt takes c. Its return type is unannotated; the code shows it returning (f"{c[0]}.{c[1]}.{c[2]}/{c[3]}.{c[4]}.{c[5]}".

(Where these words came from: inferred.)

---

## function — parse_coord

Found in custom-skills/singular/scripts/task_coordinate.py at line 54.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning tuple(int(p) for p in parts).

(Where these words came from: inferred.)

---

## function — round_half_up

Found in custom-skills/singular/scripts/task_coordinate.py at line 59.

Surface: public API.

Signature: `def round_half_up(total, n)`

What it does: Round total/n to the nearest whole step, half up, min 1.

(Where these words came from: direct.)

---

## function — load_atlas

Found in custom-skills/singular/scripts/task_coordinate.py at line 64.

Surface: public API.

Signature: `def load_atlas(path=ATLAS)`

What it does: word -> (coordinate tuple, placement) from atlas.tsv.

(Where these words came from: direct.)

---

## function — main

Found in custom-skills/singular/scripts/task_coordinate.py at line 80.

Surface: public API.

Signature: `def main(argv)`

What it does: Function main takes argv. Its return type is unannotated; the code shows it returning 2; 0.

(Where these words came from: inferred.)

---

## function — normalize

Found in custom-skills/upgoer-five/scripts/check_ug5.py at line 31.

Surface: public API.

Signature: `def normalize(word)`

What it does: Lowercase a word and drop apostrophes and other non-letters.

(Where these words came from: direct.)

---

## function — load_words

Found in custom-skills/upgoer-five/scripts/check_ug5.py at line 36.

Surface: public API.

Signature: `def load_words(path)`

What it does: Function load_words takes path. Its return type is unannotated; the code shows it returning words.

(Where these words came from: inferred.)

---

## function — stems

Found in custom-skills/upgoer-five/scripts/check_ug5.py at line 45.

Surface: public API.

Signature: `def stems(word)`

What it does: Yield shorter forms of a word made by common English endings.

(Where these words came from: direct.)

---

## function — is_allowed

Found in custom-skills/upgoer-five/scripts/check_ug5.py at line 70.

Surface: public API.

Signature: `def is_allowed(word, allowed, strict)`

What it does: Function is_allowed takes word, allowed, strict. Its return type is unannotated; the code shows it returning True; False; any(stem in allowed for stem in stems(word)).

(Where these words came from: inferred.)

---

## function — main

Found in custom-skills/upgoer-five/scripts/check_ug5.py at line 78.

Surface: public API.

Signature: `def main(argv=None)`

What it does: Function main takes argv=None. Its return type is unannotated; the code shows it returning 2; 0; 1.

(Where these words came from: inferred.)

---

## function — fmt

Found in custom-skills/upgoer-five-phext/address_word.py at line 42.

Surface: public API.

Signature: `def fmt(c)`

What it does: Function fmt takes c. Its return type is unannotated; the code shows it returning (f"{c[0]}.{c[1]}.{c[2]}/{c[3]}.{c[4]}.{c[5]}".

(Where these words came from: inferred.)

---

## function — split_parts

Found in custom-skills/upgoer-five-phext/address_word.py at line 47.

Surface: public API.

Signature: `def split_parts(w)`

What it does: Function split_parts takes w. Its return type is unannotated; the code shows it returning [w[:-3], "n't"]; [head, "'" + tail].

(Where these words came from: inferred.)

---

## function — wanted_tokens

Found in custom-skills/upgoer-five-phext/address_word.py at line 54.

Surface: public API.

Signature: `def wanted_tokens(words)`

What it does: Function wanted_tokens takes words. Its return type is unannotated; the code shows it returning wanted.

(Where these words came from: inferred.)

---

## function — scan_glove

Found in custom-skills/upgoer-five-phext/address_word.py at line 64.

Surface: public API.

Signature: `def scan_glove(wanted)`

What it does: Function scan_glove takes wanted. Its return type is unannotated; the code shows it returning found.

(Where these words came from: inferred.)

---

## function — embed

Found in custom-skills/upgoer-five-phext/address_word.py at line 77.

Surface: public API.

Signature: `def embed(word, glove)`

What it does: Function embed takes word, glove. Its return type is unannotated; the code shows it returning glove[w]; glove[squashed]; np.mean(vecs, axis=0); None.

(Where these words came from: inferred.)

---

## function — normalize

Found in custom-skills/upgoer-five-phext/address_word.py at line 93.

Surface: public API.

Signature: `def normalize(word)`

What it does: Function normalize takes word. Its return type is unannotated; the code shows it returning re.sub(r"[^a-z]", "", word.lower()).

(Where these words came from: inferred.)

---

## function — stems

Found in custom-skills/upgoer-five-phext/address_word.py at line 97.

Surface: public API.

Signature: `def stems(word)`

What it does: Function stems takes word. Its return type is unannotated; the code shows it returning values it yields (a generator).

(Where these words came from: inferred.)

---

## function — is_allowed

Found in custom-skills/upgoer-five-phext/address_word.py at line 121.

Surface: public API.

Signature: `def is_allowed(word, allowed)`

What it does: Function is_allowed takes word, allowed. Its return type is unannotated; the code shows it returning True; any(normalize(s) in allowed for s in stems(w)).

(Where these words came from: inferred.)

---

## function — Atlas.__init__

Found in custom-skills/upgoer-five-phext/address_word.py at line 129.

Surface: public API.

Signature: `def __init__(self)`

What it does: Function Atlas.__init__ takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Atlas.coord_of_vec

Found in custom-skills/upgoer-five-phext/address_word.py at line 142.

Surface: public API.

Signature: `def coord_of_vec(self, vec)`

What it does: Function Atlas.coord_of_vec takes self, vec. Its return type is unannotated; the code shows it returning tuple(int(x) for x in np.clip(q, self.qmin, self.

(Where these words came from: inferred.)

---

## function — Atlas.nearest_ug5

Found in custom-skills/upgoer-five-phext/address_word.py at line 147.

Surface: public API.

Signature: `def nearest_ug5(self, coord, k=5, exclude=None)`

What it does: Function Atlas.nearest_ug5 takes self, coord, k=5, exclude=None. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — load_cache

Found in custom-skills/upgoer-five-phext/address_word.py at line 161.

Surface: public API.

Signature: `def load_cache()`

What it does: Function load_cache takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning json.loads(CACHE.read_text()); {"vectors": {}, "unembeddable": []}.

(Where these words came from: inferred.)

---

## function — embed_words

Found in custom-skills/upgoer-five-phext/address_word.py at line 167.

Surface: public API.

Signature: `def embed_words(atlas, words)`

What it does: Embed a list of words, using and updating the query cache.

(Where these words came from: direct.)

---

## function — cmd_address

Found in custom-skills/upgoer-five-phext/address_word.py at line 194.

Surface: public API.

Signature: `def cmd_address(atlas, words)`

What it does: Function cmd_address takes atlas, words. Its return type is unannotated; the code shows it returning status.

(Where these words came from: inferred.)

---

## function — cmd_decode

Found in custom-skills/upgoer-five-phext/address_word.py at line 218.

Surface: public API.

Signature: `def cmd_decode(atlas, address)`

What it does: Function cmd_decode takes atlas, address. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — cmd_validate

Found in custom-skills/upgoer-five-phext/address_word.py at line 233.

Surface: public API.

Signature: `def cmd_validate(atlas, path, k)`

What it does: Function cmd_validate takes atlas, path, k. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — main

Found in custom-skills/upgoer-five-phext/address_word.py at line 259.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning cmd_decode(atlas, args.decode); cmd_validate(atlas, args.validate, args.neighbor; 2; cmd_address(atlas, args.words).

(Where these words came from: inferred.)

---

## function — fmt

Found in custom-skills/upgoer-five-phext/build_atlas.py at line 48.

Surface: public API.

Signature: `def fmt(coord)`

What it does: Function fmt takes coord. Its return type is unannotated; the code shows it returning (f"{c[0]}.{c[1]}.{c[2]}/{c[3]}.{c[4]}.{c[5]}".

(Where these words came from: inferred.)

---

## function — split_parts

Found in custom-skills/upgoer-five-phext/build_atlas.py at line 54.

Surface: public API.

Signature: `def split_parts(w)`

What it does: Function split_parts takes w. Its return type is unannotated; the code shows it returning [w[:-3], "n't"]; [head, "'" + tail].

(Where these words came from: inferred.)

---

## function — wanted_tokens

Found in custom-skills/upgoer-five-phext/build_atlas.py at line 61.

Surface: public API.

Signature: `def wanted_tokens(words)`

What it does: Function wanted_tokens takes words. Its return type is unannotated; the code shows it returning wanted.

(Where these words came from: inferred.)

---

## function — embed

Found in custom-skills/upgoer-five-phext/build_atlas.py at line 71.

Surface: public API.

Signature: `def embed(word, glove)`

What it does: GloVe vector with the contraction fallback chain, else None.

(Where these words came from: direct.)

---

## function — write_phext

Found in custom-skills/upgoer-five-phext/build_atlas.py at line 86.

Surface: public API.

Signature: `def write_phext(path, scrolls)`

What it does: Write {coord_tuple: text} as a phext file, dead-reckoning style.

(Where these words came from: direct.)

---

## function — main

Found in custom-skills/upgoer-five-phext/build_atlas.py at line 113.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — coord_of

Found in custom-skills/upgoer-five-phext/build_atlas.py at line 159.

Surface: public API.

Signature: `def coord_of(vec)`

What it does: Function coord_of takes vec. Its return type is unannotated; the code shows it returning tuple(int(x) for x in np.clip(q, QMIN, QMAX)), p.

(Where these words came from: inferred.)

---

## function — main

Found in custom-skills/upgoer-five-phext/build_editor.py at line 17.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — sq_status

Found in dashboard/collect_status.py at line 22.

Surface: public API.

Signature: `def sq_status()`

What it does: Function sq_status takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning info; {"running": False, "error": type(e).__name__}.

(Where these words came from: inferred.)

---

## function — sentinel

Found in dashboard/collect_status.py at line 43.

Surface: public API.

Signature: `def sentinel()`

What it does: Function sentinel takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning {; {"error": type(e).__name__}.

(Where these words came from: inferred.)

---

## function — main

Found in dashboard/collect_status.py at line 65.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — get

Found in discord-presence/pass.py at line 14.

Surface: public API.

Signature: `def get(path)`

What it does: Function get takes path. Its return type is unannotated; the code shows it returning discord_api.call("GET", path).

(Where these words came from: inferred.)

---

## function — must

Found in github-publish/publish_branch.py at line 17.

Surface: public API.

Signature: `def must(status, out, step)`

What it does: Function must takes status, out, step. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — git

Found in github-publish/publish_branch.py at line 23.

Surface: public API.

Signature: `def git(repo_dir, *args)`

What it does: Function git takes repo_dir, *args. Its return type is unannotated; the code shows it returning subprocess.run(["git", "-C", repo_dir, *args],.

(Where these words came from: inferred.)

---

## function — main

Found in github-publish/publish_branch.py at line 27.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — must

Found in github-publish/publish_incremental.py at line 17.

Surface: public API.

Signature: `def must(status, out, step)`

What it does: Function must takes status, out, step. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — git

Found in github-publish/publish_incremental.py at line 23.

Surface: public API.

Signature: `def git(repo_dir, *args)`

What it does: Function git takes repo_dir, *args. Its return type is unannotated; the code shows it returning subprocess.run(["git", "-C", repo_dir, *args],.

(Where these words came from: inferred.)

---

## function — main

Found in github-publish/publish_incremental.py at line 27.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — emit

Found in neuralese/build_neuralese.py at line 4.

Surface: public API.

Signature: `def emit(targets)`

What it does: Function emit takes targets. Its return type is unannotated; the code shows it returning ''.join(out).

(Where these words came from: inferred.)

---

## function — C

Found in neuralese/build_neuralese.py at line 16.

Surface: public API.

Signature: `def C(ch, s=1, sc=1): return (1,1,1, 1,1,1, ch,s,sc)`

What it does: Function C takes ch, s=1, sc=1. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — vec

Found in neuralese/demo/run_demo.py at line 15.

Surface: public API.

Signature: `def vec(g): return hashlib.sha256(('neuralese-vec:'+g).encode()).digest()[:16].hex()`

What it does: Function vec takes g. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — encode_phext

Found in neuralese/demo/run_demo.py at line 17.

Surface: public API.

Signature: `def encode_phext(units, path)`

What it does: Function encode_phext takes units, path. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — phext_receive

Found in neuralese/demo/run_demo.py at line 33.

Surface: public API.

Signature: `def phext_receive(path)`

What it does: Function phext_receive takes path. Its return type is unannotated; the code shows it returning units, by_coord.

(Where these words came from: inferred.)

---

## function — main

Found in neuralese/demo/run_demo.py at line 50.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — rec

Found in neuralese/demo/run_demo.py at line 55.

Surface: public API.

Signature: `def rec(chan, q, ok, cost): results.append((chan, q, ok, cost))`

What it does: Function rec takes chan, q, ok, cost. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — hops

Found in neuralese/demo/run_demo.py at line 77.

Surface: public API.

Signature: `def hops(uid, n)`

What it does: Function hops takes uid, n. Its return type is unannotated; the code shows it returning seen - {uid}.

(Where these words came from: inferred.)

---

## function — vec_digest

Found in neuralese/neuralese_codec.py at line 13.

Surface: public API.

Signature: `def vec_digest(gist: str) -> str`

What it does: Function vec_digest takes gist: str. It gives back str.

(Where these words came from: inferred.)

---

## function — fmt_coord

Found in neuralese/neuralese_codec.py at line 16.

Surface: public API.

Signature: `def fmt_coord(c): return f"{c[0]}.{c[1]}.{c[2]}/{c[3]}.{c[4]}.{c[5]}/{c[6]}.{c[7]}.{c[8]}"`

What it does: Function fmt_coord takes c. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — encode

Found in neuralese/neuralese_codec.py at line 18.

Surface: public API.

Signature: `def encode(units)`

What it does: units: list of dicts with frame=(c,v,b), gist, links=[coord strings].

(Where these words came from: direct.)

---

## function — decode

Found in neuralese/neuralese_codec.py at line 38.

Surface: public API.

Signature: `def decode(phext_text, path='/tmp/_neuralese_demo.phext')`

What it does: Function decode takes phext_text, path='/tmp/_neuralese_demo.phext'. Its return type is unannotated; the code shows it returning units.

(Where these words came from: inferred.)

---

## function — decode_all

Found in phext/audio-s2t/build_audio_phext.py at line 68.

Surface: public API.

Signature: `def decode_all(fresh=False)`

What it does: Function decode_all takes fresh=False. Its return type is unannotated; the code shows it returning json.load(open(cache)); results.

(Where these words came from: inferred.)

---

## function — load_wav

Found in phext/audio-s2t/build_audio_phext.py at line 105.

Surface: public API.

Signature: `def load_wav(uid)`

What it does: Function load_wav takes uid. Its return type is unannotated; the code shows it returning x.

(Where these words came from: inferred.)

---

## function — silence_runs

Found in phext/audio-s2t/build_audio_phext.py at line 112.

Surface: public API.

Signature: `def silence_runs(x)`

What it does: Measured silences (start_s, end_s) with an adaptive per-file threshold:

(Where these words came from: direct.)

---

## function — f0_window

Found in phext/audio-s2t/build_audio_phext.py at line 132.

Surface: public API.

Signature: `def f0_window(x, t0, t1)`

What it does: Crude autocorrelation pitch tracker: median F0 (Hz) over voiced frames

(Where these words came from: direct.)

---

## function — segment_utterance

Found in phext/audio-s2t/build_audio_phext.py at line 157.

Surface: public API.

Signature: `def segment_utterance(u)`

What it does: Split one utterance's words at measured silences. Returns segments:

(Where these words came from: direct.)

---

## function — features

Found in phext/audio-s2t/build_audio_phext.py at line 194.

Surface: public API.

Signature: `def features(seg)`

What it does: Function features takes seg. Its return type is unannotated; the code shows it returning {"t0": t0, "t1": t1, "dur": t1 - t0, "rms": rms,.

(Where these words came from: inferred.)

---

## function — emit

Found in phext/audio-s2t/build_audio_phext.py at line 222.

Surface: public API.

Signature: `def emit(targets)`

What it does: Function emit takes targets. Its return type is unannotated; the code shows it returning "".join(out).

(Where these words came from: inferred.)

---

## function — fmt

Found in phext/audio-s2t/build_audio_phext.py at line 237.

Surface: public API.

Signature: `def fmt(coord)`

What it does: Function fmt takes coord. Its return type is unannotated; the code shows it returning (f"{coord[0]}.{coord[1]}.{coord[2]}/{coord[3]}.{.

(Where these words came from: inferred.)

---

## function — norm_words

Found in phext/audio-s2t/build_audio_phext.py at line 242.

Surface: public API.

Signature: `def norm_words(text)`

What it does: Function norm_words takes text. Its return type is unannotated; the code shows it returning [w.strip("'") for w in text.split() if w.strip(".

(Where these words came from: inferred.)

---

## function — wer

Found in phext/audio-s2t/build_audio_phext.py at line 246.

Surface: public API.

Signature: `def wer(ref, hyp)`

What it does: Function wer takes ref, hyp. Its return type is unannotated; the code shows it returning d[len(r)][len(h)], len(r).

(Where these words came from: inferred.)

---

## function — main

Found in phext/audio-s2t/build_audio_phext.py at line 258.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — decode

Found in phext/audio-s2t/decode.py at line 22.

Surface: public API.

Signature: `def decode(flac_path)`

What it does: Function decode takes flac_path. Its return type is unannotated; the code shows it returning words, dur.

(Where these words came from: inferred.)

---

## function — main

Found in phext/audio-s2t/decode.py at line 46.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — load

Found in phext/audio-s2t/probe_silence.py at line 10.

Surface: public API.

Signature: `def load(path)`

What it does: Function load takes path. Its return type is unannotated; the code shows it returning x.

(Where these words came from: inferred.)

---

## function — c

Found in phext/build_exocortex.py at line 47.

Surface: public API.

Signature: `def c(s)`

What it does: Function c takes s. Its return type is unannotated; the code shows it returning tuple(int(x) for part in s.split("/") for x in p.

(Where these words came from: inferred.)

---

## function — implode

Found in phext/build_exocortex.py at line 86.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: Function implode takes scrolls. Its return type is unannotated; the code shows it returning bytes(out).

(Where these words came from: inferred.)

---

## function — parse_coord

Found in phext/build_life_phext.py at line 51.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — implode

Found in phext/build_life_phext.py at line 59.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord-string -> text. Same state machine as

(Where these words came from: direct.)

---

## function — sanitize

Found in phext/build_life_phext.py at line 77.

Surface: public API.

Signature: `def sanitize(text)`

What it does: Function sanitize takes text. Its return type is unannotated; the code shows it returning text.

(Where these words came from: inferred.)

---

## function — message_entry

Found in phext/build_life_phext.py at line 88.

Surface: public API.

Signature: `def message_entry(m)`

What it does: Function message_entry takes m. Its return type is unannotated; the code shows it returning head + "\n" + body + "\n".

(Where these words came from: inferred.)

---

## function — build

Found in phext/build_life_phext.py at line 96.

Surface: public API.

Signature: `def build(source)`

What it does: Function build takes source. Its return type is unannotated; the code shows it returning implode(scrolls).

(Where these words came from: inferred.)

---

## function — main

Found in phext/build_life_phext.py at line 143.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — c

Found in phext/build_skills_phext.py at line 148.

Surface: public API.

Signature: `def c(s)`

What it does: Function c takes s. Its return type is unannotated; the code shows it returning tuple(int(x) for part in s.split("/") for x in p.

(Where these words came from: inferred.)

---

## function — fmt

Found in phext/build_skills_phext.py at line 152.

Surface: public API.

Signature: `def fmt(coord)`

What it does: Function fmt takes coord. Its return type is unannotated; the code shows it returning "%d.%d.%d/%d.%d.%d/%d.%d.%d" % coord.

(Where these words came from: inferred.)

---

## function — coord_for

Found in phext/build_skills_phext.py at line 156.

Surface: public API.

Signature: `def coord_for(shelf, volume, scroll)`

What it does: Function coord_for takes shelf, volume, scroll. Its return type is unannotated; the code shows it returning (1, shelf, 1, 1, volume, 1, 1, 1, scroll).

(Where these words came from: inferred.)

---

## function — collect_files

Found in phext/build_skills_phext.py at line 160.

Surface: public API.

Signature: `def collect_files(skill_dir)`

What it does: Return (included, excluded) for one skill dir.

(Where these words came from: direct.)

---

## function — implode

Found in phext/build_skills_phext.py at line 206.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord-string -> text. Same state machine as build_exocortex.py.

(Where these words came from: direct.)

---

## function — build

Found in phext/build_skills_phext.py at line 223.

Surface: public API.

Signature: `def build()`

What it does: Build the phext blob from the current source tree.

(Where these words came from: direct.)

---

## function — sha256_file

Found in phext/build_skills_phext.py at line 312.

Surface: public API.

Signature: `def sha256_file(path)`

What it does: Function sha256_file takes path. Its return type is unannotated; the code shows it returning h.hexdigest().

(Where these words came from: inferred.)

---

## function — committed_index

Found in phext/build_skills_phext.py at line 320.

Surface: public API.

Signature: `def committed_index()`

What it does: Parse the per-file index out of the committed skills.phext Home scroll.

(Where these words came from: direct.)

---

## function — check

Found in phext/build_skills_phext.py at line 341.

Surface: public API.

Signature: `def check()`

What it does: Rebuild to a temp file and compare SHA-256 with the committed

(Where these words came from: direct.)

---

## function — main

Found in phext/build_skills_phext.py at line 391.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — parse_coord

Found in phext/build_tasks_phext.py at line 40.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — implode

Found in phext/build_tasks_phext.py at line 48.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord-string -> text. Same state machine as

(Where these words came from: direct.)

---

## function — render_task

Found in phext/build_tasks_phext.py at line 66.

Surface: public API.

Signature: `def render_task(t)`

What it does: Function render_task takes t. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — render_home

Found in phext/build_tasks_phext.py at line 85.

Surface: public API.

Signature: `def render_home(tasks)`

What it does: Function render_home takes tasks. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — build

Found in phext/build_tasks_phext.py at line 110.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls), tasks.

(Where these words came from: inferred.)

---

## function — main

Found in phext/build_tasks_phext.py at line 125.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — parse_coord

Found in phext/build_todo_phext.py at line 77.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — fmt

Found in phext/build_todo_phext.py at line 85.

Surface: public API.

Signature: `def fmt(c)`

What it does: Function fmt takes c. Its return type is unannotated; the code shows it returning f"{c[0]}.{c[1]}.{c[2]}/{c[3]}.{c[4]}.{c[5]}/{c[6.

(Where these words came from: inferred.)

---

## function — merge

Found in phext/build_todo_phext.py at line 89.

Surface: public API.

Signature: `def merge(text, atlas)`

What it does: Holographic merge of a text's distinct words (Singular rule).

(Where these words came from: direct.)

---

## function — implode

Found in phext/build_todo_phext.py at line 106.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord-string -> text. Same state machine as

(Where these words came from: direct.)

---

## function — render_ranked

Found in phext/build_todo_phext.py at line 148.

Surface: public API.

Signature: `def render_ranked(placed_by_title, integration)`

What it does: The good-singularity-ranked list scroll: Collection 2, Book 2.

(Where these words came from: direct.)

---

## function — render_register_entry

Found in phext/build_todo_phext.py at line 200.

Surface: public API.

Signature: `def render_register_entry(e)`

What it does: Function render_register_entry takes e. Its return type is unannotated; the code shows it returning "\n".join([.

(Where these words came from: inferred.)

---

## function — render_protocol

Found in phext/build_todo_phext.py at line 219.

Surface: public API.

Signature: `def render_protocol(integration)`

What it does: Function render_protocol takes integration. Its return type is unannotated; the code shows it returning "\n".join([.

(Where these words came from: inferred.)

---

## function — render_finding

Found in phext/build_todo_phext.py at line 274.

Surface: public API.

Signature: `def render_finding(f)`

What it does: Function render_finding takes f. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — render_home

Found in phext/build_todo_phext.py at line 293.

Surface: public API.

Signature: `def render_home(placed, n_walked, findings_sha, n_findings, integration=None)`

What it does: Function render_home takes placed, n_walked, findings_sha, n_findings, integration=None. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — build

Found in phext/build_todo_phext.py at line 368.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls).

(Where these words came from: inferred.)

---

## function — main

Found in phext/build_todo_phext.py at line 420.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — blob_of

Found in phext/emise-sync.sh at line 27.

Surface: public API.

Signature: `blob_of()`

What it does: Function blob_of takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — coordinate_for

Found in phext/life_track.py at line 42.

Surface: public API.

Signature: `def coordinate_for(content)`

What it does: Function coordinate_for takes content. Its return type is unannotated; the code shows it returning fmt(FAR), "far-corner (no words)"; fmt(tuple(round_half_up(totals[k], n) for k in r.

(Where these words came from: inferred.)

---

## function — fetch_all

Found in phext/life_track.py at line 57.

Surface: public API.

Signature: `def fetch_all(watermark)`

What it does: Return raw Discord messages newer than watermark (or all, when

(Where these words came from: direct.)

---

## function — main

Found in phext/life_track.py at line 89.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — fmt

Found in phext/parse_phext.py at line 19.

Surface: public API.

Signature: `def fmt(coord)`

What it does: Function fmt takes coord. Its return type is unannotated; the code shows it returning f"{coord[0]}.{coord[1]}.{coord[2]}/{coord[3]}.{c.

(Where these words came from: inferred.)

---

## function — parse

Found in phext/parse_phext.py at line 22.

Surface: public API.

Signature: `def parse(path)`

What it does: Function parse takes path. Its return type is unannotated; the code shows it returning scrolls.

(Where these words came from: inferred.)

---

## function — title_of

Found in phext/parse_phext.py at line 45.

Surface: public API.

Signature: `def title_of(text)`

What it does: Function title_of takes text. Its return type is unannotated; the code shows it returning line[:110]; "(empty)".

(Where these words came from: inferred.)

---

## function — load_parser

Found in phext/phext_dispatch.py at line 57.

Surface: public API.

Signature: `def load_parser()`

What it does: Load parse() from parse_phext.py via importlib; fall back to inline.

(Where these words came from: direct.)

---

## function — coord_components

Found in phext/phext_dispatch.py at line 72.

Surface: public API.

Signature: `def coord_components(coord_str)`

What it does: Function coord_components takes coord_str. Its return type is unannotated; the code shows it returning [int(x) for part in coord_str.strip().split("/").

(Where these words came from: inferred.)

---

## function — prefix_components

Found in phext/phext_dispatch.py at line 75.

Surface: public API.

Signature: `def prefix_components(prefix_str)`

What it does: Function prefix_components takes prefix_str. Its return type is unannotated; the code shows it returning parts.

(Where these words came from: inferred.)

---

## function — select

Found in phext/phext_dispatch.py at line 81.

Surface: public API.

Signature: `def select(scrolls, coords, prefixes, want_all)`

What it does: Function select takes scrolls, coords, prefixes, want_all. Its return type is unannotated; the code shows it returning chosen, missing.

(Where these words came from: inferred.)

---

## function — first_line

Found in phext/phext_dispatch.py at line 100.

Surface: public API.

Signature: `def first_line(text, limit=110)`

What it does: Function first_line takes text, limit=110. Its return type is unannotated; the code shows it returning line[:limit]; "(empty)".

(Where these words came from: inferred.)

---

## function — sha256_of

Found in phext/phext_dispatch.py at line 107.

Surface: public API.

Signature: `def sha256_of(path)`

What it does: Function sha256_of takes path. Its return type is unannotated; the code shows it returning h.hexdigest().

(Where these words came from: inferred.)

---

## function — render_scroll

Found in phext/phext_dispatch.py at line 114.

Surface: public API.

Signature: `def render_scroll(coord, text)`

What it does: Function render_scroll takes coord, text. Its return type is unannotated; the code shows it returning f"### {coord}\n\n{body}\n".

(Where these words came from: inferred.)

---

## function — paginate

Found in phext/phext_dispatch.py at line 123.

Surface: public API.

Signature: `def paginate(chosen, page_scrolls, page_chars)`

What it does: Function paginate takes chosen, page_scrolls, page_chars. Its return type is unannotated; the code shows it returning pages.

(Where these words came from: inferred.)

---

## function — main

Found in phext/phext_dispatch.py at line 136.

Surface: public API.

Signature: `def main(argv=None)`

What it does: Function main takes argv=None. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — Config.__init__

Found in phext-mailbox/phext_mailbox.py at line 60.

Surface: public API.

Signature: `def __init__(self, args)`

What it does: Function Config.__init__ takes self, args. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — fmt

Found in phext-mailbox/phext_mailbox.py at line 77.

Surface: public API.

Signature: `def fmt(c)`

What it does: Function fmt takes c. Its return type is unannotated; the code shows it returning "%d.%d.%d/%d.%d.%d/%d.%d.%d" % c.

(Where these words came from: inferred.)

---

## function — unfmt

Found in phext-mailbox/phext_mailbox.py at line 81.

Surface: public API.

Signature: `def unfmt(s)`

What it does: Function unfmt takes s. Its return type is unannotated; the code shows it returning tuple(nums).

(Where these words came from: inferred.)

---

## function — locked

Found in phext-mailbox/phext_mailbox.py at line 89.

Surface: public API.

Signature: `def locked(cfg, exclusive)`

What it does: Hold the mailbox lock. Readers share; only compact excludes.

(Where these words came from: direct.)

---

## function — load

Found in phext-mailbox/phext_mailbox.py at line 115.

Surface: public API.

Signature: `def load(cfg)`

What it does: Function load takes cfg. Its return type is unannotated; the code shows it returning {}; {k.as_tuple(): v for k, v in PX.explode(f.read().

(Where these words came from: inferred.)

---

## function — save

Found in phext-mailbox/phext_mailbox.py at line 122.

Surface: public API.

Signature: `def save(cfg, scrolls)`

What it does: Function save takes cfg, scrolls. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — index_lines

Found in phext-mailbox/phext_mailbox.py at line 138.

Surface: public API.

Signature: `def index_lines(cfg, scrolls)`

What it does: Function index_lines takes cfg, scrolls. Its return type is unannotated; the code shows it returning [l for l in text.split("\n") if l].

(Where these words came from: inferred.)

---

## function — compact

Found in phext-mailbox/phext_mailbox.py at line 143.

Surface: public API.

Signature: `def compact(cfg, dry)`

What it does: Function compact takes cfg, dry. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — init

Found in phext-mailbox/phext_mailbox.py at line 189.

Surface: public API.

Signature: `def init(cfg)`

What it does: Function init takes cfg. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in phext-mailbox/phext_mailbox.py at line 199.

Surface: public API.

Signature: `def main(argv=None)`

What it does: Function main takes argv=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — fmt

Found in site-map/insert_scroll.py at line 26.

Surface: public API.

Signature: `def fmt(c)`

What it does: Function fmt takes c. Its return type is unannotated; the code shows it returning f"{c[0]}.{c[1]}.{c[2]}/{c[3]}.{c[4]}.{c[5]}/{c[6.

(Where these words came from: inferred.)

---

## function — key_of

Found in site-map/insert_scroll.py at line 29.

Surface: public API.

Signature: `def key_of(c)`

What it does: Function key_of takes c. Its return type is unannotated; the code shows it returning tuple(c).

(Where these words came from: inferred.)

---

## function — parse_slots

Found in site-map/insert_scroll.py at line 32.

Surface: public API.

Signature: `def parse_slots(data)`

What it does: Return [(coord_tuple, text_start, text_end, terminator_byte_or_None)].

(Where these words came from: direct.)

---

## function — parse_target

Found in site-map/insert_scroll.py at line 51.

Surface: public API.

Signature: `def parse_target(s)`

What it does: Function parse_target takes s. Its return type is unannotated; the code shows it returning tuple(parts).

(Where these words came from: inferred.)

---

## function — main

Found in site-map/insert_scroll.py at line 56.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — sq_status

Found in sq-bridge/sq_bridge.py at line 162.

Surface: public API.

Signature: `def sq_status() -> str`

What it does: Report the bridge configuration and whether an SQ host is answering

(Where these words came from: direct.)

---

## function — sq_open_channel

Found in sq-bridge/sq_bridge.py at line 194.

Surface: public API.

Signature: `def sq_open_channel(phext: str, port: int = 0) -> str`

What it does: Ensure an SQ host is running that can serve `phext` from SQ_DIR, and

(Where these words came from: direct.)

---

## function — sq_toc

Found in sq-bridge/sq_bridge.py at line 239.

Surface: public API.

Signature: `def sq_toc(phext: str) -> str`

What it does: Table of contents for a phext: one '* <coordinate>: <scroll text>'

(Where these words came from: direct.)

---

## function — sq_read

Found in sq-bridge/sq_bridge.py at line 247.

Surface: public API.

Signature: `def sq_read(phext: str, coordinate: str) -> str`

What it does: Fetch one scroll's exact text by coordinate (SQ's select endpoint;

(Where these words came from: direct.)

---

## function — sq_query

Found in sq-bridge/sq_bridge.py at line 255.

Surface: public API.

Signature: `def sq_query(phext: str, from_coordinate: str, to_coordinate: str) -> str`

What it does: Range query: every scroll with a coordinate between from and to

(Where these words came from: direct.)

---

## function — sq_write

Found in sq-bridge/sq_bridge.py at line 265.

Surface: public API.

Signature: `def sq_write(phext: str, coordinate: str, text: str) -> str`

What it does: Overwrite one scroll via SQ's update endpoint (auto-saves; SQ

(Where these words came from: direct.)

---

## function — sq_close_channel

Found in sq-bridge/sq_bridge.py at line 285.

Surface: public API.

Signature: `def sq_close_channel(port: int = 0) -> str`

What it does: Stop an SQ host that THIS bridge started (SIGTERM to its process

(Where these words came from: direct.)

---

## function — server

Found in sq-bridge/test_client.py at line 22.

Surface: test.

Signature: `def server(readonly)`

What it does: Function server takes readonly. Its return type is unannotated; the code shows it returning StdioServerParameters(.

(Where these words came from: inferred.)

---

## function — show

Found in sq-bridge/test_client.py at line 36.

Surface: test.

Signature: `def show(label, result)`

What it does: Function show takes label, result. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — phase

Found in sq-bridge/test_client.py at line 43.

Surface: test.

Signature: `async def phase(readonly, fn)`

What it does: Function phase takes readonly, fn. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — phase_a

Found in sq-bridge/test_client.py at line 50.

Surface: test.

Signature: `async def phase_a(session)`

What it does: Function phase_a takes session. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — phase_b

Found in sq-bridge/test_client.py at line 69.

Surface: test.

Signature: `async def phase_b(session)`

What it does: Function phase_b takes session. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in sq-bridge/test_client.py at line 82.

Surface: test.

Signature: `async def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — show

Found in sq-bridge/test_client_remote.py at line 29.

Surface: test.

Signature: `def show(label, result)`

What it does: Function show takes label, result. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in sq-bridge/test_client_remote.py at line 35.

Surface: test.

Signature: `async def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — show

Found in sq-bridge/tests/test_client_remote_claude.py at line 33.

Surface: test.

Signature: `def show(label, result)`

What it does: Function show takes label, result. Its return type is unannotated; the code shows it returning text, getattr(result, "is_error", False).

(Where these words came from: inferred.)

---

## function — main

Found in sq-bridge/tests/test_client_remote_claude.py at line 40.

Surface: test.

Signature: `async def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — sha256

Found in sq-bridge/tests/test_readonly_dir.py at line 25.

Surface: test.

Signature: `def sha256(path)`

What it does: Function sha256 takes path. Its return type is unannotated; the code shows it returning hashlib.sha256(f.read()).hexdigest().

(Where these words came from: inferred.)

---

## function — snapshot

Found in sq-bridge/tests/test_readonly_dir.py at line 30.

Surface: test.

Signature: `def snapshot(d)`

What it does: Function snapshot takes d. Its return type is unannotated; the code shows it returning sorted(os.listdir(d)), {f: sha256(os.path.join(d.

(Where these words came from: inferred.)

---

## function — show

Found in sq-bridge/tests/test_readonly_dir.py at line 36.

Surface: test.

Signature: `def show(label, result)`

What it does: Function show takes label, result. Its return type is unannotated; the code shows it returning text, getattr(result, "is_error", False).

(Where these words came from: inferred.)

---

## function — main

Found in sq-bridge/tests/test_readonly_dir.py at line 43.

Surface: test.

Signature: `async def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — load_config

Found in tools/commit-rollup/rollup.py at line 67.

Surface: public API.

Signature: `def load_config()`

What it does: Function load_config takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning cfg.

(Where these words came from: inferred.)

---

## function — collect

Found in tools/commit-rollup/rollup.py at line 110.

Surface: public API.

Signature: `def collect(owner, start, end)`

What it does: Normalized, SHA-deduped, time-sorted commit list for a window.

(Where these words came from: direct.)

---

## function — segment_sessions

Found in tools/commit-rollup/rollup.py at line 135.

Surface: public API.

Signature: `def segment_sessions(commits, dormancy_minutes)`

What it does: Split a sorted commit list where the quiet gap exceeds the threshold.

(Where these words came from: direct.)

---

## function — extract_prs

Found in tools/commit-rollup/rollup.py at line 155.

Surface: public API.

Signature: `def extract_prs(commits)`

What it does: Function extract_prs takes commits. Its return type is unannotated; the code shows it returning prs.

(Where these words came from: inferred.)

---

## function — effort_bar

Found in tools/commit-rollup/rollup.py at line 175.

Surface: public API.

Signature: `def effort_bar(count, total, width)`

What it does: Function effort_bar takes count, total, width. Its return type is unannotated; the code shows it returning "░" * width; "█" * filled + "░" * (width - filled).

(Where these words came from: inferred.)

---

## function — build_embed

Found in tools/commit-rollup/rollup.py at line 196.

Surface: public API.

Signature: `def build_embed(commits, cfg)`

What it does: Function build_embed takes commits, cfg. Its return type is unannotated; the code shows it returning {.

(Where these words came from: inferred.)

---

## function — embed_text_mock

Found in tools/commit-rollup/rollup.py at line 281.

Surface: public API.

Signature: `def embed_text_mock(embed)`

What it does: Plain-text rendering of an embed, for sample files / eyeball checks.

(Where these words came from: direct.)

---

## function — embed_char_total

Found in tools/commit-rollup/rollup.py at line 292.

Surface: public API.

Signature: `def embed_char_total(embed)`

What it does: Function embed_char_total takes embed. Its return type is unannotated; the code shows it returning n.

(Where these words came from: inferred.)

---

## function — load_state

Found in tools/commit-rollup/rollup.py at line 302.

Surface: public API.

Signature: `def load_state()`

What it does: Function load_state takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning None; json.load(f).

(Where these words came from: inferred.)

---

## function — save_state

Found in tools/commit-rollup/rollup.py at line 309.

Surface: public API.

Signature: `def save_state(state)`

What it does: Function save_state takes state. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — init_state

Found in tools/commit-rollup/rollup.py at line 317.

Surface: public API.

Signature: `def init_state(cfg, now)`

What it does: Seed the watermark at now; mark the lookback window already posted.

(Where these words came from: direct.)

---

## function — post_embed

Found in tools/commit-rollup/rollup.py at line 338.

Surface: public API.

Signature: `def post_embed(cfg, embed)`

What it does: Function post_embed takes cfg, embed. Its return type is unannotated; the code shows it returning out.get("id").

(Where these words came from: inferred.)

---

## function — poll

Found in tools/commit-rollup/rollup.py at line 348.

Surface: public API.

Signature: `def poll(cfg, dry_run=False, now=None)`

What it does: Function poll takes cfg, dry_run=False, now=None. Its return type is unannotated; the code shows it returning 0; init_state(cfg, now).

(Where these words came from: inferred.)

---

## function — day_report

Found in tools/commit-rollup/rollup.py at line 391.

Surface: public API.

Signature: `def day_report(cfg, day_str, out_prefix=None, repos_file=None)`

What it does: Function day_report takes cfg, day_str, out_prefix=None, repos_file=None. Its return type is unannotated; the code shows it returning 0.

(Where these words came from: inferred.)

---

## function — main

Found in tools/commit-rollup/rollup.py at line 447.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; init_state(cfg, datetime.now(timezone.utc)); day_report(cfg, args.day, args.out, args.repos_f; poll(cfg, dry_run=args.dry_run); 1.

(Where these words came from: inferred.)

---

## function — df_home

Found in tools/disk_usage_report.py at line 32.

Surface: public API.

Signature: `def df_home()`

What it does: Function df_home takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning total, used, avail, parts[4].

(Where these words came from: inferred.)

---

## function — write_probe

Found in tools/disk_usage_report.py at line 39.

Surface: public API.

Signature: `def write_probe()`

What it does: Function write_probe takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning True; False.

(Where these words came from: inferred.)

---

## function — top_dirs

Found in tools/disk_usage_report.py at line 49.

Surface: public API.

Signature: `def top_dirs()`

What it does: Top-level workspace dirs by size (bytes), largest first.

(Where these words came from: direct.)

---

## function — scan_files

Found in tools/disk_usage_report.py at line 62.

Surface: public API.

Signature: `def scan_files()`

What it does: One find pass over home: total file count + files >= BIG_FILE_GB.

(Where these words came from: direct.)

---

## function — trash_size

Found in tools/disk_usage_report.py at line 82.

Surface: public API.

Signature: `def trash_size()`

What it does: Function trash_size takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; int(out.stdout.split()[0]).

(Where these words came from: inferred.)

---

## function — fmt

Found in tools/disk_usage_report.py at line 93.

Surface: public API.

Signature: `def fmt(gb)`

What it does: Function fmt takes gb. Its return type is unannotated; the code shows it returning f"{gb:.1f} GB" if gb >= 10 else f"{gb:.2f} GB".

(Where these words came from: inferred.)

---

## function — main

Found in tools/disk_usage_report.py at line 97.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0 if probe_ok else 2.

(Where these words came from: inferred.)

---

## function — progress_total

Found in tools/duty-governor/analyze_runs.py at line 16.

Surface: public API.

Signature: `def progress_total(run_dir)`

What it does: Function progress_total takes run_dir. Its return type is unannotated; the code shows it returning total.

(Where these words came from: inferred.)

---

## function — thermal_window

Found in tools/duty-governor/analyze_runs.py at line 31.

Surface: public API.

Signature: `def thermal_window(path, start_epoch, end_epoch)`

What it does: Function thermal_window takes path, start_epoch, end_epoch. Its return type is unannotated; the code shows it returning samples.

(Where these words came from: inferred.)

---

## function — main

Found in tools/duty-governor/analyze_runs.py at line 48.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — iso

Found in tools/duty-governor/duty_governor.py at line 90.

Surface: public API.

Signature: `def iso(epoch)`

What it does: Function iso takes epoch. Its return type is unannotated; the code shows it returning datetime.fromtimestamp(epoch).astimezone().isofo.

(Where these words came from: inferred.)

---

## function — resolve_tctl_path

Found in tools/duty-governor/duty_governor.py at line 94.

Surface: public API.

Signature: `def resolve_tctl_path()`

What it does: Find k10temp's temp1_input by sensor NAME (hwmon index varies).

(Where these words came from: direct.)

---

## function — Governor.__init__

Found in tools/duty-governor/duty_governor.py at line 115.

Surface: public API.

Signature: `def __init__(self, cfg, cmd, log_path, max_seconds=None, tctl_file=None)`

What it does: Function Governor.__init__ takes self, cfg, cmd, log_path, max_seconds=None, tctl_file=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Governor.log

Found in tools/duty-governor/duty_governor.py at line 146.

Surface: public API.

Signature: `def log(self, event, tctl=None, extra="")`

What it does: Function Governor.log takes self, event, tctl=None, extra="". Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Governor.read_tctl

Found in tools/duty-governor/duty_governor.py at line 158.

Surface: public API.

Signature: `def read_tctl(self)`

What it does: Function Governor.read_tctl takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning float(f.read().strip()); None; int(f.read().strip()) / 1000.0.

(Where these words came from: inferred.)

---

## function — Governor.group_signal

Found in tools/duty-governor/duty_governor.py at line 174.

Surface: public API.

Signature: `def group_signal(self, sig)`

What it does: Function Governor.group_signal takes self, sig. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Governor.duty_target

Found in tools/duty-governor/duty_governor.py at line 195.

Surface: public API.

Signature: `def duty_target(self)`

What it does: Duty fraction for the current ladder level (level 0 = 1.0).

(Where these words came from: direct.)

---

## function — Governor.run

Found in tools/duty-governor/duty_governor.py at line 227.

Surface: public API.

Signature: `def run(self)`

What it does: Function Governor.run takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 2; exit_code if exit_code is not None and exit_code.

(Where these words came from: inferred.)

---

## function — Governor.write_summary

Found in tools/duty-governor/duty_governor.py at line 357.

Surface: public API.

Signature: `def write_summary(self, start, end, exit_code, timed_out)`

What it does: Function Governor.write_summary takes self, start, end, exit_code, timed_out. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Governor.pct

Found in tools/duty-governor/duty_governor.py at line 360.

Surface: public API.

Signature: `def pct(p)`

What it does: Function Governor.pct takes p. Its return type is unannotated; the code shows it returning None; s[min(len(s) - 1, int(p * len(s)))].

(Where these words came from: inferred.)

---

## function — Governor.mean

Found in tools/duty-governor/duty_governor.py at line 365.

Surface: public API.

Signature: `def mean(xs)`

What it does: Function Governor.mean takes xs. Its return type is unannotated; the code shows it returning round(sum(xs) / len(xs), 2) if xs else None.

(Where these words came from: inferred.)

---

## function — main

Found in tools/duty-governor/duty_governor.py at line 407.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — on_signal

Found in tools/duty-governor/duty_governor.py at line 460.

Surface: public API.

Signature: `def on_signal(signum, frame)`

What it does: Function on_signal takes signum, frame. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — sensor_path

Found in tools/duty-governor/duty_governor_stage2.py at line 52.

Surface: public API.

Signature: `def sensor_path()`

What it does: Function sensor_path takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning os.path.join(base, entry, "temp1_input"); None.

(Where these words came from: inferred.)

---

## function — read_tctl

Found in tools/duty-governor/duty_governor_stage2.py at line 64.

Surface: public API.

Signature: `def read_tctl(path)`

What it does: Function read_tctl takes path. Its return type is unannotated; the code shows it returning int(fh.read().strip()) / 1000.0; None.

(Where these words came from: inferred.)

---

## function — duty_for_level

Found in tools/duty-governor/duty_governor_stage2.py at line 72.

Surface: public API.

Signature: `def duty_for_level(level)`

What it does: Function duty_for_level takes level. Its return type is unannotated; the code shows it returning ((1.0,) + BAND_DUTIES)[level].

(Where these words came from: inferred.)

---

## function — main

Found in tools/duty-governor/duty_governor_stage2.py at line 76.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 2.

(Where these words came from: inferred.)

---

## function — note_state

Found in tools/duty-governor/duty_governor_stage2.py at line 117.

Surface: public API.

Signature: `def note_state(now)`

What it does: Function note_state takes now. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — set_frozen

Found in tools/duty-governor/duty_governor_stage2.py at line 126.

Surface: public API.

Signature: `def set_frozen(want_frozen, now, why)`

What it does: Function set_frozen takes want_frozen, now, why. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — tctl

Found in tools/duty-governor/duty_test.sh at line 16.

Surface: test.

Signature: `tctl()`

What it does: Function tctl takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — tctl_path

Found in tools/duty-governor/probe_tctl.py at line 24.

Surface: public API.

Signature: `def tctl_path()`

What it does: Function tctl_path takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning os.path.join(d, "temp1_input").

(Where these words came from: inferred.)

---

## function — read_c

Found in tools/duty-governor/probe_tctl.py at line 39.

Surface: public API.

Signature: `def read_c()`

What it does: Function read_c takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning int(f.read().strip()) / 1000.0.

(Where these words came from: inferred.)

---

## function — wait_cool

Found in tools/duty-governor/probe_tctl.py at line 44.

Surface: public API.

Signature: `def wait_cool(limit, cap_s=180)`

What it does: Function wait_cool takes limit, cap_s=180. Its return type is unannotated; the code shows it returning True; read_c() <= limit.

(Where these words came from: inferred.)

---

## function — sample_for

Found in tools/duty-governor/probe_tctl.py at line 53.

Surface: public API.

Signature: `def sample_for(seconds, series, t_ref)`

What it does: Function sample_for takes seconds, series, t_ref. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — crossings

Found in tools/duty-governor/probe_tctl.py at line 60.

Surface: public API.

Signature: `def crossings(series, t_start, thresholds)`

What it does: Function crossings takes series, t_start, thresholds. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — main

Found in tools/duty-governor/probe_tctl.py at line 71.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — enumerate_repos

Found in tools/github-archive/github_archive.py at line 56.

Surface: public API.

Signature: `def enumerate_repos()`

What it does: Function enumerate_repos takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning sorted(names).

(Where these words came from: inferred.)

---

## function — hub_ssh

Found in tools/github-archive/github_archive.py at line 80.

Surface: public API.

Signature: `def hub_ssh(remote_cmd_argv, **kw)`

What it does: Function hub_ssh takes remote_cmd_argv, **kw. Its return type is unannotated; the code shows it returning subprocess.run([FLEET_SSH, HUB] + remote_cmd_arg.

(Where these words came from: inferred.)

---

## function — ship_and_start

Found in tools/github-archive/github_archive.py at line 84.

Surface: public API.

Signature: `def ship_and_start(date, names)`

What it does: Function ship_and_start takes date, names. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — hub_status

Found in tools/github-archive/github_archive.py at line 110.

Surface: public API.

Signature: `def hub_status()`

What it does: Function hub_status takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning None; json.loads(r.stdout).

(Where these words came from: inferred.)

---

## function — worker_running

Found in tools/github-archive/github_archive.py at line 121.

Surface: public API.

Signature: `def worker_running()`

What it does: Function worker_running takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning r.returncode == 0.

(Where these words came from: inferred.)

---

## function — main

Found in tools/github-archive/github_archive.py at line 127.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 3; 0 if not status["failures"] else 2.

(Where these words came from: inferred.)

---

## function — run

Found in tools/github-archive/hub_worker.py at line 40.

Surface: public API.

Signature: `def run(cmd, timeout)`

What it does: Function run takes cmd, timeout. Its return type is unannotated; the code shows it returning subprocess.run(cmd, capture_output=True, text=Tr.

(Where these words came from: inferred.)

---

## function — du_bytes

Found in tools/github-archive/hub_worker.py at line 45.

Surface: public API.

Signature: `def du_bytes(path)`

What it does: Function du_bytes takes path. Its return type is unannotated; the code shows it returning total.

(Where these words came from: inferred.)

---

## function — is_bare_mirror

Found in tools/github-archive/hub_worker.py at line 56.

Surface: public API.

Signature: `def is_bare_mirror(path)`

What it does: Function is_bare_mirror takes path. Its return type is unannotated; the code shows it returning False; r.returncode == 0 and r.stdout.strip() == "true".

(Where these words came from: inferred.)

---

## function — mirror_one

Found in tools/github-archive/hub_worker.py at line 66.

Surface: public API.

Signature: `def mirror_one(name)`

What it does: Clone or update one mirror. Returns a result dict.

(Where these words came from: direct.)

---

## function — main

Found in tools/github-archive/hub_worker.py at line 109.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 3; 0 if not failures else 2.

(Where these words came from: inferred.)

---

## function — check

Found in tools/github-archive/verify_restore.py at line 19.

Surface: public API.

Signature: `def check(date, name)`

What it does: Function check takes date, name. Its return type is unannotated; the code shows it returning f"{name}: CLONE FAILED {r.stderr.strip()[-200:]}; (f"{name}: head={'MATCH' if head == want else 'M.

(Where these words came from: inferred.)

---

## function — share

Found in wavefront-film/build_wavefront.py at line 53.

Surface: public API.

Signature: `def share(key)`

What it does: Function share takes key. Its return type is unannotated; the code shows it returning c["replies"] / c["contact"].

(Where these words came from: inferred.)

---

## function — recip_from_share

Found in wavefront-film/build_wavefront.py at line 57.

Surface: public API.

Signature: `def recip_from_share(s)`

What it does: Function recip_from_share takes s. Its return type is unannotated; the code shows it returning 4 if s >= 0.7 else 3 if s >= 0.4 else 2.

(Where these words came from: inferred.)

---

## function — span_days

Found in wavefront-film/build_wavefront.py at line 60.

Surface: public API.

Signature: `def span_days(key)`

What it does: Function span_days takes key. Its return type is unannotated; the code shows it returning (date.fromisoformat(c["last"]) - date.fromisofor.

(Where these words came from: inferred.)

---

## function — horizon_from_span

Found in wavefront-film/build_wavefront.py at line 64.

Surface: public API.

Signature: `def horizon_from_span(key)`

What it does: Function horizon_from_span takes key. Its return type is unannotated; the code shows it returning 3 if span_days(key) >= 365 else 2.

(Where these words came from: inferred.)

---

## function — region

Found in wavefront-film/build_wavefront.py at line 67.

Surface: public API.

Signature: `def region(vals)`

What it does: The gated reading: desirability is gated on alignment axes, never averaged.

(Where these words came from: direct.)

---

## function — C

Found in wavefront-film/build_wavefront.py at line 75.

Surface: public API.

Signature: `def C(s)`

What it does: Function C takes s. Its return type is unannotated; the code shows it returning tuple(parts).

(Where these words came from: inferred.)

---

## function — entry

Found in wavefront-film/build_wavefront.py at line 157.

Surface: public API.

Signature: `def entry(name, vals, kind, moment, evidence, justifs, strength, extra="", key=None, person=None)`

What it does: Function entry takes name, vals, kind, moment, evidence, justifs, strength, extra="", key=None, person=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — dist

Found in wavefront-film/build_wavefront.py at line 393.

Surface: public API.

Signature: `def dist(a, b)`

What it does: Function dist takes a, b. Its return type is unannotated; the code shows it returning sum(abs(x - y) for x, y in zip(a, b)).

(Where these words came from: inferred.)

---

## function — entry_body

Found in wavefront-film/build_wavefront.py at line 404.

Surface: public API.

Signature: `def entry_body(e)`

What it does: Function entry_body takes e. Its return type is unannotated; the code shows it returning lines.

(Where these words came from: inferred.)

---

## function — rec

Found in wavefront-film/mine_contacts.py at line 21.

Surface: public API.

Signature: `def rec(key)`

What it does: Function rec takes key. Its return type is unannotated; the code shows it returning stats[key].

(Where these words came from: inferred.)

---

## function — gate

Found in wavefront-film/viz/extract_data.py at line 36.

Surface: public API.

Signature: `def gate(vals)`

What it does: Function gate takes vals. Its return type is unannotated; the code shows it returning "shoals"; "hearth"; "open".

(Where these words came from: inferred.)

---

## function — jitter

Found in wavefront-film/viz/extract_data.py at line 43.

Surface: public API.

Signature: `def jitter(name, amp)`

What it does: Function jitter takes name, amp. Its return type is unannotated; the code shows it returning [((h[i] / 255.0) * 2 - 1) * amp for i in range(3.

(Where these words came from: inferred.)

---

## function — yavg_series

Found in wavefront-film/viz10/qa10.py at line 31.

Surface: public API.

Signature: `def yavg_series(path)`

What it does: Function yavg_series takes path. Its return type is unannotated; the code shows it returning [float(x) for x in re.findall(r"lavfi\.signalsta.

(Where these words came from: inferred.)

---

## function — gate

Found in wavefront-film/viz10/qa10.py at line 44.

Surface: public API.

Signature: `def gate(paths, exclude_joints=())`

What it does: Function gate takes paths, exclude_joints=. Its return type is unannotated; the code shows it returning ok.

(Where these words came from: inferred.)

---

## function — palette

Found in wavefront-film/viz10/qa10.py at line 59.

Surface: public API.

Signature: `def palette(path, w=1920, h=1080)`

What it does: Single decode pass, chunked (the per-frame subprocess version

(Where these words came from: direct.)

---

## function — determinism

Found in wavefront-film/viz10/qa10.py at line 80.

Surface: public API.

Signature: `def determinism(a, b)`

What it does: Function determinism takes a, b. Its return type is unannotated; the code shows it returning ha == hb.

(Where these words came from: inferred.)

---

## function — contact

Found in wavefront-film/viz10/qa10.py at line 88.

Surface: public API.

Signature: `def contact(path, outp, every_s=4)`

What it does: Function contact takes path, outp, every_s=4. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — node_pos_all

Found in wavefront-film/viz10/render10.py at line 123.

Surface: public API.

Signature: `def node_pos_all(hf)`

What it does: Function node_pos_all takes hf. Its return type is unannotated; the code shows it returning node_xyz + off.

(Where these words came from: inferred.)

---

## function — smooth

Found in wavefront-film/viz10/render10.py at line 129.

Surface: public API.

Signature: `def smooth(a, b, x)`

What it does: Function smooth takes a, b, x. Its return type is unannotated; the code shows it returning t * t * (3 - 2 * t).

(Where these words came from: inferred.)

---

## function — hop_float

Found in wavefront-film/viz10/render10.py at line 133.

Surface: public API.

Signature: `def hop_float(f)`

What it does: Function hop_float takes f. Its return type is unannotated; the code shows it returning (f - TITLE_F) * HPF.

(Where these words came from: inferred.)

---

## function — node_hop

Found in wavefront-film/viz10/render10.py at line 138.

Surface: public API.

Signature: `def node_hop(f)`

What it does: Function node_hop takes f. Its return type is unannotated; the code shows it returning hf.

(Where these words came from: inferred.)

---

## function — tone_norm

Found in wavefront-film/viz10/render10.py at line 145.

Surface: public API.

Signature: `def tone_norm(h)`

What it does: Function tone_norm takes h. Its return type is unannotated; the code shows it returning float(min(1.0, max(0.0, (tone_sm[int(min(h, N_HO.

(Where these words came from: inferred.)

---

## function — gold_rgb

Found in wavefront-film/viz10/render10.py at line 148.

Surface: public API.

Signature: `def gold_rgb(h, bright=1.0)`

What it does: Function gold_rgb takes h, bright=1.0. Its return type is unannotated; the code shows it returning tuple(int(min(255, x * bright)) for x in c).

(Where these words came from: inferred.)

---

## function — sig_at

Found in wavefront-film/viz10/render10.py at line 175.

Surface: public API.

Signature: `def sig_at(arr, h)`

What it does: Function sig_at takes arr, h. Its return type is unannotated; the code shows it returning float(arr[i] * (1.0 - t) + arr[i + 1] * t).

(Where these words came from: inferred.)

---

## function — spaced_width

Found in wavefront-film/viz10/render10.py at line 201.

Surface: public API.

Signature: `def spaced_width(text, font, tracking)`

What it does: Function spaced_width takes text, font, tracking. Its return type is unannotated; the code shows it returning sum(_dummy.textlength(ch, font=font) for ch in t.

(Where these words came from: inferred.)

---

## function — draw_spaced

Found in wavefront-film/viz10/render10.py at line 203.

Surface: public API.

Signature: `def draw_spaced(dr, cx, y, text, font, fill, tracking=6)`

What it does: Function draw_spaced takes dr, cx, y, text, font, fill, tracking=6. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — draw_spaced_left

Found in wavefront-film/viz10/render10.py at line 208.

Surface: public API.

Signature: `def draw_spaced_left(dr, x, y, text, font, fill, tracking=4)`

What it does: Function draw_spaced_left takes dr, x, y, text, font, fill, tracking=4. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — center_text

Found in wavefront-film/viz10/render10.py at line 212.

Surface: public API.

Signature: `def center_text(dr, cx, y, text, font, fill)`

What it does: Function center_text takes dr, cx, y, text, font, fill. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — drift_at

Found in wavefront-film/viz10/render10.py at line 221.

Surface: public API.

Signature: `def drift_at(hop)`

What it does: Function drift_at takes hop. Its return type is unannotated; the code shows it returning c.

(Where these words came from: inferred.)

---

## function — boil_phase

Found in wavefront-film/viz10/render10.py at line 296.

Surface: public API.

Signature: `def boil_phase(ff)`

What it does: Function boil_phase takes ff. Its return type is unannotated; the code shows it returning float(np.interp(ff, _fax, PHASE_BOIL)).

(Where these words came from: inferred.)

---

## function — song_e

Found in wavefront-film/viz10/render10.py at line 298.

Surface: public API.

Signature: `def song_e(ff)`

What it does: Function song_e takes ff. Its return type is unannotated; the code shows it returning float(np.interp(ff, _fax, SONG_RMS)).

(Where these words came from: inferred.)

---

## function — catmull_rom

Found in wavefront-film/viz10/render10.py at line 312.

Surface: public API.

Signature: `def catmull_rom(p0, p1, p2, p3, t, tension=0.0)`

What it does: Function catmull_rom takes p0, p1, p2, p3, t, tension=0.0. Its return type is unannotated; the code shows it returning ((2 * t3 - 3 * t2 + 1) * p1 + (t3 - 2 * t2 + t) .

(Where these words came from: inferred.)

---

## function — path_at

Found in wavefront-film/viz10/render10.py at line 327.

Surface: public API.

Signature: `def path_at(s)`

What it does: Function path_at takes s. Its return type is unannotated; the code shows it returning PATH[i] * (1 - t) + PATH[i + 1] * t.

(Where these words came from: inferred.)

---

## function — sanitize

Found in wavefront-film/viz10/render10.py at line 339.

Surface: public API.

Signature: `def sanitize(text)`

What it does: Function sanitize takes text. Its return type is unannotated; the code shows it returning re.sub(r"\s+", " ", t).strip().

(Where these words came from: inferred.)

---

## function — wrap

Found in wavefront-film/viz10/render10.py at line 348.

Surface: public API.

Signature: `def wrap(text, font, maxw, maxlines)`

What it does: Function wrap takes text, font, maxw, maxlines. Its return type is unannotated; the code shows it returning lines.

(Where these words came from: inferred.)

---

## function — strip_offset

Found in wavefront-film/viz10/render10.py at line 396.

Surface: public API.

Signature: `def strip_offset(f)`

What it does: Function strip_offset takes f. Its return type is unannotated; the code shows it returning float(np.sum(_DT * np.array([smooth(a, a + 26, f.

(Where these words came from: inferred.)

---

## function — strip_alpha

Found in wavefront-film/viz10/render10.py at line 398.

Surface: public API.

Signature: `def strip_alpha(f)`

What it does: Function strip_alpha takes f. Its return type is unannotated; the code shows it returning smooth(ARR_F[0] - 30, ARR_F[0] + 14, f) * (1.0 -.

(Where these words came from: inferred.)

---

## function — draw_strip

Found in wavefront-film/viz10/render10.py at line 402.

Surface: public API.

Signature: `def draw_strip(dr, f)`

What it does: Function draw_strip takes dr, f. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — highlight_env

Found in wavefront-film/viz10/render10.py at line 529.

Surface: public API.

Signature: `def highlight_env(ff, rec)`

What it does: Function highlight_env takes ff, rec. Its return type is unannotated; the code shows it returning smooth(0, 10, age) * (1.0 - smooth(65, 90, age)).

(Where these words came from: inferred.)

---

## function — card_pulse

Found in wavefront-film/viz10/render10.py at line 536.

Surface: public API.

Signature: `def card_pulse(ff)`

What it does: (env, arr_frame) of the strongest live featured-card arrival pulse.

(Where these words came from: direct.)

---

## function — legend_env

Found in wavefront-film/viz10/render10.py at line 551.

Surface: public API.

Signature: `def legend_env(ff)`

What it does: Function legend_env takes ff. Its return type is unannotated; the code shows it returning smooth(120, 150, ff) * (1.0 - smooth(296, 336, f.

(Where these words came from: inferred.)

---

## function — counter_env

Found in wavefront-film/viz10/render10.py at line 560.

Surface: public API.

Signature: `def counter_env(ff)`

What it does: Function counter_env takes ff. Its return type is unannotated; the code shows it returning smooth(48, 96, ff) * (1.0 - smooth(3380, 3500, f.

(Where these words came from: inferred.)

---

## function — spline_pos_at_hop

Found in wavefront-film/viz10/render10.py at line 591.

Surface: public API.

Signature: `def spline_pos_at_hop(h)`

What it does: Function spline_pos_at_hop takes h. Its return type is unannotated; the code shows it returning V[i] * (1 - t) + V[i + 1] * t.

(Where these words came from: inferred.)

---

## function — theta

Found in wavefront-film/viz10/render10.py at line 599.

Surface: public API.

Signature: `def theta(f)`

What it does: Function theta takes f. Its return type is unannotated; the code shows it returning math.radians(10.0) * smooth(TITLE_F, MAIN_END_F,.

(Where these words came from: inferred.)

---

## function — to_world

Found in wavefront-film/viz10/render10.py at line 601.

Surface: public API.

Signature: `def to_world(pts, f, hop)`

What it does: Function to_world takes pts, f, hop. Its return type is unannotated; the code shows it returning out * breathe + CORE.

(Where these words came from: inferred.)

---

## function — camera

Found in wavefront-film/viz10/render10.py at line 621.

Surface: public API.

Signature: `def camera(ff)`

What it does: Function camera takes ff. Its return type is unannotated; the code shows it returning pos, right, up, fwd.

(Where these words came from: inferred.)

---

## function — project

Found in wavefront-film/viz10/render10.py at line 642.

Surface: public API.

Signature: `def project(pts, cam)`

What it does: Function project takes pts, cam. Its return type is unannotated; the code shows it returning CX_CUR + FOCAL * xc / zsafe, CY_CUR - FOCAL * yc.

(Where these words came from: inferred.)

---

## function — kernel

Found in wavefront-film/viz10/render10.py at line 650.

Surface: public API.

Signature: `def kernel(n, sigma)`

What it does: Function kernel takes n, sigma. Its return type is unannotated; the code shows it returning (g / g.max()).astype(np.float32).

(Where these words came from: inferred.)

---

## function — stamp

Found in wavefront-film/viz10/render10.py at line 657.

Surface: public API.

Signature: `def stamp(buf, sx, sy, kern, color, gain)`

What it does: Function stamp takes buf, sx, sy, kern, color, gain. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — soft_knee

Found in wavefront-film/viz10/render10.py at line 665.

Surface: public API.

Signature: `def soft_knee(a)`

What it does: v9 tone map: identity below 200, smooth tanh shoulder above, so

(Where these words came from: direct.)

---

## function — wind_clock

Found in wavefront-film/viz10/render10.py at line 714.

Surface: public API.

Signature: `def wind_clock(ff): return float(np.interp(ff, _fax, WINDCLK))`

What it does: Function wind_clock takes ff. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — fall_clock

Found in wavefront-film/viz10/render10.py at line 715.

Surface: public API.

Signature: `def fall_clock(ff): return float(np.interp(ff, _fax, FALLCLK))`

What it does: Function fall_clock takes ff. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — snow_layer

Found in wavefront-film/viz10/render10.py at line 751.

Surface: public API.

Signature: `def snow_layer(ff, gain, hush_f, miss_frac)`

What it does: The whole background weather layer for sub-frame ff: a faint

(Where these words came from: direct.)

---

## function — node_color

Found in wavefront-film/viz10/render10.py at line 795.

Surface: public API.

Signature: `def node_color(n)`

What it does: Function node_color takes n. Its return type is unannotated; the code shows it returning NODE_BASE[n["kind"]][0].

(Where these words came from: inferred.)

---

## function — depth_att

Found in wavefront-film/viz10/render10.py at line 797.

Surface: public API.

Signature: `def depth_att(zc)`

What it does: Function depth_att takes zc. Its return type is unannotated; the code shows it returning float(min(1.25, max(0.45, (2.3 / max(float(zc), .

(Where these words came from: inferred.)

---

## function — fog_mix

Found in wavefront-film/viz10/render10.py at line 799.

Surface: public API.

Signature: `def fog_mix(col, zc)`

What it does: Function fog_mix takes col, zc. Its return type is unannotated; the code shows it returning tuple(int(x) for x in (c * (1 - f) + FOG_COL * f.

(Where these words came from: inferred.)

---

## function — bezier_arc9

Found in wavefront-film/viz10/render10.py at line 804.

Surface: public API.

Signature: `def bezier_arc9(p0, p1, u_max=1.0, bow_scale=1.0, jit_mag=0.0)`

What it does: v9 arc: bow and lateral wobble driven by the landing tweet's own

(Where these words came from: direct.)

---

## function — shell_fade

Found in wavefront-film/viz10/render10.py at line 970.

Surface: public API.

Signature: `def shell_fade(sh, hop)`

What it does: Fade-in rule: a shell condenses as its people arrive — invisible

(Where these words came from: direct.)

---

## function — comet_state

Found in wavefront-film/viz10/render10.py at line 979.

Surface: public API.

Signature: `def comet_state(f)`

What it does: Per comet: (tail->head base-space points, fade) at frame f.

(Where these words came from: direct.)

---

## function — render_single

Found in wavefront-film/viz10/render10.py at line 992.

Surface: public API.

Signature: `def render_single(ff)`

What it does: One sub-frame at continuous time ff (float frames — the wrapper

(Where these words came from: direct.)

---

## function — film_grade

Found in wavefront-film/viz10/render10.py at line 1436.

Surface: public API.

Signature: `def film_grade(oa, f)`

What it does: oa: post-bloom float32 (OUT_H, OUT_W, 3), 0..255. Returns the

(Where these words came from: direct.)

---

## function — render_frame

Found in wavefront-film/viz10/render10.py at line 1482.

Surface: public API.

Signature: `def render_frame(f)`

What it does: One output frame: SUBN sub-frames averaged (temporal AA / motion

(Where these words came from: direct.)

---

## function — sanitize

Found in wavefront-film/viz10/score_tweets.py at line 59.

Surface: public API.

Signature: `def sanitize(text)`

What it does: Function sanitize takes text. Its return type is unannotated; the code shows it returning re.sub(r"\s+", " ", t).strip().

(Where these words came from: inferred.)

---

## function — list_structured

Found in wavefront-film/viz10/score_tweets.py at line 88.

Surface: public API.

Signature: `def list_structured(raw)`

What it does: Function list_structured takes raw. Its return type is unannotated; the code shows it returning False.

(Where these words came from: inferred.)

---

## function — is_item

Found in wavefront-film/viz10/score_tweets.py at line 92.

Surface: public API.

Signature: `def is_item(l)`

What it does: Function is_item takes l. Its return type is unannotated; the code shows it returning (l[0] in "-*•·⟡" or re.match(r"^\d+[.)]\s", l).

(Where these words came from: inferred.)

---

## function — score

Found in wavefront-film/viz10/score_tweets.py at line 97.

Surface: public API.

Signature: `def score(text, raw=None)`

What it does: Function score takes text, raw=None. Its return type is unannotated; the code shows it returning 0, {}; max(total, 0), hits.

(Where these words came from: inferred.)

---

## function — main

Found in wavefront-film/viz10/score_tweets.py at line 119.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — load_js

Found in wavefront-film/viz2/extract2.py at line 25.

Surface: public API.

Signature: `def load_js(path)`

What it does: Function load_js takes path. Its return type is unannotated; the code shows it returning json.loads(s[s.index("=") + 1:].strip()).

(Where these words came from: inferred.)

---

## function — shell_offset

Found in wavefront-film/viz2/extract2.py at line 70.

Surface: public API.

Signature: `def shell_offset()`

What it does: Function shell_offset takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning (v * rng.uniform(0.10, 0.30)).tolist().

(Where these words came from: inferred.)

---

## function — smooth

Found in wavefront-film/viz2/render2.py at line 37.

Surface: public API.

Signature: `def smooth(a, b, x)`

What it does: Function smooth takes a, b, x. Its return type is unannotated; the code shows it returning t * t * (3 - 2 * t).

(Where these words came from: inferred.)

---

## function — hsv2rgb

Found in wavefront-film/viz2/render2.py at line 41.

Surface: public API.

Signature: `def hsv2rgb(h, s, v)`

What it does: Function hsv2rgb takes h, s, v. Its return type is unannotated; the code shows it returning [(v, t, p), (q, v, p), (p, v, t), (p, q, v), (t,.

(Where these words came from: inferred.)

---

## function — frame_of

Found in wavefront-film/viz2/render2.py at line 48.

Surface: public API.

Signature: `def frame_of(ts): # archive time -> film frame (linear over main segment)`

What it does: Function frame_of takes ts. It gives back film frame (linear over main segment).

(Where these words came from: inferred.)

---

## function — camera

Found in wavefront-film/viz2/render2.py at line 126.

Surface: public API.

Signature: `def camera(f)`

What it does: Function camera takes f. Its return type is unannotated; the code shows it returning pos, right, up, fwd, lk.

(Where these words came from: inferred.)

---

## function — project

Found in wavefront-film/viz2/render2.py at line 147.

Surface: public API.

Signature: `def project(pts, cam)`

What it does: Function project takes pts, cam. Its return type is unannotated; the code shows it returning sx, sy, zc, ok.

(Where these words came from: inferred.)

---

## function — kernel

Found in wavefront-film/viz2/render2.py at line 156.

Surface: public API.

Signature: `def kernel(n, sigma)`

What it does: Function kernel takes n, sigma. Its return type is unannotated; the code shows it returning (g / g.max()).astype(np.float32).

(Where these words came from: inferred.)

---

## function — stamp

Found in wavefront-film/viz2/render2.py at line 162.

Surface: public API.

Signature: `def stamp(buf, sx, sy, kern, color, gain)`

What it does: Function stamp takes buf, sx, sy, kern, color, gain. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — node_color

Found in wavefront-film/viz2/render2.py at line 173.

Surface: public API.

Signature: `def node_color(n)`

What it does: Function node_color takes n. Its return type is unannotated; the code shows it returning NODE_BASE[n["kind"]][0].

(Where these words came from: inferred.)

---

## function — center_text

Found in wavefront-film/viz2/render2.py at line 180.

Surface: public API.

Signature: `def center_text(dr, cx, y, text, font, fill)`

What it does: Function center_text takes dr, cx, y, text, font, fill. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — render

Found in wavefront-film/viz2/render2.py at line 183.

Surface: public API.

Signature: `def render(f)`

What it does: Function render takes f. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — smooth

Found in wavefront-film/viz3/render3.py at line 56.

Surface: public API.

Signature: `def smooth(a, b, x)`

What it does: Function smooth takes a, b, x. Its return type is unannotated; the code shows it returning t * t * (3 - 2 * t).

(Where these words came from: inferred.)

---

## function — hsv2rgb

Found in wavefront-film/viz3/render3.py at line 60.

Surface: public API.

Signature: `def hsv2rgb(h, s, v)`

What it does: Function hsv2rgb takes h, s, v. Its return type is unannotated; the code shows it returning [(v, t, p), (q, v, p), (p, v, t), (p, q, v), (t,.

(Where these words came from: inferred.)

---

## function — frame_of

Found in wavefront-film/viz3/render3.py at line 65.

Surface: public API.

Signature: `def frame_of(ts)`

What it does: Function frame_of takes ts. Its return type is unannotated; the code shows it returning TITLE_F + (ts - T0) / (T1 - T0) * (MAIN_END_F - .

(Where these words came from: inferred.)

---

## function — window_center

Found in wavefront-film/viz3/render3.py at line 75.

Surface: public API.

Signature: `def window_center(f)`

What it does: Function window_center takes f. Its return type is unannotated; the code shows it returning c.

(Where these words came from: inferred.)

---

## function — warp

Found in wavefront-film/viz3/render3.py at line 81.

Surface: public API.

Signature: `def warp(pts, f)`

What it does: Pull points toward the drifted window: displacement = drift * falloff(dist to window).

(Where these words came from: direct.)

---

## function — catmull_rom

Found in wavefront-film/viz3/render3.py at line 162.

Surface: public API.

Signature: `def catmull_rom(p0, p1, p2, p3, t)`

What it does: Function catmull_rom takes p0, p1, p2, p3, t. Its return type is unannotated; the code shows it returning 0.5 * ((2 * p1) + (-p0 + p2) * t + (2 * p0 - 5 *.

(Where these words came from: inferred.)

---

## function — spline_pos_at

Found in wavefront-film/viz3/render3.py at line 179.

Surface: public API.

Signature: `def spline_pos_at(f)`

What it does: Will's base-space position on the spline at film frame f.

(Where these words came from: direct.)

---

## function — theta

Found in wavefront-film/viz3/render3.py at line 190.

Surface: public API.

Signature: `def theta(f)`

What it does: Function theta takes f. Its return type is unannotated; the code shows it returning math.radians(165.0) * min(1.0, max(0.0, (f - TIT.

(Where these words came from: inferred.)

---

## function — to_world

Found in wavefront-film/viz3/render3.py at line 192.

Surface: public API.

Signature: `def to_world(pts, f, cen)`

What it does: base -> warped (Overton) -> rotated about CORE (galaxy). cen = window_center(f).

(Where these words came from: direct.)

---

## function — camera

Found in wavefront-film/viz3/render3.py at line 211.

Surface: public API.

Signature: `def camera(f, cen)`

What it does: Returns (pos, right, up, fwd). Stateful EMA — call once per frame, ascending.

(Where these words came from: direct.)

---

## function — project

Found in wavefront-film/viz3/render3.py at line 246.

Surface: public API.

Signature: `def project(pts, cam)`

What it does: Function project takes pts, cam. Its return type is unannotated; the code shows it returning sx, sy, zc, ok.

(Where these words came from: inferred.)

---

## function — kernel

Found in wavefront-film/viz3/render3.py at line 255.

Surface: public API.

Signature: `def kernel(n, sigma)`

What it does: Function kernel takes n, sigma. Its return type is unannotated; the code shows it returning (g / g.max()).astype(np.float32).

(Where these words came from: inferred.)

---

## function — stamp

Found in wavefront-film/viz3/render3.py at line 263.

Surface: public API.

Signature: `def stamp(buf, sx, sy, kern, color, gain)`

What it does: Function stamp takes buf, sx, sy, kern, color, gain. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — node_color

Found in wavefront-film/viz3/render3.py at line 274.

Surface: public API.

Signature: `def node_color(n)`

What it does: Function node_color takes n. Its return type is unannotated; the code shows it returning NODE_BASE[n["kind"]][0].

(Where these words came from: inferred.)

---

## function — box_edges

Found in wavefront-film/viz3/render3.py at line 286.

Surface: public API.

Signature: `def box_edges(c, sz)`

What it does: Function box_edges takes c, sz. Its return type is unannotated; the code shows it returning cs, E.

(Where these words came from: inferred.)

---

## function — center_text

Found in wavefront-film/viz3/render3.py at line 298.

Surface: public API.

Signature: `def center_text(dr, cx, y, text, font, fill)`

What it does: Function center_text takes dr, cx, y, text, font, fill. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — bezier_arc

Found in wavefront-film/viz3/render3.py at line 301.

Surface: public API.

Signature: `def bezier_arc(p0, p1, cam, u_max=1.0, n=10)`

What it does: Quadratic bezier from p0 to p1 (world space), bowed off the chord.

(Where these words came from: direct.)

---

## function — rot_only

Found in wavefront-film/viz3/render3.py at line 319.

Surface: public API.

Signature: `def rot_only(pts, f)`

What it does: Function rot_only takes pts, f. Its return type is unannotated; the code shows it returning np.stack([c * q[:, 0] - s * q[:, 1], s * q[:, 0].

(Where these words came from: inferred.)

---

## function — get_cam

Found in wavefront-film/viz3/render3.py at line 325.

Surface: public API.

Signature: `def get_cam(f, cen)`

What it does: Function get_cam takes f, cen. Its return type is unannotated; the code shows it returning cam.

(Where these words came from: inferred.)

---

## function — render

Found in wavefront-film/viz3/render3.py at line 332.

Surface: public API.

Signature: `def render(f)`

What it does: Function render takes f. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — ts_frame

Found in wavefront-film/viz3/render3.py at line 542.

Surface: public API.

Signature: `def ts_frame(y, m, d=1)`

What it does: Function ts_frame takes y, m, d=1. Its return type is unannotated; the code shows it returning int(frame_of(ts)).

(Where these words came from: inferred.)

---

## function — roll

Found in wavefront-film/viz4/extract_tone.py at line 36.

Surface: public API.

Signature: `def roll(a, w)`

What it does: Function roll takes a, w. Its return type is unannotated; the code shows it returning np.convolve(ap, np.ones(w) / w, mode="valid").

(Where these words came from: inferred.)

---

## function — ym

Found in wavefront-film/viz4/extract_tone.py at line 42.

Surface: public API.

Signature: `def ym(ts): return datetime.fromtimestamp(ts, timezone.utc).strftime("%Y-%m")`

What it does: Function ym takes ts. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — tone_norm

Found in wavefront-film/viz4/render4.py at line 47.

Surface: public API.

Signature: `def tone_norm(h)`

What it does: Function tone_norm takes h. Its return type is unannotated; the code shows it returning float(min(1.0, max(0.0, (tone_sm[int(min(h, N_HO.

(Where these words came from: inferred.)

---

## function — gold_rgb

Found in wavefront-film/viz4/render4.py at line 50.

Surface: public API.

Signature: `def gold_rgb(h, bright=1.0)`

What it does: Function gold_rgb takes h, bright=1.0. Its return type is unannotated; the code shows it returning tuple(int(min(255, x * bright)) for x in c).

(Where these words came from: inferred.)

---

## function — smooth

Found in wavefront-film/viz4/render4.py at line 65.

Surface: public API.

Signature: `def smooth(a, b, x)`

What it does: Function smooth takes a, b, x. Its return type is unannotated; the code shows it returning t * t * (3 - 2 * t).

(Where these words came from: inferred.)

---

## function — hop_float

Found in wavefront-film/viz4/render4.py at line 69.

Surface: public API.

Signature: `def hop_float(f)`

What it does: Function hop_float takes f. Its return type is unannotated; the code shows it returning (f - TITLE_F) * HPF.

(Where these words came from: inferred.)

---

## function — drift_at

Found in wavefront-film/viz4/render4.py at line 81.

Surface: public API.

Signature: `def drift_at(hop)`

What it does: Function drift_at takes hop. Its return type is unannotated; the code shows it returning c.

(Where these words came from: inferred.)

---

## function — catmull_rom

Found in wavefront-film/viz4/render4.py at line 129.

Surface: public API.

Signature: `def catmull_rom(p0, p1, p2, p3, t)`

What it does: Function catmull_rom takes p0, p1, p2, p3, t. Its return type is unannotated; the code shows it returning 0.5 * ((2 * p1) + (-p0 + p2) * t + (2 * p0 - 5 *.

(Where these words came from: inferred.)

---

## function — spline_pos_at_hop

Found in wavefront-film/viz4/render4.py at line 143.

Surface: public API.

Signature: `def spline_pos_at_hop(h)`

What it does: Function spline_pos_at_hop takes h. Its return type is unannotated; the code shows it returning V[i] * (1 - t) + V[i + 1] * t.

(Where these words came from: inferred.)

---

## function — theta

Found in wavefront-film/viz4/render4.py at line 151.

Surface: public API.

Signature: `def theta(f)`

What it does: Function theta takes f. Its return type is unannotated; the code shows it returning math.radians(38.0) * smooth(TITLE_F, MAIN_END_F,.

(Where these words came from: inferred.)

---

## function — to_world

Found in wavefront-film/viz4/render4.py at line 153.

Surface: public API.

Signature: `def to_world(pts, f, hop)`

What it does: Function to_world takes pts, f, hop. Its return type is unannotated; the code shows it returning out * breathe + CORE.

(Where these words came from: inferred.)

---

## function — camera

Found in wavefront-film/viz4/render4.py at line 171.

Surface: public API.

Signature: `def camera(f)`

What it does: Function camera takes f. Its return type is unannotated; the code shows it returning pos, right, up, fwd.

(Where these words came from: inferred.)

---

## function — project

Found in wavefront-film/viz4/render4.py at line 186.

Surface: public API.

Signature: `def project(pts, cam)`

What it does: Function project takes pts, cam. Its return type is unannotated; the code shows it returning W / 2 + FOCAL * xc / zsafe, H / 2 - FOCAL * yc /.

(Where these words came from: inferred.)

---

## function — kernel

Found in wavefront-film/viz4/render4.py at line 194.

Surface: public API.

Signature: `def kernel(n, sigma)`

What it does: Function kernel takes n, sigma. Its return type is unannotated; the code shows it returning (g / g.max()).astype(np.float32).

(Where these words came from: inferred.)

---

## function — stamp

Found in wavefront-film/viz4/render4.py at line 201.

Surface: public API.

Signature: `def stamp(buf, sx, sy, kern, color, gain)`

What it does: Function stamp takes buf, sx, sy, kern, color, gain. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — noise_field

Found in wavefront-film/viz4/render4.py at line 211.

Surface: public API.

Signature: `def noise_field(sw, sh)`

What it does: Function noise_field takes sw, sh. Its return type is unannotated; the code shows it returning np.asarray(im, dtype=np.float32) / 255.0.

(Where these words came from: inferred.)

---

## function — node_color

Found in wavefront-film/viz4/render4.py at line 231.

Surface: public API.

Signature: `def node_color(n)`

What it does: Function node_color takes n. Its return type is unannotated; the code shows it returning NODE_BASE[n["kind"]][0].

(Where these words came from: inferred.)

---

## function — bezier_arc

Found in wavefront-film/viz4/render4.py at line 235.

Surface: public API.

Signature: `def bezier_arc(p0, p1, cam, u_max=1.0, n=10)`

What it does: Function bezier_arc takes p0, p1, cam, u_max=1.0, n=10. Its return type is unannotated; the code shows it returning ((1 - ts) ** 2)[:, None] * p0 + (2 * (1 - ts) * .

(Where these words came from: inferred.)

---

## function — hop_of_ts

Found in wavefront-film/viz4/render4.py at line 258.

Surface: public API.

Signature: `def hop_of_ts(ts): return float(np.searchsorted(stab_ts, ts))`

What it does: Function hop_of_ts takes ts. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — center_text

Found in wavefront-film/viz4/render4.py at line 272.

Surface: public API.

Signature: `def center_text(dr, cx, y, text, font, fill)`

What it does: Function center_text takes dr, cx, y, text, font, fill. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — render

Found in wavefront-film/viz4/render4.py at line 277.

Surface: public API.

Signature: `def render(f)`

What it does: Function render takes f. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — frame_of_hop

Found in wavefront-film/viz4/render4.py at line 438.

Surface: public API.

Signature: `def frame_of_hop(h): return int(TITLE_F + h / HPF)`

What it does: Function frame_of_hop takes h. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — tone_norm

Found in wavefront-film/viz4/render4_overton.py at line 53.

Surface: public API.

Signature: `def tone_norm(h)`

What it does: Function tone_norm takes h. Its return type is unannotated; the code shows it returning float(min(1.0, max(0.0, (tone_sm[int(min(h, N_HO.

(Where these words came from: inferred.)

---

## function — gold_rgb

Found in wavefront-film/viz4/render4_overton.py at line 56.

Surface: public API.

Signature: `def gold_rgb(h, bright=1.0)`

What it does: Function gold_rgb takes h, bright=1.0. Its return type is unannotated; the code shows it returning tuple(int(min(255, x * bright)) for x in c).

(Where these words came from: inferred.)

---

## function — smooth

Found in wavefront-film/viz4/render4_overton.py at line 71.

Surface: public API.

Signature: `def smooth(a, b, x)`

What it does: Function smooth takes a, b, x. Its return type is unannotated; the code shows it returning t * t * (3 - 2 * t).

(Where these words came from: inferred.)

---

## function — hop_float

Found in wavefront-film/viz4/render4_overton.py at line 75.

Surface: public API.

Signature: `def hop_float(f)`

What it does: Function hop_float takes f. Its return type is unannotated; the code shows it returning (f - TITLE_F) * HPF.

(Where these words came from: inferred.)

---

## function — build_annual

Found in wavefront-film/viz4/render4_overton.py at line 83.

Surface: public API.

Signature: `def build_annual(stab_ts_arr)`

What it does: Function build_annual takes stab_ts_arr. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — drift_at

Found in wavefront-film/viz4/render4_overton.py at line 103.

Surface: public API.

Signature: `def drift_at(hop)`

What it does: Window center at a hop: c(y-1) gliding to c(y) over the first EASE_HOPS hops of year y.

(Where these words came from: direct.)

---

## function — catmull_rom

Found in wavefront-film/viz4/render4_overton.py at line 156.

Surface: public API.

Signature: `def catmull_rom(p0, p1, p2, p3, t)`

What it does: Function catmull_rom takes p0, p1, p2, p3, t. Its return type is unannotated; the code shows it returning 0.5 * ((2 * p1) + (-p0 + p2) * t + (2 * p0 - 5 *.

(Where these words came from: inferred.)

---

## function — spline_pos_at_hop

Found in wavefront-film/viz4/render4_overton.py at line 170.

Surface: public API.

Signature: `def spline_pos_at_hop(h)`

What it does: Function spline_pos_at_hop takes h. Its return type is unannotated; the code shows it returning V[i] * (1 - t) + V[i + 1] * t.

(Where these words came from: inferred.)

---

## function — theta

Found in wavefront-film/viz4/render4_overton.py at line 178.

Surface: public API.

Signature: `def theta(f)`

What it does: Function theta takes f. Its return type is unannotated; the code shows it returning math.radians(38.0) * smooth(TITLE_F, MAIN_END_F,.

(Where these words came from: inferred.)

---

## function — to_world

Found in wavefront-film/viz4/render4_overton.py at line 180.

Surface: public API.

Signature: `def to_world(pts, f, hop)`

What it does: Function to_world takes pts, f, hop. Its return type is unannotated; the code shows it returning out * breathe + CORE.

(Where these words came from: inferred.)

---

## function — flat_world

Found in wavefront-film/viz4/render4_overton.py at line 192.

Surface: public API.

Signature: `def flat_world(pts, f)`

What it does: Rotation + breathing about CORE only (goalposts/ring ride the galaxy, not the warp).

(Where these words came from: direct.)

---

## function — box_edges

Found in wavefront-film/viz4/render4_overton.py at line 205.

Surface: public API.

Signature: `def box_edges(c, sz)`

What it does: Function box_edges takes c, sz. Its return type is unannotated; the code shows it returning cs, E.

(Where these words came from: inferred.)

---

## function — camera

Found in wavefront-film/viz4/render4_overton.py at line 217.

Surface: public API.

Signature: `def camera(f)`

What it does: Function camera takes f. Its return type is unannotated; the code shows it returning pos, right, up, fwd.

(Where these words came from: inferred.)

---

## function — project

Found in wavefront-film/viz4/render4_overton.py at line 232.

Surface: public API.

Signature: `def project(pts, cam)`

What it does: Function project takes pts, cam. Its return type is unannotated; the code shows it returning W / 2 + FOCAL * xc / zsafe, H / 2 - FOCAL * yc /.

(Where these words came from: inferred.)

---

## function — kernel

Found in wavefront-film/viz4/render4_overton.py at line 240.

Surface: public API.

Signature: `def kernel(n, sigma)`

What it does: Function kernel takes n, sigma. Its return type is unannotated; the code shows it returning (g / g.max()).astype(np.float32).

(Where these words came from: inferred.)

---

## function — stamp

Found in wavefront-film/viz4/render4_overton.py at line 247.

Surface: public API.

Signature: `def stamp(buf, sx, sy, kern, color, gain)`

What it does: Function stamp takes buf, sx, sy, kern, color, gain. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — noise_field

Found in wavefront-film/viz4/render4_overton.py at line 257.

Surface: public API.

Signature: `def noise_field(sw, sh)`

What it does: Function noise_field takes sw, sh. Its return type is unannotated; the code shows it returning np.asarray(im, dtype=np.float32) / 255.0.

(Where these words came from: inferred.)

---

## function — node_color

Found in wavefront-film/viz4/render4_overton.py at line 277.

Surface: public API.

Signature: `def node_color(n)`

What it does: Function node_color takes n. Its return type is unannotated; the code shows it returning NODE_BASE[n["kind"]][0].

(Where these words came from: inferred.)

---

## function — bezier_arc

Found in wavefront-film/viz4/render4_overton.py at line 281.

Surface: public API.

Signature: `def bezier_arc(p0, p1, cam, u_max=1.0, n=10)`

What it does: Function bezier_arc takes p0, p1, cam, u_max=1.0, n=10. Its return type is unannotated; the code shows it returning ((1 - ts) ** 2)[:, None] * p0 + (2 * (1 - ts) * .

(Where these words came from: inferred.)

---

## function — hop_of_ts

Found in wavefront-film/viz4/render4_overton.py at line 304.

Surface: public API.

Signature: `def hop_of_ts(ts): return float(np.searchsorted(stab_ts, ts))`

What it does: Function hop_of_ts takes ts. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — center_text

Found in wavefront-film/viz4/render4_overton.py at line 318.

Surface: public API.

Signature: `def center_text(dr, cx, y, text, font, fill)`

What it does: Function center_text takes dr, cx, y, text, font, fill. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — render

Found in wavefront-film/viz4/render4_overton.py at line 323.

Surface: public API.

Signature: `def render(f)`

What it does: Function render takes f. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — frame_of_hop

Found in wavefront-film/viz4/render4_overton.py at line 501.

Surface: public API.

Signature: `def frame_of_hop(h): return int(TITLE_F + h / HPF)`

What it does: Function frame_of_hop takes h. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — deep_frac

Found in wavefront-film/viz5/render5.py at line 69.

Surface: public API.

Signature: `def deep_frac(hf)`

What it does: Function deep_frac takes hf. Its return type is unannotated; the code shows it returning float((deep_at[i] * (1 - fr) + deep_at[min(i + 1.

(Where these words came from: inferred.)

---

## function — node_pos_all

Found in wavefront-film/viz5/render5.py at line 93.

Surface: public API.

Signature: `def node_pos_all(hf)`

What it does: All node positions at (float) hop hf: anchor + epicycle offset.

(Where these words came from: direct.)

---

## function — node_hop

Found in wavefront-film/viz5/render5.py at line 99.

Surface: public API.

Signature: `def node_hop(f)`

What it does: Hop used for node orbits: follows hop_float, then keeps advancing at

(Where these words came from: direct.)

---

## function — tone_norm

Found in wavefront-film/viz5/render5.py at line 108.

Surface: public API.

Signature: `def tone_norm(h)`

What it does: Function tone_norm takes h. Its return type is unannotated; the code shows it returning float(min(1.0, max(0.0, (tone_sm[int(min(h, N_HO.

(Where these words came from: inferred.)

---

## function — gold_rgb

Found in wavefront-film/viz5/render5.py at line 111.

Surface: public API.

Signature: `def gold_rgb(h, bright=1.0)`

What it does: Function gold_rgb takes h, bright=1.0. Its return type is unannotated; the code shows it returning tuple(int(min(255, x * bright)) for x in c).

(Where these words came from: inferred.)

---

## function — smooth

Found in wavefront-film/viz5/render5.py at line 126.

Surface: public API.

Signature: `def smooth(a, b, x)`

What it does: Function smooth takes a, b, x. Its return type is unannotated; the code shows it returning t * t * (3 - 2 * t).

(Where these words came from: inferred.)

---

## function — hop_float

Found in wavefront-film/viz5/render5.py at line 130.

Surface: public API.

Signature: `def hop_float(f)`

What it does: Function hop_float takes f. Its return type is unannotated; the code shows it returning (f - TITLE_F) * HPF.

(Where these words came from: inferred.)

---

## function — drift_at

Found in wavefront-film/viz5/render5.py at line 142.

Surface: public API.

Signature: `def drift_at(hop)`

What it does: Function drift_at takes hop. Its return type is unannotated; the code shows it returning c.

(Where these words came from: inferred.)

---

## function — catmull_rom

Found in wavefront-film/viz5/render5.py at line 191.

Surface: public API.

Signature: `def catmull_rom(p0, p1, p2, p3, t)`

What it does: Function catmull_rom takes p0, p1, p2, p3, t. Its return type is unannotated; the code shows it returning 0.5 * ((2 * p1) + (-p0 + p2) * t + (2 * p0 - 5 *.

(Where these words came from: inferred.)

---

## function — spline_pos_at_hop

Found in wavefront-film/viz5/render5.py at line 205.

Surface: public API.

Signature: `def spline_pos_at_hop(h)`

What it does: Function spline_pos_at_hop takes h. Its return type is unannotated; the code shows it returning V[i] * (1 - t) + V[i + 1] * t.

(Where these words came from: inferred.)

---

## function — theta

Found in wavefront-film/viz5/render5.py at line 213.

Surface: public API.

Signature: `def theta(f)`

What it does: Function theta takes f. Its return type is unannotated; the code shows it returning math.radians(10.0) * smooth(TITLE_F, MAIN_END_F,.

(Where these words came from: inferred.)

---

## function — to_world

Found in wavefront-film/viz5/render5.py at line 217.

Surface: public API.

Signature: `def to_world(pts, f, hop)`

What it does: Function to_world takes pts, f, hop. Its return type is unannotated; the code shows it returning out * breathe + CORE.

(Where these words came from: inferred.)

---

## function — camera

Found in wavefront-film/viz5/render5.py at line 235.

Surface: public API.

Signature: `def camera(f)`

What it does: Function camera takes f. Its return type is unannotated; the code shows it returning pos, right, up, fwd.

(Where these words came from: inferred.)

---

## function — project

Found in wavefront-film/viz5/render5.py at line 250.

Surface: public API.

Signature: `def project(pts, cam)`

What it does: Function project takes pts, cam. Its return type is unannotated; the code shows it returning W / 2 + FOCAL * xc / zsafe, H / 2 - FOCAL * yc /.

(Where these words came from: inferred.)

---

## function — kernel

Found in wavefront-film/viz5/render5.py at line 258.

Surface: public API.

Signature: `def kernel(n, sigma)`

What it does: Function kernel takes n, sigma. Its return type is unannotated; the code shows it returning (g / g.max()).astype(np.float32).

(Where these words came from: inferred.)

---

## function — stamp

Found in wavefront-film/viz5/render5.py at line 265.

Surface: public API.

Signature: `def stamp(buf, sx, sy, kern, color, gain)`

What it does: Function stamp takes buf, sx, sy, kern, color, gain. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — noise_field

Found in wavefront-film/viz5/render5.py at line 275.

Surface: public API.

Signature: `def noise_field(sw, sh)`

What it does: Function noise_field takes sw, sh. Its return type is unannotated; the code shows it returning np.asarray(im, dtype=np.float32) / 255.0.

(Where these words came from: inferred.)

---

## function — node_color

Found in wavefront-film/viz5/render5.py at line 295.

Surface: public API.

Signature: `def node_color(n)`

What it does: Function node_color takes n. Its return type is unannotated; the code shows it returning NODE_BASE[n["kind"]][0].

(Where these words came from: inferred.)

---

## function — bezier_arc

Found in wavefront-film/viz5/render5.py at line 299.

Surface: public API.

Signature: `def bezier_arc(p0, p1, cam, u_max=1.0, n=10)`

What it does: Function bezier_arc takes p0, p1, cam, u_max=1.0, n=10. Its return type is unannotated; the code shows it returning ((1 - ts) ** 2)[:, None] * p0 + (2 * (1 - ts) * .

(Where these words came from: inferred.)

---

## function — hop_of_ts

Found in wavefront-film/viz5/render5.py at line 322.

Surface: public API.

Signature: `def hop_of_ts(ts): return float(np.searchsorted(stab_ts, ts))`

What it does: Function hop_of_ts takes ts. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — center_text

Found in wavefront-film/viz5/render5.py at line 336.

Surface: public API.

Signature: `def center_text(dr, cx, y, text, font, fill)`

What it does: Function center_text takes dr, cx, y, text, font, fill. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — render

Found in wavefront-film/viz5/render5.py at line 341.

Surface: public API.

Signature: `def render(f)`

What it does: Function render takes f. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — frame_of_hop

Found in wavefront-film/viz5/render5.py at line 505.

Surface: public API.

Signature: `def frame_of_hop(h): return int(TITLE_F + h / HPF)`

What it does: Function frame_of_hop takes h. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — node_pos_all

Found in wavefront-film/viz6/render6.py at line 87.

Surface: public API.

Signature: `def node_pos_all(hf)`

What it does: Function node_pos_all takes hf. Its return type is unannotated; the code shows it returning node_xyz + off.

(Where these words came from: inferred.)

---

## function — smooth

Found in wavefront-film/viz6/render6.py at line 93.

Surface: public API.

Signature: `def smooth(a, b, x)`

What it does: Function smooth takes a, b, x. Its return type is unannotated; the code shows it returning t * t * (3 - 2 * t).

(Where these words came from: inferred.)

---

## function — hop_float

Found in wavefront-film/viz6/render6.py at line 97.

Surface: public API.

Signature: `def hop_float(f)`

What it does: Function hop_float takes f. Its return type is unannotated; the code shows it returning (f - TITLE_F) * HPF.

(Where these words came from: inferred.)

---

## function — node_hop

Found in wavefront-film/viz6/render6.py at line 102.

Surface: public API.

Signature: `def node_hop(f)`

What it does: Function node_hop takes f. Its return type is unannotated; the code shows it returning hf.

(Where these words came from: inferred.)

---

## function — tone_norm

Found in wavefront-film/viz6/render6.py at line 109.

Surface: public API.

Signature: `def tone_norm(h)`

What it does: Function tone_norm takes h. Its return type is unannotated; the code shows it returning float(min(1.0, max(0.0, (tone_sm[int(min(h, N_HO.

(Where these words came from: inferred.)

---

## function — gold_rgb

Found in wavefront-film/viz6/render6.py at line 112.

Surface: public API.

Signature: `def gold_rgb(h, bright=1.0)`

What it does: Function gold_rgb takes h, bright=1.0. Its return type is unannotated; the code shows it returning tuple(int(min(255, x * bright)) for x in c).

(Where these words came from: inferred.)

---

## function — spaced_width

Found in wavefront-film/viz6/render6.py at line 130.

Surface: public API.

Signature: `def spaced_width(text, font, tracking)`

What it does: Function spaced_width takes text, font, tracking. Its return type is unannotated; the code shows it returning sum(_dummy.textlength(ch, font=font) for ch in t.

(Where these words came from: inferred.)

---

## function — draw_spaced

Found in wavefront-film/viz6/render6.py at line 132.

Surface: public API.

Signature: `def draw_spaced(dr, cx, y, text, font, fill, tracking=6)`

What it does: Function draw_spaced takes dr, cx, y, text, font, fill, tracking=6. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — draw_spaced_left

Found in wavefront-film/viz6/render6.py at line 137.

Surface: public API.

Signature: `def draw_spaced_left(dr, x, y, text, font, fill, tracking=4)`

What it does: Function draw_spaced_left takes dr, x, y, text, font, fill, tracking=4. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — center_text

Found in wavefront-film/viz6/render6.py at line 141.

Surface: public API.

Signature: `def center_text(dr, cx, y, text, font, fill)`

What it does: Function center_text takes dr, cx, y, text, font, fill. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — drift_at

Found in wavefront-film/viz6/render6.py at line 150.

Surface: public API.

Signature: `def drift_at(hop)`

What it does: Function drift_at takes hop. Its return type is unannotated; the code shows it returning c.

(Where these words came from: inferred.)

---

## function — catmull_rom

Found in wavefront-film/viz6/render6.py at line 218.

Surface: public API.

Signature: `def catmull_rom(p0, p1, p2, p3, t)`

What it does: Function catmull_rom takes p0, p1, p2, p3, t. Its return type is unannotated; the code shows it returning 0.5 * ((2 * p1) + (-p0 + p2) * t + (2 * p0 - 5 *.

(Where these words came from: inferred.)

---

## function — path_at

Found in wavefront-film/viz6/render6.py at line 232.

Surface: public API.

Signature: `def path_at(s)`

What it does: Function path_at takes s. Its return type is unannotated; the code shows it returning PATH[i] * (1 - t) + PATH[i + 1] * t.

(Where these words came from: inferred.)

---

## function — sanitize

Found in wavefront-film/viz6/render6.py at line 244.

Surface: public API.

Signature: `def sanitize(text)`

What it does: Function sanitize takes text. Its return type is unannotated; the code shows it returning re.sub(r"\s+", " ", t).strip().

(Where these words came from: inferred.)

---

## function — wrap

Found in wavefront-film/viz6/render6.py at line 249.

Surface: public API.

Signature: `def wrap(text, font, maxw, maxlines)`

What it does: Function wrap takes text, font, maxw, maxlines. Its return type is unannotated; the code shows it returning lines.

(Where these words came from: inferred.)

---

## function — spline_pos_at_hop

Found in wavefront-film/viz6/render6.py at line 343.

Surface: public API.

Signature: `def spline_pos_at_hop(h)`

What it does: Function spline_pos_at_hop takes h. Its return type is unannotated; the code shows it returning V[i] * (1 - t) + V[i + 1] * t.

(Where these words came from: inferred.)

---

## function — theta

Found in wavefront-film/viz6/render6.py at line 351.

Surface: public API.

Signature: `def theta(f)`

What it does: Function theta takes f. Its return type is unannotated; the code shows it returning math.radians(10.0) * smooth(TITLE_F, MAIN_END_F,.

(Where these words came from: inferred.)

---

## function — to_world

Found in wavefront-film/viz6/render6.py at line 353.

Surface: public API.

Signature: `def to_world(pts, f, hop)`

What it does: Function to_world takes pts, f, hop. Its return type is unannotated; the code shows it returning out * breathe + CORE.

(Where these words came from: inferred.)

---

## function — camera

Found in wavefront-film/viz6/render6.py at line 371.

Surface: public API.

Signature: `def camera(f)`

What it does: Function camera takes f. Its return type is unannotated; the code shows it returning pos, right, up, fwd.

(Where these words came from: inferred.)

---

## function — project

Found in wavefront-film/viz6/render6.py at line 387.

Surface: public API.

Signature: `def project(pts, cam)`

What it does: Function project takes pts, cam. Its return type is unannotated; the code shows it returning W / 2 + FOCAL * xc / zsafe, H / 2 - FOCAL * yc /.

(Where these words came from: inferred.)

---

## function — kernel

Found in wavefront-film/viz6/render6.py at line 395.

Surface: public API.

Signature: `def kernel(n, sigma)`

What it does: Function kernel takes n, sigma. Its return type is unannotated; the code shows it returning (g / g.max()).astype(np.float32).

(Where these words came from: inferred.)

---

## function — stamp

Found in wavefront-film/viz6/render6.py at line 402.

Surface: public API.

Signature: `def stamp(buf, sx, sy, kern, color, gain)`

What it does: Function stamp takes buf, sx, sy, kern, color, gain. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — noise_field

Found in wavefront-film/viz6/render6.py at line 412.

Surface: public API.

Signature: `def noise_field(sw, sh)`

What it does: Function noise_field takes sw, sh. Its return type is unannotated; the code shows it returning np.asarray(im, dtype=np.float32) / 255.0.

(Where these words came from: inferred.)

---

## function — node_color

Found in wavefront-film/viz6/render6.py at line 431.

Surface: public API.

Signature: `def node_color(n)`

What it does: Function node_color takes n. Its return type is unannotated; the code shows it returning NODE_BASE[n["kind"]][0].

(Where these words came from: inferred.)

---

## function — bezier_arc

Found in wavefront-film/viz6/render6.py at line 435.

Surface: public API.

Signature: `def bezier_arc(p0, p1, cam, u_max=1.0, n=10)`

What it does: Function bezier_arc takes p0, p1, cam, u_max=1.0, n=10. Its return type is unannotated; the code shows it returning ((1 - ts) ** 2)[:, None] * p0 + (2 * (1 - ts) * .

(Where these words came from: inferred.)

---

## function — comet_state

Found in wavefront-film/viz6/render6.py at line 456.

Surface: public API.

Signature: `def comet_state(f)`

What it does: Per comet: (tail->head base-space points, fade) at frame f.

(Where these words came from: direct.)

---

## function — card_active

Found in wavefront-film/viz6/render6.py at line 469.

Surface: public API.

Signature: `def card_active(f)`

What it does: Function card_active takes f. Its return type is unannotated; the code shows it returning any(o["start"] <= f < o["end"] for o in TOP20).

(Where these words came from: inferred.)

---

## function — render

Found in wavefront-film/viz6/render6.py at line 472.

Surface: public API.

Signature: `def render(f)`

What it does: Function render takes f. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — frame_of_hop

Found in wavefront-film/viz6/render6.py at line 659.

Surface: public API.

Signature: `def frame_of_hop(h): return int(TITLE_F + h / HPF)`

What it does: Function frame_of_hop takes h. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — node_pos_all

Found in wavefront-film/viz7/render7.py at line 95.

Surface: public API.

Signature: `def node_pos_all(hf)`

What it does: Function node_pos_all takes hf. Its return type is unannotated; the code shows it returning node_xyz + off.

(Where these words came from: inferred.)

---

## function — smooth

Found in wavefront-film/viz7/render7.py at line 101.

Surface: public API.

Signature: `def smooth(a, b, x)`

What it does: Function smooth takes a, b, x. Its return type is unannotated; the code shows it returning t * t * (3 - 2 * t).

(Where these words came from: inferred.)

---

## function — hop_float

Found in wavefront-film/viz7/render7.py at line 105.

Surface: public API.

Signature: `def hop_float(f)`

What it does: Function hop_float takes f. Its return type is unannotated; the code shows it returning (f - TITLE_F) * HPF.

(Where these words came from: inferred.)

---

## function — node_hop

Found in wavefront-film/viz7/render7.py at line 110.

Surface: public API.

Signature: `def node_hop(f)`

What it does: Function node_hop takes f. Its return type is unannotated; the code shows it returning hf.

(Where these words came from: inferred.)

---

## function — tone_norm

Found in wavefront-film/viz7/render7.py at line 117.

Surface: public API.

Signature: `def tone_norm(h)`

What it does: Function tone_norm takes h. Its return type is unannotated; the code shows it returning float(min(1.0, max(0.0, (tone_sm[int(min(h, N_HO.

(Where these words came from: inferred.)

---

## function — gold_rgb

Found in wavefront-film/viz7/render7.py at line 120.

Surface: public API.

Signature: `def gold_rgb(h, bright=1.0)`

What it does: Function gold_rgb takes h, bright=1.0. Its return type is unannotated; the code shows it returning tuple(int(min(255, x * bright)) for x in c).

(Where these words came from: inferred.)

---

## function — spaced_width

Found in wavefront-film/viz7/render7.py at line 140.

Surface: public API.

Signature: `def spaced_width(text, font, tracking)`

What it does: Function spaced_width takes text, font, tracking. Its return type is unannotated; the code shows it returning sum(_dummy.textlength(ch, font=font) for ch in t.

(Where these words came from: inferred.)

---

## function — draw_spaced

Found in wavefront-film/viz7/render7.py at line 142.

Surface: public API.

Signature: `def draw_spaced(dr, cx, y, text, font, fill, tracking=6)`

What it does: Function draw_spaced takes dr, cx, y, text, font, fill, tracking=6. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — draw_spaced_left

Found in wavefront-film/viz7/render7.py at line 147.

Surface: public API.

Signature: `def draw_spaced_left(dr, x, y, text, font, fill, tracking=4)`

What it does: Function draw_spaced_left takes dr, x, y, text, font, fill, tracking=4. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — center_text

Found in wavefront-film/viz7/render7.py at line 151.

Surface: public API.

Signature: `def center_text(dr, cx, y, text, font, fill)`

What it does: Function center_text takes dr, cx, y, text, font, fill. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — drift_at

Found in wavefront-film/viz7/render7.py at line 160.

Surface: public API.

Signature: `def drift_at(hop)`

What it does: Function drift_at takes hop. Its return type is unannotated; the code shows it returning c.

(Where these words came from: inferred.)

---

## function — catmull_rom

Found in wavefront-film/viz7/render7.py at line 228.

Surface: public API.

Signature: `def catmull_rom(p0, p1, p2, p3, t, tension=0.0)`

What it does: Function catmull_rom takes p0, p1, p2, p3, t, tension=0.0. Its return type is unannotated; the code shows it returning ((2 * t3 - 3 * t2 + 1) * p1 + (t3 - 2 * t2 + t) .

(Where these words came from: inferred.)

---

## function — path_at

Found in wavefront-film/viz7/render7.py at line 246.

Surface: public API.

Signature: `def path_at(s)`

What it does: Function path_at takes s. Its return type is unannotated; the code shows it returning PATH[i] * (1 - t) + PATH[i + 1] * t.

(Where these words came from: inferred.)

---

## function — sanitize

Found in wavefront-film/viz7/render7.py at line 258.

Surface: public API.

Signature: `def sanitize(text)`

What it does: Function sanitize takes text. Its return type is unannotated; the code shows it returning re.sub(r"\s+", " ", t).strip().

(Where these words came from: inferred.)

---

## function — wrap

Found in wavefront-film/viz7/render7.py at line 263.

Surface: public API.

Signature: `def wrap(text, font, maxw, maxlines)`

What it does: Function wrap takes text, font, maxw, maxlines. Its return type is unannotated; the code shows it returning lines.

(Where these words came from: inferred.)

---

## function — strip_offset

Found in wavefront-film/viz7/render7.py at line 309.

Surface: public API.

Signature: `def strip_offset(f)`

What it does: Function strip_offset takes f. Its return type is unannotated; the code shows it returning float(np.sum(_DT * np.array([smooth(a, a + 26, f.

(Where these words came from: inferred.)

---

## function — strip_alpha

Found in wavefront-film/viz7/render7.py at line 314.

Surface: public API.

Signature: `def strip_alpha(f)`

What it does: Function strip_alpha takes f. Its return type is unannotated; the code shows it returning 1.0 - smooth(CTA_F0, CTA_F0 + 75, f)   # gone be.

(Where these words came from: inferred.)

---

## function — draw_strip

Found in wavefront-film/viz7/render7.py at line 316.

Surface: public API.

Signature: `def draw_strip(dr, f)`

What it does: Function draw_strip takes dr, f. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — spline_pos_at_hop

Found in wavefront-film/viz7/render7.py at line 427.

Surface: public API.

Signature: `def spline_pos_at_hop(h)`

What it does: Function spline_pos_at_hop takes h. Its return type is unannotated; the code shows it returning V[i] * (1 - t) + V[i + 1] * t.

(Where these words came from: inferred.)

---

## function — theta

Found in wavefront-film/viz7/render7.py at line 435.

Surface: public API.

Signature: `def theta(f)`

What it does: Function theta takes f. Its return type is unannotated; the code shows it returning math.radians(10.0) * smooth(TITLE_F, MAIN_END_F,.

(Where these words came from: inferred.)

---

## function — to_world

Found in wavefront-film/viz7/render7.py at line 437.

Surface: public API.

Signature: `def to_world(pts, f, hop)`

What it does: Function to_world takes pts, f, hop. Its return type is unannotated; the code shows it returning out * breathe + CORE.

(Where these words came from: inferred.)

---

## function — camera

Found in wavefront-film/viz7/render7.py at line 455.

Surface: public API.

Signature: `def camera(f)`

What it does: Function camera takes f. Its return type is unannotated; the code shows it returning pos, right, up, fwd.

(Where these words came from: inferred.)

---

## function — project

Found in wavefront-film/viz7/render7.py at line 471.

Surface: public API.

Signature: `def project(pts, cam)`

What it does: Function project takes pts, cam. Its return type is unannotated; the code shows it returning CX_CUR + FOCAL * xc / zsafe, H / 2 - FOCAL * yc .

(Where these words came from: inferred.)

---

## function — kernel

Found in wavefront-film/viz7/render7.py at line 479.

Surface: public API.

Signature: `def kernel(n, sigma)`

What it does: Function kernel takes n, sigma. Its return type is unannotated; the code shows it returning (g / g.max()).astype(np.float32).

(Where these words came from: inferred.)

---

## function — stamp

Found in wavefront-film/viz7/render7.py at line 486.

Surface: public API.

Signature: `def stamp(buf, sx, sy, kern, color, gain)`

What it does: Function stamp takes buf, sx, sy, kern, color, gain. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — noise_field

Found in wavefront-film/viz7/render7.py at line 498.

Surface: public API.

Signature: `def noise_field(sw, sh)`

What it does: Function noise_field takes sw, sh. Its return type is unannotated; the code shows it returning np.asarray(im, dtype=np.float32) / 255.0.

(Where these words came from: inferred.)

---

## function — node_color

Found in wavefront-film/viz7/render7.py at line 517.

Surface: public API.

Signature: `def node_color(n)`

What it does: Function node_color takes n. Its return type is unannotated; the code shows it returning NODE_BASE[n["kind"]][0].

(Where these words came from: inferred.)

---

## function — bezier_arc

Found in wavefront-film/viz7/render7.py at line 520.

Surface: public API.

Signature: `def bezier_arc(p0, p1, cam, u_max=1.0, n=18)`

What it does: Function bezier_arc takes p0, p1, cam, u_max=1.0, n=18. Its return type is unannotated; the code shows it returning ((1 - ts) ** 2)[:, None] * p0 + (2 * (1 - ts) * .

(Where these words came from: inferred.)

---

## function — comet_state

Found in wavefront-film/viz7/render7.py at line 540.

Surface: public API.

Signature: `def comet_state(f)`

What it does: Per comet: (tail->head base-space points, fade) at frame f.

(Where these words came from: direct.)

---

## function — render

Found in wavefront-film/viz7/render7.py at line 553.

Surface: public API.

Signature: `def render(f)`

What it does: Function render takes f. Its return type is unannotated; the code shows it returning out.resize((OUT_W, OUT_H), Image.LANCZOS).

(Where these words came from: inferred.)

---

## function — frame_of_hop

Found in wavefront-film/viz7/render7.py at line 722.

Surface: public API.

Signature: `def frame_of_hop(h): return int(TITLE_F + h / HPF)`

What it does: Function frame_of_hop takes h. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — tokens_of

Found in wavefront-film/viz8/extract_features.py at line 93.

Surface: public API.

Signature: `def tokens_of(text)`

What it does: Function tokens_of takes text. Its return type is unannotated; the code shows it returning TOK.findall(t.lower()).

(Where these words came from: inferred.)

---

## function — pct

Found in wavefront-film/viz8/extract_features.py at line 128.

Surface: public API.

Signature: `def pct(a, ps=(1, 2, 5, 25, 50, 75, 95, 98, 99))`

What it does: Function pct takes a, ps=. Its return type is unannotated; the code shows it returning {f"p{p}": round(float(np.percentile(a, p)), 4) f.

(Where these words came from: inferred.)

---

## function — norm

Found in wavefront-film/viz8/extract_features.py at line 131.

Surface: public API.

Signature: `def norm(a)`

What it does: Function norm takes a. Its return type is unannotated; the code shows it returning np.clip((a - lo) / max(1e-12, hi - lo), 0.0, 1.0.

(Where these words came from: inferred.)

---

## function — smooth2

Found in wavefront-film/viz8/extract_features.py at line 140.

Surface: public API.

Signature: `def smooth2(a, w1=201, w2=51)`

What it does: Function smooth2 takes a, w1=201, w2=51. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — roll

Found in wavefront-film/viz8/extract_features.py at line 141.

Surface: public API.

Signature: `def roll(x, w)`

What it does: Function roll takes x, w. Its return type is unannotated; the code shows it returning np.convolve(xp, np.ones(w) / w, mode="valid")[:l.

(Where these words came from: inferred.)

---

## function — node_pos_all

Found in wavefront-film/viz8/render8.py at line 86.

Surface: public API.

Signature: `def node_pos_all(hf)`

What it does: Function node_pos_all takes hf. Its return type is unannotated; the code shows it returning node_xyz + off.

(Where these words came from: inferred.)

---

## function — smooth

Found in wavefront-film/viz8/render8.py at line 92.

Surface: public API.

Signature: `def smooth(a, b, x)`

What it does: Function smooth takes a, b, x. Its return type is unannotated; the code shows it returning t * t * (3 - 2 * t).

(Where these words came from: inferred.)

---

## function — hop_float

Found in wavefront-film/viz8/render8.py at line 96.

Surface: public API.

Signature: `def hop_float(f)`

What it does: Function hop_float takes f. Its return type is unannotated; the code shows it returning (f - TITLE_F) * HPF.

(Where these words came from: inferred.)

---

## function — node_hop

Found in wavefront-film/viz8/render8.py at line 101.

Surface: public API.

Signature: `def node_hop(f)`

What it does: Function node_hop takes f. Its return type is unannotated; the code shows it returning hf.

(Where these words came from: inferred.)

---

## function — tone_norm

Found in wavefront-film/viz8/render8.py at line 108.

Surface: public API.

Signature: `def tone_norm(h)`

What it does: Function tone_norm takes h. Its return type is unannotated; the code shows it returning float(min(1.0, max(0.0, (tone_sm[int(min(h, N_HO.

(Where these words came from: inferred.)

---

## function — gold_rgb

Found in wavefront-film/viz8/render8.py at line 111.

Surface: public API.

Signature: `def gold_rgb(h, bright=1.0)`

What it does: Function gold_rgb takes h, bright=1.0. Its return type is unannotated; the code shows it returning tuple(int(min(255, x * bright)) for x in c).

(Where these words came from: inferred.)

---

## function — sig_at

Found in wavefront-film/viz8/render8.py at line 133.

Surface: public API.

Signature: `def sig_at(arr, h)`

What it does: Function sig_at takes arr, h. Its return type is unannotated; the code shows it returning float(arr[i] * (1.0 - t) + arr[i + 1] * t).

(Where these words came from: inferred.)

---

## function — spaced_width

Found in wavefront-film/viz8/render8.py at line 159.

Surface: public API.

Signature: `def spaced_width(text, font, tracking)`

What it does: Function spaced_width takes text, font, tracking. Its return type is unannotated; the code shows it returning sum(_dummy.textlength(ch, font=font) for ch in t.

(Where these words came from: inferred.)

---

## function — draw_spaced

Found in wavefront-film/viz8/render8.py at line 161.

Surface: public API.

Signature: `def draw_spaced(dr, cx, y, text, font, fill, tracking=6)`

What it does: Function draw_spaced takes dr, cx, y, text, font, fill, tracking=6. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — draw_spaced_left

Found in wavefront-film/viz8/render8.py at line 166.

Surface: public API.

Signature: `def draw_spaced_left(dr, x, y, text, font, fill, tracking=4)`

What it does: Function draw_spaced_left takes dr, x, y, text, font, fill, tracking=4. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — center_text

Found in wavefront-film/viz8/render8.py at line 170.

Surface: public API.

Signature: `def center_text(dr, cx, y, text, font, fill)`

What it does: Function center_text takes dr, cx, y, text, font, fill. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — drift_at

Found in wavefront-film/viz8/render8.py at line 179.

Surface: public API.

Signature: `def drift_at(hop)`

What it does: Function drift_at takes hop. Its return type is unannotated; the code shows it returning c.

(Where these words came from: inferred.)

---

## function — catmull_rom

Found in wavefront-film/viz8/render8.py at line 247.

Surface: public API.

Signature: `def catmull_rom(p0, p1, p2, p3, t, tension=0.0)`

What it does: Function catmull_rom takes p0, p1, p2, p3, t, tension=0.0. Its return type is unannotated; the code shows it returning ((2 * t3 - 3 * t2 + 1) * p1 + (t3 - 2 * t2 + t) .

(Where these words came from: inferred.)

---

## function — path_at

Found in wavefront-film/viz8/render8.py at line 265.

Surface: public API.

Signature: `def path_at(s)`

What it does: Function path_at takes s. Its return type is unannotated; the code shows it returning PATH[i] * (1 - t) + PATH[i + 1] * t.

(Where these words came from: inferred.)

---

## function — sanitize

Found in wavefront-film/viz8/render8.py at line 277.

Surface: public API.

Signature: `def sanitize(text)`

What it does: Function sanitize takes text. Its return type is unannotated; the code shows it returning re.sub(r"\s+", " ", t).strip().

(Where these words came from: inferred.)

---

## function — wrap

Found in wavefront-film/viz8/render8.py at line 282.

Surface: public API.

Signature: `def wrap(text, font, maxw, maxlines)`

What it does: Function wrap takes text, font, maxw, maxlines. Its return type is unannotated; the code shows it returning lines.

(Where these words came from: inferred.)

---

## function — strip_offset

Found in wavefront-film/viz8/render8.py at line 328.

Surface: public API.

Signature: `def strip_offset(f)`

What it does: Function strip_offset takes f. Its return type is unannotated; the code shows it returning float(np.sum(_DT * np.array([smooth(a, a + 26, f.

(Where these words came from: inferred.)

---

## function — strip_alpha

Found in wavefront-film/viz8/render8.py at line 333.

Surface: public API.

Signature: `def strip_alpha(f)`

What it does: Function strip_alpha takes f. Its return type is unannotated; the code shows it returning 1.0 - smooth(CTA_F0, CTA_F0 + 75, f)   # gone be.

(Where these words came from: inferred.)

---

## function — draw_strip

Found in wavefront-film/viz8/render8.py at line 335.

Surface: public API.

Signature: `def draw_strip(dr, f)`

What it does: Function draw_strip takes dr, f. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — spline_pos_at_hop

Found in wavefront-film/viz8/render8.py at line 446.

Surface: public API.

Signature: `def spline_pos_at_hop(h)`

What it does: Function spline_pos_at_hop takes h. Its return type is unannotated; the code shows it returning V[i] * (1 - t) + V[i + 1] * t.

(Where these words came from: inferred.)

---

## function — theta

Found in wavefront-film/viz8/render8.py at line 454.

Surface: public API.

Signature: `def theta(f)`

What it does: Function theta takes f. Its return type is unannotated; the code shows it returning math.radians(10.0) * smooth(TITLE_F, MAIN_END_F,.

(Where these words came from: inferred.)

---

## function — to_world

Found in wavefront-film/viz8/render8.py at line 456.

Surface: public API.

Signature: `def to_world(pts, f, hop)`

What it does: Function to_world takes pts, f, hop. Its return type is unannotated; the code shows it returning out * breathe + CORE.

(Where these words came from: inferred.)

---

## function — camera

Found in wavefront-film/viz8/render8.py at line 474.

Surface: public API.

Signature: `def camera(f)`

What it does: Function camera takes f. Its return type is unannotated; the code shows it returning pos, right, up, fwd.

(Where these words came from: inferred.)

---

## function — project

Found in wavefront-film/viz8/render8.py at line 490.

Surface: public API.

Signature: `def project(pts, cam)`

What it does: Function project takes pts, cam. Its return type is unannotated; the code shows it returning CX_CUR + FOCAL * xc / zsafe, H / 2 - FOCAL * yc .

(Where these words came from: inferred.)

---

## function — kernel

Found in wavefront-film/viz8/render8.py at line 498.

Surface: public API.

Signature: `def kernel(n, sigma)`

What it does: Function kernel takes n, sigma. Its return type is unannotated; the code shows it returning (g / g.max()).astype(np.float32).

(Where these words came from: inferred.)

---

## function — stamp

Found in wavefront-film/viz8/render8.py at line 505.

Surface: public API.

Signature: `def stamp(buf, sx, sy, kern, color, gain)`

What it does: Function stamp takes buf, sx, sy, kern, color, gain. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — noise_field

Found in wavefront-film/viz8/render8.py at line 517.

Surface: public API.

Signature: `def noise_field(sw, sh)`

What it does: Function noise_field takes sw, sh. Its return type is unannotated; the code shows it returning np.asarray(im, dtype=np.float32) / 255.0.

(Where these words came from: inferred.)

---

## function — node_color

Found in wavefront-film/viz8/render8.py at line 544.

Surface: public API.

Signature: `def node_color(n)`

What it does: Function node_color takes n. Its return type is unannotated; the code shows it returning NODE_BASE[n["kind"]][0].

(Where these words came from: inferred.)

---

## function — bezier_arc

Found in wavefront-film/viz8/render8.py at line 547.

Surface: public API.

Signature: `def bezier_arc(p0, p1, cam, u_max=1.0, n=18)`

What it does: Function bezier_arc takes p0, p1, cam, u_max=1.0, n=18. Its return type is unannotated; the code shows it returning ((1 - ts) ** 2)[:, None] * p0 + (2 * (1 - ts) * .

(Where these words came from: inferred.)

---

## function — bezier_arc8

Found in wavefront-film/viz8/render8.py at line 556.

Surface: public API.

Signature: `def bezier_arc8(p0, p1, cam, u_max, bow_scale, jit_mag)`

What it does: v8 arc: bow and lateral wobble driven by the landing tweet's own

(Where these words came from: direct.)

---

## function — comet_state

Found in wavefront-film/viz8/render8.py at line 587.

Surface: public API.

Signature: `def comet_state(f)`

What it does: Per comet: (tail->head base-space points, fade) at frame f.

(Where these words came from: direct.)

---

## function — render

Found in wavefront-film/viz8/render8.py at line 600.

Surface: public API.

Signature: `def render(f)`

What it does: Function render takes f. Its return type is unannotated; the code shows it returning out.resize((OUT_W, OUT_H), Image.LANCZOS).

(Where these words came from: inferred.)

---

## function — frame_of_hop

Found in wavefront-film/viz8/render8.py at line 815.

Surface: public API.

Signature: `def frame_of_hop(h): return int(TITLE_F + h / HPF)`

What it does: Function frame_of_hop takes h. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — run

Found in wavefront-film/viz9/assemble_reel.py at line 27.

Surface: public API.

Signature: `def run(cmd)`

What it does: Function run takes cmd. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — load_js

Found in wavefront-film/viz9/derive_clusters.py at line 40.

Surface: public API.

Signature: `def load_js(path)`

What it does: Function load_js takes path. Its return type is unannotated; the code shows it returning json.loads(s[s.index("=") + 1:].strip()).

(Where these words came from: inferred.)

---

## function — dbscan

Found in wavefront-film/viz9/derive_clusters.py at line 92.

Surface: public API.

Signature: `def dbscan(eps, ms)`

What it does: Function dbscan takes eps, ms. Its return type is unannotated; the code shows it returning lab.

(Where these words came from: inferred.)

---

## function — components

Found in wavefront-film/viz9/derive_clusters.py at line 119.

Surface: public API.

Signature: `def components(tau)`

What it does: Function components takes tau. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — find

Found in wavefront-film/viz9/derive_clusters.py at line 121.

Surface: public API.

Signature: `def find(a)`

What it does: Function find takes a. Its return type is unannotated; the code shows it returning a.

(Where these words came from: inferred.)

---

## function — pearson

Found in wavefront-film/viz9/extract_features9.py at line 54.

Surface: public API.

Signature: `def pearson(a, b): return float(np.corrcoef(a, b)[0, 1])`

What it does: Function pearson takes a, b. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — spearman

Found in wavefront-film/viz9/extract_features9.py at line 55.

Surface: public API.

Signature: `def spearman(a, b)`

What it does: Function spearman takes a, b. Its return type is unannotated; the code shows it returning float(np.corrcoef(ra, rb)[0, 1]).

(Where these words came from: inferred.)

---

## function — norm

Found in wavefront-film/viz9/extract_features9.py at line 77.

Surface: public API.

Signature: `def norm(a)`

What it does: Function norm takes a. Its return type is unannotated; the code shows it returning np.clip((a - lo) / max(1e-12, hi - lo), 0.0, 1.0.

(Where these words came from: inferred.)

---

## function — smooth2

Found in wavefront-film/viz9/extract_features9.py at line 81.

Surface: public API.

Signature: `def smooth2(a, w1=201, w2=51)`

What it does: Function smooth2 takes a, w1=201, w2=51. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — roll

Found in wavefront-film/viz9/extract_features9.py at line 82.

Surface: public API.

Signature: `def roll(x, w)`

What it does: Function roll takes x, w. Its return type is unannotated; the code shows it returning np.convolve(xp, np.ones(w) / w, mode="valid")[:l.

(Where these words came from: inferred.)

---

## function — run

Found in wavefront-film/viz9/fix_reel_audio.py at line 25.

Surface: public API.

Signature: `def run(cmd, **kw)`

What it does: Function run takes cmd, **kw. Its return type is unannotated; the code shows it returning r.

(Where these words came from: inferred.)

---

## function — yavg_series

Found in wavefront-film/viz9/qa9.py at line 29.

Surface: public API.

Signature: `def yavg_series(path)`

What it does: Function yavg_series takes path. Its return type is unannotated; the code shows it returning [float(x) for x in re.findall(r"lavfi\.signalsta.

(Where these words came from: inferred.)

---

## function — gate

Found in wavefront-film/viz9/qa9.py at line 42.

Surface: public API.

Signature: `def gate(paths, exclude_joints=())`

What it does: Function gate takes paths, exclude_joints=. Its return type is unannotated; the code shows it returning ok.

(Where these words came from: inferred.)

---

## function — palette

Found in wavefront-film/viz9/qa9.py at line 57.

Surface: public API.

Signature: `def palette(path, w=1920, h=1080)`

What it does: Single decode pass, chunked (the per-frame subprocess version

(Where these words came from: direct.)

---

## function — determinism

Found in wavefront-film/viz9/qa9.py at line 78.

Surface: public API.

Signature: `def determinism(a, b)`

What it does: Function determinism takes a, b. Its return type is unannotated; the code shows it returning ha == hb.

(Where these words came from: inferred.)

---

## function — contact

Found in wavefront-film/viz9/qa9.py at line 86.

Surface: public API.

Signature: `def contact(path, outp, every_s=4)`

What it does: Function contact takes path, outp, every_s=4. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — node_pos_all

Found in wavefront-film/viz9/render9.py at line 119.

Surface: public API.

Signature: `def node_pos_all(hf)`

What it does: Function node_pos_all takes hf. Its return type is unannotated; the code shows it returning node_xyz + off.

(Where these words came from: inferred.)

---

## function — smooth

Found in wavefront-film/viz9/render9.py at line 125.

Surface: public API.

Signature: `def smooth(a, b, x)`

What it does: Function smooth takes a, b, x. Its return type is unannotated; the code shows it returning t * t * (3 - 2 * t).

(Where these words came from: inferred.)

---

## function — hop_float

Found in wavefront-film/viz9/render9.py at line 129.

Surface: public API.

Signature: `def hop_float(f)`

What it does: Function hop_float takes f. Its return type is unannotated; the code shows it returning (f - TITLE_F) * HPF.

(Where these words came from: inferred.)

---

## function — node_hop

Found in wavefront-film/viz9/render9.py at line 134.

Surface: public API.

Signature: `def node_hop(f)`

What it does: Function node_hop takes f. Its return type is unannotated; the code shows it returning hf.

(Where these words came from: inferred.)

---

## function — tone_norm

Found in wavefront-film/viz9/render9.py at line 141.

Surface: public API.

Signature: `def tone_norm(h)`

What it does: Function tone_norm takes h. Its return type is unannotated; the code shows it returning float(min(1.0, max(0.0, (tone_sm[int(min(h, N_HO.

(Where these words came from: inferred.)

---

## function — gold_rgb

Found in wavefront-film/viz9/render9.py at line 144.

Surface: public API.

Signature: `def gold_rgb(h, bright=1.0)`

What it does: Function gold_rgb takes h, bright=1.0. Its return type is unannotated; the code shows it returning tuple(int(min(255, x * bright)) for x in c).

(Where these words came from: inferred.)

---

## function — sig_at

Found in wavefront-film/viz9/render9.py at line 171.

Surface: public API.

Signature: `def sig_at(arr, h)`

What it does: Function sig_at takes arr, h. Its return type is unannotated; the code shows it returning float(arr[i] * (1.0 - t) + arr[i + 1] * t).

(Where these words came from: inferred.)

---

## function — spaced_width

Found in wavefront-film/viz9/render9.py at line 197.

Surface: public API.

Signature: `def spaced_width(text, font, tracking)`

What it does: Function spaced_width takes text, font, tracking. Its return type is unannotated; the code shows it returning sum(_dummy.textlength(ch, font=font) for ch in t.

(Where these words came from: inferred.)

---

## function — draw_spaced

Found in wavefront-film/viz9/render9.py at line 199.

Surface: public API.

Signature: `def draw_spaced(dr, cx, y, text, font, fill, tracking=6)`

What it does: Function draw_spaced takes dr, cx, y, text, font, fill, tracking=6. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — draw_spaced_left

Found in wavefront-film/viz9/render9.py at line 204.

Surface: public API.

Signature: `def draw_spaced_left(dr, x, y, text, font, fill, tracking=4)`

What it does: Function draw_spaced_left takes dr, x, y, text, font, fill, tracking=4. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — center_text

Found in wavefront-film/viz9/render9.py at line 208.

Surface: public API.

Signature: `def center_text(dr, cx, y, text, font, fill)`

What it does: Function center_text takes dr, cx, y, text, font, fill. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — drift_at

Found in wavefront-film/viz9/render9.py at line 217.

Surface: public API.

Signature: `def drift_at(hop)`

What it does: Function drift_at takes hop. Its return type is unannotated; the code shows it returning c.

(Where these words came from: inferred.)

---

## function — boil_phase

Found in wavefront-film/viz9/render9.py at line 292.

Surface: public API.

Signature: `def boil_phase(ff)`

What it does: Function boil_phase takes ff. Its return type is unannotated; the code shows it returning float(np.interp(ff, _fax, PHASE_BOIL)).

(Where these words came from: inferred.)

---

## function — song_e

Found in wavefront-film/viz9/render9.py at line 294.

Surface: public API.

Signature: `def song_e(ff)`

What it does: Function song_e takes ff. Its return type is unannotated; the code shows it returning float(np.interp(ff, _fax, SONG_RMS)).

(Where these words came from: inferred.)

---

## function — catmull_rom

Found in wavefront-film/viz9/render9.py at line 308.

Surface: public API.

Signature: `def catmull_rom(p0, p1, p2, p3, t, tension=0.0)`

What it does: Function catmull_rom takes p0, p1, p2, p3, t, tension=0.0. Its return type is unannotated; the code shows it returning ((2 * t3 - 3 * t2 + 1) * p1 + (t3 - 2 * t2 + t) .

(Where these words came from: inferred.)

---

## function — path_at

Found in wavefront-film/viz9/render9.py at line 323.

Surface: public API.

Signature: `def path_at(s)`

What it does: Function path_at takes s. Its return type is unannotated; the code shows it returning PATH[i] * (1 - t) + PATH[i + 1] * t.

(Where these words came from: inferred.)

---

## function — sanitize

Found in wavefront-film/viz9/render9.py at line 335.

Surface: public API.

Signature: `def sanitize(text)`

What it does: Function sanitize takes text. Its return type is unannotated; the code shows it returning re.sub(r"\s+", " ", t).strip().

(Where these words came from: inferred.)

---

## function — wrap

Found in wavefront-film/viz9/render9.py at line 340.

Surface: public API.

Signature: `def wrap(text, font, maxw, maxlines)`

What it does: Function wrap takes text, font, maxw, maxlines. Its return type is unannotated; the code shows it returning lines.

(Where these words came from: inferred.)

---

## function — strip_offset

Found in wavefront-film/viz9/render9.py at line 388.

Surface: public API.

Signature: `def strip_offset(f)`

What it does: Function strip_offset takes f. Its return type is unannotated; the code shows it returning float(np.sum(_DT * np.array([smooth(a, a + 26, f.

(Where these words came from: inferred.)

---

## function — strip_alpha

Found in wavefront-film/viz9/render9.py at line 390.

Surface: public API.

Signature: `def strip_alpha(f)`

What it does: Function strip_alpha takes f. Its return type is unannotated; the code shows it returning smooth(ARR_F[0] - 30, ARR_F[0] + 14, f) * (1.0 -.

(Where these words came from: inferred.)

---

## function — draw_strip

Found in wavefront-film/viz9/render9.py at line 394.

Surface: public API.

Signature: `def draw_strip(dr, f)`

What it does: Function draw_strip takes dr, f. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — highlight_env

Found in wavefront-film/viz9/render9.py at line 521.

Surface: public API.

Signature: `def highlight_env(ff, rec)`

What it does: Function highlight_env takes ff, rec. Its return type is unannotated; the code shows it returning smooth(0, 10, age) * (1.0 - smooth(65, 90, age)).

(Where these words came from: inferred.)

---

## function — card_pulse

Found in wavefront-film/viz9/render9.py at line 525.

Surface: public API.

Signature: `def card_pulse(ff)`

What it does: (env, arr_frame) of the strongest live Most-Echoed arrival pulse.

(Where these words came from: direct.)

---

## function — legend_env

Found in wavefront-film/viz9/render9.py at line 540.

Surface: public API.

Signature: `def legend_env(ff)`

What it does: Function legend_env takes ff. Its return type is unannotated; the code shows it returning smooth(120, 150, ff) * (1.0 - smooth(296, 336, f.

(Where these words came from: inferred.)

---

## function — counter_env

Found in wavefront-film/viz9/render9.py at line 549.

Surface: public API.

Signature: `def counter_env(ff)`

What it does: Function counter_env takes ff. Its return type is unannotated; the code shows it returning smooth(48, 96, ff) * (1.0 - smooth(3380, 3500, f.

(Where these words came from: inferred.)

---

## function — spline_pos_at_hop

Found in wavefront-film/viz9/render9.py at line 580.

Surface: public API.

Signature: `def spline_pos_at_hop(h)`

What it does: Function spline_pos_at_hop takes h. Its return type is unannotated; the code shows it returning V[i] * (1 - t) + V[i + 1] * t.

(Where these words came from: inferred.)

---

## function — theta

Found in wavefront-film/viz9/render9.py at line 588.

Surface: public API.

Signature: `def theta(f)`

What it does: Function theta takes f. Its return type is unannotated; the code shows it returning math.radians(10.0) * smooth(TITLE_F, MAIN_END_F,.

(Where these words came from: inferred.)

---

## function — to_world

Found in wavefront-film/viz9/render9.py at line 590.

Surface: public API.

Signature: `def to_world(pts, f, hop)`

What it does: Function to_world takes pts, f, hop. Its return type is unannotated; the code shows it returning out * breathe + CORE.

(Where these words came from: inferred.)

---

## function — camera

Found in wavefront-film/viz9/render9.py at line 610.

Surface: public API.

Signature: `def camera(ff)`

What it does: Function camera takes ff. Its return type is unannotated; the code shows it returning pos, right, up, fwd.

(Where these words came from: inferred.)

---

## function — project

Found in wavefront-film/viz9/render9.py at line 626.

Surface: public API.

Signature: `def project(pts, cam)`

What it does: Function project takes pts, cam. Its return type is unannotated; the code shows it returning CX_CUR + FOCAL * xc / zsafe, H / 2 - FOCAL * yc .

(Where these words came from: inferred.)

---

## function — kernel

Found in wavefront-film/viz9/render9.py at line 634.

Surface: public API.

Signature: `def kernel(n, sigma)`

What it does: Function kernel takes n, sigma. Its return type is unannotated; the code shows it returning (g / g.max()).astype(np.float32).

(Where these words came from: inferred.)

---

## function — stamp

Found in wavefront-film/viz9/render9.py at line 641.

Surface: public API.

Signature: `def stamp(buf, sx, sy, kern, color, gain)`

What it does: Function stamp takes buf, sx, sy, kern, color, gain. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — soft_knee

Found in wavefront-film/viz9/render9.py at line 649.

Surface: public API.

Signature: `def soft_knee(a)`

What it does: v9 tone map: identity below 200, smooth tanh shoulder above, so

(Where these words came from: direct.)

---

## function — wind_clock

Found in wavefront-film/viz9/render9.py at line 681.

Surface: public API.

Signature: `def wind_clock(ff): return float(np.interp(ff, _fax, WINDCLK))`

What it does: Function wind_clock takes ff. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — fall_clock

Found in wavefront-film/viz9/render9.py at line 682.

Surface: public API.

Signature: `def fall_clock(ff): return float(np.interp(ff, _fax, FALLCLK))`

What it does: Function fall_clock takes ff. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — snow_layer

Found in wavefront-film/viz9/render9.py at line 718.

Surface: public API.

Signature: `def snow_layer(ff, gain, hush_f, miss_frac)`

What it does: The whole background weather layer for sub-frame ff: a faint

(Where these words came from: direct.)

---

## function — node_color

Found in wavefront-film/viz9/render9.py at line 762.

Surface: public API.

Signature: `def node_color(n)`

What it does: Function node_color takes n. Its return type is unannotated; the code shows it returning NODE_BASE[n["kind"]][0].

(Where these words came from: inferred.)

---

## function — depth_att

Found in wavefront-film/viz9/render9.py at line 764.

Surface: public API.

Signature: `def depth_att(zc)`

What it does: Function depth_att takes zc. Its return type is unannotated; the code shows it returning float(min(1.25, max(0.45, (2.3 / max(float(zc), .

(Where these words came from: inferred.)

---

## function — fog_mix

Found in wavefront-film/viz9/render9.py at line 766.

Surface: public API.

Signature: `def fog_mix(col, zc)`

What it does: Function fog_mix takes col, zc. Its return type is unannotated; the code shows it returning tuple(int(x) for x in (c * (1 - f) + FOG_COL * f.

(Where these words came from: inferred.)

---

## function — bezier_arc9

Found in wavefront-film/viz9/render9.py at line 771.

Surface: public API.

Signature: `def bezier_arc9(p0, p1, u_max=1.0, bow_scale=1.0, jit_mag=0.0)`

What it does: v9 arc: bow and lateral wobble driven by the landing tweet's own

(Where these words came from: direct.)

---

## function — shell_fade

Found in wavefront-film/viz9/render9.py at line 861.

Surface: public API.

Signature: `def shell_fade(sh, hop)`

What it does: Fade-in rule: a shell condenses as its people arrive — invisible

(Where these words came from: direct.)

---

## function — comet_state

Found in wavefront-film/viz9/render9.py at line 870.

Surface: public API.

Signature: `def comet_state(f)`

What it does: Per comet: (tail->head base-space points, fade) at frame f.

(Where these words came from: direct.)

---

## function — render_single

Found in wavefront-film/viz9/render9.py at line 883.

Surface: public API.

Signature: `def render_single(ff)`

What it does: One sub-frame at continuous time ff (float frames — the wrapper

(Where these words came from: direct.)

---

## function — film_grade

Found in wavefront-film/viz9/render9.py at line 1308.

Surface: public API.

Signature: `def film_grade(oa, f)`

What it does: oa: post-bloom float32 (OUT_H, OUT_W, 3), 0..255. Returns the

(Where these words came from: direct.)

---

## function — render_frame

Found in wavefront-film/viz9/render9.py at line 1339.

Surface: public API.

Signature: `def render_frame(f)`

What it does: One output frame: SUBN sub-frames averaged (temporal AA / motion

(Where these words came from: direct.)

---

## function — node_pos_all

Found in wavefront-film/viz9/render9_hookonly.py at line 93.

Surface: public API.

Signature: `def node_pos_all(hf)`

What it does: Function node_pos_all takes hf. Its return type is unannotated; the code shows it returning node_xyz + off.

(Where these words came from: inferred.)

---

## function — smooth

Found in wavefront-film/viz9/render9_hookonly.py at line 99.

Surface: public API.

Signature: `def smooth(a, b, x)`

What it does: Function smooth takes a, b, x. Its return type is unannotated; the code shows it returning t * t * (3 - 2 * t).

(Where these words came from: inferred.)

---

## function — hop_float

Found in wavefront-film/viz9/render9_hookonly.py at line 103.

Surface: public API.

Signature: `def hop_float(f)`

What it does: Function hop_float takes f. Its return type is unannotated; the code shows it returning (f - TITLE_F) * HPF.

(Where these words came from: inferred.)

---

## function — node_hop

Found in wavefront-film/viz9/render9_hookonly.py at line 108.

Surface: public API.

Signature: `def node_hop(f)`

What it does: Function node_hop takes f. Its return type is unannotated; the code shows it returning hf.

(Where these words came from: inferred.)

---

## function — tone_norm

Found in wavefront-film/viz9/render9_hookonly.py at line 115.

Surface: public API.

Signature: `def tone_norm(h)`

What it does: Function tone_norm takes h. Its return type is unannotated; the code shows it returning float(min(1.0, max(0.0, (tone_sm[int(min(h, N_HO.

(Where these words came from: inferred.)

---

## function — gold_rgb

Found in wavefront-film/viz9/render9_hookonly.py at line 118.

Surface: public API.

Signature: `def gold_rgb(h, bright=1.0)`

What it does: Function gold_rgb takes h, bright=1.0. Its return type is unannotated; the code shows it returning tuple(int(min(255, x * bright)) for x in c).

(Where these words came from: inferred.)

---

## function — sig_at

Found in wavefront-film/viz9/render9_hookonly.py at line 145.

Surface: public API.

Signature: `def sig_at(arr, h)`

What it does: Function sig_at takes arr, h. Its return type is unannotated; the code shows it returning float(arr[i] * (1.0 - t) + arr[i + 1] * t).

(Where these words came from: inferred.)

---

## function — spaced_width

Found in wavefront-film/viz9/render9_hookonly.py at line 170.

Surface: public API.

Signature: `def spaced_width(text, font, tracking)`

What it does: Function spaced_width takes text, font, tracking. Its return type is unannotated; the code shows it returning sum(_dummy.textlength(ch, font=font) for ch in t.

(Where these words came from: inferred.)

---

## function — draw_spaced

Found in wavefront-film/viz9/render9_hookonly.py at line 172.

Surface: public API.

Signature: `def draw_spaced(dr, cx, y, text, font, fill, tracking=6)`

What it does: Function draw_spaced takes dr, cx, y, text, font, fill, tracking=6. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — draw_spaced_left

Found in wavefront-film/viz9/render9_hookonly.py at line 177.

Surface: public API.

Signature: `def draw_spaced_left(dr, x, y, text, font, fill, tracking=4)`

What it does: Function draw_spaced_left takes dr, x, y, text, font, fill, tracking=4. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — center_text

Found in wavefront-film/viz9/render9_hookonly.py at line 181.

Surface: public API.

Signature: `def center_text(dr, cx, y, text, font, fill)`

What it does: Function center_text takes dr, cx, y, text, font, fill. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — drift_at

Found in wavefront-film/viz9/render9_hookonly.py at line 190.

Surface: public API.

Signature: `def drift_at(hop)`

What it does: Function drift_at takes hop. Its return type is unannotated; the code shows it returning c.

(Where these words came from: inferred.)

---

## function — boil_phase

Found in wavefront-film/viz9/render9_hookonly.py at line 265.

Surface: public API.

Signature: `def boil_phase(ff)`

What it does: Function boil_phase takes ff. Its return type is unannotated; the code shows it returning float(np.interp(ff, _fax, PHASE_BOIL)).

(Where these words came from: inferred.)

---

## function — song_e

Found in wavefront-film/viz9/render9_hookonly.py at line 267.

Surface: public API.

Signature: `def song_e(ff)`

What it does: Function song_e takes ff. Its return type is unannotated; the code shows it returning float(np.interp(ff, _fax, SONG_RMS)).

(Where these words came from: inferred.)

---

## function — catmull_rom

Found in wavefront-film/viz9/render9_hookonly.py at line 281.

Surface: public API.

Signature: `def catmull_rom(p0, p1, p2, p3, t, tension=0.0)`

What it does: Function catmull_rom takes p0, p1, p2, p3, t, tension=0.0. Its return type is unannotated; the code shows it returning ((2 * t3 - 3 * t2 + 1) * p1 + (t3 - 2 * t2 + t) .

(Where these words came from: inferred.)

---

## function — path_at

Found in wavefront-film/viz9/render9_hookonly.py at line 296.

Surface: public API.

Signature: `def path_at(s)`

What it does: Function path_at takes s. Its return type is unannotated; the code shows it returning PATH[i] * (1 - t) + PATH[i + 1] * t.

(Where these words came from: inferred.)

---

## function — sanitize

Found in wavefront-film/viz9/render9_hookonly.py at line 308.

Surface: public API.

Signature: `def sanitize(text)`

What it does: Function sanitize takes text. Its return type is unannotated; the code shows it returning re.sub(r"\s+", " ", t).strip().

(Where these words came from: inferred.)

---

## function — wrap

Found in wavefront-film/viz9/render9_hookonly.py at line 313.

Surface: public API.

Signature: `def wrap(text, font, maxw, maxlines)`

What it does: Function wrap takes text, font, maxw, maxlines. Its return type is unannotated; the code shows it returning lines.

(Where these words came from: inferred.)

---

## function — strip_offset

Found in wavefront-film/viz9/render9_hookonly.py at line 361.

Surface: public API.

Signature: `def strip_offset(f)`

What it does: Function strip_offset takes f. Its return type is unannotated; the code shows it returning float(np.sum(_DT * np.array([smooth(a, a + 26, f.

(Where these words came from: inferred.)

---

## function — strip_alpha

Found in wavefront-film/viz9/render9_hookonly.py at line 363.

Surface: public API.

Signature: `def strip_alpha(f)`

What it does: Function strip_alpha takes f. Its return type is unannotated; the code shows it returning smooth(ARR_F[0] - 30, ARR_F[0] + 14, f) * (1.0 -.

(Where these words came from: inferred.)

---

## function — draw_strip

Found in wavefront-film/viz9/render9_hookonly.py at line 367.

Surface: public API.

Signature: `def draw_strip(dr, f)`

What it does: Function draw_strip takes dr, f. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — highlight_env

Found in wavefront-film/viz9/render9_hookonly.py at line 494.

Surface: public API.

Signature: `def highlight_env(ff, rec)`

What it does: Function highlight_env takes ff, rec. Its return type is unannotated; the code shows it returning smooth(0, 10, age) * (1.0 - smooth(65, 90, age)).

(Where these words came from: inferred.)

---

## function — card_pulse

Found in wavefront-film/viz9/render9_hookonly.py at line 498.

Surface: public API.

Signature: `def card_pulse(ff)`

What it does: (env, arr_frame) of the strongest live Most-Echoed arrival pulse.

(Where these words came from: direct.)

---

## function — legend_env

Found in wavefront-film/viz9/render9_hookonly.py at line 513.

Surface: public API.

Signature: `def legend_env(ff)`

What it does: Function legend_env takes ff. Its return type is unannotated; the code shows it returning smooth(120, 150, ff) * (1.0 - smooth(296, 336, f.

(Where these words came from: inferred.)

---

## function — spline_pos_at_hop

Found in wavefront-film/viz9/render9_hookonly.py at line 544.

Surface: public API.

Signature: `def spline_pos_at_hop(h)`

What it does: Function spline_pos_at_hop takes h. Its return type is unannotated; the code shows it returning V[i] * (1 - t) + V[i + 1] * t.

(Where these words came from: inferred.)

---

## function — theta

Found in wavefront-film/viz9/render9_hookonly.py at line 552.

Surface: public API.

Signature: `def theta(f)`

What it does: Function theta takes f. Its return type is unannotated; the code shows it returning math.radians(10.0) * smooth(TITLE_F, MAIN_END_F,.

(Where these words came from: inferred.)

---

## function — to_world

Found in wavefront-film/viz9/render9_hookonly.py at line 554.

Surface: public API.

Signature: `def to_world(pts, f, hop)`

What it does: Function to_world takes pts, f, hop. Its return type is unannotated; the code shows it returning out * breathe + CORE.

(Where these words came from: inferred.)

---

## function — camera

Found in wavefront-film/viz9/render9_hookonly.py at line 574.

Surface: public API.

Signature: `def camera(ff)`

What it does: Function camera takes ff. Its return type is unannotated; the code shows it returning pos, right, up, fwd.

(Where these words came from: inferred.)

---

## function — project

Found in wavefront-film/viz9/render9_hookonly.py at line 590.

Surface: public API.

Signature: `def project(pts, cam)`

What it does: Function project takes pts, cam. Its return type is unannotated; the code shows it returning CX_CUR + FOCAL * xc / zsafe, H / 2 - FOCAL * yc .

(Where these words came from: inferred.)

---

## function — kernel

Found in wavefront-film/viz9/render9_hookonly.py at line 598.

Surface: public API.

Signature: `def kernel(n, sigma)`

What it does: Function kernel takes n, sigma. Its return type is unannotated; the code shows it returning (g / g.max()).astype(np.float32).

(Where these words came from: inferred.)

---

## function — stamp

Found in wavefront-film/viz9/render9_hookonly.py at line 605.

Surface: public API.

Signature: `def stamp(buf, sx, sy, kern, color, gain)`

What it does: Function stamp takes buf, sx, sy, kern, color, gain. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — soft_knee

Found in wavefront-film/viz9/render9_hookonly.py at line 613.

Surface: public API.

Signature: `def soft_knee(a)`

What it does: v9 tone map: identity below 200, smooth tanh shoulder above, so

(Where these words came from: direct.)

---

## function — noise_field

Found in wavefront-film/viz9/render9_hookonly.py at line 623.

Surface: public API.

Signature: `def noise_field(sw, sh)`

What it does: Function noise_field takes sw, sh. Its return type is unannotated; the code shows it returning np.asarray(im, dtype=np.float32) / 255.0.

(Where these words came from: inferred.)

---

## function — node_color

Found in wavefront-film/viz9/render9_hookonly.py at line 663.

Surface: public API.

Signature: `def node_color(n)`

What it does: Function node_color takes n. Its return type is unannotated; the code shows it returning NODE_BASE[n["kind"]][0].

(Where these words came from: inferred.)

---

## function — depth_att

Found in wavefront-film/viz9/render9_hookonly.py at line 665.

Surface: public API.

Signature: `def depth_att(zc)`

What it does: Function depth_att takes zc. Its return type is unannotated; the code shows it returning float(min(1.25, max(0.45, (2.3 / max(float(zc), .

(Where these words came from: inferred.)

---

## function — fog_mix

Found in wavefront-film/viz9/render9_hookonly.py at line 667.

Surface: public API.

Signature: `def fog_mix(col, zc)`

What it does: Function fog_mix takes col, zc. Its return type is unannotated; the code shows it returning tuple(int(x) for x in (c * (1 - f) + FOG_COL * f.

(Where these words came from: inferred.)

---

## function — bezier_arc9

Found in wavefront-film/viz9/render9_hookonly.py at line 672.

Surface: public API.

Signature: `def bezier_arc9(p0, p1, u_max=1.0, bow_scale=1.0, jit_mag=0.0)`

What it does: v9 arc: bow and lateral wobble driven by the landing tweet's own

(Where these words came from: direct.)

---

## function — comet_state

Found in wavefront-film/viz9/render9_hookonly.py at line 704.

Surface: public API.

Signature: `def comet_state(f)`

What it does: Per comet: (tail->head base-space points, fade) at frame f.

(Where these words came from: direct.)

---

## function — render_single

Found in wavefront-film/viz9/render9_hookonly.py at line 717.

Surface: public API.

Signature: `def render_single(ff)`

What it does: One sub-frame at continuous time ff (float frames — the wrapper

(Where these words came from: direct.)

---

## function — render_frame

Found in wavefront-film/viz9/render9_hookonly.py at line 1051.

Surface: public API.

Signature: `def render_frame(f)`

What it does: One output frame: SUBN sub-frames averaged (temporal AA / motion

(Where these words came from: direct.)

---

## function — yavg

Found in wavefront-film/viz9/repro_flash.py at line 12.

Surface: public API.

Signature: `def yavg(im)`

What it does: Function yavg takes im. Its return type is unannotated; the code shows it returning float(np.asarray(im.convert("L"), dtype=np.float.

(Where these words came from: inferred.)

---

## function — head_proj

Found in wavefront-film/viz9/repro_flash.py at line 15.

Surface: public API.

Signature: `def head_proj(f)`

What it does: Function head_proj takes f. Its return type is unannotated; the code shows it returning float(sx[0]), float(sy[0]), bool(ok[0]).

(Where these words came from: inferred.)

---

## function — painted_fraction

Found in wavefront-film/viz9/repro_flash.py at line 24.

Surface: public API.

Signature: `def painted_fraction(cy, cx)`

What it does: Apply render8's exact head-block slice expression to a zero buffer

(Where these words came from: direct.)

---

## function — fmt

Found in wavefront-kit/build_your_wavefront.py at line 98.

Surface: public API.

Signature: `def fmt(coord)`

What it does: Function fmt takes coord. Its return type is unannotated; the code shows it returning f"{coord[0]}.{coord[1]}.{coord[2]}/{coord[3]}.{c.

(Where these words came from: inferred.)

---

## function — C

Found in wavefront-kit/build_your_wavefront.py at line 101.

Surface: public API.

Signature: `def C(s)`

What it does: Function C takes s. Its return type is unannotated; the code shows it returning parts.

(Where these words came from: inferred.)

---

## function — parse_phext

Found in wavefront-kit/build_your_wavefront.py at line 106.

Surface: public API.

Signature: `def parse_phext(data)`

What it does: Function parse_phext takes data. Its return type is unannotated; the code shows it returning scrolls.

(Where these words came from: inferred.)

---

## function — assemble

Found in wavefront-kit/build_your_wavefront.py at line 124.

Surface: public API.

Signature: `def assemble(scrolls)`

What it does: Function assemble takes scrolls. Its return type is unannotated; the code shows it returning bytes(out).

(Where these words came from: inferred.)

---

## function — load_lens

Found in wavefront-kit/build_your_wavefront.py at line 138.

Surface: public API.

Signature: `def load_lens(path)`

What it does: Merge an owner's lens file over the default lens, field by field.

(Where these words came from: direct.)

---

## function — recip_from_share

Found in wavefront-kit/build_your_wavefront.py at line 164.

Surface: public API.

Signature: `def recip_from_share(s)`

What it does: Function recip_from_share takes s. Its return type is unannotated; the code shows it returning 4 if s >= 0.7 else 3 if s >= 0.4 else 2.

(Where these words came from: inferred.)

---

## function — main

Found in wavefront-kit/build_your_wavefront.py at line 167.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — region

Found in wavefront-kit/build_your_wavefront.py at line 183.

Surface: public API.

Signature: `def region(vals)`

What it does: Function region takes vals. Its return type is unannotated; the code shows it returning None; "the Shoals"; "the Hearth"; "open water".

(Where these words came from: inferred.)

---

## function — detected_vals

Found in wavefront-kit/build_your_wavefront.py at line 256.

Surface: public API.

Signature: `def detected_vals(c)`

What it does: Function detected_vals takes c. Its return type is unannotated; the code shows it returning vals, glosses.

(Where these words came from: inferred.)

---

## function — dist

Found in wavefront-kit/build_your_wavefront.py at line 296.

Surface: public API.

Signature: `def dist(a, b)`

What it does: Function dist takes a, b. Its return type is unannotated; the code shows it returning sum(abs(x - y) for x, y in zip(a, b)).

(Where these words came from: inferred.)

---

## function — entry_body

Found in wavefront-kit/build_your_wavefront.py at line 306.

Surface: public API.

Signature: `def entry_body(e)`

What it does: Function entry_body takes e. Its return type is unannotated; the code shows it returning lines.

(Where these words came from: inferred.)

---

## function — load_export_file

Found in wavefront-kit/mine_your_archive.py at line 49.

Surface: public API.

Signature: `def load_export_file(path)`

What it does: Parse a .js export file (strip the window.YTD.* wrapper) or plain JSON.

(Where these words came from: direct.)

---

## function — part_number

Found in wavefront-kit/mine_your_archive.py at line 57.

Surface: public API.

Signature: `def part_number(name)`

What it does: Function part_number takes name. Its return type is unannotated; the code shows it returning int(m.group(1)) if m else 0.

(Where these words came from: inferred.)

---

## function — find_tweet_files

Found in wavefront-kit/mine_your_archive.py at line 62.

Surface: public API.

Signature: `def find_tweet_files(archive)`

What it does: Resolve the archive argument to an ordered list of tweet files.

(Where these words came from: direct.)

---

## function — detect_handle

Found in wavefront-kit/mine_your_archive.py at line 78.

Surface: public API.

Signature: `def detect_handle(archive)`

What it does: Best-effort owner handle from data/account.js (username field only).

(Where these words came from: direct.)

---

## function — parse_date

Found in wavefront-kit/mine_your_archive.py at line 98.

Surface: public API.

Signature: `def parse_date(raw)`

What it does: Function parse_date takes raw. Its return type is unannotated; the code shows it returning datetime.strptime(raw, fmt).

(Where these words came from: inferred.)

---

## function — main

Found in wavefront-kit/mine_your_archive.py at line 108.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — rec

Found in wavefront-kit/mine_your_archive.py at line 130.

Surface: public API.

Signature: `def rec(key)`

What it does: Function rec takes key. Its return type is unannotated; the code shows it returning stats[key].

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/emise-tools/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

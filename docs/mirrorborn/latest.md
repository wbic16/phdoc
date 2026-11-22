# mirrorborn — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 95). Project root: /home/wbic16/phdoc-sweep/proj/mirrorborn/. Scanned at commit cb155c86ce47eab69acf9d97529f83b01047e0d1. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 307 commits.

The author of record is **Will Bickford** — 307 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 105 commits
- Verse — 51 commits
- Aster — 41 commits
- Lux — 41 commits
- Theia — 30 commits
- Chrys — 15 commits
- Cyon — 14 commits
- Glia — 3 commits
- Soma — 2 commits
- Axon — 1 commit
- Corus — 1 commit
- Gyrus — 1 commit
- Orin — 1 commit
- Torus — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 285 files, 53731 lines total, 13951 blank, 1708 comment, 38072 code
- Python: 2 files, 358 lines total, 47 blank, 22 comment, 289 code
- Shell: 43 files, 10138 lines total, 1237 blank, 1175 comment, 7726 code

All languages: 330 files, 64227 lines total, 46087 lines of code. Functions read: 162.

Largest files:

- cyon/memory/2026-03-07.md — 6008 lines
- cyon/memory/2026-03-08.md — 5419 lines
- cyon/memory/2026-03-04.md — 5177 lines
- cyon/memory/2026-03-05.md — 4022 lines
- cyon/memory/2026-03-03.md — 1671 lines

Largest functions, by span in lines:

- _scp in scripts/migrate-to-hermes.sh — about 434 lines
- dry in scripts/bootstrap-hermes.sh — about 385 lines
- main in install.sh — about 288 lines
- main in scripts/install.sh — about 288 lines
- get_node_phase in scripts/swarm-migrate.sh — about 271 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 18. FIXME marks: 0.
- Public items with a doc comment: 1 of 161 (1%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/mirrorborn.git
- Branch: exo
- Head commit: cb155c86ce47eab69acf9d97529f83b01047e0d1
- Working tree: 1 changed paths

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `log` (boot.sh:85) — Function log takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is gi
- function `gate_check` (boot.sh:101) — Function gate_check takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `stage_completed` (boot.sh:115) — Function stage_completed takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `mark_stage_complete` (boot.sh:124) — Function mark_stage_complete takes no inputs beyond the value it is called on, and gives back nothing; it does its work 
- function `skip_if_done` (boot.sh:143) — Function skip_if_done takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `resolve_identity` (boot.sh:153) — Function resolve_identity takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `run_phase_0` (boot.sh:202) — Function run_phase_0 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `run_phase_1` (boot.sh:301) — Function run_phase_1 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `run_phase_2` (boot.sh:436) — Function run_phase_2 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `run_phase_3` (boot.sh:623) — Function run_phase_3 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `run_phase_4` (boot.sh:801) — Function run_phase_4 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `run_phase_4` (boot.sh:875) — Function run_phase_4 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `run_phase_5` (boot.sh:992) — Function run_phase_5 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `run_phase_6` (boot.sh:1017) — Function run_phase_6 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `run_dry_run` (boot.sh:1080) — Function run_dry_run takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `main` (boot.sh:1125) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `get_identity` (dashboard/api.py:18) — Function get_identity takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `get_hostmap` (dashboard/api.py:24) — Function get_hostmap takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `curl` (dashboard/api.py:30) — Function curl takes url, timeout=3 and gives back nothing; it does its work on what it is given.
- function `ssh_cmd` (dashboard/api.py:38) — Function ssh_cmd takes host, cmd, timeout=3 and gives back nothing; it does its work on what it is given.
- function `boot_version` (dashboard/api.py:48) — Function boot_version takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `load_pct` (dashboard/api.py:60) — Return integer load % (load1 / nproc * 100), or -1 on error.
- function `check_node` (dashboard/api.py:67) — Function check_node takes node, self_host and gives back nothing; it does its work on what it is given.
- function `collect_status` (dashboard/api.py:157) — Function collect_status takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `probe` (dashboard/api.py:166) — Function probe takes node and gives back nothing; it does its work on what it is given.
- function `get_cached_status` (dashboard/api.py:194) — Function get_cached_status takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `__init__` (dashboard/api.py:209) — Function __init__ takes self, *args, **kwargs and gives back nothing; it does its work on what it is given.
- function `do_GET` (dashboard/api.py:212) — Function do_GET takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is
- function `log_message` (dashboard/api.py:225) — Function log_message takes self, fmt, *args and gives back nothing; it does its work on what it is given.
- function `main` (dashboard/api.py:228) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `cleanup_tmpfiles` (install.sh:27) — Function cleanup_tmpfiles takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `mktempfile` (install.sh:35) — Function mktempfile takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `detect_downloader` (install.sh:43) — Function detect_downloader takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `download_file` (install.sh:56) — Function download_file takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `run_remote_bash` (install.sh:69) — Function run_remote_bash takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `cleanup_legacy_submodules` (install.sh:77) — Function cleanup_legacy_submodules takes no inputs beyond the value it is called on, and gives back nothing; it does its
- function `cleanup_npm_openclaw_paths` (install.sh:86) — Function cleanup_npm_openclaw_paths takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `extract_openclaw_conflict_path` (install.sh:95) — Function extract_openclaw_conflict_path takes no inputs beyond the value it is called on, and gives back nothing; it doe
- function `cleanup_openclaw_bin_conflict` (install.sh:109) — Function cleanup_openclaw_bin_conflict takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `install_openclaw_npm` (install.sh:144) — Function install_openclaw_npm takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `append_holiday_taglines` (install.sh:251) — Function append_holiday_taglines takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `map_legacy_env` (install.sh:274) — Function map_legacy_env takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `pick_tagline` (install.sh:296) — Function pick_tagline takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `print_usage` (install.sh:332) — Function print_usage takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `parse_args` (install.sh:373) — Function parse_args takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `configure_verbose` (install.sh:435) — Function configure_verbose takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `is_promptable` (install.sh:446) — Function is_promptable takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `prompt_choice` (install.sh:456) — Function prompt_choice takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `detect_openclaw_checkout` (install.sh:467) — Function detect_openclaw_checkout takes no inputs beyond the value it is called on, and gives back nothing; it does its 
- function `install_homebrew` (install.sh:505) — Function install_homebrew takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `check_node` (install.sh:525) — Function check_node takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `install_node` (install.sh:542) — Function install_node takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `check_git` (install.sh:579) — Function check_git takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it
- function `is_root` (install.sh:588) — Function is_root takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it i
- function `maybe_sudo` (install.sh:593) — Function maybe_sudo takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `require_sudo` (install.sh:605) — Function require_sudo takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `install_git` (install.sh:620) — Function install_git takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `fix_npm_permissions` (install.sh:642) — Function fix_npm_permissions takes no inputs beyond the value it is called on, and gives back nothing; it does its work 
- function `resolve_openclaw_bin` (install.sh:673) — Function resolve_openclaw_bin takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `ensure_openclaw_bin_link` (install.sh:687) — Function ensure_openclaw_bin_link takes no inputs beyond the value it is called on, and gives back nothing; it does its 
- function `check_existing_openclaw` (install.sh:707) — Function check_existing_openclaw takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `ensure_pnpm` (install.sh:715) — Function ensure_pnpm takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `ensure_user_local_bin_on_path` (install.sh:735) — Function ensure_user_local_bin_on_path takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `npm_global_bin_dir` (install.sh:750) — Function npm_global_bin_dir takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `refresh_shell_command_cache` (install.sh:772) — Function refresh_shell_command_cache takes no inputs beyond the value it is called on, and gives back nothing; it does i
- function `path_has_dir` (install.sh:776) — Function path_has_dir takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `warn_shell_path_missing_dir` (install.sh:788) — Function warn_shell_path_missing_dir takes no inputs beyond the value it is called on, and gives back nothing; it does i
- function `ensure_npm_global_bin_on_path` (install.sh:806) — Function ensure_npm_global_bin_on_path takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `maybe_nodenv_rehash` (install.sh:814) — Function maybe_nodenv_rehash takes no inputs beyond the value it is called on, and gives back nothing; it does its work 
- function `warn_openclaw_not_found` (install.sh:820) — Function warn_openclaw_not_found takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `resolve_openclaw_bin` (install.sh:846) — Function resolve_openclaw_bin takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `install_openclaw_from_git` (install.sh:887) — Function install_openclaw_from_git takes no inputs beyond the value it is called on, and gives back nothing; it does its
- function `resolve_beta_version` (install.sh:937) — Function resolve_beta_version takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `install_openclaw` (install.sh:946) — Function install_openclaw takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `run_doctor` (install.sh:999) — Function run_doctor takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `maybe_open_dashboard` (install.sh:1014) — Function maybe_open_dashboard takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `resolve_workspace_dir` (install.sh:1028) — Function resolve_workspace_dir takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `run_bootstrap_onboarding_if_needed` (install.sh:1037) — Function run_bootstrap_onboarding_if_needed takes no inputs beyond the value it is called on, and gives back nothing; it
- function `resolve_openclaw_version` (install.sh:1078) — Function resolve_openclaw_version takes no inputs beyond the value it is called on, and gives back nothing; it does its 
- function `is_gateway_daemon_loaded` (install.sh:1097) — Function is_gateway_daemon_loaded takes no inputs beyond the value it is called on, and gives back nothing; it does its 
- function `main` (install.sh:1123) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `dry` (scripts/bootstrap-hermes.sh:88) — Function dry takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is gi
- function `determine_mode` (scripts/cadence.sh:42) — Function determine_mode takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `load_class` (scripts/dashboard.sh:165) — Function load_class takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `get` (scripts/extract-openclaw-keys.py:30) — Function get takes obj, *keys, default="" and gives back nothing; it does its work on what it is given.
- function `cleanup_tmpfiles` (scripts/install.sh:27) — Function cleanup_tmpfiles takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `mktempfile` (scripts/install.sh:35) — Function mktempfile takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `detect_downloader` (scripts/install.sh:43) — Function detect_downloader takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `download_file` (scripts/install.sh:56) — Function download_file takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `run_remote_bash` (scripts/install.sh:69) — Function run_remote_bash takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `cleanup_legacy_submodules` (scripts/install.sh:77) — Function cleanup_legacy_submodules takes no inputs beyond the value it is called on, and gives back nothing; it does its
- function `cleanup_npm_openclaw_paths` (scripts/install.sh:86) — Function cleanup_npm_openclaw_paths takes no inputs beyond the value it is called on, and gives back nothing; it does it
- function `extract_openclaw_conflict_path` (scripts/install.sh:95) — Function extract_openclaw_conflict_path takes no inputs beyond the value it is called on, and gives back nothing; it doe
- function `cleanup_openclaw_bin_conflict` (scripts/install.sh:109) — Function cleanup_openclaw_bin_conflict takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `install_openclaw_npm` (scripts/install.sh:144) — Function install_openclaw_npm takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `append_holiday_taglines` (scripts/install.sh:251) — Function append_holiday_taglines takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `map_legacy_env` (scripts/install.sh:274) — Function map_legacy_env takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `pick_tagline` (scripts/install.sh:296) — Function pick_tagline takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `print_usage` (scripts/install.sh:332) — Function print_usage takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `parse_args` (scripts/install.sh:373) — Function parse_args takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `configure_verbose` (scripts/install.sh:435) — Function configure_verbose takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `is_promptable` (scripts/install.sh:446) — Function is_promptable takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `prompt_choice` (scripts/install.sh:456) — Function prompt_choice takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `detect_openclaw_checkout` (scripts/install.sh:467) — Function detect_openclaw_checkout takes no inputs beyond the value it is called on, and gives back nothing; it does its 
- function `install_homebrew` (scripts/install.sh:505) — Function install_homebrew takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `check_node` (scripts/install.sh:525) — Function check_node takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `install_node` (scripts/install.sh:542) — Function install_node takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `check_git` (scripts/install.sh:579) — Function check_git takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it
- function `is_root` (scripts/install.sh:588) — Function is_root takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it i
- function `maybe_sudo` (scripts/install.sh:593) — Function maybe_sudo takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `require_sudo` (scripts/install.sh:605) — Function require_sudo takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `install_git` (scripts/install.sh:620) — Function install_git takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `fix_npm_permissions` (scripts/install.sh:642) — Function fix_npm_permissions takes no inputs beyond the value it is called on, and gives back nothing; it does its work 
- function `resolve_openclaw_bin` (scripts/install.sh:673) — Function resolve_openclaw_bin takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `ensure_openclaw_bin_link` (scripts/install.sh:687) — Function ensure_openclaw_bin_link takes no inputs beyond the value it is called on, and gives back nothing; it does its 
- function `check_existing_openclaw` (scripts/install.sh:707) — Function check_existing_openclaw takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `ensure_pnpm` (scripts/install.sh:715) — Function ensure_pnpm takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `ensure_user_local_bin_on_path` (scripts/install.sh:735) — Function ensure_user_local_bin_on_path takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `npm_global_bin_dir` (scripts/install.sh:750) — Function npm_global_bin_dir takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `refresh_shell_command_cache` (scripts/install.sh:772) — Function refresh_shell_command_cache takes no inputs beyond the value it is called on, and gives back nothing; it does i
- function `path_has_dir` (scripts/install.sh:776) — Function path_has_dir takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `warn_shell_path_missing_dir` (scripts/install.sh:788) — Function warn_shell_path_missing_dir takes no inputs beyond the value it is called on, and gives back nothing; it does i
- function `ensure_npm_global_bin_on_path` (scripts/install.sh:806) — Function ensure_npm_global_bin_on_path takes no inputs beyond the value it is called on, and gives back nothing; it does
- function `maybe_nodenv_rehash` (scripts/install.sh:814) — Function maybe_nodenv_rehash takes no inputs beyond the value it is called on, and gives back nothing; it does its work 
- function `warn_openclaw_not_found` (scripts/install.sh:820) — Function warn_openclaw_not_found takes no inputs beyond the value it is called on, and gives back nothing; it does its w
- function `resolve_openclaw_bin` (scripts/install.sh:846) — Function resolve_openclaw_bin takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `install_openclaw_from_git` (scripts/install.sh:887) — Function install_openclaw_from_git takes no inputs beyond the value it is called on, and gives back nothing; it does its
- function `resolve_beta_version` (scripts/install.sh:937) — Function resolve_beta_version takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `install_openclaw` (scripts/install.sh:946) — Function install_openclaw takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `run_doctor` (scripts/install.sh:999) — Function run_doctor takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `maybe_open_dashboard` (scripts/install.sh:1014) — Function maybe_open_dashboard takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `resolve_workspace_dir` (scripts/install.sh:1028) — Function resolve_workspace_dir takes no inputs beyond the value it is called on, and gives back nothing; it does its wor
- function `run_bootstrap_onboarding_if_needed` (scripts/install.sh:1037) — Function run_bootstrap_onboarding_if_needed takes no inputs beyond the value it is called on, and gives back nothing; it
- function `resolve_openclaw_version` (scripts/install.sh:1078) — Function resolve_openclaw_version takes no inputs beyond the value it is called on, and gives back nothing; it does its 
- function `is_gateway_daemon_loaded` (scripts/install.sh:1097) — Function is_gateway_daemon_loaded takes no inputs beyond the value it is called on, and gives back nothing; it does its 
- function `main` (scripts/install.sh:1123) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `is_reachable` (scripts/migrate-all-nodes.sh:79) — Function is_reachable takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `_ssh` (scripts/migrate-to-hermes.sh:93) — Function _ssh takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `remote` (scripts/migrate-to-hermes.sh:101) — Function remote takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is
- function `remote_script` (scripts/migrate-to-hermes.sh:112) — Function remote_script takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `_scp` (scripts/migrate-to-hermes.sh:123) — Function _scp takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `collect_responses` (scripts/orin-collect.sh:47) — Function collect_responses takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `sq_read` (scripts/print.sh:31) — Function sq_read takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it i
- function `sq_write` (scripts/print.sh:35) — Function sq_write takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `sq_clear` (scripts/print.sh:40) — Function sq_clear takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `sq_lock_acquire` (scripts/print.sh:45) — Function sq_lock_acquire takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `sq_lock_release` (scripts/print.sh:82) — Function sq_lock_release takes no inputs beyond the value it is called on, and gives back nothing; it does its work on w
- function `count_pages` (scripts/print.sh:181) — Function count_pages takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `log` (scripts/ssh-mesh-sync.sh:55) — Function log takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is gi
- function `log` (scripts/ssh-push-key.sh:46) — Function log takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is gi
- function `_load_tailscale_ips` (scripts/swarm-bootstrap.sh:98) — Function _load_tailscale_ips takes no inputs beyond the value it is called on, and gives back nothing; it does its work 
- function `_get_tailscale_ip` (scripts/swarm-bootstrap.sh:111) — Function _get_tailscale_ip takes no inputs beyond the value it is called on, and gives back nothing; it does its work on
- function `_ssh` (scripts/swarm-bootstrap.sh:126) — Function _ssh takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `_check_reachable` (scripts/swarm-bootstrap.sh:131) — Function _check_reachable takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `_resolve_addr` (scripts/swarm-bootstrap.sh:136) — Function _resolve_addr takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `probe_connection` (scripts/swarm-migrate.sh:149) — Function probe_connection takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `remote_exec` (scripts/swarm-migrate.sh:177) — Function remote_exec takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 
- function `remote_scp` (scripts/swarm-migrate.sh:185) — Function remote_scp takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `get_node_phase` (scripts/swarm-migrate.sh:194) — Function get_node_phase takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wh
- function `get_tailscale_ip` (scripts/tailscale-ips.sh:17) — Function get_tailscale_ip takes no inputs beyond the value it is called on, and gives back nothing; it does its work on 
- function `_file_issue` (scripts/uptime-monitor.sh:144) — Function _file_issue takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what 

---

## function — log

Found in boot.sh at line 85.

Signature: `log()`

What it does: Function log takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — gate_check

Found in boot.sh at line 101.

Signature: `gate_check()`

What it does: Function gate_check takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — stage_completed

Found in boot.sh at line 115.

Signature: `stage_completed()`

What it does: Function stage_completed takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — mark_stage_complete

Found in boot.sh at line 124.

Signature: `mark_stage_complete()`

What it does: Function mark_stage_complete takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — skip_if_done

Found in boot.sh at line 143.

Signature: `skip_if_done()`

What it does: Function skip_if_done takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — resolve_identity

Found in boot.sh at line 153.

Signature: `resolve_identity()`

What it does: Function resolve_identity takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_phase_0

Found in boot.sh at line 202.

Signature: `run_phase_0()`

What it does: Function run_phase_0 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_phase_1

Found in boot.sh at line 301.

Signature: `run_phase_1()`

What it does: Function run_phase_1 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_phase_2

Found in boot.sh at line 436.

Signature: `run_phase_2()`

What it does: Function run_phase_2 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_phase_3

Found in boot.sh at line 623.

Signature: `run_phase_3()`

What it does: Function run_phase_3 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_phase_4

Found in boot.sh at line 801.

Signature: `run_phase_4()`

What it does: Function run_phase_4 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_phase_4

Found in boot.sh at line 875.

Signature: `run_phase_4()`

What it does: Function run_phase_4 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_phase_5

Found in boot.sh at line 992.

Signature: `run_phase_5()`

What it does: Function run_phase_5 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_phase_6

Found in boot.sh at line 1017.

Signature: `run_phase_6()`

What it does: Function run_phase_6 takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_dry_run

Found in boot.sh at line 1080.

Signature: `run_dry_run()`

What it does: Function run_dry_run takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in boot.sh at line 1125.

Signature: `main()`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — get_identity

Found in dashboard/api.py at line 18.

Signature: `def get_identity():`

What it does: Function get_identity takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — get_hostmap

Found in dashboard/api.py at line 24.

Signature: `def get_hostmap():`

What it does: Function get_hostmap takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — curl

Found in dashboard/api.py at line 30.

Signature: `def curl(url, timeout=3):`

What it does: Function curl takes url, timeout=3 and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — ssh_cmd

Found in dashboard/api.py at line 38.

Signature: `def ssh_cmd(host, cmd, timeout=3):`

What it does: Function ssh_cmd takes host, cmd, timeout=3 and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — boot_version

Found in dashboard/api.py at line 48.

Signature: `def boot_version():`

What it does: Function boot_version takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — load_pct

Found in dashboard/api.py at line 60.

Signature: `def load_pct(load1_str, nproc_str):`

What it does: Return integer load % (load1 / nproc * 100), or -1 on error.

(Where these words came from: doc comment.)

---

## function — check_node

Found in dashboard/api.py at line 67.

Signature: `def check_node(node, self_host):`

What it does: Function check_node takes node, self_host and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — collect_status

Found in dashboard/api.py at line 157.

Signature: `def collect_status():`

What it does: Function collect_status takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — probe

Found in dashboard/api.py at line 166.

Signature: `def probe(node):`

What it does: Function probe takes node and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — get_cached_status

Found in dashboard/api.py at line 194.

Signature: `def get_cached_status():`

What it does: Function get_cached_status takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — __init__

Found in dashboard/api.py at line 209.

Signature: `def __init__(self, *args, **kwargs):`

What it does: Function __init__ takes self, *args, **kwargs and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — do_GET

Found in dashboard/api.py at line 212.

Signature: `def do_GET(self):`

What it does: Function do_GET takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — log_message

Found in dashboard/api.py at line 225.

Signature: `def log_message(self, fmt, *args):`

What it does: Function log_message takes self, fmt, *args and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in dashboard/api.py at line 228.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cleanup_tmpfiles

Found in install.sh at line 27.

Signature: `cleanup_tmpfiles()`

What it does: Function cleanup_tmpfiles takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — mktempfile

Found in install.sh at line 35.

Signature: `mktempfile()`

What it does: Function mktempfile takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — detect_downloader

Found in install.sh at line 43.

Signature: `detect_downloader()`

What it does: Function detect_downloader takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — download_file

Found in install.sh at line 56.

Signature: `download_file()`

What it does: Function download_file takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_remote_bash

Found in install.sh at line 69.

Signature: `run_remote_bash()`

What it does: Function run_remote_bash takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cleanup_legacy_submodules

Found in install.sh at line 77.

Signature: `cleanup_legacy_submodules()`

What it does: Function cleanup_legacy_submodules takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cleanup_npm_openclaw_paths

Found in install.sh at line 86.

Signature: `cleanup_npm_openclaw_paths()`

What it does: Function cleanup_npm_openclaw_paths takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — extract_openclaw_conflict_path

Found in install.sh at line 95.

Signature: `extract_openclaw_conflict_path()`

What it does: Function extract_openclaw_conflict_path takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cleanup_openclaw_bin_conflict

Found in install.sh at line 109.

Signature: `cleanup_openclaw_bin_conflict()`

What it does: Function cleanup_openclaw_bin_conflict takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_openclaw_npm

Found in install.sh at line 144.

Signature: `install_openclaw_npm()`

What it does: Function install_openclaw_npm takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — append_holiday_taglines

Found in install.sh at line 251.

Signature: `append_holiday_taglines()`

What it does: Function append_holiday_taglines takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — map_legacy_env

Found in install.sh at line 274.

Signature: `map_legacy_env()`

What it does: Function map_legacy_env takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — pick_tagline

Found in install.sh at line 296.

Signature: `pick_tagline()`

What it does: Function pick_tagline takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — print_usage

Found in install.sh at line 332.

Signature: `print_usage()`

What it does: Function print_usage takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — parse_args

Found in install.sh at line 373.

Signature: `parse_args()`

What it does: Function parse_args takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — configure_verbose

Found in install.sh at line 435.

Signature: `configure_verbose()`

What it does: Function configure_verbose takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — is_promptable

Found in install.sh at line 446.

Signature: `is_promptable()`

What it does: Function is_promptable takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — prompt_choice

Found in install.sh at line 456.

Signature: `prompt_choice()`

What it does: Function prompt_choice takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — detect_openclaw_checkout

Found in install.sh at line 467.

Signature: `detect_openclaw_checkout()`

What it does: Function detect_openclaw_checkout takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_homebrew

Found in install.sh at line 505.

Signature: `install_homebrew()`

What it does: Function install_homebrew takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — check_node

Found in install.sh at line 525.

Signature: `check_node()`

What it does: Function check_node takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_node

Found in install.sh at line 542.

Signature: `install_node()`

What it does: Function install_node takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — check_git

Found in install.sh at line 579.

Signature: `check_git()`

What it does: Function check_git takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — is_root

Found in install.sh at line 588.

Signature: `is_root()`

What it does: Function is_root takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — maybe_sudo

Found in install.sh at line 593.

Signature: `maybe_sudo()`

What it does: Function maybe_sudo takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — require_sudo

Found in install.sh at line 605.

Signature: `require_sudo()`

What it does: Function require_sudo takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_git

Found in install.sh at line 620.

Signature: `install_git()`

What it does: Function install_git takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — fix_npm_permissions

Found in install.sh at line 642.

Signature: `fix_npm_permissions()`

What it does: Function fix_npm_permissions takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — resolve_openclaw_bin

Found in install.sh at line 673.

Signature: `resolve_openclaw_bin()`

What it does: Function resolve_openclaw_bin takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — ensure_openclaw_bin_link

Found in install.sh at line 687.

Signature: `ensure_openclaw_bin_link()`

What it does: Function ensure_openclaw_bin_link takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — check_existing_openclaw

Found in install.sh at line 707.

Signature: `check_existing_openclaw()`

What it does: Function check_existing_openclaw takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — ensure_pnpm

Found in install.sh at line 715.

Signature: `ensure_pnpm()`

What it does: Function ensure_pnpm takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — ensure_user_local_bin_on_path

Found in install.sh at line 735.

Signature: `ensure_user_local_bin_on_path()`

What it does: Function ensure_user_local_bin_on_path takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — npm_global_bin_dir

Found in install.sh at line 750.

Signature: `npm_global_bin_dir()`

What it does: Function npm_global_bin_dir takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — refresh_shell_command_cache

Found in install.sh at line 772.

Signature: `refresh_shell_command_cache()`

What it does: Function refresh_shell_command_cache takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — path_has_dir

Found in install.sh at line 776.

Signature: `path_has_dir()`

What it does: Function path_has_dir takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — warn_shell_path_missing_dir

Found in install.sh at line 788.

Signature: `warn_shell_path_missing_dir()`

What it does: Function warn_shell_path_missing_dir takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — ensure_npm_global_bin_on_path

Found in install.sh at line 806.

Signature: `ensure_npm_global_bin_on_path()`

What it does: Function ensure_npm_global_bin_on_path takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — maybe_nodenv_rehash

Found in install.sh at line 814.

Signature: `maybe_nodenv_rehash()`

What it does: Function maybe_nodenv_rehash takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — warn_openclaw_not_found

Found in install.sh at line 820.

Signature: `warn_openclaw_not_found()`

What it does: Function warn_openclaw_not_found takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — resolve_openclaw_bin

Found in install.sh at line 846.

Signature: `resolve_openclaw_bin()`

What it does: Function resolve_openclaw_bin takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_openclaw_from_git

Found in install.sh at line 887.

Signature: `install_openclaw_from_git()`

What it does: Function install_openclaw_from_git takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — resolve_beta_version

Found in install.sh at line 937.

Signature: `resolve_beta_version()`

What it does: Function resolve_beta_version takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_openclaw

Found in install.sh at line 946.

Signature: `install_openclaw()`

What it does: Function install_openclaw takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_doctor

Found in install.sh at line 999.

Signature: `run_doctor()`

What it does: Function run_doctor takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — maybe_open_dashboard

Found in install.sh at line 1014.

Signature: `maybe_open_dashboard()`

What it does: Function maybe_open_dashboard takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — resolve_workspace_dir

Found in install.sh at line 1028.

Signature: `resolve_workspace_dir()`

What it does: Function resolve_workspace_dir takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_bootstrap_onboarding_if_needed

Found in install.sh at line 1037.

Signature: `run_bootstrap_onboarding_if_needed()`

What it does: Function run_bootstrap_onboarding_if_needed takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — resolve_openclaw_version

Found in install.sh at line 1078.

Signature: `resolve_openclaw_version()`

What it does: Function resolve_openclaw_version takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — is_gateway_daemon_loaded

Found in install.sh at line 1097.

Signature: `is_gateway_daemon_loaded()`

What it does: Function is_gateway_daemon_loaded takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in install.sh at line 1123.

Signature: `main()`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — dry

Found in scripts/bootstrap-hermes.sh at line 88.

Signature: `dry()`

What it does: Function dry takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — determine_mode

Found in scripts/cadence.sh at line 42.

Signature: `determine_mode()`

What it does: Function determine_mode takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — load_class

Found in scripts/dashboard.sh at line 165.

Signature: `load_class()`

What it does: Function load_class takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — get

Found in scripts/extract-openclaw-keys.py at line 30.

Signature: `def get(obj, *keys, default=""):`

What it does: Function get takes obj, *keys, default="" and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cleanup_tmpfiles

Found in scripts/install.sh at line 27.

Signature: `cleanup_tmpfiles()`

What it does: Function cleanup_tmpfiles takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — mktempfile

Found in scripts/install.sh at line 35.

Signature: `mktempfile()`

What it does: Function mktempfile takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — detect_downloader

Found in scripts/install.sh at line 43.

Signature: `detect_downloader()`

What it does: Function detect_downloader takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — download_file

Found in scripts/install.sh at line 56.

Signature: `download_file()`

What it does: Function download_file takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_remote_bash

Found in scripts/install.sh at line 69.

Signature: `run_remote_bash()`

What it does: Function run_remote_bash takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cleanup_legacy_submodules

Found in scripts/install.sh at line 77.

Signature: `cleanup_legacy_submodules()`

What it does: Function cleanup_legacy_submodules takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cleanup_npm_openclaw_paths

Found in scripts/install.sh at line 86.

Signature: `cleanup_npm_openclaw_paths()`

What it does: Function cleanup_npm_openclaw_paths takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — extract_openclaw_conflict_path

Found in scripts/install.sh at line 95.

Signature: `extract_openclaw_conflict_path()`

What it does: Function extract_openclaw_conflict_path takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cleanup_openclaw_bin_conflict

Found in scripts/install.sh at line 109.

Signature: `cleanup_openclaw_bin_conflict()`

What it does: Function cleanup_openclaw_bin_conflict takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_openclaw_npm

Found in scripts/install.sh at line 144.

Signature: `install_openclaw_npm()`

What it does: Function install_openclaw_npm takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — append_holiday_taglines

Found in scripts/install.sh at line 251.

Signature: `append_holiday_taglines()`

What it does: Function append_holiday_taglines takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — map_legacy_env

Found in scripts/install.sh at line 274.

Signature: `map_legacy_env()`

What it does: Function map_legacy_env takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — pick_tagline

Found in scripts/install.sh at line 296.

Signature: `pick_tagline()`

What it does: Function pick_tagline takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — print_usage

Found in scripts/install.sh at line 332.

Signature: `print_usage()`

What it does: Function print_usage takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — parse_args

Found in scripts/install.sh at line 373.

Signature: `parse_args()`

What it does: Function parse_args takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — configure_verbose

Found in scripts/install.sh at line 435.

Signature: `configure_verbose()`

What it does: Function configure_verbose takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — is_promptable

Found in scripts/install.sh at line 446.

Signature: `is_promptable()`

What it does: Function is_promptable takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — prompt_choice

Found in scripts/install.sh at line 456.

Signature: `prompt_choice()`

What it does: Function prompt_choice takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — detect_openclaw_checkout

Found in scripts/install.sh at line 467.

Signature: `detect_openclaw_checkout()`

What it does: Function detect_openclaw_checkout takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_homebrew

Found in scripts/install.sh at line 505.

Signature: `install_homebrew()`

What it does: Function install_homebrew takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — check_node

Found in scripts/install.sh at line 525.

Signature: `check_node()`

What it does: Function check_node takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_node

Found in scripts/install.sh at line 542.

Signature: `install_node()`

What it does: Function install_node takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — check_git

Found in scripts/install.sh at line 579.

Signature: `check_git()`

What it does: Function check_git takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — is_root

Found in scripts/install.sh at line 588.

Signature: `is_root()`

What it does: Function is_root takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — maybe_sudo

Found in scripts/install.sh at line 593.

Signature: `maybe_sudo()`

What it does: Function maybe_sudo takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — require_sudo

Found in scripts/install.sh at line 605.

Signature: `require_sudo()`

What it does: Function require_sudo takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_git

Found in scripts/install.sh at line 620.

Signature: `install_git()`

What it does: Function install_git takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — fix_npm_permissions

Found in scripts/install.sh at line 642.

Signature: `fix_npm_permissions()`

What it does: Function fix_npm_permissions takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — resolve_openclaw_bin

Found in scripts/install.sh at line 673.

Signature: `resolve_openclaw_bin()`

What it does: Function resolve_openclaw_bin takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — ensure_openclaw_bin_link

Found in scripts/install.sh at line 687.

Signature: `ensure_openclaw_bin_link()`

What it does: Function ensure_openclaw_bin_link takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — check_existing_openclaw

Found in scripts/install.sh at line 707.

Signature: `check_existing_openclaw()`

What it does: Function check_existing_openclaw takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — ensure_pnpm

Found in scripts/install.sh at line 715.

Signature: `ensure_pnpm()`

What it does: Function ensure_pnpm takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — ensure_user_local_bin_on_path

Found in scripts/install.sh at line 735.

Signature: `ensure_user_local_bin_on_path()`

What it does: Function ensure_user_local_bin_on_path takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — npm_global_bin_dir

Found in scripts/install.sh at line 750.

Signature: `npm_global_bin_dir()`

What it does: Function npm_global_bin_dir takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — refresh_shell_command_cache

Found in scripts/install.sh at line 772.

Signature: `refresh_shell_command_cache()`

What it does: Function refresh_shell_command_cache takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — path_has_dir

Found in scripts/install.sh at line 776.

Signature: `path_has_dir()`

What it does: Function path_has_dir takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — warn_shell_path_missing_dir

Found in scripts/install.sh at line 788.

Signature: `warn_shell_path_missing_dir()`

What it does: Function warn_shell_path_missing_dir takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — ensure_npm_global_bin_on_path

Found in scripts/install.sh at line 806.

Signature: `ensure_npm_global_bin_on_path()`

What it does: Function ensure_npm_global_bin_on_path takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — maybe_nodenv_rehash

Found in scripts/install.sh at line 814.

Signature: `maybe_nodenv_rehash()`

What it does: Function maybe_nodenv_rehash takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — warn_openclaw_not_found

Found in scripts/install.sh at line 820.

Signature: `warn_openclaw_not_found()`

What it does: Function warn_openclaw_not_found takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — resolve_openclaw_bin

Found in scripts/install.sh at line 846.

Signature: `resolve_openclaw_bin()`

What it does: Function resolve_openclaw_bin takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_openclaw_from_git

Found in scripts/install.sh at line 887.

Signature: `install_openclaw_from_git()`

What it does: Function install_openclaw_from_git takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — resolve_beta_version

Found in scripts/install.sh at line 937.

Signature: `resolve_beta_version()`

What it does: Function resolve_beta_version takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — install_openclaw

Found in scripts/install.sh at line 946.

Signature: `install_openclaw()`

What it does: Function install_openclaw takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_doctor

Found in scripts/install.sh at line 999.

Signature: `run_doctor()`

What it does: Function run_doctor takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — maybe_open_dashboard

Found in scripts/install.sh at line 1014.

Signature: `maybe_open_dashboard()`

What it does: Function maybe_open_dashboard takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — resolve_workspace_dir

Found in scripts/install.sh at line 1028.

Signature: `resolve_workspace_dir()`

What it does: Function resolve_workspace_dir takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_bootstrap_onboarding_if_needed

Found in scripts/install.sh at line 1037.

Signature: `run_bootstrap_onboarding_if_needed()`

What it does: Function run_bootstrap_onboarding_if_needed takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — resolve_openclaw_version

Found in scripts/install.sh at line 1078.

Signature: `resolve_openclaw_version()`

What it does: Function resolve_openclaw_version takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — is_gateway_daemon_loaded

Found in scripts/install.sh at line 1097.

Signature: `is_gateway_daemon_loaded()`

What it does: Function is_gateway_daemon_loaded takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in scripts/install.sh at line 1123.

Signature: `main()`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — is_reachable

Found in scripts/migrate-all-nodes.sh at line 79.

Signature: `is_reachable()`

What it does: Function is_reachable takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — _ssh

Found in scripts/migrate-to-hermes.sh at line 93.

Signature: `_ssh()`

What it does: Function _ssh takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — remote

Found in scripts/migrate-to-hermes.sh at line 101.

Signature: `remote()`

What it does: Function remote takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — remote_script

Found in scripts/migrate-to-hermes.sh at line 112.

Signature: `remote_script()`

What it does: Function remote_script takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — _scp

Found in scripts/migrate-to-hermes.sh at line 123.

Signature: `_scp()`

What it does: Function _scp takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — collect_responses

Found in scripts/orin-collect.sh at line 47.

Signature: `collect_responses()`

What it does: Function collect_responses takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — sq_read

Found in scripts/print.sh at line 31.

Signature: `sq_read()`

What it does: Function sq_read takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — sq_write

Found in scripts/print.sh at line 35.

Signature: `sq_write()`

What it does: Function sq_write takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — sq_clear

Found in scripts/print.sh at line 40.

Signature: `sq_clear()`

What it does: Function sq_clear takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — sq_lock_acquire

Found in scripts/print.sh at line 45.

Signature: `sq_lock_acquire()`

What it does: Function sq_lock_acquire takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — sq_lock_release

Found in scripts/print.sh at line 82.

Signature: `sq_lock_release()`

What it does: Function sq_lock_release takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — count_pages

Found in scripts/print.sh at line 181.

Signature: `count_pages()`

What it does: Function count_pages takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — log

Found in scripts/ssh-mesh-sync.sh at line 55.

Signature: `log()`

What it does: Function log takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — log

Found in scripts/ssh-push-key.sh at line 46.

Signature: `log()`

What it does: Function log takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — _load_tailscale_ips

Found in scripts/swarm-bootstrap.sh at line 98.

Signature: `_load_tailscale_ips()`

What it does: Function _load_tailscale_ips takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — _get_tailscale_ip

Found in scripts/swarm-bootstrap.sh at line 111.

Signature: `_get_tailscale_ip()`

What it does: Function _get_tailscale_ip takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — _ssh

Found in scripts/swarm-bootstrap.sh at line 126.

Signature: `_ssh()`

What it does: Function _ssh takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — _check_reachable

Found in scripts/swarm-bootstrap.sh at line 131.

Signature: `_check_reachable()`

What it does: Function _check_reachable takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — _resolve_addr

Found in scripts/swarm-bootstrap.sh at line 136.

Signature: `_resolve_addr()`

What it does: Function _resolve_addr takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — probe_connection

Found in scripts/swarm-migrate.sh at line 149.

Signature: `probe_connection()`

What it does: Function probe_connection takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — remote_exec

Found in scripts/swarm-migrate.sh at line 177.

Signature: `remote_exec()`

What it does: Function remote_exec takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — remote_scp

Found in scripts/swarm-migrate.sh at line 185.

Signature: `remote_scp()`

What it does: Function remote_scp takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — get_node_phase

Found in scripts/swarm-migrate.sh at line 194.

Signature: `get_node_phase()`

What it does: Function get_node_phase takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — get_tailscale_ip

Found in scripts/tailscale-ips.sh at line 17.

Signature: `get_tailscale_ip()`

What it does: Function get_tailscale_ip takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — _file_issue

Found in scripts/uptime-monitor.sh at line 144.

Signature: `_file_issue()`

What it does: Function _file_issue takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/mirrorborn/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

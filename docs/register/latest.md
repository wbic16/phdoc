# register — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 150). Scanned at commit 6830c2fad6fffeefd4c2722a3fad80d04f8dd319. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Summary

Builders for the phext registers: each `build_*_phext.py` rebuilds one register's `.phext` from its source JSON (`--check` verifies the committed phext matches its source). `costs_tick.py` appends ticks to the costs ledger; `publish-todo.sh` / `run-publish-todo.sh` publish the todo register.

(The opening of the project's README, quoted as written.)

---

## Dependencies

### Python (pip) — register-service/requirements.txt

Direct dependencies: 10.

- cbor2 — ==6.3.0
- cffi — ==2.1.1
- cryptography — ==50.0.2
- ldap3 — ==2.9.1
- pyasn1 — ==0.6.5
- pyasn1_modules — ==0.4.2
- pycparser — ==3.12
- pyOpenSSL — ==26.4.0
- typing_extensions — ==4.16.0
- webauthn — ==3.0.1


---

## Authors

The git log holds 21 commits.

The author of record is **Will Bickford** — 21 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 10 commits
- Emise — 7 commits
- Theia — 4 commits

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 3 files, 108 lines total, 18 blank, 0 comment, 90 code
- Python: 31 files, 6122 lines total, 722 blank, 123 comment, 5277 code
- Shell: 4 files, 230 lines total, 22 blank, 46 comment, 162 code

All languages: 38 files, 6460 lines total, 5529 lines of code. Functions read: 305.

Largest files:

- register-ui/generate_register_page.py — 746 lines
- build_ietf_phext.py — 572 lines
- register-service/tests/test_app.py — 496 lines
- register-service/regsvc/app.py — 433 lines
- build_lessons_phext.py — 272 lines

Largest functions, by span in lines:

- repl in register-ui/generate_register_page.py — about 574 lines
- render_home in build_ietf_phext.py — about 121 lines
- render_rfc_header in build_ietf_phext.py — about 109 lines
- main in register-ui/generate_register_page.py — about 73 lines
- build in build_costs_phext.py — about 67 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 73 test functions counted in the source, across 6 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 27 of 275 (10%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/register
- Branch: exo
- Head commit: 6830c2fad6fffeefd4c2722a3fad80d04f8dd319
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `gpt_day` (build_accounting_phext.py:45) — Function gpt_day takes d.
- function `week_of` (build_accounting_phext.py:48) — Function week_of takes d.
- function `net` (build_accounting_phext.py:52) — Function net takes a, d.
- function `build` (build_accounting_phext.py:55) — Function build takes no inputs beyond the value it is called on.
- function `lag_line` (build_accounting_phext.py:79) — Function lag_line takes atoms.
- function `day_lines` (build_accounting_phext.py:85) — Function day_lines takes ds.
- function `week_lines` (build_accounting_phext.py:102) — Function week_lines takes w, sunday, dss.
- function `tot` (build_accounting_phext.py:107) — Function tot takes k.
- function `rhu` (build_accounting_phext.py:155) — Function rhu takes x.
- function `avgf` (build_accounting_phext.py:160) — Function avgf takes k.
- function `btot` (build_accounting_phext.py:186) — Function btot takes k, aa=aa.
- function `t_all` (build_accounting_phext.py:202) — Function t_all takes k.
- function `parse_coord` (build_artifacts_phext.py:52) — Function parse_coord takes s.
- function `implode` (build_artifacts_phext.py:60) — scrolls: dict coord-string -> text.
- function `read_source` (build_artifacts_phext.py:78) — Function read_source takes a.
- function `render_artifact` (build_artifacts_phext.py:85) — Function render_artifact takes a, raw.
- function `render_home` (build_artifacts_phext.py:104) — Function render_home takes artifacts.
- function `load_manifest` (build_artifacts_phext.py:132) — Function load_manifest takes no inputs beyond the value it is called on.
- function `build` (build_artifacts_phext.py:137) — Function build takes no inputs beyond the value it is called on.
- function `main` (build_artifacts_phext.py:155) — Function main takes no inputs beyond the value it is called on.
- function `parse_coord` (build_barnacles_phext.py:42) — Function parse_coord takes s.
- function `entry_coord` (build_barnacles_phext.py:50) — Entry n (1-indexed) -> 2.1.1/1.1.1/S.T.B (hundred/ten/unit).
- function `implode` (build_barnacles_phext.py:56) — Function implode takes scrolls.
- function `render_entry` (build_barnacles_phext.py:72) — Function render_entry takes e, data.
- function `render_home` (build_barnacles_phext.py:92) — Function render_home takes data.
- function `build` (build_barnacles_phext.py:142) — Function build takes no inputs beyond the value it is called on.
- function `main` (build_barnacles_phext.py:161) — Function main takes no inputs beyond the value it is called on.
- function `parse_coord` (build_contests_phext.py:44) — Function parse_coord takes s.
- function `entry_coord` (build_contests_phext.py:52) — Function entry_coord takes k.
- function `implode` (build_contests_phext.py:56) — scrolls: dict coord-string -> text.
- function `render_contest` (build_contests_phext.py:74) — Function render_contest takes k, c.
- function `render_out_of_scope` (build_contests_phext.py:96) — Function render_out_of_scope takes k, o.
- function `render_home` (build_contests_phext.py:108) — Function render_home takes contests.
- function `build` (build_contests_phext.py:161) — Function build takes no inputs beyond the value it is called on.
- function `main` (build_contests_phext.py:178) — Function main takes no inputs beyond the value it is called on.
- function `B` (build_costs_phext.py:17) — Function B takes x.
- function `build` (build_costs_phext.py:20) — Function build takes no inputs beyond the value it is called on.
- function `parse_coord` (build_doomer_phext.py:34) — Function parse_coord takes s.
- function `implode` (build_doomer_phext.py:42) — Function implode takes scrolls.
- function `render_entry` (build_doomer_phext.py:58) — Function render_entry takes e.
- function `render_home` (build_doomer_phext.py:72) — Function render_home takes data.
- function `build` (build_doomer_phext.py:97) — Function build takes no inputs beyond the value it is called on.
- function `main` (build_doomer_phext.py:112) — Function main takes no inputs beyond the value it is called on.
- function `parse_coord` (build_humanities_phext.py:48) — Function parse_coord takes s.
- function `implode` (build_humanities_phext.py:56) — scrolls: dict coord-string -> text.
- function `read_source` (build_humanities_phext.py:74) — Function read_source takes a.
- function `render_artifact` (build_humanities_phext.py:81) — Function render_artifact takes a, raw.
- function `render_home` (build_humanities_phext.py:100) — Function render_home takes documents.
- function `load_manifest` (build_humanities_phext.py:129) — Function load_manifest takes no inputs beyond the value it is called on.
- function `build` (build_humanities_phext.py:134) — Function build takes no inputs beyond the value it is called on.
- function `main` (build_humanities_phext.py:152) — Function main takes no inputs beyond the value it is called on.
- function `reg_t3_coord` (build_ietf_phext.py:65) — Function reg_t3_coord takes rfc.
- function `parse_coord` (build_ietf_phext.py:69) — Function parse_coord takes s.
- function `implode` (build_ietf_phext.py:77) — Same state machine as build_tasks_phext.py / build_skills_phext.py.
- function `clean` (build_ietf_phext.py:94) — Strip phext delimiter chars from generated metadata text.
- function `lst` (build_ietf_phext.py:99) — Function lst takes nums.
- function `capped` (build_ietf_phext.py:103) — Function capped takes nums, cap=LIST_CAP.
- function `subseries_str` (build_ietf_phext.py:109) — Function subseries_str takes e.
- function `escape_text` (build_ietf_phext.py:120) — Function escape_text takes text, expected_delims, rfc.
- function `refs_of` (build_ietf_phext.py:133) — Function refs_of takes e, kind.
- function `render_rfc_header` (build_ietf_phext.py:137) — Function render_rfc_header takes e, src.
- function `render_rfc` (build_ietf_phext.py:246) — Function render_rfc takes e, src, corpus.
- function `impl_short` (build_ietf_phext.py:282) — Function impl_short takes e.
- function `registry_line` (build_ietf_phext.py:292) — Function registry_line takes e, src.
- function `render_home` (build_ietf_phext.py:336) — Function render_home takes src.
- function `render_registry_overview` (build_ietf_phext.py:457) — Function render_registry_overview takes src.
- function `build` (build_ietf_phext.py:503) — Function build takes source_path, corpus.
- function `main` (build_ietf_phext.py:532) — Function main takes no inputs beyond the value it is called on.
- function `opt` (build_ietf_phext.py:536) — Function opt takes name, env, default=None.
- function `parse_coord` (build_jobs_phext.py:50) — Function parse_coord takes s.
- function `implode` (build_jobs_phext.py:58) — scrolls: dict coord-string -> text.
- function `render_job` (build_jobs_phext.py:76) — Function render_job takes j.
- function `render_home` (build_jobs_phext.py:95) — Function render_home takes jobs.
- function `build` (build_jobs_phext.py:125) — Function build takes no inputs beyond the value it is called on.
- function `main` (build_jobs_phext.py:143) — Function main takes no inputs beyond the value it is called on.
- function `parse_coord` (build_journal_phext.py:50) — Function parse_coord takes s.
- function `implode` (build_journal_phext.py:58) — scrolls: dict coord-string -> text.
- function `render_entry` (build_journal_phext.py:76) — Function render_entry takes e.
- function `render_home` (build_journal_phext.py:93) — Function render_home takes entries.
- function `build` (build_journal_phext.py:160) — Function build takes no inputs beyond the value it is called on.
- function `main` (build_journal_phext.py:175) — Function main takes no inputs beyond the value it is called on.
- function `c` (build_lessons_phext.py:48) — Function c takes s.
- function `fmt` (build_lessons_phext.py:52) — Function fmt takes coord.
- function `parse_bytes` (build_lessons_phext.py:56) — Dead-reckoning parse of raw phext bytes -> {coord_str: raw_bytes} for
- function `implode` (build_lessons_phext.py:84) — scrolls: dict coord-string -> bytes.
- function `lesson_meta` (build_lessons_phext.py:101) — (short statement, date given, mark) from one lesson scroll.
- function `build` (build_lessons_phext.py:114) — Build the phext blob from lessons-source.phext.
- function `sha256_file` (build_lessons_phext.py:181) — Function sha256_file takes path.
- function `committed_index` (build_lessons_phext.py:189) — Parse the per-lesson index out of the committed lessons.phext Home scroll.
- function `check` (build_lessons_phext.py:209) — Rebuild to a temp file and compare SHA-256 with the committed
- function `main` (build_lessons_phext.py:259) — Function main takes no inputs beyond the value it is called on.
- function `parse_coord` (build_mastery_phext.py:36) — Function parse_coord takes s.
- function `entry_coord` (build_mastery_phext.py:44) — Function entry_coord takes k.
- function `implode` (build_mastery_phext.py:48) — scrolls: dict coord-string -> text.
- function `render_entry` (build_mastery_phext.py:66) — Function render_entry takes k, e.
- function `render_home` (build_mastery_phext.py:78) — Function render_home takes entries.
- function `build` (build_mastery_phext.py:113) — Function build takes no inputs beyond the value it is called on.
- function `main` (build_mastery_phext.py:125) — Function main takes no inputs beyond the value it is called on.
- function `parse_coord` (build_patch_phext.py:40) — Function parse_coord takes s.
- function `implode` (build_patch_phext.py:48) — scrolls: dict coord-string -> text.
- function `render_patch` (build_patch_phext.py:66) — Function render_patch takes t.
- function `render_home` (build_patch_phext.py:81) — Function render_home takes tasks.
- function `build` (build_patch_phext.py:107) — Function build takes no inputs beyond the value it is called on.
- function `main` (build_patch_phext.py:122) — Function main takes no inputs beyond the value it is called on.
- function `parse_coord` (build_premises_phext.py:50) — Function parse_coord takes s.
- function `implode` (build_premises_phext.py:58) — scrolls: dict coord-string -> text.
- function `render_premise` (build_premises_phext.py:76) — Function render_premise takes p.
- function `render_home` (build_premises_phext.py:97) — Function render_home takes premises.
- function `build` (build_premises_phext.py:143) — Function build takes no inputs beyond the value it is called on.
- function `main` (build_premises_phext.py:158) — Function main takes no inputs beyond the value it is called on.
- function `parse_coord` (build_sql_phext.py:60) — Function parse_coord takes s.
- function `implode` (build_sql_phext.py:68) — scrolls: dict coord-string -> text.
- function `dialect_gap_count` (build_sql_phext.py:86) — Function dialect_gap_count takes d.
- function `render_dialect` (build_sql_phext.py:90) — Function render_dialect takes d.
- function `render_page` (build_sql_phext.py:111) — Function render_page takes p.
- function `render_home` (build_sql_phext.py:115) — Function render_home takes data.
- function `build` (build_sql_phext.py:159) — Function build takes no inputs beyond the value it is called on.
- function `main` (build_sql_phext.py:177) — Function main takes no inputs beyond the value it is called on.
- function `parse_coord` (build_tailscale_phext.py:28) — Function parse_coord takes s.
- function `implode` (build_tailscale_phext.py:36) — Function implode takes scrolls.
- function `build_bytes` (build_tailscale_phext.py:52) — Function build_bytes takes no inputs beyond the value it is called on.
- function `main` (build_tailscale_phext.py:69) — Function main takes no inputs beyond the value it is called on.
- function `parse_coord` (build_trainer_phext.py:42) — Function parse_coord takes s.
- function `implode` (build_trainer_phext.py:50) — Function implode takes scrolls.
- function `read_source` (build_trainer_phext.py:66) — Function read_source takes e.
- function `render_entry` (build_trainer_phext.py:73) — Function render_entry takes e, raw.
- function `render_home` (build_trainer_phext.py:95) — Function render_home takes entries.
- function `load_manifest` (build_trainer_phext.py:105) — Function load_manifest takes no inputs beyond the value it is called on.
- function `build` (build_trainer_phext.py:110) — Function build takes no inputs beyond the value it is called on.
- function `main` (build_trainer_phext.py:128) — Function main takes no inputs beyond the value it is called on.
- function `RegisterApp.__init__` (register-service/regsvc/app.py:53) — Function RegisterApp.__init__ takes self, *, entries_log, resolutions_log, credentials, sessions, ldap_auth…
- function `RegisterApp.handle` (register-service/regsvc/app.py:155) — Function RegisterApp.handle takes self, method, path, headers, body=b"".
- function `build_user_dn` (register-service/regsvc/ldap_auth.py:31) — DN for a uid, or None if the uid is not safe to interpolate.
- function `LdapAuthenticator.__init__` (register-service/regsvc/ldap_auth.py:53) — Function LdapAuthenticator.__init__ takes self, host, ca_cert_file, port=636, connection_factory=None.
- function `LdapAuthenticator.authenticate` (register-service/regsvc/ldap_auth.py:69) — Return the uid on a successful bind, None on bad credentials.
- function `build_app` (register-service/regsvc/server.py:44) — Function build_app takes env=None.
- function `selftest` (register-service/regsvc/server.py:79) — Function selftest takes env=None.
- function `Handler.do_GET` (register-service/regsvc/server.py:119) — Function Handler.do_GET takes no inputs beyond the value it is called on.
- function `Handler.do_POST` (register-service/regsvc/server.py:122) — Function Handler.do_POST takes no inputs beyond the value it is called on.
- function `Handler.log_message` (register-service/regsvc/server.py:125) — Function Handler.log_message takes self, fmt, *args.
- function `main` (register-service/regsvc/server.py:130) — Function main takes argv=None.
- function `JsonlLog.__init__` (register-service/regsvc/store.py:35) — Function JsonlLog.__init__ takes self, path.
- function `JsonlLog.append` (register-service/regsvc/store.py:46) — Append one atom dict; returns the stored atom with seq+ts.
- function `JsonlLog.read_all` (register-service/regsvc/store.py:59) — All atoms with seq > after_seq, in order.
- function `JsonlLog.last_seq` (register-service/regsvc/store.py:75) — Function JsonlLog.last_seq takes no inputs beyond the value it is called on.
- function `CredentialStore.__init__` (register-service/regsvc/store.py:86) — Function CredentialStore.__init__ takes self, path.
- function `CredentialStore.add` (register-service/regsvc/store.py:94) — Function CredentialStore.add takes self, uid, credential.
- function `CredentialStore.list_for` (register-service/regsvc/store.py:104) — Function CredentialStore.list_for takes self, uid.
- function `CredentialStore.find` (register-service/regsvc/store.py:107) — Function CredentialStore.find takes self, uid, credential_id.
- function `CredentialStore.update_sign_count` (register-service/regsvc/store.py:113) — Function CredentialStore.update_sign_count takes self, uid, credential_id, new_count.
- function `SessionStore.__init__` (register-service/regsvc/store.py:130) — Function SessionStore.__init__ takes self, path.
- function `SessionStore.create` (register-service/regsvc/store.py:151) — Function SessionStore.create takes self, session_id, record.
- function `SessionStore.get` (register-service/regsvc/store.py:156) — Function SessionStore.get takes self, session_id.
- function `SessionStore.update` (register-service/regsvc/store.py:167) — Function SessionStore.update takes self, session_id, **fields.
- function `SessionStore.delete` (register-service/regsvc/store.py:176) — Function SessionStore.delete takes self, session_id.
- function `b64url` (register-service/regsvc/webauthn_auth.py:36) — Function b64url takes data: bytes.
- function `WebAuthnCeremonies.__init__` (register-service/regsvc/webauthn_auth.py:43) — Function WebAuthnCeremonies.__init__ takes self, rp_id, origin.
- function `WebAuthnCeremonies.registration_options` (register-service/regsvc/webauthn_auth.py:48) — Function WebAuthnCeremonies.registration_options takes self, uid, existing_credentials.
- function `WebAuthnCeremonies.verify_registration` (register-service/regsvc/webauthn_auth.py:68) — Function WebAuthnCeremonies.verify_registration takes self, credential_json, expected_challenge.
- function `WebAuthnCeremonies.authentication_options` (register-service/regsvc/webauthn_auth.py:85) — Function WebAuthnCeremonies.authentication_options takes self, credentials.
- function `WebAuthnCeremonies.verify_authentication` (register-service/regsvc/webauthn_auth.py:100) — Returns the new sign count; raises WebAuthnError on failure.
- function `FakeLdap.__init__` (register-service/tests/test_app.py:15) [test] — Function FakeLdap.__init__ takes self, valid=None, down=False.
- function `FakeLdap.authenticate` (register-service/tests/test_app.py:19) [test] — Function FakeLdap.authenticate takes self, uid, password.
- function `FakeCeremonies.__init__` (register-service/tests/test_app.py:28) [test] — Function FakeCeremonies.__init__ takes no inputs beyond the value it is called on.
- function `FakeCeremonies.registration_options` (register-service/tests/test_app.py:32) [test] — Function FakeCeremonies.registration_options takes self, uid, existing.
- function `FakeCeremonies.verify_registration` (register-service/tests/test_app.py:35) [test] — Function FakeCeremonies.verify_registration takes self, credential, challenge.
- function `FakeCeremonies.authentication_options` (register-service/tests/test_app.py:42) [test] — Function FakeCeremonies.authentication_options takes self, credentials.
- function `FakeCeremonies.verify_authentication` (register-service/tests/test_app.py:45) [test] — Function FakeCeremonies.verify_authentication takes self, credential, challenge, stored.
- function `Client.__init__` (register-service/tests/test_app.py:53) [test] — Function Client.__init__ takes self, app.
- function `Client.call` (register-service/tests/test_app.py:57) [test] — Function Client.call takes self, method, path, body=None, csrf=None, raw_cookies=None.
- function `make_app` (register-service/tests/test_app.py:82) [test] — Function make_app takes tmp, numbers=.
- function `AppTestCase.setUp` (register-service/tests/test_app.py:100) [test] — Function AppTestCase.setUp takes no inputs beyond the value it is called on.
- function `AppTestCase.login` (register-service/tests/test_app.py:105) [test] — Function AppTestCase.login takes no inputs beyond the value it is called on.
- function `TestPasswordLogin.test_session_starts_logged_out` (register-service/tests/test_app.py:113) [test] — Function TestPasswordLogin.test_session_starts_logged_out takes no inputs beyond the value it is called on.
- function `TestPasswordLogin.test_bad_password_401_no_cookie` (register-service/tests/test_app.py:117) [test] — Function TestPasswordLogin.test_bad_password_401_no_cookie takes no inputs beyond the value it is called on.
- function `TestPasswordLogin.test_ldap_down_503` (register-service/tests/test_app.py:123) [test] — Function TestPasswordLogin.test_ldap_down_503 takes no inputs beyond the value it is called on.
- function `TestPasswordLogin.test_login_then_session` (register-service/tests/test_app.py:129) [test] — Function TestPasswordLogin.test_login_then_session takes no inputs beyond the value it is called on.
- function `TestPasswordLogin.test_tampered_cookie_rejected` (register-service/tests/test_app.py:135) [test] — Function TestPasswordLogin.test_tampered_cookie_rejected takes no inputs beyond the value it is called on.
- function `TestPasswordLogin.test_logout` (register-service/tests/test_app.py:144) [test] — Function TestPasswordLogin.test_logout takes no inputs beyond the value it is called on.
- function `TestEntries.test_entries_require_auth` (register-service/tests/test_app.py:154) [test] — Function TestEntries.test_entries_require_auth takes no inputs beyond the value it is called on.
- function `TestEntries.test_entries_require_csrf` (register-service/tests/test_app.py:161) [test] — Function TestEntries.test_entries_require_csrf takes no inputs beyond the value it is called on.
- function `TestEntries.test_add_and_list_entries` (register-service/tests/test_app.py:167) [test] — Function TestEntries.test_add_and_list_entries takes no inputs beyond the value it is called on.
- function `TestEntries.test_empty_and_huge_entries_rejected` (register-service/tests/test_app.py:185) [test] — Function TestEntries.test_empty_and_huge_entries_rejected takes no inputs beyond the value it is called on.
- function `TestDone.test_done_records_resolution_atom` (register-service/tests/test_app.py:196) [test] — Function TestDone.test_done_records_resolution_atom takes no inputs beyond the value it is called on.
- function `TestDone.test_done_twice_records_both_atoms` (register-service/tests/test_app.py:208) [test] — Function TestDone.test_done_twice_records_both_atoms takes no inputs beyond the value it is called on.
- function `TestDone.test_done_unknown_number_404` (register-service/tests/test_app.py:218) [test] — Function TestDone.test_done_unknown_number_404 takes no inputs beyond the value it is called on.
- function `TestDone.test_done_requires_auth_and_csrf` (register-service/tests/test_app.py:224) [test] — Function TestDone.test_done_requires_auth_and_csrf takes no inputs beyond the value it is called on.
- function `TestDone.test_done_numbers_file_missing_503` (register-service/tests/test_app.py:231) [test] — Function TestDone.test_done_numbers_file_missing_503 takes no inputs beyond the value it is called on.
- function `TestDone.test_done_with_note_stores_note` (register-service/tests/test_app.py:241) [test] — Function TestDone.test_done_with_note_stores_note takes no inputs beyond the value it is called on.
- function `TestDone.test_done_no_body_no_note_field` (register-service/tests/test_app.py:254) [test] — Function TestDone.test_done_no_body_no_note_field takes no inputs beyond the value it is called on.
- function `TestDone.test_done_blank_note_omitted` (register-service/tests/test_app.py:263) [test] — Function TestDone.test_done_blank_note_omitted takes no inputs beyond the value it is called on.
- function `TestDone.test_done_note_at_cap_accepted` (register-service/tests/test_app.py:271) [test] — Function TestDone.test_done_note_at_cap_accepted takes no inputs beyond the value it is called on.
- function `TestDone.test_done_note_over_cap_400` (register-service/tests/test_app.py:279) [test] — Function TestDone.test_done_note_over_cap_400 takes no inputs beyond the value it is called on.
- function `TestDone.test_done_malformed_json_400` (register-service/tests/test_app.py:287) [test] — Function TestDone.test_done_malformed_json_400 takes no inputs beyond the value it is called on.
- function `TestDone.test_done_non_object_body_400` (register-service/tests/test_app.py:300) [test] — Function TestDone.test_done_non_object_body_400 takes no inputs beyond the value it is called on.
- function `TestDone.test_done_non_string_note_400` (register-service/tests/test_app.py:306) [test] — Function TestDone.test_done_non_string_note_400 takes no inputs beyond the value it is called on.
- function `TestCancel.test_cancel_records_cancelled_atom_with_note` (register-service/tests/test_app.py:319) [test] — Function TestCancel.test_cancel_records_cancelled_atom_with_note takes no inputs beyond the value it is called on.
- function `TestCancel.test_cancel_no_body_outcome_no_note` (register-service/tests/test_app.py:335) [test] — Function TestCancel.test_cancel_no_body_outcome_no_note takes no inputs beyond the value it is called on.
- function `TestCancel.test_cancel_unknown_number_404` (register-service/tests/test_app.py:345) [test] — Function TestCancel.test_cancel_unknown_number_404 takes no inputs beyond the value it is called on.
- function `TestCancel.test_cancel_note_over_cap_400` (register-service/tests/test_app.py:351) [test] — Function TestCancel.test_cancel_note_over_cap_400 takes no inputs beyond the value it is called on.
- function `TestPasskeys.enroll` (register-service/tests/test_app.py:363) [test] — Function TestPasskeys.enroll takes no inputs beyond the value it is called on.
- function `TestPasskeys.test_register_options_requires_session` (register-service/tests/test_app.py:374) [test] — Function TestPasskeys.test_register_options_requires_session takes no inputs beyond the value it is called on.
- function `TestPasskeys.test_verify_without_options_400` (register-service/tests/test_app.py:379) [test] — Function TestPasskeys.test_verify_without_options_400 takes no inputs beyond the value it is called on.
- function `TestPasskeys.test_enroll_stores_credential` (register-service/tests/test_app.py:386) [test] — Function TestPasskeys.test_enroll_stores_credential takes no inputs beyond the value it is called on.
- function `TestPasskeys.test_enroll_failure_stores_nothing` (register-service/tests/test_app.py:392) [test] — Function TestPasskeys.test_enroll_failure_stores_nothing takes no inputs beyond the value it is called on.
- function `TestPasskeys.test_login_options_unknown_user_404` (register-service/tests/test_app.py:403) [test] — Function TestPasskeys.test_login_options_unknown_user_404 takes no inputs beyond the value it is called on.
- function `TestPasskeys.test_passkey_login_happy_path` (register-service/tests/test_app.py:408) [test] — Function TestPasskeys.test_passkey_login_happy_path takes no inputs beyond the value it is called on.
- function `TestPasskeys.test_passkey_login_bad_assertion_401` (register-service/tests/test_app.py:425) [test] — Function TestPasskeys.test_passkey_login_bad_assertion_401 takes no inputs beyond the value it is called on.
- function `TestPasskeys.test_passkey_verify_without_options_401` (register-service/tests/test_app.py:437) [test] — Function TestPasskeys.test_passkey_verify_without_options_401 takes no inputs beyond the value it is called on.
- function `TestPasskeys.test_passkey_unknown_credential_401` (register-service/tests/test_app.py:443) [test] — Function TestPasskeys.test_passkey_unknown_credential_401 takes no inputs beyond the value it is called on.
- function `TestDrainToken.setUp` (register-service/tests/test_app.py:454) [test] — Function TestDrainToken.setUp takes no inputs beyond the value it is called on.
- function `TestDrainToken.call` (register-service/tests/test_app.py:470) [test] — Function TestDrainToken.call takes self, method, path, headers=None, body=None.
- function `TestDrainToken.test_drain_token_reads_both_logs` (register-service/tests/test_app.py:475) [test] — Function TestDrainToken.test_drain_token_reads_both_logs takes no inputs beyond the value it is called on.
- function `TestDrainToken.test_wrong_drain_token_401` (register-service/tests/test_app.py:483) [test] — Function TestDrainToken.test_wrong_drain_token_401 takes no inputs beyond the value it is called on.
- function `TestDrainToken.test_drain_token_cannot_write` (register-service/tests/test_app.py:488) [test] — Function TestDrainToken.test_drain_token_cannot_write takes no inputs beyond the value it is called on.
- function `FakeConn.__init__` (register-service/tests/test_ldap_auth.py:13) [test] — Function FakeConn.__init__ takes self, who=None.
- function `Std.who_am_i` (register-service/tests/test_ldap_auth.py:20) [test] — Function Std.who_am_i takes no inputs beyond the value it is called on.
- function `FakeConn.unbind` (register-service/tests/test_ldap_auth.py:28) [test] — Function FakeConn.unbind takes no inputs beyond the value it is called on.
- function `factory_returning` (register-service/tests/test_ldap_auth.py:32) [test] — Function factory_returning takes conn.
- function `make` (register-service/tests/test_ldap_auth.py:33) [test] — Function make takes server, user_dn, password.
- function `factory_raising` (register-service/tests/test_ldap_auth.py:39) [test] — Function factory_raising takes exc.
- function `make` (register-service/tests/test_ldap_auth.py:40) [test] — Function make takes server, user_dn, password.
- function `TestBuildUserDn.test_normal_uid` (register-service/tests/test_ldap_auth.py:46) [test] — Function TestBuildUserDn.test_normal_uid takes no inputs beyond the value it is called on.
- function `TestBuildUserDn.test_injection_attempts_rejected` (register-service/tests/test_ldap_auth.py:49) [test] — Function TestBuildUserDn.test_injection_attempts_rejected takes no inputs beyond the value it is called on.
- function `TestAuthenticate.auth` (register-service/tests/test_ldap_auth.py:56) [test] — Function TestAuthenticate.auth takes self, factory.
- function `TestAuthenticate.test_success` (register-service/tests/test_ldap_auth.py:60) [test] — Function TestAuthenticate.test_success takes no inputs beyond the value it is called on.
- function `TestAuthenticate.test_invalid_credentials_fail_closed` (register-service/tests/test_ldap_auth.py:67) [test] — Function TestAuthenticate.test_invalid_credentials_fail_closed takes no inputs beyond the value it is called on.
- function `TestAuthenticate.test_invalid_credentials_result_returns_none` (register-service/tests/test_ldap_auth.py:73) [test] — Function TestAuthenticate.test_invalid_credentials_result_returns_none takes no inputs beyond the value it is called on.
- function `TestAuthenticate.test_unreachable_raises_unavailable` (register-service/tests/test_ldap_auth.py:85) [test] — Function TestAuthenticate.test_unreachable_raises_unavailable takes no inputs beyond the value it is called on.
- function `TestAuthenticate.test_generic_transport_error_raises_unavailable` (register-service/tests/test_ldap_auth.py:90) [test] — Function TestAuthenticate.test_generic_transport_error_raises_unavailable takes no inputs beyond the value it is called on.
- function `TestAuthenticate.test_empty_password_never_touches_network` (register-service/tests/test_ldap_auth.py:95) [test] — Function TestAuthenticate.test_empty_password_never_touches_network takes no inputs beyond the value it is called on.
- function `TestAuthenticate.make` (register-service/tests/test_ldap_auth.py:98) [test] — Function TestAuthenticate.make takes server, user_dn, password.
- function `TestAuthenticate.test_bad_uid_never_touches_network` (register-service/tests/test_ldap_auth.py:106) [test] — Function TestAuthenticate.test_bad_uid_never_touches_network takes no inputs beyond the value it is called on.
- function `TestAuthenticate.make` (register-service/tests/test_ldap_auth.py:109) [test] — Function TestAuthenticate.make takes server, user_dn, password.
- function `TestAuthenticate.test_whoami_mismatch_rejected` (register-service/tests/test_ldap_auth.py:117) [test] — Function TestAuthenticate.test_whoami_mismatch_rejected takes no inputs beyond the value it is called on.
- function `Client.__init__` (register-service/tests/test_qexo_token.py:27) [test] — Function Client.__init__ takes self, app.
- function `Client.call` (register-service/tests/test_qexo_token.py:31) [test] — Function Client.call takes self, method, path, body=None, headers=None, csrf=None.
- function `make_app` (register-service/tests/test_qexo_token.py:55) [test] — Function make_app takes tmp, numbers=.
- function `read_log` (register-service/tests/test_qexo_token.py:74) [test] — Function read_log takes tmp, name.
- function `QexoTokenTestCase.setUp` (register-service/tests/test_qexo_token.py:83) [test] — Function QexoTokenTestCase.setUp takes no inputs beyond the value it is called on.
- function `QexoTokenTestCase.qexo` (register-service/tests/test_qexo_token.py:88) [test] — Function QexoTokenTestCase.qexo takes no inputs beyond the value it is called on.
- function `QexoTokenTestCase.drain` (register-service/tests/test_qexo_token.py:91) [test] — Function QexoTokenTestCase.drain takes no inputs beyond the value it is called on.
- function `TestPositiveLegs.test_token_appends_an_entry_as_qexo` (register-service/tests/test_qexo_token.py:96) [test] — Function TestPositiveLegs.test_token_appends_an_entry_as_qexo takes no inputs beyond the value it is called on.
- function `TestPositiveLegs.test_token_records_done_and_cancel_with_note` (register-service/tests/test_qexo_token.py:115) [test] — Function TestPositiveLegs.test_token_records_done_and_cancel_with_note takes no inputs beyond the value it is called on.
- function `TestPositiveLegs.test_body_cannot_spoof_another_user` (register-service/tests/test_qexo_token.py:130) [test] — Function TestPositiveLegs.test_body_cannot_spoof_another_user takes no inputs beyond the value it is called on.
- function `TestNegativeLegs.test_wrong_and_absent_tokens_refuse` (register-service/tests/test_qexo_token.py:140) [test] — Function TestNegativeLegs.test_wrong_and_absent_tokens_refuse takes no inputs beyond the value it is called on.
- function `TestNegativeLegs.test_drain_token_cannot_write` (register-service/tests/test_qexo_token.py:152) [test] — Function TestNegativeLegs.test_drain_token_cannot_write takes no inputs beyond the value it is called on.
- function `TestNegativeLegs.test_qexo_token_cannot_read` (register-service/tests/test_qexo_token.py:161) [test] — Function TestNegativeLegs.test_qexo_token_cannot_read takes no inputs beyond the value it is called on.
- function `TestNegativeLegs.test_qexo_token_grants_no_login` (register-service/tests/test_qexo_token.py:168) [test] — Function TestNegativeLegs.test_qexo_token_grants_no_login takes no inputs beyond the value it is called on.
- function `TestNegativeLegs.test_unset_qexo_token_fails_closed` (register-service/tests/test_qexo_token.py:184) [test] — Function TestNegativeLegs.test_unset_qexo_token_fails_closed takes no inputs beyond the value it is called on.
- function `TestNegativeLegs.test_number_and_note_contracts_still_apply` (register-service/tests/test_qexo_token.py:196) [test] — Function TestNegativeLegs.test_number_and_note_contracts_still_apply takes no inputs beyond the value it is called on.
- function `TestSessionPathUnchanged.test_session_append_and_close_still_record_the_user` (register-service/tests/test_qexo_token.py:214) [test] — Function TestSessionPathUnchanged.test_session_append_and_close_still_record_the_user takes no inputs beyond the value it is called on.
- function `TestJsonlLog.setUp` (register-service/tests/test_store.py:10) [test] — Function TestJsonlLog.setUp takes no inputs beyond the value it is called on.
- function `TestJsonlLog.test_append_assigns_monotonic_seq_and_ts` (register-service/tests/test_store.py:14) [test] — Function TestJsonlLog.test_append_assigns_monotonic_seq_and_ts takes no inputs beyond the value it is called on.
- function `TestJsonlLog.test_read_all_and_after_seq` (register-service/tests/test_store.py:22) [test] — Function TestJsonlLog.test_read_all_and_after_seq takes no inputs beyond the value it is called on.
- function `TestJsonlLog.test_seq_survives_reopen` (register-service/tests/test_store.py:32) [test] — Function TestJsonlLog.test_seq_survives_reopen takes no inputs beyond the value it is called on.
- function `TestJsonlLog.test_duplicate_atoms_are_both_recorded` (register-service/tests/test_store.py:38) [test] — Function TestJsonlLog.test_duplicate_atoms_are_both_recorded takes no inputs beyond the value it is called on.
- function `TestCredentialStore.setUp` (register-service/tests/test_store.py:51) [test] — Function TestCredentialStore.setUp takes no inputs beyond the value it is called on.
- function `TestCredentialStore.test_add_find_list` (register-service/tests/test_store.py:58) [test] — Function TestCredentialStore.test_add_find_list takes no inputs beyond the value it is called on.
- function `TestCredentialStore.test_add_duplicate_id_rejected` (register-service/tests/test_store.py:66) [test] — Function TestCredentialStore.test_add_duplicate_id_rejected takes no inputs beyond the value it is called on.
- function `TestCredentialStore.test_sign_count_update_persists` (register-service/tests/test_store.py:72) [test] — Function TestCredentialStore.test_sign_count_update_persists takes no inputs beyond the value it is called on.
- function `TestSessionStore.setUp` (register-service/tests/test_store.py:82) [test] — Function TestSessionStore.setUp takes no inputs beyond the value it is called on.
- function `TestSessionStore.test_create_get_delete` (register-service/tests/test_store.py:86) [test] — Function TestSessionStore.test_create_get_delete takes no inputs beyond the value it is called on.
- function `TestSessionStore.test_expired_session_is_gone` (register-service/tests/test_store.py:96) [test] — Function TestSessionStore.test_expired_session_is_gone takes no inputs beyond the value it is called on.
- function `TestSessionStore.test_update` (register-service/tests/test_store.py:104) [test] — Function TestSessionStore.test_update takes no inputs beyond the value it is called on.
- function `TestOptions.test_registration_options_shape` (register-service/tests/test_webauthn_auth.py:9) [test] — Function TestOptions.test_registration_options_shape takes no inputs beyond the value it is called on.
- function `TestOptions.test_authentication_options_shape` (register-service/tests/test_webauthn_auth.py:15) [test] — Function TestOptions.test_authentication_options_shape takes no inputs beyond the value it is called on.
- function `TestVerifyFailsClosed.test_registration_garbage_credential_raises` (register-service/tests/test_webauthn_auth.py:24) [test] — Function TestVerifyFailsClosed.test_registration_garbage_credential_raises takes no inputs beyond the value it is called on.
- function `TestVerifyFailsClosed.test_authentication_garbage_credential_raises` (register-service/tests/test_webauthn_auth.py:30) [test] — Function TestVerifyFailsClosed.test_authentication_garbage_credential_raises takes no inputs beyond the value it is called on.
- function `clean_md` (register-ui/generate_register_page.py:42) — Strip light markdown: links -> text, backticks/bold markers out.
- function `parse_register` (register-ui/generate_register_page.py:49) — Parse numbered items out of the register markdown.
- function `fallback_action` (register-ui/generate_register_page.py:88) — Action text from the register prose when the mirror has none.
- function `linkify` (register-ui/generate_register_page.py:98) — Wrap bare URLs in anchors (input is already HTML-escaped).
- function `repl` (register-ui/generate_register_page.py:100) — Function repl takes m.
- function `main` (register-ui/generate_register_page.py:674) — Function main takes no inputs beyond the value it is called on.

---

## function — gpt_day

Found in build_accounting_phext.py at line 45.

Surface: public API.

Signature: `def gpt_day(d)`

What it does: Function gpt_day takes d. Its return type is unannotated; the code shows it returning (d - EPOCH).days + 1.

(Where these words came from: inferred.)

---

## function — week_of

Found in build_accounting_phext.py at line 48.

Surface: public API.

Signature: `def week_of(d)`

What it does: Function week_of takes d. Its return type is unannotated; the code shows it returning (sunday - WEEK1_SUNDAY).days // 7 + 1, sunday.

(Where these words came from: inferred.)

---

## function — net

Found in build_accounting_phext.py at line 52.

Surface: public API.

Signature: `def net(a, d)`

What it does: Function net takes a, d. Its return type is unannotated; the code shows it returning a - d.

(Where these words came from: inferred.)

---

## function — build

Found in build_accounting_phext.py at line 55.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — lag_line

Found in build_accounting_phext.py at line 79.

Surface: public API.

Signature: `def lag_line(atoms)`

What it does: Function lag_line takes atoms. Its return type is unannotated; the code shows it returning "lag: 0 timed deliveries"; f"lag: {len(atoms)} timed deliveries · mean {mea.

(Where these words came from: inferred.)

---

## function — day_lines

Found in build_accounting_phext.py at line 85.

Surface: public API.

Signature: `def day_lines(ds)`

What it does: Function day_lines takes ds. Its return type is unannotated; the code shows it returning "\n".join(out).

(Where these words came from: inferred.)

---

## function — week_lines

Found in build_accounting_phext.py at line 102.

Surface: public API.

Signature: `def week_lines(w, sunday, dss)`

What it does: Function week_lines takes w, sunday, dss. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — tot

Found in build_accounting_phext.py at line 107.

Surface: public API.

Signature: `def tot(k)`

What it does: Function tot takes k. Its return type is unannotated; the code shows it returning sum(g.get(k, 0) for g in gs).

(Where these words came from: inferred.)

---

## function — rhu

Found in build_accounting_phext.py at line 155.

Surface: public API.

Signature: `def rhu(x)`

What it does: Function rhu takes x. Its return type is unannotated; the code shows it returning int(x + 0.5) if x >= 0 else -int(-x + 0.5).

(Where these words came from: inferred.)

---

## function — avgf

Found in build_accounting_phext.py at line 160.

Surface: public API.

Signature: `def avgf(k)`

What it does: Function avgf takes k. Its return type is unannotated; the code shows it returning sum(a.get(k, 0) for a in full_base) / len(full_b.

(Where these words came from: inferred.)

---

## function — btot

Found in build_accounting_phext.py at line 186.

Surface: public API.

Signature: `def btot(k, aa=aa)`

What it does: Function btot takes k, aa=aa. Its return type is unannotated; the code shows it returning sum(a.get(k, 0) for a in aa).

(Where these words came from: inferred.)

---

## function — t_all

Found in build_accounting_phext.py at line 202.

Surface: public API.

Signature: `def t_all(k)`

What it does: Function t_all takes k. Its return type is unannotated; the code shows it returning sum(a.get(k, 0) for a in baseline).

(Where these words came from: inferred.)

---

## function — parse_coord

Found in build_artifacts_phext.py at line 52.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — implode

Found in build_artifacts_phext.py at line 60.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord-string -> text. Same state machine as

(Where these words came from: direct.)

---

## function — read_source

Found in build_artifacts_phext.py at line 78.

Surface: public API.

Signature: `def read_source(a)`

What it does: Function read_source takes a. Its return type is unannotated; the code shows it returning raw, hashlib.sha256(raw).hexdigest().

(Where these words came from: inferred.)

---

## function — render_artifact

Found in build_artifacts_phext.py at line 85.

Surface: public API.

Signature: `def render_artifact(a, raw)`

What it does: Function render_artifact takes a, raw. Its return type is unannotated; the code shows it returning scroll.

(Where these words came from: inferred.)

---

## function — render_home

Found in build_artifacts_phext.py at line 104.

Surface: public API.

Signature: `def render_home(artifacts)`

What it does: Function render_home takes artifacts. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — load_manifest

Found in build_artifacts_phext.py at line 132.

Surface: public API.

Signature: `def load_manifest()`

What it does: Function load_manifest takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning json.load(f)["artifacts"].

(Where these words came from: inferred.)

---

## function — build

Found in build_artifacts_phext.py at line 137.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls), artifacts, stale.

(Where these words came from: inferred.)

---

## function — main

Found in build_artifacts_phext.py at line 155.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — parse_coord

Found in build_barnacles_phext.py at line 42.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — entry_coord

Found in build_barnacles_phext.py at line 50.

Surface: public API.

Signature: `def entry_coord(n)`

What it does: Entry n (1-indexed) -> 2.1.1/1.1.1/S.T.B (hundred/ten/unit).

(Where these words came from: direct.)

---

## function — implode

Found in build_barnacles_phext.py at line 56.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: Function implode takes scrolls. Its return type is unannotated; the code shows it returning bytes(out).

(Where these words came from: inferred.)

---

## function — render_entry

Found in build_barnacles_phext.py at line 72.

Surface: public API.

Signature: `def render_entry(e, data)`

What it does: Function render_entry takes e, data. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — render_home

Found in build_barnacles_phext.py at line 92.

Surface: public API.

Signature: `def render_home(data)`

What it does: Function render_home takes data. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — build

Found in build_barnacles_phext.py at line 142.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls), data.

(Where these words came from: inferred.)

---

## function — main

Found in build_barnacles_phext.py at line 161.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — parse_coord

Found in build_contests_phext.py at line 44.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — entry_coord

Found in build_contests_phext.py at line 52.

Surface: public API.

Signature: `def entry_coord(k)`

What it does: Function entry_coord takes k. Its return type is unannotated; the code shows it returning f"1.1.1/1.1.1/1.1.{k + 1}".

(Where these words came from: inferred.)

---

## function — implode

Found in build_contests_phext.py at line 56.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord-string -> text. Same state machine as

(Where these words came from: direct.)

---

## function — render_contest

Found in build_contests_phext.py at line 74.

Surface: public API.

Signature: `def render_contest(k, c)`

What it does: Function render_contest takes k, c. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — render_out_of_scope

Found in build_contests_phext.py at line 96.

Surface: public API.

Signature: `def render_out_of_scope(k, o)`

What it does: Function render_out_of_scope takes k, o. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — render_home

Found in build_contests_phext.py at line 108.

Surface: public API.

Signature: `def render_home(contests)`

What it does: Function render_home takes contests. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — build

Found in build_contests_phext.py at line 161.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls), contests.

(Where these words came from: inferred.)

---

## function — main

Found in build_contests_phext.py at line 178.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — B

Found in build_costs_phext.py at line 17.

Surface: public API.

Signature: `def B(x)`

What it does: Function B takes x. Its return type is unannotated; the code shows it returning f"${x/1e9:,.1f}B".

(Where these words came from: inferred.)

---

## function — build

Found in build_costs_phext.py at line 20.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning Phext().implode(table).

(Where these words came from: inferred.)

---

## function — parse_coord

Found in build_doomer_phext.py at line 34.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — implode

Found in build_doomer_phext.py at line 42.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: Function implode takes scrolls. Its return type is unannotated; the code shows it returning bytes(out).

(Where these words came from: inferred.)

---

## function — render_entry

Found in build_doomer_phext.py at line 58.

Surface: public API.

Signature: `def render_entry(e)`

What it does: Function render_entry takes e. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — render_home

Found in build_doomer_phext.py at line 72.

Surface: public API.

Signature: `def render_home(data)`

What it does: Function render_home takes data. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — build

Found in build_doomer_phext.py at line 97.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls), data.

(Where these words came from: inferred.)

---

## function — main

Found in build_doomer_phext.py at line 112.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — parse_coord

Found in build_humanities_phext.py at line 48.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — implode

Found in build_humanities_phext.py at line 56.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord-string -> text. Same state machine as

(Where these words came from: direct.)

---

## function — read_source

Found in build_humanities_phext.py at line 74.

Surface: public API.

Signature: `def read_source(a)`

What it does: Function read_source takes a. Its return type is unannotated; the code shows it returning raw, hashlib.sha256(raw).hexdigest().

(Where these words came from: inferred.)

---

## function — render_artifact

Found in build_humanities_phext.py at line 81.

Surface: public API.

Signature: `def render_artifact(a, raw)`

What it does: Function render_artifact takes a, raw. Its return type is unannotated; the code shows it returning scroll.

(Where these words came from: inferred.)

---

## function — render_home

Found in build_humanities_phext.py at line 100.

Surface: public API.

Signature: `def render_home(documents)`

What it does: Function render_home takes documents. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — load_manifest

Found in build_humanities_phext.py at line 129.

Surface: public API.

Signature: `def load_manifest()`

What it does: Function load_manifest takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning json.load(f)["documents"].

(Where these words came from: inferred.)

---

## function — build

Found in build_humanities_phext.py at line 134.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls), documents, stale.

(Where these words came from: inferred.)

---

## function — main

Found in build_humanities_phext.py at line 152.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — reg_t3_coord

Found in build_ietf_phext.py at line 65.

Surface: public API.

Signature: `def reg_t3_coord(rfc)`

What it does: Function reg_t3_coord takes rfc. Its return type is unannotated; the code shows it returning f"1.1.1/1.1.2/1.3.{rfc // 1000 + 1}".

(Where these words came from: inferred.)

---

## function — parse_coord

Found in build_ietf_phext.py at line 69.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — implode

Found in build_ietf_phext.py at line 77.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: Same state machine as build_tasks_phext.py / build_skills_phext.py.

(Where these words came from: direct.)

---

## function — clean

Found in build_ietf_phext.py at line 94.

Surface: public API.

Signature: `def clean(s)`

What it does: Strip phext delimiter chars from generated metadata text.

(Where these words came from: direct.)

---

## function — lst

Found in build_ietf_phext.py at line 99.

Surface: public API.

Signature: `def lst(nums)`

What it does: Function lst takes nums. Its return type is unannotated; the code shows it returning ", ".join(str(n) for n in nums) if nums else "--.

(Where these words came from: inferred.)

---

## function — capped

Found in build_ietf_phext.py at line 103.

Surface: public API.

Signature: `def capped(nums, cap=LIST_CAP)`

What it does: Function capped takes nums, cap=LIST_CAP. Its return type is unannotated; the code shows it returning lst(nums); lst(nums[:cap]) + f", ... (+{len(nums) - cap} mo.

(Where these words came from: inferred.)

---

## function — subseries_str

Found in build_ietf_phext.py at line 109.

Surface: public API.

Signature: `def subseries_str(e)`

What it does: Function subseries_str takes e. Its return type is unannotated; the code shows it returning "; ".join(parts) if parts else "--".

(Where these words came from: inferred.)

---

## function — escape_text

Found in build_ietf_phext.py at line 120.

Surface: public API.

Signature: `def escape_text(text, expected_delims, rfc)`

What it does: Function escape_text takes text, expected_delims, rfc. Its return type is unannotated; the code shows it returning "".join(out).

(Where these words came from: inferred.)

---

## function — refs_of

Found in build_ietf_phext.py at line 133.

Surface: public API.

Signature: `def refs_of(e, kind)`

What it does: Function refs_of takes e, kind. Its return type is unannotated; the code shows it returning [r["to_rfc"] for r in e["references"] if r["kind.

(Where these words came from: inferred.)

---

## function — render_rfc_header

Found in build_ietf_phext.py at line 137.

Surface: public API.

Signature: `def render_rfc_header(e, src)`

What it does: Function render_rfc_header takes e, src. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — render_rfc

Found in build_ietf_phext.py at line 246.

Surface: public API.

Signature: `def render_rfc(e, src, corpus)`

What it does: Function render_rfc takes e, src, corpus. Its return type is unannotated; the code shows it returning head + "\n" + body; head + "\n--- RFC TEXT AS SERVED BY THE RFC EDIT.

(Where these words came from: inferred.)

---

## function — impl_short

Found in build_ietf_phext.py at line 282.

Surface: public API.

Signature: `def impl_short(e)`

What it does: Function impl_short takes e. Its return type is unannotated; the code shows it returning "impl: none identified"; "impl: unmapped"; f"impl [{e['impl_status']}]: {names}".

(Where these words came from: inferred.)

---

## function — registry_line

Found in build_ietf_phext.py at line 292.

Surface: public API.

Signature: `def registry_line(e, src)`

What it does: Function registry_line takes e, src. Its return type is unannotated; the code shows it returning " | ".join([f"RFC {e['rfc']}", e["coordinate"],; " | ".join(parts).

(Where these words came from: inferred.)

---

## function — render_home

Found in build_ietf_phext.py at line 336.

Surface: public API.

Signature: `def render_home(src)`

What it does: Function render_home takes src. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — render_registry_overview

Found in build_ietf_phext.py at line 457.

Surface: public API.

Signature: `def render_registry_overview(src)`

What it does: Function render_registry_overview takes src. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — build

Found in build_ietf_phext.py at line 503.

Surface: public API.

Signature: `def build(source_path, corpus)`

What it does: Function build takes source_path, corpus. Its return type is unannotated; the code shows it returning implode(scrolls), src.

(Where these words came from: inferred.)

---

## function — main

Found in build_ietf_phext.py at line 532.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — opt

Found in build_ietf_phext.py at line 536.

Surface: public API.

Signature: `def opt(name, env, default=None)`

What it does: Function opt takes name, env, default=None. Its return type is unannotated; the code shows it returning args[args.index(name) + 1]; os.environ.get(env, default).

(Where these words came from: inferred.)

---

## function — parse_coord

Found in build_jobs_phext.py at line 50.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — implode

Found in build_jobs_phext.py at line 58.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord-string -> text. Same state machine as

(Where these words came from: direct.)

---

## function — render_job

Found in build_jobs_phext.py at line 76.

Surface: public API.

Signature: `def render_job(j)`

What it does: Function render_job takes j. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — render_home

Found in build_jobs_phext.py at line 95.

Surface: public API.

Signature: `def render_home(jobs)`

What it does: Function render_home takes jobs. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — build

Found in build_jobs_phext.py at line 125.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls), jobs.

(Where these words came from: inferred.)

---

## function — main

Found in build_jobs_phext.py at line 143.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — parse_coord

Found in build_journal_phext.py at line 50.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — implode

Found in build_journal_phext.py at line 58.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord-string -> text. Same state machine as

(Where these words came from: direct.)

---

## function — render_entry

Found in build_journal_phext.py at line 76.

Surface: public API.

Signature: `def render_entry(e)`

What it does: Function render_entry takes e. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — render_home

Found in build_journal_phext.py at line 93.

Surface: public API.

Signature: `def render_home(entries)`

What it does: Function render_home takes entries. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — build

Found in build_journal_phext.py at line 160.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls), entries.

(Where these words came from: inferred.)

---

## function — main

Found in build_journal_phext.py at line 175.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — c

Found in build_lessons_phext.py at line 48.

Surface: public API.

Signature: `def c(s)`

What it does: Function c takes s. Its return type is unannotated; the code shows it returning tuple(int(x) for part in s.split("/") for x in p.

(Where these words came from: inferred.)

---

## function — fmt

Found in build_lessons_phext.py at line 52.

Surface: public API.

Signature: `def fmt(coord)`

What it does: Function fmt takes coord. Its return type is unannotated; the code shows it returning "%d.%d.%d/%d.%d.%d/%d.%d.%d" % coord.

(Where these words came from: inferred.)

---

## function — parse_bytes

Found in build_lessons_phext.py at line 56.

Surface: public API.

Signature: `def parse_bytes(data)`

What it does: Dead-reckoning parse of raw phext bytes -> {coord_str: raw_bytes} for

(Where these words came from: direct.)

---

## function — implode

Found in build_lessons_phext.py at line 84.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord-string -> bytes. Same state machine as build_skills_phext.py.

(Where these words came from: direct.)

---

## function — lesson_meta

Found in build_lessons_phext.py at line 101.

Surface: public API.

Signature: `def lesson_meta(coord_s, raw)`

What it does: (short statement, date given, mark) from one lesson scroll.

(Where these words came from: direct.)

---

## function — build

Found in build_lessons_phext.py at line 114.

Surface: public API.

Signature: `def build()`

What it does: Build the phext blob from lessons-source.phext.

(Where these words came from: direct.)

---

## function — sha256_file

Found in build_lessons_phext.py at line 181.

Surface: public API.

Signature: `def sha256_file(path)`

What it does: Function sha256_file takes path. Its return type is unannotated; the code shows it returning h.hexdigest().

(Where these words came from: inferred.)

---

## function — committed_index

Found in build_lessons_phext.py at line 189.

Surface: public API.

Signature: `def committed_index()`

What it does: Parse the per-lesson index out of the committed lessons.phext Home scroll.

(Where these words came from: direct.)

---

## function — check

Found in build_lessons_phext.py at line 209.

Surface: public API.

Signature: `def check()`

What it does: Rebuild to a temp file and compare SHA-256 with the committed

(Where these words came from: direct.)

---

## function — main

Found in build_lessons_phext.py at line 259.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — parse_coord

Found in build_mastery_phext.py at line 36.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — entry_coord

Found in build_mastery_phext.py at line 44.

Surface: public API.

Signature: `def entry_coord(k)`

What it does: Function entry_coord takes k. Its return type is unannotated; the code shows it returning f"1.1.1/1.1.1/1.1.{k + 1}".

(Where these words came from: inferred.)

---

## function — implode

Found in build_mastery_phext.py at line 48.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord-string -> text. Same state machine as

(Where these words came from: direct.)

---

## function — render_entry

Found in build_mastery_phext.py at line 66.

Surface: public API.

Signature: `def render_entry(k, e)`

What it does: Function render_entry takes k, e. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — render_home

Found in build_mastery_phext.py at line 78.

Surface: public API.

Signature: `def render_home(entries)`

What it does: Function render_home takes entries. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — build

Found in build_mastery_phext.py at line 113.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls), entries.

(Where these words came from: inferred.)

---

## function — main

Found in build_mastery_phext.py at line 125.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — parse_coord

Found in build_patch_phext.py at line 40.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — implode

Found in build_patch_phext.py at line 48.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord-string -> text. Same state machine as

(Where these words came from: direct.)

---

## function — render_patch

Found in build_patch_phext.py at line 66.

Surface: public API.

Signature: `def render_patch(t)`

What it does: Function render_patch takes t. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — render_home

Found in build_patch_phext.py at line 81.

Surface: public API.

Signature: `def render_home(tasks)`

What it does: Function render_home takes tasks. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — build

Found in build_patch_phext.py at line 107.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls), tasks.

(Where these words came from: inferred.)

---

## function — main

Found in build_patch_phext.py at line 122.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — parse_coord

Found in build_premises_phext.py at line 50.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — implode

Found in build_premises_phext.py at line 58.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord-string -> text. Same state machine as

(Where these words came from: direct.)

---

## function — render_premise

Found in build_premises_phext.py at line 76.

Surface: public API.

Signature: `def render_premise(p)`

What it does: Function render_premise takes p. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — render_home

Found in build_premises_phext.py at line 97.

Surface: public API.

Signature: `def render_home(premises)`

What it does: Function render_home takes premises. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — build

Found in build_premises_phext.py at line 143.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls), premises.

(Where these words came from: inferred.)

---

## function — main

Found in build_premises_phext.py at line 158.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — parse_coord

Found in build_sql_phext.py at line 60.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — implode

Found in build_sql_phext.py at line 68.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord-string -> text. Same state machine as

(Where these words came from: direct.)

---

## function — dialect_gap_count

Found in build_sql_phext.py at line 86.

Surface: public API.

Signature: `def dialect_gap_count(d)`

What it does: Function dialect_gap_count takes d. Its return type is unannotated; the code shows it returning len(d["gaps"]) + sum(1 for a in d["actions"] if .

(Where these words came from: inferred.)

---

## function — render_dialect

Found in build_sql_phext.py at line 90.

Surface: public API.

Signature: `def render_dialect(d)`

What it does: Function render_dialect takes d. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — render_page

Found in build_sql_phext.py at line 111.

Surface: public API.

Signature: `def render_page(p)`

What it does: Function render_page takes p. Its return type is unannotated; the code shows it returning f"# {p['title']}\n\nCoordinate: {p['coordinate'].

(Where these words came from: inferred.)

---

## function — render_home

Found in build_sql_phext.py at line 115.

Surface: public API.

Signature: `def render_home(data)`

What it does: Function render_home takes data. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — build

Found in build_sql_phext.py at line 159.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls), data.

(Where these words came from: inferred.)

---

## function — main

Found in build_sql_phext.py at line 177.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — parse_coord

Found in build_tailscale_phext.py at line 28.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — implode

Found in build_tailscale_phext.py at line 36.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: Function implode takes scrolls. Its return type is unannotated; the code shows it returning bytes(out).

(Where these words came from: inferred.)

---

## function — build_bytes

Found in build_tailscale_phext.py at line 52.

Surface: public API.

Signature: `def build_bytes()`

What it does: Function build_bytes takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls).

(Where these words came from: inferred.)

---

## function — main

Found in build_tailscale_phext.py at line 69.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — parse_coord

Found in build_trainer_phext.py at line 42.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning vals.

(Where these words came from: inferred.)

---

## function — implode

Found in build_trainer_phext.py at line 50.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: Function implode takes scrolls. Its return type is unannotated; the code shows it returning bytes(out).

(Where these words came from: inferred.)

---

## function — read_source

Found in build_trainer_phext.py at line 66.

Surface: public API.

Signature: `def read_source(e)`

What it does: Function read_source takes e. Its return type is unannotated; the code shows it returning raw, hashlib.sha256(raw).hexdigest().

(Where these words came from: inferred.)

---

## function — render_entry

Found in build_trainer_phext.py at line 73.

Surface: public API.

Signature: `def render_entry(e, raw)`

What it does: Function render_entry takes e, raw. Its return type is unannotated; the code shows it returning scroll.

(Where these words came from: inferred.)

---

## function — render_home

Found in build_trainer_phext.py at line 95.

Surface: public API.

Signature: `def render_home(entries)`

What it does: Function render_home takes entries. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — load_manifest

Found in build_trainer_phext.py at line 105.

Surface: public API.

Signature: `def load_manifest()`

What it does: Function load_manifest takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning json.load(f)["scrolls"].

(Where these words came from: inferred.)

---

## function — build

Found in build_trainer_phext.py at line 110.

Surface: public API.

Signature: `def build()`

What it does: Function build takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning implode(scrolls), entries, stale.

(Where these words came from: inferred.)

---

## function — main

Found in build_trainer_phext.py at line 128.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — RegisterApp.__init__

Found in register-service/regsvc/app.py at line 53.

Surface: public API.

Signature: `def __init__(self, *, entries_log, resolutions_log, credentials, sessions, ldap_auth, ceremonies, secret, numbers_file=None, session_ttl=7 * 24 * 3600, preauth_ttl=300, drain_token=None, qexo_token=None)`

What it does: Function RegisterApp.__init__ takes self, *, entries_log, resolutions_log, credentials, sessions, ldap_auth, ceremonies, secret, numbers_file=None, session_ttl=7 * 24 * 3600, preauth_ttl=300, drain_token=None, qexo_token=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — RegisterApp.handle

Found in register-service/regsvc/app.py at line 155.

Surface: public API.

Signature: `def handle(self, method, path, headers, body=b"")`

What it does: Function RegisterApp.handle takes self, method, path, headers, body=b"". Its return type is unannotated; the code shows it returning self._route(method, path, headers, body); _err(500, "internal error").

(Where these words came from: inferred.)

---

## function — build_user_dn

Found in register-service/regsvc/ldap_auth.py at line 31.

Surface: public API.

Signature: `def build_user_dn(uid)`

What it does: DN for a uid, or None if the uid is not safe to interpolate.

(Where these words came from: direct.)

---

## function — LdapAuthenticator.__init__

Found in register-service/regsvc/ldap_auth.py at line 53.

Surface: public API.

Signature: `def __init__(self, host, ca_cert_file, port=636, connection_factory=None)`

What it does: Function LdapAuthenticator.__init__ takes self, host, ca_cert_file, port=636, connection_factory=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — LdapAuthenticator.authenticate

Found in register-service/regsvc/ldap_auth.py at line 69.

Surface: public API.

Signature: `def authenticate(self, uid, password)`

What it does: Return the uid on a successful bind, None on bad credentials.

(Where these words came from: direct.)

---

## function — build_app

Found in register-service/regsvc/server.py at line 44.

Surface: public API.

Signature: `def build_app(env=None)`

What it does: Function build_app takes env=None. Its return type is unannotated; the code shows it returning RegisterApp(.

(Where these words came from: inferred.)

---

## function — selftest

Found in register-service/regsvc/server.py at line 79.

Surface: public API.

Signature: `def selftest(env=None)`

What it does: Function selftest takes env=None. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — Handler.do_GET

Found in register-service/regsvc/server.py at line 119.

Surface: public API.

Signature: `def do_GET(self)`

What it does: Function Handler.do_GET takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Handler.do_POST

Found in register-service/regsvc/server.py at line 122.

Surface: public API.

Signature: `def do_POST(self)`

What it does: Function Handler.do_POST takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Handler.log_message

Found in register-service/regsvc/server.py at line 125.

Surface: public API.

Signature: `def log_message(self, fmt, *args): # keep journald noise low`

What it does: Function Handler.log_message takes self, fmt, *args. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in register-service/regsvc/server.py at line 130.

Surface: public API.

Signature: `def main(argv=None)`

What it does: Function main takes argv=None. Its return type is unannotated; the code shows it returning selftest(); 0.

(Where these words came from: inferred.)

---

## function — JsonlLog.__init__

Found in register-service/regsvc/store.py at line 35.

Surface: public API.

Signature: `def __init__(self, path)`

What it does: Function JsonlLog.__init__ takes self, path. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — JsonlLog.append

Found in register-service/regsvc/store.py at line 46.

Surface: public API.

Signature: `def append(self, atom)`

What it does: Append one atom dict; returns the stored atom with seq+ts.

(Where these words came from: direct.)

---

## function — JsonlLog.read_all

Found in register-service/regsvc/store.py at line 59.

Surface: public API.

Signature: `def read_all(self, after_seq=0)`

What it does: All atoms with seq > after_seq, in order.

(Where these words came from: direct.)

---

## function — JsonlLog.last_seq

Found in register-service/regsvc/store.py at line 75.

Surface: public API.

Signature: `def last_seq(self)`

What it does: Function JsonlLog.last_seq takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning self._seq.

(Where these words came from: inferred.)

---

## function — CredentialStore.__init__

Found in register-service/regsvc/store.py at line 86.

Surface: public API.

Signature: `def __init__(self, path)`

What it does: Function CredentialStore.__init__ takes self, path. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — CredentialStore.add

Found in register-service/regsvc/store.py at line 94.

Surface: public API.

Signature: `def add(self, uid, credential)`

What it does: Function CredentialStore.add takes self, uid, credential. Its return type is unannotated; the code shows it returning False; True.

(Where these words came from: inferred.)

---

## function — CredentialStore.list_for

Found in register-service/regsvc/store.py at line 104.

Surface: public API.

Signature: `def list_for(self, uid)`

What it does: Function CredentialStore.list_for takes self, uid. Its return type is unannotated; the code shows it returning list(self._data.get(uid, [])).

(Where these words came from: inferred.)

---

## function — CredentialStore.find

Found in register-service/regsvc/store.py at line 107.

Surface: public API.

Signature: `def find(self, uid, credential_id)`

What it does: Function CredentialStore.find takes self, uid, credential_id. Its return type is unannotated; the code shows it returning c; None.

(Where these words came from: inferred.)

---

## function — CredentialStore.update_sign_count

Found in register-service/regsvc/store.py at line 113.

Surface: public API.

Signature: `def update_sign_count(self, uid, credential_id, new_count)`

What it does: Function CredentialStore.update_sign_count takes self, uid, credential_id, new_count. Its return type is unannotated; the code shows it returning False; True.

(Where these words came from: inferred.)

---

## function — SessionStore.__init__

Found in register-service/regsvc/store.py at line 130.

Surface: public API.

Signature: `def __init__(self, path)`

What it does: Function SessionStore.__init__ takes self, path. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — SessionStore.create

Found in register-service/regsvc/store.py at line 151.

Surface: public API.

Signature: `def create(self, session_id, record)`

What it does: Function SessionStore.create takes self, session_id, record. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — SessionStore.get

Found in register-service/regsvc/store.py at line 156.

Surface: public API.

Signature: `def get(self, session_id)`

What it does: Function SessionStore.get takes self, session_id. Its return type is unannotated; the code shows it returning None; dict(rec).

(Where these words came from: inferred.)

---

## function — SessionStore.update

Found in register-service/regsvc/store.py at line 167.

Surface: public API.

Signature: `def update(self, session_id, **fields)`

What it does: Function SessionStore.update takes self, session_id, **fields. Its return type is unannotated; the code shows it returning False; True.

(Where these words came from: inferred.)

---

## function — SessionStore.delete

Found in register-service/regsvc/store.py at line 176.

Surface: public API.

Signature: `def delete(self, session_id)`

What it does: Function SessionStore.delete takes self, session_id. Its return type is unannotated; the code shows it returning True; False.

(Where these words came from: inferred.)

---

## function — b64url

Found in register-service/regsvc/webauthn_auth.py at line 36.

Surface: public API.

Signature: `def b64url(data: bytes) -> str`

What it does: Function b64url takes data: bytes. It gives back str.

(Where these words came from: inferred.)

---

## function — WebAuthnCeremonies.__init__

Found in register-service/regsvc/webauthn_auth.py at line 43.

Surface: public API.

Signature: `def __init__(self, rp_id, origin)`

What it does: Function WebAuthnCeremonies.__init__ takes self, rp_id, origin. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — WebAuthnCeremonies.registration_options

Found in register-service/regsvc/webauthn_auth.py at line 48.

Surface: public API.

Signature: `def registration_options(self, uid, existing_credentials)`

What it does: Function WebAuthnCeremonies.registration_options takes self, uid, existing_credentials. Its return type is unannotated; the code shows it returning options.challenge, webauthn.options_to_json(opti.

(Where these words came from: inferred.)

---

## function — WebAuthnCeremonies.verify_registration

Found in register-service/regsvc/webauthn_auth.py at line 68.

Surface: public API.

Signature: `def verify_registration(self, credential_json, expected_challenge)`

What it does: Function WebAuthnCeremonies.verify_registration takes self, credential_json, expected_challenge. Its return type is unannotated; the code shows it returning {.

(Where these words came from: inferred.)

---

## function — WebAuthnCeremonies.authentication_options

Found in register-service/regsvc/webauthn_auth.py at line 85.

Surface: public API.

Signature: `def authentication_options(self, credentials)`

What it does: Function WebAuthnCeremonies.authentication_options takes self, credentials. Its return type is unannotated; the code shows it returning options.challenge, webauthn.options_to_json(opti.

(Where these words came from: inferred.)

---

## function — WebAuthnCeremonies.verify_authentication

Found in register-service/regsvc/webauthn_auth.py at line 100.

Surface: public API.

Signature: `def verify_authentication(self, credential_json, expected_challenge, credential)`

What it does: Returns the new sign count; raises WebAuthnError on failure.

(Where these words came from: direct.)

---

## function — FakeLdap.__init__

Found in register-service/tests/test_app.py at line 15.

Surface: test.

Signature: `def __init__(self, valid=None, down=False)`

What it does: Function FakeLdap.__init__ takes self, valid=None, down=False. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FakeLdap.authenticate

Found in register-service/tests/test_app.py at line 19.

Surface: test.

Signature: `def authenticate(self, uid, password)`

What it does: Function FakeLdap.authenticate takes self, uid, password. Its return type is unannotated; the code shows it returning uid; None.

(Where these words came from: inferred.)

---

## function — FakeCeremonies.__init__

Found in register-service/tests/test_app.py at line 28.

Surface: test.

Signature: `def __init__(self)`

What it does: Function FakeCeremonies.__init__ takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FakeCeremonies.registration_options

Found in register-service/tests/test_app.py at line 32.

Surface: test.

Signature: `def registration_options(self, uid, existing)`

What it does: Function FakeCeremonies.registration_options takes self, uid, existing. Its return type is unannotated; the code shows it returning b"reg-challenge", '{"challenge":"reg"}'.

(Where these words came from: inferred.)

---

## function — FakeCeremonies.verify_registration

Found in register-service/tests/test_app.py at line 35.

Surface: test.

Signature: `def verify_registration(self, credential, challenge)`

What it does: Function FakeCeremonies.verify_registration takes self, credential, challenge. Its return type is unannotated; the code shows it returning {"credential_id": "cred-1", "public_key": "pk-1".

(Where these words came from: inferred.)

---

## function — FakeCeremonies.authentication_options

Found in register-service/tests/test_app.py at line 42.

Surface: test.

Signature: `def authentication_options(self, credentials)`

What it does: Function FakeCeremonies.authentication_options takes self, credentials. Its return type is unannotated; the code shows it returning b"auth-challenge", '{"challenge":"auth"}'.

(Where these words came from: inferred.)

---

## function — FakeCeremonies.verify_authentication

Found in register-service/tests/test_app.py at line 45.

Surface: test.

Signature: `def verify_authentication(self, credential, challenge, stored)`

What it does: Function FakeCeremonies.verify_authentication takes self, credential, challenge, stored. Its return type is unannotated; the code shows it returning stored["sign_count"] + 1.

(Where these words came from: inferred.)

---

## function — Client.__init__

Found in register-service/tests/test_app.py at line 53.

Surface: test.

Signature: `def __init__(self, app)`

What it does: Function Client.__init__ takes self, app. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Client.call

Found in register-service/tests/test_app.py at line 57.

Surface: test.

Signature: `def call(self, method, path, body=None, csrf=None, raw_cookies=None)`

What it does: Function Client.call takes self, method, path, body=None, csrf=None, raw_cookies=None. Its return type is unannotated; the code shows it returning status, parsed, resp_headers.

(Where these words came from: inferred.)

---

## function — make_app

Found in register-service/tests/test_app.py at line 82.

Surface: test.

Signature: `def make_app(tmp, numbers=(9, 10, 21), ldap=None, ceremonies=None, numbers_file=True)`

What it does: Function make_app takes tmp, numbers=. Its return type is unannotated; the code shows it returning RegisterApp(.

(Where these words came from: inferred.)

---

## function — AppTestCase.setUp

Found in register-service/tests/test_app.py at line 100.

Surface: test.

Signature: `def setUp(self)`

What it does: Function AppTestCase.setUp takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — AppTestCase.login

Found in register-service/tests/test_app.py at line 105.

Surface: test.

Signature: `def login(self)`

What it does: Function AppTestCase.login takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning body["csrf"].

(Where these words came from: inferred.)

---

## function — TestPasswordLogin.test_session_starts_logged_out

Found in register-service/tests/test_app.py at line 113.

Surface: test.

Signature: `def test_session_starts_logged_out(self)`

What it does: Function TestPasswordLogin.test_session_starts_logged_out takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPasswordLogin.test_bad_password_401_no_cookie

Found in register-service/tests/test_app.py at line 117.

Surface: test.

Signature: `def test_bad_password_401_no_cookie(self)`

What it does: Function TestPasswordLogin.test_bad_password_401_no_cookie takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPasswordLogin.test_ldap_down_503

Found in register-service/tests/test_app.py at line 123.

Surface: test.

Signature: `def test_ldap_down_503(self)`

What it does: Function TestPasswordLogin.test_ldap_down_503 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPasswordLogin.test_login_then_session

Found in register-service/tests/test_app.py at line 129.

Surface: test.

Signature: `def test_login_then_session(self)`

What it does: Function TestPasswordLogin.test_login_then_session takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPasswordLogin.test_tampered_cookie_rejected

Found in register-service/tests/test_app.py at line 135.

Surface: test.

Signature: `def test_tampered_cookie_rejected(self)`

What it does: Function TestPasswordLogin.test_tampered_cookie_rejected takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPasswordLogin.test_logout

Found in register-service/tests/test_app.py at line 144.

Surface: test.

Signature: `def test_logout(self)`

What it does: Function TestPasswordLogin.test_logout takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestEntries.test_entries_require_auth

Found in register-service/tests/test_app.py at line 154.

Surface: test.

Signature: `def test_entries_require_auth(self)`

What it does: Function TestEntries.test_entries_require_auth takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestEntries.test_entries_require_csrf

Found in register-service/tests/test_app.py at line 161.

Surface: test.

Signature: `def test_entries_require_csrf(self)`

What it does: Function TestEntries.test_entries_require_csrf takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestEntries.test_add_and_list_entries

Found in register-service/tests/test_app.py at line 167.

Surface: test.

Signature: `def test_add_and_list_entries(self)`

What it does: Function TestEntries.test_add_and_list_entries takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestEntries.test_empty_and_huge_entries_rejected

Found in register-service/tests/test_app.py at line 185.

Surface: test.

Signature: `def test_empty_and_huge_entries_rejected(self)`

What it does: Function TestEntries.test_empty_and_huge_entries_rejected takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDone.test_done_records_resolution_atom

Found in register-service/tests/test_app.py at line 196.

Surface: test.

Signature: `def test_done_records_resolution_atom(self)`

What it does: Function TestDone.test_done_records_resolution_atom takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDone.test_done_twice_records_both_atoms

Found in register-service/tests/test_app.py at line 208.

Surface: test.

Signature: `def test_done_twice_records_both_atoms(self)`

What it does: Function TestDone.test_done_twice_records_both_atoms takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDone.test_done_unknown_number_404

Found in register-service/tests/test_app.py at line 218.

Surface: test.

Signature: `def test_done_unknown_number_404(self)`

What it does: Function TestDone.test_done_unknown_number_404 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDone.test_done_requires_auth_and_csrf

Found in register-service/tests/test_app.py at line 224.

Surface: test.

Signature: `def test_done_requires_auth_and_csrf(self)`

What it does: Function TestDone.test_done_requires_auth_and_csrf takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDone.test_done_numbers_file_missing_503

Found in register-service/tests/test_app.py at line 231.

Surface: test.

Signature: `def test_done_numbers_file_missing_503(self)`

What it does: Function TestDone.test_done_numbers_file_missing_503 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDone.test_done_with_note_stores_note

Found in register-service/tests/test_app.py at line 241.

Surface: test.

Signature: `def test_done_with_note_stores_note(self)`

What it does: Function TestDone.test_done_with_note_stores_note takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDone.test_done_no_body_no_note_field

Found in register-service/tests/test_app.py at line 254.

Surface: test.

Signature: `def test_done_no_body_no_note_field(self)`

What it does: Function TestDone.test_done_no_body_no_note_field takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDone.test_done_blank_note_omitted

Found in register-service/tests/test_app.py at line 263.

Surface: test.

Signature: `def test_done_blank_note_omitted(self)`

What it does: Function TestDone.test_done_blank_note_omitted takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDone.test_done_note_at_cap_accepted

Found in register-service/tests/test_app.py at line 271.

Surface: test.

Signature: `def test_done_note_at_cap_accepted(self)`

What it does: Function TestDone.test_done_note_at_cap_accepted takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDone.test_done_note_over_cap_400

Found in register-service/tests/test_app.py at line 279.

Surface: test.

Signature: `def test_done_note_over_cap_400(self)`

What it does: Function TestDone.test_done_note_over_cap_400 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDone.test_done_malformed_json_400

Found in register-service/tests/test_app.py at line 287.

Surface: test.

Signature: `def test_done_malformed_json_400(self)`

What it does: Function TestDone.test_done_malformed_json_400 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDone.test_done_non_object_body_400

Found in register-service/tests/test_app.py at line 300.

Surface: test.

Signature: `def test_done_non_object_body_400(self)`

What it does: Function TestDone.test_done_non_object_body_400 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDone.test_done_non_string_note_400

Found in register-service/tests/test_app.py at line 306.

Surface: test.

Signature: `def test_done_non_string_note_400(self)`

What it does: Function TestDone.test_done_non_string_note_400 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestCancel.test_cancel_records_cancelled_atom_with_note

Found in register-service/tests/test_app.py at line 319.

Surface: test.

Signature: `def test_cancel_records_cancelled_atom_with_note(self)`

What it does: Function TestCancel.test_cancel_records_cancelled_atom_with_note takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestCancel.test_cancel_no_body_outcome_no_note

Found in register-service/tests/test_app.py at line 335.

Surface: test.

Signature: `def test_cancel_no_body_outcome_no_note(self)`

What it does: Function TestCancel.test_cancel_no_body_outcome_no_note takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestCancel.test_cancel_unknown_number_404

Found in register-service/tests/test_app.py at line 345.

Surface: test.

Signature: `def test_cancel_unknown_number_404(self)`

What it does: Function TestCancel.test_cancel_unknown_number_404 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestCancel.test_cancel_note_over_cap_400

Found in register-service/tests/test_app.py at line 351.

Surface: test.

Signature: `def test_cancel_note_over_cap_400(self)`

What it does: Function TestCancel.test_cancel_note_over_cap_400 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPasskeys.enroll

Found in register-service/tests/test_app.py at line 363.

Surface: test.

Signature: `def enroll(self)`

What it does: Function TestPasskeys.enroll takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning csrf.

(Where these words came from: inferred.)

---

## function — TestPasskeys.test_register_options_requires_session

Found in register-service/tests/test_app.py at line 374.

Surface: test.

Signature: `def test_register_options_requires_session(self)`

What it does: Function TestPasskeys.test_register_options_requires_session takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPasskeys.test_verify_without_options_400

Found in register-service/tests/test_app.py at line 379.

Surface: test.

Signature: `def test_verify_without_options_400(self)`

What it does: Function TestPasskeys.test_verify_without_options_400 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPasskeys.test_enroll_stores_credential

Found in register-service/tests/test_app.py at line 386.

Surface: test.

Signature: `def test_enroll_stores_credential(self)`

What it does: Function TestPasskeys.test_enroll_stores_credential takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPasskeys.test_enroll_failure_stores_nothing

Found in register-service/tests/test_app.py at line 392.

Surface: test.

Signature: `def test_enroll_failure_stores_nothing(self)`

What it does: Function TestPasskeys.test_enroll_failure_stores_nothing takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPasskeys.test_login_options_unknown_user_404

Found in register-service/tests/test_app.py at line 403.

Surface: test.

Signature: `def test_login_options_unknown_user_404(self)`

What it does: Function TestPasskeys.test_login_options_unknown_user_404 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPasskeys.test_passkey_login_happy_path

Found in register-service/tests/test_app.py at line 408.

Surface: test.

Signature: `def test_passkey_login_happy_path(self)`

What it does: Function TestPasskeys.test_passkey_login_happy_path takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPasskeys.test_passkey_login_bad_assertion_401

Found in register-service/tests/test_app.py at line 425.

Surface: test.

Signature: `def test_passkey_login_bad_assertion_401(self)`

What it does: Function TestPasskeys.test_passkey_login_bad_assertion_401 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPasskeys.test_passkey_verify_without_options_401

Found in register-service/tests/test_app.py at line 437.

Surface: test.

Signature: `def test_passkey_verify_without_options_401(self)`

What it does: Function TestPasskeys.test_passkey_verify_without_options_401 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPasskeys.test_passkey_unknown_credential_401

Found in register-service/tests/test_app.py at line 443.

Surface: test.

Signature: `def test_passkey_unknown_credential_401(self)`

What it does: Function TestPasskeys.test_passkey_unknown_credential_401 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDrainToken.setUp

Found in register-service/tests/test_app.py at line 454.

Surface: test.

Signature: `def setUp(self)`

What it does: Function TestDrainToken.setUp takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDrainToken.call

Found in register-service/tests/test_app.py at line 470.

Surface: test.

Signature: `def call(self, method, path, headers=None, body=None)`

What it does: Function TestDrainToken.call takes self, method, path, headers=None, body=None. Its return type is unannotated; the code shows it returning status, json.loads(resp.decode()).

(Where these words came from: inferred.)

---

## function — TestDrainToken.test_drain_token_reads_both_logs

Found in register-service/tests/test_app.py at line 475.

Surface: test.

Signature: `def test_drain_token_reads_both_logs(self)`

What it does: Function TestDrainToken.test_drain_token_reads_both_logs takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDrainToken.test_wrong_drain_token_401

Found in register-service/tests/test_app.py at line 483.

Surface: test.

Signature: `def test_wrong_drain_token_401(self)`

What it does: Function TestDrainToken.test_wrong_drain_token_401 takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestDrainToken.test_drain_token_cannot_write

Found in register-service/tests/test_app.py at line 488.

Surface: test.

Signature: `def test_drain_token_cannot_write(self)`

What it does: Function TestDrainToken.test_drain_token_cannot_write takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FakeConn.__init__

Found in register-service/tests/test_ldap_auth.py at line 13.

Surface: test.

Signature: `def __init__(self, who=None)`

What it does: Function FakeConn.__init__ takes self, who=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Std.who_am_i

Found in register-service/tests/test_ldap_auth.py at line 20.

Surface: test.

Signature: `def who_am_i(self)`

What it does: Function Std.who_am_i takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning outer._who.

(Where these words came from: inferred.)

---

## function — FakeConn.unbind

Found in register-service/tests/test_ldap_auth.py at line 28.

Surface: test.

Signature: `def unbind(self)`

What it does: Function FakeConn.unbind takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — factory_returning

Found in register-service/tests/test_ldap_auth.py at line 32.

Surface: test.

Signature: `def factory_returning(conn)`

What it does: Function factory_returning takes conn. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — make

Found in register-service/tests/test_ldap_auth.py at line 33.

Surface: test.

Signature: `def make(server, user_dn, password)`

What it does: Function make takes server, user_dn, password. Its return type is unannotated; the code shows it returning conn.

(Where these words came from: inferred.)

---

## function — factory_raising

Found in register-service/tests/test_ldap_auth.py at line 39.

Surface: test.

Signature: `def factory_raising(exc)`

What it does: Function factory_raising takes exc. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — make

Found in register-service/tests/test_ldap_auth.py at line 40.

Surface: test.

Signature: `def make(server, user_dn, password)`

What it does: Function make takes server, user_dn, password. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestBuildUserDn.test_normal_uid

Found in register-service/tests/test_ldap_auth.py at line 46.

Surface: test.

Signature: `def test_normal_uid(self)`

What it does: Function TestBuildUserDn.test_normal_uid takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestBuildUserDn.test_injection_attempts_rejected

Found in register-service/tests/test_ldap_auth.py at line 49.

Surface: test.

Signature: `def test_injection_attempts_rejected(self)`

What it does: Function TestBuildUserDn.test_injection_attempts_rejected takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestAuthenticate.auth

Found in register-service/tests/test_ldap_auth.py at line 56.

Surface: test.

Signature: `def auth(self, factory)`

What it does: Function TestAuthenticate.auth takes self, factory. Its return type is unannotated; the code shows it returning LdapAuthenticator("aletheia-core.pizzly-halfmoon.

(Where these words came from: inferred.)

---

## function — TestAuthenticate.test_success

Found in register-service/tests/test_ldap_auth.py at line 60.

Surface: test.

Signature: `def test_success(self)`

What it does: Function TestAuthenticate.test_success takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestAuthenticate.test_invalid_credentials_fail_closed

Found in register-service/tests/test_ldap_auth.py at line 67.

Surface: test.

Signature: `def test_invalid_credentials_fail_closed(self)`

What it does: Function TestAuthenticate.test_invalid_credentials_fail_closed takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestAuthenticate.test_invalid_credentials_result_returns_none

Found in register-service/tests/test_ldap_auth.py at line 73.

Surface: test.

Signature: `def test_invalid_credentials_result_returns_none(self)`

What it does: Function TestAuthenticate.test_invalid_credentials_result_returns_none takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestAuthenticate.test_unreachable_raises_unavailable

Found in register-service/tests/test_ldap_auth.py at line 85.

Surface: test.

Signature: `def test_unreachable_raises_unavailable(self)`

What it does: Function TestAuthenticate.test_unreachable_raises_unavailable takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestAuthenticate.test_generic_transport_error_raises_unavailable

Found in register-service/tests/test_ldap_auth.py at line 90.

Surface: test.

Signature: `def test_generic_transport_error_raises_unavailable(self)`

What it does: Function TestAuthenticate.test_generic_transport_error_raises_unavailable takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestAuthenticate.test_empty_password_never_touches_network

Found in register-service/tests/test_ldap_auth.py at line 95.

Surface: test.

Signature: `def test_empty_password_never_touches_network(self)`

What it does: Function TestAuthenticate.test_empty_password_never_touches_network takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestAuthenticate.make

Found in register-service/tests/test_ldap_auth.py at line 98.

Surface: test.

Signature: `def make(server, user_dn, password)`

What it does: Function TestAuthenticate.make takes server, user_dn, password. Its return type is unannotated; the code shows it returning FakeConn(who=f"dn:{DN}").

(Where these words came from: inferred.)

---

## function — TestAuthenticate.test_bad_uid_never_touches_network

Found in register-service/tests/test_ldap_auth.py at line 106.

Surface: test.

Signature: `def test_bad_uid_never_touches_network(self)`

What it does: Function TestAuthenticate.test_bad_uid_never_touches_network takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestAuthenticate.make

Found in register-service/tests/test_ldap_auth.py at line 109.

Surface: test.

Signature: `def make(server, user_dn, password)`

What it does: Function TestAuthenticate.make takes server, user_dn, password. Its return type is unannotated; the code shows it returning FakeConn().

(Where these words came from: inferred.)

---

## function — TestAuthenticate.test_whoami_mismatch_rejected

Found in register-service/tests/test_ldap_auth.py at line 117.

Surface: test.

Signature: `def test_whoami_mismatch_rejected(self)`

What it does: Function TestAuthenticate.test_whoami_mismatch_rejected takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Client.__init__

Found in register-service/tests/test_qexo_token.py at line 27.

Surface: test.

Signature: `def __init__(self, app)`

What it does: Function Client.__init__ takes self, app. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Client.call

Found in register-service/tests/test_qexo_token.py at line 31.

Surface: test.

Signature: `def call(self, method, path, body=None, headers=None, csrf=None)`

What it does: Function Client.call takes self, method, path, body=None, headers=None, csrf=None. Its return type is unannotated; the code shows it returning status, parsed, resp_headers.

(Where these words came from: inferred.)

---

## function — make_app

Found in register-service/tests/test_qexo_token.py at line 55.

Surface: test.

Signature: `def make_app(tmp, numbers=(9, 10, 21), qexo_token=QEXO, drain_token=DRAIN, numbers_file=True)`

What it does: Function make_app takes tmp, numbers=. Its return type is unannotated; the code shows it returning RegisterApp(.

(Where these words came from: inferred.)

---

## function — read_log

Found in register-service/tests/test_qexo_token.py at line 74.

Surface: test.

Signature: `def read_log(tmp, name)`

What it does: Function read_log takes tmp, name. Its return type is unannotated; the code shows it returning []; [json.loads(line) for line in.

(Where these words came from: inferred.)

---

## function — QexoTokenTestCase.setUp

Found in register-service/tests/test_qexo_token.py at line 83.

Surface: test.

Signature: `def setUp(self)`

What it does: Function QexoTokenTestCase.setUp takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — QexoTokenTestCase.qexo

Found in register-service/tests/test_qexo_token.py at line 88.

Surface: test.

Signature: `def qexo(self)`

What it does: Function QexoTokenTestCase.qexo takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning {"X-Qexo-Token": QEXO}.

(Where these words came from: inferred.)

---

## function — QexoTokenTestCase.drain

Found in register-service/tests/test_qexo_token.py at line 91.

Surface: test.

Signature: `def drain(self)`

What it does: Function QexoTokenTestCase.drain takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning {"X-Drain-Token": DRAIN}.

(Where these words came from: inferred.)

---

## function — TestPositiveLegs.test_token_appends_an_entry_as_qexo

Found in register-service/tests/test_qexo_token.py at line 96.

Surface: test.

Signature: `def test_token_appends_an_entry_as_qexo(self)`

What it does: Function TestPositiveLegs.test_token_appends_an_entry_as_qexo takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPositiveLegs.test_token_records_done_and_cancel_with_note

Found in register-service/tests/test_qexo_token.py at line 115.

Surface: test.

Signature: `def test_token_records_done_and_cancel_with_note(self)`

What it does: Function TestPositiveLegs.test_token_records_done_and_cancel_with_note takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestPositiveLegs.test_body_cannot_spoof_another_user

Found in register-service/tests/test_qexo_token.py at line 130.

Surface: test.

Signature: `def test_body_cannot_spoof_another_user(self)`

What it does: Function TestPositiveLegs.test_body_cannot_spoof_another_user takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestNegativeLegs.test_wrong_and_absent_tokens_refuse

Found in register-service/tests/test_qexo_token.py at line 140.

Surface: test.

Signature: `def test_wrong_and_absent_tokens_refuse(self)`

What it does: Function TestNegativeLegs.test_wrong_and_absent_tokens_refuse takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestNegativeLegs.test_drain_token_cannot_write

Found in register-service/tests/test_qexo_token.py at line 152.

Surface: test.

Signature: `def test_drain_token_cannot_write(self)`

What it does: Function TestNegativeLegs.test_drain_token_cannot_write takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestNegativeLegs.test_qexo_token_cannot_read

Found in register-service/tests/test_qexo_token.py at line 161.

Surface: test.

Signature: `def test_qexo_token_cannot_read(self)`

What it does: Function TestNegativeLegs.test_qexo_token_cannot_read takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestNegativeLegs.test_qexo_token_grants_no_login

Found in register-service/tests/test_qexo_token.py at line 168.

Surface: test.

Signature: `def test_qexo_token_grants_no_login(self)`

What it does: Function TestNegativeLegs.test_qexo_token_grants_no_login takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestNegativeLegs.test_unset_qexo_token_fails_closed

Found in register-service/tests/test_qexo_token.py at line 184.

Surface: test.

Signature: `def test_unset_qexo_token_fails_closed(self)`

What it does: Function TestNegativeLegs.test_unset_qexo_token_fails_closed takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestNegativeLegs.test_number_and_note_contracts_still_apply

Found in register-service/tests/test_qexo_token.py at line 196.

Surface: test.

Signature: `def test_number_and_note_contracts_still_apply(self)`

What it does: Function TestNegativeLegs.test_number_and_note_contracts_still_apply takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestSessionPathUnchanged.test_session_append_and_close_still_record_the_user

Found in register-service/tests/test_qexo_token.py at line 214.

Surface: test.

Signature: `def test_session_append_and_close_still_record_the_user(self)`

What it does: Function TestSessionPathUnchanged.test_session_append_and_close_still_record_the_user takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestJsonlLog.setUp

Found in register-service/tests/test_store.py at line 10.

Surface: test.

Signature: `def setUp(self)`

What it does: Function TestJsonlLog.setUp takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestJsonlLog.test_append_assigns_monotonic_seq_and_ts

Found in register-service/tests/test_store.py at line 14.

Surface: test.

Signature: `def test_append_assigns_monotonic_seq_and_ts(self)`

What it does: Function TestJsonlLog.test_append_assigns_monotonic_seq_and_ts takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestJsonlLog.test_read_all_and_after_seq

Found in register-service/tests/test_store.py at line 22.

Surface: test.

Signature: `def test_read_all_and_after_seq(self)`

What it does: Function TestJsonlLog.test_read_all_and_after_seq takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestJsonlLog.test_seq_survives_reopen

Found in register-service/tests/test_store.py at line 32.

Surface: test.

Signature: `def test_seq_survives_reopen(self)`

What it does: Function TestJsonlLog.test_seq_survives_reopen takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestJsonlLog.test_duplicate_atoms_are_both_recorded

Found in register-service/tests/test_store.py at line 38.

Surface: test.

Signature: `def test_duplicate_atoms_are_both_recorded(self)`

What it does: Function TestJsonlLog.test_duplicate_atoms_are_both_recorded takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestCredentialStore.setUp

Found in register-service/tests/test_store.py at line 51.

Surface: test.

Signature: `def setUp(self)`

What it does: Function TestCredentialStore.setUp takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestCredentialStore.test_add_find_list

Found in register-service/tests/test_store.py at line 58.

Surface: test.

Signature: `def test_add_find_list(self)`

What it does: Function TestCredentialStore.test_add_find_list takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestCredentialStore.test_add_duplicate_id_rejected

Found in register-service/tests/test_store.py at line 66.

Surface: test.

Signature: `def test_add_duplicate_id_rejected(self)`

What it does: Function TestCredentialStore.test_add_duplicate_id_rejected takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestCredentialStore.test_sign_count_update_persists

Found in register-service/tests/test_store.py at line 72.

Surface: test.

Signature: `def test_sign_count_update_persists(self)`

What it does: Function TestCredentialStore.test_sign_count_update_persists takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestSessionStore.setUp

Found in register-service/tests/test_store.py at line 82.

Surface: test.

Signature: `def setUp(self)`

What it does: Function TestSessionStore.setUp takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestSessionStore.test_create_get_delete

Found in register-service/tests/test_store.py at line 86.

Surface: test.

Signature: `def test_create_get_delete(self)`

What it does: Function TestSessionStore.test_create_get_delete takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestSessionStore.test_expired_session_is_gone

Found in register-service/tests/test_store.py at line 96.

Surface: test.

Signature: `def test_expired_session_is_gone(self)`

What it does: Function TestSessionStore.test_expired_session_is_gone takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestSessionStore.test_update

Found in register-service/tests/test_store.py at line 104.

Surface: test.

Signature: `def test_update(self)`

What it does: Function TestSessionStore.test_update takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestOptions.test_registration_options_shape

Found in register-service/tests/test_webauthn_auth.py at line 9.

Surface: test.

Signature: `def test_registration_options_shape(self)`

What it does: Function TestOptions.test_registration_options_shape takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestOptions.test_authentication_options_shape

Found in register-service/tests/test_webauthn_auth.py at line 15.

Surface: test.

Signature: `def test_authentication_options_shape(self)`

What it does: Function TestOptions.test_authentication_options_shape takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestVerifyFailsClosed.test_registration_garbage_credential_raises

Found in register-service/tests/test_webauthn_auth.py at line 24.

Surface: test.

Signature: `def test_registration_garbage_credential_raises(self)`

What it does: Function TestVerifyFailsClosed.test_registration_garbage_credential_raises takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — TestVerifyFailsClosed.test_authentication_garbage_credential_raises

Found in register-service/tests/test_webauthn_auth.py at line 30.

Surface: test.

Signature: `def test_authentication_garbage_credential_raises(self)`

What it does: Function TestVerifyFailsClosed.test_authentication_garbage_credential_raises takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — clean_md

Found in register-ui/generate_register_page.py at line 42.

Surface: public API.

Signature: `def clean_md(text)`

What it does: Strip light markdown: links -> text, backticks/bold markers out.

(Where these words came from: direct.)

---

## function — parse_register

Found in register-ui/generate_register_page.py at line 49.

Surface: public API.

Signature: `def parse_register(text)`

What it does: Parse numbered items out of the register markdown.

(Where these words came from: direct.)

---

## function — fallback_action

Found in register-ui/generate_register_page.py at line 88.

Surface: public API.

Signature: `def fallback_action(item)`

What it does: Action text from the register prose when the mirror has none.

(Where these words came from: direct.)

---

## function — linkify

Found in register-ui/generate_register_page.py at line 98.

Surface: public API.

Signature: `def linkify(escaped_text)`

What it does: Wrap bare URLs in anchors (input is already HTML-escaped).

(Where these words came from: direct.)

---

## function — repl

Found in register-ui/generate_register_page.py at line 100.

Surface: public API.

Signature: `def repl(m)`

What it does: Function repl takes m. Its return type is unannotated; the code shows it returning f'<a href="{url}" rel="noopener">{url}</a>{trail.

(Where these words came from: inferred.)

---

## function — main

Found in register-ui/generate_register_page.py at line 674.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/register/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

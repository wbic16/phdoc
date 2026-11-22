# site-wishnode-net — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 150). Scanned at commit 04fc512ec10d1a743f397ba68c3f767c03358332. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 18 commits.

The author of record is **Will Bickford** — 18 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Theia — 11 commits
- Chrys — 3 commits
- Emise — 2 commits
- Cyon — 1 commit
- Verse — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- JavaScript: 5 files, 796 lines total, 80 blank, 142 comment, 574 code
- Markdown: 4 files, 569 lines total, 89 blank, 0 comment, 480 code
- Python: 5 files, 1840 lines total, 150 blank, 88 comment, 1602 code
- Shell: 3 files, 198 lines total, 19 blank, 46 comment, 133 code

All languages: 17 files, 3403 lines total, 2789 lines of code. Functions read: 82.

Largest files:

- backend/wishnode_backend.py — 992 lines
- scripts/render_site.py — 462 lines
- ARCHITECTURE.md — 291 lines
- js/maturity-display.js — 247 lines
- js/domain-nav.js — 227 lines

Largest functions, by span in lines:

- render_statute in scripts/render_site.py — about 80 lines
- one in backend/wishnode_backend.py — about 67 lines
- env in backend/wishnode_backend.py — about 57 lines
- chrome in scripts/render_site.py — about 56 lines
- do_POST in backend/wishnode_backend.py — about 44 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 0 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 10 of 82 (12%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/site-wishnode-net
- Branch: exo
- Head commit: 04fc512ec10d1a743f397ba68c3f767c03358332
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `env` (backend/wishnode_backend.py:44) — Function env takes name, default=None.
- function `now_rfc3339` (backend/wishnode_backend.py:101) — Function now_rfc3339 takes no inputs beyond the value it is called on.
- function `sha256_hex` (backend/wishnode_backend.py:104) — Function sha256_hex takes text.
- function `coord` (backend/wishnode_backend.py:107) — Function coord takes ch, sec, base.
- function `b64url_encode` (backend/wishnode_backend.py:110) — Function b64url_encode takes raw: bytes.
- function `b64url_decode` (backend/wishnode_backend.py:113) — Function b64url_decode takes text: str.
- function `sign_token` (backend/wishnode_backend.py:117) — Function sign_token takes secret: str, claims: dict.
- function `verify_signed_token` (backend/wishnode_backend.py:122) — Function verify_signed_token takes secret: str, token: str.
- function `citizen_id_for` (backend/wishnode_backend.py:136) — Function citizen_id_for takes identity: str.
- function `new_tracking_code` (backend/wishnode_backend.py:142) — Function new_tracking_code takes no inputs beyond the value it is called on.
- function `sq_request` (backend/wishnode_backend.py:153) — Function sq_request takes key, method, endpoint, params=None, body=None.
- function `sq_select` (backend/wishnode_backend.py:166) — Function sq_select takes key, phext_name, coordinate.
- function `sq_update` (backend/wishnode_backend.py:172) — Function sq_update takes key, phext_name, coordinate, text.
- function `sq_query` (backend/wishnode_backend.py:178) — Function sq_query takes key, phext_name.
- function `citizen_lock` (backend/wishnode_backend.py:197) — Function citizen_lock takes citizen_id.
- function `citizen_path` (backend/wishnode_backend.py:201) — Function citizen_path takes citizen_id.
- function `citizen_read` (backend/wishnode_backend.py:204) — Function citizen_read takes citizen_id.
- function `citizen_fetch` (backend/wishnode_backend.py:211) — Function citizen_fetch takes buffer, coordinate.
- function `create_submission` (backend/wishnode_backend.py:221) — Provision the citizen phext if absent, append one submission chapter,
- function `citizen_append_transition` (backend/wishnode_backend.py:261) — Function citizen_append_transition takes citizen_id, chapter, line.
- function `citizen_submissions` (backend/wishnode_backend.py:277) — Return a list of dicts describing each submission chapter.
- function `claim_base_for` (backend/wishnode_backend.py:319) — Function claim_base_for takes code.
- function `find_claim` (backend/wishnode_backend.py:327) — Locate a claim by tracking code, applying the documented +20 probe
- function `read_claim_at` (backend/wishnode_backend.py:343) — Function read_claim_at takes ch, sec, base.
- function `create_claim` (backend/wishnode_backend.py:365) — Function create_claim takes code, citizen_id, pointer, content_hash.
- function `bump_register_root` (backend/wishnode_backend.py:384) — Function bump_register_root takes no inputs beyond the value it is called on.
- function `append_transition` (backend/wishnode_backend.py:396) — Function append_transition takes code, to_state, role, note.
- function `verify_assertion` (backend/wishnode_backend.py:425) — Verify an edit.phext.io assertion exactly as exo-sso does:
- function `seal_session` (backend/wishnode_backend.py:462) — Function seal_session takes sub, handle.
- function `open_session` (backend/wishnode_backend.py:468) — Function open_session takes value.
- function `rate_ok` (backend/wishnode_backend.py:492) — Function rate_ok takes ip, endpoint.
- function `page_shell` (backend/wishnode_backend.py:508) — Function page_shell takes title, body, session=None.
- function `esc` (backend/wishnode_backend.py:544) — Function esc takes text.
- function `plain_state` (backend/wishnode_backend.py:547) — Function plain_state takes state.
- function `clean` (backend/wishnode_backend.py:556) — Function clean takes value, maxlen, multiline=False.
- function `handle_submission` (backend/wishnode_backend.py:561) — Shared engine for both workflows.
- function `one` (backend/wishnode_backend.py:568) — Function one takes name.
- function `csrf_ok` (backend/wishnode_backend.py:635) — Function csrf_ok takes handler, form=None.
- function `current_session` (backend/wishnode_backend.py:658) — Function current_session takes handler.
- function `Handler.setup` (backend/wishnode_backend.py:669) — Function Handler.setup takes no inputs beyond the value it is called on.
- function `Handler.log_message` (backend/wishnode_backend.py:673) — Function Handler.log_message takes self, fmt, *args.
- function `Handler.send_html` (backend/wishnode_backend.py:676) — Function Handler.send_html takes self, status, body, extra_headers=None.
- function `Handler.send_json` (backend/wishnode_backend.py:694) — Function Handler.send_json takes self, status, obj.
- function `Handler.redirect` (backend/wishnode_backend.py:704) — Function Handler.redirect takes self, location, extra_headers=None.
- function `Handler.do_GET` (backend/wishnode_backend.py:713) — Function Handler.do_GET takes no inputs beyond the value it is called on.
- function `Handler.do_POST` (backend/wishnode_backend.py:742) — Function Handler.do_POST takes no inputs beyond the value it is called on.
- function `Handler.csrf_token` (backend/wishnode_backend.py:786) — Function Handler.csrf_token takes no inputs beyond the value it is called on.
- function `Handler.serve_css` (backend/wishnode_backend.py:800) — Function Handler.serve_css takes no inputs beyond the value it is called on.
- function `Handler.auth_login` (backend/wishnode_backend.py:812) — Function Handler.auth_login takes no inputs beyond the value it is called on.
- function `Handler.auth_callback` (backend/wishnode_backend.py:821) — Function Handler.auth_callback takes self, qs.
- function `Handler.status_page` (backend/wishnode_backend.py:861) — Function Handler.status_page takes self, qs.
- function `my_page` (backend/wishnode_backend.py:905) — Function my_page takes no inputs beyond the value it is called on.
- function `admin_claim` (backend/wishnode_backend.py:934) — Function admin_claim takes self, qs.
- function `admin_advance` (backend/wishnode_backend.py:945) — Function admin_advance takes self, raw_body.
- function `main` (backend/wishnode_backend.py:964) — Function main takes no inputs beyond the value it is called on.
- function `coord` (scripts/add_demo_leads.py:99) — Function coord takes c, s, b.
- function `load` (scripts/add_demo_leads.py:103) — Function load takes name.
- function `save` (scripts/add_demo_leads.py:108) — Function save takes name, buffer.
- function `check` (scripts/add_demo_leads.py:113) — Function check takes label, text, failures.
- function `main` (scripts/add_demo_leads.py:121) — Function main takes no inputs beyond the value it is called on.
- function `main` (scripts/add_home_ch8.py:42) — Function main takes no inputs beyond the value it is called on.
- function `syllables` (scripts/build_security_phext.py:126) — Function syllables takes word.
- function `readability` (scripts/build_security_phext.py:135) — Function readability takes text.
- function `split_blocks` (scripts/build_security_phext.py:144) — Function split_blocks takes md.
- function `main` (scripts/build_security_phext.py:147) — Function main takes no inputs beyond the value it is called on.
- function `sq_query` (scripts/render_site.py:35) — Function sq_query takes phext_name.
- function `esc` (scripts/render_site.py:52) — Function esc takes t.
- function `md_inline` (scripts/render_site.py:55) — Strip the markdown markers that read as noise in prose.
- function `block` (scripts/render_site.py:61) — Render one scroll: code/table-ish blocks as <pre>, bullet lists
- function `chapter_sections` (scripts/render_site.py:79) — Return list of (heading, [scroll texts]) for sections in a chapter.
- function `render_chapter` (scripts/render_site.py:94) — Render a standard chapter: section 1 is heading + paragraphs;
- function `scrub_offsite` (scripts/render_site.py:135) — Chapter 9 holds pre-rewrite page text as provenance, and that old
- function `tech_details` (scripts/render_site.py:143) — Function tech_details takes scrolls.
- function `chrome` (scripts/render_site.py:158) — Function chrome takes home, title, body, hero=None, extra_head="".
- function `render_app` (scripts/render_site.py:214) — Function render_app takes home, slug.
- function `render_home` (scripts/render_site.py:235) — Function render_home takes home.
- function `render_verification` (scripts/render_site.py:265) — Function render_verification takes home.
- function `render_security` (scripts/render_site.py:295) — Function render_security takes home.
- function `render_statute` (scripts/render_site.py:327) — Function render_statute takes home, statutes, chapter, slug.
- function `render_how_it_works` (scripts/render_site.py:407) — Function render_how_it_works takes home.
- function `write` (scripts/render_site.py:426) — Function write takes relpath, content.
- function `main` (scripts/render_site.py:433) — Function main takes no inputs beyond the value it is called on.

---

## function — env

Found in backend/wishnode_backend.py at line 44.

Surface: public API.

Signature: `def env(name, default=None)`

What it does: Function env takes name, default=None. Its return type is unannotated; the code shows it returning v if v not in (None, "") else default.

(Where these words came from: inferred.)

---

## function — now_rfc3339

Found in backend/wishnode_backend.py at line 101.

Surface: public API.

Signature: `def now_rfc3339()`

What it does: Function now_rfc3339 takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(.

(Where these words came from: inferred.)

---

## function — sha256_hex

Found in backend/wishnode_backend.py at line 104.

Surface: public API.

Signature: `def sha256_hex(text)`

What it does: Function sha256_hex takes text. Its return type is unannotated; the code shows it returning hashlib.sha256(text.encode("utf-8")).hexdigest().

(Where these words came from: inferred.)

---

## function — coord

Found in backend/wishnode_backend.py at line 107.

Surface: public API.

Signature: `def coord(ch, sec, base)`

What it does: Function coord takes ch, sec, base. Its return type is unannotated; the code shows it returning f"1.1.1/1.1.1/{ch}.{sec}.{base}".

(Where these words came from: inferred.)

---

## function — b64url_encode

Found in backend/wishnode_backend.py at line 110.

Surface: public API.

Signature: `def b64url_encode(raw: bytes) -> str`

What it does: Function b64url_encode takes raw: bytes. It gives back str.

(Where these words came from: inferred.)

---

## function — b64url_decode

Found in backend/wishnode_backend.py at line 113.

Surface: public API.

Signature: `def b64url_decode(text: str) -> bytes`

What it does: Function b64url_decode takes text: str. It gives back bytes.

(Where these words came from: inferred.)

---

## function — sign_token

Found in backend/wishnode_backend.py at line 117.

Surface: public API.

Signature: `def sign_token(secret: str, claims: dict) -> str`

What it does: Function sign_token takes secret: str, claims: dict. It gives back str.

(Where these words came from: inferred.)

---

## function — verify_signed_token

Found in backend/wishnode_backend.py at line 122.

Surface: public API.

Signature: `def verify_signed_token(secret: str, token: str) -> dict`

What it does: Function verify_signed_token takes secret: str, token: str. It gives back dict.

(Where these words came from: inferred.)

---

## function — citizen_id_for

Found in backend/wishnode_backend.py at line 136.

Surface: public API.

Signature: `def citizen_id_for(identity: str) -> str`

What it does: Function citizen_id_for takes identity: str. It gives back str.

(Where these words came from: inferred.)

---

## function — new_tracking_code

Found in backend/wishnode_backend.py at line 142.

Surface: public API.

Signature: `def new_tracking_code() -> str`

What it does: Function new_tracking_code takes no inputs beyond the value it is called on. It gives back str.

(Where these words came from: inferred.)

---

## function — sq_request

Found in backend/wishnode_backend.py at line 153.

Surface: public API.

Signature: `def sq_request(key, method, endpoint, params=None, body=None)`

What it does: Function sq_request takes key, method, endpoint, params=None, body=None. Its return type is unannotated; the code shows it returning resp.status, resp.read().decode("utf-8"); e.code, e.read().decode("utf-8", "replace").

(Where these words came from: inferred.)

---

## function — sq_select

Found in backend/wishnode_backend.py at line 166.

Surface: public API.

Signature: `def sq_select(key, phext_name, coordinate)`

What it does: Function sq_select takes key, phext_name, coordinate. Its return type is unannotated; the code shows it returning text.

(Where these words came from: inferred.)

---

## function — sq_update

Found in backend/wishnode_backend.py at line 172.

Surface: public API.

Signature: `def sq_update(key, phext_name, coordinate, text)`

What it does: Function sq_update takes key, phext_name, coordinate, text. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — sq_query

Found in backend/wishnode_backend.py at line 178.

Surface: public API.

Signature: `def sq_query(key, phext_name)`

What it does: Function sq_query takes key, phext_name. Its return type is unannotated; the code shows it returning json.loads(out).

(Where these words came from: inferred.)

---

## function — citizen_lock

Found in backend/wishnode_backend.py at line 197.

Surface: public API.

Signature: `def citizen_lock(citizen_id)`

What it does: Function citizen_lock takes citizen_id. Its return type is unannotated; the code shows it returning _citizen_locks.setdefault(citizen_id, threading..

(Where these words came from: inferred.)

---

## function — citizen_path

Found in backend/wishnode_backend.py at line 201.

Surface: public API.

Signature: `def citizen_path(citizen_id)`

What it does: Function citizen_path takes citizen_id. Its return type is unannotated; the code shows it returning os.path.join(CITIZEN_DIR, citizen_id + ".phext").

(Where these words came from: inferred.)

---

## function — citizen_read

Found in backend/wishnode_backend.py at line 204.

Surface: public API.

Signature: `def citizen_read(citizen_id)`

What it does: Function citizen_read takes citizen_id. Its return type is unannotated; the code shows it returning ""; f.read().

(Where these words came from: inferred.)

---

## function — citizen_fetch

Found in backend/wishnode_backend.py at line 211.

Surface: public API.

Signature: `def citizen_fetch(buffer, coordinate)`

What it does: Function citizen_fetch takes buffer, coordinate. Its return type is unannotated; the code shows it returning PX.fetch(buffer, Coordinate.from_string(coordina.

(Where these words came from: inferred.)

---

## function — create_submission

Found in backend/wishnode_backend.py at line 221.

Surface: public API.

Signature: `def create_submission(citizen_id, form_kind, code, fields)`

What it does: Provision the citizen phext if absent, append one submission chapter,

(Where these words came from: direct.)

---

## function — citizen_append_transition

Found in backend/wishnode_backend.py at line 261.

Surface: public API.

Signature: `def citizen_append_transition(citizen_id, chapter, line)`

What it does: Function citizen_append_transition takes citizen_id, chapter, line. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — citizen_submissions

Found in backend/wishnode_backend.py at line 277.

Surface: public API.

Signature: `def citizen_submissions(citizen_id)`

What it does: Return a list of dicts describing each submission chapter.

(Where these words came from: direct.)

---

## function — claim_base_for

Found in backend/wishnode_backend.py at line 319.

Surface: public API.

Signature: `def claim_base_for(code)`

What it does: Function claim_base_for takes code. Its return type is unannotated; the code shows it returning ch, sec, base.

(Where these words came from: inferred.)

---

## function — find_claim

Found in backend/wishnode_backend.py at line 327.

Surface: public API.

Signature: `def find_claim(code)`

What it does: Locate a claim by tracking code, applying the documented +20 probe

(Where these words came from: direct.)

---

## function — read_claim_at

Found in backend/wishnode_backend.py at line 343.

Surface: public API.

Signature: `def read_claim_at(ch, sec, base)`

What it does: Function read_claim_at takes ch, sec, base. Its return type is unannotated; the code shows it returning claim.

(Where these words came from: inferred.)

---

## function — create_claim

Found in backend/wishnode_backend.py at line 365.

Surface: public API.

Signature: `def create_claim(code, citizen_id, pointer, content_hash)`

What it does: Function create_claim takes code, citizen_id, pointer, content_hash. Its return type is unannotated; the code shows it returning ch, sec, base.

(Where these words came from: inferred.)

---

## function — bump_register_root

Found in backend/wishnode_backend.py at line 384.

Surface: public API.

Signature: `def bump_register_root()`

What it does: Function bump_register_root takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — append_transition

Found in backend/wishnode_backend.py at line 396.

Surface: public API.

Signature: `def append_transition(code, to_state, role, note)`

What it does: Function append_transition takes code, to_state, role, note. Its return type is unannotated; the code shows it returning None, "no such tracking code"; None, f"illegal transition {current} -> {to_stat; None, f"role {role} may not make the {to_state} ; {"from": current, "to": to_state, "line": line},.

(Where these words came from: inferred.)

---

## function — verify_assertion

Found in backend/wishnode_backend.py at line 425.

Surface: public API.

Signature: `def verify_assertion(token)`

What it does: Verify an edit.phext.io assertion exactly as exo-sso does:

(Where these words came from: direct.)

---

## function — seal_session

Found in backend/wishnode_backend.py at line 462.

Surface: public API.

Signature: `def seal_session(sub, handle)`

What it does: Function seal_session takes sub, handle. Its return type is unannotated; the code shows it returning sign_token(SESSION_SECRET, {.

(Where these words came from: inferred.)

---

## function — open_session

Found in backend/wishnode_backend.py at line 468.

Surface: public API.

Signature: `def open_session(value)`

What it does: Function open_session takes value. Its return type is unannotated; the code shows it returning None; {"sub": sub, "handle": claims.get("handle", ""),.

(Where these words came from: inferred.)

---

## function — rate_ok

Found in backend/wishnode_backend.py at line 492.

Surface: public API.

Signature: `def rate_ok(ip, endpoint)`

What it does: Function rate_ok takes ip, endpoint. Its return type is unannotated; the code shows it returning False; True.

(Where these words came from: inferred.)

---

## function — page_shell

Found in backend/wishnode_backend.py at line 508.

Surface: public API.

Signature: `def page_shell(title, body, session=None)`

What it does: Function page_shell takes title, body, session=None. Its return type is unannotated; the code shows it returning f"""<!DOCTYPE html>.

(Where these words came from: inferred.)

---

## function — esc

Found in backend/wishnode_backend.py at line 544.

Surface: public API.

Signature: `def esc(text)`

What it does: Function esc takes text. Its return type is unannotated; the code shows it returning html.escape(text or "", quote=True).

(Where these words came from: inferred.)

---

## function — plain_state

Found in backend/wishnode_backend.py at line 547.

Surface: public API.

Signature: `def plain_state(state)`

What it does: Function plain_state takes state. Its return type is unannotated; the code shows it returning STATE_PLAIN.get(state, state).

(Where these words came from: inferred.)

---

## function — clean

Found in backend/wishnode_backend.py at line 556.

Surface: public API.

Signature: `def clean(value, maxlen, multiline=False)`

What it does: Function clean takes value, maxlen, multiline=False. Its return type is unannotated; the code shows it returning value[:maxlen].

(Where these words came from: inferred.)

---

## function — handle_submission

Found in backend/wishnode_backend.py at line 561.

Surface: public API.

Signature: `def handle_submission(handler, form_kind, raw)`

What it does: Shared engine for both workflows. raw: dict of form lists.

(Where these words came from: direct.)

---

## function — one

Found in backend/wishnode_backend.py at line 568.

Surface: public API.

Signature: `def one(name)`

What it does: Function one takes name. Its return type is unannotated; the code shows it returning raw.get(name, [""])[0].

(Where these words came from: inferred.)

---

## function — csrf_ok

Found in backend/wishnode_backend.py at line 635.

Surface: public API.

Signature: `def csrf_ok(handler, form=None)`

What it does: Function csrf_ok takes handler, form=None. Its return type is unannotated; the code shows it returning False; bool(authority) and authority == host; bool(field) and bool(cookie_token) and hmac.comp.

(Where these words came from: inferred.)

---

## function — current_session

Found in backend/wishnode_backend.py at line 658.

Surface: public API.

Signature: `def current_session(handler)`

What it does: Function current_session takes handler. Its return type is unannotated; the code shows it returning open_session(part.split("=", 1)[1]); None.

(Where these words came from: inferred.)

---

## function — Handler.setup

Found in backend/wishnode_backend.py at line 669.

Surface: public API.

Signature: `def setup(self)`

What it does: Function Handler.setup takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Handler.log_message

Found in backend/wishnode_backend.py at line 673.

Surface: public API.

Signature: `def log_message(self, fmt, *args)`

What it does: Function Handler.log_message takes self, fmt, *args. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Handler.send_html

Found in backend/wishnode_backend.py at line 676.

Surface: public API.

Signature: `def send_html(self, status, body, extra_headers=None)`

What it does: Function Handler.send_html takes self, status, body, extra_headers=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Handler.send_json

Found in backend/wishnode_backend.py at line 694.

Surface: public API.

Signature: `def send_json(self, status, obj)`

What it does: Function Handler.send_json takes self, status, obj. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Handler.redirect

Found in backend/wishnode_backend.py at line 704.

Surface: public API.

Signature: `def redirect(self, location, extra_headers=None)`

What it does: Function Handler.redirect takes self, location, extra_headers=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Handler.do_GET

Found in backend/wishnode_backend.py at line 713.

Surface: public API.

Signature: `def do_GET(self)`

What it does: Function Handler.do_GET takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning self.send_json(200, {"ok": True, "sso_enabled": ; self.csrf_token(); self.serve_css(); self.auth_login(); self.auth_callback(qs); self.redirect("/", {"Set-Cookie": "wishnode_sess; self.status_page(qs); self.my_page(); self.admin_claim(qs); self.send_html(404, page_shell("Not found", "<se; self.send_html(500, page_shell("Something went w.

(Where these words came from: inferred.)

---

## function — Handler.do_POST

Found in backend/wishnode_backend.py at line 742.

Surface: public API.

Signature: `def do_POST(self)`

What it does: Function Handler.do_POST takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning self.send_html(400, page_shell("Bad request", "<; self.send_html(413, page_shell("Too large", "<se; self.send_html(429, page_shell("Slow down",; self.send_html(403, page_shell("That form did no; self.send_html(status, body); self.send_json(403, {"error": "origin mismatch"}; self.admin_advance(raw_body); self.send_html(404, page_shell("Not found", "<se; self.send_html(500, page_shell("Something went w.

(Where these words came from: inferred.)

---

## function — Handler.csrf_token

Found in backend/wishnode_backend.py at line 786.

Surface: public API.

Signature: `def csrf_token(self)`

What it does: Function Handler.csrf_token takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Handler.serve_css

Found in backend/wishnode_backend.py at line 800.

Surface: public API.

Signature: `def serve_css(self)`

What it does: Function Handler.serve_css takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning self.send_html(404, "not found").

(Where these words came from: inferred.)

---

## function — Handler.auth_login

Found in backend/wishnode_backend.py at line 812.

Surface: public API.

Signature: `def auth_login(self)`

What it does: Function Handler.auth_login takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning self.send_html(200, page_shell("Sign in",; self.redirect(EDIT_SSO_LOGIN_URL).

(Where these words came from: inferred.)

---

## function — Handler.auth_callback

Found in backend/wishnode_backend.py at line 821.

Surface: public API.

Signature: `def auth_callback(self, qs)`

What it does: Function Handler.auth_callback takes self, qs. Its return type is unannotated; the code shows it returning self.send_html(503, page_shell("Sign in",; self.send_html(401, page_shell("Sign-in did not ; self.redirect(ret, {"Set-Cookie":.

(Where these words came from: inferred.)

---

## function — Handler.status_page

Found in backend/wishnode_backend.py at line 861.

Surface: public API.

Signature: `def status_page(self, qs)`

What it does: Function Handler.status_page takes self, qs. Its return type is unannotated; the code shows it returning self.send_html(400, page_shell("Check a submissi; self.send_html(404, page_shell("Code not found",.

(Where these words came from: inferred.)

---

## function — my_page

Found in backend/wishnode_backend.py at line 905.

Surface: public API.

Signature: `def my_page(self)`

What it does: Function my_page takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning self.send_html(200, page_shell("Your forms",.

(Where these words came from: inferred.)

---

## function — admin_claim

Found in backend/wishnode_backend.py at line 934.

Surface: public API.

Signature: `def admin_claim(self, qs)`

What it does: Function admin_claim takes self, qs. Its return type is unannotated; the code shows it returning self.send_json(401, {"error": "unauthorized"}); self.send_json(404, {"error": "no such tracking ; self.send_json(200, {"code": code, "at": list(lo.

(Where these words came from: inferred.)

---

## function — admin_advance

Found in backend/wishnode_backend.py at line 945.

Surface: public API.

Signature: `def admin_advance(self, raw_body)`

What it does: Function admin_advance takes self, raw_body. Its return type is unannotated; the code shows it returning self.send_json(401, {"error": "unauthorized"}); self.send_json(400, {"error": "body must be JSON; self.send_json(400, {"error": "need code and a t; self.send_json(409, {"error": error}); self.send_json(200, {"ok": True, "code": code, *.

(Where these words came from: inferred.)

---

## function — main

Found in backend/wishnode_backend.py at line 964.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — coord

Found in scripts/add_demo_leads.py at line 99.

Surface: public API.

Signature: `def coord(c, s, b)`

What it does: Function coord takes c, s, b. Its return type is unannotated; the code shows it returning Coordinate.from_string(f"1.1.1/1.1.1/{c}.{s}.{b}.

(Where these words came from: inferred.)

---

## function — load

Found in scripts/add_demo_leads.py at line 103.

Surface: public API.

Signature: `def load(name)`

What it does: Function load takes name. Its return type is unannotated; the code shows it returning f.read().

(Where these words came from: inferred.)

---

## function — save

Found in scripts/add_demo_leads.py at line 108.

Surface: public API.

Signature: `def save(name, buffer)`

What it does: Function save takes name, buffer. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — check

Found in scripts/add_demo_leads.py at line 113.

Surface: public API.

Signature: `def check(label, text, failures)`

What it does: Function check takes label, text, failures. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in scripts/add_demo_leads.py at line 121.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — main

Found in scripts/add_home_ch8.py at line 42.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — syllables

Found in scripts/build_security_phext.py at line 126.

Surface: public API.

Signature: `def syllables(word)`

What it does: Function syllables takes word. Its return type is unannotated; the code shows it returning 0; max(1, groups).

(Where these words came from: inferred.)

---

## function — readability

Found in scripts/build_security_phext.py at line 135.

Surface: public API.

Signature: `def readability(text)`

What it does: Function readability takes text. Its return type is unannotated; the code shows it returning fk, fre, n_w.

(Where these words came from: inferred.)

---

## function — split_blocks

Found in scripts/build_security_phext.py at line 144.

Surface: public API.

Signature: `def split_blocks(md)`

What it does: Function split_blocks takes md. Its return type is unannotated; the code shows it returning [b.strip() for b in re.split(r"\n\s*\n", md) if .

(Where these words came from: inferred.)

---

## function — main

Found in scripts/build_security_phext.py at line 147.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — sq_query

Found in scripts/render_site.py at line 35.

Surface: public API.

Signature: `def sq_query(phext_name)`

What it does: Function sq_query takes phext_name. Its return type is unannotated; the code shows it returning scrolls.

(Where these words came from: inferred.)

---

## function — esc

Found in scripts/render_site.py at line 52.

Surface: public API.

Signature: `def esc(t)`

What it does: Function esc takes t. Its return type is unannotated; the code shows it returning html.escape(t or "", quote=True).

(Where these words came from: inferred.)

---

## function — md_inline

Found in scripts/render_site.py at line 55.

Surface: public API.

Signature: `def md_inline(text)`

What it does: Strip the markdown markers that read as noise in prose.

(Where these words came from: direct.)

---

## function — block

Found in scripts/render_site.py at line 61.

Surface: public API.

Signature: `def block(text)`

What it does: Render one scroll: code/table-ish blocks as <pre>, bullet lists

(Where these words came from: direct.)

---

## function — chapter_sections

Found in scripts/render_site.py at line 79.

Surface: public API.

Signature: `def chapter_sections(scrolls, chapter, skip_first_heading=False)`

What it does: Return list of (heading, [scroll texts]) for sections in a chapter.

(Where these words came from: direct.)

---

## function — render_chapter

Found in scripts/render_site.py at line 94.

Surface: public API.

Signature: `def render_chapter(scrolls, chapter, slug=None, plain_heading_h2=True)`

What it does: Render a standard chapter: section 1 is heading + paragraphs;

(Where these words came from: direct.)

---

## function — scrub_offsite

Found in scripts/render_site.py at line 135.

Surface: public API.

Signature: `def scrub_offsite(text)`

What it does: Chapter 9 holds pre-rewrite page text as provenance, and that old

(Where these words came from: direct.)

---

## function — tech_details

Found in scripts/render_site.py at line 143.

Surface: public API.

Signature: `def tech_details(scrolls)`

What it does: Function tech_details takes scrolls. Its return type is unannotated; the code shows it returning ""; f'<details class="tech"><summary>For technical s.

(Where these words came from: inferred.)

---

## function — chrome

Found in scripts/render_site.py at line 158.

Surface: public API.

Signature: `def chrome(home, title, body, hero=None, extra_head="")`

What it does: Function chrome takes home, title, body, hero=None, extra_head="". Its return type is unannotated; the code shows it returning f"""<!DOCTYPE html>.

(Where these words came from: inferred.)

---

## function — render_app

Found in scripts/render_site.py at line 214.

Surface: public API.

Signature: `def render_app(home, slug)`

What it does: Function render_app takes home, slug. Its return type is unannotated; the code shows it returning chrome(home, title, body).

(Where these words came from: inferred.)

---

## function — render_home

Found in scripts/render_site.py at line 235.

Surface: public API.

Signature: `def render_home(home)`

What it does: Function render_home takes home. Its return type is unannotated; the code shows it returning chrome(home, title, body, hero=hero).

(Where these words came from: inferred.)

---

## function — render_verification

Found in scripts/render_site.py at line 265.

Surface: public API.

Signature: `def render_verification(home)`

What it does: Function render_verification takes home. Its return type is unannotated; the code shows it returning chrome(home, title, body).

(Where these words came from: inferred.)

---

## function — render_security

Found in scripts/render_site.py at line 295.

Surface: public API.

Signature: `def render_security(home)`

What it does: Function render_security takes home. Its return type is unannotated; the code shows it returning chrome(home, title, body).

(Where these words came from: inferred.)

---

## function — render_statute

Found in scripts/render_site.py at line 327.

Surface: public API.

Signature: `def render_statute(home, statutes, chapter, slug)`

What it does: Function render_statute takes home, statutes, chapter, slug. Its return type is unannotated; the code shows it returning chrome(home, f"{title} | Phext, Inc.", body).

(Where these words came from: inferred.)

---

## function — render_how_it_works

Found in scripts/render_site.py at line 407.

Surface: public API.

Signature: `def render_how_it_works(home)`

What it does: Function render_how_it_works takes home. Its return type is unannotated; the code shows it returning chrome(home, "How this site works | Phext, Inc.".

(Where these words came from: inferred.)

---

## function — write

Found in scripts/render_site.py at line 426.

Surface: public API.

Signature: `def write(relpath, content)`

What it does: Function write takes relpath, content. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in scripts/render_site.py at line 433.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/site-wishnode-net/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

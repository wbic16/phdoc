# rfcbox — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 95). Project root: /home/wbic16/phdoc-sweep/proj/rfcbox/. Scanned at commit da2c4d7048883a12b3c281c30ee260d5a76d4a23. Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Dependencies

### Python (pip) — examples/ftp-client/requirements.txt

Direct dependencies: 3.

- pyftpdlib — (unpinned)
- pyopenssl — (unpinned)
- cryptography — (unpinned)

### Python (pip) — examples/p25-iki/requirements.txt

Direct dependencies: 1.

- cryptography — >=42


---

## Authors

The git log holds 32 commits.

The author of record is **Will Bickford** — 32 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Emise — 25 commits
- Theia — 5 commits
- Rogue — 1 commit
- Will Bickford (his own identities) — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 21 files, 917 lines total, 287 blank, 3 comment, 627 code
- Python: 7 files, 2206 lines total, 311 blank, 100 comment, 1795 code
- Shell: 1 files, 14 lines total, 2 blank, 5 comment, 7 code

All languages: 29 files, 3137 lines total, 2429 lines of code. Functions read: 129.

Largest files:

- examples/web-browser/browser.py — 537 lines
- examples/p25-iki/iki_server.py — 500 lines
- examples/web-browser/test_browser.py — 367 lines
- examples/ftp-client/ftp_client.py — 345 lines
- examples/ftp-client/test_ftp.py — 211 lines

Largest functions, by span in lines:

- main in examples/ftp-client/ftp_client.py — about 88 lines
- check in examples/p25-iki/demo.py — about 55 lines
- test_slice in examples/web-browser/test_browser.py — about 53 lines
- test_http in examples/web-browser/test_browser.py — about 53 lines
- make_kmf_identity in examples/p25-iki/iki_server.py — about 44 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 13 test functions counted in the source, across 3 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 19 of 109 (17%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: /home/wbic16/github-archive/mirrors/rfcbox.git
- Branch: exo
- Head commit: da2c4d7048883a12b3c281c30ee260d5a76d4a23
- Working tree: 1 changed paths

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.

- function `__init__` (examples/ftp-client/ftp_client.py:53) — Function __init__ takes self, host, port=21, user="anonymous", password="anonymous@", and gives back nothing; it does it
- function `connect` (examples/ftp-client/ftp_client.py:69) — Function connect takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it i
- function `close` (examples/ftp-client/ftp_client.py:122) — Function close takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is 
- function `__enter__` (examples/ftp-client/ftp_client.py:130) — Function __enter__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it
- function `__exit__` (examples/ftp-client/ftp_client.py:133) — Function __exit__ takes self, *exc and gives back nothing; it does its work on what it is given.
- function `pwd` (examples/ftp-client/ftp_client.py:143) — Function pwd takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is gi
- function `cd` (examples/ftp-client/ftp_client.py:146) — Function cd takes self, path and gives back nothing; it does its work on what it is given.
- function `list` (examples/ftp-client/ftp_client.py:149) — Directory listing: MLSD (RFC 3659) when offered, LIST fallback.
- function `collect` (examples/ftp-client/ftp_client.py:160) — Function collect takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it i
- function `get` (examples/ftp-client/ftp_client.py:179) — Function get takes self, remote, local=None and gives back nothing; it does its work on what it is given.
- function `run` (examples/ftp-client/ftp_client.py:181) — Function run takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is gi
- function `get_bytes` (examples/ftp-client/ftp_client.py:196) — Function get_bytes takes self, remote and gives back nothing; it does its work on what it is given.
- function `put` (examples/ftp-client/ftp_client.py:201) — Function put takes self, local, remote=None and gives back nothing; it does its work on what it is given.
- function `run` (examples/ftp-client/ftp_client.py:203) — Function run takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is gi
- function `put_bytes` (examples/ftp-client/ftp_client.py:209) — Function put_bytes takes self, data, remote and gives back nothing; it does its work on what it is given.
- function `size` (examples/ftp-client/ftp_client.py:214) — RFC 3659 SIZE: byte count of a remote file.
- function `mdtm` (examples/ftp-client/ftp_client.py:222) — RFC 3659 MDTM: modification time of a remote file (UTC).
- function `delete` (examples/ftp-client/ftp_client.py:231) — Function delete takes self, path and gives back nothing; it does its work on what it is given.
- function `mkdir` (examples/ftp-client/ftp_client.py:234) — Function mkdir takes self, path and gives back nothing; it does its work on what it is given.
- function `rmdir` (examples/ftp-client/ftp_client.py:237) — Function rmdir takes self, path and gives back nothing; it does its work on what it is given.
- function `parse_target` (examples/ftp-client/ftp_client.py:241) — Target is a host or an ftp:// URL (RFC 3986 syntax; the ftp scheme
- function `main` (examples/ftp-client/ftp_client.py:258) — Function main takes argv=None and gives back nothing; it does its work on what it is given.
- function `check` (examples/ftp-client/test_ftp.py:29) — Function check takes label, cond, detail="" and gives back nothing; it does its work on what it is given.
- function `test_slice` (examples/ftp-client/test_ftp.py:38) — Function test_slice takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `make_cert` (examples/ftp-client/test_ftp.py:68) — Function make_cert takes tmp and gives back nothing; it does its work on what it is given.
- function `start_server` (examples/ftp-client/test_ftp.py:94) — Function start_server takes root, tls=None and gives back nothing; it does its work on what it is given.
- function `exercise` (examples/ftp-client/test_ftp.py:120) — Function exercise takes port, tls, label and gives back nothing; it does its work on what it is given.
- function `test_errors` (examples/ftp-client/test_ftp.py:151) — Function test_errors takes port and gives back nothing; it does its work on what it is given.
- function `test_cli` (examples/ftp-client/test_ftp.py:164) — Function test_cli takes port, tmp and gives back nothing; it does its work on what it is given.
- function `main` (examples/ftp-client/test_ftp.py:183) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `run_demo` (examples/p25-iki/demo.py:27) — Function run_demo takes base: Path = RUN_DIR, verbose: bool = True and gives back dict:.
- function `say` (examples/p25-iki/demo.py:28) — Function say takes msg: str and gives back nothing; it does its work on what it is given.
- function `check` (examples/p25-iki/demo.py:41) — Function check takes label: str, ok: bool and gives back nothing; it does its work on what it is given.
- function `main` (examples/p25-iki/demo.py:96) — Function main takes no inputs beyond the value it is called on, and gives back int:.
- function `utcnow` (examples/p25-iki/iki_server.py:62) — Function utcnow takes no inputs beyond the value it is called on, and gives back str:.
- function `canonical` (examples/p25-iki/iki_server.py:66) — Deterministic JSON encoding used for everything that gets signed.
- function `b64e` (examples/p25-iki/iki_server.py:71) — Function b64e takes b: bytes and gives back str:.
- function `b64d` (examples/p25-iki/iki_server.py:75) — Function b64d takes s: str and gives back bytes:.
- function `make_ca` (examples/p25-iki/iki_server.py:87) — Create a self-signed test CA.
- function `make_kmf_identity` (examples/p25-iki/iki_server.py:110) — Issue a KMF identity certificate (server + client + signing use).
- function `cms_sign_detached` (examples/p25-iki/iki_server.py:154) — Detached CMS SignedData (DER) over `data`, signed by the KMF identity.
- function `cms_verify_detached` (examples/p25-iki/iki_server.py:165) — Verify a detached CMS signature against the test CA using OpenSSL.
- function `__init__` (examples/p25-iki/iki_server.py:184) — Function __init__ takes self, path: Path, actor: str and gives back nothing; it does its work on what it is given.
- function `append` (examples/p25-iki/iki_server.py:195) — Function append takes self, event: str, details: dict and gives back dict:.
- function `verify_chain` (examples/p25-iki/iki_server.py:207) — Function verify_chain takes path: Path and gives back bool:.
- function `__init__` (examples/p25-iki/iki_server.py:226) — Function __init__ takes self, db_path: Path, master_key: bytes and gives back nothing; it does its work on what it is gi
- function `put_key` (examples/p25-iki/iki_server.py:252) — Function put_key takes self, owner: str, key_id: int, alg_id: int, and gives back nothing; it does its work on what it i
- function `get_key` (examples/p25-iki/iki_server.py:261) — Function get_key takes self, owner: str, key_id: int, alg_id: int and gives back bytes | None:.
- function `list_keys` (examples/p25-iki/iki_server.py:269) — Function list_keys takes self, owner: str and gives back list[dict]:.
- function `provision_peer` (examples/p25-iki/iki_server.py:276) — Function provision_peer takes self, peer: str, root_secret: bytes and gives back None:.
- function `directional_kek` (examples/p25-iki/iki_server.py:283) — HKDF (RFC 5869): derive the src->dst wrap key from the pairwise root.
- function `put_receipt` (examples/p25-iki/iki_server.py:294) — Function put_receipt takes self, exchange_id: str, receipt: dict, signature: bytes and gives back None:.
- function `get_receipt` (examples/p25-iki/iki_server.py:301) — Function get_receipt takes self, exchange_id: str and gives back dict | None:.
- function `__init__` (examples/p25-iki/iki_server.py:315) — Function __init__ takes self, name: str, workdir: Path, ca: dict and gives back nothing; it does its work on what it is 
- function `create_key` (examples/p25-iki/iki_server.py:331) — Function create_key takes self, key_id: int, alg_id: int = ALG_AES_256 and gives back bytes:.
- function `provision_peer` (examples/p25-iki/iki_server.py:338) — Function provision_peer takes self, peer_name: str, root_secret: bytes and gives back None:.
- function `export_keyset` (examples/p25-iki/iki_server.py:343) — Function export_keyset takes self, peer: "KMF", key_ids: list[int], and gives back nothing; it does its work on what it 
- function `handle_exchange` (examples/p25-iki/iki_server.py:384) — Function handle_exchange takes self, manifest_bytes: bytes, signature: bytes, and gives back nothing; it does its work o
- function `start` (examples/p25-iki/iki_server.py:443) — Function start takes no inputs beyond the value it is called on, and gives back None:.
- function `log_message` (examples/p25-iki/iki_server.py:447) — Function log_message takes self, *args and gives back nothing; it does its work on what it is given.
- function `do_GET` (examples/p25-iki/iki_server.py:450) — Function do_GET takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is
- function `do_POST` (examples/p25-iki/iki_server.py:456) — Function do_POST takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it i
- function `stop` (examples/p25-iki/iki_server.py:488) — Function stop takes no inputs beyond the value it is called on, and gives back None:.
- function `check` (examples/p25-iki/test_iki.py:32) — Function check takes label: str, ok: bool, extra: str = "" and gives back nothing; it does its work on what it is given.
- function `test_slice_against_corpus` (examples/p25-iki/test_iki.py:49) — Function test_slice_against_corpus takes no inputs beyond the value it is called on, and gives back nothing; it does its
- function `test_keywrap` (examples/p25-iki/test_iki.py:66) — Function test_keywrap takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `test_hkdf_directions` (examples/p25-iki/test_iki.py:79) — Function test_hkdf_directions takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `test_store_at_rest` (examples/p25-iki/test_iki.py:91) — Function test_store_at_rest takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `test_cms_sign_verify` (examples/p25-iki/test_iki.py:105) — Function test_cms_sign_verify takes no inputs beyond the value it is called on, and gives back nothing; it does its work
- function `test_full_exchange` (examples/p25-iki/test_iki.py:118) — Function test_full_exchange takes no inputs beyond the value it is called on, and gives back nothing; it does its work o
- function `main` (examples/p25-iki/test_iki.py:126) — Function main takes no inputs beyond the value it is called on, and gives back int:.
- function `__init__` (examples/web-browser/browser.py:58) — Function __init__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `extract` (examples/web-browser/browser.py:61) — Function extract takes self, url, msg and gives back nothing; it does its work on what it is given.
- function `header_for` (examples/web-browser/browser.py:72) — Function header_for takes self, url and gives back nothing; it does its work on what it is given.
- function `__init__` (examples/web-browser/browser.py:118) — Function __init__ takes self, url, status, reason, headers, body, history and gives back nothing; it does its work on wh
- function `content_type` (examples/web-browser/browser.py:127) — Function content_type takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what
- function `text` (examples/web-browser/browser.py:130) — Decode the body: MIME charset parameter (RFC 2045) if given,
- function `fetch` (examples/web-browser/browser.py:139) — GET with redirect following (RFC 9110 section 15.4).
- function `open_stream` (examples/web-browser/browser.py:164) — Like fetch(), but returns (final_url, response, conn) with the body
- function `__init__` (examples/web-browser/browser.py:202) — Function __init__ takes self, base_url and gives back nothing; it does its work on what it is given.
- function `handle_starttag` (examples/web-browser/browser.py:235) — Function handle_starttag takes self, tag, attrs and gives back nothing; it does its work on what it is given.
- function `handle_endtag` (examples/web-browser/browser.py:273) — Function handle_endtag takes self, tag and gives back nothing; it does its work on what it is given.
- function `handle_data` (examples/web-browser/browser.py:299) — Function handle_data takes self, data and gives back nothing; it does its work on what it is given.
- function `rendered_text` (examples/web-browser/browser.py:309) — Function rendered_text takes no inputs beyond the value it is called on, and gives back nothing; it does its work on wha
- function `__init__` (examples/web-browser/browser.py:315) — Function __init__ takes self, result and gives back nothing; it does its work on what it is given.
- function `fetch_page` (examples/web-browser/browser.py:332) — Function fetch_page takes url, jar=None and gives back nothing; it does its work on what it is given.
- function `follow` (examples/web-browser/browser.py:336) — Follow link `number` (1-based, as printed) from a rendered page.
- function `display` (examples/web-browser/browser.py:344) — Function display takes page, out=None and gives back nothing; it does its work on what it is given.
- function `coord_str` (examples/web-browser/browser.py:372) — Function coord_str takes state and gives back nothing; it does its work on what it is given.
- function `__init__` (examples/web-browser/browser.py:383) — Function __init__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it 
- function `feed` (examples/web-browser/browser.py:387) — Returns [(coordinate, scroll_bytes)] completed by this data.
- function `finish` (examples/web-browser/browser.py:402) — The final scroll: (coordinate, bytes) for whatever remains.
- function `iter_phext_scrolls` (examples/web-browser/browser.py:407) — Stream a .phext from an HTTP/1.1 endpoint, yielding
- function `phext_toc` (examples/web-browser/browser.py:433) — The table of contents, as it streams: [(coordinate, byte_count)].
- function `phext_select` (examples/web-browser/browser.py:438) — Stream until the scroll at `wanted` completes; return its bytes
- function `run_phext` (examples/web-browser/browser.py:456) — Function run_phext takes url, select=None, jar=None and gives back nothing; it does its work on what it is given.
- function `main` (examples/web-browser/browser.py:496) — Function main takes argv and gives back nothing; it does its work on what it is given.
- function `check` (examples/web-browser/test_browser.py:32) — Function check takes label, condition, detail="" and gives back nothing; it does its work on what it is given.
- function `test_slice` (examples/web-browser/test_browser.py:57) — Function test_slice takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what i
- function `log_message` (examples/web-browser/test_browser.py:110) — Function log_message takes self, *args and gives back nothing; it does its work on what it is given.
- function `do_GET` (examples/web-browser/test_browser.py:123) — Function do_GET takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is
- function `test_http` (examples/web-browser/test_browser.py:140) — Function test_http takes base and gives back nothing; it does its work on what it is given.
- function `__init__` (examples/web-browser/test_browser.py:227) — Function __init__ takes self, payload, first_chunk_len, pause=0.5 and gives back nothing; it does its work on what it is
- function `run` (examples/web-browser/test_browser.py:240) — Function run takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is gi
- function `send_chunk` (examples/web-browser/test_browser.py:266) — Function send_chunk takes data and gives back nothing; it does its work on what it is given.
- function `stop` (examples/web-browser/test_browser.py:281) — Function stop takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g
- function `test_phext` (examples/web-browser/test_browser.py:289) — Function test_phext takes port, server and gives back nothing; it does its work on what it is given.
- function `test_live` (examples/web-browser/test_browser.py:326) — Function test_live takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it
- function `main` (examples/web-browser/test_browser.py:337) — Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is g

---

## function — __init__

Found in examples/ftp-client/ftp_client.py at line 53.

Signature: `def __init__(self, host, port=21, user="anonymous", password="anonymous@",`

What it does: Function __init__ takes self, host, port=21, user="anonymous", password="anonymous@", and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — connect

Found in examples/ftp-client/ftp_client.py at line 69.

Signature: `def connect(self):`

What it does: Function connect takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — close

Found in examples/ftp-client/ftp_client.py at line 122.

Signature: `def close(self):`

What it does: Function close takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — __enter__

Found in examples/ftp-client/ftp_client.py at line 130.

Signature: `def __enter__(self):`

What it does: Function __enter__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — __exit__

Found in examples/ftp-client/ftp_client.py at line 133.

Signature: `def __exit__(self, *exc):`

What it does: Function __exit__ takes self, *exc and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — pwd

Found in examples/ftp-client/ftp_client.py at line 143.

Signature: `def pwd(self):`

What it does: Function pwd takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — cd

Found in examples/ftp-client/ftp_client.py at line 146.

Signature: `def cd(self, path):`

What it does: Function cd takes self, path and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — list

Found in examples/ftp-client/ftp_client.py at line 149.

Signature: `def list(self, path=None):`

What it does: Directory listing: MLSD (RFC 3659) when offered, LIST fallback.

(Where these words came from: doc comment.)

---

## function — collect

Found in examples/ftp-client/ftp_client.py at line 160.

Signature: `def collect():`

What it does: Function collect takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — get

Found in examples/ftp-client/ftp_client.py at line 179.

Signature: `def get(self, remote, local=None):`

What it does: Function get takes self, remote, local=None and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run

Found in examples/ftp-client/ftp_client.py at line 181.

Signature: `def run():`

What it does: Function run takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — get_bytes

Found in examples/ftp-client/ftp_client.py at line 196.

Signature: `def get_bytes(self, remote):`

What it does: Function get_bytes takes self, remote and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — put

Found in examples/ftp-client/ftp_client.py at line 201.

Signature: `def put(self, local, remote=None):`

What it does: Function put takes self, local, remote=None and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run

Found in examples/ftp-client/ftp_client.py at line 203.

Signature: `def run():`

What it does: Function run takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — put_bytes

Found in examples/ftp-client/ftp_client.py at line 209.

Signature: `def put_bytes(self, data, remote):`

What it does: Function put_bytes takes self, data, remote and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — size

Found in examples/ftp-client/ftp_client.py at line 214.

Signature: `def size(self, path):`

What it does: RFC 3659 SIZE: byte count of a remote file.

(Where these words came from: doc comment.)

---

## function — mdtm

Found in examples/ftp-client/ftp_client.py at line 222.

Signature: `def mdtm(self, path):`

What it does: RFC 3659 MDTM: modification time of a remote file (UTC).

(Where these words came from: doc comment.)

---

## function — delete

Found in examples/ftp-client/ftp_client.py at line 231.

Signature: `def delete(self, path):`

What it does: Function delete takes self, path and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — mkdir

Found in examples/ftp-client/ftp_client.py at line 234.

Signature: `def mkdir(self, path):`

What it does: Function mkdir takes self, path and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — rmdir

Found in examples/ftp-client/ftp_client.py at line 237.

Signature: `def rmdir(self, path):`

What it does: Function rmdir takes self, path and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — parse_target

Found in examples/ftp-client/ftp_client.py at line 241.

Signature: `def parse_target(target, args):`

What it does: Target is a host or an ftp:// URL (RFC 3986 syntax; the ftp scheme

(Where these words came from: doc comment.)

---

## function — main

Found in examples/ftp-client/ftp_client.py at line 258.

Signature: `def main(argv=None):`

What it does: Function main takes argv=None and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — check

Found in examples/ftp-client/test_ftp.py at line 29.

Signature: `def check(label, cond, detail=""):`

What it does: Function check takes label, cond, detail="" and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_slice

Found in examples/ftp-client/test_ftp.py at line 38.

Signature: `def test_slice():`

What it does: Function test_slice takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — make_cert

Found in examples/ftp-client/test_ftp.py at line 68.

Signature: `def make_cert(tmp):`

What it does: Function make_cert takes tmp and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — start_server

Found in examples/ftp-client/test_ftp.py at line 94.

Signature: `def start_server(root, tls=None):`

What it does: Function start_server takes root, tls=None and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — exercise

Found in examples/ftp-client/test_ftp.py at line 120.

Signature: `def exercise(port, tls, label):`

What it does: Function exercise takes port, tls, label and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_errors

Found in examples/ftp-client/test_ftp.py at line 151.

Signature: `def test_errors(port):`

What it does: Function test_errors takes port and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_cli

Found in examples/ftp-client/test_ftp.py at line 164.

Signature: `def test_cli(port, tmp):`

What it does: Function test_cli takes port, tmp and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in examples/ftp-client/test_ftp.py at line 183.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run_demo

Found in examples/p25-iki/demo.py at line 27.

Signature: `def run_demo(base: Path = RUN_DIR, verbose: bool = True) -> dict:`

What it does: Function run_demo takes base: Path = RUN_DIR, verbose: bool = True and gives back dict:.

(Where these words came from: signature.)

---

## function — say

Found in examples/p25-iki/demo.py at line 28.

Signature: `def say(msg: str):`

What it does: Function say takes msg: str and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — check

Found in examples/p25-iki/demo.py at line 41.

Signature: `def check(label: str, ok: bool):`

What it does: Function check takes label: str, ok: bool and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in examples/p25-iki/demo.py at line 96.

Signature: `def main() -> int:`

What it does: Function main takes no inputs beyond the value it is called on, and gives back int:.

(Where these words came from: signature.)

---

## function — utcnow

Found in examples/p25-iki/iki_server.py at line 62.

Signature: `def utcnow() -> str:`

What it does: Function utcnow takes no inputs beyond the value it is called on, and gives back str:.

(Where these words came from: signature.)

---

## function — canonical

Found in examples/p25-iki/iki_server.py at line 66.

Signature: `def canonical(obj) -> bytes:`

What it does: Deterministic JSON encoding used for everything that gets signed.

(Where these words came from: doc comment.)

---

## function — b64e

Found in examples/p25-iki/iki_server.py at line 71.

Signature: `def b64e(b: bytes) -> str:`

What it does: Function b64e takes b: bytes and gives back str:.

(Where these words came from: signature.)

---

## function — b64d

Found in examples/p25-iki/iki_server.py at line 75.

Signature: `def b64d(s: str) -> bytes:`

What it does: Function b64d takes s: str and gives back bytes:.

(Where these words came from: signature.)

---

## function — make_ca

Found in examples/p25-iki/iki_server.py at line 87.

Signature: `def make_ca(workdir: Path, cn: str = "IKI Test CA") -> dict:`

What it does: Create a self-signed test CA. Returns paths + objects.

(Where these words came from: doc comment.)

---

## function — make_kmf_identity

Found in examples/p25-iki/iki_server.py at line 110.

Signature: `def make_kmf_identity(workdir: Path, name: str, ca: dict) -> dict:`

What it does: Issue a KMF identity certificate (server + client + signing use).

(Where these words came from: doc comment.)

---

## function — cms_sign_detached

Found in examples/p25-iki/iki_server.py at line 154.

Signature: `def cms_sign_detached(data: bytes, identity: dict) -> bytes:`

What it does: Detached CMS SignedData (DER) over `data`, signed by the KMF identity.

(Where these words came from: doc comment.)

---

## function — cms_verify_detached

Found in examples/p25-iki/iki_server.py at line 165.

Signature: `def cms_verify_detached(data: bytes, der_sig: bytes, ca_pem: Path) -> bool:`

What it does: Verify a detached CMS signature against the test CA using OpenSSL.

(Where these words came from: doc comment.)

---

## function — __init__

Found in examples/p25-iki/iki_server.py at line 184.

Signature: `def __init__(self, path: Path, actor: str):`

What it does: Function __init__ takes self, path: Path, actor: str and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — append

Found in examples/p25-iki/iki_server.py at line 195.

Signature: `def append(self, event: str, details: dict) -> dict:`

What it does: Function append takes self, event: str, details: dict and gives back dict:.

(Where these words came from: signature.)

---

## function — verify_chain

Found in examples/p25-iki/iki_server.py at line 207.

Signature: `def verify_chain(path: Path) -> bool:`

What it does: Function verify_chain takes path: Path and gives back bool:.

(Where these words came from: signature.)

---

## function — __init__

Found in examples/p25-iki/iki_server.py at line 226.

Signature: `def __init__(self, db_path: Path, master_key: bytes):`

What it does: Function __init__ takes self, db_path: Path, master_key: bytes and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — put_key

Found in examples/p25-iki/iki_server.py at line 252.

Signature: `def put_key(self, owner: str, key_id: int, alg_id: int,`

What it does: Function put_key takes self, owner: str, key_id: int, alg_id: int, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — get_key

Found in examples/p25-iki/iki_server.py at line 261.

Signature: `def get_key(self, owner: str, key_id: int, alg_id: int) -> bytes | None:`

What it does: Function get_key takes self, owner: str, key_id: int, alg_id: int and gives back bytes | None:.

(Where these words came from: signature.)

---

## function — list_keys

Found in examples/p25-iki/iki_server.py at line 269.

Signature: `def list_keys(self, owner: str) -> list[dict]:`

What it does: Function list_keys takes self, owner: str and gives back list[dict]:.

(Where these words came from: signature.)

---

## function — provision_peer

Found in examples/p25-iki/iki_server.py at line 276.

Signature: `def provision_peer(self, peer: str, root_secret: bytes) -> None:`

What it does: Function provision_peer takes self, peer: str, root_secret: bytes and gives back None:.

(Where these words came from: signature.)

---

## function — directional_kek

Found in examples/p25-iki/iki_server.py at line 283.

Signature: `def directional_kek(self, peer: str, src: str, dst: str) -> bytes:`

What it does: HKDF (RFC 5869): derive the src->dst wrap key from the pairwise root.

(Where these words came from: doc comment.)

---

## function — put_receipt

Found in examples/p25-iki/iki_server.py at line 294.

Signature: `def put_receipt(self, exchange_id: str, receipt: dict, signature: bytes) -> None:`

What it does: Function put_receipt takes self, exchange_id: str, receipt: dict, signature: bytes and gives back None:.

(Where these words came from: signature.)

---

## function — get_receipt

Found in examples/p25-iki/iki_server.py at line 301.

Signature: `def get_receipt(self, exchange_id: str) -> dict | None:`

What it does: Function get_receipt takes self, exchange_id: str and gives back dict | None:.

(Where these words came from: signature.)

---

## function — __init__

Found in examples/p25-iki/iki_server.py at line 315.

Signature: `def __init__(self, name: str, workdir: Path, ca: dict):`

What it does: Function __init__ takes self, name: str, workdir: Path, ca: dict and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — create_key

Found in examples/p25-iki/iki_server.py at line 331.

Signature: `def create_key(self, key_id: int, alg_id: int = ALG_AES_256) -> bytes:`

What it does: Function create_key takes self, key_id: int, alg_id: int = ALG_AES_256 and gives back bytes:.

(Where these words came from: signature.)

---

## function — provision_peer

Found in examples/p25-iki/iki_server.py at line 338.

Signature: `def provision_peer(self, peer_name: str, root_secret: bytes) -> None:`

What it does: Function provision_peer takes self, peer_name: str, root_secret: bytes and gives back None:.

(Where these words came from: signature.)

---

## function — export_keyset

Found in examples/p25-iki/iki_server.py at line 343.

Signature: `def export_keyset(self, peer: "KMF", key_ids: list[int],`

What it does: Function export_keyset takes self, peer: "KMF", key_ids: list[int], and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — handle_exchange

Found in examples/p25-iki/iki_server.py at line 384.

Signature: `def handle_exchange(self, manifest_bytes: bytes, signature: bytes,`

What it does: Function handle_exchange takes self, manifest_bytes: bytes, signature: bytes, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — start

Found in examples/p25-iki/iki_server.py at line 443.

Signature: `def start(self) -> None:`

What it does: Function start takes no inputs beyond the value it is called on, and gives back None:.

(Where these words came from: signature.)

---

## function — log_message

Found in examples/p25-iki/iki_server.py at line 447.

Signature: `def log_message(self, *args): # keep test output clean`

What it does: Function log_message takes self, *args and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — do_GET

Found in examples/p25-iki/iki_server.py at line 450.

Signature: `def do_GET(self):`

What it does: Function do_GET takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — do_POST

Found in examples/p25-iki/iki_server.py at line 456.

Signature: `def do_POST(self):`

What it does: Function do_POST takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — stop

Found in examples/p25-iki/iki_server.py at line 488.

Signature: `def stop(self) -> None:`

What it does: Function stop takes no inputs beyond the value it is called on, and gives back None:.

(Where these words came from: signature.)

---

## function — check

Found in examples/p25-iki/test_iki.py at line 32.

Signature: `def check(label: str, ok: bool, extra: str = ""):`

What it does: Function check takes label: str, ok: bool, extra: str = "" and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_slice_against_corpus

Found in examples/p25-iki/test_iki.py at line 49.

Signature: `def test_slice_against_corpus():`

What it does: Function test_slice_against_corpus takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_keywrap

Found in examples/p25-iki/test_iki.py at line 66.

Signature: `def test_keywrap():`

What it does: Function test_keywrap takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_hkdf_directions

Found in examples/p25-iki/test_iki.py at line 79.

Signature: `def test_hkdf_directions():`

What it does: Function test_hkdf_directions takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_store_at_rest

Found in examples/p25-iki/test_iki.py at line 91.

Signature: `def test_store_at_rest():`

What it does: Function test_store_at_rest takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_cms_sign_verify

Found in examples/p25-iki/test_iki.py at line 105.

Signature: `def test_cms_sign_verify():`

What it does: Function test_cms_sign_verify takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_full_exchange

Found in examples/p25-iki/test_iki.py at line 118.

Signature: `def test_full_exchange():`

What it does: Function test_full_exchange takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in examples/p25-iki/test_iki.py at line 126.

Signature: `def main() -> int:`

What it does: Function main takes no inputs beyond the value it is called on, and gives back int:.

(Where these words came from: signature.)

---

## function — __init__

Found in examples/web-browser/browser.py at line 58.

Signature: `def __init__(self):`

What it does: Function __init__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — extract

Found in examples/web-browser/browser.py at line 61.

Signature: `def extract(self, url, msg):`

What it does: Function extract takes self, url, msg and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — header_for

Found in examples/web-browser/browser.py at line 72.

Signature: `def header_for(self, url):`

What it does: Function header_for takes self, url and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — __init__

Found in examples/web-browser/browser.py at line 118.

Signature: `def __init__(self, url, status, reason, headers, body, history):`

What it does: Function __init__ takes self, url, status, reason, headers, body, history and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — content_type

Found in examples/web-browser/browser.py at line 127.

Signature: `def content_type(self):`

What it does: Function content_type takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — text

Found in examples/web-browser/browser.py at line 130.

Signature: `def text(self):`

What it does: Decode the body: MIME charset parameter (RFC 2045) if given,

(Where these words came from: doc comment.)

---

## function — fetch

Found in examples/web-browser/browser.py at line 139.

Signature: `def fetch(url, jar=None):`

What it does: GET with redirect following (RFC 9110 section 15.4). HTTP error

(Where these words came from: doc comment.)

---

## function — open_stream

Found in examples/web-browser/browser.py at line 164.

Signature: `def open_stream(url, jar=None):`

What it does: Like fetch(), but returns (final_url, response, conn) with the body

(Where these words came from: doc comment.)

---

## function — __init__

Found in examples/web-browser/browser.py at line 202.

Signature: `def __init__(self, base_url):`

What it does: Function __init__ takes self, base_url and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — handle_starttag

Found in examples/web-browser/browser.py at line 235.

Signature: `def handle_starttag(self, tag, attrs):`

What it does: Function handle_starttag takes self, tag, attrs and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — handle_endtag

Found in examples/web-browser/browser.py at line 273.

Signature: `def handle_endtag(self, tag):`

What it does: Function handle_endtag takes self, tag and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — handle_data

Found in examples/web-browser/browser.py at line 299.

Signature: `def handle_data(self, data):`

What it does: Function handle_data takes self, data and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — rendered_text

Found in examples/web-browser/browser.py at line 309.

Signature: `def rendered_text(self):`

What it does: Function rendered_text takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — __init__

Found in examples/web-browser/browser.py at line 315.

Signature: `def __init__(self, result):`

What it does: Function __init__ takes self, result and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — fetch_page

Found in examples/web-browser/browser.py at line 332.

Signature: `def fetch_page(url, jar=None):`

What it does: Function fetch_page takes url, jar=None and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — follow

Found in examples/web-browser/browser.py at line 336.

Signature: `def follow(page, number):`

What it does: Follow link `number` (1-based, as printed) from a rendered page.

(Where these words came from: doc comment.)

---

## function — display

Found in examples/web-browser/browser.py at line 344.

Signature: `def display(page, out=None):`

What it does: Function display takes page, out=None and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — coord_str

Found in examples/web-browser/browser.py at line 372.

Signature: `def coord_str(state):`

What it does: Function coord_str takes state and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — __init__

Found in examples/web-browser/browser.py at line 383.

Signature: `def __init__(self):`

What it does: Function __init__ takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — feed

Found in examples/web-browser/browser.py at line 387.

Signature: `def feed(self, data):`

What it does: Returns [(coordinate, scroll_bytes)] completed by this data.

(Where these words came from: doc comment.)

---

## function — finish

Found in examples/web-browser/browser.py at line 402.

Signature: `def finish(self):`

What it does: The final scroll: (coordinate, bytes) for whatever remains.

(Where these words came from: doc comment.)

---

## function — iter_phext_scrolls

Found in examples/web-browser/browser.py at line 407.

Signature: `def iter_phext_scrolls(url, jar=None, chunk_size=4096):`

What it does: Stream a .phext from an HTTP/1.1 endpoint, yielding

(Where these words came from: doc comment.)

---

## function — phext_toc

Found in examples/web-browser/browser.py at line 433.

Signature: `def phext_toc(url, jar=None):`

What it does: The table of contents, as it streams: [(coordinate, byte_count)].

(Where these words came from: doc comment.)

---

## function — phext_select

Found in examples/web-browser/browser.py at line 438.

Signature: `def phext_select(url, wanted, jar=None):`

What it does: Stream until the scroll at `wanted` completes; return its bytes

(Where these words came from: doc comment.)

---

## function — run_phext

Found in examples/web-browser/browser.py at line 456.

Signature: `def run_phext(url, select=None, jar=None):`

What it does: Function run_phext takes url, select=None, jar=None and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in examples/web-browser/browser.py at line 496.

Signature: `def main(argv):`

What it does: Function main takes argv and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — check

Found in examples/web-browser/test_browser.py at line 32.

Signature: `def check(label, condition, detail=""):`

What it does: Function check takes label, condition, detail="" and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_slice

Found in examples/web-browser/test_browser.py at line 57.

Signature: `def test_slice():`

What it does: Function test_slice takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — log_message

Found in examples/web-browser/test_browser.py at line 110.

Signature: `def log_message(self, *args):`

What it does: Function log_message takes self, *args and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — do_GET

Found in examples/web-browser/test_browser.py at line 123.

Signature: `def do_GET(self):`

What it does: Function do_GET takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_http

Found in examples/web-browser/test_browser.py at line 140.

Signature: `def test_http(base):`

What it does: Function test_http takes base and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — __init__

Found in examples/web-browser/test_browser.py at line 227.

Signature: `def __init__(self, payload, first_chunk_len, pause=0.5):`

What it does: Function __init__ takes self, payload, first_chunk_len, pause=0.5 and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — run

Found in examples/web-browser/test_browser.py at line 240.

Signature: `def run(self):`

What it does: Function run takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — send_chunk

Found in examples/web-browser/test_browser.py at line 266.

Signature: `def send_chunk(data):`

What it does: Function send_chunk takes data and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — stop

Found in examples/web-browser/test_browser.py at line 281.

Signature: `def stop(self):`

What it does: Function stop takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_phext

Found in examples/web-browser/test_browser.py at line 289.

Signature: `def test_phext(port, server):`

What it does: Function test_phext takes port, server and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — test_live

Found in examples/web-browser/test_browser.py at line 326.

Signature: `def test_live():`

What it does: Function test_live takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## function — main

Found in examples/web-browser/test_browser.py at line 337.

Signature: `def main():`

What it does: Function main takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.

(Where these words came from: signature.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/rfcbox/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

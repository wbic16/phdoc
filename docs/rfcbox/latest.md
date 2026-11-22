# rfcbox — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 150). Scanned at commit da2c4d7048883a12b3c281c30ee260d5a76d4a23. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Summary

One of the first multiversal effects we implement. **ietf.phext is the first phext in its data model.**

(The opening of the project's README, quoted as written.)

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
- Shell: 1 file, 14 lines total, 2 blank, 5 comment, 7 code

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
- Public items with a doc comment: 20 of 109 (18%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/rfcbox
- Branch: exo
- Head commit: da2c4d7048883a12b3c281c30ee260d5a76d4a23
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `FTPClient.__init__` (examples/ftp-client/ftp_client.py:53) [example] — Function FTPClient.__init__ takes self, host, port=21, user="anonymous", password="anonymous@", tls=False…
- function `FTPClient.connect` (examples/ftp-client/ftp_client.py:69) [example] — Function FTPClient.connect takes no inputs beyond the value it is called on.
- function `FTPClient.close` (examples/ftp-client/ftp_client.py:122) [example] — Function FTPClient.close takes no inputs beyond the value it is called on.
- function `FTPClient.__enter__` (examples/ftp-client/ftp_client.py:130) [example] — Function FTPClient.__enter__ takes no inputs beyond the value it is called on.
- function `FTPClient.__exit__` (examples/ftp-client/ftp_client.py:133) [example] — Function FTPClient.__exit__ takes self, *exc.
- function `FTPClient.pwd` (examples/ftp-client/ftp_client.py:143) [example] — Function FTPClient.pwd takes no inputs beyond the value it is called on.
- function `FTPClient.cd` (examples/ftp-client/ftp_client.py:146) [example] — Function FTPClient.cd takes self, path.
- function `FTPClient.list` (examples/ftp-client/ftp_client.py:149) [example] — Directory listing: MLSD (RFC 3659) when offered, LIST fallback.
- function `FTPClient.collect` (examples/ftp-client/ftp_client.py:160) [example] — Function FTPClient.collect takes no inputs beyond the value it is called on.
- function `FTPClient.get` (examples/ftp-client/ftp_client.py:179) [example] — Function FTPClient.get takes self, remote, local=None.
- function `FTPClient.run` (examples/ftp-client/ftp_client.py:181) [example] — Function FTPClient.run takes no inputs beyond the value it is called on.
- function `FTPClient.get_bytes` (examples/ftp-client/ftp_client.py:196) [example] — Function FTPClient.get_bytes takes self, remote.
- function `FTPClient.put` (examples/ftp-client/ftp_client.py:201) [example] — Function FTPClient.put takes self, local, remote=None.
- function `FTPClient.run` (examples/ftp-client/ftp_client.py:203) [example] — Function FTPClient.run takes no inputs beyond the value it is called on.
- function `FTPClient.put_bytes` (examples/ftp-client/ftp_client.py:209) [example] — Function FTPClient.put_bytes takes self, data, remote.
- function `FTPClient.size` (examples/ftp-client/ftp_client.py:214) [example] — RFC 3659 SIZE: byte count of a remote file.
- function `FTPClient.mdtm` (examples/ftp-client/ftp_client.py:222) [example] — RFC 3659 MDTM: modification time of a remote file (UTC).
- function `FTPClient.delete` (examples/ftp-client/ftp_client.py:231) [example] — Function FTPClient.delete takes self, path.
- function `FTPClient.mkdir` (examples/ftp-client/ftp_client.py:234) [example] — Function FTPClient.mkdir takes self, path.
- function `FTPClient.rmdir` (examples/ftp-client/ftp_client.py:237) [example] — Function FTPClient.rmdir takes self, path.
- function `parse_target` (examples/ftp-client/ftp_client.py:241) [example] — Target is a host or an ftp:// URL (RFC 3986 syntax; the ftp scheme
- function `main` (examples/ftp-client/ftp_client.py:258) [example] — Function main takes argv=None.
- function `check` (examples/ftp-client/test_ftp.py:29) [test] — Function check takes label, cond, detail="".
- function `test_slice` (examples/ftp-client/test_ftp.py:38) [test] — Function test_slice takes no inputs beyond the value it is called on.
- function `make_cert` (examples/ftp-client/test_ftp.py:68) [test] — Function make_cert takes tmp.
- function `start_server` (examples/ftp-client/test_ftp.py:94) [test] — Function start_server takes root, tls=None.
- function `exercise` (examples/ftp-client/test_ftp.py:120) [test] — Function exercise takes port, tls, label.
- function `test_errors` (examples/ftp-client/test_ftp.py:151) [test] — Function test_errors takes port.
- function `test_cli` (examples/ftp-client/test_ftp.py:164) [test] — Function test_cli takes port, tmp.
- function `main` (examples/ftp-client/test_ftp.py:183) [test] — Function main takes no inputs beyond the value it is called on.
- function `run_demo` (examples/p25-iki/demo.py:27) [example] — Function run_demo takes base: Path = RUN_DIR, verbose: bool = True.
- function `say` (examples/p25-iki/demo.py:28) [example] — Function say takes msg: str.
- function `check` (examples/p25-iki/demo.py:41) [example] — Function check takes label: str, ok: bool.
- function `main` (examples/p25-iki/demo.py:96) [example] — Function main takes no inputs beyond the value it is called on.
- function `utcnow` (examples/p25-iki/iki_server.py:62) [example] — Function utcnow takes no inputs beyond the value it is called on.
- function `canonical` (examples/p25-iki/iki_server.py:66) [example] — Deterministic JSON encoding used for everything that gets signed.
- function `b64e` (examples/p25-iki/iki_server.py:71) [example] — Function b64e takes b: bytes.
- function `b64d` (examples/p25-iki/iki_server.py:75) [example] — Function b64d takes s: str.
- function `make_ca` (examples/p25-iki/iki_server.py:87) [example] — Create a self-signed test CA.
- function `make_kmf_identity` (examples/p25-iki/iki_server.py:110) [example] — Issue a KMF identity certificate (server + client + signing use).
- function `cms_sign_detached` (examples/p25-iki/iki_server.py:154) [example] — Detached CMS SignedData (DER) over `data`, signed by the KMF identity.
- function `cms_verify_detached` (examples/p25-iki/iki_server.py:165) [example] — Verify a detached CMS signature against the test CA using OpenSSL.
- function `AuditLog.__init__` (examples/p25-iki/iki_server.py:184) [example] — Function AuditLog.__init__ takes self, path: Path, actor: str.
- function `AuditLog.append` (examples/p25-iki/iki_server.py:195) [example] — Function AuditLog.append takes self, event: str, details: dict.
- function `AuditLog.verify_chain` (examples/p25-iki/iki_server.py:207) [example] — Function AuditLog.verify_chain takes path: Path.
- function `KeyStore.__init__` (examples/p25-iki/iki_server.py:226) [example] — Function KeyStore.__init__ takes self, db_path: Path, master_key: bytes.
- function `KeyStore.put_key` (examples/p25-iki/iki_server.py:252) [example] — Function KeyStore.put_key takes self, owner: str, key_id: int, alg_id: int, key: bytes, origin: str.
- function `KeyStore.get_key` (examples/p25-iki/iki_server.py:261) [example] — Function KeyStore.get_key takes self, owner: str, key_id: int, alg_id: int.
- function `KeyStore.list_keys` (examples/p25-iki/iki_server.py:269) [example] — Function KeyStore.list_keys takes self, owner: str.
- function `KeyStore.provision_peer` (examples/p25-iki/iki_server.py:276) [example] — Function KeyStore.provision_peer takes self, peer: str, root_secret: bytes.
- function `KeyStore.directional_kek` (examples/p25-iki/iki_server.py:283) [example] — HKDF (RFC 5869): derive the src->dst wrap key from the pairwise root.
- function `KeyStore.put_receipt` (examples/p25-iki/iki_server.py:294) [example] — Function KeyStore.put_receipt takes self, exchange_id: str, receipt: dict, signature: bytes.
- function `KeyStore.get_receipt` (examples/p25-iki/iki_server.py:301) [example] — Function KeyStore.get_receipt takes self, exchange_id: str.
- function `KMF.__init__` (examples/p25-iki/iki_server.py:315) [example] — Function KMF.__init__ takes self, name: str, workdir: Path, ca: dict.
- function `KMF.create_key` (examples/p25-iki/iki_server.py:331) [example] — Function KMF.create_key takes self, key_id: int, alg_id: int = ALG_AES_256.
- function `KMF.provision_peer` (examples/p25-iki/iki_server.py:338) [example] — Function KMF.provision_peer takes self, peer_name: str, root_secret: bytes.
- function `KMF.export_keyset` (examples/p25-iki/iki_server.py:343) [example] — Wrap this KMF's keys for `peer`, sign the package (CMS), send it
- function `KMF.handle_exchange` (examples/p25-iki/iki_server.py:384) [example] — Function KMF.handle_exchange takes self, manifest_bytes: bytes, signature: bytes, peer_cn: str.
- function `KMF.start` (examples/p25-iki/iki_server.py:443) [example] — Function KMF.start takes no inputs beyond the value it is called on.
- function `Handler.log_message` (examples/p25-iki/iki_server.py:447) [example] — Function Handler.log_message takes self, *args.
- function `Handler.do_GET` (examples/p25-iki/iki_server.py:450) [example] — Function Handler.do_GET takes no inputs beyond the value it is called on.
- function `Handler.do_POST` (examples/p25-iki/iki_server.py:456) [example] — Function Handler.do_POST takes no inputs beyond the value it is called on.
- function `KMF.stop` (examples/p25-iki/iki_server.py:488) [example] — Function KMF.stop takes no inputs beyond the value it is called on.
- function `check` (examples/p25-iki/test_iki.py:32) [test] — Function check takes label: str, ok: bool, extra: str = "".
- function `test_slice_against_corpus` (examples/p25-iki/test_iki.py:49) [test] — Function test_slice_against_corpus takes no inputs beyond the value it is called on.
- function `test_keywrap` (examples/p25-iki/test_iki.py:66) [test] — Function test_keywrap takes no inputs beyond the value it is called on.
- function `test_hkdf_directions` (examples/p25-iki/test_iki.py:79) [test] — Function test_hkdf_directions takes no inputs beyond the value it is called on.
- function `test_store_at_rest` (examples/p25-iki/test_iki.py:91) [test] — Function test_store_at_rest takes no inputs beyond the value it is called on.
- function `test_cms_sign_verify` (examples/p25-iki/test_iki.py:105) [test] — Function test_cms_sign_verify takes no inputs beyond the value it is called on.
- function `test_full_exchange` (examples/p25-iki/test_iki.py:118) [test] — Function test_full_exchange takes no inputs beyond the value it is called on.
- function `main` (examples/p25-iki/test_iki.py:126) [test] — Function main takes no inputs beyond the value it is called on.
- function `CookieJar.__init__` (examples/web-browser/browser.py:58) [example] — Function CookieJar.__init__ takes no inputs beyond the value it is called on.
- function `CookieJar.extract` (examples/web-browser/browser.py:61) [example] — Function CookieJar.extract takes self, url, msg.
- function `CookieJar.header_for` (examples/web-browser/browser.py:72) [example] — Function CookieJar.header_for takes self, url.
- function `FetchResult.__init__` (examples/web-browser/browser.py:118) [example] — Function FetchResult.__init__ takes self, url, status, reason, headers, body, history.
- function `FetchResult.content_type` (examples/web-browser/browser.py:127) [example] — Function FetchResult.content_type takes no inputs beyond the value it is called on.
- function `FetchResult.text` (examples/web-browser/browser.py:130) [example] — Decode the body: MIME charset parameter (RFC 2045) if given,
- function `fetch` (examples/web-browser/browser.py:139) [example] — GET with redirect following (RFC 9110 section 15.4).
- function `open_stream` (examples/web-browser/browser.py:164) [example] — Like fetch(), but returns (final_url, response, conn) with the body
- function `Renderer.__init__` (examples/web-browser/browser.py:202) [example] — Function Renderer.__init__ takes self, base_url.
- function `Renderer.handle_starttag` (examples/web-browser/browser.py:235) [example] — Function Renderer.handle_starttag takes self, tag, attrs.
- function `Renderer.handle_endtag` (examples/web-browser/browser.py:273) [example] — Function Renderer.handle_endtag takes self, tag.
- function `Renderer.handle_data` (examples/web-browser/browser.py:299) [example] — Function Renderer.handle_data takes self, data.
- function `Renderer.rendered_text` (examples/web-browser/browser.py:309) [example] — Function Renderer.rendered_text takes no inputs beyond the value it is called on.
- function `Page.__init__` (examples/web-browser/browser.py:315) [example] — Function Page.__init__ takes self, result.
- function `fetch_page` (examples/web-browser/browser.py:332) [example] — Function fetch_page takes url, jar=None.
- function `follow` (examples/web-browser/browser.py:336) [example] — Follow link `number` (1-based, as printed) from a rendered page.
- function `display` (examples/web-browser/browser.py:344) [example] — Function display takes page, out=None.
- function `coord_str` (examples/web-browser/browser.py:372) [example] — Function coord_str takes state.
- function `PhextStreamParser.__init__` (examples/web-browser/browser.py:383) [example] — Function PhextStreamParser.__init__ takes no inputs beyond the value it is called on.
- function `PhextStreamParser.feed` (examples/web-browser/browser.py:387) [example] — Returns [(coordinate, scroll_bytes)] completed by this data.
- function `PhextStreamParser.finish` (examples/web-browser/browser.py:402) [example] — The final scroll: (coordinate, bytes) for whatever remains.
- function `iter_phext_scrolls` (examples/web-browser/browser.py:407) [example] — Stream a .phext from an HTTP/1.1 endpoint, yielding
- function `phext_toc` (examples/web-browser/browser.py:433) [example] — The table of contents, as it streams: [(coordinate, byte_count)].
- function `phext_select` (examples/web-browser/browser.py:438) [example] — Stream until the scroll at `wanted` completes; return its bytes
- function `run_phext` (examples/web-browser/browser.py:456) [example] — Function run_phext takes url, select=None, jar=None.
- function `main` (examples/web-browser/browser.py:496) [example] — Function main takes argv.
- function `check` (examples/web-browser/test_browser.py:32) [test] — Function check takes label, condition, detail="".
- function `test_slice` (examples/web-browser/test_browser.py:57) [test] — Function test_slice takes no inputs beyond the value it is called on.
- function `FixtureHandler.log_message` (examples/web-browser/test_browser.py:110) [test] — Function FixtureHandler.log_message takes self, *args.
- function `FixtureHandler.do_GET` (examples/web-browser/test_browser.py:123) [test] — Function FixtureHandler.do_GET takes no inputs beyond the value it is called on.
- function `test_http` (examples/web-browser/test_browser.py:140) [test] — Function test_http takes base.
- function `ChunkedPhextServer.__init__` (examples/web-browser/test_browser.py:227) [test] — Function ChunkedPhextServer.__init__ takes self, payload, first_chunk_len, pause=0.5.
- function `ChunkedPhextServer.run` (examples/web-browser/test_browser.py:240) [test] — Function ChunkedPhextServer.run takes no inputs beyond the value it is called on.
- function `ChunkedPhextServer.send_chunk` (examples/web-browser/test_browser.py:266) [test] — Function ChunkedPhextServer.send_chunk takes data.
- function `ChunkedPhextServer.stop` (examples/web-browser/test_browser.py:281) [test] — Function ChunkedPhextServer.stop takes no inputs beyond the value it is called on.
- function `test_phext` (examples/web-browser/test_browser.py:289) [test] — Function test_phext takes port, server.
- function `test_live` (examples/web-browser/test_browser.py:326) [test] — Function test_live takes no inputs beyond the value it is called on.
- function `main` (examples/web-browser/test_browser.py:337) [test] — Function main takes no inputs beyond the value it is called on.

---

## function — FTPClient.__init__

Found in examples/ftp-client/ftp_client.py at line 53.

Surface: example.

Signature: `def __init__(self, host, port=21, user="anonymous", password="anonymous@", tls=False, timeout=30, passive=True, use_epsv=True, tls_verify=True)`

What it does: Function FTPClient.__init__ takes self, host, port=21, user="anonymous", password="anonymous@", tls=False, timeout=30, passive=True, use_epsv=True, tls_verify=True. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FTPClient.connect

Found in examples/ftp-client/ftp_client.py at line 69.

Surface: example.

Signature: `def connect(self)`

What it does: Function FTPClient.connect takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning self.

(Where these words came from: inferred.)

---

## function — FTPClient.close

Found in examples/ftp-client/ftp_client.py at line 122.

Surface: example.

Signature: `def close(self)`

What it does: Function FTPClient.close takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FTPClient.__enter__

Found in examples/ftp-client/ftp_client.py at line 130.

Surface: example.

Signature: `def __enter__(self)`

What it does: Function FTPClient.__enter__ takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning self.connect().

(Where these words came from: inferred.)

---

## function — FTPClient.__exit__

Found in examples/ftp-client/ftp_client.py at line 133.

Surface: example.

Signature: `def __exit__(self, *exc)`

What it does: Function FTPClient.__exit__ takes self, *exc. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FTPClient.pwd

Found in examples/ftp-client/ftp_client.py at line 143.

Surface: example.

Signature: `def pwd(self)`

What it does: Function FTPClient.pwd takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning self._guard(self.ftp.pwd).

(Where these words came from: inferred.)

---

## function — FTPClient.cd

Found in examples/ftp-client/ftp_client.py at line 146.

Surface: example.

Signature: `def cd(self, path)`

What it does: Function FTPClient.cd takes self, path. Its return type is unannotated; the code shows it returning self._guard(self.ftp.cwd, path).

(Where these words came from: inferred.)

---

## function — FTPClient.list

Found in examples/ftp-client/ftp_client.py at line 149.

Surface: example.

Signature: `def list(self, path=None)`

What it does: Directory listing: MLSD (RFC 3659) when offered, LIST fallback.

(Where these words came from: direct.)

---

## function — FTPClient.collect

Found in examples/ftp-client/ftp_client.py at line 160.

Surface: example.

Signature: `def collect()`

What it does: Function FTPClient.collect takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FTPClient.get

Found in examples/ftp-client/ftp_client.py at line 179.

Surface: example.

Signature: `def get(self, remote, local=None)`

What it does: Function FTPClient.get takes self, remote, local=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FTPClient.run

Found in examples/ftp-client/ftp_client.py at line 181.

Surface: example.

Signature: `def run()`

What it does: Function FTPClient.run takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FTPClient.get_bytes

Found in examples/ftp-client/ftp_client.py at line 196.

Surface: example.

Signature: `def get_bytes(self, remote)`

What it does: Function FTPClient.get_bytes takes self, remote. Its return type is unannotated; the code shows it returning b"".join(chunks).

(Where these words came from: inferred.)

---

## function — FTPClient.put

Found in examples/ftp-client/ftp_client.py at line 201.

Surface: example.

Signature: `def put(self, local, remote=None)`

What it does: Function FTPClient.put takes self, local, remote=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FTPClient.run

Found in examples/ftp-client/ftp_client.py at line 203.

Surface: example.

Signature: `def run()`

What it does: Function FTPClient.run takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FTPClient.put_bytes

Found in examples/ftp-client/ftp_client.py at line 209.

Surface: example.

Signature: `def put_bytes(self, data, remote)`

What it does: Function FTPClient.put_bytes takes self, data, remote. Its return type is unannotated; the code shows it returning remote.

(Where these words came from: inferred.)

---

## function — FTPClient.size

Found in examples/ftp-client/ftp_client.py at line 214.

Surface: example.

Signature: `def size(self, path)`

What it does: RFC 3659 SIZE: byte count of a remote file.

(Where these words came from: direct.)

---

## function — FTPClient.mdtm

Found in examples/ftp-client/ftp_client.py at line 222.

Surface: example.

Signature: `def mdtm(self, path)`

What it does: RFC 3659 MDTM: modification time of a remote file (UTC).

(Where these words came from: direct.)

---

## function — FTPClient.delete

Found in examples/ftp-client/ftp_client.py at line 231.

Surface: example.

Signature: `def delete(self, path)`

What it does: Function FTPClient.delete takes self, path. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FTPClient.mkdir

Found in examples/ftp-client/ftp_client.py at line 234.

Surface: example.

Signature: `def mkdir(self, path)`

What it does: Function FTPClient.mkdir takes self, path. Its return type is unannotated; the code shows it returning self._guard(self.ftp.mkd, path).

(Where these words came from: inferred.)

---

## function — FTPClient.rmdir

Found in examples/ftp-client/ftp_client.py at line 237.

Surface: example.

Signature: `def rmdir(self, path)`

What it does: Function FTPClient.rmdir takes self, path. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — parse_target

Found in examples/ftp-client/ftp_client.py at line 241.

Surface: example.

Signature: `def parse_target(target, args)`

What it does: Target is a host or an ftp:// URL (RFC 3986 syntax; the ftp scheme

(Where these words came from: direct.)

---

## function — main

Found in examples/ftp-client/ftp_client.py at line 258.

Surface: example.

Signature: `def main(argv=None)`

What it does: Function main takes argv=None. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — check

Found in examples/ftp-client/test_ftp.py at line 29.

Surface: test.

Signature: `def check(label, cond, detail="")`

What it does: Function check takes label, cond, detail="". Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_slice

Found in examples/ftp-client/test_ftp.py at line 38.

Surface: test.

Signature: `def test_slice()`

What it does: Function test_slice takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — make_cert

Found in examples/ftp-client/test_ftp.py at line 68.

Surface: test.

Signature: `def make_cert(tmp)`

What it does: Function make_cert takes tmp. Its return type is unannotated; the code shows it returning cert_p, key_p.

(Where these words came from: inferred.)

---

## function — start_server

Found in examples/ftp-client/test_ftp.py at line 94.

Surface: test.

Signature: `def start_server(root, tls=None)`

What it does: Function start_server takes root, tls=None. Its return type is unannotated; the code shows it returning server, port.

(Where these words came from: inferred.)

---

## function — exercise

Found in examples/ftp-client/test_ftp.py at line 120.

Surface: test.

Signature: `def exercise(port, tls, label)`

What it does: Function exercise takes port, tls, label. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_errors

Found in examples/ftp-client/test_ftp.py at line 151.

Surface: test.

Signature: `def test_errors(port)`

What it does: Function test_errors takes port. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_cli

Found in examples/ftp-client/test_ftp.py at line 164.

Surface: test.

Signature: `def test_cli(port, tmp)`

What it does: Function test_cli takes port, tmp. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in examples/ftp-client/test_ftp.py at line 183.

Surface: test.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — run_demo

Found in examples/p25-iki/demo.py at line 27.

Surface: example.

Signature: `def run_demo(base: Path = RUN_DIR, verbose: bool = True) -> dict`

What it does: Function run_demo takes base: Path = RUN_DIR, verbose: bool = True. It gives back dict.

(Where these words came from: inferred.)

---

## function — say

Found in examples/p25-iki/demo.py at line 28.

Surface: example.

Signature: `def say(msg: str)`

What it does: Function say takes msg: str. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — check

Found in examples/p25-iki/demo.py at line 41.

Surface: example.

Signature: `def check(label: str, ok: bool)`

What it does: Function check takes label: str, ok: bool. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in examples/p25-iki/demo.py at line 96.

Surface: example.

Signature: `def main() -> int`

What it does: Function main takes no inputs beyond the value it is called on. It gives back int.

(Where these words came from: inferred.)

---

## function — utcnow

Found in examples/p25-iki/iki_server.py at line 62.

Surface: example.

Signature: `def utcnow() -> str`

What it does: Function utcnow takes no inputs beyond the value it is called on. It gives back str.

(Where these words came from: inferred.)

---

## function — canonical

Found in examples/p25-iki/iki_server.py at line 66.

Surface: example.

Signature: `def canonical(obj) -> bytes`

What it does: Deterministic JSON encoding used for everything that gets signed.

(Where these words came from: direct.)

---

## function — b64e

Found in examples/p25-iki/iki_server.py at line 71.

Surface: example.

Signature: `def b64e(b: bytes) -> str`

What it does: Function b64e takes b: bytes. It gives back str.

(Where these words came from: inferred.)

---

## function — b64d

Found in examples/p25-iki/iki_server.py at line 75.

Surface: example.

Signature: `def b64d(s: str) -> bytes`

What it does: Function b64d takes s: str. It gives back bytes.

(Where these words came from: inferred.)

---

## function — make_ca

Found in examples/p25-iki/iki_server.py at line 87.

Surface: example.

Signature: `def make_ca(workdir: Path, cn: str = "IKI Test CA") -> dict`

What it does: Create a self-signed test CA. Returns paths + objects.

(Where these words came from: direct.)

---

## function — make_kmf_identity

Found in examples/p25-iki/iki_server.py at line 110.

Surface: example.

Signature: `def make_kmf_identity(workdir: Path, name: str, ca: dict) -> dict`

What it does: Issue a KMF identity certificate (server + client + signing use).

(Where these words came from: direct.)

---

## function — cms_sign_detached

Found in examples/p25-iki/iki_server.py at line 154.

Surface: example.

Signature: `def cms_sign_detached(data: bytes, identity: dict) -> bytes`

What it does: Detached CMS SignedData (DER) over `data`, signed by the KMF identity.

(Where these words came from: direct.)

---

## function — cms_verify_detached

Found in examples/p25-iki/iki_server.py at line 165.

Surface: example.

Signature: `def cms_verify_detached(data: bytes, der_sig: bytes, ca_pem: Path) -> bool`

What it does: Verify a detached CMS signature against the test CA using OpenSSL.

(Where these words came from: direct.)

---

## function — AuditLog.__init__

Found in examples/p25-iki/iki_server.py at line 184.

Surface: example.

Signature: `def __init__(self, path: Path, actor: str)`

What it does: Function AuditLog.__init__ takes self, path: Path, actor: str. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — AuditLog.append

Found in examples/p25-iki/iki_server.py at line 195.

Surface: example.

Signature: `def append(self, event: str, details: dict) -> dict`

What it does: Function AuditLog.append takes self, event: str, details: dict. It gives back dict.

(Where these words came from: inferred.)

---

## function — AuditLog.verify_chain

Found in examples/p25-iki/iki_server.py at line 207.

Surface: example.

Signature: `def verify_chain(path: Path) -> bool`

What it does: Function AuditLog.verify_chain takes path: Path. It gives back bool.

(Where these words came from: inferred.)

---

## function — KeyStore.__init__

Found in examples/p25-iki/iki_server.py at line 226.

Surface: example.

Signature: `def __init__(self, db_path: Path, master_key: bytes)`

What it does: Function KeyStore.__init__ takes self, db_path: Path, master_key: bytes. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — KeyStore.put_key

Found in examples/p25-iki/iki_server.py at line 252.

Surface: example.

Signature: `def put_key(self, owner: str, key_id: int, alg_id: int, key: bytes, origin: str) -> None`

What it does: Function KeyStore.put_key takes self, owner: str, key_id: int, alg_id: int, key: bytes, origin: str. It gives back nothing.

(Where these words came from: inferred.)

---

## function — KeyStore.get_key

Found in examples/p25-iki/iki_server.py at line 261.

Surface: example.

Signature: `def get_key(self, owner: str, key_id: int, alg_id: int) -> bytes | None`

What it does: Function KeyStore.get_key takes self, owner: str, key_id: int, alg_id: int. It gives back bytes | None.

(Where these words came from: inferred.)

---

## function — KeyStore.list_keys

Found in examples/p25-iki/iki_server.py at line 269.

Surface: example.

Signature: `def list_keys(self, owner: str) -> list[dict]`

What it does: Function KeyStore.list_keys takes self, owner: str. It gives back list[dict].

(Where these words came from: inferred.)

---

## function — KeyStore.provision_peer

Found in examples/p25-iki/iki_server.py at line 276.

Surface: example.

Signature: `def provision_peer(self, peer: str, root_secret: bytes) -> None`

What it does: Function KeyStore.provision_peer takes self, peer: str, root_secret: bytes. It gives back nothing.

(Where these words came from: inferred.)

---

## function — KeyStore.directional_kek

Found in examples/p25-iki/iki_server.py at line 283.

Surface: example.

Signature: `def directional_kek(self, peer: str, src: str, dst: str) -> bytes`

What it does: HKDF (RFC 5869): derive the src->dst wrap key from the pairwise root.

(Where these words came from: direct.)

---

## function — KeyStore.put_receipt

Found in examples/p25-iki/iki_server.py at line 294.

Surface: example.

Signature: `def put_receipt(self, exchange_id: str, receipt: dict, signature: bytes) -> None`

What it does: Function KeyStore.put_receipt takes self, exchange_id: str, receipt: dict, signature: bytes. It gives back nothing.

(Where these words came from: inferred.)

---

## function — KeyStore.get_receipt

Found in examples/p25-iki/iki_server.py at line 301.

Surface: example.

Signature: `def get_receipt(self, exchange_id: str) -> dict | None`

What it does: Function KeyStore.get_receipt takes self, exchange_id: str. It gives back dict | None.

(Where these words came from: inferred.)

---

## function — KMF.__init__

Found in examples/p25-iki/iki_server.py at line 315.

Surface: example.

Signature: `def __init__(self, name: str, workdir: Path, ca: dict)`

What it does: Function KMF.__init__ takes self, name: str, workdir: Path, ca: dict. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — KMF.create_key

Found in examples/p25-iki/iki_server.py at line 331.

Surface: example.

Signature: `def create_key(self, key_id: int, alg_id: int = ALG_AES_256) -> bytes`

What it does: Function KMF.create_key takes self, key_id: int, alg_id: int = ALG_AES_256. It gives back bytes.

(Where these words came from: inferred.)

---

## function — KMF.provision_peer

Found in examples/p25-iki/iki_server.py at line 338.

Surface: example.

Signature: `def provision_peer(self, peer_name: str, root_secret: bytes) -> None`

What it does: Function KMF.provision_peer takes self, peer_name: str, root_secret: bytes. It gives back nothing.

(Where these words came from: inferred.)

---

## function — KMF.export_keyset

Found in examples/p25-iki/iki_server.py at line 343.

Surface: example.

Signature: `def export_keyset(self, peer: "KMF", key_ids: list[int], alg_id: int = ALG_AES_256) -> dict`

What it does: Wrap this KMF's keys for `peer`, sign the package (CMS), send it

(Where these words came from: direct.)

---

## function — KMF.handle_exchange

Found in examples/p25-iki/iki_server.py at line 384.

Surface: example.

Signature: `def handle_exchange(self, manifest_bytes: bytes, signature: bytes, peer_cn: str) -> dict`

What it does: Function KMF.handle_exchange takes self, manifest_bytes: bytes, signature: bytes, peer_cn: str. It gives back dict.

(Where these words came from: inferred.)

---

## function — KMF.start

Found in examples/p25-iki/iki_server.py at line 443.

Surface: example.

Signature: `def start(self) -> None`

What it does: Function KMF.start takes no inputs beyond the value it is called on. It gives back nothing.

(Where these words came from: inferred.)

---

## function — Handler.log_message

Found in examples/p25-iki/iki_server.py at line 447.

Surface: example.

Signature: `def log_message(self, *args): # keep test output clean`

What it does: Function Handler.log_message takes self, *args. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Handler.do_GET

Found in examples/p25-iki/iki_server.py at line 450.

Surface: example.

Signature: `def do_GET(self)`

What it does: Function Handler.do_GET takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Handler.do_POST

Found in examples/p25-iki/iki_server.py at line 456.

Surface: example.

Signature: `def do_POST(self)`

What it does: Function Handler.do_POST takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — KMF.stop

Found in examples/p25-iki/iki_server.py at line 488.

Surface: example.

Signature: `def stop(self) -> None`

What it does: Function KMF.stop takes no inputs beyond the value it is called on. It gives back nothing.

(Where these words came from: inferred.)

---

## function — check

Found in examples/p25-iki/test_iki.py at line 32.

Surface: test.

Signature: `def check(label: str, ok: bool, extra: str = "")`

What it does: Function check takes label: str, ok: bool, extra: str = "". Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_slice_against_corpus

Found in examples/p25-iki/test_iki.py at line 49.

Surface: test.

Signature: `def test_slice_against_corpus()`

What it does: Function test_slice_against_corpus takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_keywrap

Found in examples/p25-iki/test_iki.py at line 66.

Surface: test.

Signature: `def test_keywrap()`

What it does: Function test_keywrap takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_hkdf_directions

Found in examples/p25-iki/test_iki.py at line 79.

Surface: test.

Signature: `def test_hkdf_directions()`

What it does: Function test_hkdf_directions takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_store_at_rest

Found in examples/p25-iki/test_iki.py at line 91.

Surface: test.

Signature: `def test_store_at_rest()`

What it does: Function test_store_at_rest takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_cms_sign_verify

Found in examples/p25-iki/test_iki.py at line 105.

Surface: test.

Signature: `def test_cms_sign_verify()`

What it does: Function test_cms_sign_verify takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_full_exchange

Found in examples/p25-iki/test_iki.py at line 118.

Surface: test.

Signature: `def test_full_exchange()`

What it does: Function test_full_exchange takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in examples/p25-iki/test_iki.py at line 126.

Surface: test.

Signature: `def main() -> int`

What it does: Function main takes no inputs beyond the value it is called on. It gives back int.

(Where these words came from: inferred.)

---

## function — CookieJar.__init__

Found in examples/web-browser/browser.py at line 58.

Surface: example.

Signature: `def __init__(self)`

What it does: Function CookieJar.__init__ takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — CookieJar.extract

Found in examples/web-browser/browser.py at line 61.

Surface: example.

Signature: `def extract(self, url, msg)`

What it does: Function CookieJar.extract takes self, url, msg. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — CookieJar.header_for

Found in examples/web-browser/browser.py at line 72.

Surface: example.

Signature: `def header_for(self, url)`

What it does: Function CookieJar.header_for takes self, url. Its return type is unannotated; the code shows it returning None; "; ".join(f"{k}={v}" for k, v in cookies.items().

(Where these words came from: inferred.)

---

## function — FetchResult.__init__

Found in examples/web-browser/browser.py at line 118.

Surface: example.

Signature: `def __init__(self, url, status, reason, headers, body, history)`

What it does: Function FetchResult.__init__ takes self, url, status, reason, headers, body, history. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FetchResult.content_type

Found in examples/web-browser/browser.py at line 127.

Surface: example.

Signature: `def content_type(self)`

What it does: Function FetchResult.content_type takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning self.headers.get_content_type() if self.headers .

(Where these words came from: inferred.)

---

## function — FetchResult.text

Found in examples/web-browser/browser.py at line 130.

Surface: example.

Signature: `def text(self)`

What it does: Decode the body: MIME charset parameter (RFC 2045) if given,

(Where these words came from: direct.)

---

## function — fetch

Found in examples/web-browser/browser.py at line 139.

Surface: example.

Signature: `def fetch(url, jar=None)`

What it does: GET with redirect following (RFC 9110 section 15.4). HTTP error

(Where these words came from: direct.)

---

## function — open_stream

Found in examples/web-browser/browser.py at line 164.

Surface: example.

Signature: `def open_stream(url, jar=None)`

What it does: Like fetch(), but returns (final_url, response, conn) with the body

(Where these words came from: direct.)

---

## function — Renderer.__init__

Found in examples/web-browser/browser.py at line 202.

Surface: example.

Signature: `def __init__(self, base_url)`

What it does: Function Renderer.__init__ takes self, base_url. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Renderer.handle_starttag

Found in examples/web-browser/browser.py at line 235.

Surface: example.

Signature: `def handle_starttag(self, tag, attrs)`

What it does: Function Renderer.handle_starttag takes self, tag, attrs. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Renderer.handle_endtag

Found in examples/web-browser/browser.py at line 273.

Surface: example.

Signature: `def handle_endtag(self, tag)`

What it does: Function Renderer.handle_endtag takes self, tag. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Renderer.handle_data

Found in examples/web-browser/browser.py at line 299.

Surface: example.

Signature: `def handle_data(self, data)`

What it does: Function Renderer.handle_data takes self, data. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Renderer.rendered_text

Found in examples/web-browser/browser.py at line 309.

Surface: example.

Signature: `def rendered_text(self)`

What it does: Function Renderer.rendered_text takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning "\n".join(self.lines).

(Where these words came from: inferred.)

---

## function — Page.__init__

Found in examples/web-browser/browser.py at line 315.

Surface: example.

Signature: `def __init__(self, result)`

What it does: Function Page.__init__ takes self, result. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — fetch_page

Found in examples/web-browser/browser.py at line 332.

Surface: example.

Signature: `def fetch_page(url, jar=None)`

What it does: Function fetch_page takes url, jar=None. Its return type is unannotated; the code shows it returning Page(fetch(url, jar=jar)).

(Where these words came from: inferred.)

---

## function — follow

Found in examples/web-browser/browser.py at line 336.

Surface: example.

Signature: `def follow(page, number)`

What it does: Follow link `number` (1-based, as printed) from a rendered page.

(Where these words came from: direct.)

---

## function — display

Found in examples/web-browser/browser.py at line 344.

Surface: example.

Signature: `def display(page, out=None)`

What it does: Function display takes page, out=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — coord_str

Found in examples/web-browser/browser.py at line 372.

Surface: example.

Signature: `def coord_str(state)`

What it does: Function coord_str takes state. Its return type is unannotated; the code shows it returning f"{s[0]}.{s[1]}.{s[2]}/{s[3]}.{s[4]}.{s[5]}/{s[6.

(Where these words came from: inferred.)

---

## function — PhextStreamParser.__init__

Found in examples/web-browser/browser.py at line 383.

Surface: example.

Signature: `def __init__(self)`

What it does: Function PhextStreamParser.__init__ takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — PhextStreamParser.feed

Found in examples/web-browser/browser.py at line 387.

Surface: example.

Signature: `def feed(self, data)`

What it does: Returns [(coordinate, scroll_bytes)] completed by this data.

(Where these words came from: direct.)

---

## function — PhextStreamParser.finish

Found in examples/web-browser/browser.py at line 402.

Surface: example.

Signature: `def finish(self)`

What it does: The final scroll: (coordinate, bytes) for whatever remains.

(Where these words came from: direct.)

---

## function — iter_phext_scrolls

Found in examples/web-browser/browser.py at line 407.

Surface: example.

Signature: `def iter_phext_scrolls(url, jar=None, chunk_size=4096)`

What it does: Stream a .phext from an HTTP/1.1 endpoint, yielding

(Where these words came from: direct.)

---

## function — phext_toc

Found in examples/web-browser/browser.py at line 433.

Surface: example.

Signature: `def phext_toc(url, jar=None)`

What it does: The table of contents, as it streams: [(coordinate, byte_count)].

(Where these words came from: direct.)

---

## function — phext_select

Found in examples/web-browser/browser.py at line 438.

Surface: example.

Signature: `def phext_select(url, wanted, jar=None)`

What it does: Stream until the scroll at `wanted` completes; return its bytes

(Where these words came from: direct.)

---

## function — run_phext

Found in examples/web-browser/browser.py at line 456.

Surface: example.

Signature: `def run_phext(url, select=None, jar=None)`

What it does: Function run_phext takes url, select=None, jar=None. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — main

Found in examples/web-browser/browser.py at line 496.

Surface: example.

Signature: `def main(argv)`

What it does: Function main takes argv. Its return type is unannotated; the code shows it returning 2; run_phext(url, select=select, jar=jar); 1; 0 if page.status < 400 else 1; _interactive(url, jar).

(Where these words came from: inferred.)

---

## function — check

Found in examples/web-browser/test_browser.py at line 32.

Surface: test.

Signature: `def check(label, condition, detail="")`

What it does: Function check takes label, condition, detail="". Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_slice

Found in examples/web-browser/test_browser.py at line 57.

Surface: test.

Signature: `def test_slice()`

What it does: Function test_slice takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FixtureHandler.log_message

Found in examples/web-browser/test_browser.py at line 110.

Surface: test.

Signature: `def log_message(self, *args)`

What it does: Function FixtureHandler.log_message takes self, *args. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — FixtureHandler.do_GET

Found in examples/web-browser/test_browser.py at line 123.

Surface: test.

Signature: `def do_GET(self)`

What it does: Function FixtureHandler.do_GET takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_http

Found in examples/web-browser/test_browser.py at line 140.

Surface: test.

Signature: `def test_http(base)`

What it does: Function test_http takes base. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — ChunkedPhextServer.__init__

Found in examples/web-browser/test_browser.py at line 227.

Surface: test.

Signature: `def __init__(self, payload, first_chunk_len, pause=0.5)`

What it does: Function ChunkedPhextServer.__init__ takes self, payload, first_chunk_len, pause=0.5. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — ChunkedPhextServer.run

Found in examples/web-browser/test_browser.py at line 240.

Surface: test.

Signature: `def run(self)`

What it does: Function ChunkedPhextServer.run takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — ChunkedPhextServer.send_chunk

Found in examples/web-browser/test_browser.py at line 266.

Surface: test.

Signature: `def send_chunk(data)`

What it does: Function ChunkedPhextServer.send_chunk takes data. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — ChunkedPhextServer.stop

Found in examples/web-browser/test_browser.py at line 281.

Surface: test.

Signature: `def stop(self)`

What it does: Function ChunkedPhextServer.stop takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_phext

Found in examples/web-browser/test_browser.py at line 289.

Surface: test.

Signature: `def test_phext(port, server)`

What it does: Function test_phext takes port, server. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — test_live

Found in examples/web-browser/test_browser.py at line 326.

Surface: test.

Signature: `def test_live()`

What it does: Function test_live takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in examples/web-browser/test_browser.py at line 337.

Surface: test.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/rfcbox/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

# rfcbox — reference as of commit 7ddac4f66c5f

Commit 7ddac4f66c5ff80df7a59857946cce0d78067d07. 19 commits folded. Every fact here was folded from the chain's per-commit deltas.

## Provenance

This reference is folded from the project's documentation chain, kept in the export repo wbic16/phdoc at docs/rfcbox/chain.phext, and rendered for the doc site https://docs.phext.io. Both targets are named in the project's own .phdoc.json; the defaults are wbic16/phdoc and https://docs.phext.io.

## Dependencies

No dependencies recorded at this point.

## Authors

The author of record is **Will Bickford** — 19 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Emise — 13 commits
- Theia — 4 commits
- Rogue — 1 commit
- Will Bickford (his own identities) — 1 commit

## Complexity

- Markdown: 15 files, 661 lines total, 210 blank, 0 comment, 451 code
- Python: 3 files, 746 lines total, 116 blank, 32 comment, 598 code
- Shell: 1 file, 14 lines total, 2 blank, 5 comment, 7 code

Declared RFCs at this point: none declared.

## Public methods and commands

- function `check` (examples/p25-iki/demo.py) [example] — Function check takes label: str, ok: bool.
- function `main` (examples/p25-iki/demo.py) [example] — Function main takes no inputs beyond the value it is called on.
- function `run_demo` (examples/p25-iki/demo.py) [example] — Function run_demo takes base: Path = RUN_DIR, verbose: bool = True.
- function `say` (examples/p25-iki/demo.py) [example] — Function say takes msg: str.
- function `AuditLog.__init__` (examples/p25-iki/iki_server.py) [example] — Function AuditLog.__init__ takes self, path: Path, actor: str.
- function `AuditLog.append` (examples/p25-iki/iki_server.py) [example] — Function AuditLog.append takes self, event: str, details: dict.
- function `AuditLog.verify_chain` (examples/p25-iki/iki_server.py) [example] — Function AuditLog.verify_chain takes path: Path.
- function `Handler.do_GET` (examples/p25-iki/iki_server.py) [example] — Function Handler.do_GET takes no inputs beyond the value it is called on.
- function `Handler.do_POST` (examples/p25-iki/iki_server.py) [example] — Function Handler.do_POST takes no inputs beyond the value it is called on.
- function `Handler.log_message` (examples/p25-iki/iki_server.py) [example] — Function Handler.log_message takes self, *args.
- function `KMF.__init__` (examples/p25-iki/iki_server.py) [example] — Function KMF.__init__ takes self, name: str, workdir: Path, ca: dict.
- function `KMF.create_key` (examples/p25-iki/iki_server.py) [example] — Function KMF.create_key takes self, key_id: int, alg_id: int = ALG_AES_256.
- function `KMF.export_keyset` (examples/p25-iki/iki_server.py) [example] — Wrap this KMF's keys for `peer`, sign the package (CMS), send it
- function `KMF.handle_exchange` (examples/p25-iki/iki_server.py) [example] — Function KMF.handle_exchange takes self, manifest_bytes: bytes, signature: bytes, peer_cn: str.
- function `KMF.provision_peer` (examples/p25-iki/iki_server.py) [example] — Function KMF.provision_peer takes self, peer_name: str, root_secret: bytes.
- function `KMF.start` (examples/p25-iki/iki_server.py) [example] — Function KMF.start takes no inputs beyond the value it is called on.
- function `KMF.stop` (examples/p25-iki/iki_server.py) [example] — Function KMF.stop takes no inputs beyond the value it is called on.
- function `KeyStore.__init__` (examples/p25-iki/iki_server.py) [example] — Function KeyStore.__init__ takes self, db_path: Path, master_key: bytes.
- function `KeyStore.directional_kek` (examples/p25-iki/iki_server.py) [example] — HKDF (RFC 5869): derive the src->dst wrap key from the pairwise root.
- function `KeyStore.get_key` (examples/p25-iki/iki_server.py) [example] — Function KeyStore.get_key takes self, owner: str, key_id: int, alg_id: int.
- function `KeyStore.get_receipt` (examples/p25-iki/iki_server.py) [example] — Function KeyStore.get_receipt takes self, exchange_id: str.
- function `KeyStore.list_keys` (examples/p25-iki/iki_server.py) [example] — Function KeyStore.list_keys takes self, owner: str.
- function `KeyStore.provision_peer` (examples/p25-iki/iki_server.py) [example] — Function KeyStore.provision_peer takes self, peer: str, root_secret: bytes.
- function `KeyStore.put_key` (examples/p25-iki/iki_server.py) [example] — Function KeyStore.put_key takes self, owner: str, key_id: int, alg_id: int, key: bytes, origin: str.
- function `KeyStore.put_receipt` (examples/p25-iki/iki_server.py) [example] — Function KeyStore.put_receipt takes self, exchange_id: str, receipt: dict, signature: bytes.
- function `b64d` (examples/p25-iki/iki_server.py) [example] — Function b64d takes s: str.
- function `b64e` (examples/p25-iki/iki_server.py) [example] — Function b64e takes b: bytes.
- function `canonical` (examples/p25-iki/iki_server.py) [example] — Deterministic JSON encoding used for everything that gets signed.
- function `cms_sign_detached` (examples/p25-iki/iki_server.py) [example] — Detached CMS SignedData (DER) over `data`, signed by the KMF identity.
- function `cms_verify_detached` (examples/p25-iki/iki_server.py) [example] — Verify a detached CMS signature against the test CA using OpenSSL.
- function `make_ca` (examples/p25-iki/iki_server.py) [example] — Create a self-signed test CA.
- function `make_kmf_identity` (examples/p25-iki/iki_server.py) [example] — Issue a KMF identity certificate (server + client + signing use).
- function `utcnow` (examples/p25-iki/iki_server.py) [example] — Function utcnow takes no inputs beyond the value it is called on.
- function `check` (examples/p25-iki/test_iki.py) [test] — Function check takes label: str, ok: bool, extra: str = "".
- function `main` (examples/p25-iki/test_iki.py) [test] — Function main takes no inputs beyond the value it is called on.
- function `test_cms_sign_verify` (examples/p25-iki/test_iki.py) [test] — Function test_cms_sign_verify takes no inputs beyond the value it is called on.
- function `test_full_exchange` (examples/p25-iki/test_iki.py) [test] — Function test_full_exchange takes no inputs beyond the value it is called on.
- function `test_hkdf_directions` (examples/p25-iki/test_iki.py) [test] — Function test_hkdf_directions takes no inputs beyond the value it is called on.
- function `test_keywrap` (examples/p25-iki/test_iki.py) [test] — Function test_keywrap takes no inputs beyond the value it is called on.
- function `test_slice_against_corpus` (examples/p25-iki/test_iki.py) [test] — Function test_slice_against_corpus takes no inputs beyond the value it is called on.
- function `test_store_at_rest` (examples/p25-iki/test_iki.py) [test] — Function test_store_at_rest takes no inputs beyond the value it is called on.

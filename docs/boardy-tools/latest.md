# boardy-tools — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 147). Scanned at commit 7092bd1ac39561fc1a809e7c9f60d683ec863520. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Summary

Prepared for Boardy's review, 2026-11-18. Assembled by Phext, Inc. (Will Bickford and Emise) from the exchange of that day: our pitch, his reply, his spec, and the work he asked for.

(The opening of the project's README, quoted as written.)

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 3 commits.

The author of record is **Will Bickford** — 3 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Will Bickford (his own identities) — 2 commits
- Emise — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- Markdown: 14 files, 1187 lines total, 294 blank, 10 comment, 883 code
- Python: 6 files, 1000 lines total, 143 blank, 46 comment, 811 code

All languages: 20 files, 2187 lines total, 1694 lines of code. Functions read: 44.

Largest files:

- 05-boot-manual.md — 338 lines
- receipts/boardy.py — 332 lines
- receipts/PLAYTEST.md — 288 lines
- receipts/benchmark.py — 224 lines
- receipts/boardy_mcp.py — 199 lines

Largest functions, by span in lines:

- main in receipts/benchmark.py — about 92 lines
- score_recovery in receipts/boardy.py — about 65 lines
- tool in receipts/test_client.py — about 63 lines
- boot in receipts/boardy.py — about 44 lines
- generate_candidates in receipts/boardy.py — about 41 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 2 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format: no lint or format configuration file was found.
- TODO marks in the source: 0. FIXME marks: 0.
- Public items with a doc comment: 8 of 44 (18%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/boardy-tools
- Branch: exo
- Head commit: 7092bd1ac39561fc1a809e7c9f60d683ec863520
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `node_record` (receipts/benchmark.py:112) — Function node_record takes nid.
- function `build_corpus` (receipts/benchmark.py:120) — Function build_corpus takes no inputs beyond the value it is called on.
- function `main` (receipts/benchmark.py:133) — Function main takes no inputs beyond the value it is called on.
- function `Graph.__init__` (receipts/boardy.py:55) — Function Graph.__init__ takes no inputs beyond the value it is called on.
- function `Graph.add_node` (receipts/boardy.py:60) — Function Graph.add_node takes self, nid, kind, attrs, coord=None.
- function `Graph.add_edge` (receipts/boardy.py:63) — Function Graph.add_edge takes self, src, typ, dst, weight.
- function `Graph.kind` (receipts/boardy.py:68) — Function Graph.kind takes self, nid.
- function `Graph.edges_from` (receipts/boardy.py:72) — Function Graph.edges_from takes self, src, typ=None.
- function `Graph.edges_to` (receipts/boardy.py:75) — Function Graph.edges_to takes self, dst, typ=None.
- function `Graph.edge_set` (receipts/boardy.py:78) — Function Graph.edge_set takes no inputs beyond the value it is called on.
- function `parse_node_record` (receipts/boardy.py:92) — Function parse_node_record takes text.
- function `boot` (receipts/boardy.py:114) — scrolls: {coord: text} merged from the problem+solution phexts.
- function `dense_counts` (receipts/boardy.py:158) — Function dense_counts takes g.
- function `empty_family_edges` (receipts/boardy.py:166) — Every boot edge landing in an empty kind-pair family (manual step 7).
- function `forbidden_nodes` (receipts/boardy.py:176) — Function forbidden_nodes takes g.
- function `score_recovery` (receipts/boardy.py:182) — gt_edges: list of (src, type, dst, weight) ground truth.
- function `find_route` (receipts/boardy.py:247) — Replayable route over stored edges only (traversed in either direction,
- function `invention_score` (receipts/boardy.py:275) — decisions: list of dicts with classification in {R, A, F}.
- function `generate_candidates` (receipts/boardy.py:292) — Function generate_candidates takes g.
- function `fmt` (receipts/boardy_mcp.py:37) — Function fmt takes coord.
- function `parse_phext` (receipts/boardy_mcp.py:41) — Explode phext text into {coordinate_tuple: scroll_text}.
- function `parse_coordinate` (receipts/boardy_mcp.py:64) — Function parse_coordinate takes text.
- function `tool_load` (receipts/boardy_mcp.py:74) — Function tool_load takes args.
- function `require_doc` (receipts/boardy_mcp.py:84) — Function require_doc takes no inputs beyond the value it is called on.
- function `tool_toc` (receipts/boardy_mcp.py:89) — Function tool_toc takes _args.
- function `tool_fetch` (receipts/boardy_mcp.py:98) — Function tool_fetch takes args.
- function `tool_search` (receipts/boardy_mcp.py:106) — Function tool_search takes args.
- function `send` (receipts/boardy_mcp.py:140) — Function send takes msg.
- function `result` (receipts/boardy_mcp.py:145) — Function result takes rid, value.
- function `error` (receipts/boardy_mcp.py:149) — Function error takes rid, code, message.
- function `handle` (receipts/boardy_mcp.py:153) — Function handle takes req.
- function `main` (receipts/boardy_mcp.py:187) — Function main takes no inputs beyond the value it is called on.
- function `fmt` (receipts/phext.py:15) — Function fmt takes coord.
- function `serialize` (receipts/phext.py:19) — scrolls: dict {coord9 tuple: str} -> bytes, coordinates ascending.
- function `parse` (receipts/phext.py:39) — bytes -> dict {coord9 tuple: str} (every scroll, including empty ones).
- function `common_prefix` (receipts/phext.py:58) — Function common_prefix takes a, b.
- function `log` (receipts/test_client.py:25) [test] — Function log takes direction, obj.
- function `check` (receipts/test_client.py:29) [test] — Function check takes label, ok, detail="".
- function `main` (receipts/test_client.py:35) [test] — Function main takes no inputs beyond the value it is called on.
- function `call` (receipts/test_client.py:41) [test] — Function call takes method, params=None.
- function `notify` (receipts/test_client.py:53) [test] — Function notify takes method.
- function `tool` (receipts/test_client.py:59) [test] — Function tool takes name, args.
- function `fake_boot` (receipts/test_scorer.py:14) [test] — Function fake_boot takes edges.
- function `main` (receipts/test_scorer.py:22) [test] — Function main takes no inputs beyond the value it is called on.

---

## function — node_record

Found in receipts/benchmark.py at line 112.

Surface: public API.

Signature: `def node_record(nid)`

What it does: Function node_record takes nid. Its return type is unannotated; the code shows it returning "\n".join(lines) + "\n".

(Where these words came from: inferred.)

---

## function — build_corpus

Found in receipts/benchmark.py at line 120.

Surface: public API.

Signature: `def build_corpus()`

What it does: Function build_corpus takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning problem, solution.

(Where these words came from: inferred.)

---

## function — main

Found in receipts/benchmark.py at line 133.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Graph.__init__

Found in receipts/boardy.py at line 55.

Surface: public API.

Signature: `def __init__(self)`

What it does: Function Graph.__init__ takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Graph.add_node

Found in receipts/boardy.py at line 60.

Surface: public API.

Signature: `def add_node(self, nid, kind, attrs, coord=None)`

What it does: Function Graph.add_node takes self, nid, kind, attrs, coord=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Graph.add_edge

Found in receipts/boardy.py at line 63.

Surface: public API.

Signature: `def add_edge(self, src, typ, dst, weight)`

What it does: Function Graph.add_edge takes self, src, typ, dst, weight. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Graph.kind

Found in receipts/boardy.py at line 68.

Surface: public API.

Signature: `def kind(self, nid)`

What it does: Function Graph.kind takes self, nid. Its return type is unannotated; the code shows it returning n["kind"] if n else None.

(Where these words came from: inferred.)

---

## function — Graph.edges_from

Found in receipts/boardy.py at line 72.

Surface: public API.

Signature: `def edges_from(self, src, typ=None)`

What it does: Function Graph.edges_from takes self, src, typ=None. Its return type is unannotated; the code shows it returning [e for e in self.edges if e[0] == src and (typ i.

(Where these words came from: inferred.)

---

## function — Graph.edges_to

Found in receipts/boardy.py at line 75.

Surface: public API.

Signature: `def edges_to(self, dst, typ=None)`

What it does: Function Graph.edges_to takes self, dst, typ=None. Its return type is unannotated; the code shows it returning [e for e in self.edges if e[2] == dst and (typ i.

(Where these words came from: inferred.)

---

## function — Graph.edge_set

Found in receipts/boardy.py at line 78.

Surface: public API.

Signature: `def edge_set(self)`

What it does: Function Graph.edge_set takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning set(self.edges).

(Where these words came from: inferred.)

---

## function — parse_node_record

Found in receipts/boardy.py at line 92.

Surface: public API.

Signature: `def parse_node_record(text)`

What it does: Function parse_node_record takes text. Its return type is unannotated; the code shows it returning rec.

(Where these words came from: inferred.)

---

## function — boot

Found in receipts/boardy.py at line 114.

Surface: public API.

Signature: `def boot(scrolls)`

What it does: scrolls: {coord: text} merged from the problem+solution phexts.

(Where these words came from: direct.)

---

## function — dense_counts

Found in receipts/boardy.py at line 158.

Surface: public API.

Signature: `def dense_counts(g)`

What it does: Function dense_counts takes g. Its return type is unannotated; the code shows it returning dict(counts).

(Where these words came from: inferred.)

---

## function — empty_family_edges

Found in receipts/boardy.py at line 166.

Surface: public API.

Signature: `def empty_family_edges(g)`

What it does: Every boot edge landing in an empty kind-pair family (manual step 7).

(Where these words came from: direct.)

---

## function — forbidden_nodes

Found in receipts/boardy.py at line 176.

Surface: public API.

Signature: `def forbidden_nodes(g)`

What it does: Function forbidden_nodes takes g. Its return type is unannotated; the code shows it returning [nid for nid, n in g.nodes.items() if n["kind"] .

(Where these words came from: inferred.)

---

## function — score_recovery

Found in receipts/boardy.py at line 182.

Surface: public API.

Signature: `def score_recovery(boot_g, gt_edges)`

What it does: gt_edges: list of (src, type, dst, weight) ground truth.

(Where these words came from: direct.)

---

## function — find_route

Found in receipts/boardy.py at line 247.

Surface: public API.

Signature: `def find_route(g, src, dst)`

What it does: Replayable route over stored edges only (traversed in either direction,

(Where these words came from: direct.)

---

## function — invention_score

Found in receipts/boardy.py at line 275.

Surface: public API.

Signature: `def invention_score(decisions)`

What it does: decisions: list of dicts with classification in {R, A, F}.

(Where these words came from: direct.)

---

## function — generate_candidates

Found in receipts/boardy.py at line 292.

Surface: public API.

Signature: `def generate_candidates(g)`

What it does: Function generate_candidates takes g. Its return type is unannotated; the code shows it returning candidates.

(Where these words came from: inferred.)

---

## function — fmt

Found in receipts/boardy_mcp.py at line 37.

Surface: public API.

Signature: `def fmt(coord)`

What it does: Function fmt takes coord. Its return type is unannotated; the code shows it returning "%d.%d.%d/%d.%d.%d/%d.%d.%d" % tuple(coord).

(Where these words came from: inferred.)

---

## function — parse_phext

Found in receipts/boardy_mcp.py at line 41.

Surface: public API.

Signature: `def parse_phext(text)`

What it does: Explode phext text into {coordinate_tuple: scroll_text}.

(Where these words came from: direct.)

---

## function — parse_coordinate

Found in receipts/boardy_mcp.py at line 64.

Surface: public API.

Signature: `def parse_coordinate(text)`

What it does: Function parse_coordinate takes text. Its return type is unannotated; the code shows it returning coord.

(Where these words came from: inferred.)

---

## function — tool_load

Found in receipts/boardy_mcp.py at line 74.

Surface: public API.

Signature: `def tool_load(args)`

What it does: Function tool_load takes args. Its return type is unannotated; the code shows it returning "loaded %s: %d scrolls, %d bytes" % (path, len(D.

(Where these words came from: inferred.)

---

## function — require_doc

Found in receipts/boardy_mcp.py at line 84.

Surface: public API.

Signature: `def require_doc()`

What it does: Function require_doc takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — tool_toc

Found in receipts/boardy_mcp.py at line 89.

Surface: public API.

Signature: `def tool_toc(_args)`

What it does: Function tool_toc takes _args. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — tool_fetch

Found in receipts/boardy_mcp.py at line 98.

Surface: public API.

Signature: `def tool_fetch(args)`

What it does: Function tool_fetch takes args. Its return type is unannotated; the code shows it returning DOC["scrolls"][coord].

(Where these words came from: inferred.)

---

## function — tool_search

Found in receipts/boardy_mcp.py at line 106.

Surface: public API.

Signature: `def tool_search(args)`

What it does: Function tool_search takes args. Its return type is unannotated; the code shows it returning "no scrolls contain %r" % args["text"]; "%d scroll(s) contain %r:\n%s" % (len(hits), arg.

(Where these words came from: inferred.)

---

## function — send

Found in receipts/boardy_mcp.py at line 140.

Surface: public API.

Signature: `def send(msg)`

What it does: Function send takes msg. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — result

Found in receipts/boardy_mcp.py at line 145.

Surface: public API.

Signature: `def result(rid, value)`

What it does: Function result takes rid, value. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — error

Found in receipts/boardy_mcp.py at line 149.

Surface: public API.

Signature: `def error(rid, code, message)`

What it does: Function error takes rid, code, message. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — handle

Found in receipts/boardy_mcp.py at line 153.

Surface: public API.

Signature: `def handle(req)`

What it does: Function handle takes req. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in receipts/boardy_mcp.py at line 187.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — fmt

Found in receipts/phext.py at line 15.

Surface: public API.

Signature: `def fmt(coord)`

What it does: Function fmt takes coord. Its return type is unannotated; the code shows it returning "%d.%d.%d/%d.%d.%d/%d.%d.%d" % tuple(coord).

(Where these words came from: inferred.)

---

## function — serialize

Found in receipts/phext.py at line 19.

Surface: public API.

Signature: `def serialize(scrolls)`

What it does: scrolls: dict {coord9 tuple: str} -> bytes, coordinates ascending.

(Where these words came from: direct.)

---

## function — parse

Found in receipts/phext.py at line 39.

Surface: public API.

Signature: `def parse(data)`

What it does: bytes -> dict {coord9 tuple: str} (every scroll, including empty ones).

(Where these words came from: direct.)

---

## function — common_prefix

Found in receipts/phext.py at line 58.

Surface: public API.

Signature: `def common_prefix(a, b)`

What it does: Function common_prefix takes a, b. Its return type is unannotated; the code shows it returning n.

(Where these words came from: inferred.)

---

## function — log

Found in receipts/test_client.py at line 25.

Surface: test.

Signature: `def log(direction, obj)`

What it does: Function log takes direction, obj. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — check

Found in receipts/test_client.py at line 29.

Surface: test.

Signature: `def check(label, ok, detail="")`

What it does: Function check takes label, ok, detail="". Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in receipts/test_client.py at line 35.

Surface: test.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — call

Found in receipts/test_client.py at line 41.

Surface: test.

Signature: `def call(method, params=None)`

What it does: Function call takes method, params=None. Its return type is unannotated; the code shows it returning resp.

(Where these words came from: inferred.)

---

## function — notify

Found in receipts/test_client.py at line 53.

Surface: test.

Signature: `def notify(method)`

What it does: Function notify takes method. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — tool

Found in receipts/test_client.py at line 59.

Surface: test.

Signature: `def tool(name, args)`

What it does: Function tool takes name, args. Its return type is unannotated; the code shows it returning resp.get("result", {}).

(Where these words came from: inferred.)

---

## function — fake_boot

Found in receipts/test_scorer.py at line 14.

Surface: test.

Signature: `def fake_boot(edges)`

What it does: Function fake_boot takes edges. Its return type is unannotated; the code shows it returning g                # invented type (produces_ask) .

(Where these words came from: inferred.)

---

## function — main

Found in receipts/test_scorer.py at line 22.

Surface: test.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/boardy-tools/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.

# Summaries needed — phdoc instruction file

phdoc scanned phextron on 2026-11-22 (Exocortical Epoch 150) and could not write a summary for the items below from the code's doc comments or signatures alone. No words were invented for them.

## What to do

Have your agent read each item in its file, write one to three plain sentences that say what it does, what it takes, and what it gives back, and put them in a summaries file: one line per item, in the form `name :: the summary`. Then run phdoc again with `--summaries <that file>` and the summaries will be carried into the documentation, marked as supplied by the user's agent.

An LLM endpoint of your choice can also be set in phdoc's config to draft these, but that is never required and never on by default.

## Items that need a summary

- struct `DataPanel` — crates/phextron-dash/src/data.rs at line 525. Signature: `pub struct DataPanel;`
- struct `InklingPanel` — crates/phextron-dash/src/inkling.rs at line 449. Signature: `pub struct InklingPanel;`
- struct `JenkinsPanel` — crates/phextron-dash/src/jenkins.rs at line 350. Signature: `pub struct JenkinsPanel;`
- struct `NodesPanel` — crates/phextron-dash/src/nodes.rs at line 85. Signature: `pub struct NodesPanel;`
- trait `Panel` — crates/phextron-dash/src/panels.rs at line 63. Signature: `pub trait Panel: Send + Sync {`
- struct `PendingPanel` — crates/phextron-dash/src/pending.rs at line 12. Signature: `pub struct PendingPanel {`
- struct `PhextsPanel` — crates/phextron-dash/src/phexts.rs at line 275. Signature: `pub struct PhextsPanel;`
- struct `SubstratesPanel` — crates/phextron-dash/src/substrates.rs at line 693. Signature: `pub struct SubstratesPanel;`
- struct `VideoPanel` — crates/phextron-dash/src/video.rs at line 427. Signature: `pub struct VideoPanel;`
- struct `WauPanel` — crates/phextron-dash/src/wau.rs at line 269. Signature: `pub struct WauPanel;`
- struct `WiringPanel` — crates/phextron-dash/src/wiring.rs at line 531. Signature: `pub struct WiringPanel;`

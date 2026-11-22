# Summaries needed — phdoc instruction file

phdoc scanned SQ on 2026-11-22 (Exocortical Epoch 150) and could not write a summary for the items below from the code's doc comments or signatures alone. No words were invented for them.

## What to do

Have your agent read each item in its file, write one to three plain sentences that say what it does, what it takes, and what it gives back, and put them in a summaries file: one line per item, in the form `name :: the summary`. Then run phdoc again with `--summaries <that file>` and the summaries will be carried into the documentation, marked as supplied by the user's agent.

An LLM endpoint of your choice can also be set in phdoc's config to draft these, but that is never required and never on by default.

## Items that need a summary

- enum `TocMode` — src/sq.rs at line 495. Signature: `pub enum TocMode { Full, Children, Summary, Scrolls, Matches }`

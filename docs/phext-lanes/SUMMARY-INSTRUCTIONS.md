# Summaries needed — phdoc instruction file

phdoc scanned phext-lanes on 2026-11-22 (Exocortical Epoch 150) and could not write a summary for the items below from the code's doc comments or signatures alone. No words were invented for them.

## What to do

Have your agent read each item in its file, write one to three plain sentences that say what it does, what it takes, and what it gives back, and put them in a summaries file: one line per item, in the form `name :: the summary`. Then run phdoc again with `--summaries <that file>` and the summaries will be carried into the documentation, marked as supplied by the user's agent.

An LLM endpoint of your choice can also be set in phdoc's config to draft these, but that is never required and never on by default.

## Items that need a summary

- struct `Cache` — src/cache.rs at line 23. Signature: `pub struct Cache {`
- struct `Hub` — src/fabric.rs at line 301. Signature: `pub struct Hub {`
- struct `Owners` — src/invite.rs at line 27. Signature: `pub struct Owners {`
- struct `Limiter` — src/limit.rs at line 24. Signature: `pub struct Limiter {`
- trait `IdentitySource` — src/oauth.rs at line 35. Signature: `pub trait IdentitySource: Send + Sync {`
- struct `Oauth` — src/oauth.rs at line 53. Signature: `pub struct Oauth {`
- struct `AppState` — src/server.rs at line 36. Signature: `pub struct AppState {`
- struct `Store` — src/store.rs at line 199. Signature: `pub struct Store {`

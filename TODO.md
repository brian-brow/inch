# TODO

## Wanted

- [ ] Stop the ticker with a channel (`recv_timeout`) so `complete()` doesn't wait up to 80ms on a sleeping thread.
- [ ] Move the percent/fill math into a pure function and test `total == 0`, `progress == total` and a partial value.
- [ ] Add `AGENTS.md`: learning project, explain and review, don't write the code.

## Nice to have

- [ ] Remove the tracked `log.txt`. It's output from an old run.
- [ ] Guard `progress * width` and `progress * 100` against overflow on huge totals.
- [ ] Take the lock once in `Bar::start` instead of twice.
- [ ] Drive `inch` from Python and TypeScript (stdin command protocol or native bindings).

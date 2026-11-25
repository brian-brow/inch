# TODO

## Wanted

- [x] Stop the ticker with a channel (`recv_timeout`) so `complete()` doesn't wait up to 80ms on a sleeping thread.
- [x] Move the percent/fill math into a pure function and test `total == 0`, `progress == total` and a partial value.

## Nice to have

- [x] Remove the tracked `log.txt`. It's output from an old run.
- [x] Guard `progress * width` and `progress * 100` against overflow on huge totals.
- [x] Take the lock once in `Bar::start` instead of twice.
- [ ] Drive `inch` from Python and TypeScript (stdin command protocol or native bindings).

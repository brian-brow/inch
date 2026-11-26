# TODO

## Wanted

- [x] Stop the ticker with a channel (`recv_timeout`) so `complete()` doesn't wait up to 80ms on a sleeping thread.
- [x] Move the percent/fill math into a pure function and test `total == 0`, `progress == total` and a partial value.
- [x] Add `Bar::inc()` to bump progress by 1, so loops don't have to track their own counter.
- [x] Brainstorm more features.
- [x] Add `Bar::fail(reason)`: a red `✗ ... failed after 1.2s` with the reason and full log, instead of dropping the bar to fail.
- [x] Add `Bar::set_title()` to change the title while a task runs.
- [x] Add an iterator adapter, `bar.wrap(title, iter)`, that calls `inc()` per item and completes at the end.
- [ ] Show rate and ETA (`1.2k/s · 4s left`), smoothed so it doesn't jump around.
- [x] Tune the worm: drive its gait from progress instead of time, so it crawls instead of sliding.

## Nice to have

- [x] Remove the tracked `log.txt`. It's output from an old run.
- [x] Guard `progress * width` and `progress * 100` against overflow on huge totals.
- [x] Take the lock once in `Bar::start` instead of twice.
- [ ] Drive `inch` from Python and TypeScript (stdin command protocol or native bindings).

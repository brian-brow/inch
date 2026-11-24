# inch

A small terminal progress bar for Rust. Standard library only.

```
✓ installing your mom... 2.5s
⠹ downloading rocket league... 1.2s
├─ going up
╰─ going down
[████████████········] 60%
```

## Ideas

- **Stderr only.** Stdout stays free for real output.
- **Always moving.** A background thread redraws every 80ms, so the spinner and timer move even when progress stalls.
- **Finished work collapses.** `complete()` replaces the bar and its logs with one `✓` line, so a run leaves a clean list of what was done and how long it took.
- **Logs belong to their task.** `println()` adds a line to the running task's log. The last 10 show under the title while it runs.
- **Failure keeps its context.** A task that never completes, because the bar is dropped or `start` is called again, leaves a red `✗ ... cancelled after 1.2s` with its full log underneath.
- **Pipes keep everything.** When stderr is not a terminal, nothing animates. Logs print as they arrive and each task ends with one plain `✓` or `✗` line.

## Usage

```rust
use inch::Bar;

let mut bar = Bar::new().with_width(30).with_fill('#').with_empty('-');

bar.start("downloading...", 100);
for i in 0..=100 {
    bar.update(i);
    if i == 50 {
        bar.println("halfway");
    }
}
bar.complete();
```

`cargo run` runs the demo in `src/main.rs`.

## Does not

- Run more than one bar at a time.
- Handle titles or log lines wider than the terminal. They wrap and break the redraw.
- Protect against output that skips `println()`. Anything else printed to the terminal while a bar is live corrupts the display.

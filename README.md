# inch

A small terminal progress bar for Rust. Standard library only.

```
✓ installing your mom... 2.5s
⠹ downloading rocket league... 1.2s
[████████████········] 60%
```

## Ideas

- **Stderr only.** Stdout stays free for real output.
- **Always moving.** A background thread redraws every 80ms, so the spinner and timer move even when progress stalls.
- **Finished work collapses.** `complete()` replaces the bar with one `✓` line, so a run leaves a clean list of what was done and how long it took.
- **Failure is visible.** A task that never completes, because the bar is dropped or `start` is called again, leaves a red `✗ ... cancelled after 1.2s`.
- **Pipes stay clean.** When stderr is not a terminal, nothing animates and each task prints one plain `✓` or `✗` line.

## Usage

```rust
use inch::Bar;

let mut bar = Bar::new().with_width(30).with_fill('#').with_empty('-');

bar.start("downloading...", 100);
for i in 0..=100 {
    bar.update(i);
}
bar.complete();
```

`cargo run` runs the demo in `src/main.rs`.

## Does not

- Run more than one bar at a time.
- Handle titles wider than the terminal. They wrap and break the redraw.
- Protect against other output. Anything printed to the terminal while a bar is live corrupts the display.

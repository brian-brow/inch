# inch

A small terminal progress bar for Rust. Standard library only.

```
✓ installing money... 2.5s
⠹ downloading rocket league... 1.2s
├─ going up
╰─ going down
[████████████········] 60%
```

## Ideas

- **Stderr only.** Stdout stays free for real output.
- **Always moving.** A background thread redraws at least every 80ms, so the spinner, timer and worm move even when progress stalls.
- **Finished work collapses.** `complete()` replaces the bar and its logs with one `✓` line, so a run leaves a clean list of what was done and how long it took.
- **Logs belong to their task.** `println()` adds a line to the running task's log. The last 10 show under the title while it runs.
- **Failure keeps its context.** `fail(reason)` leaves a red `✗ ... failed after 1.2s` with its full log underneath and the reason as the last line. A task that never finishes, because the bar is dropped or `start` is called again, leaves `✗ ... cancelled after 1.2s` the same way.
- **Pipes keep everything.** When stderr is not a terminal, nothing animates. Logs print as they arrive and each task ends with one plain `✓` or `✗` line.

## Usage

```rust
use inch::Bar;

let mut bar = Bar::new().with_width(30).with_fill('#').with_empty('-');

bar.start("downloading...", 100);
for i in 0..100 {
    bar.inc();
    if i == 50 {
        bar.println("halfway");
    }
}
bar.complete();
```

`wrap(title, iter)` runs a task over any iterator with a known length. It adds 1 per item and completes when the loop ends:

```rust
for file in bar.wrap("copying...", files) {
    copy(file);
    bar.println("copied");
}
```

`inc()` adds 1 to progress. `update(n)` sets it directly. Both stop at the task's total. `set_title()` renames the running task, and `fail(reason)` ends it as failed instead of `complete()`.

`with_spinner(inch::spinners::LINE)` picks one of the 50 single-character spinners from [cli-spinners](https://github.com/sindresorhus/cli-spinners). The default is `DOTS`.

`with_style(inch::Style::Worm)` swaps the filled bar for an inchworm that crawls along it. Its gait is driven by progress, not time: every cell of progress is one full step, so it crawls as fast as the work goes and rests when the work stalls. The fill character becomes the trail it leaves behind, and the empty character is the track ahead:

```
[        ▁▃▁··········] 42%
```

`cargo run` runs the demo in `src/main.rs`: one Rocket League session that installs, updates, fails to find a match, plays one with the worm, and gets cancelled mid rage-quit. `cargo run --example spinners [name]` previews the spinners.

## Does not

- Run more than one bar at a time.
- Truncate long lines neatly. Titles and logs wider than the terminal are cut off at the edge, with no `…`.
- Survive resizing the terminal while a bar is live. The redraw leaves a stray copy behind.
- Protect against output that skips `println()`. Anything else printed to the terminal while a bar is live corrupts the display.

## License

MIT, see [LICENSE](LICENSE). The spinners in `src/spinners.rs` come from [cli-spinners](https://github.com/sindresorhus/cli-spinners), also MIT, with its notice at the top of that file.

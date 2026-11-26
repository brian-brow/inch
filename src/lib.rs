use std::fmt::Write as _;
use std::io::{IsTerminal, Write, stderr};
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

pub mod spinners;

use spinners::Spinner;

/// Starts an ANSI escape sequence ("Control Sequence Introducer").
const CSI: &str = "\x1b[";

/// Bar width used when the caller doesn't set one.
const DEFAULT_WIDTH: usize = 20;
/// Fill character used when the caller doesn't set one.
const DEFAULT_FILL: char = '█';
/// Empty character used when the caller doesn't set one.
const DEFAULT_EMPTY: char = '·';

/// Spinner used when the caller doesn't set one.
const DEFAULT_SPINNER: Spinner = spinners::DOTS;
/// Slowest the bar redraws, so the timer keeps moving under slow spinners.
const MAX_TICK: Duration = Duration::from_millis(80);
/// Most log lines shown under a running task.
const LOG_LIMIT: usize = 10;

/// The worm's gait, one body shape per frame, each ending at the head (last char).
/// Stretched, tail creeps in, arches up, bunches, then reaches back out to stretched.
/// Each cell of progress plays every frame once, so more frames make a smoother step,
/// not a slower one.
const WORM: &[&str] = &["▁▁▁▁", "▁▁▂▁", "▁▄▁", "▇▁", "▁▄▁", "▁▁▂▁"];

/// How the bar between the brackets is drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Style {
    /// Filled blocks up to the progress, empty after.
    #[default]
    Blocks,
    /// An inchworm crawling along the bar, leaving the fill behind it.
    /// Its gait follows progress, so it crawls as work gets done and rests when it stalls.
    Worm,
}

/// The redraw thread and the sender that stops it
struct Ticker {
    /// Dropping this sender disconnects the channel, which stops the ticker.
    stop: Sender<()>,
    handle: JoinHandle<()>,
}

/// A progress bar drawn to stderr, reused across tasks.
pub struct Bar {
    state: Arc<Mutex<State>>,
    ticker: Mutex<Option<Ticker>>,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(State::new())),
            ticker: Mutex::new(None),
        }
    }

    pub fn with_width(self, width: usize) -> Self {
        self.state.lock().unwrap().width = width;
        self
    }

    pub fn with_fill(self, fill_char: char) -> Self {
        self.state.lock().unwrap().fill_char = fill_char;
        self
    }

    pub fn with_empty(self, empty_char: char) -> Self {
        self.state.lock().unwrap().empty_char = empty_char;
        self
    }

    pub fn with_spinner(self, spinner: Spinner) -> Self {
        self.state.lock().unwrap().spinner = spinner;
        self
    }

    /// Picks how the bar is drawn. See [`Style`].
    pub fn with_style(self, style: Style) -> Self {
        self.state.lock().unwrap().style = style;
        self
    }

    pub fn wrap<I>(&self, title: &str, iter: I) -> Wrap<'_, I::IntoIter>
    where
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator,
    {
        let inner = iter.into_iter();
        self.start(title, inner.len());
        Wrap { bar: self, inner }
    }

    /// Begins a new task, cancelling the current one if it never completed.
    pub fn start(&self, title: &str, total: usize) {
        self.stop_ticker();

        let tick = {
            let mut state = self.state.lock().unwrap();
            state.start(title, total);
            if !state.is_tty {
                return;
            }
            state.spinner.interval.min(MAX_TICK)
        };

        let (tx, rx) = mpsc::channel::<()>();
        let state = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            loop {
                state.lock().unwrap().draw();
                match rx.recv_timeout(tick) {
                    Err(RecvTimeoutError::Timeout) => {}
                    _ => break,
                }
            }
        });
        *self.ticker.lock().unwrap() = Some(Ticker { stop: tx, handle });
    }

    /// Sets progress, clamped to the task's total.
    pub fn update(&self, progress: usize) {
        self.state.lock().unwrap().update(progress);
    }

    /// Adds 1 to progress, stopping at the task's total.
    pub fn inc(&self) {
        self.state.lock().unwrap().inc();
    }

    /// Renames the running task. The next redraw and the final line use the new title.
    pub fn set_title(&self, title: &str) {
        self.state.lock().unwrap().set_title(title);
    }

    /// Adds a line to the running task's log, or prints it directly when no bar is drawn.
    pub fn println(&self, message: &str) {
        if let Ok(mut state) = self.state.lock()
            && state.is_tty
            && let Some(task) = &mut state.task
        {
            task.logs.push(message.to_string());
        } else {
            let _ = writeln!(stderr(), "{message}");
        }
    }

    /// Replaces the bar and its logs with a single ✓ line.
    pub fn complete(&self) {
        self.stop_ticker();
        self.state.lock().unwrap().complete();
    }

    /// Ends the task with a red ✗, its full log, and `reason` as the last line.
    pub fn fail(&self, reason: &str) {
        self.stop_ticker();
        self.state.lock().unwrap().fail(reason);
    }

    fn stop_ticker(&self) {
        let ticker = self.ticker.lock().unwrap().take();
        if let Some(Ticker { stop, handle }) = ticker {
            drop(stop);
            let _ = handle.join();
        }
    }
}

impl Default for Bar {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Bar {
    fn drop(&mut self) {
        self.stop_ticker();
        if let Ok(mut state) = self.state.lock() {
            state.cancel();
        }
    }
}

/// Display config plus the running task, shared with the ticker thread.
struct State {
    width: usize,
    fill_char: char,
    empty_char: char,
    spinner: Spinner,
    style: Style,
    is_tty: bool,
    task: Option<Task>,
}

impl State {
    fn new() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            fill_char: DEFAULT_FILL,
            empty_char: DEFAULT_EMPTY,
            spinner: DEFAULT_SPINNER,
            style: Style::Blocks,
            is_tty: stderr().is_terminal(),
            task: None,
        }
    }

    fn start(&mut self, title: &str, total: usize) {
        self.cancel();
        self.task = Some(Task::new(title, total));
        self.draw();
    }

    fn update(&mut self, progress: usize) {
        if let Some(task) = &mut self.task {
            task.update(progress);
        }
    }

    fn inc(&mut self) {
        if let Some(task) = &mut self.task {
            task.inc();
        }
    }

    fn set_title(&mut self, title: &str) {
        if let Some(task) = &mut self.task {
            task.title = title.to_string();
        }
    }

    fn bar(&self, task: &Task) -> String {
        if self.width == 0 {
            return String::new();
        }
        match self.style {
            Style::Blocks => {
                let filled = scale(task.progress, task.total, self.width);

                let mut bar = String::new();
                bar.extend(std::iter::repeat_n(self.fill_char, filled));
                bar.extend(std::iter::repeat_n(self.empty_char, self.width - filled));
                bar
            }
            Style::Worm => {
                let steps = scale(task.progress, task.total, (self.width - 1) * WORM.len());
                let head = steps / WORM.len();
                let phase = steps % WORM.len();

                let frame: Vec<char> = WORM[phase].chars().collect();
                let visible = &frame[frame.len().saturating_sub(head + 1)..];
                let before = head + 1 - visible.len();
                let after = self.width - head - 1;

                let mut bar = String::new();
                bar.extend(std::iter::repeat_n(self.fill_char, before));
                bar.extend(visible);
                bar.extend(std::iter::repeat_n(self.empty_char, after));
                bar
            }
        }
    }

    fn draw(&mut self) {
        if !self.is_tty {
            return;
        }
        let Some(task) = &self.task else { return };
        let bar = self.bar(task);

        let Some(task) = &mut self.task else { return };
        let percentage = scale(task.progress, task.total, 100);
        let spinner = self.spinner.frame(task.started.elapsed());

        let mut frame = String::new();
        write!(frame, "{CSI}?7l").unwrap();
        if task.drawn > 0 {
            write!(frame, "{CSI}{}A", task.drawn).unwrap();
        }
        writeln!(
            frame,
            "\r{CSI}2K{spinner} {} {:.1?}",
            task.title,
            task.started.elapsed()
        )
        .unwrap();
        task.write_logs(&mut frame, LOG_LIMIT);
        writeln!(frame, "\r{CSI}2K[{bar}] {percentage}%").unwrap();
        writeln!(frame).unwrap();
        write!(frame, "{CSI}?7h").unwrap();

        let _ = write!(stderr(), "{frame}");
        task.drawn = frame.matches('\n').count();
        let _ = stderr().flush();
    }

    fn finish(&self, task: &Task, color: &str, icon: char, prefix: &str) {
        if self.is_tty {
            let _ = write!(stderr(), "{CSI}{}A", task.drawn);
            let _ = write!(stderr(), "\r{CSI}0J");
            let _ = writeln!(
                stderr(),
                "{CSI}{color}{icon}{CSI}0m {} {CSI}90m{prefix}{:.1?}{CSI}0m",
                task.title,
                task.started.elapsed()
            );
            let _ = stderr().flush();
        } else {
            let _ = writeln!(
                stderr(),
                "{icon} {} {prefix}{:.1?}",
                task.title,
                task.started.elapsed()
            );
        }
    }

    fn interrupt(&mut self, word: &str, reason: Option<&str>) {
        let Some(mut task) = self.task.take() else {
            return;
        };
        if let Some(reason) = reason {
            task.logs.push(reason.to_string());
        }
        self.finish(&task, "31m", '✗', &format!("{word} after "));
        if self.is_tty {
            let mut dump = String::new();
            task.write_logs(&mut dump, task.logs.len());
            let _ = write!(stderr(), "{dump}");
        } else if let Some(reason) = reason {
            let _ = writeln!(stderr(), "{reason}");
        }
    }

    fn cancel(&mut self) {
        self.interrupt("cancelled", None);
    }

    fn fail(&mut self, reason: &str) {
        self.interrupt("failed", Some(reason));
    }

    fn complete(&mut self) {
        let Some(task) = self.task.take() else { return };
        self.finish(&task, "32m", '✓', "");
    }
}

/// One run, from `start` to `complete` or cancel.
struct Task {
    title: String,
    total: usize,
    progress: usize,
    started: Instant,
    logs: Vec<String>,
    drawn: usize,
}

impl Task {
    fn new(title: &str, total: usize) -> Self {
        Self {
            title: title.to_string(),
            total,
            progress: 0,
            started: Instant::now(),
            logs: Vec::new(),
            drawn: 0,
        }
    }

    fn update(&mut self, progress: usize) {
        self.progress = progress.min(self.total);
    }

    fn inc(&mut self) {
        self.update(self.progress.saturating_add(1));
    }

    fn write_logs(&self, out: &mut String, limit: usize) {
        let shown = &self.logs[self.logs.len().saturating_sub(limit)..];
        for (i, log) in shown.iter().enumerate() {
            let prefix = if i == shown.len() - 1 {
                "╰─ "
            } else {
                "├─ "
            };
            writeln!(out, "\r{CSI}2K{CSI}2m{prefix}{log}{CSI}0m").unwrap();
        }
    }
}

/// An iterator that advances a bar one step per item and completes it when it runs out.
/// Made by [`Bar::wrap`].
pub struct Wrap<'a, I> {
    bar: &'a Bar,
    inner: I,
}

impl<'a, I: Iterator> Iterator for Wrap<'a, I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(item) => {
                self.bar.inc();
                Some(item)
            }
            None => {
                self.bar.complete();
                None
            }
        }
    }
}

/// Scales `progress` out of `total` onto `0..=max`, rounding down.
/// An empty task (`total == 0`) counts as done.
fn scale(progress: usize, total: usize, max: usize) -> usize {
    (progress as u128 * max as u128)
        .checked_div(total as u128)
        .unwrap_or(max as u128) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_task_is_full() {
        assert_eq!(scale(0, 0, 100), 100);
        assert_eq!(scale(0, 0, 20), 20);
    }

    #[test]
    fn finished_task_is_full() {
        assert_eq!(scale(10, 10, 100), 100);
        assert_eq!(scale(10, 10, 20), 20);
    }

    #[test]
    fn partial_task_scales() {
        assert_eq!(scale(3, 10, 100), 30);
        assert_eq!(scale(3, 10, 20), 6);
    }

    #[test]
    fn partial_task_rounds_down() {
        assert_eq!(scale(1, 3, 100), 33);
        assert_eq!(scale(1, 3, 20), 6);
    }

    #[test]
    fn huge_totals_dont_overflow() {
        assert_eq!(scale(usize::MAX, usize::MAX, 100), 100);
        assert_eq!(scale(usize::MAX / 2, usize::MAX, 100), 49);
    }

    #[test]
    fn inc_stops_at_total() {
        let mut task = Task::new("t", 2);
        task.inc();
        task.inc();
        task.inc();
        assert_eq!(task.progress, 2);
    }
}

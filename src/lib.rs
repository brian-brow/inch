use std::fmt::Write as _;
use std::io::{IsTerminal, Write, stderr};
use std::sync::atomic::{AtomicBool, Ordering};
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
const DEFAULT_EMPTY: char = '░';

/// Spinner used when the caller doesn't set one.
const DEFAULT_SPINNER: Spinner = spinners::DOTS;
/// Slowest the bar redraws, so the timer keeps moving under slow spinners.
const MAX_TICK: Duration = Duration::from_millis(80);
/// Most log lines shown under a running task.
const LOG_LIMIT: usize = 10;

/// A progress bar drawn to stderr, reused across tasks.
pub struct Bar {
    state: Arc<Mutex<State>>,
    stop: Arc<AtomicBool>,
    ticker: Option<JoinHandle<()>>,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(State::new())),
            stop: Arc::new(AtomicBool::new(false)),
            ticker: None,
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

    /// Begins a new task, cancelling the current one if it never completed.
    pub fn start(&mut self, title: &str, total: usize) {
        self.stop_ticker();
        self.state.lock().unwrap().start(title, total);
        if !self.state.lock().unwrap().is_tty {
            return;
        }
        self.stop.store(false, Ordering::Relaxed);
        let state = Arc::clone(&self.state);
        let stop = Arc::clone(&self.stop);
        let tick = self.state.lock().unwrap().spinner.interval.min(MAX_TICK);

        self.ticker = Some(thread::spawn(move || {
            while !stop.load(Ordering::Relaxed) {
                state.lock().unwrap().draw();
                thread::sleep(tick);
            }
        }));
    }

    /// Sets progress, clamped to the task's total.
    pub fn update(&self, progress: usize) {
        self.state.lock().unwrap().update(progress);
    }

    /// Adds a line to the running task's log, or prints it directly when no bar is drawn.
    pub fn println(&self, message: &str) {
        if let Ok(mut state) = self.state.lock()
            && state.is_tty
            && let Some(task) = &mut state.task
        {
            task.logs.push(message.to_string());
        } else {
            eprintln!("{message}");
        }
    }

    /// Replaces the bar and its logs with a single ✓ line.
    pub fn complete(&mut self) {
        self.stop_ticker();
        self.state.lock().unwrap().complete();
    }

    fn stop_ticker(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.ticker.take() {
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

    fn draw(&mut self) {
        if !self.is_tty {
            return;
        }
        let Some(task) = &mut self.task else { return };
        let percentage = (task.progress * 100).checked_div(task.total).unwrap_or(100);
        let filled = (task.progress * self.width)
            .checked_div(task.total)
            .unwrap_or(self.width);
        let fill = self.fill_char.to_string().repeat(filled);
        let empty = self.empty_char.to_string().repeat(self.width - filled);
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
        writeln!(frame, "\r{CSI}2K[{fill}{empty}] {percentage}%").unwrap();
        writeln!(frame).unwrap();
        write!(frame, "{CSI}?7h").unwrap();

        eprint!("{frame}");
        task.drawn = frame.matches('\n').count();
        stderr().flush().unwrap();
    }

    fn finish(&self, task: &Task, color: &str, icon: char, prefix: &str) {
        if self.is_tty {
            eprint!("{CSI}{}A", task.drawn);
            eprint!("\r{CSI}0J");
            eprintln!(
                "{CSI}{color}{icon}{CSI}0m {} {CSI}90m{prefix}{:.1?}{CSI}0m",
                task.title,
                task.started.elapsed()
            );
            let _ = stderr().flush();
        } else {
            eprintln!(
                "{icon} {} {prefix}{:.1?}",
                task.title,
                task.started.elapsed()
            );
        }
    }

    fn cancel(&mut self) {
        let Some(task) = self.task.take() else { return };
        self.finish(&task, "31m", '✗', "cancelled after ");
        if self.is_tty {
            let mut dump = String::new();
            task.write_logs(&mut dump, task.logs.len());
            eprint!("{dump}");
        }
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

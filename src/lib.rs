use std::{
    io::{IsTerminal, Write},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

/// Starts an ANSI escape sequence ("Control Sequence Introducer").
const CSI: &str = "\x1b[";
/// How many terminal lines one draw of the bar takes up.
const BAR_LINES: u32 = 3;
/// Empty character used when the caller doesn't set one.
const DEFAULT_EMPTY: char = '░';
/// Bar width used when caller doesn't set one.
const DEFAULT_WIDTH: usize = 20;
/// Fill character used when the caller doesn't set one.
const DEFAULT_FILL: char = '█';
/// Spinner animation
const SPINNER: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

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

    pub fn update(&self, progress: usize) {
        self.state.lock().unwrap().update(progress);
    }

    pub fn start(&mut self, title: &str, total: usize) {
        self.stop_ticker();
        self.state.lock().unwrap().start(title, total);
        if !self.state.lock().unwrap().is_tty {
            return;
        }
        self.stop.store(false, Ordering::Relaxed);
        let state = Arc::clone(&self.state);
        let stop = Arc::clone(&self.stop);

        self.ticker = Some(thread::spawn(move || {
            while !stop.load(Ordering::Relaxed) {
                state.lock().unwrap().draw();
                thread::sleep(Duration::from_millis(80));
            }
        }));
    }

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
        let was_running = self.ticker.is_some();
        self.stop_ticker();
        if was_running && let Ok(state) = self.state.lock() {
            state.cancel();
        }
    }
}

struct State {
    title: String,
    width: usize,
    progress: usize,
    total: usize,
    started: Instant,
    fill_char: char,
    empty_char: char,
    is_tty: bool,
}

impl State {
    fn new() -> Self {
        Self {
            title: String::new(),
            width: DEFAULT_WIDTH,
            progress: 0,
            total: 0,
            started: Instant::now(),
            fill_char: DEFAULT_FILL,
            empty_char: DEFAULT_EMPTY,
            is_tty: std::io::stderr().is_terminal(),
        }
    }

    fn draw(&self) {
        if !self.is_tty {
            return;
        }
        let percentage = (self.progress * 100).checked_div(self.total).unwrap_or(100);
        let filled = (self.progress * self.width)
            .checked_div(self.total)
            .unwrap_or(self.width);
        let fill = self.fill_char.to_string().repeat(filled);
        let empty = self.empty_char.to_string().repeat(self.width - filled);
        let frame = SPINNER[(self.started.elapsed().as_millis() / 80) as usize % SPINNER.len()];
        eprint!("{CSI}{BAR_LINES}A");
        eprintln!(
            "\r{CSI}2K{frame} {} {:.1?}",
            self.title,
            self.started.elapsed()
        );
        eprintln!("\r{CSI}2K[{fill}{empty}] {percentage}%");
        eprintln!();
        std::io::stderr().flush().unwrap();
    }

    fn update(&mut self, progress: usize) {
        self.progress = progress.min(self.total);
    }

    fn start(&mut self, title: &str, total: usize) {
        if self.is_tty {
            for _ in 0..BAR_LINES {
                eprintln!();
            }
        }
        self.title = title.to_string();
        self.total = total;
        self.progress = 0;
        self.started = Instant::now();
        self.draw();
    }

    fn cancel(&self) {
        eprint!("{CSI}{BAR_LINES}A");
        eprint!("\r{CSI}0J");
        eprintln!(
            "\x1b[31m✗\x1b[0m {} (cancelled after {:.1?})",
            self.title,
            self.started.elapsed()
        );
        let _ = std::io::stderr().flush();
    }

    fn complete(&self) {
        if self.is_tty {
            eprint!("{CSI}{BAR_LINES}A");
            eprint!("\r{CSI}0J");
            eprintln!(
                "\x1b[32m✓\x1b[0m {} {:.1?}",
                self.title,
                self.started.elapsed()
            );
            std::io::stderr().flush().unwrap();
        } else {
            eprintln!("✓ {} {:.1?}", self.title, self.started.elapsed())
        }
    }
}

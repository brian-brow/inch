use std::{
    io::{IsTerminal, Write},
    time::Instant,
};

/// Starts an ANSI escape sequence ("Control Sequence Introducer").
const CSI: &str = "\x1b[";
/// How many terminal lines one draw of the bar takes up.
const BAR_LINES: u32 = 3;
/// Empty character used when the caller doesn't set one.
const DEFAULT_EMPTY: char = '░';
/// Bar width used when caller doesn't set one.
const DEFAULT_WIDTH: usize = 30;
/// Fill character used when the caller doesn't set one.
const DEFAULT_FILL: char = '█';

pub struct Bar {
    title: String,
    width: usize,
    last_percent: Option<usize>,
    progress: usize,
    total: usize,
    started: Instant,
    fill_char: char,
    empty_char: char,
    is_tty: bool,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            width: DEFAULT_WIDTH,
            last_percent: None,
            progress: 0,
            total: 0,
            started: Instant::now(),
            fill_char: DEFAULT_FILL,
            empty_char: DEFAULT_EMPTY,
            is_tty: std::io::stderr().is_terminal(),
        }
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn with_fill(mut self, fill_char: char) -> Self {
        self.fill_char = fill_char;
        self
    }

    pub fn with_empty(mut self, empty_char: char) -> Self {
        self.empty_char = empty_char;
        self
    }

    fn draw(&mut self) {
        if !self.is_tty {
            return;
        }
        let percentage = (self.progress * 100).checked_div(self.total).unwrap_or(100);
        if self.last_percent == Some(percentage) {
            return;
        }
        self.last_percent = Some(percentage);
        let filled = (self.progress * self.width)
            .checked_div(self.total)
            .unwrap_or(self.width);
        let fill = self.fill_char.to_string().repeat(filled);
        let empty = self.empty_char.to_string().repeat(self.width - filled);
        eprint!("{CSI}{BAR_LINES}A");
        eprintln!("\r{CSI}2K{} {:.1?}", self.title, self.started.elapsed());
        eprintln!("\r{CSI}2K[{fill}{empty}] {percentage}%");
        eprintln!();
        std::io::stderr().flush().unwrap();
    }

    pub fn update(&mut self, progress: usize) {
        self.progress = progress.min(self.total);
        self.draw();
    }

    pub fn start(&mut self, title: &str, total: usize) {
        if self.is_tty {
            for _ in 0..BAR_LINES {
                eprintln!();
            }
        }
        self.title = title.to_string();
        self.total = total;
        self.progress = 0;
        self.last_percent = None;
        self.started = Instant::now();
        self.draw();
    }

    pub fn complete(&mut self) {
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

impl Default for Bar {
    fn default() -> Self {
        Self::new()
    }
}

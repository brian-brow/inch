use std::io::Write;
use std::thread;
use std::time::Duration;

/// Starts an ANSI escape sequence ("Control Sequence Introducer").
const CSI: &str = "\x1b[";
/// How many terminal lines one draw of the bar takes up (title + bar).
const BAR_LINES: u32 = 2;
/// Fills the unfinished part of the bar.
const EMPTY_CHAR: &str = " ";

struct Bar {
    title: String,
    width: u32,
    fill_char: char,
}

impl Bar {
    fn new(title: &str, width: u32, fill_char: char) -> Self {
        Self { title: title.to_string(), width, fill_char }
    }

    fn print(&self, progress: u32, total: u32) {
        let filled = (progress * self.width) / total;
        let fill = self.fill_char.to_string().repeat(filled as usize);
        let empty = EMPTY_CHAR.repeat((self.width - filled) as usize);
        print!("{CSI}{BAR_LINES}A");
        println!("\r{}", self.title);
        println!("\r[{}{}]", fill, empty);
        if progress >= total {
            println!();
        }
        std::io::stdout().flush().unwrap();
    }

}

impl Drop for Bar {
    fn drop(&mut self) {
        std::io::stdout().flush().unwrap();
    }
}


fn main() {
    let title = "installing your mother...";
    let bar = Bar::new(title, 10, '█');
    bar.print(1, 10);
    thread::sleep(Duration::from_secs(2));
    bar.print(5, 10);
    thread::sleep(Duration::from_secs(2));
    bar.print(10, 10);
}

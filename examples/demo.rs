//! One Rocket League session, a task at a time, showing off every part of inch.

use std::thread;
use std::time::Duration;

use inch::{Bar, Style, spinners};

/// How long one unit of fake work takes.
const STEP: Duration = Duration::from_millis(2);

fn main() {
    // Blocks style, with a wider bar.
    let bar = Bar::new().with_width(30);
    install(&bar);
    download_update(&bar);
    queue(&bar);

    // Worm style on a second bar: the fill becomes the trail it leaves behind.
    let worm = Bar::new()
        .with_style(Style::Worm)
        .with_fill(' ')
        .with_spinner(spinners::ARC);
    play(&worm);
    rage_quit(&worm);

    // `worm` is dropped here with its task still running, so that task ends as cancelled.
}

/// `wrap()` around a range: one step per item, completed when the loop ends.
fn install(bar: &Bar) {
    const LOGS: [&str; 3] = [
        "unpacking cars",
        "unpacking boost pads",
        "unpacking toxicity",
    ];

    for i in bar.wrap("installing rocket league...", 0..1200) {
        if i % 400 == 0 {
            bar.println(LOGS[i / 400]);
        }
        thread::sleep(STEP);
    }
}

/// `update(n)` jumping a chunk at a time, with `set_title` naming each file.
fn download_update(bar: &Bar) {
    const FILES: [(&str, usize); 3] = [("maps.pak", 300), ("cars.pak", 600), ("audio.pak", 200)];
    const CHUNK: usize = 50;
    let total = FILES.iter().map(|(_, size)| size).sum();

    bar.start("downloading update...", total);
    let mut done = 0;
    for (n, (name, size)) in FILES.iter().enumerate() {
        bar.set_title(&format!(
            "downloading update... {name} ({}/{})",
            n + 1,
            FILES.len()
        ));
        for _ in 0..size / CHUNK {
            done += CHUNK;
            bar.update(done);
            thread::sleep(STEP * CHUNK as u32);
        }
        bar.println(&format!("got {name}"));
    }
    bar.complete();
}

/// Gives up partway with `fail()`, which keeps the log and adds the reason.
fn queue(bar: &Bar) {
    const REGIONS: [&str; 3] = ["searching US-East", "searching EU", "searching Oceania"];
    let total = 1000;

    bar.start("joining ranked...", total);
    for i in 0..total {
        if i % 200 == 0 && i / 200 < REGIONS.len() {
            bar.println(REGIONS[i / 200]);
        }
        if i == 700 {
            bar.fail("no servers found");
            return;
        }
        bar.inc();
        thread::sleep(STEP);
    }
    bar.complete();
}

/// The worm crawling along with the match via `inc()`, the score kept in the title.
fn play(bar: &Bar) {
    /// When each goal happens, and whether it was ours.
    const GOALS: [(usize, bool, &str); 4] = [
        (400, true, "goal! what a save... what a save..."),
        (900, false, "own goal"),
        (1300, true, "goal!"),
        (1700, true, "buzzer beater"),
    ];
    let total = 2000;
    let (mut us, mut them) = (0, 0);

    bar.start("playing match... 0-0", total);
    for i in 0..total {
        if let Some(&(_, ours, log)) = GOALS.iter().find(|(at, _, _)| *at == i) {
            if ours {
                us += 1;
            } else {
                them += 1;
            }
            bar.set_title(&format!("playing match... {us}-{them}"));
            bar.println(log);
        }
        bar.inc();
        thread::sleep(STEP);
    }
    bar.complete();
}

/// Never completes. `main` drops the bar mid-task, which cancels it.
fn rage_quit(bar: &Bar) {
    const LOGS: [&str; 3] = [
        "this game sucks anyways",
        "garbage teammates",
        "that wasn't even my fault",
    ];

    bar.start("uninstalling rocket league...", 1000);
    for log in LOGS {
        bar.println(log);
        for _ in 0..200 {
            bar.inc();
            thread::sleep(STEP);
        }
    }
    bar.println("...actually, one more game");
}

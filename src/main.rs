use std::thread;
use std::time::Duration;

use inch::Bar;

fn main() {
    let installs = [
        ("installing money...", 10_000),
        ("downloading rocket league...", 20_000),
        ("uninstalling rocket league...", 7_000),
    ];
    // Each install is a (title, total) pair that drives one run of the bar.
    // The totals are step counts, not bytes; each step sleeps 200µs below.
    // "downloading rocket league..." bails out at 18_000 to show an incomplete bar.
    // The log arrays below get printed above the bar mid-run via bar.println().

    const DOWNLOAD_LOGS: [&str; 5] = [
        "going up",
        "going down",
        "getting boost",
        "faking",
        "missing",
    ];

    const UNINSTALL_LOGS: [&str; 3] = [
        "this game sucks anyways",
        "garbage teammates",
        "that wasn't even my fault",
    ];

    // let mut bar = Bar::new().with_fill(' ').with_empty('·').with_style(inch::Style::Worm);
    let mut bar = Bar::new().with_fill('*').with_empty('·');
    let mut stop = false;

    for (title, total) in installs {
        bar.start(title, total);
        for i in 0..total {
            bar.inc();
            thread::sleep(Duration::from_micros(200));
            if title == "installing money..." && i == 5000 {
                bar.println(
                    &"this log line is far too long for any reasonable terminal ".repeat(6),
                );
                bar.println("wow!!!!!!");
            }
            if title == "downloading rocket league..." {
                if i % 4000 == 0 {
                    bar.println(DOWNLOAD_LOGS[i / 4000 % DOWNLOAD_LOGS.len()]);
                }
                if i == 18_000 {
                    stop = true;
                    break;
                }
            }
            if title == "uninstalling rocket league..." && i % 2000 == 0 {
                bar.println(UNINSTALL_LOGS[i / 2000 % UNINSTALL_LOGS.len()]);
            }
        }
        if !stop {
            bar.complete();
        }
        stop = false;
    }
}

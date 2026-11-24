use std::thread;
use std::time::Duration;

use inch::Bar;

fn main() {
    let installs = [
        ("installing your mom...", 10_000),
        ("downloading rocket league...", 10_000),
        ("uninstalling rocket league...", 20_000),
    ];

    let mut bar = Bar::new().with_empty('·');
    let mut stop = false;

    for (title, total) in installs {
        bar.start(title, total);
        for i in 0..=total {
            bar.update(i);
            thread::sleep(Duration::from_micros(200));
            if title == "downloading rocket league..." && i >= total / 2 {
                stop = true;
                break;
            }
        }
        if !stop {
            bar.complete();
        }
        stop = false;
    }
}

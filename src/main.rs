use std::thread;
use std::time::Duration;

use bar::Bar;

fn main() {
    let installs = [
        ("installing your mom...", 10_000),
        ("downloading rocket league...", 10_000),
        ("uninstalling rocket league...", 20_000),
    ];

    let mut bar = Bar::new().with_width(20).with_empty('·');

    for (title, total) in installs {
        bar.start(title, total);
        for i in 0..=total {
            bar.update(i);
            thread::sleep(Duration::from_micros(200));
        }
        bar.complete();
    }
}

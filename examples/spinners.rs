//! Shows every spinner for a moment. `cargo run --example spinners [name]`
//! previews only the spinners whose name contains `name`.

use std::thread;
use std::time::Duration;

use inch::Bar;
use inch::spinners::ALL;

fn main() {
    let filter = std::env::args().nth(1).unwrap_or_default().to_lowercase();
    let mut bar = Bar::new();

    for (name, spinner) in ALL {
        if !name.to_lowercase().contains(&filter) {
            continue;
        }
        bar = bar.with_spinner(*spinner);
        bar.start(name, 100);
        for i in 0..=100 {
            bar.update(i);
            thread::sleep(Duration::from_millis(15));
        }
        bar.complete();
    }
}

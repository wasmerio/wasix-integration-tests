/// Spawn a number of threads that sleep for inifinity.
use std::io::BufRead;

fn main() {
    let count: usize = if let Some(v) = std::env::args().skip(1).next() {
        v.parse()
            .expect("first argument must be the number of threads")
    } else {
        99
    };

    eprintln!("Spawning {} threads", count);

    let sleeptime = std::time::Duration::from_secs(1000000);

    for index in 1..=count {
        std::thread::spawn(move || {
            println!("thread {} started", index);
            std::thread::sleep(sleeptime);
        });
    }

    eprintln!("All threads spawned");
    eprintln!("Press enter to exit");

    std::io::stdin().lock().lines().next();
}

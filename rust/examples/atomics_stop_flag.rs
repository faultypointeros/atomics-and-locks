use std::{
    sync::atomic::{AtomicBool, AtomicUsize},
    thread,
    time::Duration,
};

fn main() {
    let stop: AtomicBool = AtomicBool::new(false);
    let counter: AtomicUsize = AtomicUsize::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            let sleep_durations = [0.4, 0.6, 0.2, 0.9, 0.1];
            while !stop.load(std::sync::atomic::Ordering::Relaxed) {
                let idx = counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                thread::sleep(Duration::from_secs_f32(
                    sleep_durations[idx % sleep_durations.len()],
                ));
            }
        });

        for line in std::io::stdin().lines() {
            match line.unwrap().as_str() {
                "help" => println!("commands: help, stop, print"),
                "stop" => break,
                "print" => println!(
                    "counter: {}",
                    counter.load(std::sync::atomic::Ordering::Relaxed)
                ),
                cmd => eprintln!("unknown command {cmd}"),
            }
        }
        stop.store(true, std::sync::atomic::Ordering::Relaxed);
    });
}

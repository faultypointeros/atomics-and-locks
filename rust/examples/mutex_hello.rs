use std::{sync::Mutex, thread, time::Duration};

fn main() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard);
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    let n = n.into_inner().unwrap();
    assert_eq!(n, 1000);
    println!("n: {n}");
}

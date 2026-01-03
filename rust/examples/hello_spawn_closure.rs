use std::thread;

fn main() {
    let t1 = thread::spawn(|| {
        println!("Hello from another thread.");
    });
    println!("Hello from main thread");
    t1.join().unwrap();
}

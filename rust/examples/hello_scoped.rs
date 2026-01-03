use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            println!("Sum: {}", v.iter().sum::<usize>());
        });

        s.spawn(|| {
            println!("Count: {}", v.len());
        });
    });
}

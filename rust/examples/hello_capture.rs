use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // See `Notes/Hello.md/(Moving value to closure)` for info on why the move here is necessary.
    // without the `move` v would be passed as a reference which is not allowed.
    thread::spawn(move || {
        println!("Sum: {}", v.iter().sum::<usize>());
    })
    .join()
    .unwrap();
}

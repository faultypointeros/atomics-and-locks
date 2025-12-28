use std::thread;

fn hello() {
    let mut threads = Vec::with_capacity(2);

    for _ in 0..2 {
        threads.push(thread::spawn(|| {
            println!("Hello from another thread");

            let id = thread::current().id();
            println!("This is my thread id: {id:?}");
        }));
    }
    println!("Hello from main thread");
    threads.into_iter().for_each(|t| t.join().unwrap());
}

fn captured() {
    let v = vec![1, 2, 3];

    // The signature of spawn function here is this
    // ```rust
    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>where
    //     F: FnOnce() -> T + Send + 'static,
    //     T: Send + 'static,
    // ```
    // The function or closure passed to spawn and it's return value must me valid
    // until the lifetime of the whole program ('static)
    // This is because the thread and its return value may outlive the calling thread.
    // Hence without the `move` v would be passed as a reference which is not allowed.
    thread::spawn(move || {
        println!("Hello from another thread. This is a vec: {v:?}");
    })
    .join()
    .unwrap();

    println!("Hello from main thread");
}

fn returned() {
    let v = vec![1, 2, 3, 4, 5];

    let t = thread::spawn(move || v.iter().sum::<usize>());

    println!("The sum is: ... (wait for it)");
    println!("{}", t.join().unwrap());
}

fn builder() {
    // thread builer allows building thread by specifying various options
    // before spawning the thread. It also returns a result instead of panicking
    // in case of error like std::thread::spawn
    thread::Builder::new()
        .name("another thread".to_string())
        .stack_size(100)
        .spawn(|| println!("Hello from another thread"))
        .ok();
}

fn scoped() {
    // remember when the closure passed to spawn couldn't borrow local values
    // because the closure had to be 'static
    // well we can use thread::scope, which gives a Scope object which we can use
    // to spawn threads
    // the scope object guarantees that the thread spawned using the scope object
    // will be joined before the end of scope.
    // hence we can pass references to values that outlives this scope
    let v = Vec::from_iter(1..=1000);
    thread::scope(|s| {
        s.spawn(|| {
            println!("Sum of v: {}", v.iter().sum::<isize>());
        });

        s.spawn(|| {
            println!("Length of v: {}", v.len());
        });
    });
}

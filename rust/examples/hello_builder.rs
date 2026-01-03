fn main() {
    let Ok(t1) = std::thread::Builder::new()
        .name("mythread".to_string())
        .spawn(|| {
            println!("Hello from {}", std::thread::current().name().unwrap());
        })
    else {
        println!("No mythread for you");
        return;
    };
    println!("Hello from main");
    t1.join().unwrap();
}

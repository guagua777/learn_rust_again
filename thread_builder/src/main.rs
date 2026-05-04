use std::thread;


fn main() {
    let handler = thread::Builder::new()
    .name("task".into())
    .stack_size(4 * 1024 * 1024)
    .spawn(task)
    .unwrap();

    handler.join().unwrap();
}


fn task() {
    let name = thread::current();
    let name = name.name().unwrap();
    println!("in this thread, {name:?}");
}

use std::sync::Arc;
use std::thread;

fn main() {
    let a = Arc::new([1, 2, 3]);

    thread::spawn({
        // scope starts
        let a = a.clone(); // shadows outer a
        move || {
            dbg!(a);
        }
    }); // scope ends, inner a goes out of scope

    dbg!(a); // original a
}

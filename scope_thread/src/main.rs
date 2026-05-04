use std::thread;


fn main() {

    let a = String::from("hello");

    // 没有move
    thread::scope(|s| {
      for i in 0..5 {
          s.spawn(|| {
              thread::sleep(std::time::Duration::from_secs(1));
             println!("in this thread, {a}");
          });
      }  
    })
}


fn main10() {
    // 有move
    // 在thread::scope之前创建的变量，不需要move，在scope里面创建的变量，需要move
    thread::scope(|s| {
      for i in 0..5 {
          s.spawn(move || {
              thread::sleep(std::time::Duration::from_secs(1));
             println!("in this thread, {i}");
          });
      }  
    })
}

fn main1() {
    let mut handlers = Vec::new();

    for i in 0..5 {
        let handler = thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(1));
           println!("in this thread, {i}");
        });
        handlers.push(handler);
    }
}

fn main2() {
    let mut handlers = Vec::new();

    for i in 0..5 {
        let handler = thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(1));
           println!("in this thread, {i}");
        });
        handlers.push(handler);
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());
}

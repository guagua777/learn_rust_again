use counter::test_const_fn;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use std::{fs, thread, vec};
use std::sync::Arc;

// cargo run --bin multi

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let path = "./Rust_multi_thread";

    let mut map = HashMap::new();

    let files: Vec<_> = fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("txt") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // scope thread, 作用域线程，线程间传递所有权
    const CHUNK_SIZE: usize = 20;
    let files = Arc::new(files);
    let files_clone = Arc::clone(&files);
    // chunks方法，引用了原始数据
    // 问题不在arc，而在chunks方法，chunks方法返回的切片引用了原始数据，线程间传递所有权时，切片的生命周期不满足线程要求
    let chunks = files_clone.chunks(CHUNK_SIZE);

    let mut handlers = vec![];
    for chunk in chunks {
        
        let chunk = chunk.to_vec();

        let mut local_map = HashMap::new();
        let handler = thread::spawn(move || {
            chunk
                .iter()
                .filter_map(|p| fs::read_to_string(p).ok())
                .for_each(|text| {
                    text.split_whitespace().for_each(|w| {
                        let word = w
                            .trim_matches(|c: char| c.is_ascii_punctuation())
                            .to_lowercase();
                        if !w.is_empty() {
                            *local_map.entry(word).or_insert(0) += 1;
                        }
                    });
                });
            local_map
        });
        handlers.push(handler);
    }

    for h in handlers {
        let local_map = h.join().unwrap();
        for (word, count) in local_map {
            *map.entry(word).or_insert(0) += count;
        }
    }

    println!("map len is {}", map.len());
    let mut vec: Vec<(&String, &usize)> = map.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(a.1));
    for (word, count) in vec.iter().take(10) {
        println!("{}: {}", word, count);
    }

    let elapsed = start.elapsed();
    println!("Elapsed: {}", elapsed.as_millis());

    Ok(())
}

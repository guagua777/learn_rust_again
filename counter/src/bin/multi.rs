use counter::test_const_fn;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use std::{fs, thread, vec};

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

    // .for_each(|p| {
    //         if let Ok(text) = fs::read_to_string(p) {
    //             text.split_whitespace().for_each(|w| {
    //                 let word = w
    //                     .trim_matches(|c: char| c.is_ascii_punctuation())
    //                     .to_lowercase();
    //                 if !w.is_empty() {
    //                     *map.entry(word).or_insert(0) += 1;
    //                 }
    //             });
    //         }
    //     });


    // scope thread, 作用域线程，线程间传递所有权
    const CHUNK_SIZE: usize = 20;
    let chunks = files.chunks(CHUNK_SIZE);

    thread::scope(|s| {

        let mut handlers = vec![];    
        for chunk in chunks {
            // let chunk = chunk.to_owned(); // 克隆切片，线程间传递所有权
            // let chunk = chunk.to_vec(); // 转成 Vec，线程间传递所有权
            let mut local_map = HashMap::new();
            //thread::spawn
            // 使用s::spawn
            // let handler = thread::spawn(move || {
            let handler = s.spawn( || {
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

    });
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

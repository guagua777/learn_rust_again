use counter::test_const_fn;
use std::collections::HashMap;
use std::error::Error;
use std::{fs, vec};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let path = "./Rust_multi_thread";

    let mut map = HashMap::new();

    fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("txt") {
                Some(path)
            } else {
                None
            }
        })
        .for_each(|p| {
            if let Ok(text) = fs::read_to_string(p) {
                text.split_whitespace().for_each(|w| {
                    let word = w
                        .trim_matches(|c: char| c.is_ascii_punctuation())
                        .to_lowercase();
                    if !w.is_empty() {
                        *map.entry(word).or_insert(0) += 1;
                    }
                });
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

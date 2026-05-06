use std::option;

fn main() {
    let option: Option<i32> = Some(123);
    assert_eq!(option, Some(123));

    let r = option
    .map(|v| v == 2) // 返回值为Some(false)，并不是None
    .filter(|v| *v) // 过滤出true
    .unwrap_or_else(|| {
        println!("option is None");
        true
    });

    println!("r: {}", r);

    // assert_eq!(r, false);


    let option: Option<i32> = None;
    let r = option
    .map(|v| v == 2) // 此时才是None
    .unwrap_or_else(|| {
        println!("option 2222 is None");
        true
    });

    println!("r2222: {}", r);

    assert_eq!(r, true);


}
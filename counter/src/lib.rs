use rand::Rng;

pub mod collect_test;
pub mod collect_test1;
pub mod cross_thread;
pub mod scope;



const fn double(x: i32) -> Option<i32> {
    Some(x * 2)
}

fn random(x: i32) -> Option<i32> {
    println!("Random number: {}", rand::random::<i32>());
    Some(rand::random()) // 随机数，必须运行时才能算
}



pub fn test_const_fn() {
    let RESULT: Option<i32> = Some(5).and_then(double);
    // 编译期直接算出：Some(10)

    let RESULT = Some(5).and_then(random); 
}




#[cfg(test)]
mod tests {
    use super::*;   

    #[test]
    fn test_const_fn() {
        super::test_const_fn();
    }
}
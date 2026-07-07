

#[link(name = "m")] // 链接数学库 libm.so / libm.dylib
unsafe extern "C" {
    fn sin(x: f64) -> f64;
}   

pub fn main() {
    let x = 0.5;
    let sin_x = unsafe { sin(x) };
    println!("sin(0.5) = {}", sin_x);
}
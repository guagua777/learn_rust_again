fn random_vec() -> &'static [u64; 100] {
    let mut boxed = Box::new([0u64; 100]);
    rand::fill(boxed.as_mut_slice());
    Box::leak(boxed)
}

fn main() {
    let first: &'static [u64; 100] = random_vec();
    let second: &'static [u64; 100] = random_vec();
    assert_ne!(first, second)
}
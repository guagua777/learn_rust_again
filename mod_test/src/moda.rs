
use crate::modb;

pub fn moda_fun() {
    println!("moda_fun");
}


fn use_modb_fun() {
    modb::modb_fun();
}

fn use_modb_suba_fun() {
    modb::modb_suba::modb_suba_fun();
}


#[test]
fn test_moda_fun() {
    moda_fun();
    use_modb_fun();
    use_modb_suba_fun();
}

use moda::moda_sub;
use modb::modb_sub;


mod moda;
mod modb;


fn main() {
    println!("Hello, world!");
    moda_sub::moda_sub_fun();
    modb_sub::modb_sub_fun();
}

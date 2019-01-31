extern crate phony;

use phony::random::Random;


fn main() {
    let mut random = Random::new(8191);

    println!("--->  {}", random.random_int(10, 20));
    println!("===> {}", random.random_digit());
}

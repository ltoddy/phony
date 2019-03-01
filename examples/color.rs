extern crate phony;

use phony::Provider;

fn main() {
    let mut provide = Provider::new();

    println!("{}", provide.color.safe_color_name());
    println!("{}", provide.color.safe_color_name());
}

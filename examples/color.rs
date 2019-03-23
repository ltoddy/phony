extern crate phony;

use phony::Provider;

fn main() {
    let mut provide = Provider::new();

    println!("{}", provide.color.safe_color_name());
    println!("{}", provide.color.safe_color_name());
    println!("{}", provide.color.safe_color_name());

    println!("{}", provide.color.color_name());
    println!("{}", provide.color.color_name());
    println!("{}", provide.color.color_name());

    println!("{}", provide.color.hex_color());
    println!("{}", provide.color.hex_color());
    println!("{}", provide.color.hex_color());

    println!("{}", provide.color.safe_hex_color());
    println!("{}", provide.color.safe_hex_color());
    println!("{}", provide.color.safe_hex_color());

    println!("{}", provide.color.rgb_color());
    println!("{}", provide.color.rgb_color());
    println!("{}", provide.color.rgb_color());

    println!("{}", provide.color.rgb_css_color());
    println!("{}", provide.color.rgb_css_color());
    println!("{}", provide.color.rgb_css_color());
}

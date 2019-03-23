<div align="center">
  <h1>phony</h1>
  <a href="https://travis-ci.com/ltoddy/phony"><img src="https://travis-ci.com/ltoddy/phony.svg?branch=master" alt="Build Status"/></a>
  <a href="https://crates.io/crates/phony"><img src="https://img.shields.io/crates/v/phony.svg" alt="Latest version"/></a>
  <a href="https://docs.rs/phony"><img src="https://docs.rs/phony/badge.svg" alt="Documentation"/></a>
</div>

<p align="center">generates phony data</p>


### Color

```rust
extern crate phony;

use phony::Provider;

fn main() {
    let mut provide = Provider::new();

    println!("{}", provide.color.safe_color_name());
    println!("{}", provide.color.color_name());
    println!("{}", provide.color.hex_color());
    println!("{}", provide.color.safe_hex_color());
    println!("{}", provide.color.rgb_color());
    println!("{}", provide.color.rgb_css_color());
}
```

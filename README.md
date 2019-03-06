# Identicon-rs

[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/fluffy-samurai/identicon-rs/blob/master/LICENSE)
[![Build Status](https://travis-ci.com/fluffy-samurai/identicon-rs.svg?branch=master)](https://travis-ci.com/fluffy-samurai/identicon-rs)
[![dependency status](https://deps.rs/crate/identicon-rs/1.2.1/status.svg)](https://deps.rs/crate/identicon-rs/1.2.1)

This is an Identicon implementation in rust.

## Example:
```rust
use identicon_rs::Identicon;

fn main() {
    let fluffy_samurai = String::from("fluffy-samurai");
    let test_string = "identicon_rs";

    // stored example
    let identicon_fluffy = Identicon::new_default(&fluffy_samurai);
    identicon_fluffy.save_image("output_1.png");

    // chained example with no border
    Identicon::new_no_border(test_string).save_image("output_2.png");
}
```

## TODO
- **add symmetry option for image generation**
- **add documentation to code**
- **add more information to README.md**
- **add more examples**
- catch file extensions in the input value
- build a rust cli using this
- build an npm/wasm package using this

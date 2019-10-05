# Identicon-rs

![](https://github.com/fluffy-samurai/identicon-rs/workflows/CI%20Pipeline/badge.svg)
[![dependency status](https://deps.rs/crate/identicon-rs/1.4.0/status.svg)](https://deps.rs/crate/identicon-rs/1.4.0)

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
- add more information to README.md
- add more examples

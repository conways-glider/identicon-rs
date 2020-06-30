# Identicon-rs

![](https://github.com/fluffy-samurai/identicon-rs/workflows/CI%20Pipeline/badge.svg)
[![dependency status](https://deps.rs/crate/identicon-rs/1.4.1/status.svg)](https://deps.rs/crate/identicon-rs/1.4.1)

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

You can run this example with `cargo run --example main`.

The repository contains an example webservice that you can run with `cargo run --example webserver`.

## TODO
- add more information to README.md
- add more examples

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

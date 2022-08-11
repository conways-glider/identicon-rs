# Identicon-rs

[![Continuous Integration](https://github.com/conways-glider/identicon-rs/actions/workflows/ci-workflow.yml/badge.svg)](https://github.com/conways-glider/identicon-rs/actions/workflows/ci-workflow.yml)
[![dependency status](https://deps.rs/crate/identicon-rs/3.1.0/status.svg)](https://deps.rs/crate/identicon-rs/3.1.0)

This is an Identicon implementation in rust.

## Example:
```rust
use identicon_rs::error::IdenticonError;
use identicon_rs::Identicon;

fn main() -> Result<(), IdenticonError> {
    let conways_glider = String::from("conways-glider");
    let test_string = "identicon_rs";

    // stored example
    let identicon_conways_glider = Identicon::new(&conways_glider);
    identicon_conways_glider.save_image("output_1.png")?;

    // chained example with no border
    Identicon::new(test_string)
        .set_border(0)
        .save_image("output_2.png")?;
    Ok(())
}
```

You can run this example with `cargo run --example main`.

The repository contains an example webservice that you can run with `cargo run --example webserver`.

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

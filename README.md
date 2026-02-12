# Identicon-rs

[![Rust](https://github.com/conways-glider/identicon-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/conways-glider/identicon-rs/actions/workflows/rust.yml)
[![dependency status](https://deps.rs/crate/identicon-rs/7.1.0/status.svg)](https://deps.rs/crate/identicon-rs/7.1.0)
[![Crates.io](https://img.shields.io/crates/v/identicon-rs)](https://crates.io/crates/identicon-rs)
[![Documentation](https://docs.rs/identicon-rs/badge.svg)](https://docs.rs/identicon-rs)

This is an Identicon implementation in rust.

## Documentation

- [docs.rs](https://docs.rs/identicon-rs)
- [GitHub Pages](https://conways-glider.github.io/identicon-rs/)

## Example

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

You will obtain images analogous to the following ones:

<p align="middle">
  <img src="examples/example1.png" width="100" />
  <img src="examples/example2.png" width="100" />
  <img src="examples/example3.png" width="100" />
</p>

## Minimum Supported Rust Version (MSRV) Policy

This project currently intends to keep our MSRV on a N-2 policy. That means that we will update the MSRV to at most 2 versions behind the current Rust version.

The MSRV is not automatically updated, however we will do our best to consider the benefits of each Rust release.

This policy is subject to changing at any time, but it won't change without a substantial feature needed.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

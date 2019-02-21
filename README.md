# Identicon

This is an Identicon implementation in rust.

## Example:
```rust
use identicon_rs;

fn main() {
    let fluffy_samurai = "fluffy-samurai";
    let test_string = "identicon_rs";
    identicon_rs::save_scaled_image(fluffy_samurai, "output_1.png", 500);
    identicon_rs::save_scaled_image(test_string, "output_2.png", 500);

    identicon_rs::save_bordered_image(fluffy_samurai, "output_1_bordered.png", 500, 50);
    identicon_rs::save_bordered_image(test_string, "output_2_bordered.png", 500, 50);
}
```

## TODO
- investigate removal of palette crate
- move functions into an object that stores the generated data, along with the input data
- build a rust cli using this
- build a rust server (using actix?) to publish images
- build an npm/wasm package using this

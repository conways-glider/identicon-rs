# Identicon

This is an Identicon implementation in rust.

## Example:
```rust
use identicon_rs::Identicon;

fn main() {
    let fluffy_samurai = "fluffy-samurai";
    let test_string = "identicon_rs";

    // stored example
    let identicon_fluffy = Identicon::new_default(fluffy_samurai);
    identicon_fluffy.save_image("output_1.png");

    // chained example with no border
    Identicon::new_no_border(test_string).save_image("output_2.png");
}
```

## TODO
- prevent second image generation loop due to borders
- build a rust cli using this
- build a rust server (using actix?) to publish images
- build an npm/wasm package using this

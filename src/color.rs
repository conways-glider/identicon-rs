use sha2::{
    Sha512Trunc256,
    Digest
};

use rand::prelude::*;

use palette::{
    LinSrgb,
    Hsl
};

pub fn generate_color(input_value: &str) -> LinSrgb<u8> {
    // Compute the seed
    // Due to limitations of rand, rand::SeedableRng needs
    // an array of 32 u8 values.
    // Slices are not currently accepted.
    let hash_256 = Sha512Trunc256::digest(input_value.as_bytes());
    let hash_256_slice = hash_256.as_slice();
    let mut seed: [u8; 32] = [0;32];
    seed.copy_from_slice(hash_256_slice);

    let mut rng: StdRng = rand::SeedableRng::from_seed(seed);
    let hue = rng.gen_range(0.0, 360.0);
    let saturation = rng.gen_range(0.25, 0.95);
    let lightness = rng.gen_range(0.75, 0.85);

    println!("HSL: {}, {}, {}", hue, saturation, lightness);

    let color_hsl = Hsl::new(hue, saturation, lightness);

    let color_rgb = LinSrgb::from(color_hsl)
        .into_format();

    color_rgb
}

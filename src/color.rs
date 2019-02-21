use sha2::{Digest, Sha512};

use palette::{Hsl, LinSrgb};

use crate::map_values::map_values;

pub fn generate_color(input_value: &str) -> LinSrgb<u8> {
    // compute hash for hue space in larger bitspace
    let hash = Sha512::digest(input_value.as_bytes());
    let hue_hash_1 = (hash[0] as u16 & 0x0f) << 8;
    let hue_hash_2 = hash[1] as u16;
    let hue_hash = (hue_hash_1 | hue_hash_2) as u32;

    // compute hsl values
    let hue = map_values(hue_hash, 0, 4095, 0, 360);
    let saturation = map_values(hash[2] as u32, 0, 255, 25, 95) / 100.0;
    let lightness = map_values(hash[3] as u32, 0, 255, 75, 85) / 100.0;

    // convert color to rgb value
    let color_hsl = Hsl::new(hue, saturation, lightness);
    let color_rgb = LinSrgb::from(color_hsl).into_format();

    color_rgb
}

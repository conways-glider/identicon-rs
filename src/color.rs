use crate::map_values::map_values;
use image::Rgb;
use palette::{FromColor, Hsl, Srgb};

pub fn generate_color(hash: &[u8]) -> Rgb<u8> {
    // compute hash for hue space in larger bitspace
    let hue_hash_1 = (hash[0] as u16 & 0x0f) << 8;
    let hue_hash_2 = hash[1] as u16;
    let hue_hash = (hue_hash_1 | hue_hash_2) as u32;

    // compute hsl values
    let hue = map_values(hue_hash, 0, 4095, 0, 360);
    let saturation = map_values(hash[2] as u32, 0, 255, 50, 75) / 100.0;
    let lightness = map_values(hash[3] as u32, 0, 255, 60, 70) / 100.0;

    // convert color to rgb value
    let color_hsl = Hsl::new(hue, saturation, lightness);
    let color_rgb = Srgb::from_color(color_hsl);
    image::Rgb([
        color_rgb.red as u8,
        color_rgb.green as u8,
        color_rgb.blue as u8,
    ])
}

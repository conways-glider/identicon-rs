use palette::{FromColor, Srgb};

use crate::map_values::map_values;

/// Generates the color of the active pixels in RGB.
///
/// This is based on [https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB](https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB).
pub fn generate_rgb_color(hash: &[u8]) -> (u8, u8, u8) {
    // Convert HSL to RGB
    let hsl_raw = generate_hsl(hash);

    // Convert to palette object
    let hsl = palette::Hsl::new(hsl_raw.0, hsl_raw.1, hsl_raw.2);

    // Convert to RGB
    let color_rgb: Srgb<u8> = Srgb::from_color(hsl).into_format();

    // Convert from palette object to (u8, u8, u8)
    color_rgb.into_components()
}

/// Generates the HSL based on the hash.
///
/// Returns a tuple of (hue, saturation, lightness).
///
/// The specific ranges are made to generate pastel colors.
fn generate_hsl(hash: &[u8]) -> (f32, f32, f32) {
    // Compute hash for hue space in larger bitspace
    let hue_hash_1 = (hash[0] as u16 & 0x0f) << 8;
    let hue_hash_2 = hash[1] as u16;
    let hue_hash = (hue_hash_1 | hue_hash_2) as u32;

    // Compute HSL values
    let hue = map_values(hue_hash as f32, 0.0, 4095.0, 0.0, 360.0);

    // Handle 0 degree hue is equivalent to 360 degree hue
    let hue = hue % 360.0;

    // Saturation should be between 0.5 and 0.75 for pastel colors
    let saturation = map_values(hash[2] as f32, 0.0, 255.0, 50.0, 75.0) / 100.0;

    // Lightness should be between 0.6 and 0.70 for pastel colors
    let lightness = map_values(hash[3] as f32, 0.0, 255.0, 60.0, 70.0) / 100.0;
    (hue, saturation, lightness)
}

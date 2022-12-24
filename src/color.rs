use crate::map_values::map_values;

/// Generates the color of the active pixels.
///
/// This is based on [https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB](https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB).
pub fn generate_color(hash: &[u8]) -> (u8, u8, u8) {
    let hsl = Hsl::from(hash);

    // convert hsl to rgb
    let chroma = (1.0 - ((2.0 * hsl.lightness) - 1.0).abs()) * hsl.saturation;
    let hue_prime = hsl.hue / 60.0;
    let x = chroma * (1.0 - (hue_prime % 2.0 - 1.0).abs());

    let (r_prime, g_prime, b_prime) = if (0.0..1.0).contains(&hue_prime) {
        (chroma, x, 0.0)
    } else if (1.0..2.0).contains(&hue_prime) {
        (x, chroma, 0.0)
    } else if (2.0..3.0).contains(&hue_prime) {
        (0.0, chroma, x)
    } else if (3.0..4.0).contains(&hue_prime) {
        (0.0, x, chroma)
    } else if (4.0..5.0).contains(&hue_prime) {
        (x, 0.0, chroma)
    } else if (5.0..6.0).contains(&hue_prime) {
        (chroma, 0.0, x)
    } else {
        // This should not occur as the hue is between 0 and 360, which casts down to between 0-6
        (0.0, 0.0, 0.0)
    };

    // lightness modifier
    let m = hsl.lightness - chroma * 0.5;

    let red = (r_prime + m) * 255.0;
    let green = (g_prime + m) * 255.0;
    let blue = (b_prime + m) * 255.0;

    (red as u8, green as u8, blue as u8)
}

#[derive(Debug)]
pub struct Hsl {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}

impl From<&[u8]> for Hsl {
    fn from(hash: &[u8]) -> Self {
        // compute hash for hue space in larger bitspace
        let hue_hash_1 = (hash[0] as u16 & 0x0f) << 8;
        let hue_hash_2 = hash[1] as u16;
        let hue_hash = (hue_hash_1 | hue_hash_2) as u32;

        // compute hsl values
        let hue = map_values(hue_hash as f32, 0.0, 4095.0, 0.0, 360.0);

        let hue = hue % 360.0;

        // saturation should be between 0.5 and 0.75 for pastel colors
        let saturation = map_values(hash[2] as f32, 0.0, 255.0, 50.0, 75.0) / 100.0;

        // lightness should be between 0.6 and 0.70 for pastel colors
        let lightness = map_values(hash[3] as f32, 0.0, 255.0, 60.0, 70.0) / 100.0;
        Hsl {
            hue,
            saturation,
            lightness,
        }
    }
}

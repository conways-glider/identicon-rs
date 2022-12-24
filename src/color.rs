use crate::map_values::map_values;

/// Generates the color of the active pixels.
///
/// This is based on [https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB](https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB).
pub fn generate_color(hash: &[u8]) -> (u8, u8, u8) {
    let hsl = get_hsl_from_hash(hash);

    // convert hsl to rgb
    let chroma = (1.0 - ((2.0 * hsl.lightness) - 1.0).abs()) * hsl.saturation;
    let hue_prime = hsl.hue / 60.0;
    let x = chroma * (1.0 - (hue_prime % 2.0 - 1.0).abs());
    let m = hsl.lightness - chroma * 0.5;

    let (r_prime, g_prime, b_prime) = if hue_prime >= 0.0 && hue_prime < 1.0 {
        (chroma, x, 0.0)
    } else if hue_prime >= 1.0 && hue_prime < 2.0 {
        (x, chroma, 0.0)
    } else if hue_prime >= 2.0 && hue_prime < 3.0 {
        (0.0, chroma, x)
    } else if hue_prime >= 3.0 && hue_prime < 4.0 {
        (0.0, x, chroma)
    } else if hue_prime >= 4.0 && hue_prime < 5.0 {
        (x, 0.0, chroma)
    } else if hue_prime >= 5.0 && hue_prime < 6.0 {
        (chroma, 0.0, x)
    } else {
        (0.0, 0.0, 0.0)
    };

    let red = (r_prime + m) * 255.0;
    let green = (g_prime + m) * 255.0;
    let blue = (b_prime + m) * 255.0;

    (red as u8, green as u8, blue as u8)
}

pub struct Hsl {
    hue: f32,
    saturation: f32,
    lightness: f32,
}

pub fn get_hsl_from_hash(hash: &[u8]) -> Hsl {
    // compute hash for hue space in larger bitspace
    let hue_hash_1 = (hash[0] as u16 & 0x0f) << 8;
    let hue_hash_2 = hash[1] as u16;
    let hue_hash = (hue_hash_1 | hue_hash_2) as u32;

    // compute hsl values
    let hue = map_values(hue_hash as f32, 0.0, 4095.0, 0.0, 360.0);

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

use crate::map_values::map_values;

// use palette::{FromColor, Hsl, Srgb};

// from_color_unclamped(hsl: Hsl<S, T>) -> Self {
//     let c = (T::one() - (hsl.lightness.clone() * T::from_f64(2.0) - T::one()).abs())
//         * hsl.saturation;
//     let h = hsl.hue.into_positive_degrees() / T::from_f64(60.0);
//     // We avoid using %, since it's not always (or never?) supported in SIMD
//     let h_mod_two = h.clone() - Round::floor(h.clone() * T::from_f64(0.5)) * T::from_f64(2.0);
//     let x = c.clone() * (T::one() - (h_mod_two - T::one()).abs());
//     let m = hsl.lightness - c.clone() * T::from_f64(0.5);

//     let is_zone0 = h.gt_eq(&T::zero()) & h.lt(&T::one());
//     let is_zone1 = h.gt_eq(&T::one()) & h.lt(&T::from_f64(2.0));
//     let is_zone2 = h.gt_eq(&T::from_f64(2.0)) & h.lt(&T::from_f64(3.0));
//     let is_zone3 = h.gt_eq(&T::from_f64(3.0)) & h.lt(&T::from_f64(4.0));
//     let is_zone4 = h.gt_eq(&T::from_f64(4.0)) & h.lt(&T::from_f64(5.0));

//     let red = lazy_select! {
//         if is_zone1.clone() | &is_zone4 => x.clone(),
//         if is_zone2.clone() | &is_zone3 => T::zero(),
//         else => c.clone(),
//     };

//     let green = lazy_select! {
//         if is_zone0.clone() | &is_zone3 => x.clone(),
//         if is_zone1.clone() | &is_zone2 => c.clone(),
//         else => T::zero(),
//     };

//     let blue = lazy_select! {
//         if is_zone0 | is_zone1 => T::zero(),
//         if is_zone3 | is_zone4 => c,
//         else => x,
//     };

//     Rgb {
//         red: red + m.clone(),
//         green: green + m.clone(),
//         blue: blue + m,
//         standard: PhantomData,
//     }
// }

// pub fn generate_color(hash: &[u8]) -> Rgb<u8> {
//     // compute hash for hue space in larger bitspace
//     let hue_hash_1 = (hash[0] as u16 & 0x0f) << 8;
//     let hue_hash_2 = hash[1] as u16;
//     let hue_hash = (hue_hash_1 | hue_hash_2) as u32;

//     // compute hsl values
//     let hue = map_values(hue_hash, 0, 4095, 0, 360);
//     let saturation = map_values(hash[2] as u32, 0, 255, 50, 75) / 100.0;
//     let lightness = map_values(hash[3] as u32, 0, 255, 60, 70) / 100.0;

//     // convert color to rgb value
//     let color_hsl = Hsl::new(hue, saturation, lightness);
//     let color_rgb: Srgb<u8> = Srgb::from_color(color_hsl).into_format();
//     image::Rgb([color_rgb.red, color_rgb.green, color_rgb.blue])
// }

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

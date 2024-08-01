use crate::map_values::map_values;

/// RGB Color Struct
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct RGB {
    /// The RGB Red Value
    pub red: u8,

    /// The RGB Green Value
    pub green: u8,

    /// The RGB Blue Value
    pub blue: u8,
}

impl From<(u8, u8, u8)> for RGB {
    fn from(value: (u8, u8, u8)) -> Self {
        RGB {
            red: value.0,
            green: value.1,
            blue: value.2,
        }
    }
}

/// Theme Enum
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Theme {
    /// Generates a palette based on pre-selected colors
    Selection {
        /// A vector of input colors to choose from based on the input hash.
        /// This can be a vector of one value to allow for constant image colors.
        main: Vec<RGB>,

        /// A vector of background colors to choose from based on the input hash.
        /// This can be a vector of one value to allow for constant backgrounds.
        background: Vec<RGB>,
    },

    /// Generates a palette based on [HSL ranges](https://en.wikipedia.org/wiki/HSL_and_HSV)
    Range {
        /// The minimum hue
        /// A value between 0.0 and 360.0
        hue_min: f32,

        /// The maximum hue
        /// A value between 0.0 and 360.0
        hue_max: f32,

        /// The minimum saturation
        /// A value between 0.0 and 100.0 as a percent.
        /// e.g. 75.0 will become 0.750 in a HSL calculation.
        saturation_min: f32,

        /// The maximum saturation
        /// A value between 0.0 and 100.0 as a percent.
        /// e.g. 75.0 will become 0.750 in a HSL calculation.
        saturation_max: f32,

        /// The minimum lightness
        /// A value between 0.0 and 100.0 as a percent.
        /// e.g. 75.0 will become 0.750 in a HSL calculation.
        lightness_min: f32,

        /// The maximum lightness
        /// A value between 0.0 and 100.0 as a percent.
        /// e.g. 75.0 will become 0.750 in a HSL calculation.
        lightness_max: f32,

        /// A vector of background colors to choose from based on the input hash.
        /// This can be a vector of one value to allow for constant backgrounds.
        background: Vec<RGB>,
    },
}

impl Theme {
    /// Calculate the main image color.
    pub fn get_main_color(self, hash: &[u8]) -> RGB {
        match self {
            Theme::Selection {
                main,
                background: _,
            } => {
                let index = hash[0 % hash.len()] as usize % main.len();
                main[index]
            }
            Theme::Range {
                hue_min,
                hue_max,
                saturation_min,
                saturation_max,
                lightness_min,
                lightness_max,
                background: _,
            } => {
                // Compute hash for hue space in larger bitspace
                let hue_hash = ((hash[0 % hash.len()] as u16) << 8) | hash[1 % hash.len()] as u16;

                // Compute HSL values
                let hash_hue = map_values(hue_hash as f32, u16::MIN as f32, u16::MAX as f32, hue_min, hue_max);

                // Handle 0 degree hue is equivalent to 360 degree hue
                let hue = hash_hue % 360.0;

                // Saturation should be between 0.5 and 0.75 for pastel colors
                let saturation = map_values(
                    hash[2 % hash.len()] as f32,
                    u8::MIN as f32,
                    u8::MAX as f32,
                    saturation_min,
                    saturation_max,
                ) / 100.0;

                // Lightness should be between 0.6 and 0.70 for pastel colors
                let lightness = map_values(
                    hash[3 % hash.len()] as f32,
                    u8::MIN as f32,
                    u8::MAX as f32,
                    lightness_min,
                    lightness_max,
                ) / 100.0;

                // Convert HSL to RGB
                let chroma = (1.0 - ((2.0 * lightness) - 1.0).abs()) * saturation;
                let hue_prime = hue / 60.0;
                let x = chroma * (1.0 - ((hue_prime % 2.0) - 1.0).abs());

                // Get Prime RGB Values
                let (r_prime, g_prime, b_prime) = match hue_prime {
                    0.0..1.0 => (chroma, x, 0.0),
                    1.0..2.0 => (x, chroma, 0.0),
                    2.0..3.0 => (0.0, chroma, x),
                    3.0..4.0 => (0.0, x, chroma),
                    4.0..5.0 => (x, 0.0, chroma),
                    5.0..=6.0 => (chroma, 0.0, x),
                    // This should not occur as the hue is between 0 and 360, which casts down to between 0-6
                    _ => (0.0, 0.0, 0.0),
                };

                // Lightness modifier
                let m = lightness - chroma * 0.5;

                let red = (r_prime + m) * 255.0;
                let green = (g_prime + m) * 255.0;
                let blue = (b_prime + m) * 255.0;

                RGB {
                    red: red as u8,
                    green: green as u8,
                    blue: blue as u8,
                }
            }
        }
    }

    /// Calculate the background image color.
    pub fn get_background_color(self, hash: &[u8]) -> RGB {
        match self {
            Theme::Selection {
                main: _,
                background,
            } => {
                let index = hash[2 % hash.len()] as usize % background.len();
                background[index]
            }
            Theme::Range {
                hue_min: _,
                hue_max: _,
                saturation_min: _,
                saturation_max: _,
                lightness_min: _,
                lightness_max: _,
                background,
            } => {
                let index = hash[2 % hash.len()] as usize % background.len();
                background[index]
            }
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        default_theme()
    }
}

/// The default theme.
/// This is a muted pastel theme.
/// The original color theme, before theme customization existed.
pub fn default_theme() -> Theme {
    Theme::Range {
        hue_min: 0.0,
        hue_max: 360.0,
        saturation_min: 50.0,
        saturation_max: 75.0,
        lightness_min: 60.0,
        lightness_max: 70.0,
        background: vec![RGB {
            red: 240,
            green: 240,
            blue: 240,
        }],
    }
}

use error::ThemeError;

use crate::{color::RGB, map_values::map_values};

/// Theme Errors
///
/// Identicon Errors can wrap these errors
pub mod error;

/// Trait defining requirements for an identicon theme
pub trait Theme {
    /// This should return the main color within the identicon image
    fn main_color(&self, hash: &[u8]) -> Result<RGB, ThemeError>;

    /// This should return the background color within the identicon image
    fn background_color(&self, hash: &[u8]) -> Result<RGB, ThemeError>;
}

/// Simple selection theme struct
///
/// This will choose a predefined main color and background color based on the hash.
///
/// Both the main and background colors are defined as a `Vec<RGB>`.
///
/// Implements [Themey]
pub struct Selection {
    /// A vector of input colors to choose from based on the input hash.
    /// This can be a vector of one value to allow for constant image colors.
    main: Vec<RGB>,

    /// A vector of background colors to choose from based on the input hash.
    /// This can be a vector of one value to allow for constant backgrounds.
    background: Vec<RGB>,
}

impl Theme for Selection {
    fn main_color(&self, hash: &[u8]) -> Result<RGB, ThemeError> {
        if self.main.is_empty() {
            Err(ThemeError::ThemeValidationError(
                "main color selection is empty".to_string(),
            ))
        } else {
            let index = hash[0 % hash.len()] as usize % self.main.len();
            Ok(self.main[index])
        }
    }

    fn background_color(&self, hash: &[u8]) -> Result<RGB, ThemeError> {
        if self.background.is_empty() {
            Err(ThemeError::ThemeValidationError(
                "background color selection is empty".to_string(),
            ))
        } else {
            let index = hash[2 % hash.len()] as usize % self.background.len();
            Ok(self.background[index])
        }
    }
}

/// Complex HSL Range theme struct
///
/// This will generate a main color within the defined HSL Range.
///
/// The background color is based on a predefined `Vec<RGB>` and the color is selected by the hash value.
///
/// Implements [Themey]
pub struct HSLRange {
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
}

impl HSLRange {
    fn validate(&self) -> Result<(), ThemeError> {
        if self.hue_max < self.hue_min {
            Err(ThemeError::ThemeValidationError(
                "hue_max must be larger than hue_min".to_string(),
            ))
        } else if self.saturation_max < self.saturation_min {
            Err(ThemeError::ThemeValidationError(
                "saturation_max must be larger than saturation_min".to_string(),
            ))
        } else if self.lightness_max < self.lightness_min {
            Err(ThemeError::ThemeValidationError(
                "lightness_max must be larger than lightness_min".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

impl Theme for HSLRange {
    fn main_color(&self, hash: &[u8]) -> Result<RGB, ThemeError> {
        // Validate the fields
        self.validate()?;

        // Compute hash for hue space in larger bitspace
        let hue_hash = ((hash[0 % hash.len()] as u16) << 8) | hash[1 % hash.len()] as u16;

        // Compute HSL values
        let hash_hue = map_values(
            hue_hash as f32,
            u16::MIN as f32,
            u16::MAX as f32,
            self.hue_min,
            self.hue_max,
        );

        // Handle 0 degree hue is equivalent to 360 degree hue
        let hue = hash_hue % 360.0;

        // Saturation should be between 0.5 and 0.75 for pastel colors
        let saturation = map_values(
            hash[2 % hash.len()] as f32,
            u8::MIN as f32,
            u8::MAX as f32,
            self.saturation_min,
            self.saturation_max,
        ) / 100.0;

        // Lightness should be between 0.6 and 0.70 for pastel colors
        let lightness = map_values(
            hash[3 % hash.len()] as f32,
            u8::MIN as f32,
            u8::MAX as f32,
            self.lightness_min,
            self.lightness_max,
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

        Ok(RGB {
            red: red as u8,
            green: green as u8,
            blue: blue as u8,
        })
    }

    fn background_color(&self, hash: &[u8]) -> Result<RGB, ThemeError> {
        if self.background.is_empty() {
            Err(ThemeError::ThemeValidationError(
                "background color selection is empty".to_string(),
            ))
        } else {
            let index = hash[2 % hash.len()] as usize % self.background.len();
            Ok(self.background[index])
        }
    }
}

/// The default theme
///
/// This is a muted pastel theme.
/// The original color theme, before theme customization existed.
pub fn default_theme() -> crate::SharedPtr<dyn Theme> {
    crate::SharedPtr::new(HSLRange {
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
    })
}

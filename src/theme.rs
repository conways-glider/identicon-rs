/// RGB Color Struct
#[derive(Clone, PartialEq, PartialOrd, Debug)]
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
    },

    /// Generates a palette based on pre-selected colors
    Selection {
        /// A vector of background colors to choose from based on the input hash.
        /// This can be a vector of one value to allow for constant backgrounds.
        /// This should be in the format of RGB Values, ranging from 0 to
        background: Vec<RGB>,

        /// A vector of input colors to choose from based on the input hash.
        /// This can be a vector of one value to allow for constant image colors.
        main: Vec<RGB>,
    },
}

impl Default for Theme {
    fn default() -> Self {
        DEFAULT_THEME
    }
}

/// The default theme.
/// This is the original theme used by this library.
pub const DEFAULT_THEME: Theme = Theme::Range {
    hue_min: 0.0,
    hue_max: 360.0,
    saturation_min: 50.0,
    saturation_max: 75.0,
    lightness_min: 60.0,
    lightness_max: 70.0,
};

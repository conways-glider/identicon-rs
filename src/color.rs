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

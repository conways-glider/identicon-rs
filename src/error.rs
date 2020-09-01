use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum IdenticonError {
    SaveImageError,
    EncodeImageError,
    ScaleTooSmallError(u32),
    SizeTooLargeError(u32),
}

impl fmt::Display for IdenticonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            // Could use the `write!` macro here, but `f.write_str` is slightly faster
            IdenticonError::SaveImageError => f.write_str("could not save image"),
            IdenticonError::EncodeImageError => f.write_str("could not encode image"),
            IdenticonError::ScaleTooSmallError(size) => f.write_str(&format!(
                "identicon scale too small, must be greater or equal to {}",
                size
            )),
            IdenticonError::SizeTooLargeError(size) => f.write_str(&format!(
                "identicon size too large, must be lesser or equal to {}",
                size
            )),
        }
    }
}

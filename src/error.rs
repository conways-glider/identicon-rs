use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum IdenticonError {
    SaveImageError,
    EncodeImageError,
}

impl fmt::Display for IdenticonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            // Could use the `write!` macro here, but `f.write_str` is slightly faster
            IdenticonError::SaveImageError => f.write_str("could not save image"),
            IdenticonError::EncodeImageError => f.write_str("could not encode image"),
        }
    }
}

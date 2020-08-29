use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum IdenticonError {
    SaveImageError,
}

impl fmt::Display for IdenticonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IdenticonError::SaveImageError => write!(f, "could not save image"),
        }
    }
}

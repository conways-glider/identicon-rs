use thiserror::Error;

/// Theme Errors
///
/// These are provided for writing new types of themes using the Theme trait.
#[derive(Error, Debug)]
pub enum ThemeError {
    /// Theme failed validation
    #[error("theme validation failed: {0}")]
    ThemeValidationError(String),

    /// Theme failed to generate a color
    #[error("theme processing failed: {0}")]
    ThemeProcessingError(String),
}

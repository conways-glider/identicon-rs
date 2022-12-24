use thiserror::Error;

/// Identicon errors.
#[derive(Error, Debug)]
pub enum IdenticonError {
    /// Failed to generate the image.
    #[error("could not generate image")]
    GenerateImageError,

    /// Failed to save the image to a file.
    #[error("could not save image")]
    SaveImageError,

    /// Failed to encode the image.
    #[error("could not encode image")]
    EncodeImageError,

    /// Indicates an issue with using a scale smaller than the size.
    #[error(
        "identicon scale too small: {scale}, must be greater or equal to identicon size: {size}"
    )]
    ScaleTooSmallError {
        /// Attempted scale value.
        scale: u32,
        /// Currently set size value.
        size: u32,
    },

    /// Indicates an issues with using a size larger than the scale.
    #[error("identicon size too large: {size}, must be less or equal to identicon scale: {scale}")]
    SizeTooLargeError {
        /// Attempted size value.
        size: u32,
        /// Currently set scale value.
        scale: u32,
    },
}

#[cfg(test)]
mod tests {
    use crate::error::IdenticonError;

    #[test]
    fn generate_image_error_works() {
        let error = IdenticonError::GenerateImageError;
        let expected_text = "could not generate image";
        assert_eq!(expected_text, error.to_string());
    }

    #[test]
    fn save_image_error_works() {
        let error = IdenticonError::SaveImageError;
        let expected_text = "could not save image";
        assert_eq!(expected_text, error.to_string());
    }

    #[test]
    fn encode_image_error_works() {
        let error = IdenticonError::EncodeImageError;
        let expected_text = "could not encode image";
        assert_eq!(expected_text, error.to_string());
    }

    #[test]
    fn scale_too_small_error_works() {
        let error = IdenticonError::ScaleTooSmallError { scale: 3, size: 5 };
        let expected_text = "identicon scale too small: 3, must be greater or equal to identicon size: 5";
        assert_eq!(expected_text, error.to_string());
    }

    #[test]
    fn size_too_large_error_works() {
        let error = IdenticonError::SizeTooLargeError { size: 5, scale: 3 };
        let expected_text = "identicon size too large: 5, must be less or equal to identicon scale: 3";
        assert_eq!(expected_text, error.to_string());
    }
}

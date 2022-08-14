use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdenticonError {
    #[error("could not generate image")]
    GenerateImageError,
    #[error("could not save image")]
    SaveImageError,
    #[error("could not encode image")]
    EncodeImageError,
    #[error("identicon scale too small, must be greater or equal to {0}")]
    ScaleTooSmallError(u32),
    #[error("identicon scale too large, must be less or equal to {0}")]
    SizeTooLargeError(u32),
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
        let error = IdenticonError::ScaleTooSmallError(3);
        let expected_text = "identicon scale too small, must be greater or equal to 3";
        assert_eq!(expected_text, error.to_string());
    }

    #[test]
    fn size_too_large_error_works() {
        let error = IdenticonError::SizeTooLargeError(3);
        let expected_text = "identicon scale too large, must be less or equal to 3";
        assert_eq!(expected_text, error.to_string());
    }
}

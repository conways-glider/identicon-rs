use crate::error::IdenticonError;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImage, ImageBuffer, ImageEncoder};
use sha2::{Digest, Sha512};

mod color;
pub mod error;
mod grid;
mod map_values;

/// Generic Identicon struct
///
/// This is the base struct to be used.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Identicon {
    hash: Vec<u8>,
    border: u32,
    size: u32,
    scale: u32,
    background_color: (u8, u8, u8),
    mirrored: bool,
}

/// Generates a new identicon.
///
/// This is a wrapper around [`identicon_rs::Identicon::new`]
///
/// [`identicon_rs::Identicon::new`]: struct.Identicon.html#method.new
pub fn new<T>(input_value: T) -> Identicon
where
    T: AsRef<str>,
{
    Identicon::new(input_value)
}

impl Identicon {
    /// Generates a new identicon
    ///
    /// The defaults are:
    /// - border: 50
    /// - size: 5
    /// - scale: 500
    /// - background_color: (240, 240, 240)
    /// - mirrored: true
    pub fn new<T>(input_value: T) -> Self
    where
        T: AsRef<str>,
    {
        let hash = Identicon::hash_value(input_value.as_ref());
        let default_background_color = 240;

        Identicon {
            hash,
            border: 50,
            size: 5,
            scale: 500,
            background_color: (
                default_background_color,
                default_background_color,
                default_background_color,
            ),
            mirrored: true,
        }
    }

    /// Sets the identicon border size.
    ///
    /// Default is 5
    pub fn border(mut self, border: u32) -> Self {
        self.border = border;
        self
    }

    /// Sets the number of viewable blocks of the identicon.
    ///
    /// This must be <= the scale.
    ///
    /// Default is 5x5
    pub fn size(mut self, size: u32) -> Result<Self, IdenticonError> {
        if size <= self.scale {
            self.size = size;
            Ok(self)
        } else {
            Err(IdenticonError::SizeTooLargeError(self.scale))
        }
    }

    /// Sets the scale of the image.
    ///
    /// The scale plus 2 times the border is the final pixel size of the image.
    ///
    /// This must be >= the size.
    ///
    /// Default is 500
    pub fn scale(mut self, scale: u32) -> Result<Self, IdenticonError> {
        if scale >= self.size {
            self.scale = scale;
            Ok(self)
        } else {
            Err(IdenticonError::ScaleTooSmallError(self.size))
        }
    }

    /// Sets the background, non-active color of the identicon.
    ///
    /// This is a tuble of (red, green, blue) values.
    ///
    /// Default is (240, 240, 240)
    pub fn background_color(mut self, background_color: (u8, u8, u8)) -> Self {
        self.background_color = background_color;
        self
    }

    /// Sets whether the identicon is mirrored along the y axis.
    ///
    /// This is a boolean.
    ///
    /// Default is true
    pub fn mirrored(mut self, mirrored: bool) -> Self {
        self.mirrored = mirrored;
        self
    }

    fn hash_value(input_value: &str) -> Vec<u8> {
        let input_trimmed = input_value.trim();
        Sha512::digest(input_trimmed.as_bytes()).as_slice().to_vec()
    }

    /// Generates the DynamicImage representing the Identicon
    pub fn generate_image(&self) -> Result<DynamicImage, IdenticonError> {
        // create a new ImgBuf with width: imgx and height: imgy
        let mut image_buffer = ImageBuffer::new(self.size, self.size);

        // create a new grid
        let grid = grid::generate_full_grid(self.size, &self.hash);

        // create pixel objects
        let pixel_active = color::generate_color(&self.hash);
        let pixel_background = image::Rgb([
            self.background_color.0,
            self.background_color.1,
            self.background_color.2,
        ]);

        // iterate over the coordinates and pixels of the image
        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            // handles mirroring the x location
            let x_location = if self.mirrored && x > self.size / 2 {
                self.size - x - 1
            } else {
                x
            };

            // get location within the generated grid
            let grid_location = (x_location + y * self.size) % self.size.pow(2);

            // set the pixel color based on the value within the grid at the given position
            if grid[grid_location as usize] {
                *pixel = pixel_active;
            } else {
                *pixel = pixel_background;
            }
        }

        let scaled_image_buffer = DynamicImage::ImageRgb8(image_buffer)
            .resize(self.scale, self.scale, FilterType::Nearest)
            .to_rgb8();

        let final_size = self.scale + 2 * self.border;
        let mut bordered_image_buffer =
            ImageBuffer::from_fn(final_size, final_size, |_, _| pixel_background);

        match bordered_image_buffer.copy_from(&scaled_image_buffer, self.border, self.border) {
            Ok(_) => Ok(DynamicImage::ImageRgb8(bordered_image_buffer)),
            Err(_) => Err(error::IdenticonError::GenerateImageError),
        }
    }

    /// Saves the generated image to the given filename
    ///
    /// The file formats `.png`, `.jpg`, `.jpeg`, `.bmp`, and `.ico` work.
    pub fn save_image(&self, output_filename: &str) -> Result<(), error::IdenticonError> {
        let image = self.generate_image()?;
        image
            .save(output_filename)
            .map_err(|_| error::IdenticonError::SaveImageError)
    }

    /// Export a PNG file buffer as a Vec<u8>
    ///
    /// This is for creating a file for a buffer or network response without creating a file on the
    /// filesystem.
    pub fn export_png_data(&self) -> Result<Vec<u8>, error::IdenticonError> {
        let image = self.generate_image()?;
        let image_size = image.to_rgb8().width();
        let mut buffer = Vec::new();

        PngEncoder::new(&mut buffer)
            .write_image(
                image.to_rgb8().into_raw().as_slice(),
                image_size,
                image_size,
                image::ColorType::Rgb8,
            )
            .map_err(|_| error::IdenticonError::EncodeImageError)?;
        Ok(buffer)
    }

    /// Export a JPEG file buffer as a Vec<u8>
    ///
    /// This is for creating a file for a buffer or network response without creating a file on the
    /// filesystem.
    pub fn export_jpeg_data(&self) -> Result<Vec<u8>, error::IdenticonError> {
        let image = self.generate_image()?;
        let image_size = image.to_rgb8().width();
        let mut buffer = Vec::new();

        JpegEncoder::new(&mut buffer)
            .write_image(
                image.to_rgb8().into_raw().as_slice(),
                image_size,
                image_size,
                image::ColorType::Rgb8,
            )
            .map_err(|_| error::IdenticonError::EncodeImageError)?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use crate::Identicon;

    #[test]
    fn trim_of_input_works() {
        let image_normal = Identicon::new("test").generate_image().unwrap();
        let image_padded = Identicon::new("  test  ").generate_image().unwrap();
        assert_eq!(
            image_normal.to_rgb8().into_raw(),
            image_padded.to_rgb8().into_raw()
        );
    }

    #[test]
    fn trim_of_input_failure_works() {
        let image_normal = Identicon::new("test").generate_image().unwrap();
        let image_padded = Identicon::new("  test1  ").generate_image().unwrap();
        assert_ne!(
            image_normal.to_rgb8().into_raw(),
            image_padded.to_rgb8().into_raw()
        );
    }
}

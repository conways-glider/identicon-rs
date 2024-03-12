#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::str::FromStr;

use crate::error::IdenticonError;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImage, ImageBuffer, ImageEncoder};
use sha3::{Digest, Sha3_256};

mod color;

/// Identicon errors.
pub mod error;
mod grid;
mod map_values;

/// Generic Identicon struct.
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
/// This is a wrapper around [`identicon_rs::Identicon::new`].
///
/// [`identicon_rs::Identicon::new`]: struct.Identicon.html#method.new
pub fn new(input_value: &str) -> Identicon {
    Identicon::new(input_value)
}

impl Identicon {
    /// Generates a new identicon from an input value.
    ///
    /// The defaults are:
    /// - border: 50
    /// - size: 5
    /// - scale: 500
    /// - background_color: (240, 240, 240)
    /// - mirrored: true
    pub fn new(input_value: &str) -> Self {
        let mut identicon = Self::default();
        identicon.set_input(input_value);
        identicon
    }

    /// Sets the identicon input value, regenerating the hash.
    pub fn set_input(&mut self, input_value: &str) -> &mut Self {
        self.hash = Self::hash_value(input_value);
        self
    }

    /// Gets the identicon border size.
    pub fn border(&self) -> u32 {
        self.border
    }

    /// Sets the identicon border size.
    ///
    /// Default is 5
    pub fn set_border(&mut self, border: u32) -> &mut Self {
        self.border = border;
        self
    }

    /// Gets the identicon size.
    ///
    /// The size represents the number of viewable blocks of the identicon.
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Sets the number of viewable blocks of the identicon.
    ///
    /// This must be <= the scale.
    ///
    /// Default is 5, representing an identicon with a grid of 5x5.
    pub fn set_size(&mut self, size: u32) -> Result<&mut Self, IdenticonError> {
        if size <= self.scale {
            self.size = size;
            Ok(self)
        } else {
            Err(IdenticonError::SizeTooLargeError {
                size,
                scale: self.scale,
            })
        }
    }

    /// Gets the identicon scale.
    ///
    /// The scale represents the height and width of the identicon portion of any generated image.
    ///
    /// The full image size is: `scale + ( 2 * border )`
    pub fn scale(&self) -> u32 {
        self.scale
    }

    /// Sets the scale of the image.
    ///
    /// The full image size is: `scale + ( 2 * border )`
    ///
    /// This must be >= the size.
    pub fn set_scale(&mut self, scale: u32) -> Result<&mut Self, IdenticonError> {
        if scale >= self.size {
            self.scale = scale;
            Ok(self)
        } else {
            Err(IdenticonError::ScaleTooSmallError {
                scale,
                size: self.size,
            })
        }
    }

    /// Gets the identicon background color.
    pub fn background_color(&self) -> (u8, u8, u8) {
        self.background_color
    }

    /// Sets the background, non-active color of the identicon.
    ///
    /// This is a tuble of (red, green, blue) values.
    pub fn set_background_color(&mut self, background_color: (u8, u8, u8)) -> &mut Self {
        self.background_color = background_color;
        self
    }

    /// Gets if the identicon is mirrored.
    pub fn mirrored(&self) -> bool {
        self.mirrored
    }

    /// Sets whether the identicon is mirrored along the y axis.
    ///
    /// This is a boolean.
    pub fn set_mirrored(&mut self, mirrored: bool) -> &mut Self {
        self.mirrored = mirrored;
        self
    }

    fn hash_value(input_value: &str) -> Vec<u8> {
        let input_trimmed = input_value.trim();
        Sha3_256::digest(input_trimmed.as_bytes())
            .as_slice()
            .to_vec()
    }

    /// Generates the DynamicImage representing the Identicon.
    pub fn generate_image(&self) -> Result<DynamicImage, IdenticonError> {
        // Create a new grid
        let grid = grid::generate_full_grid(self.size, &self.hash);

        // Create pixel objects
        let color_active = color::generate_color(&self.hash);
        let pixel_active = image::Rgb([color_active.0, color_active.1, color_active.2]);
        let pixel_background = image::Rgb([
            self.background_color.0,
            self.background_color.1,
            self.background_color.2,
        ]);

        // Create image buffer from grid
        let image_buffer = ImageBuffer::from_fn(self.size, self.size, |x, y| {
            let x_location = if self.mirrored && x > self.size / 2 {
                self.size - x - 1
            } else {
                x
            };

            // Get location within the generated grid
            let grid_location = (x_location + y * self.size) % self.size.pow(2);

            // Set the pixel color based on the value within the grid at the given position
            if grid[grid_location as usize] {
                pixel_active
            } else {
                pixel_background
            }
        });

        let scaled_image_buffer = DynamicImage::ImageRgb8(image_buffer)
            .resize(self.scale, self.scale, FilterType::Nearest)
            .to_rgb8();

        let final_size = self.scale + (2 * self.border);
        let mut bordered_image_buffer =
            ImageBuffer::from_fn(final_size, final_size, |_, _| pixel_background);

        match bordered_image_buffer.copy_from(&scaled_image_buffer, self.border, self.border) {
            Ok(_) => Ok(DynamicImage::ImageRgb8(bordered_image_buffer)),
            Err(_) => Err(error::IdenticonError::GenerateImageError),
        }
    }

    /// Saves the generated image to the given filename.
    ///
    /// The file formats `.png`, `.jpg`, `.jpeg`, `.bmp`, and `.ico` work.
    pub fn save_image(&self, output_filename: &str) -> Result<(), error::IdenticonError> {
        let image = self.generate_image()?;
        image
            .save(output_filename)
            .map_err(|_| error::IdenticonError::SaveImageError)
    }

    /// Export a PNG file buffer as a Vec<u8>.
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

    /// Export a JPEG file buffer as a Vec<u8>.
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

impl Default for Identicon {
    fn default() -> Self {
        let default_background_color = 240;
        Self {
            hash: Self::hash_value(""),
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
}

impl FromStr for Identicon {
    type Err = IdenticonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

#[cfg(test)]
mod tests {
    use crate::Identicon;

    #[test]
    fn consistency() {
        let expected_color = (111, 212, 172);
        let expected_grid = vec![
            true, true, true, true, false, true, true, true, false, true, true, true, false, true,
            true, false, true, true, true, true, true, true, false, true, true,
        ];

        let image = Identicon::new("test");
        let grid = crate::grid::generate_full_grid(image.size, &image.hash);
        let color = crate::color::generate_color(&image.hash);

        assert_eq!(expected_color, color);

        assert_eq!(expected_grid, grid);
    }

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

    #[test]
    fn chained_setters_work() {
        let identicon_chained = Identicon::new("test")
            .set_border(10)
            .set_background_color((0, 0, 0))
            .clone();

        let mut identicon_mutated = Identicon::new("test");
        identicon_mutated.set_border(10);
        identicon_mutated.set_background_color((0, 0, 0));

        assert_eq!(identicon_chained, identicon_mutated);
    }

    #[test]
    fn getters_work() {
        let identicon = Identicon::new("test")
            .set_border(10)
            .set_background_color((0, 0, 0))
            .clone();

        assert_eq!(identicon.border(), identicon.border);
    }

    #[test]
    fn from_str_works() {
        let identicon = Identicon::new("test");
        let identicon_from_str = "test".parse::<Identicon>().unwrap();
        assert_eq!(identicon, identicon_from_str);
    }

    #[test]
    fn from_str_failure_works() {
        let identicon = Identicon::new("test");
        let identicon_from_str = "test1".parse::<Identicon>().unwrap();
        assert_ne!(identicon, identicon_from_str);
    }
}

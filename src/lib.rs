use image::{jpeg::JPEGEncoder, png::PNGEncoder, DynamicImage, ImageBuffer};
use sha2::{Digest, Sha512};

use palette::LinSrgb;

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
}

impl Identicon {
    /// Generates a new identicon with all given parameters
    ///
    /// The defaults are:
    /// - border: 50
    /// - size: 5
    /// - scale: 500
    /// - background_color: (240, 240, 240)
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
        }
    }

    pub fn border(mut self, border: u32) -> Self {
        self.border = border;
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    pub fn scale(mut self, scale: u32) -> Self {
        self.scale = scale;
        self
    }

    pub fn background_color(mut self, background_color: (u8, u8, u8)) -> Self {
        self.background_color = background_color;
        self
    }

    fn hash_value(input_value: &str) -> Vec<u8> {
        let input_trimmed = input_value.trim();
        Sha512::digest(input_trimmed.as_bytes()).as_slice().to_vec()
    }

    /// Generates the DynamicImage representing the Identicon
    pub fn generate_image(&self) -> DynamicImage {
        let size = self.scale + self.border * 2;

        // create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf = ImageBuffer::new(size, size);

        // create a new grid
        let grid = grid::generate_full_grid(self.size, &self.hash);

        // create a clojure to check whether the given location is within the border space
        let check_within_border = |location: u32| -> bool {
            location < self.border || location >= self.border + self.scale
        };

        // create pixel objects
        let color = color::generate_color(&self.hash);
        let background_color = LinSrgb::new(
            self.background_color.0,
            self.background_color.1,
            self.background_color.2,
        );
        let pixel_active = image::Rgb([color.red, color.green, color.blue]);
        let pixel_background = image::Rgb([
            background_color.red,
            background_color.green,
            background_color.blue,
        ]);

        // iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            if check_within_border(x) || check_within_border(y) {
                *pixel = pixel_background;
            } else {
                // get location within the generated grid
                let location_scale = self.scale / self.size;
                let x_location = (x - self.border) / location_scale;
                let y_location = (y - self.border) / location_scale;
                let grid_location = (x_location + y_location * self.size) % self.size.pow(2);

                // set the pixel color based on the value within the grid at the given position
                if grid[grid_location as usize] {
                    *pixel = pixel_active;
                } else {
                    *pixel = pixel_background;
                }
            }
        }

        DynamicImage::ImageRgb8(imgbuf)
    }

    /// Saves the generated image to the given filename
    ///
    /// The file formats `.png`, `.jpg`, `.jpeg`, `.bmp`, and `.ico` work.
    pub fn save_image(&self, output_filename: &str) -> Result<(), error::IdenticonError> {
        self.generate_image()
            .save(output_filename)
            .map_err(|_| error::IdenticonError::SaveImageError)
    }

    /// Export a PNG file buffer as a Vec<u8>
    ///
    /// This is for creating a file for a buffer or network response without creating a file on the
    /// filesystem.
    pub fn export_png_data(&self) -> Result<Vec<u8>, error::IdenticonError> {
        let image = self.generate_image();
        let image_size = image.to_rgb().width();
        let mut file = Vec::new();

        PNGEncoder::new(&mut file)
            .encode(
                image.to_rgb().into_raw().as_slice(),
                image_size,
                image_size,
                image::ColorType::Rgb8,
            )
            .map_err(|_| error::IdenticonError::SaveImageError)?;
        Ok(file)
    }

    /// Export a JPEG file buffer as a Vec<u8>
    ///
    /// This is for creating a file for a buffer or network response without creating a file on the
    /// filesystem.
    pub fn export_jpeg_data(&self) -> Result<Vec<u8>, error::IdenticonError> {
        let image = self.generate_image();
        let image_size = image.to_rgb().width();
        let mut file = Vec::new();

        JPEGEncoder::new(&mut file)
            .encode(
                image.to_rgb().into_raw().as_slice(),
                image_size,
                image_size,
                image::ColorType::Rgb8,
            )
            .map_err(|_| error::IdenticonError::SaveImageError)?;
        Ok(file)
    }
}

#[cfg(test)]
mod tests {
    use crate::Identicon;

    #[test]
    fn trim_of_input_works() {
        let image_normal = Identicon::new("test").generate_image();
        let image_padded = Identicon::new("  test  ").generate_image();
        assert_eq!(
            image_normal.to_rgb().into_raw(),
            image_padded.to_rgb().into_raw()
        );
    }

    #[test]
    fn trim_of_input_failure_works() {
        let image_normal = Identicon::new("test").generate_image();
        let image_padded = Identicon::new("  test1  ").generate_image();
        assert_ne!(
            image_normal.to_rgb().into_raw(),
            image_padded.to_rgb().into_raw()
        );
    }
}

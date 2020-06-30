use image::{jpeg::JPEGEncoder, png::PNGEncoder, DynamicImage, ImageBuffer};
use sha2::{Digest, Sha512};

use palette::LinSrgb;

mod color;
mod grid;
mod map_values;

/// Represents available image types
pub enum ImageType {
    /// PNG file type
    PNG,
    /// JPEG or JPG file type
    JPEG,
}

/// Generic Identicon struct
///
/// This is the base struct to be used.
pub struct Identicon {
    hash: Vec<u8>,
    border: u32,
    size: u32,
    scale: u32,
    background_color: (u8, u8, u8),
}

impl Identicon {
    /// Generates a new identicon with all given parameters
    pub fn new(
        input_value: &str,
        border: u32,
        size: u32,
        scale: u32,
        background_color: (u8, u8, u8),
    ) -> Identicon {
        let hash = Identicon::get_hash_value(input_value);

        Identicon {
            hash,
            border,
            size,
            scale,
            background_color,
        }
    }

    /// Generates a new identicon with base library defaults
    ///
    /// The defaults are:
    /// - border: 50
    /// - size: 5
    /// - scale: 500
    /// - background_color: (240, 240, 240)
    pub fn new_default(input_value: &str) -> Identicon {
        let hash = Identicon::get_hash_value(input_value);
        let background_color = 240;

        Identicon {
            hash,
            border: 50,
            size: 5,
            scale: 500,
            background_color: (background_color, background_color, background_color),
        }
    }

    /// Generates a new identicon with the defaults with no border
    ///
    /// The defaults are:
    /// - border: 0
    /// - size: 5
    /// - scale: 500
    /// - background_color: (240, 240, 240)
    pub fn new_no_border(input_value: &str) -> Identicon {
        let hash = Identicon::get_hash_value(input_value);
        let background_color = 240;

        Identicon {
            hash,
            border: 0,
            size: 5,
            scale: 500,
            background_color: (background_color, background_color, background_color),
        }
    }

    fn get_hash_value(input_value: &str) -> Vec<u8> {
        let input_trimmed = input_value.trim();
        Sha512::digest(input_trimmed.as_bytes()).as_slice().to_vec()
    }

    fn get_background_color(&self) -> LinSrgb<u8> {
        let background_color = self.background_color;
        LinSrgb::new(background_color.0, background_color.1, background_color.2)
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
        let background_color = self.get_background_color();
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
    pub fn save_image(&self, output_filename: &str) {
        self.generate_image().save(output_filename).unwrap();
    }

    /// Export the file buffer as a Vec<u8>
    ///
    /// This is for creating a file for a buffer or network response without creating a file on the
    /// filesystem.
    pub fn export_file_data(&self, image_type: ImageType) -> Vec<u8> {
        let image = self.generate_image();
        let image_size = image.to_rgb().width();
        let mut file = Vec::new();

        match image_type {
            ImageType::PNG => PNGEncoder::new(&mut file)
                .encode(
                    image.to_rgb().into_raw().as_slice(),
                    image_size,
                    image_size,
                    image::RGB(8),
                )
                .unwrap(),
            ImageType::JPEG => JPEGEncoder::new(&mut file)
                .encode(
                    image.to_rgb().into_raw().as_slice(),
                    image_size,
                    image_size,
                    image::RGB(8),
                )
                .unwrap(),
        }
        file
    }
}

#[cfg(test)]
mod tests {
    use crate::Identicon;

    #[test]
    fn trim_of_input_works() {
        let image_normal = Identicon::new_default("test").generate_image();
        let image_padded = Identicon::new_default("  test  ").generate_image();
        assert_eq!(
            image_normal.to_rgb().into_raw(),
            image_padded.to_rgb().into_raw()
        );
    }

    #[test]
    fn trim_of_input_failure_works() {
        let image_normal = Identicon::new_default("test").generate_image();
        let image_padded = Identicon::new_default("  test1  ").generate_image();
        assert_ne!(
            image_normal.to_rgb().into_raw(),
            image_padded.to_rgb().into_raw()
        );
    }
}

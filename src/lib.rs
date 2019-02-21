use image::{DynamicImage, FilterType, ImageBuffer};
use sha2::{Digest, Sha512};

use palette::LinSrgb;

mod color;
mod grid;
mod map_values;

pub enum Symmetry {
    None,
    X,
    Y
}

pub struct Identicon {
    hash: Vec<u8>,
    symmetry: Symmetry,
    border: u8,
    size: u8,
    scale: u32,
    background_color: (u8, u8, u8)
}

impl Identicon {
    pub fn new(input_value: &str, symmetry: Symmetry, border: u8, size: u8, scale: u32, background_color: (u8, u8, u8)) -> Identicon {
        let input_trimmed = input_value.trim();
        let hash = Sha512::digest(input_trimmed.as_bytes()).as_slice().to_vec();

        Identicon {
            hash,
            symmetry,
            border,
            size,
            scale,
            background_color
        }
    }

    pub fn new_default(input_value: &str) -> Identicon {
        let input_trimmed = input_value.trim();
        let hash = Sha512::digest(input_trimmed.as_bytes()).as_slice().to_vec();
        let background_color = 240;

        Identicon {
            hash,
            symmetry: Symmetry::None,
            border: 1,
            size: 5,
            scale: 500,
            background_color: (background_color, background_color, background_color)
        }
    }

    fn get_background_color(&self) -> LinSrgb<u8> {
        let background_color = self.background_color;
        LinSrgb::new(background_color.0, background_color.1, background_color.2)
    }

    pub fn generate_image(&self) -> DynamicImage {
        let background_color = self.get_background_color();

        let color = color::generate_color(&self.hash);

        // create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf = ImageBuffer::new(self.size as u32, self.size as u32);

        // create a new grid
        let grid = grid::generate_full_grid(self.size, &self.hash);

        // iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let count = x + y * self.size as u32;

            if grid[count as usize] {
                *pixel = image::Rgb([color.red, color.green, color.blue]);
            } else {
                *pixel = image::Rgb([
                                    background_color.red,
                                    background_color.green,
                                    background_color.blue,
                ]);
            }
        }

        // return the image
        DynamicImage::ImageRgb8(imgbuf)
    }

    pub fn save_image(&self, output_filename: &str) {
        self.generate_image().resize(self.scale, self.scale, FilterType::Nearest).save(output_filename).unwrap();
    }
}

// pub fn generate_scaled_image(input_value: &str, scale: u32) -> DynamicImage {
//     generate_image(input_value).resize(scale, scale, FilterType::Nearest)
// }
//
// pub fn generate_bordered_image(input_value: &str, scale: u32, border_width: u32) -> DynamicImage {
//     let background_color = get_background_color();
//     let base_image = generate_scaled_image(input_value, scale).to_rgb();
//
//     let image_size = base_image.width() + (border_width * 2);
//
//     // create a new ImgBuf with width: imgx and height: imgy
//     let mut imgbuf = ImageBuffer::new(image_size, image_size);
//
//     // create a clojure to check whether the given location is within the border space
//     let check_within_border =
//         |location: u32| -> bool { location < border_width || location >= border_width + scale };
//
//     // iterate over the coordinates and pixels of the image
//     for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//         if check_within_border(x) || check_within_border(y) {
//             *pixel = image::Rgb([
//                 background_color.red,
//                 background_color.green,
//                 background_color.blue,
//             ]);
//         } else {
//             *pixel = base_image
//                 .get_pixel(x - border_width, y - border_width)
//                 .clone();
//         }
//     }
//
//     DynamicImage::ImageRgb8(imgbuf)
// }
//
// pub fn save_scaled_image(input_value: &str, output_filename: &str, scale: u32) {
//     generate_scaled_image(input_value, scale)
//         .save(output_filename)
//         .unwrap();
// }
//
// pub fn save_bordered_image(
//     input_value: &str,
//     output_filename: &str,
//     scale: u32,
//     border_width: u32,
// ) {
//     generate_bordered_image(input_value, scale, border_width)
//         .save(output_filename)
//         .unwrap();
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn trim_of_input_works() {
    //     let image_normal = crate::generate_image("test");
    //     let image_padded = crate::generate_image("  test  ");
    //     assert_eq!(
    //         image_normal.to_rgb().into_raw(),
    //         image_padded.to_rgb().into_raw()
    //     );
    // }
    //
    // #[test]
    // fn trim_of_input_failure_works() {
    //     let image_normal = crate::generate_image("test");
    //     let image_padded = crate::generate_image("  test1  ");
    //     assert_ne!(
    //         image_normal.to_rgb().into_raw(),
    //         image_padded.to_rgb().into_raw()
    //     );
    // }
}

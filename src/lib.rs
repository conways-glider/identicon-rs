// use sha2::{
//     Sha512,
//     Sha512Trunc256,
//     Digest
// };

use image::{
    ImageBuffer,
    DynamicImage
};

use palette::{
    LinSrgb
};

mod color;
mod grid;

const IMAGE_SIZE: u32 = 5;

/// Returns a DynamicImage based on the input given
///
/// # Arguments
///
/// * `input_value` - A string that will be used for the hashing
///
/// # Example
///
/// ```
/// use identicon::generate_image;
/// let image_buffer = generate_image("test name");
/// ```
pub fn generate_image(input_value: &str) -> DynamicImage {

    let imgx = IMAGE_SIZE;
    let imgy = IMAGE_SIZE;
    let background_color: LinSrgb<u8> = LinSrgb::new(245,245,245);

    // Compute the hash value
    // let hash = Sha512::digest(input_value.as_bytes());
    // let hash_slice = hash.as_slice();

    // Compute the color values
    let color = color::generate_color(input_value);
    println!("{:?}", color);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    // Create a new grid
    let grid = grid::generate_full_grid(IMAGE_SIZE as usize, input_value);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let count = x + y * IMAGE_SIZE;

        if grid[count as usize] {
        *pixel = image::Rgb([color.red, color.green, color.blue]);
        } else {
            *pixel = image::Rgb([background_color.red, background_color.green, background_color.blue]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    DynamicImage::ImageRgb8(imgbuf)
}

/// This is a wrapper around generate_image to directly save the file to the local filesystem
///
/// # Arguments
///
/// * `input_value` - A string that will be used for the hashing
/// * `output_filename` - A string that will be the name of the output file
///
/// # Example
/// ```
/// use identicon::save_image;
/// save_image("test name", "output.png")
/// ```
pub fn save_image(input_value: &str, output_filename: &str) {
    generate_image(input_value).save(output_filename).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use sha2::{
    Sha512,
    Digest
};

use image::{
    ImageBuffer,
    DynamicImage
};

const IMAGE_SIZE: u32 = 8;

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

    let hash = Sha512::new()
        .chain(input_value)
        .result();

    println!("{:?}", hash);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = 32 * x as u8;
        let g = (32 * (-1 * (x as i32) + imgx as i32) as i32) as u8;
        let b = 32 * y as u8;
        *pixel = image::Rgb([r, g, b]);
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

use image::{
    ImageBuffer,
    DynamicImage
};

use palette::{
    LinSrgb
};

mod color;
mod grid;

pub fn generate_image(input_value: &str) -> DynamicImage {
    let img_size = 5;
    let background_color: LinSrgb<u8> = LinSrgb::new(245,245,245);

    // Compute the color values
    let color = color::generate_color(input_value);
    println!("{:?}", color);

    // create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = ImageBuffer::new(img_size, img_size);

    // create a new grid
    let grid = grid::generate_full_grid(img_size as usize, input_value);

    // iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let count = x + y * img_size;

        if grid[count as usize] {
        *pixel = image::Rgb([color.red, color.green, color.blue]);
        } else {
            *pixel = image::Rgb([background_color.red, background_color.green, background_color.blue]);
        }
    }

    // return the image
    DynamicImage::ImageRgb8(imgbuf)
}

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

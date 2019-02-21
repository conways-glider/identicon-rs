use image::{
    ImageBuffer,
    DynamicImage,
    FilterType
};

use palette::{
    LinSrgb
};

mod color;
mod grid;
mod map_values;

const BACKGROUND_COLOR: u8 = 240;

fn get_background_color() -> LinSrgb<u8> {
    LinSrgb::new(
        BACKGROUND_COLOR,
        BACKGROUND_COLOR,
        BACKGROUND_COLOR
    )
}

pub fn generate_image(input_value: &str) -> DynamicImage {
    let image_size = 5;
    let background_color = get_background_color();
    let input_trimmed = input_value.trim();

    let color = color::generate_color(input_trimmed);

    // create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = ImageBuffer::new(image_size, image_size);

    // create a new grid
    let grid = grid::generate_full_grid(image_size, input_trimmed);

    // iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let count = x + y * image_size;

        if grid[count as usize] {
            *pixel = image::Rgb([color.red, color.green, color.blue]);
        } else {
            *pixel = image::Rgb([background_color.red, background_color.green, background_color.blue]);
        }
    }

    // return the image
    DynamicImage::ImageRgb8(imgbuf)
}

pub fn generate_scaled_image(input_value: &str, scale: u32) -> DynamicImage {
    generate_image(input_value)
        .resize(scale, scale, FilterType::Nearest)
}

pub fn generate_bordered_image(input_value: &str, scale: u32, border_width: u32) -> DynamicImage {
    let background_color = get_background_color();
    let base_image = generate_scaled_image(input_value, scale).to_rgb();

    let image_size = base_image.width() + (border_width * 2);

    // create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = ImageBuffer::new(image_size, image_size);

    // create a clojure to check whether the given location is within the border space
    let check_within_border = |location: u32| -> bool {
        location < border_width || location >= border_width + scale
    };

    // iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if check_within_border(x) || check_within_border(y) {
            *pixel = image::Rgb([background_color.red, background_color.green, background_color.blue]);
        } else {
            *pixel = base_image.get_pixel(x - border_width, y - border_width).clone();
        }
    }

    DynamicImage::ImageRgb8(imgbuf)
}

pub fn save_image(input_value: &str, output_filename: &str) {
    generate_image(input_value)
        .save(output_filename)
        .unwrap();
}

pub fn save_scaled_image(input_value: &str, output_filename: &str, scale: u32) {
    generate_scaled_image(input_value, scale)
        .save(output_filename)
        .unwrap();
}

pub fn save_bordered_image(input_value: &str, output_filename: &str, scale: u32, border_width: u32) {
    generate_bordered_image(input_value, scale, border_width)
        .save(output_filename)
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn trim_of_input_works() {
        let image_normal = crate::generate_image("test");
        let image_padded = crate::generate_image("  test  ");
        assert_eq!(image_normal.to_rgb().into_raw(), image_padded.to_rgb().into_raw());
    }

    #[test]
    fn trim_of_input_failure_works() {
        let image_normal = crate::generate_image("test");
        let image_padded = crate::generate_image("  test1  ");
        assert_ne!(image_normal.to_rgb().into_raw(), image_padded.to_rgb().into_raw());
    }
}

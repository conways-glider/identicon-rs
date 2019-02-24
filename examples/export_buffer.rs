use identicon_rs::Identicon;
use image::GenericImageView;

fn main() {
    let fluffy_samurai = "fluffy-samurai";
    let test_string = "identicon_rs";

    // stored example
    let identicon_fluffy = Identicon::new_default(fluffy_samurai);
    identicon_fluffy.save_image("output_1.png");
    let identicon_fluffy_image = identicon_fluffy.generate_image();

    // export buffer
    let mut file = Vec::new();
    let encoded = image::png::PNGEncoder::new(&mut file)
        .encode(
            identicon_fluffy_image.to_rgb().into_raw().as_slice(),
            identicon_fluffy_image.width(),
            identicon_fluffy_image.height(),
            image::RGB(8),
        )
        .unwrap();
    println!("{:?}", file);

    // chained example with no border
    Identicon::new_no_border(test_string).save_image("output_2.png");
}

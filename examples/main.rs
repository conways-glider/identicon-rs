use identicon_rs::Identicon;

fn main() {
    let fluffy_samurai = "fluffy-samurai";
    let test_string = "identicon_rs";
    let identicon_fluffy = Identicon::new_default(fluffy_samurai);
    identicon_fluffy.save_image("output_1.png");
    // identicon_rs::save_scaled_image(fluffy_samurai, "output_1.png", 500);
    // identicon_rs::save_scaled_image(test_string, "output_2.png", 500);

    // identicon_rs::save_bordered_image(fluffy_samurai, "output_1_bordered.png", 500, 50);
    // identicon_rs::save_bordered_image(test_string, "output_2_bordered.png", 500, 50);
}

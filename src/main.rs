use identicon_rs;

fn main() {
    identicon_rs::save_scaled_image("fluffy-samurai", "output_1.png", 500);
    identicon_rs::save_scaled_image("identicon", "output_2.png", 500);

    identicon_rs::save_bordered_image("fluffy-samurai", "output_1_bordered.png", 500, 50);
    identicon_rs::save_bordered_image("identicon", "output_2_bordered.png", 500, 50);
}

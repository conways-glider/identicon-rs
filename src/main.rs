use identicon;

fn main() {
    identicon::save_scaled_image("fluffy-samurai", "output_1.png", 500);
    identicon::save_scaled_image("identicon", "output_2.png", 500);

    identicon::save_bordered_image("fluffy-samurai", "output_1_bordered.png", 500, 50);
    identicon::save_bordered_image("identicon", "output_2_bordered.png", 500, 50);
}

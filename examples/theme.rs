use std::sync::Arc;

use identicon_rs::color::RGB;
use identicon_rs::error::IdenticonError;
use identicon_rs::theme::HSLRange;
use identicon_rs::Identicon;

fn main() -> Result<(), IdenticonError> {
    let identicon_theme = HSLRange::new(
        0.0,
        360.0,
        50.0,
        90.0,
        40.0,
        60.0,
        vec![RGB {
            red: 240,
            green: 240,
            blue: 240,
        }],
    )?;

    let identicon_theme = Arc::new(identicon_theme);

    let conways_glider = String::from("conways-glider");
    let test_string = "identicon_rs";

    // Stored example
    let mut identicon_conways_glider = Identicon::new(&conways_glider);
    identicon_conways_glider.set_theme(identicon_theme.clone());
    identicon_conways_glider.save_image("output_1.png")?;

    // Chained example with no border
    Identicon::new(test_string)
        .set_border(0)
        .set_theme(identicon_theme.clone())
        .save_image("output_2.png")?;
    Ok(())
}

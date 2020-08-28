use identicon_rs::Identicon;
use std::io;

fn main() -> Result<(), io::Error> {
    let fluffy_samurai = String::from("fluffy-samurai");
    let test_string = "identicon_rs";

    // stored example
    let identicon_fluffy = Identicon::new(&fluffy_samurai);
    identicon_fluffy.save_image("output_1.png")?;

    // chained example with no border
    Identicon::new(test_string)
        .border(0)
        .save_image("output_2.png")?;
    Ok(())
}

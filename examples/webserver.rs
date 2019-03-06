use actix_web::{server, App, HttpRequest, HttpResponse};
use identicon_rs::{Identicon, ImageType};

fn generate_png(req: &HttpRequest) -> HttpResponse {
    let input_string: String = req.match_info().query("input_string").unwrap();
    let identicon_fluffy = Identicon::new_default(&input_string);
    let file = identicon_fluffy.export_file_data(ImageType::PNG);

    HttpResponse::Ok().content_type("image/png").body(file)
}

fn generate_jpeg(req: &HttpRequest) -> HttpResponse {
    let input_string: String = req.match_info().query("input_string").unwrap();
    let identicon_fluffy = Identicon::new_default(&input_string);
    let file = identicon_fluffy.export_file_data(ImageType::JPEG);

    HttpResponse::Ok().content_type("image/jpeg").body(file)
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/{input_string}.jpeg", |r| r.f(generate_jpeg))
            .resource("/{input_string}.jpg", |r| r.f(generate_jpeg))
            .resource("/{input_string}.png", |r| r.f(generate_png))
            .resource("/{input_string}", |r| r.f(generate_png))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();
}

use actix_web::{server, App, HttpRequest, HttpResponse};
use identicon_rs::Identicon;

fn index(req: &HttpRequest) -> HttpResponse {
    let input_string: String = req.match_info().query("input_string").unwrap();
    let identicon_fluffy = Identicon::new_default(&input_string[..]);
    let file = identicon_fluffy.export_file_data();

    HttpResponse::Ok().content_type("image/png").body(file)
}

fn main() {
    server::new(|| App::new().resource("/{input_string}", |r| r.f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}

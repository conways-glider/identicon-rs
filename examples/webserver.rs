use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use identicon_rs::{Identicon, ImageType};

fn index(path_input: web::Path<String>) -> impl Responder {
    let identicon = Identicon::new_default(&path_input);
    let file = identicon.export_file_data(ImageType::PNG);

    HttpResponse::Ok().content_type("image/png").body(file)
}

fn generate_jpeg(path_input: web::Path<String>) -> impl Responder {
    let identicon = Identicon::new_default(&path_input);
    let file = identicon.export_file_data(ImageType::JPEG);

    HttpResponse::Ok().content_type("image/jpeg").body(file)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/{input_string}.jpeg", |r| r.f(generate_jpeg))
            .route("/{input_string}.jpg", |r| r.f(generate_jpeg))
            .route("/{input_string}.png", |r| r.f(generate_png))
            .route("/{input_string}", web::get().to(generate_png))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}

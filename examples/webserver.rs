use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use identicon_rs::Identicon;
use std::io;

fn generate_png(path_input: web::Path<String>) -> Result<impl Responder, io::Error> {
    let identicon = Identicon::new(path_input.as_str());
    let file = identicon.export_png_data()?;

    Ok(HttpResponse::Ok().content_type("image/png").body(file))
}

fn generate_jpeg(path_input: web::Path<String>) -> Result<impl Responder, io::Error> {
    let identicon = Identicon::new(path_input.as_str());
    let file = identicon.export_jpeg_data()?;

    Ok(HttpResponse::Ok().content_type("image/jpeg").body(file))
}

fn main() {
    let address = "127.0.0.1:8088";
    println!(
        "Navigate to http://{}/{{input_string}} to see the image",
        address
    );

    HttpServer::new(|| {
        App::new()
            .route("/{input_string}.jpeg", web::get().to(generate_jpeg))
            .route("/{input_string}.jpg", web::get().to(generate_jpeg))
            .route("/{input_string}.png", web::get().to(generate_png))
            .route("/{input_string}", web::get().to(generate_png))
    })
    .bind(address)
    .expect("Cannot bind to address")
    .run()
    .unwrap();
}

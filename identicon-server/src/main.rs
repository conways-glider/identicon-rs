use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use identicon_rs::Identicon;

#[get("/{name}.png")]
async fn generate_png(path: web::Path<String>) -> impl Responder {
    let identicon_string = path.into_inner();
    let identicon = Identicon::new(identicon_string);
    let file = identicon.export_png_data().unwrap();

    HttpResponse::Ok().content_type("image/png").body(file)
}

#[get("/{name}.jpg")]
async fn generate_jpeg(path: web::Path<String>) -> impl Responder {
    let identicon_string = path.into_inner();
    let identicon = Identicon::new(identicon_string);
    let file = identicon.export_jpeg_data().unwrap();

    HttpResponse::Ok().content_type("image/jpeg").body(file)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(generate_png).service(generate_jpeg))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

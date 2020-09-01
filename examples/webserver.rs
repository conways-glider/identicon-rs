use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use identicon_rs::Identicon;

async fn generate_png(req: HttpRequest) -> impl Responder {
    let identicon_string = req.match_info().get("name").unwrap();
    let identicon = Identicon::new(identicon_string);
    let file = identicon.export_png_data().unwrap();

    HttpResponse::Ok().content_type("image/png").body(file)
}

async fn generate_jpeg(req: HttpRequest) -> impl Responder {
    let identicon_string = req.match_info().get("name").unwrap();
    let identicon = Identicon::new(identicon_string);
    let file = identicon.export_jpeg_data().unwrap();

    HttpResponse::Ok().content_type("image/jpeg").body(file)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = "[::1]:8088";
    println!(
        "Navigate to http://{}/{{input_string}} to see the image",
        address
    );
    HttpServer::new(|| {
        App::new()
            .route("/{name}", web::get().to(generate_png))
            .route("/{name}.png", web::get().to(generate_png))
            .route("/{name}.jpg", web::get().to(generate_jpeg))
            .route("/{name}.jpeg", web::get().to(generate_jpeg))
    })
    .bind(address)?
    .run()
    .await
}

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use identicon_rs::Identicon;

fn index(path_input: web::Path<String>) -> impl Responder {
    let identicon_fluffy = Identicon::new_default(&path_input);
    let file = identicon_fluffy.export_file_data();

    HttpResponse::Ok().content_type("image/png").body(file)
}

fn main() {
    HttpServer::new(|| App::new().route("/{input_string}", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}

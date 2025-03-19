use std::ffi::OsStr;

use axum::{
    Router,
    extract::Path,
    http::{StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use identicon_rs::Identicon;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:input", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root(Path(input): Path<String>) -> impl IntoResponse {
    let file_path = std::path::Path::new(&input);
    match file_path.file_stem() {
        None => StatusCode::BAD_REQUEST.into_response(),
        Some(input) => match input.to_str() {
            None => StatusCode::BAD_REQUEST.into_response(),
            Some(name) => {
                let identicon: Identicon = Identicon::new(name);
                generate_image(file_path.extension(), &identicon).into_response()
            }
        },
    }
}

fn generate_image(extention: Option<&OsStr>, identicon: &Identicon) -> impl IntoResponse {
    match extention {
        Some(os_str) => match os_str.to_str() {
            Some("jpeg") | Some("jpg") => (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "image/jpeg")],
                identicon.export_jpeg_data().unwrap(),
            )
                .into_response(),
            Some("png") => (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "image/png")],
                identicon.export_png_data().unwrap(),
            )
                .into_response(),
            _ => StatusCode::BAD_REQUEST.into_response(),
        },
        None => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "image/png")],
            identicon.export_png_data().unwrap(),
        )
            .into_response(),
    }
}

use axum::{Router, http::header, response::IntoResponse, routing::get};
use tokio::fs;

mod download;
mod serve;

async fn serve_image() -> impl IntoResponse {
    let bytes = fs::read("/home/tuxzilla/Downloads/image.webp")
        .await
        .unwrap();
    ([(header::CONTENT_TYPE, "image/webp")], bytes)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(serve_image))
        .route("/upload", get(download::download));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

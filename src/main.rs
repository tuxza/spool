use axum::{Router, routing::get};

mod download;
mod serve;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/serve", get(serve::files::send_image))
        .route("/upload", get(download::files::download));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

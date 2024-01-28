mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use handlers::{authorize::authorize, protected::protected};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/protected", get(protected))
        .route("/authorize", post(authorize));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

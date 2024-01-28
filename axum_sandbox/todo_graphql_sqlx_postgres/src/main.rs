mod handlers;
mod model;

use async_graphql::{EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router, Server};
use dotenv::dotenv;
use handlers::{graphql_handler::graphql_handler, graphql_playground::graphql_playground};
use model::{MutationRoot, QueryRoot};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let pool = {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").unwrap();
        PgPoolOptions::new().connect(&database_url).await.unwrap()
    };

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

    println!("Playground: http://localhost:8000");

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

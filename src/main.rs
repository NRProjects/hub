use axum::{http::StatusCode, routing::post, Json, Router};
use postgres::connect_db;
use serde_json::Value;
mod postgres;

#[tokio::main]
async fn main() {
    match connect_db().await {
        Ok(_) => println!("Connected to database"),
        Err(e) => eprintln!("Error connecting to database: {}", e),
    }

    let app = Router::new()
        .route("/", post(handle_request));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_request(Json(payload): Json<Value>) -> (StatusCode, Json<String>) {
    println!("Received: {}", payload);
    
    (StatusCode::OK, Json("Hello, World!".to_string()))
}
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GreetingParams {
    name: Option<String>,
}

async fn greet_someone(Json(request_body): Json<GreetingParams>) -> impl IntoResponse {
    let name_to_greet = request_body.name.unwrap_or("World".to_string());
    let return_value = format!("Hello {}!", name_to_greet);

    (StatusCode::OK, return_value)
}

async fn greet_by_url(Path(name): Path<String>) -> impl IntoResponse {
    let response = format!("Hello to {name:?}");

    (StatusCode::OK, response)
}

#[tokio::main]
async fn main() {
    let api = Router::new()
        .route("/", get(|| async { "Hello maciek!" }))
        .route("/greeting", post(greet_someone))
        .route("/hello/:name", get(greet_by_url));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, api).await.unwrap();
}

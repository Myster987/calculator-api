use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

async fn add_route(Path((first_num, second_num)): Path<(String, String)>) -> impl IntoResponse {
    let first_num = match first_num.parse::<f64>() {
        Ok(num) => num,
        Err(_) => return (StatusCode::BAD_REQUEST, "Enter only numbers".to_string()),
    };

    let second_num = match second_num.parse::<f64>() {
        Ok(num) => num,
        Err(_) => return (StatusCode::BAD_REQUEST, "Enter only numbers".to_string()),
    };

    let response_body = format!(
        "{first_num} + {second_num} = {result}",
        result = first_num + second_num
    );
    (StatusCode::OK, response_body)
}

async fn subtract_route(
    Path((first_num, second_num)): Path<(String, String)>,
) -> impl IntoResponse {
    let first_num = match first_num.parse::<f64>() {
        Ok(num) => num,
        Err(_) => return (StatusCode::BAD_REQUEST, "Enter only numbers".to_string()),
    };

    let second_num = match second_num.parse::<f64>() {
        Ok(num) => num,
        Err(_) => return (StatusCode::BAD_REQUEST, "Enter only numbers".to_string()),
    };

    let response_body = format!(
        "{first_num} - {second_num} = {result}",
        result = first_num - second_num
    );
    (StatusCode::OK, response_body)
}

async fn multiply_route(
    Path((first_num, second_num)): Path<(String, String)>,
) -> impl IntoResponse {
    let first_num = match first_num.parse::<f64>() {
        Ok(num) => num,
        Err(_) => return (StatusCode::BAD_REQUEST, "Enter only numbers".to_string()),
    };

    let second_num = match second_num.parse::<f64>() {
        Ok(num) => num,
        Err(_) => return (StatusCode::BAD_REQUEST, "Enter only numbers".to_string()),
    };

    let response_body = format!(
        "{first_num} * {second_num} = {result}",
        result = first_num * second_num
    );
    (StatusCode::OK, response_body)
}

async fn divide_route(Path((first_num, second_num)): Path<(String, String)>) -> impl IntoResponse {
    let first_num = match first_num.parse::<f64>() {
        Ok(num) => num,
        Err(_) => return (StatusCode::BAD_REQUEST, "Enter only numbers".to_string()),
    };

    let second_num = match second_num.parse::<f64>() {
        Ok(num) => num,
        Err(_) => return (StatusCode::BAD_REQUEST, "Enter only numbers".to_string()),
    };

    if second_num == 0.0 {
        return (StatusCode::BAD_REQUEST, "Can't divide by 0".to_string());
    }

    let response_body = format!(
        "{first_num} / {second_num} = {result}",
        result = first_num / second_num
    );
    (StatusCode::OK, response_body)
}

#[tokio::main]
async fn main() {
    let api = Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route("/add/:first_num/:second_num", get(add_route))
        .route("/subtract/:first_num/:second_num", get(subtract_route))
        .route("/multiply/:first_num/:second_num", get(multiply_route))
        .route("/divide/:first_num/:second_num", get(divide_route));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, api).await.unwrap();
}

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use std::sync::atomic::{AtomicU8, Ordering};

static COUNT: AtomicU8 = AtomicU8::new(0);

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello))
        .route("/clicked", post(click));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    async fn hello() -> Response {
        match std::fs::read_to_string("./html/hello.html") {
            Ok(content) => Html(content).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read file. Error: {}", err),
            ).into_response(),
        }
    }

    async fn click() -> Response {
        COUNT.fetch_add(1, Ordering::Relaxed);
        format!("Count: {}", COUNT.load(Ordering::Relaxed)).into_response()
    }
}
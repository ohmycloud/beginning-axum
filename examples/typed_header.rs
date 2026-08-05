use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};
use axum_extra::{
    TypedHeader,
    headers::{ContentLength, ContentType},
};
use serde_json::{Value, json};

async fn hello(
    Path(num): Path<i32>,
) -> (
    TypedHeader<ContentType>,
    TypedHeader<ContentLength>,
    (StatusCode, Json<Value>),
) {
    match num {
        0 => (
            TypedHeader(ContentType::json()),
            TypedHeader(ContentLength(27)),
            (
                StatusCode::CREATED,
                Json(json!({ "message": "Hello, World!".to_string() })),
            ),
        ),
        _ => (
            TypedHeader(ContentType::json()),
            TypedHeader(ContentLength(35)),
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Error during creation".to_string() })),
            ),
        ),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/{num}", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

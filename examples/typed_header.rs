use axum::{
    Json, Router,
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use axum_extra::{
    TypedHeader,
    headers::{ContentLength, ContentType},
};
use serde_json::json;

async fn response() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("x-foo", "custom header")
        .body(Body::from("not found"))
        .unwrap()
}

async fn hello(Path(num): Path<i32>) -> Response {
    match num {
        0 => (
            TypedHeader(ContentType::json()),
            TypedHeader(ContentLength(27)),
            (
                StatusCode::CREATED,
                Json(json!({ "message": "Hello, World!".to_string() })),
            ),
        )
            .into_response(),
        1 => response().await,
        _ => (
            TypedHeader(ContentType::json()),
            TypedHeader(ContentLength(35)),
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Error during creation".to_string() })),
            ),
        )
            .into_response(),
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

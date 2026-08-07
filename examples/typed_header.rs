use axum::{
    Extension, Json, Router,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use axum_extra::{
    TypedHeader,
    headers::{ContentLength, ContentType},
};
use serde_json::json;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct AppState {}

async fn handler(Extension(state): Extension<AppState>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "message": format!("hello {state:?}") })),
    )
}

async fn response() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("x-foo", "custom header")
        .body(Body::from("not found"))
        .unwrap()
}

#[axum::debug_handler]
async fn hello(Path(num): Path<i32>, State(data): State<Arc<Mutex<Vec<u8>>>>) -> impl IntoResponse {
    let message = {
        let mut data = data.lock().unwrap();
        data[0] += 1;
        format!("Hello, World! {data:?}")
    };

    match num {
        0 => (
            TypedHeader(ContentType::json()),
            TypedHeader(ContentLength(37)),
            (StatusCode::CREATED, Json(json!({ "message": message }))),
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
    let state = AppState {};
    let data = Arc::new(Mutex::new(vec![0; 3]));
    let app = Router::new()
        .route("/{num}", get(hello))
        .with_state(data)
        .route("/", get(handler))
        .layer(Extension(state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

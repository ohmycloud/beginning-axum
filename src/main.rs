use axum::Router;
use axum::http::header::{CONTENT_TYPE, HeaderMap, USER_AGENT};
use axum::routing::get;

async fn hello(headers: HeaderMap) -> String {
    let user_agent = headers
        .get(USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let content_type = headers
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    format!("User-Agent: {}, Content-Type: {}", user_agent, content_type)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

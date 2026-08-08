use crate::api::users::get_users;
use crate::db::init_db;
use axum::Router;
use axum::http::header::{CONTENT_TYPE, HeaderMap, USER_AGENT};
use axum::routing::get;
use axum_extra::{
    extract::TypedHeader,
    headers::{ContentType, UserAgent},
};
use sea_orm::DatabaseConnection;

mod api;
mod db;
mod entities;
mod utils;

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

async fn greet(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    TypedHeader(content_type): TypedHeader<ContentType>,
) -> String {
    format!("User-Agent: {}, Content-Type: {}", user_agent, content_type)
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let conn: DatabaseConnection = init_db().await;

    let app = Router::new()
        .route("/", get(hello))
        .route("/greet", get(greet))
        .route("/users", get(get_users))
        .with_state(conn);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

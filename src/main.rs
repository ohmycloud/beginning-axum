use std::collections::HashMap;

use axum::extract::Query;
use axum::http::header::{CONTENT_TYPE, HeaderMap, USER_AGENT};
use axum::routing::get;
use axum::{Json, Router};
use axum_extra::{
    extract::TypedHeader,
    headers::{ContentType, UserAgent},
};
use sea_orm::{ColumnTrait, Condition, Database, EntityTrait, QueryFilter};

use crate::entities::users;

mod entities;

const DATABASE_URL: &str = "postgres://axum:1234@localhost/axum";

async fn get_user(Query(params): Query<HashMap<String, String>>) -> Json<users::Model> {
    let conn = Database::connect(DATABASE_URL).await.unwrap();
    let mut condition = Condition::any();
    if let Some(id) = params.get("id") {
        condition = condition.add(users::Column::Id.eq(id.parse::<i32>().unwrap()));
    }
    if let Some(username) = params.get("username") {
        condition = condition.add(users::Column::Username.contains(username));
    }

    let user = users::Entity::find()
        .filter(condition)
        .one(&conn)
        .await
        .unwrap()
        .unwrap();

    Json(user)
}

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
    let app = Router::new()
        .route("/", get(hello))
        .route("/greet", get(greet))
        .route("/users", get(get_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

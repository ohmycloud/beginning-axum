use axum::Json;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;

use crate::entities::users;
use axum::extract::{Query, State};

pub async fn get_user(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<users::Model>> {
    let mut condition = Condition::all();

    if let Some(id) = params.get("id") {
        condition = condition.add(users::Column::Id.eq(id.parse::<i32>().unwrap()));
    }

    if let Some(username) = params.get("username") {
        condition = condition.add(users::Column::Username.contains(username));
    }

    let users = users::Entity::find()
        .filter(condition)
        .all(&conn)
        .await
        .unwrap();

    Json(users)
}

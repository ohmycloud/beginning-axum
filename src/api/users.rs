use axum::{Json, http::StatusCode};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder,
};
use std::collections::HashMap;

use crate::{entities::users, utils::app_error::AppError};
use axum::extract::{Query, State};

pub async fn get_users(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<users::Model>>, AppError> {
    let mut condition = Condition::all();

    if let Some(id) = params.get("id") {
        match id.parse::<i32>() {
            Ok(parsed_id) => condition = condition.add(users::Column::Id.eq(parsed_id)),
            Err(_) => {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    "ID must be an integer",
                ));
            }
        }
    }

    if let Some(username) = params.get("username") {
        condition = condition.add(users::Column::Username.contains(username));
    }

    match users::Entity::find()
        .filter(condition)
        .order_by(users::Column::Username, Order::Asc)
        .all(&conn)
        .await
    {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error",
        )),
    }
}

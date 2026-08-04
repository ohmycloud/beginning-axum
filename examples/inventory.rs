use axum::Json;
use axum::Router;
use axum::routing::get;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub details: ItemDetails,
}

#[derive(Serialize, Deserialize)]
pub struct ItemDetails {
    pub color: String,
    pub origin: String,
}

async fn hello() -> Json<Value> {
    Json(serde_json::json!(
        {
            "items": [
                { "name": "apple", "details": { "color": "red", "origin": "South Korea" } },
                { "name": "banana", "details": { "color": "yellow", "origin": "South America" } }
            ]
        }
    ))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

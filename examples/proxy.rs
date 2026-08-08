use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{Json, Router, body::Bytes, extract::State, http::StatusCode, routing::post};
use reqwest::Client;
use serde::Deserialize;

// HashMap for cache
type Cache = Arc<Mutex<HashMap<String, Bytes>>>;

#[derive(Deserialize)]
struct Data {
    // Breed
    breed: String,
    // Number of photos option
    num_pics: Option<i32>,
}

#[axum::debug_handler]
async fn proxy_handler(State(state): State<Cache>, Json(data): Json<Data>) -> (StatusCode, Bytes) {
    // Check cache
    if let Some(body) = state.lock().unwrap().get(&data.breed).cloned() {
        println!("{} cache hit", &data.breed);
        return (StatusCode::OK, body);
    }

    println!("{} cache miss", &data.breed);

    let mut url = format!("https://dog.ceo/api/breed/{}/images/random", &data.breed);
    if let Some(num_pics) = data.num_pics {
        url.push_str(&format!("/{}", num_pics));
    }

    // Request to backend server
    let client = Client::new();
    let res = client.get(url).send().await.unwrap();

    // Cache response
    let code = res.status().as_u16();
    let body = res.bytes().await.unwrap();
    let mut cache = state.lock().unwrap();
    cache.insert(data.breed, body.clone());

    // Return proxy response
    (StatusCode::from_u16(code).unwrap(), body)
}

#[tokio::main]
async fn main() {
    let state: Cache = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/", post(proxy_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

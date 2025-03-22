pub mod config;
use axum::{
    routing::{get, post},
    http::{StatusCode, Method},
    Json, Router,
    extract::State,
};
use core::CompilerJob;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{CorsLayer, Any};
use config::CompilerConfig;


struct AppState {
    counter: Mutex<i32>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        counter: Mutex::new(0),
    });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/hello", get(hello_handler))
        .route("/api/counter", get(get_counter).post(increment_counter))
        .with_state(state)
        .layer(cors);

    let addr = "127.0.0.1:3000";
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "message": "Hello from Rust backend!" }))
}

async fn get_counter(
    State(state): State<Arc<AppState>>
) -> Json<serde_json::Value> {
    let counter = *state.counter.lock().await;
    println!("Counter requested: {}", counter);
    Json(serde_json::json!({ "counter": counter }))
}

async fn increment_counter(
    State(state): State<Arc<AppState>>
) -> Json<serde_json::Value> {
    let mut counter = state.counter.lock().await;
    *counter += 1;
    println!("Counter incremented to: {}", *counter);
    Json(serde_json::json!({ "counter": *counter }))
}


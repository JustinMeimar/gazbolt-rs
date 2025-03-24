pub mod config;
pub mod api;

// use api::*;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use core::CompilerJob;
use serde::{Serialize, Deserialize};
use config::{CompilerConfig, read_configs};
use clap::Parser;
use axum::{
    routing::{get, post},
    http::{StatusCode, Method},
    extract::{State, Path},
    Router,
};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    config_dir: PathBuf,
    #[arg(long, default_value="3000")]
    port: String
}

struct AppState {
    configs: Vec<CompilerConfig>,
}

#[tokio::main]
async fn main() {
    
    let args = Args::parse();

    // read the compiler configs into memory
    let configs = read_configs(args.config_dir);
    if let Err(e) = &configs {
        eprintln!("Gazbolt Server Error: {}", e);
        std::process::exit(1);
    }

    let state = Arc::new(AppState { configs: configs.unwrap() });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/compilers", get(api::get_compilers_handler))
        .route("/api/programs/{compiler}", get(api::get_programs_handler))
        .route("/api/run", post(api::run_compiler_handler))
        .with_state(state)
        .layer(cors);

    let addr = "127.0.0.1:3000";
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


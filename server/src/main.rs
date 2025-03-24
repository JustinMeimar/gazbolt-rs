pub mod api;
pub mod config;
use crate::config::CompilerConfig;
use api::*;
use axum::{
  extract::{Path, State},
  http::{Method, StatusCode},
  routing::{get, post},
  Router,
};
use clap::Parser;
use config::read_configs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
  #[arg(long)]
  config_dir: PathBuf,
  #[arg(long, default_value = "3000")]
  port: String,
}

//
pub struct AppState {
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

  let state = Arc::new(AppState {
    configs: configs.unwrap(),
  });

  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);

  let app = Router::new()
    .route("/api/compilers", get(api::get_compilers_handler))
    .route("/api/compiler/:compiler", get(api::get_compiler_handler))
    .route("/api/programs/:compiler", get(api::get_programs_handler))
    .route("/api/run/:compiler", post(api::run_compiler_handler))
    .with_state(state)
    .layer(cors);

  let addr = "127.0.0.1:3000";
  println!("Server running on http://{}", addr);
  axum::Server::bind(&addr.parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

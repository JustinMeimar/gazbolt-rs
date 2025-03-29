pub mod api;
pub mod config;
use crate::config::CompilerConfig;
use axum::{
    http::{header, Method},
    routing::{get, post},
    Router,
};
use clap::Parser;
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

pub struct ServerState {
    configs: Vec<CompilerConfig>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // read the compiler configs into memory
    let configs = CompilerConfig::read_from_directory(args.config_dir);
    if let Err(e) = &configs {
        eprintln!("Gazbolt Server Error: {}", e);
        std::process::exit(1);
    }

    let state = Arc::new(ServerState {
        configs: configs.unwrap(),
    });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/api/compilers",                        get(api::compiler_list_view))
        .route("/api/compilers/:compiler",              get(api::compiler_versions_view))
        .route("/api/compilers/:compiler/:version",     get(api::compiler_version_view))
        .route("/api/programs/:compiler/:version",      get(api::get_programs_handler))
        .route("/api/run/:compiler/:version",           post(api::run_compiler_handler))
        .with_state(state)
        .layer(cors);

    let addr = "127.0.0.1:3000";
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


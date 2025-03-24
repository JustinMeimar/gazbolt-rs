use axum::Json;
use axum::extract::{Path, State};
use std::sync::Arc;
use crate::AppState;
use crate::config::CompilerConfig;

pub async fn get_compilers_handler(State(state): State<Arc<AppState>>)
    -> Json<serde_json::Value>
{
    let compiler_list: Vec<_> = state.configs.iter()
        .map(|config| {
            serde_json::json!({
                "name": config.info.name,
                "version": config.info.version
            })
        })
        .collect();
    
    Json(serde_json::json!({ "compilers": compiler_list }))
}

pub async fn get_programs_handler(Path(compiler): Path<String>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"compiler": compiler}))
}

pub async fn run_compiler_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "stdout": "abc", "stderr": "none"}))
}

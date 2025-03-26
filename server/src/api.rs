use crate::config::{CompilerConfig, CompilerInfo};
use crate::AppState;
use crate::views::{CompilerView, CompilerListView};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

///=====================================================///
/// API
///=====================================================///

pub async fn compiler_list_view(State(state): State<Arc<AppState>>)
                            -> (StatusCode, Json<serde_json::Value>)
{
  let config_views = CompilerListView::from(&state.configs);
  let config_json = serde_json::to_value(config_views).unwrap();
  (StatusCode::OK, Json(config_json))
}

pub async fn get_compiler_handler(State(state): State<Arc<AppState>>,
                                  Path(compiler): Path<String>)
                              -> (StatusCode, Json<serde_json::Value>)
{
  let compiler_config = state.configs.iter().find(|c| c.info.name == compiler);
  match compiler_config {
    Some(cc) => {
      let view = CompilerView::from(cc);
      let value = serde_json::to_value(view).unwrap();
      (StatusCode::OK, Json(value))
    },
    None => {
      let error = serde_json::json!({
          "error": format!("Compiler: '{}' not found.", compiler)
        });  
      (StatusCode::NOT_FOUND, Json(error))
    }
  }
}

pub async fn get_programs_handler(State(state): State<Arc<AppState>>,
                                  Path(compiler): Path<String>)
                              -> (StatusCode, Json<serde_json::Value>)
{
  (
    StatusCode::NOT_FOUND,
    Json(serde_json::json!({"error": "not implemented"}))
  ) 
}

#[derive(Deserialize)]
pub struct ExecRequest {
    code: String,
}

pub async fn run_compiler_handler(State(state): State<Arc<AppState>>,
                                  Path(compiler): Path<String>,
                                  Json(request): Json<ExecRequest>)
                              -> (StatusCode, Json<serde_json::Value>)
{
  println!("Received Code: {}", request.code);
  (
    StatusCode::OK,
    Json(serde_json::json!({
      "stdout": "Received request successfully",
      "stderr": "Yep",
      "exit_code": 3
    }))
  ) 
}


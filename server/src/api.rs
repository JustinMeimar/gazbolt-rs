use crate::config::{CompilerConfig, CompilerInfo};
use crate::AppState;
use crate::views;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use core::types::CompilersListResponse;
use std::sync::Arc;

///=====================================================///
/// Views
///=====================================================///
#[derive(Serialize)]
pub struct CompilerView {
  name: String,
  version: String,
}

#[derive(Serialize)]
pub struct CompilerListView {
  configs: Vec<CompilerView>,
  page_no: usize,
}

impl From<&CompilerConfig> for CompilerView {
  fn from(config: &CompilerConfig) -> Self {
    Self {
      name: config.info.name.clone(),
      version: config.info.version.clone(),
    }
  }
}

impl From<&Vec<CompilerConfig>> for CompilerListView {
  fn from(configs: &Vec<CompilerConfig>) -> Self {
    let config_views = configs
      .iter()
      .map(|c| CompilerView::from(c))
      .collect(); 

    // Only support one, big page for now. 
    Self {
      configs: config_views,
      page_no: 1
    }
  }
}

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
          "error": format!("Compiler: '{}' not found.", compiler) // Fixed typo
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

pub async fn run_compiler_handler(State(state): State<Arc<AppState>>,
                                  Path(compiler): Path<String>)
                              -> (StatusCode, Json<serde_json::Value>)
{ 
  (
    StatusCode::NOT_FOUND,
    Json(serde_json::json!({"error": "not implemented"}))
  ) 
}


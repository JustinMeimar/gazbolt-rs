use crate::config::{CompilerConfig, CompilerInfo};
use crate::ServerState;
use core::{ApiCompilerItemView, ApiCompilerListView};
use core::{ApiExecResponse, ApiExecRequest};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

///=====================================================///
/// API
///=====================================================///

pub async fn compiler_list_view(State(state): State<Arc<ServerState>>)
                            -> (StatusCode, Json<serde_json::Value>)
{
    // Query the configs from server state.
    let config_views: Vec<ApiCompilerItemView> = state.configs
        .iter()
        .map(|cc| ApiCompilerItemView {
            name: cc.info.name.clone(),
            version: cc.info.version.clone()
        })
        .collect();
    
    // Construct the List View.
    let view = ApiCompilerListView {
        configs: config_views,
        page_no: 1, // Only support one, big page for now. 
    };
    
    // Serialize into JSON.
    let json_view = serde_json::to_value(view)
        .expect("Failed to serialize");
    
    // Return with 200
    (StatusCode::OK, Json(json_view))
}

pub async fn get_compiler_handler(
    State(state): State<Arc<ServerState>>,
    Path(compiler): Path<String>)
-> (StatusCode, Json<serde_json::Value>)
{
    // Find the queried compiler
    let compiler_config = state.configs
        .iter()
        .find(|c| c.info.name == compiler);
    
    // Match on find result.
    match compiler_config {
        Some(cc) => {
            // Construct the view to return
            let value = ApiCompilerItemView {
                name: cc.info.name.clone(),
                version: cc.info.version.clone(),
            };
            
            // Serialize the JSON view
            let json_view = serde_json::to_value(value)
                .expect("Failed to serialize");
            
            // Return with 200 OK
            (StatusCode::OK, Json(json_view))
        },
        None => {
            // Construct an error. TODO: Define proper error API.
            let error = serde_json::json!({
                "error": format!("Compiler: '{}' not found.", compiler)
            });  
            (StatusCode::NOT_FOUND, Json(error))
        }
    }
}

pub async fn get_programs_handler(State(state): State<Arc<ServerState>>,
                                  Path(compiler): Path<String>)
                              -> (StatusCode, Json<serde_json::Value>)
{
    // TODO: Implement.
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": "not implemented"}))
    ) 
}

pub async fn run_compiler_handler(State(state): State<Arc<ServerState>>,
                                  Path(compiler): Path<String>,
                                  Json(request): Json<ApiExecRequest>)
                              -> (StatusCode, Json<serde_json::Value>)
{
    // TODO: Implement.
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


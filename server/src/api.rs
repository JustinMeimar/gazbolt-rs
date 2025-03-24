use crate::config::{CompilerConfig, CompilerInfo};
use crate::AppState;
use axum::extract::{Path, State};
use axum::Json;
use core::{CompilerResponse, CompilersResponse, ExecRequest, ExecResponse, Request, Response};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// #[derive(Serialize)]
// pub struct RunResponse {
//     stdout: String,
//     stderr: String,
//     exit_code: u8,
// }

/// @desc: retrieve a vector of compilers.
/// @http: GET
/// @route: `https://service/api/compilers`
pub async fn get_compilers_handler(State(state): State<Arc<AppState>>) -> Json<Response> {
  let compilers: Vec<CompilerResponse> = state
    .configs
    .iter()
    .map(|config| CompilerResponse {
      name: config.info.name.clone(),
      version: config.info.version.clone(),
    })
    .collect();

  // Return the Response enum with CompilersResponse variant
  Json(Response::Compilers(CompilersResponse { compilers }))
}

/// @desc: retreive a single compiler config at name.
/// @http: GET
/// ```
/// https://service/api/compiler/<NAME>
/// ```
pub async fn get_compiler_handler(
  State(state): State<Arc<AppState>>,
  Path(compiler): Path<String>,
) -> Json<serde_json::Value> {
  let compiler_config = state.configs.iter().find(|c| c.info.name == compiler);

  Json(serde_json::json!({ "compiler": compiler_config }))
}

/// @desc:
/// @http: GET
/// ```
/// ```
pub async fn get_programs_handler(
  State(state): State<Arc<AppState>>,
  Path(compiler): Path<String>,
) -> Json<serde_json::Value> {
  // find the compiler requested to run
  let compiler_config = state.configs.iter().find(|c| c.info.name == compiler);

  Json(serde_json::json!({"config": compiler_config}))
}

/// @desc:
/// @http: POSTk
/// ```
/// ```
pub async fn run_compiler_handler(
  State(state): State<Arc<AppState>>,
  Path(compiler): Path<String>,
  Json(payload): Json<ExecRequest>,
) -> Json<serde_json::Value> {
  // find the compiler requested to run
  let compiler_config = state.configs.iter().find(|c| c.info.name == compiler);

  match compiler_config {
    Some(config) => {
      let response = ExecResponse {
        stdout: "one".to_string(),
        stderr: "two".to_string(),
        exit_code: 0,
      };
      Json(serde_json::to_value(response).unwrap())
    }
    None => Json(serde_json::json!({
        "error": format!("Compiler: '{}' not found", compiler)
    })),
  }
}

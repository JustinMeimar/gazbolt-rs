use crate::ServerState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use core::ApiExecRequest;
use core::{ApiCompilerItemView, ApiCompilerListView};
use std::sync::Arc;

///=====================================================///
/// API
///=====================================================///

pub async fn compiler_list_view(
    State(state): State<Arc<ServerState>>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Query the configs from server state.
    let config_views: Vec<ApiCompilerItemView> = state
        .configs
        .iter()
        .map(|cc| ApiCompilerItemView {
            name: cc.info.name.clone(),
            version: cc.info.version.clone(),
        })
        .collect();

    // Construct the List View.
    let view = ApiCompilerListView {
        configs: config_views,
        page_no: 1, // Only support one, big page for now.
    };

    // Serialize into JSON.
    let json_view = serde_json::to_value(view).expect("Failed to serialize");

    // Return with 200
    (StatusCode::OK, Json(json_view))
}


pub async fn compiler_versions_view(
    State(state): State<Arc<ServerState>>,
    Path(compiler): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Find all versions of the compiler
    let compiler_versions: Vec<_> = state.configs
        .iter()
        .filter(|c| c.info.name == compiler)
        .map(|cc| ApiCompilerItemView {
            name: cc.info.name.clone(),
            version: cc.info.version.clone(),
        })
        .collect();
    
    if compiler_versions.is_empty() {
        let error = serde_json::json!({
            "error": format!("Compiler: '{}' not found.", compiler)
        });
        return (StatusCode::NOT_FOUND, Json(error));
    }
    
    // Serialize the JSON view
    let json_view = serde_json::to_value(compiler_versions).expect("Failed to serialize");
    // Return with 200 OK
    (StatusCode::OK, Json(json_view))
}

pub async fn compiler_version_view(
    State(state): State<Arc<ServerState>>,
    Path((compiler, version)): Path<(String, String)>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Find the specific compiler version
    let compiler_config = state.configs
        .iter()
        .find(|c| c.info.name == compiler && c.info.version == version);
    
    match compiler_config {
        Some(cc) => {
            // Construct the view to return
            let view = ApiCompilerItemView {
                name: cc.info.name.clone(),
                version: cc.info.version.clone(),
            };
            // Serialize the JSON view
            let json_view = serde_json::to_value(view).expect("Failed to serialize");
            // Return with 200 OK
            (StatusCode::OK, Json(json_view))
        }
        None => {
            // Construct an error
            let error = serde_json::json!({
                "error": format!("Compiler: '{}' version '{}' not found.", compiler, version)
            });
            (StatusCode::NOT_FOUND, Json(error))
        }
    }
}

pub async fn get_programs_handler(
    State(_state): State<Arc<ServerState>>,
    Path(_compiler): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    // TODO: Implement.
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": "not implemented"})),
    )
}

pub async fn run_compiler_handler(
    State(state): State<Arc<ServerState>>,
    Path((compiler, version)): Path<(String, String)>,
    Json(request): Json<ApiExecRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Check that the provided compiler exists
    let (status, compiler_json) =
        compiler_version_view(
            State(state.clone()), Path((compiler.clone(), version.clone()))
        ).await;
    
    // Return with 404 if not
    if status != StatusCode::OK {
        return (status, compiler_json);
    }

    // Compiler should be in the state
    let compiler_config = state
        .configs
        .iter()
        .find(|c| c.info.name == compiler)
        .unwrap();

    // Run the received code on the compiler
    let exec_response_view = compiler_config.run(request.code);

    // Serialize view to JSON
    let json_response = serde_json::to_value(exec_response_view).expect("Failed to serialize JSON");

    // Return response
    (StatusCode::OK, Json(json_response))
}

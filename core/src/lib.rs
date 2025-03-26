use serde::{Serialize, Deserialize};

///==========================================================================///
/// This file contains the shared JSON request and response types between the 
/// frontend and the backend. This powerful notion of sharing types between
/// the frontend and backend is a major reason this project uses Rust.
///==========================================================================///

#[derive(Serialize, Deserialize)]
pub struct ApiExecRequest {
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiExecResponse {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: u8
}

#[derive(Serialize, Deserialize)]
pub struct ApiCompilerItemView {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiCompilerListView {
    pub configs: Vec<ApiCompilerItemView>,
    pub page_no: usize,
}


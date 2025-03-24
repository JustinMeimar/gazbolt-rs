use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Response types server can send and client can receive.
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Response {
  #[serde(rename = "compiler")]
  Compiler(CompilerResponse),

  #[serde(rename = "compilers")]
  Compilers(CompilersResponse),

  #[serde(rename = "programs")]
  Programs(ProgramsResponse),

  #[serde(rename = "exec")]
  Exec(ExecResponse),
}

/// Request types server can receive and client can send.
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
  #[serde(rename = "exec")]
  Exec(ExecRequest),
}

/// Specific structure of each type below:

#[derive(Serialize, Deserialize)]
pub struct CompilerResponse {
  pub name: String,
  pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompilersResponse {
  pub compilers: Vec<CompilerResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ExecResponse {
  pub stdout: String,
  pub stderr: String,
  pub exit_code: u8,
}

/// TODO:
#[derive(Serialize, Deserialize)]
pub struct ProgramsResponse {}

#[derive(Serialize, Deserialize)]
pub struct ExecRequest {
  program: String,
}

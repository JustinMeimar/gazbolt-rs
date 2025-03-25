use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

///=====================================================///
/// public types
///=====================================================///


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag="type")]
pub struct CompilersListResponse {
    pub items: Vec<CompilerItemResponse>    
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag="type")]
pub struct CompilerItemResponse {
    id: u32,
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag="type")]
pub struct ProgramListResponse {
    items: Vec<ProgramItemResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag="type")]
pub struct ProgramItemResponse {
    name: String,
    contents: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag="type")]
pub struct ExecResponse {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    exit_code: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag="type")]
pub struct ExecRequest {
    compiler: CompilerItemResponse,
    program: Vec<u8>
}

///=====================================================///
/// private types
///=====================================================///

#[derive(Debug, Serialize, Deserialize)]
struct CompilerConfig {
  info: CompilerInfo,
  steps: Vec<Step>,
  #[serde(default)]
  temp_files: TempFilesConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompilerInfo {
  name: String,
  version: String,
  icon: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct Step {
  cmd: String,
  flags: Vec<String>,
  output_file: String,
  hidden: bool,
  use_stdin: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct TempFilesConfig {
  dir: PathBuf,
  prefix: String,
  cleanup: bool,
}


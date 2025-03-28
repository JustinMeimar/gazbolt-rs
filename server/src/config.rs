use serde::{Deserialize, Serialize};
use core::ApiExecResponse;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerConfig {
  pub info: CompilerInfo,
  steps: Vec<Step>,
  #[serde(default)]
  temp_files: TempFilesConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerInfo {
  pub name: String,
  pub version: String,
  pub icon: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
  cmd: String,
  flags: Vec<String>,
  output_file: String,
  hidden: bool,
  use_stdin: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TempFilesConfig {
  dir: PathBuf,
  prefix: String,
  cleanup: bool,
}

impl CompilerConfig {
    
    // Execute a program on a configuration.
    pub fn run(&self, program: String) -> ApiExecResponse {
        
        println!("DEBUG: running program: {} on compiler: {}", program, self.info.name);

        self.steps
            .iter()
            .map(|s| println!("{:?}", s));

        ApiExecResponse {
            stdout: "This is stdout.".to_string(),
            stderr: "This is stderr.".to_string(),
            exit_code: 255,
        } 
    }
    
    // Classmethod to create a vector of Self from a directory of TOML configs
    pub fn read_from_directory(config_dir: PathBuf) -> io::Result<Vec<Self>> {
        
      // Check path exists.
      if !config_dir.exists() {
        let err_ty = io::ErrorKind::NotFound;
        return Err(io::Error::new(err_ty, "Config dir not found."));
      }

      // Check path is a directory.
      if !config_dir.is_dir() {
        let err_ty = io::ErrorKind::NotADirectory;
        return Err(io::Error::new(err_ty, "Path is not a directory."));
      }

      // Deserailize into a configuration each file found in the config directory.
      let configs: Vec<Self> = fs::read_dir(&config_dir)?
        .filter_map(Result::ok)
        .filter_map(
          |entry: fs::DirEntry| match fs::read_to_string(entry.path()) {
            Ok(content) => match toml::from_str::<CompilerConfig>(&content) {
              Ok(config) => Some(config),
              Err(e) => {
                eprintln!("Error parsing config file {:?}: {}", entry.path(), e);
                None
              }
            },
            Err(e) => {
              eprintln!("Error reading file {:?}: {}", entry.path(), e);
              None
            }
          },
        )
        .collect();
      
      // Check we have found at least one config to serve.
      if configs.is_empty() {
        return Err(io::Error::new(
          io::ErrorKind::NotFound,
          "No valid config files found.",
        ));
      }
      
      // Create compilers object.
      Ok(configs)
    }
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;
  use std::fs::File;
  use std::io::{BufReader, Read, Write};
  #[test]
  fn test_read_and_serialize_toml() {
    // Create a sample TOML file
    let config_path = "./configs/gcc.toml";
    let toml_content = fs::read_to_string(config_path).expect("Failed to read the TOML file");

    let config: CompilerConfig = toml::from_str(&toml_content).expect("Failed to deserialize TOML");

    // verify the deserialized content
    assert_eq!(config.info.name, "gcc");
    assert_eq!(config.info.version, "0.10");
    assert_eq!(config.steps[0].cmd, "gcc");
    assert_eq!(config.steps[1].cmd, "prog.exe");

    let serialized = toml::to_string(&config).expect("Failed to serialize to TOML");
    let deserialized: CompilerConfig =
      toml::from_str(&serialized).expect("Failed to deserialize the serialized TOML");

    assert_eq!(deserialized.info.name, config.info.name);
  }
}


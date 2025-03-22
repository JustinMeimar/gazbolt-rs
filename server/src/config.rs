use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerConfig {
    info: CompilerInfo,
    steps: Vec<Step>,
    #[serde(default)]
    temp_files: TempFilesConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerInfo {
    pub name: String,
    pub version: String,
    pub icon: PathBuf
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
    cleanup: bool
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{Read, BufReader, Write};
    use std::fs::File;
    #[test]
    fn test_read_and_serialize_toml() {
        // Create a sample TOML file
        let config_path = "./configs/gcc.toml";  
        let toml_content = fs::read_to_string(config_path)
            .expect("Failed to read the TOML file");
        
        let config: CompilerConfig = toml::from_str(&toml_content)
            .expect("Failed to deserialize TOML");
        
        // verify the deserialized content
        assert_eq!(config.info.name, "gcc");
        assert_eq!(config.info.version, "0.10");
        assert_eq!(config.steps[0].cmd, "gcc");
        assert_eq!(config.steps[1].cmd, "prog.exe");
              
        let serialized = toml::to_string(&config)
            .expect("Failed to serialize to TOML");
        let deserialized: CompilerConfig = toml::from_str(&serialized)
            .expect("Failed to deserialize the serialized TOML");
        
        assert_eq!(deserialized.info.name, config.info.name);
    }
}

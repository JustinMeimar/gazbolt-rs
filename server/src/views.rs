use crate::config::CompilerConfig;
use serde::Serialize;

///=====================================================///
/// API Views
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

/// Type conversions from private server types 

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


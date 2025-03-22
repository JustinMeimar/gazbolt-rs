use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Compiler {
    Generator,
    SCalc,
    VCalc,
    Gazprea,
}

impl Compiler {
    pub fn default() -> Self {
        Compiler::Generator
    }
    pub fn collect() -> Vec<Self> {
        return vec![
            Compiler::Generator,
            Compiler::SCalc,
            Compiler::VCalc,
            Compiler::Gazprea
        ]
    }
}

impl fmt::Display for Compiler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Compiler::Generator => write!(f, "Generator"),
            Compiler::SCalc => write!(f, "SCalc"),
            Compiler::VCalc => write!(f, "VCalc"),
            Compiler::Gazprea => write!(f, "Gazprea"),
        }
    }
}


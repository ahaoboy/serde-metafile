use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportKind {
    #[serde(rename = "require-call")]
    RequireCall,

    #[serde(rename = "import-statement")]
    ImportStatement,

    #[serde(rename = "dynamic-import")]
    DynamicImport,

    #[serde(rename = "import-rule")]
    ImportRule,

    #[serde(rename = "url-token")]
    UrlToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ImportKind>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub external: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub bytes: u64,
    pub imports: Vec<Import>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputDetail {
    #[serde(rename = "bytesInOutput")]
    pub bytes_in_output: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputImport {
    pub path: String,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub bytes: u64,
    pub inputs: HashMap<String, InputDetail>,
    pub imports: Vec<OutputImport>,
    pub exports: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "entryPoint"
    )]
    pub entry_point: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cssBundle")]
    pub css_bundle: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metafile {
    pub inputs: HashMap<String, Input>,
    pub outputs: HashMap<String, Output>,
}

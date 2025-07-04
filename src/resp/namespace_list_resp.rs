use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NamespaceListResp {
    pub code: i32,
    pub message: Option<String>,
    pub data: Vec<Namespace>,
}

#[derive(Debug, Deserialize)]
pub struct Namespace {
    // Namespace ID
    pub namespace: String,
    pub namespaceShowName: String,
    pub namespaceDesc: Option<String>,
    pub quota: i32,
    pub configCount: i32,
    #[serde(rename = "type")]
    pub type_ : i32,
}
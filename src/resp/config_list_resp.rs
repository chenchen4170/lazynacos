use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfigListResp{
    pub code: i32,
    pub message: Option<String>,
    pub data: Vec<Config>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    // Namespace ID
    pub id: String,
    pub dataId: String,
    pub group: String,
    pub content: Option<String>,
    pub md5: Option<String>,
    pub encryptedDataKey: Option<String>,
    pub tenant: String,
    pub appName: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub lastModified: i64,
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthLoginResp {
    pub accessToken: String,
    pub tokenTtl: i64,
    pub globalAdmin: bool,
    pub username: String,
}
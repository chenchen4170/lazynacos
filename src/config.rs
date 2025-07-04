use serde::Deserialize;
use config::Config;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub nacos: NacosConfig,
}

#[derive(Debug, Deserialize)]
pub struct NacosConfig{
    pub url: String,
    pub username: String,
    pub password: String,
}

pub fn load_config() -> AppConfig {
    let env = std::env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());
    println!("Loading configuration for environment: {}", env);

    let settings = Config::builder()
        .add_source(config::File::with_name(&format!("config/{}", env)).required(false))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    settings.try_deserialize().unwrap()
}
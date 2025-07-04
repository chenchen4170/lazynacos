mod api;
mod resp;

mod config;
use config::{load_config, AppConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let app_config: AppConfig = load_config();

    let resp = api::auth::login(
        &app_config.nacos.url,
        &app_config.nacos.username,
        &app_config.nacos.password,
    ).await?;
    println!("{resp:#?}");

    Ok(())
}
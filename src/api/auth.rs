use crate::resp::auth_login_resp::AuthLoginResp;
use crate::config::{load_config, AppConfig};

pub async fn login(url: &str, username: &str, password: &str) -> Result<AuthLoginResp, String> {
    let client = reqwest::Client::new();
    let resp = client.post(format!("{}/nacos/v1/auth/login", url))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("username={}&password={}", username, password))
        .send()
        .await;

        match resp {
            Ok(response) => {
                if response.status().is_success() {
                    let json_resp = response.json::<AuthLoginResp>().await.map_err(|e| e.to_string())?;
                    Ok(json_resp)
                } else {
                    let text_resp = response.text().await.map_err(|e| e.to_string())?;
                    Err(format!("Request failed: {}", text_resp))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_config;
    use crate::config::AppConfig;

    #[tokio::test]
    async fn test_login() {
        let app_config: AppConfig = load_config();
        let resp = login(
            &app_config.nacos.url, 
            &app_config.nacos.username, 
            &app_config.nacos.password).await;
        assert!(resp.is_ok(), "Login failed: {:?}", resp.err());
    }

    #[tokio::test]
    async fn test_login_with_invalid_credentials() {
        let app_config: AppConfig = load_config();
        let resp = login(
            &app_config.nacos.url, "wrong_user", "wrong_pass").await;
        println!("Login response: {:?}", resp);
        assert!(resp.is_err());
    }
}
//ns_id 命名空间id，不传表示查询public空间
pub async fn get(
    url: &str,
    access_token: &str,
    ns_id: Option<String>,
    data_id: &str,
    group: &str,
) -> Result<String, String> {
    let params = match ns_id {
        Some(ns_id) => format!("tenant={ns_id}&dataId={data_id}&group={group}"),
        None => format!("dataId={data_id}&group={group}"),
    };

    let client = reqwest::Client::new();
    let resp = client
        .get(&format!(
            "{url}/nacos/v1/cs/configs?accessToken={access_token}&{params}"
        ))
        .send()
        .await;
    match resp {
        Ok(response) => {
            if response.status().is_success() {
                let json_resp = response.text().await.map_err(|e| e.to_string())?;
                Ok(json_resp)
            } else {
                let text_resp = response.text().await.map_err(|e| e.to_string())?;
                Err(format!("Request failed: {}", text_resp))
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

//发布配置/更新配置
pub async fn publish(
    url: &str,
    access_token: &str,
    ns_id: Option<String>,
    data_id: &str,
    group: &str,
    content: &str,
    type_: &str,
) -> Result<bool, String> {
    let params = match ns_id {
        Some(ns_id) => format!("tenant={ns_id}&dataId={data_id}&group={group}&content={content}&type={type_}"),
        None => format!("dataId={data_id}&group={group}&content={content}&type={type_}"),
    };

    let client = reqwest::Client::new();
    let resp = client
        .post(&format!("{url}/nacos/v1/cs/configs?accessToken={access_token}&{params}"))
        .send()
        .await;
    match resp {
        Ok(response) => {
            if response.status().is_success() {
                let json_resp = response.json::<bool>().await.map_err(|e| e.to_string())?;
                Ok(json_resp)
            } else {
                let text_resp = response.text().await.map_err(|e| e.to_string())?;
                Err(format!("Request failed: {}", text_resp))
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

pub async fn delete(
    url: &str,
    access_token: &str,
    ns_id: Option<String>,
    data_id: &str,
    group: &str,
) -> Result<bool, String> {
    let params = match ns_id {
        Some(ns_id) => format!("tenant={ns_id}&dataId={data_id}&group={group}"),
        None => format!("dataId={data_id}&group={group}"),
    };

    let client = reqwest::Client::new();
    let resp = client
        .delete(&format!("{url}/nacos/v1/cs/configs?accessToken={access_token}&{params}"))
        .send()
        .await;
    match resp {
        Ok(response) => {
            if response.status().is_success() {
                let json_resp = response.json::<bool>().await.map_err(|e| e.to_string())?;
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
    use crate::api;
    use crate::config::AppConfig;
    use crate::config::load_config;
    use std::sync::OnceLock;

    // 全局缓存 access_token
    static ACCESS_TOKEN_CACHE: OnceLock<String> = OnceLock::new();
    static APP_CONFIG_CACHE: OnceLock<AppConfig> = OnceLock::new();

    // 辅助函数：加载并缓存 AppConfig
    fn get_app_config() -> &'static AppConfig {
        APP_CONFIG_CACHE.get_or_init(|| {
            load_config()
        })
    }

    // 辅助函数：登录并获取 access_token，支持缓存
    async fn get_access_token() -> Result<String, String> {
        if let Some(token) = ACCESS_TOKEN_CACHE.get() {
            return Ok(token.clone());
        }

        let app_config = get_app_config();

        let auth_login_resp = api::auth::login(
            &app_config.nacos.url,
            &app_config.nacos.username,
            &app_config.nacos.password,
        )
        .await;

        match auth_login_resp {
            Ok(resp) => {
                ACCESS_TOKEN_CACHE.set(resp.accessToken.clone()).unwrap();
                Ok(resp.accessToken)
            }
            Err(e) => Err(format!("Login failed: {}", e)),
        }
    }

    #[tokio::test]
    async fn test_get() {
        let app_config = get_app_config();
        let access_token = match get_access_token().await {
            Ok(token) => token,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        let resp = get(
            &app_config.nacos.url,
            &access_token,
            Some("8fa56574-e685-495c-833e-42b525b35c1a".to_string()),
            "lute-iot-admin.yml",
            "DEFAULT_GROUP",
        )
        .await;
        assert!(resp.is_ok(), "Failed to get config: {:?}", resp.err());
    }
}

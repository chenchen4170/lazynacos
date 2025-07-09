
use crate::resp::namespace_list_resp::NamespaceListResp;

pub async fn list(url: &str) -> Result<NamespaceListResp, String> {

    let client = reqwest::Client::new();
    let resp = client
        .get(&format!("{url}/nacos/v1/console/namespaces"))
        .send()
        .await;
    match resp {
        Ok(response) => {
            if response.status().is_success() {
                let json_resp = response.json::<NamespaceListResp>().await.map_err(|e| e.to_string())?;
                Ok(json_resp)
            } else {
                let text_resp = response.text().await.map_err(|e| e.to_string())?;
                Err(format!("Request failed: {}", text_resp))
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    } 
}

// 创建成功返回true， 创建失败返回false
pub async fn create(url: &str, access_token: &str, ns_id: &str, ns_name: &str, ns_desc: &str) -> Result<bool, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post(&format!("{url}/nacos/v1/console/namespaces?accessToken={access_token}"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("customNamespaceId{}=&namespaceName={}&namespaceDesc={}",ns_id, ns_name, ns_desc))
        .send()
        .await;
    match resp {
        Ok(response) => {
            if response.status().is_success() {
                Ok(true)
            } else {
                let text_resp = response.text().await.map_err(|e| e.to_string())?;
                Err(format!("Request failed: {}", text_resp))   
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

pub async fn update(url: &str, access_token: &str, ns_id: &str, ns_name: &str, ns_desc: &str) -> Result<bool, String> {
    let client = reqwest::Client::new();
    let resp = client
        .put(&format!("{url}/nacos/v1/console/namespaces?accessToken={access_token}"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("namespace={ns_id}&namespaceShowName={ns_name}&namespaceDesc={ns_desc}"))
        .send()
        .await;
    match resp {
        Ok(response) => {
            if response.status().is_success() {
                Ok(true)
            } else {
                let text_resp = response.text().await.map_err(|e| e.to_string())?;
                Err(format!("Request failed: {}", text_resp))   
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

pub async fn delete(url: &str, access_token: &str, ns_id: &str) -> Result<bool, String> {
    let client = reqwest::Client::new();
    let resp = client
        .delete(&format!("{url}/nacos/v1/console/namespaces?accessToken={access_token}"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("namespaceId={ns_id}"))
        .send()
        .await;
    match resp {
        Ok(response) => {
            if response.status().is_success() {
                Ok(true)
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
    async fn test_list() {
        let app_config = get_app_config();
        let resp = list(&app_config.nacos.url).await;
        assert!(resp.is_ok(), "Failed to list namespaces: {:?}", resp.err());
    }

    #[tokio::test]
    async fn test_create() {
        let app_config = get_app_config();
        let access_token = get_access_token().await.unwrap();

        let ns_id = "123";
        let ns_name = "Test-Namespace";
        let ns_desc = "This is a test namespace";

        let resp = create(&app_config.nacos.url, &access_token, ns_id, ns_name, ns_desc).await;
        assert!(resp.is_ok(), "Failed to create namespace: {:?}", resp.err());
        assert_eq!(resp.unwrap(), true, "Namespace creation should return true");
    }

    #[tokio::test]
    async fn test_update() {
        let app_config = get_app_config();
        let access_token = get_access_token().await.unwrap();

        let ns_id = "123";
        let ns_name = "Updated-Namespace";
        let ns_desc = "This is an updated test namespace";

        let resp = update(&app_config.nacos.url, &access_token, ns_id, ns_name, ns_desc).await;
        assert!(resp.is_ok(), "Failed to update namespace: {:?}", resp.err());
        assert_eq!(resp.unwrap(), true, "Namespace update should return true");
    }

    #[tokio::test]
    async fn test_delete() {
        let app_config = get_app_config();
        let access_token = get_access_token().await.unwrap();

        let ns_id = "123"; // 确保这个ID是存在的
        let resp = delete(&app_config.nacos.url, &access_token, ns_id).await;
        assert!(resp.is_ok(), "Failed to delete namespace: {:?}", resp.err());
        assert_eq!(resp.unwrap(), true, "Namespace deletion should return true");
    }
}
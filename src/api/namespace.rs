
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
pub async fn create(url: &str, access_token: &str, ns_name: &str, ns_desc: &str) -> Result<bool, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post(&format!("{url}/nacos/v1/console/namespaces?accessToken={access_token}"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("customNamespaceId=&namespaceName={}&namespaceDesc={}", ns_name, ns_desc))
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
    use crate::config::load_config;
    use crate::config::AppConfig;

    #[tokio::test]
    async fn test_list() {
        let app_config: AppConfig = load_config();
        let resp = list(&app_config.nacos.url).await;
        assert!(resp.is_ok(), "Failed to list namespaces: {:?}", resp.err());
    }
}
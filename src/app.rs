use crate::api::auth::login;
use crate::resp::auth_login_resp::AuthLoginResp;

use crate::api::namespace;
use crate::resp::namespace_list_resp::Namespace;

use crate::api::config;
use crate::resp::config_list_resp::Config;

use crate::config::{load_config, AppConfig};

pub struct App {
    pub exit: bool,
    pub current_block: u8,
    pub current_tab: usize, // 当前选中的标签索引
    pub namespaces: Vec<Namespace>,
    pub configs: Vec<Config>, // 假设有一个 configs 字段用于存储配置
    pub auth_login_resp: AuthLoginResp, // 新增字段，用于保存登录后的 token  
}

impl App {
    pub async fn new() -> App {
        let app_config: AppConfig = load_config();
        let resp = login(
            &app_config.nacos.url, 
            &app_config.nacos.username, 
            &app_config.nacos.password).await;

        let auth_resp = match resp {
            Ok(auth_resp) => auth_resp,
            Err(e) => {
                panic!("Login failed: {}", e);
            }
        };

        let namespaces_resp = namespace::list(
            &app_config.nacos.url).await;
        let namespaces = match namespaces_resp {
            Ok(ns_list) => ns_list,
            Err(e) => {
                panic!("Failed to fetch namespaces: {}", e);
            }
        };

        let configs_resp = config::list(
            &app_config.nacos.url,
            &auth_resp.accessToken,
            &namespaces.data[0].namespace).await;
        let configs = match configs_resp {
            Ok(config_list) => config_list,
            Err(e) => {
                panic!("Failed to fetch configs: {}", e);
            }
        };

        App {
            exit: false,
            current_tab: 0,
            current_block: 1,
            namespaces: namespaces.data,
            configs: configs.data,
            auth_login_resp: auth_resp,
        }
    }

}
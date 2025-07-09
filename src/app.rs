use ratatui::crossterm;
use ratatui::style::{Style, Color};
use ratatui::widgets::{Block, Borders};
use tui_textarea::{Input, TextArea};

use crate::api::auth::login;

use crate::api::namespace;
use crate::resp::namespace_list_resp::{self, Namespace};

use crate::api::config;
use crate::resp::config_list_resp::Config;

use crate::config::{load_config, AppConfig};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Running,
    Quitting,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CurrentMenu {
    Config,
    Service,
    Namespace,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CurrentScreen {
    Main,
    NamespaceAdd,
}

pub struct App<'a> {
    pub state: AppState,
    pub current_screen: CurrentScreen,
    pub current_menu: CurrentMenu,
    //config
    pub config_current_tab: usize,
    pub configs: Vec<Config>,

    // service

    // namespace
    pub namespaces: Vec<Namespace>,
    pub namespace_current_line: u8, 

    pub namespace_current_edit_index: usize, 

    //0-id, 1-name, 2-desc
    pub ns_add_textarea_vec: Vec<TextArea<'a>>,

}

impl App<'_> {
    pub async fn new() -> App<'static> {
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

        let mut id_textarea = TextArea::default();
        id_textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("ns_id(auto generated if empty)"));

        let mut name_textarea = TextArea::default();
        name_textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("ns_name"));

        let mut desc_textarea = TextArea::default();
        desc_textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("ns_desc"));

        let mut ns_add_textarea_vec = vec![id_textarea, name_textarea, desc_textarea];

        App {
            state: AppState::Running,
            current_screen: CurrentScreen::Main,
            current_menu: CurrentMenu::Config,

            configs: configs.data,
            config_current_tab: 0,

            namespaces: namespaces.data,
            namespace_current_line: 0,

            namespace_current_edit_index: 0,
            ns_add_textarea_vec,

        }
    }

    pub fn handle_input(&mut self, input: Input) {
        let index = self.namespace_current_edit_index;
        self.ns_add_textarea_vec[index].input(input);
    }

}
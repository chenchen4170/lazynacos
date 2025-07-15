use ratatui::widgets::{Block, Borders};
use tui_textarea::{CursorMove, Input, TextArea};

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
    NamespaceDelete,
    NamespaceAdd,
    NamespaceEdit,
}

pub struct ConfigItem {
    pub data_id: String,
    pub group: String,
    pub format: String,
    pub content: String,
}

pub struct NamespaceItem {
    pub ns_name: String,
    pub ns_id: String,
    pub ns_desc: Option<String>,
    pub quota: i32,
    pub config_count: i32,
    pub ns_typs: i32, //0:默认，2:用户创建
}

pub struct App<'a> {

    pub state: AppState,
    pub current_screen: CurrentScreen,
    pub current_menu: CurrentMenu,

    //config配置列表
    pub config_current_tab: usize,
    pub config_list: Vec<ConfigItem>,

    // service服务列表

    // namespace命名空间列表
    pub namespace_list: Vec<NamespaceItem>,
    pub namespace_current_line: u8, 

    pub namespace_current_edit_index: usize,  
    pub ns_add_textarea_vec: Vec<TextArea<'a>>, //0-id, 1-name, 2-desc

}

impl App<'_> {
    pub fn new() -> App<'static> {

        App {
            state: AppState::Running,
            current_screen: CurrentScreen::Main,
            current_menu: CurrentMenu::Config,

            config_list: vec![
                ConfigItem {
                    data_id: "example-data-id".to_string(),
                    group: "example-group".to_string(),
                    format: "json".to_string(),
                    content: "example content".to_string(),
                },
                ConfigItem {
                    data_id: "another-data-id".to_string(),
                    group: "another-group".to_string(),
                    format: "xml".to_string(),
                    content: "another content".to_string(),
                },
            ],
            config_current_tab: 0,

            namespace_list:  vec![
                NamespaceItem { ns_name: "public".to_string(),
                    ns_id: "".to_string(),
                    ns_desc: None,
                    quota: 200,
                    config_count: 2,
                    ns_typs: 0,
                },
                NamespaceItem {
                    ns_name: "cc-test".to_string(),
                    ns_id: "12a6c145-6632-4131-9f1d-a04e46d9e714".to_string(),
                    ns_desc: Some("cc-test desc".to_string()),
                    quota: 200,
                    config_count: 2,
                    ns_typs: 2,
                },
                NamespaceItem {
                    ns_name: "cc-test2".to_string(),
                    ns_id: "12a6c145-6632-4131-9f1d-a04e46d9e715".to_string(),
                    ns_desc: Some("cc-test2 desc".to_string()),
                    quota: 200,
                    config_count: 0,
                    ns_typs: 2,
                },
            ],
            namespace_current_line: 0,

            namespace_current_edit_index: 0,
            ns_add_textarea_vec: vec![],
        }
    }

    pub fn handle_input(&mut self, input: Input) {
        let index = self.namespace_current_edit_index;
        self.ns_add_textarea_vec[index].input(input);
    }

    pub fn move_screen_main_to_ns_add(&mut self) {
        self.current_screen = CurrentScreen::NamespaceAdd;

        let mut id_textarea = TextArea::default();
        let mut name_textarea = TextArea::default();
        let mut desc_textarea = TextArea::default();

        self.ns_add_textarea_vec = vec![id_textarea, name_textarea, desc_textarea];
    }

    pub fn move_screen_main_to_ns_edit(&mut self) {
        self.current_screen = CurrentScreen::NamespaceEdit;

        let ns_item = &self.namespace_list[self.namespace_current_line as usize];

        let mut name = vec![ns_item.ns_name.clone()];
        let mut name_textarea = TextArea::new(name);
        name_textarea.move_cursor(CursorMove::End);

        let mut desc = vec![ns_item.ns_desc.clone().unwrap_or_default()]; 
        let mut desc_textarea = TextArea::new(desc);
        desc_textarea.move_cursor(CursorMove::End);

        self.ns_add_textarea_vec = vec![name_textarea, desc_textarea];
    }

    pub fn move_screen_main_to_ns_delete(&mut self) {
        self.current_screen = CurrentScreen::NamespaceDelete;
    }

    pub fn move_screen_ns_add_to_main(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.namespace_current_edit_index = 0;
        self.ns_add_textarea_vec.clear();
    }

    pub fn ns_add_submit(&mut self) {
        //get info from ns_add_textarea_vec
        let id = self.ns_add_textarea_vec[0].lines().join("\n");
        let name = self.ns_add_textarea_vec[1].lines().join("\n");
        let desc = self.ns_add_textarea_vec[2].lines().join("\n");
        let new_ns = NamespaceItem {
            ns_name: name,
            ns_id: id,
            ns_desc: if desc.is_empty() { None } else { Some(desc) },
            quota: 200, // default quota
            config_count: 0, // default config count
            ns_typs: 2, // user created type
        };
        self.namespace_list.push(new_ns);
        self.move_screen_ns_add_to_main();
    }

    pub fn ns_edit_submit(&mut self) {
        //get info from ns_add_textarea_vec
        let name = self.ns_add_textarea_vec[0].lines().join("\n");
        let desc = self.ns_add_textarea_vec[1].lines().join("\n");

        let ns_item = &mut self.namespace_list[self.namespace_current_line as usize];
        ns_item.ns_name = name;
        ns_item.ns_desc = if desc.is_empty() { None } else { Some(desc) };

        self.move_screen_ns_add_to_main();
    }

    pub fn ns_delete(&mut self) {
        // 移除当前选项
        if self.namespace_current_line < self.namespace_list.len() as u8 {
            self.namespace_list.remove(self.namespace_current_line as usize);
            if self.namespace_current_line > 0 {
                self.namespace_current_line -= 1;
            }
        }
        self.current_screen = CurrentScreen::Main;
    }

}
use ratatui::{
    layout::{Constraint, Direction, Layout, Position, Rect}, 
    style::{Color, Style, Stylize}, 
    text::{self, Line, Span, Text}, 
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph, Tabs, Widget, Wrap}, 
    Frame
};
use tui_textarea::TextArea;

use crate::{app::{self, App}, main};

pub fn ui(frame: &mut Frame, app: &mut App) {
    // Create the layout sections. 
    let [main_rect, hint_rect]= Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1)])
        .areas(frame.area());

    let [menu_rect, body_rect] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), 
            Constraint::Percentage(75)]) 
        .areas(main_rect);

    // Split sub_chunks[0] into three vertical sections
    let [config_rect, service_rect, namespace_rect] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33), //config
            Constraint::Percentage(33), //service
            Constraint::Percentage(34), //namespace
        ])
        .areas(menu_rect);

    // Config section
    let config_block = Block::default()
        .borders(Borders::ALL)
        .title("[1] Config")
        .style(Style::default())
        .fg(match app.current_menu {
            app::CurrentMenu::Config => Color::Green,
            _ => Color::Reset,
        });
    let config_text = Paragraph::new(
        Text::styled(
            "Config List",
             Style::default().fg(Color::Reset)))
        .block(config_block);

    frame.render_widget(config_text, config_rect);

    // Service section
    let service_block = Block::default()
        .borders(Borders::ALL)
        .title("[2] Service")
        .style(Style::default())
        .fg(match app.current_menu{
            app::CurrentMenu::Service => Color::Green,
            _ => Color::Reset,
        });
    let service_text = Paragraph::new(
        Text::styled(
            "Todo", 
            Style::default().fg(Color::Reset)))
        .block(service_block);

    frame.render_widget(service_text, service_rect);

    // Namespace section
    let namespace_block = Block::default()
        .borders(Borders::ALL)
        .title("[3] Namespace")
        .style(Style::default())
        .fg(match app.current_menu{
            app::CurrentMenu::Namespace => Color::Green,
            _ => Color::Reset,
        });
    let namespace_text = Paragraph::new(
        Text::styled(
            "Namespace List", 
            Style::default().fg(Color::Reset)))
        .block(namespace_block);

    frame.render_widget(namespace_text, namespace_rect);

    // content section
    let content_block = Block::default()
        .borders(Borders::ALL)
        .title("content")
        .style(Style::default());
        
    // 根据menu选择渲染不同的内容
    if app.current_menu == app::CurrentMenu::Config {

        let [tab_rect, content_rect] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs section height
            Constraint::Min(1),    // Content section takes the remaining space
        ])
        .areas(body_rect);

        let titles: Vec<Span> = app.namespaces
            .iter()
            .map(|ns| Span::styled(
                ns.namespaceShowName.clone(), 
                Style::default()))
            .collect();
        let tabs = Tabs::new(titles)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" config management "))
        .style(Style::default())
        .highlight_style(Style::default().fg(Color::Green)) // Highlight the selected tab
        .select(app.config_current_tab); 

        frame.render_widget(tabs, tab_rect);

        // 命名空间下的配置列表
        let header = ListItem::new(Text::styled(
            format!("{:<20} {:<20} {:<20}", "data_id", "group", "type"),
            Style::default().fg(Color::Yellow),
        ));
        let config_items: Vec<ListItem> = app.configs
            .iter()
            .map(|config| {
                ListItem::new(Text::styled(
                    format!("{:<20} {:<20} {:<20}", 
                    config.dataId, config.group, config.type_),
                    Style::default(),
                ))
            })
            .collect();
        let mut items = vec![header];
        items.extend(config_items);

        let config_list = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Configs"))
            .style(Style::default());
        frame.render_widget(config_list, content_rect);
    }
    else if app.current_menu == app::CurrentMenu::Service {
        let content_text = Paragraph::new(Text::styled(
            "Todo", Style::default()))
            .block(content_block);
        frame.render_widget(content_text, body_rect);
    }
    else if app.current_menu == app::CurrentMenu::Namespace {
        let header = ListItem::new(Text::styled(
            format!("{:<20} {:<36} {:<20} {:<15}", 
                "ns_name", "ns_id", "desc", "config_count/quota"),
            Style::default().fg(Color::Yellow),
        ));

        let namespace_items: Vec<ListItem> = app.namespaces
            .iter()
            .enumerate()
            .map(|(index, namespace)| {
                let is_selected = index as u8 == app.namespace_current_line;
                let item_style = if is_selected {
                    Style::default().bg(Color::Gray)
                } else {
                    Style::default()
                };

                let content = format!(
                    "{:<20} {:<36} {:<20} {:<15}",
                    namespace.namespaceShowName,
                    namespace.namespace,
                    namespace.namespaceDesc.clone().unwrap_or_else(|| "N/A".to_string()),
                    format!("{}/{}", namespace.configCount, namespace.quota)
                );

                ListItem::new(Text::raw(content))
                    .style(item_style)
            })
            .collect();

        // Combine header and namespace rows
        let mut items = vec![header];
        items.extend(namespace_items);
        
        let namespace_list = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Namespaces"))
            .style(Style::default());

        frame.render_widget(namespace_list, body_rect);
    }

    // 根据menu选择渲染不同的hint
    if app.current_menu == app::CurrentMenu::Config {
        let hint = Paragraph::new("press q to exit");
        frame.render_widget(hint, hint_rect);
    }
    else if app.current_menu == app::CurrentMenu::Service {
        let hint = Paragraph::new("press q to exit");
        frame.render_widget(hint, hint_rect);
    }
    else if app.current_menu == app::CurrentMenu::Namespace {
        let hint = Paragraph::new(
            Line::from(vec![
                Span::raw("esc: cancel, "),
                Span::raw("a: add, "),
                Span::raw("r: refresh, "),
                Span::raw("d: delete, "),
                Span::raw("e: edit"),
                Span::raw("/: search")
            ])
        );
        frame.render_widget(hint, hint_rect);
    }

    // 添加 Namespace 的弹出窗口
    if app.current_screen == app::CurrentScreen::NamespaceAdd {
        let area = centered_rect(60, 50, frame.area());
        frame.render_widget(Clear, area); //清空背景内容

        let popup_block = Block::default()
            .title("Create Namespace")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));

        frame.render_widget(popup_block, area);

        let [id_rect, name_rect, desc_rect, hint_rect] = Layout::default()
            .direction(Direction::Vertical)
            .margin(1) // popup_chunks与area之间的间距
            .constraints([
                Constraint::Length(3), // ns_id row
                Constraint::Length(3), // ns_name row
                Constraint::Length(3), // ns_desc row
                Constraint::Length(1), // hint
            ])
            .areas(area);

        //遍历app.ns_add_textarea_vec，渲染每个TextArea
        for (i, textarea) in app.ns_add_textarea_vec.iter_mut().enumerate() {
            if app.namespace_current_edit_index == i {
                //fix 改成边框高亮
                textarea.set_style(Style::default().fg(Color::Green)); // 高亮当前编辑的TextArea
            } else {
                textarea.set_style(Style::default()); // 恢复默认样式
            }
            let rect = match i {
                0 => id_rect,   // ns_id
                1 => name_rect, // ns_name
                2 => desc_rect, // ns_desc
                _ => continue,  // 其他情况不处理
            };
            frame.render_widget(&*textarea, rect);
        }

        let hint_text = Paragraph::new(
            Line::from(vec![
                Span::raw("esc: cancel, "),
                Span::raw("enter: submit, "),
                Span::raw("tab: switch focus")
            ]));
        frame.render_widget(hint_text, hint_rect);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
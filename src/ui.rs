use ratatui::{
    layout::{Constraint, Direction, Layout, Rect}, 
    style::{Color, Style, Stylize}, 
    text::{Line, Span, Text}, 
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph, Tabs, Wrap}, 
    Frame
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    // Create the layout sections. 
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1), // main content area
            Constraint::Length(1)]) //hint section
        .split(frame.area());

    // Split chunks[0] into two horizontal sections
    let sub_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), //nav
            Constraint::Percentage(75)]) //content
        .split(chunks[0]);

    // Split sub_chunks[0] into three vertical sections
    let left_sub_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33), //config
            Constraint::Percentage(33), //service
            Constraint::Percentage(34), //namespace
        ])
        .split(sub_chunks[0]);

    // Config section
    let config_block = Block::default()
        .borders(Borders::ALL)
        .title("[1] Config")
        .style(Style::default())
        .fg(match app.current_block {
            1 => Color::Green,
            _ => Color::Reset,
        });
    let config_text = Paragraph::new(
        Text::styled(
            "Config Section Content",
             Style::default().fg(Color::Reset)))
        .block(config_block);

    frame.render_widget(config_text, left_sub_chunks[0]);

    // Service section
    let service_block = Block::default()
        .borders(Borders::ALL)
        .title("[2] Service")
        .style(Style::default())
        .fg(match app.current_block {
            2 => Color::Green,
            _ => Color::Reset,
        });
    let service_text = Paragraph::new(
        Text::styled(
            "Service Section Content", 
            Style::default().fg(Color::Reset)))
        .block(service_block);

    frame.render_widget(service_text, left_sub_chunks[1]);

    // Namespace section
    let namespace_block = Block::default()
        .borders(Borders::ALL)
        .title("[3] Namespace")
        .style(Style::default())
        .fg(match app.current_block {
            3 => Color::Green,
            _ => Color::Reset,
        });
    let namespace_text = Paragraph::new(
        Text::styled(
            "Namespace Section Content", 
            Style::default().fg(Color::Reset)))
        .block(namespace_block);

    frame.render_widget(namespace_text, left_sub_chunks[2]);

    // content section
    let content_block = Block::default()
        .borders(Borders::ALL)
        .title("content")
        .style(Style::default());
        
    if app.current_block == 1 {

        let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs section height
            Constraint::Min(1),    // Content section takes the remaining space
        ])
        .split(sub_chunks[1]);

        let titles: Vec<Span> = app.namespaces
            .iter()
            .map(|ns| Span::styled(ns.namespaceShowName.clone(), Style::default()))
            .collect();
        let tabs = Tabs::new(titles)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" config management "))
        .style(Style::default())
        .highlight_style(Style::default().fg(Color::Green)) // Highlight the selected tab
        .select(app.current_tab); // 

        frame.render_widget(tabs, content_chunks[0]);

        // 命名空间下的配置列表
        let header = ListItem::new(Text::styled(
            format!("{:<20} {:<20} {:<20}", "data_id", "group", "type"),
            Style::default().fg(Color::Yellow),
        ));
        let config_items: Vec<ListItem> = app.configs
            .iter()
            .map(|config| {
                ListItem::new(Text::styled(
                    format!(
                        "{:<20} {:<20} {:<20}",
                        config.dataId,
                        config.group,
                        config.type_),
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
        frame.render_widget(config_list, content_chunks[1]);

        // let content_text = Paragraph::new(Text::styled(
        //     format!("Selected namespace: {}", app.namespaces[app.current_tab].namespaceShowName),
        //     Style::default(),
        // ))
        // .block(Block::default()
        //     .borders(Borders::ALL))
        //     .style(Style::default());

        // frame.render_widget(content_text, content_chunks[1]);
    }
    else if app.current_block == 2 {
        let content_text = Paragraph::new(Text::styled(
            "Service Section Content", Style::default()))
            .block(content_block);
        frame.render_widget(content_text, sub_chunks[1]);
    }
    else if app.current_block == 3 {
        // Create the header row
        let header = ListItem::new(Text::styled(
            format!("{:<20} {:<36} {:<20} {:<15}", "ns_name", "ns_id", "desc", "config_count/quota"),
            Style::default().fg(Color::Yellow),
        ));

        // Create rows for each namespace
        let namespace_items: Vec<ListItem> = app.namespaces
            .iter()
            .map(|namespace| {
                ListItem::new(Text::styled(
                    format!(
                        "{:<20} {:<36} {:<20} {:<15}",
                        namespace.namespaceShowName,
                        namespace.namespace,
                        namespace.namespaceDesc.clone().unwrap_or_else(|| "".to_string()),
                        format!("{}/{}", namespace.configCount, namespace.quota)
                    ),
                    Style::default(),
                ))
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

        frame.render_widget(namespace_list, sub_chunks[1]);
    }

    //hint
    let hint = Paragraph::new("press q to exit");
    frame.render_widget(hint, chunks[1]);
}

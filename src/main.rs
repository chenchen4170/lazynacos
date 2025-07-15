use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use tui_textarea::{Input, TextArea};

mod app;
mod ui;
mod resp;
mod config;
mod api;
use crate::{
    app::{App, AppState,},
    ui::ui,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.state{
                AppState::Quitting => {
                    return Ok(true);
                }
                AppState::Running => match app.current_screen {
                    app::CurrentScreen::Main => {
                        match key.code {
                            KeyCode::Char('q') =>{
                                app.state = AppState::Quitting;
                                return Ok(true);
                            }
                            KeyCode::Char('1') => {
                                app.current_menu = app::CurrentMenu::Config;
                            }
                            KeyCode::Char('2') => {
                                app.current_menu = app::CurrentMenu::Service;
                            }
                            KeyCode::Char('3') => {
                                app.current_menu = app::CurrentMenu::Namespace;
                            }
                            //namespace
                            KeyCode::Up | KeyCode::Char('k') => {
                                if app.current_menu == app::CurrentMenu::Namespace {
                                    if app.namespace_current_line > 0 {
                                        app.namespace_current_line -= 1;
                                    }
                                }
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                if app.current_menu == app::CurrentMenu::Namespace {
                                    if app.namespace_current_line < (app.namespace_list.len() as u8 - 1) {
                                        app.namespace_current_line += 1;
                                    }
                                }
                            }
                            KeyCode::Char('d') => {
                                if app.current_menu == app::CurrentMenu::Namespace {
                                    app.move_screen_main_to_ns_delete();
                                }
                            }
                            KeyCode::Char('a') => {
                                if app.current_menu == app::CurrentMenu::Namespace {
                                    app.move_screen_main_to_ns_add();
                                }
                            }
                            KeyCode::Char('e') => {
                                if app.current_menu == app::CurrentMenu::Namespace {
                                    app.move_screen_main_to_ns_edit();
                                }
                            }
                            _ => {}
                        }
                    }
                    app::CurrentScreen::NamespaceDelete => {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('n') => {
                                app.current_screen = app::CurrentScreen::Main;
                            }
                            KeyCode::Char('y') => {
                                app.ns_delete();
                            }
                            _ => {
                                // Handle other keys if necessary
                            }
                        }
                    }
                    app::CurrentScreen::NamespaceAdd => {
                        match key.code {
                            KeyCode::Esc => {
                                app.move_screen_ns_add_to_main();
                            }
                            KeyCode::Tab => {
                                // 切换输入焦点
                                app.namespace_current_edit_index = (app.namespace_current_edit_index + 1) % 3;
                            }
                            KeyCode::Enter => {
                                app.ns_add_submit();
                            }
                            _ => {
                                let input = Input::from(key);
                                app.handle_input(input);
                            }
                        }
                    }
                    app::CurrentScreen::NamespaceEdit => {
                        match key.code {
                            KeyCode::Esc => {
                                app.move_screen_ns_add_to_main();
                            }
                            KeyCode::Tab => {
                                // 切换输入焦点
                                app.namespace_current_edit_index = (app.namespace_current_edit_index + 1) % 2;
                            }
                            KeyCode::Enter => {
                                app.ns_edit_submit();
                            }
                            _ => {
                                let input = Input::from(key);
                                app.handle_input(input);
                            }
                        }
                    }
                }
            }
        }
    }
}
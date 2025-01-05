mod app;
mod ui;
mod library;

use app::{App, CurrentScreen};
use crossterm::event::{self, DisableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use library::LibrarySearchCriteria;
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::Terminal;
use ui::ui;
use std::error::Error;
use std::io;

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), io::Error> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Loading => match key.code {
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        app.load();
                        if app.loaded {
                            app.current_screen = CurrentScreen::Home;
                        } else {
                            app.current_screen = CurrentScreen::NewOwner;
                        }
                    }
                    _ => {}
                },
                CurrentScreen::NewOwner => match key.code {
                    KeyCode::Char(value) => {
                        app.owner_input.push(value);
                    }
                    KeyCode::Enter => {
                        app.initialize_demo();
                        app.current_screen = CurrentScreen::Home;
                    }
                    KeyCode::Backspace => {
                        app.owner_input.pop();
                    }
                    _ => {}
                },
                CurrentScreen::Home => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Char('s') => {
                        app.current_screen = CurrentScreen::Searching;
                        app.term_input_mode = true;
                        app.searching_input.clear();
                    }
                    _ => {}
                },
                CurrentScreen::Searching => match key.code {
                    KeyCode::Tab => {
                        app.term_input_mode = !app.term_input_mode;
                    }
                    KeyCode::Char(value) => {
                        if app.term_input_mode {
                            app.searching_input.push(value);
                        } else {
                            match value {
                                't' => app.searching_criteria = LibrarySearchCriteria::Title,
                                'a' => app.searching_criteria = LibrarySearchCriteria::Author,
                                'i' => app.searching_criteria = LibrarySearchCriteria::ISBN,
                                'q' => app.current_screen = CurrentScreen::Exiting,
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        app.searching_input.pop();
                    }
                    KeyCode::Enter => {
                        app.apply_search();
                        if app.selected_book.is_some() {
                            app.current_screen = CurrentScreen::CheckingOut;
                        }
                    }
                    _ => {}
                },
                CurrentScreen::CheckingOut => match key.code {
                    KeyCode::Enter => {
                        app.check_out();
                        if app.checkout_success.is_none() {
                            app.error_message = Some("Book not found".to_string());
                        }
                        app.current_screen = CurrentScreen::CheckedOutResult;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    },
                    KeyCode::Char('b') => {
                        app.current_screen = CurrentScreen::Home;
                    },
                    _ => {}
                },
                CurrentScreen::CheckedOutResult => match key.code {
                    KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Home;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(());
                    }
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::Home;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(_) = res {
        app.library.unwrap().save(&app.config.library_path)?;
    } else
    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

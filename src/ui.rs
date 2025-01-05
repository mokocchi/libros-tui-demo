use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn popup_screen(frame: &mut Frame, title: &str, message: &str, borders: Borders) {
    frame.render_widget(Clear, frame.area());
    let popup_block = Block::default()
        .title(title)
        .title_alignment(ratatui::layout::Alignment::Center)
        .borders(borders)
        .padding(Padding::new(0, 0, 1, 1))
        .style(Style::default());

    let text = Text::styled(message, Style::default());
    let paragraph = Paragraph::new(text).block(popup_block).centered();

    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(paragraph, area);
}

fn loading_screen(frame: &mut Frame) {
    popup_screen(
        frame,
        "Loading library, press Enter and wait...",
        "You can cancel by pressing ESC",
        Borders::ALL,
    );
}

fn new_owner_screen(frame: &mut Frame, app: &App) {
    let owner_text = format!(
        "Enter the owner of the library: {}",
        app.owner_input.clone()
    );
    popup_screen(frame, "New Library", &owner_text, Borders::ALL);
}

fn checked_out_result_screen(frame: &mut Frame, app: &App) {
    let result_text = match app.checkout_success.as_ref() {
        Some(Err(e)) => &format!("Error: {}\nPress Enter", e),
        Some(Ok(_)) => "Success",
        None => "Nothing happened",
    };
    popup_screen(frame, "Checkout Result", result_text, Borders::ALL);
}

fn exiting_screen(frame: &mut Frame) {
    popup_screen(
        frame,
        "Exiting Library Management Tool",
        "Are you sure you want to exit? (y/n)",
        Borders::ALL,
    );
}

fn main_screen_title_bar(frame: &mut Frame, app: &App, area: Rect) {
    let title = format!(
        "Library Management Tool - {}'s Library",
        app.library.as_ref().unwrap().get_owner()
    );
    let title_text = Text::styled(title, Style::default().fg(Color::White).bg(Color::Black));
    let title_paragraph = Paragraph::new(title_text)
        .block(Block::default().borders(Borders::ALL))
        .centered();
    frame.render_widget(title_paragraph, area);
}

fn main_screen_content(frame: &mut Frame, app: &App, area: Rect) {
    let mut list_items = Vec::<ListItem>::new();

    let books = app.library.as_ref().unwrap().get_books();

    for book in books.iter() {
        let item = ListItem::new(Line::from(Span::styled(
            format!("{: <25} - {: <50}", book.get_author(), book.get_title()),
            Style::default().fg(match book.get_available() {
                true => Color::Green,
                false => Color::Red,
            }),
        )));

        list_items.push(item);
    }

    let list = List::new(list_items);
    frame.render_widget(list, area);
}

fn checking_out_screen_content(frame: &mut Frame, app: &App, area: Rect) {
    let book = app.selected_book.as_ref().unwrap();
    let book_info = vec![
        Line::from(Span::styled(
            format!("Title: {}", book.get_title()),
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            format!("Author: {}", book.get_author()),
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            format!("ISBN: {}", book.get_isbn()),
            Style::default().fg(Color::White),
        )),
    ];

    let book_info_paragraph = Paragraph::new(book_info)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false })
        .centered();

    let new_area = centered_rect(60, 25, area);

    frame.render_widget(book_info_paragraph, new_area);
}

fn main_screen_mode_footer(frame: &mut Frame, app: &App, area: Rect) {
    let search_term = Text::from(vec![Line::from(vec![
        "Query: ".into(),
        app.searching_input.clone().into(),
    ])]);

    let navigation_text = match app.current_screen {
        CurrentScreen::Home => Span::styled("Home", Style::default().fg(Color::Green)),
        CurrentScreen::Searching => Span::styled("Search", Style::default().fg(Color::Yellow)),
        CurrentScreen::CheckingOut => Span::styled("Check Out", Style::default().fg(Color::Cyan)),
        _ => Span::default(),
    };

    let status_text = match app.current_screen {
        CurrentScreen::Home => Span::styled("OK", Style::default().fg(Color::DarkGray)),
        CurrentScreen::Searching => match app.term_input_mode {
            true => Span::styled(
                format!("Searching by {} - {}", app.searching_criteria, search_term),
                Style::default().fg(Color::White),
            ),
            false => Span::styled(
                format!("Switching search criteria ({})", app.searching_criteria),
                Style::default().fg(Color::Yellow),
            ),
        },
        CurrentScreen::CheckingOut => {
            let book = app.selected_book.as_ref().unwrap();
            Span::styled(
                format!(
                    "Checking out '{}', by {}",
                    book.get_title(),
                    book.get_author()
                ),
                Style::default().fg(Color::LightBlue),
            )
        }
        _ => Span::default(),
    };
    let current_navigation_text = vec![
        navigation_text,
        Span::styled(" | ", Style::default().fg(Color::White)),
        status_text,
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(mode_footer, area);
}

fn main_screen_key_hints(frame: &mut Frame, app: &App, area: Rect) {
    let mut keys = Vec::<String>::new();
    match app.current_screen {
        CurrentScreen::Home => {
            keys.push("(s) to search".into());
            keys.push("(q) to quit".into());
        }
        CurrentScreen::Searching => {
            if app.term_input_mode {
                keys.push("Type to search".into());
                keys.push("(tab) to switch to search criteria selection".into());
                keys.push("(enter) to check out".into());
                keys.push("(esc) main screen".into());
            } else {
                keys.push("(a) by Author".into());
                keys.push("(t) by Title".into());
                keys.push("(i) by ISBN".into());
                keys.push("(tab) to switch to query input".into());
                keys.push("(esc) main screen".into());
                keys.push("(q) to quit".into());
            }
        }
        CurrentScreen::CheckingOut => {
            keys.push("(enter) to check out book".into());
            keys.push("(b) to go back".into());
            keys.push("(esc) main screen".into());
            keys.push("(q) to quit".into());
        }
        _ => {}
    }
    let current_keys_hint = Span::styled(keys.join(" - "), Style::default().fg(Color::Green));

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().borders(Borders::NONE))
        .centered();

    frame.render_widget(key_notes_footer, area);
}

fn main_screen(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(6),
        ])
        .split(frame.area());

    main_screen_title_bar(frame, app, chunks[0]);

    match app.current_screen {
        CurrentScreen::CheckingOut => checking_out_screen_content(frame, app, chunks[1]),
        _ => main_screen_content(frame, app, chunks[1]),
    }

    let footer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    main_screen_mode_footer(frame, app, footer_chunks[0]);

    main_screen_key_hints(frame, app, footer_chunks[1]);
}

pub fn ui(frame: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::Loading => loading_screen(frame),
        CurrentScreen::NewOwner => new_owner_screen(frame, app),
        CurrentScreen::CheckedOutResult => checked_out_result_screen(frame, app),
        CurrentScreen::Exiting => exiting_screen(frame),
        _ => main_screen(frame, app),
    }
}

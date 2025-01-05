use std::fmt::format;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::{
    app::{App, CurrentScreen},
    library::LibrarySearchCriteria,
};

pub fn ui(frame: &mut Frame, app: &App) {
    if let CurrentScreen::Loading = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Loading library, press Enter and wait...")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "You can cancel by pressing ESC",
            Style::default().fg(Color::Red),
        );
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
        return;
    }

    if let CurrentScreen::NewOwner = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("New Library")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let owner_text = Text::from(vec![Line::from(vec![
            "Owner: ".into(),
            app.owner_input.clone().into(),
        ])]);
        let exit_paragraph = Paragraph::new(owner_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
        return;
    }

    if let CurrentScreen::CheckedOutResult = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Checkout Result")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let result_text = match app.checkout_success.as_ref() {
            Some(Err(e)) => Text::from(vec![Line::from(vec![
                "Error: ".into(),
                e.clone().into(),
            ]), 
            Line::from(vec![
                "Press Enter".into(),
            ])]),
            Some(Ok(_)) => Text::from("Success".to_string()),
            None => Text::from("Nothing happened".to_string()),
        };
        let exit_paragraph = Paragraph::new(result_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let owner = app.library.as_ref().unwrap().get_owner();

    let title = Paragraph::new(Text::styled(
        format(format_args!("Library Management Tool: {}'s library", owner)),
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();

    let books = app.library.as_ref().unwrap().get_books();

    for book in books.iter() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} - {: <50}", book.get_author(), book.get_title()),
            Style::default().fg(match book.get_available() {
                true => Color::Green,
                false => Color::Red,
            }),
        ))));
    }

    let list = List::new(list_items);

    frame.render_widget(list, chunks[1]);

    let search_term = Text::from(vec![Line::from(vec![
        "Query: ".into(),
        app.searching_input.clone().into(),
    ])]);

    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Home => Span::styled("Home", Style::default().fg(Color::Green)),
            CurrentScreen::Searching => {
                Span::styled(search_term.to_string(), Style::default().fg(Color::Yellow))
            }
            CurrentScreen::CheckingOut => {
                Span::styled("Checking Out", Style::default().fg(Color::LightBlue))
            }
            _ => Span::default(),
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        {
            match app.current_screen {
                CurrentScreen::Home => {
                    Span::styled("Nothing selected", Style::default().fg(Color::DarkGray))
                }
                CurrentScreen::Searching => match app.term_input_mode {
                    true => match app.searching_criteria {
                        LibrarySearchCriteria::Author => {
                            Span::styled("Searching by author", Style::default().fg(Color::Yellow))
                        }
                        LibrarySearchCriteria::Title => {
                            Span::styled("Searching by title", Style::default().fg(Color::Yellow))
                        }
                        LibrarySearchCriteria::ISBN => {
                            Span::styled("Searching by ISBN", Style::default().fg(Color::Yellow))
                        }
                    },
                    false => Span::styled(
                        format!("Switching search criteria ({})", app.searching_criteria),
                        Style::default().fg(Color::Yellow),
                    ),
                },
                CurrentScreen::CheckingOut => {
                    let book = app.selected_book.as_ref().unwrap();
                    Span::styled(
                        format!("{}, by {}", book.get_title(), book.get_author()),
                        Style::default().fg(Color::LightBlue),
                    )
                }
                _ => Span::default(),
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Home => Span::styled(
                "(s) to search - (q) to quit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Searching => Span::styled(
                "(a) by author - (t) by title - (i) by ISBN - (q) to exit - (tab) to switch input mode", 
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::CheckingOut => Span::styled(
                "(enter) to check out - (b) to go to Home screen - (q) to quit",
                Style::default().fg(Color::Red),
            ),
            _ => Span::styled("Press 'q' to exit", Style::default().fg(Color::Red)),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Exiting Library Management Tool")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Are you sure you want to exit? (y/n)",
            Style::default().fg(Color::Red),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
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

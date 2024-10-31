use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(frame: &mut Frame, app: &mut App) {
    // Create the layout sections.
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

    let title = Paragraph::new(Text::styled(
        "LOFAR MSOxide",
        Style::default().fg(Color::Green),
    ))
    .block(title_block)
    .centered()
    .bold();

    frame.render_widget(title, chunks[0]);
    let mut table_items = Vec::<ListItem>::new();
    let mut column_items = Vec::<ListItem>::new();

    for key in app.tables.iter() {
        table_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25}", key),
            Style::default().fg(Color::Yellow),
        ))));
    }

    for key in app.columns.iter() {
        column_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25}", key),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let [left, right] =
        Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(75)])
            .areas(chunks[1]);
    let [top_left, bottom_left] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(left);
    // Adapt the number of lines we'll read in the info panel
    // based on how many lines are shown in the terminal.
    app.line_height = right.height;

    let mut solset_block = Block::default()
        .borders(Borders::ALL)
        .title("Tables")
        .style(Style::default());
    let mut soltab_block = Block::default()
        .borders(Borders::ALL)
        .title("Fields")
        .style(Style::default());
    let mut info_block = Block::default()
        .borders(Borders::ALL)
        .title("Information")
        .style(Style::default());
    let active_style = Style::default().bg(Color::White).fg(Color::Black);
    match &app.currently_editing {
        CurrentlyEditing::Table => solset_block = solset_block.border_style(active_style),
        CurrentlyEditing::Column => soltab_block = soltab_block.border_style(active_style),
        CurrentlyEditing::Information => info_block = info_block.border_style(active_style),
    }

    let table_list = List::new(table_items)
        .block(solset_block)
        .highlight_style(Style::default().bold())
        .highlight_symbol(">> ");
    let mut table_list_state = ListState::default();
    table_list_state.select(Some(app.current_table));

    let column_list = List::new(column_items)
        .block(soltab_block)
        .highlight_style(Style::default().bold())
        .highlight_symbol(">> ")
        .style(Style::default().fg(Color::White));
    let mut column_list_state = ListState::default();
    column_list_state.select(Some(app.current_column));

    let info_text = Paragraph::new(app.text_buffer.clone())
        .block(info_block)
        .wrap(Wrap { trim: true })
        .scroll((app.text_scroll, 0));

    frame.render_stateful_widget(table_list, top_left, &mut table_list_state);
    frame.render_stateful_widget(column_list, bottom_left, &mut column_list_state);
    frame.render_widget(info_text, right);
    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main | CurrentScreen::Help => {
                Span::styled("Normal Mode", Style::default().fg(Color::Green))
            }
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            match &app.currently_editing {
                CurrentlyEditing::Table => Span::styled(
                    "<up/down> move / <Enter> Select / <Tab> switch panel",
                    Style::default().fg(Color::LightGreen),
                ),
                CurrentlyEditing::Column => Span::styled(
                    "<up/down> move / <Enter> Select / <Tab> switch panel",
                    Style::default().fg(Color::LightGreen),
                ),
                CurrentlyEditing::Information => Span::styled(
                    "<up/down> scroll text / <Tab> switch panel",
                    Style::default().fg(Color::LightGreen),
                ),
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main | CurrentScreen::Help => {
                Span::styled("<q> to quit", Style::default().fg(Color::Red))
            }
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().borders(Borders::ALL))
        .centered();

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    if let CurrentScreen::Exiting = app.current_screen {
        let popup_block = Block::default()
            .title("Exiting...")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled("Exit LOFAR H5stat? (y/n)", Style::default().fg(Color::Red));
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .centered()
            .wrap(Wrap { trim: false });

        let area = centered_rect(30, 10, frame.area());
        frame.render_widget(exit_paragraph, area);
    }

    if let CurrentScreen::Help = app.current_screen {
        let popup_block = Block::default()
            .title(" Help (q/Esc to exit)")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));

        let help_text = Text::styled(
            "Tab - cycle through panels\nup/down/j/k - select entry or move through data by one line\nJ/K - move through data by 10 lines\nEnter - load data from field\nq - quit program",
            Style::default().fg(Color::White),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let help_paragraph = Paragraph::new(help_text)
            .block(popup_block)
            .centered()
            .wrap(Wrap { trim: false });

        let area = centered_rect(45, 65, frame.area());
        frame.render_widget(help_paragraph, area);
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

//Where ui portion of app is handled such as
//drawing graphical cli text, update, etc.

use crate::App;
use crate::DiskDisplay;
use tui::layout::Rect;
use tui::widgets::Tabs;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(67)].as_ref())
        .split(f.size());
    //TODO main graphic here
    let block = Block::default().borders(Borders::ALL).title("Main canvas");
    f.render_widget(block, chunks[0]);

    match app.status.index {
        0 => draw_drive_selection(f, app, chunks[1]),
        1 => draw_wipe_method_selection(f, app, chunks[1]),
        2 => draw_confirmation(f, app, chunks[1]),
        _ => {}
    }
}

fn draw_drive_selection<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(67), Constraint::Percentage(33)].as_ref())
        .split(area);
    // Iterate through all elements in the `items` app and append some debug text to it.
    let entries: Vec<ListItem> = app
        .drives
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(i.name.to_str().unwrap())];
            ListItem::new(lines).style(Style::default())
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(entries)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.status.titles[app.status.index]),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, chunks[0], &mut app.drives.state);

    let current_index = app.drives.state.selected();
    if current_index != None {
        let selected_drive: &DiskDisplay = &app.drives.items[current_index.unwrap()];
        let text = vec![
            Spans::from(Span::styled(
                format!(
                    "Available space: {}",
                    selected_drive.available_space.to_string()
                ),
                Style::default().bg(Color::Yellow),
            )),
            Spans::from(Span::styled(
                format!("Total space: {}", selected_drive.total_space.to_string()),
                Style::default().bg(Color::Green),
            )),
            Spans::from(Span::styled(
                format!("Mount: {}", selected_drive.mount_point.to_str().unwrap()),
                Style::default().bg(Color::Magenta),
            )),
            Spans::from(Span::styled(
                format!(
                    "File System: {}",
                    std::str::from_utf8(selected_drive.file_system).unwrap()
                ),
                Style::default().bg(Color::Blue),
            )),
        ];
        let paragraph = Paragraph::new(text.clone()).style(Style::default()).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Drive Information"),
        );

        //Render left side selection and corresponding info
        f.render_widget(paragraph, chunks[1]);
    }
}

fn draw_wipe_method_selection<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(67), Constraint::Percentage(33)].as_ref())
        .split(area);

    let items: Vec<ListItem> = app
        .deletion_methods
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(*i)];
            ListItem::new(lines).style(Style::default())
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Deletion Methods"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let s = "Lorem ipsem dolor ipset deler runtime ";
    s.repeat(4);
    let info = Paragraph::new(s.clone())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Details"));
    f.render_widget(info, chunks[1]);
    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.deletion_methods.state);
}

fn draw_confirmation<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(area);
    let current_index = app.drives.state.selected();
    let selected_drive: &DiskDisplay = &app.drives.items[current_index.unwrap()];
    let warning = format!(
        "Warning! You are about to permanently erase `{}` this action cannot be undone!
            If you need to ensure a zero chance of data recovery, consider physical destruction",
        selected_drive.name.to_str().unwrap()
    );
    let info = Paragraph::new(warning.clone())
        .style(Style::default().bg(Color::Red))
        .block(Block::default().borders(Borders::ALL).title("Details"));

    let titles = app
        .confirmation
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default())))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Red))
        .select(app.confirmation.index);
    f.render_widget(info, chunks[0]);
    f.render_widget(tabs, chunks[1]);
}






//Where ui portion of app is handled such as 
//drawing graphical cli text, update, etc. 

use crate::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, Borders, List, ListItem,
        Paragraph,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(67), Constraint::Percentage(33)].as_ref())
        .split(f.size());

    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app
        .drives
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(*i)];
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::Blue))
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Available Drives"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let s = "Lorem ipsem dolor ipset deler runtime ";
    s.repeat(4);
    let info = Paragraph::new(s.clone())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Drive Info"));

    f.render_widget(info, chunks[1]);
    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.drives.state);
}

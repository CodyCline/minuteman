//Where ui portion of app is handled such as
//drawing graphical cli text, update, etc.

use tui::widgets::Table;
use crate::App;
use crate::DiskDisplay;
use tui::layout::Rect;
use tui::widgets::Gauge;
use tui::widgets::Tabs;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

const FG_THEME: Color = Color::Rgb(129, 66, 38);
const BG_THEME: Color = Color::Rgb(184, 102, 64);

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(f.size());
    //TODO main graphic here
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Main canvas")
        .style(Style::default().bg(Color::Rgb(32,32,32)));
    f.render_widget(block, chunks[0]);

    match app.status.index {
        0 => draw_drive_selection(f, app, chunks[1]),
        1 => draw_wipe_method_selection(f, app, chunks[1]),
        2 => draw_confirmation(f, app, chunks[1]),
        3 => draw_deletion_progress(f, app, chunks[1]),
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
                .style(Style::default().fg(Color::Yellow).bg(Color::Rgb(32, 32, 32)))
                .title(app.status.titles[app.status.index])
        )
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(229, 83, 0))
                .fg(Color::White)
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
                Style::default().bg(Color::Yellow).fg(Color::White),
            )),
            Spans::from(Span::styled(
                format!("Total space: {}", selected_drive.total_space.to_string()),
                Style::default().bg(Color::Green).fg(Color::White),
            )),
            Spans::from(Span::styled(
                format!("Mount: {}", selected_drive.mount_point.to_str().unwrap()),
                Style::default().bg(Color::Magenta).fg(Color::White),
            )),
            Spans::from(Span::styled(
                format!(
                    "File System: {}",
                    std::str::from_utf8(selected_drive.file_system).unwrap()
                ),
                Style::default().bg(Color::Blue).fg(Color::White),
            )),
        ];
        let paragraph = Paragraph::new(text.clone()).style(Style::default()).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Drive Information")
                .style(Style::default().fg(Color::Yellow).bg(Color::Rgb(32,32,32))),
        );

        //Render right side selection and corresponding info
        f.render_widget(paragraph, chunks[1]);
    } else {
        let paragraph = Paragraph::new("Select a drive for more information").style(Style::default()).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Drive Information")
                .style(Style::default().fg(Color::Yellow).bg(Color::Rgb(32,32,32))),
        );

        //Render right side selection and corresponding info
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
                .title("Deletion Methods")
                .style(Style::default().fg(Color::Yellow).bg(Color::Rgb(32,32,32))),
        )
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(229, 83, 0))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let s = "Lorem ipsem dolor ipset deler runtime ";
    s.repeat(4);
    let info = Paragraph::new(s.clone())
        .style(Style::default().fg(Color::Yellow).bg(Color::Rgb(32,32,32)))
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
        .constraints(
            [
                Constraint::Length(4),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .margin(2)
        .split(area);
    let block = Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Yellow).bg(Color::Rgb(32,32,32)));
    f.render_widget(block, area);


    let current_index = app.drives.state.selected();
    let selected_drive: &DiskDisplay = &app.drives.items[current_index.unwrap()];
    let warning_message = format!(
        "Warning! You are about to permanently erase \"{}\" this action cannot be undone!
        Disk deletion may take some time, leave this window open until the process is completed! 
        If you need to ensure a zero chance of data recovery, consider physical destruction of the drive afterwards. 
        Proceed with caution",
        selected_drive.name.to_str().unwrap()
    );

    let prompt = Paragraph::new("Are you sure you want yo delete this drive?");

    
    let info = Paragraph::new(warning_message.clone())
        .style(Style::default().fg(Color::Red).bg(Color::Rgb(32,32,32)));

    let titles = app
        .confirmation
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default())))
        .collect();
    let tabs = Tabs::new(titles)
        .highlight_style(Style::default().fg(Color::Red))
        .select(app.confirmation.index);
    f.render_widget(info, chunks[0]);
    f.render_widget(prompt, chunks[1]);
    f.render_widget(tabs, chunks[2]);
}

fn draw_deletion_progress<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(area);
    let block = Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Yellow).bg(Color::Rgb(32,32,32)));
    f.render_widget(block, area);

    let message = Paragraph::new("Deletion in progress, do not close this window!")
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(message, chunks[0]);

    

    let label = format!("[{:.2}%, round 1 of 7, pass 1 of 3]", app.deletion_progress * 100.0);
    let gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .label(label)
        .ratio(app.deletion_progress);
    f.render_widget(gauge, chunks[1]);
}

fn draw_status<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    //TODO check status show either success page or error
    // if app.status.errors {
    // }
    // else if app.status.success {
    // }
}

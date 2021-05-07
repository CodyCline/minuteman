#[allow(dead_code)]
mod util;

use crate::util::{
    event::{Event, Events},
    StatefulList,
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use sysinfo::{System, SystemExt, DiskExt};

struct Info (String, String);


pub const WIPE_METHODS: [&str; 8] = [
    "British HMG IS5 (1 rewrite and 1 verify)",
    "Russian GOST P50739-95 (2 rewrites)",
    "NAVSO P-5239-26 (RLL), (3 rewrites and 1 verify)",
    "NAVSO P-5239-26 (ALT), (3 rewrites and 1 verify)",
    "Department of Defense (DoD, USA 5220.22-M) (3 rewrites and 3 verify)",
    "Department of Defense (DoD, USA 5220.22-M ECE) (7 rewrites)",
    "Canadian RCMP TSSIT OPS-II (7 rewrites)",
    "German VSITR (7 rewrites)",
];


enum State {
    MainSelect,
    DeletionMethod,
    DeletionStatus,
    VerifyStatus,
}
/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
struct App<'a> {
    drives: StatefulList<&'a str>,
}


impl<'a> App<'a> {
    
    fn new(drives: Vec<&'a str>) -> App<'a> {
        

        App {
            drives: StatefulList::with_items(drives),
        }
    }

}



fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();
    let mut system = System::new_all();
    system.refresh_all();
    
    let mut drives = Vec::new();
    for disk in system.get_disks() {
        drives.push(disk.get_name().to_str().unwrap());
    }
    // Create a new app with some exapmle state
    let mut app = App::new(drives);
    

    loop {
        terminal.draw(|f| {
            // Create two chunks with equal horizontal screen space
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
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
                .block(Block::default().borders(Borders::ALL).title("Available Drives"))
                .highlight_style(
                    Style::default()
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            // We can now render the item list
            f.render_stateful_widget(items, chunks[0], &mut app.drives.state);

        })?;

        // This is a simple example on how to handle events
        // 1. This breaks the loop and exits the program on `q` button press.
        // 2. The `up`/`down` keys change the currently selected item in the App's `items` list.
        // 3. `left` unselects the current item.
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Char('x') => {
                    break;
                }
                Key::Esc => {
                    break;
                }
                // Key::Left => {
                //     app.drives.unselect();
                // }
                Key::Down => {
                    app.drives.next();
                }
                Key::Up => {
                    app.drives.previous();
                }
                Key::Char('e') => {
                    println!("Selected")
                }
                _ => {}
            },
            Event::Tick => {
                continue;
            }
            
        }
    }
    Ok(())
}
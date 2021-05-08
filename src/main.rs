#[allow(dead_code)]
mod util;
mod app;

use argh::FromArgs;
use crate::app::{ui, App};
use core::time::Duration;
use crate::util::event::Config;
use crate::util::{
    event::{Event, Events},
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};
use sysinfo::{System, SystemExt, DiskExt};




//Deletion algorithm options
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

///Minuteman CLI
#[derive(Debug, FromArgs)]
struct Cli {
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}



fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();

    let events = Events::with_config(Config {
        // tick_rate: Duration::from_millis(cli.tick_rate),
        ..Config::default()
    });

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut system = System::new_all();
    system.refresh_all();
    
    let mut drives = Vec::new();
    for disk in system.get_disks() {
        drives.push(disk.get_name().to_str().unwrap());
    }
    // Create a new app with some exapmle state
    let mut app = App::new(drives, "Minuteman");
    

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        // This is a simple example on how to handle events
        // 1. This breaks the loop and exits the program on `q` button press.
        // 2. The `up`/`down` keys change the currently selected item in the App's `items` list.
        // 3. `left` unselects the current item.
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char(c) => {
                    app.on_key(c);
                }
                Key::Char('q') => {
                    break;
                }
                Key::Char('x') => {
                    break;
                }
                Key::Esc => {
                    break;
                }
                Key::Down => {
                    app.drives.next();
                }
                Key::Up => {
                    app.drives.previous();
                }
                Key::Char('e') => {
                    println!("Selected drive")
                }
                _ => {}
            },
            Event::Tick => {
                continue;
            }   
        }
        if app.should_quit {
            break;
        }
    }
    Ok(())
}
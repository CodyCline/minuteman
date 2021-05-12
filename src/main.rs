#[allow(dead_code)]
mod util;
mod app;

use crate::util::DiskDisplay;
use argh::FromArgs;
use crate::app::{ui, App};
// use core::time::Duration;
use crate::util::event::Config;
use crate::util::{
    event::{Event, Events},
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};
use sysinfo::{System, SystemExt};




//Deletion algorithm options
const WIPE_METHODS: [&str; 8] = [
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
        let drive = DiskDisplay::new(disk);
        drives.push(drive);
    }
    // Create a new app with some exaple state
    let mut app = App::new(drives, WIPE_METHODS.to_vec(), "Minuteman");
    

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        // This is the main event handler where user input is handled and dispatched according to the app state 
        match events.next()? {
            Event::Input(input) => match input {
                Key::Down => {
                    app.on_down();
                }
                Key::Up => {
                    app.on_up();
                }
                Key::Left => {
                    app.on_left();
                }
                Key::Right => {
                    app.on_right();
                }
                Key::Char('q') => {
                    app.quit();
                }
                Key::Esc => {
                    app.quit();
                }
                Key::Char('e') => {
                    app.on_continue();
                }
                Key::Char('b') => {
                    app.on_back();
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
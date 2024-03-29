#[allow(dead_code)]
mod disk;
mod app;
mod ui;
mod util;
mod clone;

use crate::clone::create_disk_backup;

use crate::disk::{ find_external_disks };
use argh::FromArgs;
use crate::app::{App};
use crate::ui::Ui;

use crate::util::event::Config;
use crate::util::{
    event::{Event, Events},
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};





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

    
    //Instaniate disk get method here returns a vector of drives available to use
    let disks = find_external_disks();
    // for disk in disks.iter() {
    //     println!("DISK !{:?}", disk);
    // }

    let first_disk = &disks.unwrap()[0];
    let size = &first_disk.free_space;
    let name = &first_disk.name;

    let msg = create_disk_backup(name, size);
    
    // let clone = create_disk_backup(std::path::PathBuf::from("/dev/sda1"));
    // println!("{:?}", clone);
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

    


    // Create a new app
    // let mut app = App::new(disks.unwrap(), WIPE_METHODS.to_vec(), "Minuteman");
    
    // loop {
    //     terminal.draw(|f| Ui::draw(f, &mut app))?;

    //     // This is the main event handler where user input is handled and dispatched according to the app state 
    //     match events.next()? {
    //         Event::Input(input) => match input {
    //             Key::Down => {
    //                 app.on_down();
    //             }
    //             Key::Up => {
    //                 app.on_up();
    //             }
    //             Key::Left => {
    //                 app.on_left();
    //             }
    //             Key::Right => {
    //                 app.on_right();
    //             }
    //             Key::Char('q') => {
    //                 app.quit();
    //             }
    //             Key::Esc => {
    //                 app.quit();
    //             }
    //             Key::Char('e') => {
    //                 app.on_continue();
    //             }
    //             Key::Char('c') => {
    //                 app.on_back();
    //             }
    //             _ => {}
    //         },
    //         Event::Tick => {
    //             if app.status.index == 3 {
    //                 //Simulate app progress here
    //                 app.deletion_progress += 0.008;
                    
    //             }
                
    //         }   
    //     }
    //     if app.should_quit {
    //         break;
    //     }
    // }
    Ok(())
}




use crate::util::{StatefulList, TabsState};
use crate::disk::Disk;
//Where state is handeled 

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
pub struct App<'a> {
    pub debug_mode: bool,
    pub title: &'a str,
    pub should_quit: bool,
    pub is_deleting: bool,
    pub deletion_progress: f64,
    pub drives: StatefulList<Disk>,
    pub deletion_methods: StatefulList<&'a str>,
    pub confirmation: TabsState<'a>, //yes no
    pub status: TabsState<'a>, //Which phase of cli state is shown
}


impl<'a> App<'a> {
    pub fn new(drives: Vec<Disk>, deletion_methods: Vec<&'a str>, title: &'a str) -> App<'a> {
        App {
            debug_mode: true, //Prevent anything destructive from happening
            title: title,
            status: TabsState::new(vec!["Select Drive", "Select Deletion Method", "Confirm", "Deletion In progress", "Verify in progress", "Complete", "Error"]),
            should_quit: false,
            is_deleting: false,
            confirmation: TabsState::new(vec!["<CANCEL>", "<DELETE>"]),
            deletion_progress: 0.00,
            deletion_methods: StatefulList::with_items(deletion_methods),
            drives: StatefulList::with_items(drives),
        }
    }

    pub fn on_up(&mut self) {
        match self.status.index {
            0 => { self.drives.previous() }
            1 => { self.deletion_methods.previous() }
            _ => {}
        }
    }


    pub fn on_down(&mut self) {
        match self.status.index {
            0 => { self.drives.next() }
            1 => { self.deletion_methods.next() }
            _ => {}
        }
    }

    pub fn on_left(&mut self) {
        if self.status.index == 2 {
            self.confirmation.previous();
        }
    }

    pub fn on_right(&mut self) {
        if self.status.index == 2 {
            self.confirmation.next();
        }
    }

    

    //The key "e" is what continues the state 
    pub fn on_continue(&mut self) {
        match self.status.index {
            0 => {
                if self.drives.state.selected() != None {
                    self.status.next();
                }
            }
            1 => {
                if self.deletion_methods.state.selected() != None {
                    self.status.next();
                }
            }
            2 => {
                if self.confirmation.titles[self.confirmation.index] == String::from("<DELETE>") {
                    self.is_deleting = true;
                    self.status.next();
                } else {
                    self.status.previous();
                }
            }
            3 => {
                self.is_deleting = true;
            }
            _ => {}
        }

    }

    pub fn on_back(&mut self) {
        match self.status.index {
            0 => {}
            1 => { self.status.previous() }
            2 => {
                self.status.previous();
                self.is_deleting = true;
            }
            3 => {
                if !self.is_deleting {
                    self.status.previous();
                }
            }
            4 => { self.status.previous() }
            _ => {}
        }
    }

    //TODO Quit the app but only allow that if it not in process of wiping drive
    pub fn quit(&mut self) {
        if !self.is_deleting {
            self.should_quit = true;
        }
    }
}

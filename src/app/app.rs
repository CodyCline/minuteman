use crate::util::DiskDisplay;
use crate::util::{StatefulList, TabsState};


enum State {
    MainMenu,
    DriveSelection,
    DeletionMethod,
    DeletionStatus,
    VerifyStatus,
    Success,
    Error,
}

//Where state is handeled 

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub is_deleting: bool,
    pub deletion_progress: f32,
    pub drives: StatefulList<&'a DiskDisplay<'a>>,
    pub deletion_methods: StatefulList<&'a str>,
    pub status: TabsState<'a>, //Which phase of file deletion is shown
}


impl<'a> App<'a> {
    pub fn new(drives: Vec<&'a DiskDisplay<'a>>, deletion_methods: Vec<&'a str>, title: &'a str) -> App<'a> {
        App {
            title: title,
            status: TabsState::new(vec!["Select Drive", "Select Deletion Method", "Confirm", "Deletion In progress", "Verify in progress", "Complete", "Error"]),
            should_quit: false,
            is_deleting: false,
            deletion_progress: 0.0,
            deletion_methods: StatefulList::with_items(deletion_methods),
            drives: StatefulList::with_items(drives),
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            _ => {
            }
        }
    }

    pub fn can_quit(&self) -> bool {
        if self.status.index != 3 || self.status.index != 4 {
            false
        } else {
            true
        }
    }

    //The key "e" is what continues the state 
    pub fn on_continue(&mut self) {
        match self.status.index {
            0 => {
                self.status.next()
            }
            1 => {
                self.status.next();
            }
            2 => {
                self.status.next();
            }
            3 => {
                self.is_deleting = true;
            }
            _ => {}
        }


    }

    pub fn on_back(&mut self) {
        if self.status.index != 3 || self.status.index != 4 {
            self.is_deleting = false;
            self.status.previous();
        }
    }

    //Quit the app but only allow that if it not in process of wiping drive
    pub fn quit(&mut self) {
        // if self.can_quit() {
            self.should_quit = true;
        // }
    }
}

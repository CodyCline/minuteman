


use crate::util::{StatefulList, TabsState};

struct SelectedDrive {
    name: String,
    drive_type: String, 
    mem_avail: f64,
    mem_used: f64,
}


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
    pub drives: StatefulList<&'a str>,
    pub deletion_methods: StatefulList<&'a str>,
    pub state: TabsState<'a>, //Which phase of file deletion is shown
}


impl<'a> App<'a> {
    pub fn new(drives: Vec<&'a str>, deletion_methods: Vec<&'a str>, title: &'a str) -> App<'a> {
        App {
            title: title,
            state: TabsState::new(vec!["Select Drive", "Select Deletion Method", "Confirm", "Deletion In progress", "Verify in progress", "Complete", "Error"]),
            should_quit: false,
            is_deleting: false,
            deletion_progress: 0.0,
            deletion_methods: StatefulList::with_items(deletion_methods),
            drives: StatefulList::with_items(drives),
            // status: State::MainMenu,
            // selected_drive: String::from("None for now"),
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}

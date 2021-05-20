// #[cfg(feature = "termion")]
pub mod event;
use crate::App;
use termion::event::Key;
use std::path::Path;
use sysinfo::{DiskExt, DiskType};
use std::ffi::OsStr;
use tui::widgets::ListState;





pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}



pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }


    pub fn unselect(&mut self) {
        self.state.select(None);
    }

}




//DiskDisplay is metadata containing information about a specific hard drive 
pub struct DiskDisplay<'a> {
    pub name: &'a OsStr,
    pub disk_type: DiskType,
    pub file_system: &'a[u8],
    pub mount_point: &'a Path,
    pub total_space: u64,
    pub available_space: u64,    
}


impl<'a> DiskDisplay<'a> {
    pub fn new(disk: &'a sysinfo::Disk) -> DiskDisplay<'a> {
        DiskDisplay {
            name: disk.get_name(),
            disk_type: disk.get_type(),
            file_system: disk.get_file_system(),
            mount_point: disk.get_mount_point(),
            total_space: disk.get_total_space(),
            available_space: disk.get_available_space(),
        }
    }
}


//Todo, method to handle global events. From here then handle events through blocks.
pub fn handle_app(key: Key, app: &mut App) {
    match key {
        Key::Char('c') => {

        }
        _ => {}
    }
} 
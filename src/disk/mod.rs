//Todo here export the module depending on operating system

use std::path::PathBuf;
use anyhow::Result;

mod linux;
#[cfg(unix)]
use linux as os;

pub use os::find_external_disks;

pub use os::calculate_disk_usage;


pub trait DriveAccessor {
    fn position(&mut self) -> Result<u64>;
    fn seek(&mut self, position: u64) -> Result<u64>;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize>;
    fn write(&mut self, data: &[u8]) -> Result<()>;
    fn flush(&mut self) -> Result<()>;
}





/// `Disk` represents a single Disk/drive which contains metadata about it
/// such as `name`, `total_space` or `drive_type`.
#[derive(Clone, Debug)]
pub struct Disk {
    pub name: PathBuf,
    pub model: String,
    pub serial_number: String,
    pub disk_type: DiskType,
    pub file_system: String,
    pub mount_point: String,
    pub version: String, //Technically a float but reads as string
    pub total_space: u64,
    pub used_space: u64,
    pub free_space: u64,
}


/// Enum which contains supported disk types by application.
///
///
#[derive(Clone, Debug, Copy)]
pub enum DiskType {
    HDD,
    SSD,
    Fixed,
    File,
    Partition,
    Removable,
    CD,
    RAID,
    Unknown(isize),
    Other,
}

impl std::fmt::Display for DiskType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


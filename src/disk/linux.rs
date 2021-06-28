//Get all disk attributes like read_only, serial, model, etc
use std::os::unix::prelude::OsStrExt;
use std::ffi::OsStr;
use nix::sys::statvfs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;
use anyhow::Result;

use crate::disk::{Disk, Partition, DiskType};

///Retrieves all partitions by matching if `name` starts in the /proc/mounts file 
/// then returns them in a vector of `Partition`
pub fn read_partitions(name : &str) -> std::io::Result<Vec<Partition>> {
    let mut partitions: Vec<Partition> = Vec::new();
    let file = File::open("/proc/mounts")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line?;
        let parts: Vec<&str> = l.split_whitespace().collect();
        if parts[0].starts_with(name) { //e.g.g  
            partitions.push(Partition {
                name: parts[0].to_string(),
                mount_point: Path::new(parts[1]).to_path_buf(),
                file_system: parts[2].to_string(),
                read_only: false,
            })
        }
    }
    Ok(partitions)
}

///Computes the disk size, total space and used space
pub fn calculate_disk_usage<P: AsRef<Path>>(path: P) -> Result<(u64, u64, u64)> {
    let statvfs = statvfs::statvfs(path.as_ref())?;
    let total = statvfs.blocks() as u64 * statvfs.fragment_size() as u64;

	let avail_to_root = statvfs.blocks_free() as u64 * statvfs.fragment_size() as u64;
	let used = total - avail_to_root;

	let free = statvfs.blocks_available() as u64 * statvfs.fragment_size() as u64;

	let total_space = used + free;
    Ok((total_space, used, free))
}

fn is_external_device(path: &Path) -> bool {
    for path in path.ancestors() {
        if let Some(name) = path.file_name() {
            if let Some(name) = name.to_str() {
                if name.starts_with("usb") {
                    return true;
                }
            }
        }
    }
    false
}


fn disk_attributes(path: &Path) -> Option<PathBuf> {
    for path in path.ancestors() {
        if path.join("manufacturer").exists()
            && path.join("product").exists()
            && path.join("serial").exists()
        {
            return Some(path.into());
        }
    }
    None
}



///Resolve disk type takes in the block path of a device, then 
/// deduces its possible type by reading file properties such as "rotational" or "removable", then
/// if its rotational
pub fn resolve_disk_type(block_path: PathBuf) -> std::io::Result<DiskType> {
    let read = |name| -> std::io::Result<String> {
        let path = block_path.join(name);
        let contents = std::fs::read_to_string(path)?;
        Ok(contents.trim().into())
    };
    //If the disk is removable return that type 
    let is_removable = read("removable")?;
    if is_removable == String::from("1") {
       Ok(DiskType::Removable) //CD, Flash, Floppy, etc.
    } else {
        
        //Todo check for partition type
        
        //Check if its rotational, if true very likely that its a HDD
        let disk_queue = block_path.join("queue");

        if disk_queue.exists() {
            let is_rotational = read("queue/rotational")?; 
            if is_rotational == String::from("1") {
                return Ok(DiskType::HDD);
            } else {
                return Ok(DiskType::SSD);
            }
        } else {
            //Unknown disk types
            return Ok(DiskType::Unknown);
        }
    }
} 


//https://www.kernel.org/doc/html/latest/_sources/admin-guide/sysfs-rules.rst.txt
//Todo maybe make this multi threaded?
pub fn find_external_disks() -> std::io::Result<Vec<Disk>> {
    let mut disks: Vec<Disk> = Vec::new();
    for entry in std::fs::read_dir("/sys/block")? {
        let entry = entry?;
        let path = entry.path();
        let device_path = path.join("device");
        if !device_path.exists() {
            continue;
        }



        // This will give a very long path such as:
        // /sys/devices/pci0000:00/0000:00:01.2/0000:02:00.0/
        //     0000:03:08.0/0000:08:00.3/usb4/4-3/4-3.2/4-3.2:1.0/
        //     host7/target7:0:0/7:0:0:0
        let device_path = device_path.canonicalize()?;
        
  
        // Skip non-USB devices
        if !is_external_device(&device_path) {
            continue;
        }

        //Once we get our path and are certain its an external drive then get attributes like name, serial, etc.

        let disk_name = Path::new("/dev").join(entry.file_name());

        let _type = resolve_disk_type(path);

        let usage = calculate_disk_usage(&disk_name);
        let (total_space, used, free) = usage.unwrap();
        let partitions = read_partitions(&disk_name.to_str().unwrap());

        if let Some(info_path) = disk_attributes(&device_path) {
            //Read is an closure that displays specific disk attribute(s)
            let read = |name| -> std::io::Result<String> {
                let path = info_path.join(name);
                let contents = std::fs::read_to_string(path)?;
                Ok(contents.trim().into())
            };

            disks.push(Disk {
                name: disk_name,
                model: read("product")?,
                serial_number: read("serial")?,
                disk_type: _type.unwrap_or(DiskType::Unknown), 
                version: read("version")?,
                total_space: total_space,
                used_space: used,
                partitions: partitions.unwrap(),
                free_space: free,
            });
        }
    }
    Ok(disks)
}



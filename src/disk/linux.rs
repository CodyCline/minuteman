//Get all disk attributes like read_only, serial, model, etc
use nix::sys::statvfs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;
use anyhow::Result;

use crate::disk::{Disk, DiskType};

///Retrieves the `mount_point` and `file_system` of a disk if it finds a matching pathname 
/// from given argument in the `"/proc/mounts"` file
pub fn disk_info<P: AsRef<Path>>(path: P) -> Result<Option<(String, String)>> {
    let s = path.as_ref().to_str().unwrap();
    let file = File::open("/proc/mount")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line?;
        let parts: Vec<&str> = l.split_whitespace().collect();
        if parts[0] == s {
            return Ok(Some(
                (parts[1].to_string(), parts[2].to_string())
            ));
        }
    }
    Ok(Some((String::from("Unknown"), String::from("Unknown"))))
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



pub fn resolve_disk_type() {}


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

        println!("{:?}", path);

        // This will give a very long path such as:
        // /sys/devices/pci0000:00/0000:00:01.2/0000:02:00.0/
        //     0000:03:08.0/0000:08:00.3/usb4/4-3/4-3.2/4-3.2:1.0/
        //     host7/target7:0:0/7:0:0:0
        let device_path = device_path.canonicalize()?;
        
        println!("{:?}", device_path);

        // Skip non-USB devices
        if !is_external_device(&device_path) {
            continue;
        }

        //Once we get our path and are certain its an external drive then get attributes like name, serial, etc.

        let disk_name = Path::new("/dev").join(entry.file_name());

        //Get
        let usage = calculate_disk_usage(&disk_name);
        let (total_space, used, free) = usage.unwrap();


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
                disk_type: DiskType::Unknown(-1),
                version: read("version")?,
                file_system: String::from("NOT IMPLEMENTED"), //ext4
                mount_point: String::from("NOT IMPLEMENTED"), //media/disk
                total_space: total_space,
                used_space: used,
                free_space: free,
            });
        }
    }
    Ok(disks)
}



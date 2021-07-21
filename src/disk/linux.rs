//Get all disk attributes like read_only, serial, model, etc
use std::ffi::OsStr;
use libc::statvfs;
use std::fs::File;
use std::os::unix::ffi::OsStrExt;

use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;

use crate::disk::{Disk, DiskType, Partition};

///Retrieves all partitions by matching if `name` starts in the /proc/mounts file
/// then returns them in a vector of `Partition`
pub fn read_partitions(name: &str) -> std::io::Result<Vec<Partition>> {
    let mut partitions: Vec<Partition> = Vec::new();
    let file = File::open("/proc/mounts")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line?;
        let parts: Vec<&str> = l.split_whitespace().collect();
        if parts[0].starts_with(name) {
            let mount = Path::new(parts[1]).to_path_buf();
            let (total, free) = calculate_partition_size(&mount)?;
            partitions.push(Partition {
                name: parts[0].to_string(),
                mount_point: mount,
                file_system: parts[2].to_string(),
                total: total,
                free: free,
                read_only: false,
            })
        }
    }
    Ok(partitions)
}

//Function takes in all block device partitions and adds up their allocated space
//Unsure if this is correct method 
pub fn calculate_disk_usage(partitions: &Vec<Partition>) -> std::io::Result<(u64, u64, u64)> {
    let mut total = 0;
    let mut free_space = 0;
    let mut used = 0;

    match partitions.len() {
        0 => Ok((total, free_space, used)),
        1 => {
            let partition = &partitions[0];
            used = partition.total - partition.free;
            Ok((partition.total, partition.free, used))
        },
        _ => {
            for partition in partitions.iter() {
                total += partition.total;
                free_space += partition.free;
            }
            used = total - free_space;
    
            Ok((total, free_space, used))
        }
    }

}

pub fn calculate_partition_size(mount_point: &PathBuf) -> std::io::Result<(u64, u64)> {
    
    let path_os: &OsStr = mount_point.as_ref();
    let mut cpath = path_os.as_bytes().to_vec();
    cpath.push(0);
    

    let mut total = 0;
    let mut available = 0;
    unsafe {
    let mut stat: statvfs = std::mem::zeroed();
        if statvfs(cpath.as_ptr() as *const _, &mut stat) == 0 {
            total = stat.f_bsize as u64 * stat.f_blocks as u64;
            available = stat.f_bsize * stat.f_bavail;
        }
    }


    Ok((total, available))
    


}

fn is_usb_device(path: &Path) -> bool {
    for path in path.ancestors() {
        if let Some(name) = path.file_name() {
            if let Some(name) = name.to_str() {
                if name.starts_with("usb") {
                    true;
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

///Resolve disk type takes in the block path of a device, and
/// deduces its possible type by reading file properties such as "rotational" or "removable", which
/// returns a `DiskType`
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
        

        let device_path = device_path.canonicalize()?;
        // Skip non-USB devices
        if is_usb_device(&device_path) {
            continue;
        }
        //Once we get our path and are certain its an external drive then get attributes like name, serial, etc.
        


        let name = Path::new("/dev").join(entry.file_name());

        let _type = resolve_disk_type(path);
        let partitions = read_partitions(&name.to_str().unwrap())?;
        let (total_space, free, used) = calculate_disk_usage(&partitions)?;
        if let Some(info_path) = disk_attributes(&device_path) {
            println!("{:?}", device_path);
            //Read is a closure that displays specific disk attribute by reading a file value to string if it exists
            let read = |name| -> std::io::Result<String> {
                let path = info_path.join(name);
                let contents = std::fs::read_to_string(path)?;
                Ok(contents.trim().into())
            };
            


            disks.push(Disk {
                name: name,
                model: read("product")?,
                serial_number: read("serial")?,
                disk_type: _type.unwrap_or(DiskType::Unknown),
                version: read("version")?,
                partitions: partitions,
                total_space: total_space,
                free_space: free,
                used_space: used,
            });
        }
    }
    Ok(disks)
}

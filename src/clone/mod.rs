use std::io::BufReader;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::mem::*;
use std::io::prelude::*;
use std::path::PathBuf;
use std::os::unix::io::AsRawFd;



pub struct WritePath {
    pub source: PathBuf,
    pub destination: PathBuf,
}

impl WritePath {
    fn new(source: PathBuf, destination: PathBuf) -> WritePath {
        WritePath { source, destination }
    }
}




pub fn burn_disk(device: PathBuf) {

}


///Function which takes a source and destination as arg and then attempts to copy the source
/// to an iso file at the destination
pub fn create_disk_backup(device: &PathBuf, size: &u64) -> std::io::Result<String> {
    if device.exists() {


        let mut src = std::fs::File::open(device)?;
        let src_meta = src.metadata().unwrap();
        let src_size = src_meta.len();
        // let src_size = 138000000;
        println!("SIZE {}", src_size);
        let mut destination_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("backup.iso")?;
        
        //How much to chip away at cloning process
        // let mut remaining = src_size;
        // let mut bytes_written = 0;
        
        //TODO redo the disk function to get accurate disk sizes
        
        //Future use of retrying from blocks of data
        src.seek(std::io::SeekFrom::Start(0));


        destination_file.seek(std::io::SeekFrom::Start(0));
        
        //Chunk size is ~ 1mb
        let mut buf: Vec<u8> = vec![0; 1048576];

        let mut src = src.take(*size);
        println!("{}", src_size);
        loop {
            println!("WRITING!");
            let len = match src.read(&mut buf) {
                Ok(0) =>  {
                    println!("NOTHING TO WRITE");
                    break;
                },
                Ok(len) => len,
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => {
                    println!("ERROR {}", e);
                    break;
                }
            };

            destination_file.write_all(&buf[..len]).unwrap();

        }

        Ok(String::from("COOL DONE"))

        // while remaining > 0 {
        //     let read_size = if chunk_size > remaining {
        //         remaining
        //     } else {
        //         chunk_size 
        //     };

        //     buf.resize(read_size as usize, 0);

        //     src.read_exact(&mut buf).unwrap();
        //     destination_file.write_all(&buf).unwrap();

        //     remaining -= read_size;
        //     bytes_written += read_size;
        // }
        // destination_file.sync_data().unwrap();
        // Ok(bytes_written.to_string())
    } else {
        Ok(String::from("NOT FOUND"))
    }
}

///Clone disk takes in a source and destination as arguments and then creates a clone


//Check if there is enough memory in destination
fn memory_is_available() -> bool {
    true
}

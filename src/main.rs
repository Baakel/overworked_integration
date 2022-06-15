use serde::{Deserialize, Serialize};
use std::process::Command;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::OpenOptions;
use chrono::{DateTime, NaiveDateTime, Utc};

fn main() {
    let mut since = String::new();
    let data_path = std::path::Path::new("/home/baakel/.local/share/overworked/data");
    let data_path_prefix = data_path.parent().unwrap();
    if !&data_path_prefix.is_dir() {
        std::fs::create_dir_all(&data_path_prefix).expect("Couldn't create the dirs");
    }
    let today = Utc::today();
    println!("{today}");
    
    let last_used = Utc::now();
    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&data_path) {
        Err(error) => panic!("Couldn't create {}: {}", &data_path.display(), error),
        Ok(file) => file,
    };
    let mut file_buffer = BufReader::new(&file); 
    let len = file_buffer.read_line(&mut since).expect("Unable to read_line to file");
    println!("First line is {len} bytes long");
    println!("Read {}", &since);
    let output = Command::new("timew").args(["export", "from", "may", "for", "1mo", "Echobot workday"]).output().expect("Failed to call this");
    let entries_vector = TimeEntriesVector {
        entries: serde_json::from_str(&String::from_utf8(output.stdout).expect("Error making it to string")).expect("Error deserializing"),
    };
    match file.write_all(last_used.to_string().as_bytes()) {
        Err(error) => panic!("Couldn't write to {}: {}", &data_path.display(), error),
        Ok(_) => println!("Successfully wrote to {}", &data_path.display()),
    };
    println!("{:?}", &entries_vector)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct TimeEntry {
    id: u16,
    start: String, 
    end: String,
    tags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct TimeEntriesVector {
    entries: Vec<TimeEntry>,
}

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::fs;
use chrono::{DateTime, NaiveDateTime, Utc};

fn main() {
    let mut since = String::new();
    let f = File::open("/home/baakel/.local/share/overworked/data").expect("Unable to open file");
    let mut buf_str = BufReader::new(f); 
    buf_str.read_to_string(&mut since).expect("Unable to read string");
    println!("Read {}", &since);
    let output = Command::new("timew").args(["export", "from", "may", "for", "1mo", "Echobot workday"]).output().expect("Failed to call this");
    let entries_vector = TimeEntriesVector {
        entries: serde_json::from_str(&String::from_utf8(output.stdout).expect("Error making it to string")).expect("Error deserializing"),
    };
    let last_used = Utc::now();
    fs::write("/home/baakel/.local/share/overworked/data", last_used.to_string()).expect("Unable to write file");
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

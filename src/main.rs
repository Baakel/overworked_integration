use serde::{Deserialize, Serialize};
use std::process::Command;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::OpenOptions;
use std::ops::{Add, Sub};
use chrono::{Datelike, DateTime, Duration, NaiveDateTime, Utc};

const FORMAT_STRING: &str = "%Y%m%dT%H%M%SZ";

fn main() {
    let mut since = String::new();
    let data_path = std::path::Path::new("/home/baakel/.local/share/overworked/data");
    let data_path_prefix = data_path.parent().unwrap();
    if !&data_path_prefix.is_dir() {
        std::fs::create_dir_all(&data_path_prefix).expect("Couldn't create the dirs");
    }
    let today = Utc::today();
    let mut balance_string = String::new();
    let mut balance = Duration::zero();

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
    let _ = file_buffer.read_line(&mut balance_string).expect("Cannot read the second line");
    if len == 0 {
        match file.write_all(format!("{}\n{}", last_used.sub(Duration::days(1)).to_rfc3339(), balance.num_seconds()).as_bytes()) {
            Err(error) => panic!("Couldn't write to {}: {}", &data_path.display(), error),
            Ok(_) => println!("Successfully updated work hours! New balance should be {}", &balance),
        };
        return ()
    }
    println!("Read {} and the next line is {}", &since, &balance_string);

    balance = balance.add(Duration::seconds(balance_string.trim().parse::<i64>().unwrap()));
    let since_date = DateTime::parse_from_rfc3339(since.trim()).expect("Couldn't parse since");
    let formatted_since = &since_date.format(FORMAT_STRING);
    let output = Command::new("timew").args(
        [
            "export",
            "from",
            &formatted_since.to_string(),
            "Echobot workday"
        ]
    ).output().expect("Failed to call this");
    let entries_vector = TimeEntriesVector {
        entries: serde_json::from_str(&String::from_utf8(output.stdout).expect("Error making it to string")).expect("Error deserializing"),
    };
    for entry in &entries_vector.entries {
        if entry.end.is_none() {
            continue;
        };
        let start_date = NaiveDateTime::parse_from_str(&entry.start, FORMAT_STRING).expect("Failed to parse start_date");
        let end_date = NaiveDateTime::parse_from_str(entry.end.as_ref().unwrap(), FORMAT_STRING).expect("Failed to parse end_date");
        let work_time = end_date.signed_duration_since(start_date);
        balance = balance.sub(work_time);
    }
    if since_date.day() < today.day() && since_date.month() <= today.month() {
        balance = balance.add(Duration::hours(8));
        println!("New day, adding 8hrs of work to balance, current balance is: {balance}");
    }
    // Needed since the file_buffer moves the cursor position on the file, so when writing it always
    // starts at the end of the first read line and thus it doesn't overwrite contents but appends instead
    file_buffer.rewind().expect("Couldn't rewind the buffer");
    match file.write_all(format!("{}\n{}", last_used.to_rfc3339(), balance.num_seconds()).as_bytes()) {
        Err(error) => panic!("Couldn't write to {}: {}", &data_path.display(), error),
        Ok(_) => println!("Successfully updated work hours! New balance should be {}", &balance),
    };
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct TimeEntry {
    id: u16,
    start: String, 
    end: Option<String>,
    tags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct TimeEntriesVector {
    entries: Vec<TimeEntry>,
}

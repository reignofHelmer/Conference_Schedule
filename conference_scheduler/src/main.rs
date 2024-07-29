// src/main.rs
mod models;

use models::{Conference, Session};
use serde_json::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};

const FILE_PATH: &str = "conference_data.json";

fn read_json() -> Result<Conference, Error> {
    let file = File::open(FILE_PATH).unwrap_or_else(|_| File::create(FILE_PATH).unwrap());
    let reader = BufReader::new(file);
    let conference = serde_json::from_reader(reader).unwrap_or_else(|_| Conference { sessions: vec![] });
    Ok(conference)
}

fn write_json(conference: &Conference) -> io::Result<()> {
    let file = OpenOptions::new().write(true).truncate(true).open(FILE_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, conference)?;
    Ok(())
}

fn main() {
    // Read existing conference data
    let mut conference = read_json().expect("Failed to read JSON file");

    // Add a new session
    let new_session = Session {
        title: "Triumph".to_string(),
        speaker: "Helmer".to_string(),
        time: "10:00 AM".to_string(),
        duration: 60,
        description: "Empire Expansion".to_string(),
    };
    conference.sessions.push(new_session);

    // Write updated conference data
    write_json(&conference).expect("Failed to write JSON file");

    // Display updated sessions
    for session in &conference.sessions {
        println!("{:?}", session);
    }
}

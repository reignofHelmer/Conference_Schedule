// src/main.rs
mod models;

use models::Session;
use mysql::*;
use mysql::prelude::*;
use std::env;

fn main() {
    // Get database connection details from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the database
    let pool = Pool::new(database_url).expect("Failed to create database pool.");
    let mut conn = pool.get_conn().expect("Failed to get database connection.");

    // Create the sessions table if it doesn't exist
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS sessions (
            id INT AUTO_INCREMENT PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            speaker VARCHAR(255) NOT NULL,
            time VARCHAR(255) NOT NULL,
            duration INT NOT NULL,
            description TEXT NOT NULL
        )"
    ).expect("Failed to create table.");

    // Add a new session
    let new_session = Session {
        id: None,
        title: "Empire Realty".to_string(),
        speaker: "Helmer".to_string(),
        time: "10:00 AM".to_string(),
        duration: 60,
        description: "Introduction to Human programming".to_string(),
    };
    add_session(&mut conn, new_session);

    // Retrieve and display all sessions
    let sessions = get_sessions(&mut conn);
    for session in sessions {
        println!("{:?}", session);
    }
}

fn add_session(conn: &mut PooledConn, session: Session) {
    conn.exec_drop(
        r"INSERT INTO sessions (title, speaker, time, duration, description)
          VALUES (:title, :speaker, :time, :duration, :description)",
        params! {
            "title" => &session.title,
            "speaker" => &session.speaker,
            "time" => &session.time,
            "duration" => session.duration,
            "description" => &session.description,
        },
    ).expect("Failed to insert session.");
}

fn get_sessions(conn: &mut PooledConn) -> Vec<Session> {
    conn.query_map(
        "SELECT id, title, speaker, time, duration, description FROM sessions",
        |(id, title, speaker, time, duration, description)| {
            Session {
                id: Some(id),
                title,
                speaker,
                time,
                duration,
                description,
            }
        },
    ).expect("Failed to fetch sessions.")
}

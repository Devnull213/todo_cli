extern crate chrono;
use crate::cli;
use crate::task;
// use chrono::prelude::DateTime;
// use chrono::{TimeZone, Utc};
use serde_json;
use std::fs::File;
use std::io::Write;
// use std::time::{SystemTime, UNIX_EPOCH};

//TODO: get rid of every unwrap
// pub fn get_time() -> DateTime<Utc> {
//     let systime = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_secs();
//
//     Utc.timestamp_opt(systime as i64, 0).unwrap()
// }

//TODO: get rid of every unwrap
pub fn new_task(args: cli::Cli) -> task::Task {
    task::Task {
        name: args.add_task.unwrap(),
        description: args.task_description.unwrap(),
        // creation_date: get_time(),
        completed: false,
    }
}

pub fn save_to_file(task: &Vec<task::Task>, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename).unwrap();
    let json_data = serde_json::to_string(task).unwrap(); // serialize
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

extern crate chrono;
use crate::cli;
use crate::task;
use chrono::prelude::DateTime;
use chrono::{TimeZone, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

//TODO: get rid of every unwrap
pub fn get_time() -> DateTime<Utc> {
    let systime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Utc.timestamp_opt(systime as i64, 0).unwrap()
}

//TODO: get rid of every unwrap
pub fn new_task(args: cli::Cli) -> task::Task {
    task::Task {
        name: args.add_task.unwrap(),
        description: args.task_description.unwrap(),
        creation_date: get_time(),
    }
}

extern crate chrono;
use chrono::prelude::DateTime;
use chrono::{Utc, TimeZone};
use clap::Parser;
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    #[arg(short='a', long="add-task")]
    add_task: Option<String>,

    #[arg(short='d', long="task-desc")]
    task_description: Option<String>,

    #[arg(short='l', long="list")]
    list: Option<String>,
}


#[derive(Debug)]
struct Task {
    name: String,
    description: String,
    creation_date: DateTime<Utc>, 
}

fn get_time() -> DateTime<Utc> {
    let systime = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    Utc.timestamp_opt(systime as i64, 0).unwrap()
}

//TODO: get rid of every unwrap
fn new_task(args: Cli) -> Task{
    Task {
        name: args.add_task.unwrap(),
        description: args.task_description.unwrap(),
        creation_date: get_time()
    }
}

fn main() {
    let cli = Cli::parse();

    let task = new_task(cli);
    println!("Taskname: {}\nDescription: {}\nCreation time: {:?}", task.name, task.description, task.creation_date);
}

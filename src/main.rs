extern crate chrono;
use clap::Parser;
mod cli;
mod task;
mod utils;

fn main() {
    let console = cli::Cli::parse();
    let task = utils::new_task(console);

    println!(
        "Taskname: {}\nDescription: {}\nCreation time: {:?}",
        task.name, task.description, task.creation_date
    );
}

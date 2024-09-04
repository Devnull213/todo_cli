extern crate chrono;
use clap::Parser;
mod cli;
mod task;
mod utils;

fn main() -> std::io::Result<()> {
    let console = cli::Cli::parse();
    let mut task_list: Vec<task::Task> = Vec::new();
    let task = utils::new_task(console);

    task_list.push(task);
    utils::save_to_file(&task_list, "test.json");
    // println!("{:?}", task_list);
    Ok(())
}

extern crate chrono;
use clap::Parser;
mod cli;
mod task;
mod utils;


// //TODO: get rid of every unwrap
// fn get_time() -> DateTime<Utc> {
//     let systime = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_secs();
//
//     Utc.timestamp_opt(systime as i64, 0).unwrap()
// }
//
// //TODO: get rid of every unwrap
// fn new_task(args: cli::Cli) -> task::Task{
//     task::Task {
//         name: args.add_task.unwrap(),
//         description: args.task_description.unwrap(),
//         creation_date: get_time()
//     }
// }

fn main() {
    let console = cli::Cli::parse();
    let task = utils::new_task(console);

    println!("Taskname: {}\nDescription: {}\nCreation time: {:?}", task.name, task.description, task.creation_date);
}

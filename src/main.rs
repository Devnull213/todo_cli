use clap::Parser;


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
}


//TODO: get rid of every unwrap
fn new_task(args: Cli) -> Task{
    Task {
        name: args.add_task.unwrap(),
        description: args.task_description.unwrap(),
    } 
}

fn main() {
    let cli = Cli::parse();

    let task = new_task(cli);
    println!("Taskname: {}, and descr: {}", task.name, task.description);
}

use clap::Parser;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {

    #[arg(short='a', long="add-task")]
    pub add_task: Option<String>,

    #[arg(short='d', long="task-desc")]
    pub task_description: Option<String>,

    #[arg(short='l', long="list")]
    pub list: Option<String>,
}


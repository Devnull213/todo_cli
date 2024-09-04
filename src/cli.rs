use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
// TODO: Let's make some mods. Need a --search, --list, --new(or add)flag, and description will be
// interactive. Just take the name of the task as a parameter from cli, and can be modified
pub struct Cli {
    #[arg(short = 'a', long = "add-task")]
    pub add_task: Option<String>,

    #[arg(short = 'd', long = "task-desc")]
    pub task_description: Option<String>,

    #[arg(short = 'l', long = "list")]
    pub list: Option<String>,
}

use clap::{Parser, Subcommand};
use cli_todo::*;
use std::path::Path;

#[derive(Parser)]
#[command(name = "ToDo")]
#[command(about = "A simple CLI to do list", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add { task: String },
    Remove { id: u32 },
    Finish { id: u32 },
    List,
}
fn main() {
    let args = Cli::parse();

    let mut tasks: Vec<Task> = load_tasks(Path::new(TASKS_FILE_PATH)).unwrap();

    match &args.command {
        Command::Finish { id } => {
            let _ = finish_task(id, &mut tasks);
        }
        Command::Add { task } => {
            let _ = add_task(task, &mut tasks);
        }
        Command::Remove { id } => {
            let _ = remove_task(id, &mut tasks);
        }
        Command::List => {
            let _ = list_tasks(&tasks);
        }
    }
}

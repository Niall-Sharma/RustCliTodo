use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::{Error, stdin};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}

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
    Done { id: u32 },
    List,
}
fn save_task(tasks: &Vec<Task>) -> Result<(), Error> {
    let json = serde_json::to_string_pretty(tasks).unwrap();

    fs::write("tasks.json", json)?;
    Ok(())
}

fn load_tasks() -> Result<Vec<Task>, Error> {
    match fs::read_to_string("tasks.json") {
        Ok(data) => match serde_json::from_str(&data) {
            Ok(tasks) => Ok(tasks),
            Err(_e) => {
                eprintln!("Error parsing saved tasks");
                Ok(vec![])
            }
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            fs::write("tasks.json", "[]")?;
            Ok(vec![])
        }
        Err(e) => {
            eprintln!("Error reading tasks");
            Err(e)
        }
    }
}

fn remove_task(id: &u32, tasks: &mut Vec<Task>) -> Result<(), Error> {
    let before_length = tasks.len();
    tasks.retain(|t| &t.id != id);
    let after_length = tasks.len();

    if before_length == after_length {
        println!("Could not find task");
    } else {
        let _ = save_task(&tasks);
        println!("Removed Task {}", id)
    }

    Ok(())
}
fn finish_task(id: &u32, tasks: &mut Vec<Task>) -> Result<(), Error> {
    if let Some(task) = tasks.iter_mut().find(|t| &t.id == id) {
        task.done = true;

        let _ = save_task(&tasks);
        println!("Task Completed!");
        println!("Remove task? (N/y)");
        let mut confirmation = String::new();

        stdin().read_line(&mut confirmation).expect("Y or N");
        match confirmation.to_lowercase().trim() {
            "y" => remove_task(id, tasks)?,
            _ => println!("Task not removed"),
        }
        Ok(())
    } else {
        println!("Task not found");
        Ok(())
    }
}

fn main() {
    let args = Cli::parse();

    let mut tasks: Vec<Task> = load_tasks().unwrap();

    match &args.command {
        Command::Done { id } => {
            let _ = finish_task(id, &mut tasks);
        }
        Command::Add { task } => {
            let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

            let task_obj = Task {
                id: next_id,
                description: task.to_owned(),
                done: false,
            };

            tasks.push(task_obj);

            let _ = save_task(&tasks);

            println!("Saved Task!");
        }
        Command::Remove { id } => {
            let _ = remove_task(id, &mut tasks);
        }
        Command::List => {
            println!("TASK LIST");
            for task in &tasks {
                println!("===========================");
                println!(
                    "Name: {}\nID: {}\nDone: {}",
                    task.description, task.id, task.done
                );
            }
        }
    }
}

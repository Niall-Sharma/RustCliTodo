use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::{Error, stdin};
use std::path::Path;

#[cfg(test)]
mod tests;

pub const TASKS_FILE_PATH: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: u32,
    description: String,
    done: bool,
}

pub fn save_task(tasks: &Vec<Task>, path: &Path) -> Result<(), Error> {
    let json = serde_json::to_string_pretty(tasks).unwrap();

    fs::write(path, json)?;
    Ok(())
}

pub fn load_tasks(path: &Path) -> Result<Vec<Task>, Error> {
    match fs::read_to_string("tasks.json") {
        Ok(data) => match serde_json::from_str(&data) {
            Ok(tasks) => Ok(tasks),
            Err(_e) => {
                eprintln!("Error parsing saved tasks");
                Ok(vec![])
            }
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            fs::write(path, "[]")?;
            Ok(vec![])
        }
        Err(e) => {
            eprintln!("Error reading tasks");
            Err(e)
        }
    }
}

pub fn remove_task(id: &u32, tasks: &mut Vec<Task>) -> Result<(), Error> {
    let before_length = tasks.len();
    tasks.retain(|t| &t.id != id);
    let after_length = tasks.len();

    if before_length == after_length {
        println!("Could not find task");
    } else {
        let _ = save_task(&tasks, Path::new(TASKS_FILE_PATH));
        println!("Removed Task {}", id)
    }

    Ok(())
}
pub fn finish_task(id: &u32, tasks: &mut Vec<Task>) -> Result<(), Error> {
    if let Some(task) = tasks.iter_mut().find(|t| &t.id == id) {
        task.done = true;

        let _ = save_task(&tasks, Path::new(TASKS_FILE_PATH));
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

pub fn add_task(description: &String, tasks: &mut Vec<Task>) -> Result<(), Error> {
    let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    let task_obj = Task {
        id: next_id,
        description: description.to_owned(),
        done: false,
    };

    tasks.push(task_obj);

    let _ = save_task(&tasks, Path::new(TASKS_FILE_PATH));

    println!("Saved Task!");
    Ok(())
}

pub fn list_tasks(tasks: &Vec<Task>) -> Result<(), Error> {
    println!("TASK LIST");
    for task in tasks {
        let status = if task.done {
            "✔".green()
        } else {
            "✗".red()
        };
        println!("===========================");
        println!("{}. {}: {}", task.id, task.description, status);
    }
    Ok(())
}

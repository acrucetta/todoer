// src/main.rs
mod task;
mod task_manager;

use std::{env, path::Path, fs::File};
use task_manager::TaskManager;
use clap::{arg, Command, command};

fn main() {
    let mut task_manager: TaskManager = TaskManager::new();

    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(arg!([TASK]))
                .arg_required_else_help(true) 
        )
        .subcommand(
            Command::new("do")
                .about("Complete a task by its ID")
                .arg(arg!([ID]))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a task by its ID")
                .arg(arg!([ID]))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("ls")
                .about("List all tasks")
        )
        .get_matches();

    let file_path = format!("{}/tasks.csv", get_output_dir());

    // Load the file into the task manager if it exists
    if Path::new(&file_path).exists() {
        task_manager.load(&file_path);
    } else {
        // Create the file if it doesn't exist
        File::create(&file_path).expect("Unable to create file");
    } 

    match matches.subcommand() {
        ("add", Some(sub_m)) => {
            let task = sub_m.value_of(TASK).unwrap();
            task_manager.add_task(task.to_string());
        }
        ("do", Some(sub_m)) => {
            let id = sub_m.value_of(ID).unwrap().parse::<u32>().unwrap();
            task_manager.mark_done(id);
        }
        ("rm", Some(sub_m)) => {
            let id = sub_m.value_of(ID).unwrap().parse::<u32>().unwrap();
            task_manager.remove_task(id);
        }
        ("ls", Some(_)) => {
            task_manager.list_tasks();
        }
        _ => unreachable!(),
    }

    match save_tasks(&task_manager, &file_path) {
        Ok(_) => println!("Tasks saved successfully"),
        Err(e) => println!("Error saving tasks: {}", e),
    }

}

fn save_tasks(task_manager: &TaskManager, file_path: _) -> Result<_, _> {
    todo!()
}


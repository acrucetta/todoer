// src/main.rs
mod file_handler;
mod task;
mod task_manager;

use clap::{arg, command, Command};
use file_handler::{get_output_dir, save_tasks};
use task::Status;
use std::{env, fs::File, path::Path};
use task_manager::TaskManager;

fn main() {

    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a new task"),
        )
        .subcommand(
            Command::new("do")
                .about("Complete a task by its ID")
                .arg(arg!([ID]))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a task by its ID")
                .arg(arg!([ID]))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("ls").about("List all tasks"))
        .get_matches();

    let file_path = format!("{}/tasks.csv", get_output_dir());

    let mut task_manager = match TaskManager::from_file(&file_path) {
        Ok(tm) => tm,
        Err(e) => {
            println!("Error loading tasks: {}", e);
            TaskManager::new()
        }
    };
    
    // TODO: Make the "add" command be an interactive CLI for adding tasks
    // TODO: Make the "do" command be an interactive CLI for marking tasks as done

    match matches.subcommand() {
        Some(("add", _sub_m)) => {
            task_manager.add_task();
        }
        Some(("do", sub_m)) => {
            let id = sub_m.get_one::<u32>("ID").unwrap();
            task_manager.adjust_status(*id, Status::Done);
        }
        Some(("rm", sub_m)) => {
            let id = sub_m.get_one::<u32>("ID").unwrap();
            task_manager.remove_task(*id);
        }
        Some(("ls", _sub_m)) => {
            task_manager.list_tasks();
        }
        _ => unreachable!(),
    }

    match save_tasks(&file_path, task_manager) {
        Ok(_) => println!("Tasks saved successfully"),
        Err(e) => println!("Error saving tasks: {}", e),
    }
}

// src/main.rs
mod file_handler;
mod task;
mod task_manager;

use clap::{arg, command, Command};
use file_handler::{get_output_dir, save_tasks};
use std::env;
use task::Status;
use task_manager::TaskManager;

fn main() {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(arg!([TASK]))
                .arg_required_else_help(true),
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
        .subcommand(
            Command::new("ls")
                .about("List all tasks")
                .arg(arg!(--tag[TAG]))
                .arg(arg!(--status[STATUS])),
        )
        .get_matches();

    let file_path = format!("{}/tasks.csv", get_output_dir());

    let mut task_manager = match TaskManager::from_file(&file_path) {
        Ok(tm) => tm,
        Err(e) => {
            println!("Error loading tasks: {}", e);
            TaskManager::new()
        }
    };

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let task = sub_m.get_one::<String>("TASK").unwrap();
            task_manager.add_task(&task);
        }
        Some(("do", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            task_manager.adjust_status(id.parse::<u32>().unwrap(), Status::Done);
        }
        Some(("rm", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            task_manager.remove_task(id.parse::<u32>().unwrap());
        }
        Some(("ls", sub_m)) => match sub_m.get_one::<String>("TAG") {
            Some(tag) => task_manager.list_by_tag(&tag),
            None => task_manager.list_tasks(),
        },
        _ => unreachable!(),
    }

    match save_tasks(&file_path, task_manager) {
        Ok(_) => println!(""),
        Err(e) => eprintln!("Error saving tasks: {}", e),
    }
}

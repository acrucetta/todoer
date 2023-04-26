// src/main.rs
mod file_handler;
mod helpers;
mod notion_api;
mod notion_handler;
mod notion_props;
mod task;
mod task_manager;

use clap::{arg, command, Command};
use file_handler::{get_output_dir, save_tasks};
use notion_handler::NotionManager;

use std::env;
use task::Status;
use task_manager::{TaskManager, ViewFilters};

#[tokio::main]
async fn main() {
    let matches = command!()
        .subcommand_required(true)
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
            Command::new("hold")
                .about("Hold a task by its ID")
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
            Command::new("reset")
                .about("Reset a task by its ID")
                .arg(arg!([ID]))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("ls")
                .about("List all tasks")
                .arg(arg!(--tag[TAG]))
                .arg(arg!(--status[STATUS]))
                .arg(arg!(--due[DUE]))
                .arg(arg!(--priority[PRIORITY]))
                .arg(arg!(--view[VIEW])),
        )
        .subcommand(
            Command::new("nadd")
                .about("Add a task to the set notion db")
                .arg(arg!([TASK]))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("nls")
                .about("List all task in the set notion db")
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("nrm_keys")
                .about("Remove notion keys from config")
                .arg_required_else_help(false),
        )
        .get_matches();

    // We're loading the .env as a binary, so we need to get the path of the binary
    let file_path = format!("{}/tasks.csv", get_output_dir());

    let mut task_manager = match TaskManager::from_file(&file_path) {
        Ok(tm) => tm,
        Err(e) => {
            println!("Error loading tasks: {}", e);
            TaskManager::new()
        }
    };

    let mut notion_manager = NotionManager::new();
    let subcommand = matches.subcommand();
    let (subcommand, sub_m) = if let Some(subc) = subcommand {
        subc
    } else {
        eprintln!("Missing subcommand.");
        return;
    };

    match subcommand {
        "add" => {
            let task = sub_m.get_one::<String>("TASK").unwrap();
            task_manager.add_task(task);
        }
        "do" => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            task_manager.adjust_status(id.parse::<u32>().unwrap(), Status::Done);
        }
        "hold" => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            task_manager.adjust_status(id.parse::<u32>().unwrap(), Status::Hold);
        }
        "reset" => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            // Get the description of the task
            let description = task_manager
                .get_task(id.parse::<u32>().unwrap())
                .description
                .clone();
            // Remove the task
            task_manager.remove_task(id.parse::<u32>().unwrap());
            // Re-add the task with the same description
            task_manager.add_task(&description);
        }
        "rm" => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            task_manager.remove_task(id.parse::<u32>().unwrap());
        }
        "ls" => {
            let tag = sub_m.get_one::<String>("tag");
            let status = sub_m.get_one::<String>("status");
            let due = sub_m.get_one::<String>("due");
            let priority = sub_m.get_one::<String>("priority");

            let mut view_args = ViewFilters::new();

            if let Some(tag) = tag {
                let tags = tag.split(',').map(|t| t.trim().to_owned()).collect();
                view_args.tag = Some(tags);
            }
            if let Some(status) = status {
                let statuses = status.split(',').map(|t| t.trim().to_owned()).collect();
                view_args.status = Some(statuses);
            }
            if let Some(due) = due {
                view_args.due = Some(due.to_owned());
            }
            if let Some(priority) = priority {
                let priorities: Vec<String> =
                    priority.split(',').map(|t| t.trim().to_owned()).collect();
                view_args.priority = Some(priorities);
            }
            if let Some(view) = sub_m.get_one::<String>("view") {
                match view.as_str() {
                    "tags" => view_args.view = Some(String::from("tags")),
                    "due" => view_args.view = Some(String::from("due")),
                    _ => eprintln!("Invalid view type"),
                };
            }
            task_manager.list_tasks(view_args);
        }
        "nadd" => {
            let task = match sub_m.try_get_one::<String>("TASK") {
                Ok(Some(task)) => task,
                _ => {
                    helpers::handle_error("Missing or invalid task argument");
                    return;
                }
            };
            notion_manager.add_task(task).await;
        }
        "nls" => {
            notion_manager.list_all_tasks().await;
        }
        "nrm_keys" => {
            notion_manager.remove_notion_keys();
        }
        otherwise => {
            eprintln!("Unrecognized subcommand \"{otherwise}\".")
        }
    }

    match save_tasks(&file_path, task_manager) {
        Ok(_) => println!(),
        Err(e) => eprintln!("Error saving tasks: {}", e),
    }
}

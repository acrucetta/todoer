// src/main.rs
mod task;
mod task_manager;

use std::env;
use task_manager::TaskManager;
use clap::{Arg, App};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut task_manager: TaskManager = TaskManager::new();
    
    let matches = App::new("Task Manager")
        .version("1.0")
        .author("John Doe")
        .about("A simple task manager")
        .arg(Arg::with_name("add")
            .short("a")
            .long("add")
            .value_name("TASK")
            .help("Add a new task")
            .takes_value(true))
        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .help("List all tasks"))
        .arg(Arg::with_name("remove")
            .short("r")
            .long("remove")
            .value_name("ID")
            .help("Remove a task")
            .takes_value(true))
        .arg(Arg::with_name("done")
            .short("d")
            .long("done")
            .value_name("ID")
            .help("Mark a task as done")
            .takes_value(true))
        .arg(Arg::with_name("tag")
            .short("t")
            .long("tag")
            .value_name("TAG")
            .help("List tasks by tag")
            .takes_value(true))
        .arg(Arg::with_name("status")
            .short("s")
            .long("status")
            .value_name("STATUS")
            .help("List tasks by status")
            .takes_value(true))
        .get_matches();

}


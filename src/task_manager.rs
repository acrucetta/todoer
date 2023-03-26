use std::{
    io::{self, Write},
    time::SystemTime,
};

use crate::task::{Due, Priority, Status, Task};

pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    pub fn get_max_id(&self) -> u32 {
        let mut max_id = 0;
        for task in &self.tasks {
            if task.id > max_id {
                max_id = task.id;
            }
        }
        max_id
    }

    pub fn add_task(&mut self, description: &String) {
        // Ask the user for input in the command line for
        // each field of the task; make some fields optional
        // and use default values for them
        let mut task = Task::new();
        task.description = description.to_string();
        task.id = self.get_max_id() + 1;

        println!("Tag: ");
        io::stdout().flush().unwrap();
        let mut tags = String::new();
        io::stdin()
            .read_line(&mut tags)
            .expect("Failed to read line");
        task.tags = tags.trim().split(',').map(|s| s.to_string()).collect();

        // Provide a set of options when the task is due based on the enum
        // the user will select with a number
        println!("Due:");
        println!("1. Today");
        println!("2. Tomorrow");
        println!("3. This Week");
        println!("4. Sometime");
        let mut due = String::new();
        std::io::stdin()
            .read_line(&mut due)
            .expect("Failed to read line");
        match due.trim().parse::<u32>() {
            Ok(1) => task.due = Due::Today,
            Ok(2) => task.due = Due::Tomorrow,
            Ok(3) => task.due = Due::ThisWeek,
            Ok(4) => task.due = Due::Sometime,
            _ => task.due = Due::Sometime,
        };

        // Provide a set of options for the priority of the task
        println!("Priority:");
        println!("1. Low");
        println!("2. Medium");
        println!("3. High");
        let mut priority = String::new();
        std::io::stdin()
            .read_line(&mut priority)
            .expect("Failed to read line");
        match priority.trim().parse::<u32>() {
            Ok(1) => task.priority = Priority::Low,
            Ok(2) => task.priority = Priority::Medium,
            Ok(3) => task.priority = Priority::High,
            _ => task.priority = Priority::Low,
        };

        // Set the task's status to "Todo"
        task.status = Status::Todo;

        // Push the task to the list of tasks
        self.tasks.push(task);
    }

    pub fn list_tasks(&self) {
        // Print the list of tasks to the console
        print!("ID, Description, Due \n");
        for task in &self.tasks {
            println!("{}, {}, {}", task.id, task.description, task.due);
        }
    }

    pub fn remove_task(&mut self, id: u32) {
        // Remove the task with the given id
        let _ = &self.tasks.retain(|task| task.id != id);
    }

    pub fn adjust_status(&mut self, id: u32, status: Status) {
        // Adjust the status of the task with the given id
        let task = self.tasks.iter_mut().find(|task| task.id == id).unwrap();

        // Set the task's status to the given status
        task.status = status;
    }

    pub fn list_by_tag(&self, tag: &str) {
        // Print the list of tasks to the console
        for task in &self.tasks {
            if task.tags.contains(&tag.to_string()) {
                println!("{}, {}, {}", task.id, task.description, task.due);
            }
        }
    }

    pub fn list_by_status(&self, status: Status) {
        // Print the list of tasks to the console
        for task in &self.tasks {
            if task.status == status {
                println!("{}, {}, {}", task.id, task.description, task.due);
            }
        }
    }

    pub(crate) fn from_file(file_path: &str) -> Result<TaskManager, csv::Error> {
        // Read a CSV file and return a TaskManager
        let mut tasks: Vec<Task> = Vec::new();

        // Read the CSV file
        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(file_path);

        let mut rdr = match rdr {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {}", e);
                return Ok(TaskManager::new());
            }
        };

        for result in rdr.records() {
            let record = result?;
            if record.get(0).unwrap() == "id" {
                continue;
            }
            if record.len() == 0 {
                continue;
            } 
            let task = Task::from_record(record);
            tasks.push(task);
        }
        Ok(TaskManager { tasks })
    }
}

use std::{
    collections::HashMap,
    io::{self, Write},
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

    fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }

    pub fn add_task(&mut self, description: &String) {
        let mut task = Task::new();
        task.description = description.to_string();
        task.id = self.get_max_id() + 1;
        task.tags = TaskManager::get_input("Tags:")
            .split(',')
            .map(|s| s.to_string())
            .collect();
        task.due = match TaskManager::get_input("Due:").as_str() {
            "1" => Due::Today,
            "2" => Due::Tomorrow,
            "3" => Due::ThisWeek,
            _ => Due::Sometime,
        };
        task.priority = match TaskManager::get_input("Priority:").as_str() {
            "1" => Priority::Low,
            "2" => Priority::Medium,
            _ => Priority::High,
        };
        task.status = Status::Todo;
        self.tasks.push(task);
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

    pub fn list_tasks(&self, filters: HashMap<&str, &str>) {
        print!("Description, Status, Due, Tags\n");
        for task in &self.tasks {
            let mut found = true;
            // If no filters are given, print all tasks
            if filters == HashMap::new() {
                println!(
                    "{}, {}, {}, {}, {}",
                    task.id,
                    task.description,
                    task.status,
                    task.due,
                    task.tags.join(", ")
                );
                continue;
            }
            for (key, value) in &filters {
                match key {
                    &"description" => {
                        if !task.description.contains(value) {
                            found = false;
                        }
                    }
                    &"status" => {
                        if task.status.to_string() != *value {
                            found = false;
                        }
                    }
                    &"due" => {
                        if task.due.to_string() != *value {
                            found = false;
                        }
                    }
                    &"tags" => {
                        if !task.tags.contains(&value.to_string()) {
                            found = false;
                        }
                    }
                    _ => {}
                }
            }
            if found {
                println!(
                    "{}, {}, {}, {}, {}",
                    task.id,
                    task.description,
                    task.status,
                    task.due,
                    task.tags.join(", ")
                );
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

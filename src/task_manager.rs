// src/task_manager.rs
use crate::task::{Status, Task};

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn list_tasks(&self) {
        // Print the list of tasks to the console
        for task in &self.tasks {
            println!("{}, {}, {}", task.id, task.description, task.due);
        }
    }

    pub fn remove_task(&mut self, id: u32) {
        // Remove the task with the given id
        let _ = &self.tasks.retain(|task| task.id != id);
    }

    pub fn mark_done(&mut self, id: u32) {
        // Mark the id as done using functional programming
        let _ = &self.tasks.iter_mut().find(|task| task.id == id).map(|task| task.status = Status::Done);
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
}


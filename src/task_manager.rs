use crate::task::{Task, Status};

pub struct TaskManager {
    pub tasks: Vec<Task>,
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

    pub(crate) fn from_file(file_path: &str) -> Result<TaskManager,csv::Error> {
        // Read a CSV file and return a TaskManager
        let mut tasks: Vec<Task>= Vec::new();

        let mut rdr = csv::Reader::from_path(file_path)?;

        for result in rdr.records() {
            let record = result?;
            let id: u32 = record[0].parse().unwrap();
            let description: String = record[1].to_string();
            let tags: Vec<String> = record[2].split(',').map(|s| s.to_string()).collect();
            let due: String = record[3].to_string();
            let timestamp: String = record[4].to_string();
            let priority: String = record[5].to_string();
            let status: Status = match record[6].as_ref() {
                "Todo" => Status::Todo,
                "Done" => Status::Done,
                "Hold" => Status::Hold,
                "Blocked" => Status::Blocked,
                _ => Status::Todo,
            };

            let task = Task {
                id,
                description,
                tags,
                due,
                timestamp,
                priority,
                status,
            };

            tasks.push(task);
        };

        Ok(TaskManager { tasks })
    }
}


use std::{
    collections::HashMap,
    io::{self},
};

use chrono::{Datelike, Local};

use crate::task::{Priority, Status, Task};

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

    fn get_input(prompt: &str, options: Option<&str>) -> String {
        println!("{}", prompt);
        if let Some(options) = options {
            println!("{}", options);
        }
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
        task.tags = TaskManager::get_input("\nTags", None)
            .split(',')
            .map(|s| s.to_string())
            .collect();
        task.due = match TaskManager::get_input(
            "\nDue",
            Some(
                "1. Today, 2. Tomorrow, 3. This Week, 4.Sometime\nOtherwise, press enter for a custom date YYYY-MM-DD",
            ),
        )
        .as_str()
        {
            // We will use the chrono crate to parse dates and assign them
            // to the task's due field; if it's not a 1,2,3,4, then we will
            // assume it is a date in the format of YYYY-MM-DD
            "1" => Local::now().naive_utc().date(),
            "2" => Local::now().naive_utc().date() + chrono::Duration::days(1),
            "3" => Local::now().naive_utc().date() + chrono::Duration::weeks(1),
            _ => match chrono::NaiveDate::parse_from_str(
                &TaskManager::get_input("\nDue Date (YYYY-MM-DD)", None),
                "%Y-%m-%d",
            ) {
                Ok(date) => date,
                Err(_) => {
                    chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
                }
            },
        };
        task.priority = match TaskManager::get_input(
            "\nPriority:",
            Some("1. Low, 2. Medium, 3. High"),
        )
        .as_str()
        {
            "1" => Priority::Low,
            "2" => Priority::Medium,
            "3" => Priority::High,
            _ => Priority::Low,
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
        let mut found_tasks: Vec<&Task> = vec![];
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
                        if task.status.to_string().to_ascii_lowercase() != *value {
                            found = false;
                        }
                    }
                    &"due" => {
                        let due_yymmdd = match value {
                            &"today" => Local::now().naive_utc().date(),
                            &"tomorrow" => {
                                Local::now().naive_utc().date() + chrono::Duration::days(1)
                            }
                            &"thisweek" => {
                                // Due this week is defined as until the end of the weekday (Friday)
                                // We want to take whatever weekday next Friday is
                                Local::now().naive_utc().date()
                                    + chrono::Duration::days(
                                        5 - Local::now().weekday().num_days_from_monday() as i64,
                                    )
                            }
                            &"sometime" => chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                            _ => match chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d") {
                                Ok(date) => date,
                                Err(_) => chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                            },
                        };
                        if value == &"overdue" {
                            if task.due > Local::now().naive_utc().date() {
                                found = false;
                            }
                        } else if value == &"thisweek" {
                            // Due this week is defined as until the end of the weekday (Friday)
                            if task.due > due_yymmdd
                                || task.due
                                    < Local::now().naive_utc().date() - chrono::Duration::days(1)
                            {
                                found = false;
                            }
                        } else if task.due.to_string() != due_yymmdd.to_string() {
                            found = false;
                        }
                    }
                    &"tag" => {
                        if !task.tags.contains(&value.to_string()) {
                            found = false;
                        }
                    }
                    _ => {}
                }
            }
            if found {
                found_tasks.push(task.clone());
            }
        }
        TaskManager::print_tasks(found_tasks);
    }

    fn print_tasks(tasks: Vec<&Task>) {
        // We want to print tasks to the command line in the following format:
        //
        // Due: YYYY-MM-DD (Day of Week)
        // ---------------
        // # Tag
        // [id - Priority] Description
        // [id - Priority] Description
        
        // Due: YYYY-MM-DD
        // ---------------
        // etc.

        // First, we need to sort the tasks by due date, tags, and priority
        let mut sorted_tasks: Vec<&Task> = tasks.clone();
        sorted_tasks.sort_by(|a, b| a.due.cmp(&b.due));
        sorted_tasks.sort_by(|a, b| a.tags.cmp(&b.tags));
        sorted_tasks.sort_by(|a, b| a.priority.cmp(&b.priority));

        // Now we can print the tasks
        let mut current_due = "".to_string();
        let mut current_tag = "".to_string();

        for task in sorted_tasks {
            if task.due.to_string() != current_due {
                println!("\nDue: {} ({})", task.due, task.due.format("%A"));
                println!("--------------------------------");
                current_due = task.due.to_string();
            }
            if task.tags.len() > 0 && task.tags[0] != current_tag {
                println!("# {}", task.tags[0]);
                current_tag = task.tags[0].clone();
            }
            println!(
                "[#{} - {}] {}",
                task.id,
                task.priority.to_string(),
                task.description
            );
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

    pub(crate) fn get_task(&self, unwrap: u32) -> &Task {
        self.tasks.iter().find(|task| task.id == unwrap).unwrap()
    }
}

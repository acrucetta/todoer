use std::{env, path::Path};

use crate::task_manager::TaskManager;

pub fn get_output_dir() -> String {
    const DOTENV_PATH: &str = "/Users/andrescrucettanieto/Library/CloudStorage/OneDrive-WaltzHealth/Documents/Code/todoer/.env";
    dotenv::from_path(DOTENV_PATH).ok();
    match env::var("DOER_OUTPUT_DIR") {
        Ok(val) => return val,
        Err(_) => println!("DOER_OUTPUT_DIR not set, using current directory"),
    }
    let curr_dir = ".";
    curr_dir.to_string()
}

pub fn save_tasks(file_path: &str, task_manager: TaskManager) -> Result<(), csv::Error> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;

    writer.write_record([
        "id",
        "description",
        "tags",
        "due",
        "timestamp",
        "priority",
        "status",
    ])?;

    for task in task_manager.tasks {
        writer.write_record([
            &task.id.to_string(),
            &task.description,
            &task.tags.join(","),
            &task.due.to_string(),
            &task
                .timestamp
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            &task.priority.to_string(),
            &task.status.to_string(),
        ])?;
    }
    writer.flush()?;

    Ok(())
}

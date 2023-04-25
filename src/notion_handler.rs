use dialoguer::Input;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use serde_json::from_value;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::helpers::{self, AppError};

use crate::{notion_api, notion_props};

#[derive(Serialize, Deserialize, Debug)]
pub struct NotionKeys {
    pub api_key: String,
    pub database_id: String,
}

pub struct NotionManager {
    pub tasks: Vec<String>,
}

impl NotionManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub async fn add_task(&mut self, task: &str) {
        let (notion_api_key, database_key) = match get_notion_keys() {
            Some(value) => value,
            None => return,
        };

        let notion_api = notion_api::NotionApi::new(&notion_api_key, &database_key);
        match notion_api.add(task).await {
            Ok(()) => println!("Task added successfully"),
            Err(e) => helpers::handle_error(&e.to_string()),
        }
    }

    pub async fn list_all_tasks(&self) {
        let (notion_api_key, database_key) = match get_notion_keys() {
            Some(value) => value,
            None => return,
        };

        let notion_api = notion_api::NotionApi::new(&notion_api_key, &database_key);
        let pages = dbg!(notion_api.read_database_pages().await.unwrap_or_else(|e| {
            helpers::handle_error(&e.to_string());
            Vec::new()
        }));

        for page in pages {
            if let Some(page_properties) = page.as_object() {
                // props and order are not guaranteed, so we loop and stuff each one then print
                let mut title: (&str, Option<notion_props::Title>) = ("", None);
                let mut checkbox: (&str, Option<notion_props::Checkbox>) = ("", None);
                let mut date: (&str, Option<notion_props::Date>) = ("", None);
                let mut relation: (&str, Option<notion_props::Relation>) = ("", None);

                for prop in page_properties {
                    if title.1.is_none() {
                        title = (
                            prop.0,
                            from_value::<notion_props::Title>(prop.1.clone()).ok(),
                        );
                    }
                    if checkbox.1.is_none() {
                        checkbox = (
                            prop.0,
                            from_value::<notion_props::Checkbox>(prop.1.clone()).ok(),
                        );
                    }
                    if date.1.is_none() {
                        date = (
                            prop.0,
                            from_value::<notion_props::Date>(prop.1.clone()).ok(),
                        );
                    }
                    if relation.1.is_none() {
                        relation = (
                            prop.0,
                            from_value::<notion_props::Relation>(prop.1.clone()).ok(),
                        );
                    }
                }

                if let (field_name, Some(title)) = title {
                    print!("{}: ", field_name);
                    for text in title.title {
                        print!("{}", text.plain_text)
                    }
                    println!()
                }
                if let (field_name, Some(checkbox)) = checkbox {
                    println!("{}: {}", field_name, checkbox.checkbox);
                }
                if let (field_name, Some(date)) = date {
                    println!("{}: {}", field_name, date.date.start);
                }
                if let (field_name, Some(relation)) = relation {
                    println!("{}: {}", field_name, relation.id);
                }
                println!()
            }
        }
    }
}

fn get_notion_keys() -> Option<(String, String)> {
    let mut config_path = match config_dir() {
        Some(path) => path,
        None => {
            helpers::handle_error("Failed to find the config directory");
            return None;
        }
    };
    config_path.push("todoer");
    config_path.push("config");
    let (notion_api_key, database_key) = if config_path.exists() {
        match read_and_parse_config(&config_path) {
            Ok(keys) => keys,
            Err(e) => {
                helpers::handle_error(&e.to_string());
                match prompt_and_store_notion_keys() {
                    Ok(keys) => keys,
                    Err(e) => {
                        helpers::handle_error(&e.to_string());
                        return None;
                    }
                }
            }
        }
    } else {
        match prompt_and_store_notion_keys() {
            Ok(keys) => keys,
            Err(e) => {
                helpers::handle_error(&e.to_string());
                return None;
            }
        }
    };
    Some((notion_api_key, database_key))
}

fn prompt_and_store_notion_keys() -> Result<(String, String), AppError> {
    let api_key: String = Input::new()
        .with_prompt("Enter your Notion API key")
        .interact_text()
        .map_err(|e| AppError::IOError("Failed to get user api key".to_string(), e))?;

    let database_key: String = Input::new()
        .with_prompt("Enter your Notion database id")
        .interact_text()
        .map_err(|e| AppError::IOError("Failed to get user database id".to_string(), e))?;

    let keys = NotionKeys {
        api_key: api_key.clone(),
        database_id: database_key.clone(),
    };

    let keys_json = serde_json::to_string(&keys)
        .map_err(|e| AppError::JsonError("Failed to serialize keys".to_string(), e))?;

    let mut config_path = config_dir().ok_or(AppError::ConfigDirNotFound)?;

    config_path.push("todoer");
    std::fs::create_dir_all(&config_path)
        .map_err(|e| AppError::IOError("Failed to create config directory".to_string(), e))?;

    config_path.push("config");
    let mut file = File::create(&config_path)
        .map_err(|e| AppError::IOError("Failed to create config file".to_string(), e))?;
    file.write_all(keys_json.as_bytes())
        .map_err(|e| AppError::IOError("Failed to write API key to file".to_string(), e))?;

    Ok((api_key, database_key))
}

pub fn read_and_parse_config(config_path: &Path) -> Result<(String, String), AppError> {
    let content = fs::read_to_string(config_path).map_err(|_| AppError::ConfigReadError)?;

    let keys: NotionKeys =
        serde_json::from_str(&content).map_err(|_| AppError::ConfigParseError)?;

    if keys.api_key.is_empty() || keys.database_id.is_empty() {
        Err(AppError::ConfigReadError)
    } else {
        Ok((keys.api_key, keys.database_id))
    }
}

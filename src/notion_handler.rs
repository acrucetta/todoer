use dialoguer::Input;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use serde_json::from_value;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

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

    pub async fn add_page(&mut self, title: &str) {
        let (notion_api_key, database_key) = match get_notion_keys() {
            Some(value) => value,
            None => return,
        };

        let notion_api = notion_api::NotionApi::new(&notion_api_key, &database_key);
        let db_props = match notion_api.read_database_properties().await {
            Ok(props) => props,
            Err(e) => {
                helpers::handle_error(&e.to_string());
                return;
            }
        };

        let mut title_property: (String, Option<notion_props::SendTitle>) = ("".to_string(), None);
        let mut checkbox_properties: HashMap<String, Option<notion_props::SendCheckbox>> =
            HashMap::new();
        let mut date_properties: HashMap<String, Option<notion_props::SendDate>> = HashMap::new();
        let mut relation_properties: HashMap<String, Option<notion_props::SendRelation>> =
            HashMap::new();

        for prop in db_props {
            let property_type: &str = prop.1["type"].as_str().unwrap_or("");

            match property_type {
                "title" => {
                    let inner_text_to_send = notion_props::SendInnerText {
                        content: title.to_string(),
                    };
                    let text_to_send = notion_props::SendText {
                        text: inner_text_to_send,
                    };
                    title_property = (
                        prop.0.clone(),
                        Some(notion_props::SendTitle {
                            title: vec![text_to_send],
                        }),
                    );
                }
                "checkbox" => {
                    let bool_to_send: bool = loop {
                        let user_input: String = match Input::new()
                            .with_prompt(format!("{} (y/n/true/false)", prop.0).to_string())
                            .allow_empty(true)
                            .interact()
                        {
                            Ok(value) => value,
                            Err(e) => {
                                helpers::handle_error(
                                    &AppError::IOError(e.to_string(), e).to_string(),
                                );
                                continue;
                            }
                        };

                        if user_input.is_empty() {
                            break false;
                        }

                        match parse_bool(&user_input) {
                            Ok(value) => break value,
                            Err(_) => eprintln!(
                                "Please provide a valid input (y/n/true/false) or press enter to skip."
                            ),
                        }
                    };

                    checkbox_properties.insert(
                        prop.0.clone(),
                        Some(notion_props::SendCheckbox {
                            checkbox: bool_to_send,
                        }),
                    );
                }
                "date" => {
                    let inner_date_to_send = loop {
                        let user_input: String = match Input::new()
                            .with_prompt(format!("{} (yyyy-mm-dd)", prop.0).to_string())
                            .interact()
                        {
                            Ok(value) => value,
                            Err(e) => {
                                helpers::handle_error(
                                    &AppError::IOError(e.to_string(), e).to_string(),
                                );
                                continue;
                            }
                        };

                        let date = match chrono::NaiveDate::parse_from_str(&user_input, "%Y-%m-%d")
                        {
                            Ok(date) => date,
                            Err(_) => {
                                eprintln!("Please provide a valid input (yyyy-mm-dd)");
                                continue;
                            }
                        };

                        let iso_date = date.format("%Y-%m-%d").to_string();
                        if iso_date != user_input {
                            eprintln!(
                                "Please provide a valid input in the ISO 8601 format (yyyy-mm-dd)"
                            );
                            continue;
                        }

                        break notion_props::SendInnerDate {
                            start: iso_date,
                            end: None,
                        };
                    };

                    date_properties.insert(
                        prop.0.clone(),
                        Some(notion_props::SendDate {
                            date: inner_date_to_send,
                        }),
                    );
                }
                "relation" => {
                    let relations_to_send: Vec<String> = loop {
                        let user_input: String = match Input::new()
                            .with_prompt(
                                format!("{} (separate UUIDv4 with commas)", prop.0).to_string(),
                            )
                            .allow_empty(true)
                            .interact()
                        {
                            Ok(value) => value,
                            Err(e) => {
                                helpers::handle_error(
                                    &AppError::IOError(e.to_string(), e).to_string(),
                                );
                                continue;
                            }
                        };

                        if user_input.is_empty() {
                            break Vec::new();
                        } else if is_valid_relation_input(&user_input) {
                            break user_input
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .collect();
                        } else {
                            eprintln!("Please provide valid UUIDv4 (separate with commas) or press enter to skip.");
                        }
                    };

                    let relation_ids_to_send: Vec<notion_props::SendRelationId> = relations_to_send
                        .iter()
                        .map(|id| notion_props::SendRelationId { id: id.clone() })
                        .collect();

                    relation_properties.insert(
                        prop.0.clone(),
                        Some(notion_props::SendRelation {
                            relation: relation_ids_to_send,
                        }),
                    );
                }
                _ => {}
            }
        }

        match notion_api
            .add(
                title_property,
                checkbox_properties,
                date_properties,
                relation_properties,
            )
            .await
        {
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
        let pages = notion_api.read_database_pages().await.unwrap_or_else(|e| {
            helpers::handle_error(&e.to_string());
            Vec::new()
        });

        for page in pages {
            let mut title_property: (&str, Option<notion_props::Title>) = ("", None);
            let mut checkbox_properties: HashMap<&str, Option<notion_props::Checkbox>> =
                HashMap::new();
            let mut date_properties: HashMap<&str, Option<notion_props::Date>> = HashMap::new();
            let mut relation_properties: HashMap<&str, Option<notion_props::Relation>> =
                HashMap::new();

            if let Some(page_properties) = page.as_object() {
                for prop in page_properties {
                    let property_type: &str = prop.1["type"].as_str().unwrap_or("");
                    match property_type {
                        "title" => {
                            title_property = (
                                &prop.0,
                                from_value::<notion_props::Title>(prop.1.clone()).ok(),
                            );
                        }
                        "checkbox" => {
                            checkbox_properties.insert(
                                &prop.0,
                                from_value::<notion_props::Checkbox>(prop.1.clone()).ok(),
                            );
                        }
                        "date" => {
                            date_properties.insert(
                                prop.0,
                                from_value::<notion_props::Date>(prop.1.clone()).ok(),
                            );
                        }
                        "relation" => {
                            relation_properties.insert(
                                prop.0,
                                from_value::<notion_props::Relation>(prop.1.clone()).ok(),
                            );
                        }
                        _ => {
                            helpers::handle_error("Read unsupported property type");
                        }
                    }
                }

                if let (field_name, Some(title)) = title_property {
                    print!("{}: ", field_name);
                    for text in title.title {
                        print!("{}", text.plain_text)
                    }
                    println!()
                }
                for (field_name, value) in checkbox_properties {
                    if let Some(checkbox) = value {
                        println!("{}: {}", field_name, checkbox.checkbox);
                    }
                }
                for (field_name, value) in date_properties {
                    if let Some(date) = value {
                        println!(
                            "{}: {}",
                            field_name,
                            date.date
                                .map_or_else(|| "null".to_string(), |date| date.start)
                        );
                    }
                }
                for (field_name, value) in relation_properties {
                    if let Some(relation) = value {
                        println!("{}: {}", field_name, relation.id);
                    }
                }
                println!()
            }
        }
    }

    pub fn remove_notion_keys(&self) {
        let config_path = match get_config_path() {
            Ok(config_path) => config_path,
            Err(e) => {
                helpers::handle_error(&e.to_string());
                return;
            }
        };

        let content = match fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(e) => {
                helpers::handle_error(&AppError::IOError("".to_string(), e).to_string());
                return;
            }
        };

        let mut config_json: serde_json::Value = match serde_json::from_str(&content) {
            Ok(value) => value,
            Err(e) => {
                helpers::handle_error(
                    &AppError::JsonError("Could not deserialize config content".to_string(), e)
                        .to_string(),
                );
                return;
            }
        };

        if let Some(obj) = config_json.as_object_mut() {
            obj.insert(
                "api_key".to_string(),
                serde_json::Value::String("".to_string()),
            );
            obj.insert(
                "database_id".to_string(),
                serde_json::Value::String("".to_string()),
            );
        } else {
            helpers::handle_error("Unable to remove notion keys");
            return;
        }

        let serialized_updated_config_content = match serde_json::to_string(&config_json) {
            Ok(content) => content,
            Err(e) => {
                helpers::handle_error(
                    &AppError::JsonError("Could not serialize new config content".to_string(), e)
                        .to_string(),
                );
                return;
            }
        };

        let mut updated_config_file = match File::create(&config_path) {
            Ok(file) => file,
            Err(e) => {
                helpers::handle_error(
                    &AppError::IOError("Could not create updated config file".to_string(), e)
                        .to_string(),
                );
                return;
            }
        };

        match updated_config_file.write_all(serialized_updated_config_content.as_bytes()) {
            Ok(_) => println!("Successfully removed notion keys"),
            Err(e) => helpers::handle_error(
                &AppError::IOError("Could not write update config file".to_string(), e).to_string(),
            ),
        }
    }
}

fn get_notion_keys() -> Option<(String, String)> {
    let config_path = match get_config_path() {
        Ok(config_path) => config_path,
        Err(e) => {
            helpers::handle_error(&e.to_string());
            return None;
        }
    };

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

fn get_config_path() -> Result<PathBuf, AppError> {
    let mut config_path = match config_dir() {
        Some(path) => path,
        None => {
            return Err(AppError::ConfigDirNotFound);
        }
    };
    config_path.push("todoer");
    config_path.push("config");
    Ok(config_path)
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

fn read_and_parse_config(config_path: &Path) -> Result<(String, String), AppError> {
    let content = fs::read_to_string(config_path).map_err(|_| AppError::ConfigReadError)?;

    let keys: NotionKeys =
        serde_json::from_str(&content).map_err(|_| AppError::ConfigParseError)?;

    if keys.api_key.is_empty() || keys.database_id.is_empty() {
        Err(AppError::ConfigReadError)
    } else {
        Ok((keys.api_key, keys.database_id))
    }
}

fn parse_bool(input: &str) -> Result<bool, &'static str> {
    match input.to_lowercase().as_str() {
        "true" | "y" => Ok(true),
        "false" | "n" => Ok(false),
        _ => Err("Invalid input"),
    }
}

fn is_valid_relation_input(input: &str) -> bool {
    input.split(',').all(|s| {
        !s.is_empty()
            && Uuid::parse_str(s.trim()).map_or(false, |uuid| {
                uuid.get_version() == Some(uuid::Version::Random)
            })
    })
}

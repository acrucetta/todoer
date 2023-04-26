use dialoguer::Input;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use serde_json::from_value;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

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

    pub async fn add_page(&mut self, task: &str) {
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

        // props and order are not guaranteed, so we loop and stuff each one then print
        let mut title: (String, Option<notion_props::Title>) = ("".to_string(), None);
        let mut title_to_send: (String, Option<notion_props::SendTitle>) = ("".to_string(), None);
        let mut checkbox: (String, Option<notion_props::Checkbox>) = ("".to_string(), None);
        let mut date: (String, Option<notion_props::Date>) = ("".to_string(), None);
        let mut relation: (String, Option<notion_props::Relation>) = ("".to_string(), None);

        for prop in db_props {
            // todo: title is a guarantee, just skip this
            if title.1.is_none() {
                title = (
                    prop.0.clone(),
                    from_value::<notion_props::Title>(prop.1.clone()).ok(),
                );
            }
            if title_to_send.1.is_none()
                && from_value::<notion_props::Title>(prop.1.clone()).is_ok()
            {
                let content: String = match Input::new().with_prompt("Title?:").interact_text() {
                    Ok(title) => title,
                    Err(e) => {
                        helpers::handle_error(
                            &AppError::IOError("Failed to get title".to_string(), e).to_string(),
                        );
                        return;
                    }
                };
                let inner_text_to_send = notion_props::SendInnerText { content };
                let text_to_send = notion_props::SendText {
                    text: inner_text_to_send,
                };
                title_to_send = (
                    prop.0.clone(),
                    Some(notion_props::SendTitle {
                        title: vec![text_to_send],
                    }),
                );
            }
            // if checkbox.1.is_none() {
            //     checkbox = (
            //         prop.0.clone(),
            //         from_value::<notion_props::Checkbox>(prop.1.clone()).ok(),
            //     );
            // }
            // if date.1.is_none() {
            //     date = (
            //         prop.0.clone(),
            //         from_value::<notion_props::Date>(prop.1.clone()).ok(),
            //     );
            // }
            // if relation.1.is_none() {
            //     relation = (
            //         prop.0.clone(),
            //         from_value::<notion_props::Relation>(prop.1.clone()).ok(),
            //     );
            // }
        }

        match notion_api.add(title_to_send).await {
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

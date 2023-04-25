use serde_json::{json, Value};

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, StatusCode};

use crate::helpers::AppError;

pub struct NotionApi {
    api_key: String,
    database_id: String,
}

impl NotionApi {
    pub fn new(api_key: &str, database_id: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            database_id: database_id.to_string(),
        }
    }

    pub async fn add(&self, task_title: &str) -> Result<(), AppError> {
        let bearer_token = format!("Bearer {}", &self.api_key);

        let body = json!({
            "parent": { "database_id": &self.database_id },
            "properties": {
                "title": {
                    "title": [
                        {
                            "text": {
                                "content": task_title
                            }
                        }
                    ]
                }
            }
        });

        let body_string = body.to_string();

        let client = Client::new();
        let res = client
            .post("https://api.notion.com/v1/pages")
            .header(AUTHORIZATION, bearer_token)
            .header(CONTENT_TYPE, "application/json")
            .header("Notion-Version", "2021-08-16")
            .body(body_string)
            .send()
            .await
            .map_err(|e| AppError::ReqwestError(e.to_string(), e))?;

        match res.status() {
            StatusCode::OK => Ok(()),
            StatusCode::BAD_REQUEST => {
                let error_message = res
                    .text()
                    .await
                    .map_err(|e| AppError::ReqwestError(e.to_string(), e))?;
                Err(AppError::InvalidArgument(error_message))
            }
            _ => Err(AppError::UnknownError(format!(
                "Notion API returned an unexpected response status: {}",
                res.status()
            ))),
        }
    }

    pub async fn read_database_pages(&self) -> Result<Vec<Value>, AppError> {
        let bearer_token = format!("Bearer {}", &self.api_key);
        let post_url = format!(
            "https://api.notion.com/v1/databases/{}/query",
            &self.database_id
        );

        let client = Client::new();
        let res = client
            .post(post_url)
            .header(AUTHORIZATION, bearer_token)
            .header("Notion-Version", "2022-06-28")
            .header(CONTENT_TYPE, "application/json")
            .body(json!({}).to_string())
            .send()
            .await
            .map_err(|e| AppError::ReqwestError(e.to_string(), e))?;

        match res.status() {
            StatusCode::OK => {
                let body = res
                    .text()
                    .await
                    .map_err(|e| AppError::ReqwestError(e.to_string(), e))?;

                let json: Value = serde_json::from_str(&body)
                    .map_err(|e| AppError::JsonError(e.to_string(), e))?;

                let results = json
                    .as_object()
                    .and_then(|o| (o.get("results")))
                    .ok_or_else(|| AppError::MapError("Key 'results' not found in map".to_string()))
                    .and_then(|v| {
                        v.as_array().ok_or_else(|| {
                            AppError::MapError("'results' is not an array".to_string())
                        })
                    });

                let properties_for_all_pages = results.map(|vec| {
                    vec.iter()
                        .filter_map(|item| item.get("properties"))
                        .cloned()
                        .collect::<Vec<Value>>()
                })?;

                Ok(properties_for_all_pages)
            }
            StatusCode::BAD_REQUEST => {
                let error_message = res
                    .text()
                    .await
                    .map_err(|e| AppError::ReqwestError(e.to_string(), e))?;
                Err(AppError::InvalidArgument(error_message))
            }
            _ => Err(AppError::UnknownError(format!(
                "Notion API returned an unexpected response status: {}",
                res.status()
            ))),
        }
    }

    pub async fn read_database_properties(&self) -> Result<(), AppError> {
        let bearer_token = format!("Bearer {}", &self.api_key);
        let post_url = format!("https://api.notion.com/v1/databases/{}", &self.database_id);

        let client = Client::new();
        let res = client
            .get(post_url)
            .header(AUTHORIZATION, bearer_token)
            .header("Notion-Version", "2022-06-28")
            .header(CONTENT_TYPE, "application/json")
            .body(json!({}).to_string())
            .send()
            .await
            .map_err(|e| AppError::ReqwestError(e.to_string(), e))?;

        match res.status() {
            StatusCode::OK => {
                let body = res
                    .text()
                    .await
                    .map_err(|e| AppError::ReqwestError(e.to_string(), e))?;

                let json: Value = serde_json::from_str(&body)
                    .map_err(|e| AppError::JsonError(e.to_string(), e))?;

                let results = json
                    .as_object()
                    .and_then(|o| o.get("properties"))
                    .ok_or_else(|| {
                        AppError::MapError("Key 'properties' not found in map".to_string())
                    })
                    .and_then(|v| {
                        v.as_array().ok_or_else(|| {
                            AppError::MapError("'properties' is not an array".to_string())
                        })
                    });

                match results {
                    Ok(vec) => {
                        for item in vec {
                            println!("{}\n", item);
                        }
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }
            StatusCode::BAD_REQUEST => {
                let error_message = res
                    .text()
                    .await
                    .map_err(|e| AppError::ReqwestError(e.to_string(), e))?;
                Err(AppError::InvalidArgument(error_message))
            }
            _ => Err(AppError::UnknownError(format!(
                "Notion API returned an unexpected response status: {}",
                res.status()
            ))),
        }
    }
}

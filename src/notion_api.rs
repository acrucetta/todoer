use serde_json::json;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Error};

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

    pub async fn add(&self, task_title: &str) -> Result<(), Error> {
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
            .await?;

        println!("{:?}", res);
        Ok(())
    }

    // Implement other API methods here, using self.database_id as needed
}

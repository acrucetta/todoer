// notion docs: https://developers.notion.com/reference/page-property-values#date
use serde::{Deserialize, Serialize};

// Checkbox
#[derive(Debug, Deserialize)]
pub struct Checkbox {
    pub checkbox: bool,
    pub id: String,
    pub r#type: String,
}

#[derive(Serialize, Debug)]
pub struct SendCheckbox {
    pub checkbox: bool,
}

// Date
#[derive(Debug, Deserialize)]
pub struct Date {
    pub date: InnerDate,
    pub id: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct InnerDate {
    pub end: Option<String>,
    pub start: String,
    pub time_zone: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct SendDate {
    date: SendInnerDate,
}

#[derive(Serialize, Debug)]
pub struct SendInnerDate {
    start: String,
    end: Option<String>,
}

// Title
#[derive(Deserialize, Debug)]
pub struct Title {
    pub id: String,
    pub title: Vec<Text>,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct Text {
    pub annotations: Annotations,
    pub href: Option<serde_json::Value>,
    pub plain_text: String,
    pub text: InnerText,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct Annotations {
    pub bold: bool,
    pub code: bool,
    pub color: String,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
}

#[derive(Deserialize, Debug)]
pub struct InnerText {
    pub content: String,
    pub link: Option<Link>,
}

#[derive(Serialize, Debug)]
pub struct SendTitle {
    pub title: Vec<SendText>,
}

#[derive(Serialize, Debug)]
pub struct SendText {
    pub text: SendInnerText,
}

#[derive(Serialize, Debug)]
pub struct SendInnerText {
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    pub url: String,
}

// Relation
#[derive(serde::Deserialize)]
pub struct Relation {
    pub has_more: bool,
    pub id: String,
    pub relation: Vec<serde_json::Value>,
    pub r#type: String,
}

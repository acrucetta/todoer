use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Checkbox {
    pub checkbox: bool,
    pub id: String,
    pub r#type: String,
}

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

#[derive(Deserialize, Debug)]
pub struct Link {
    pub url: String,
}

#[derive(serde::Deserialize)]
pub struct Relation {
    pub has_more: bool,
    pub id: String,
    pub relation: Vec<serde_json::Value>,
    pub r#type: String,
}

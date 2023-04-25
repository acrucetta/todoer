#[derive(Debug, Deserialize)]
pub struct Checkbox {
    checkbox: bool,
    id: String,
    r#type: String,
}

#[derive(Debug, Deserialize)]
struct DueDate {
    date: Date,
    id: String,
    r#type: String,
}

#[derive(Debug, Deserialize)]
struct Date {
    end: Option<String>,
    start: String,
    time_zone: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Name {
    id: String,
    title: Vec<Title>,
    r#type: String,
}

#[derive(Deserialize, Debug)]
struct Title {
    annotations: Annotations,
    href: Option<serde_json::Value>,
    plain_text: String,
    text: Text,
    r#type: String,
}

#[derive(Deserialize, Debug)]
struct Annotations {
    bold: bool,
    code: bool,
    color: String,
    italic: bool,
    strikethrough: bool,
    underline: bool,
}

#[derive(Deserialize, Debug)]
struct Text {
    content: String,
    link: Option<Link>,
}

#[derive(Deserialize, Debug)]
struct Link {
    url: String,
}

#[derive(serde::Deserialize)]
struct Project {
    has_more: bool,
    id: String,
    relation: Vec<serde_json::Value>,
    r#type: String,
}

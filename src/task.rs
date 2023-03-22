
pub struct Task {
    pub id: u32,
    pub description: String,
    pub tags: Vec<String>,
    pub due: String,
    pub timestamp: String,
    pub priority: String,
    pub status: Status,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Todo,
    Done,
}


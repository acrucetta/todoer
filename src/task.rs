
pub struct Task {
    pub id: u32,
    pub description: String,
    pub tags: Vec<String>,
    pub due: String,
    pub timestamp: String,
    pub priority: String,
    pub status: Status,
}
impl Task {
    pub(crate) fn new() -> Task {
        Task {
            id: 0,
            description: "".to_string(),
            tags: Vec::new(),
            due: "".to_string(),
            timestamp: "".to_string(),
            priority: "".to_string(),
            status: Status::Todo,
        }
    }
}

pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Todo,
    Done,
    Hold,
    Blocked,
}

pub enum Due {
    Today,
    Tomorrow,
    ThisWeek,
    ThisMonth,
    ThisYear,
    Overdue,
}

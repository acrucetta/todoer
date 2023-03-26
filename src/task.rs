use core::{time, fmt};
use std::time::SystemTime;

pub struct Task {
    pub id: u32,
    pub description: String,
    pub tags: Vec<String>,
    pub due: Due,
    pub timestamp: SystemTime,
    pub priority: Priority,
    pub status: Status,
}
impl Task {
    pub(crate) fn new() -> Task {
        Task {
            id: 0,
            description: "".to_string(),
            tags: Vec::new(),
            due: Due::Today,
            timestamp: SystemTime::now(),
            priority: Priority::Low,
            status: Status::Todo,
        }
    }
}

pub enum Priority {
    Low,
    Medium,
    High,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
        }
    }
}

impl Priority {
    pub fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_string(),
            Priority::Medium => "Medium".to_string(),
            Priority::High => "High".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Todo,
    Done,
    Hold,
    Blocked,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Todo => write!(f, "Todo"),
            Status::Done => write!(f, "Done"),
            Status::Hold => write!(f, "Hold"),
            Status::Blocked => write!(f, "Blocked"),
        }
    }
}

impl Status {
    pub fn to_string(&self) -> String {
        match self {
            Status::Todo => "Todo".to_string(),
            Status::Done => "Done".to_string(),
            Status::Hold => "Hold".to_string(),
            Status::Blocked => "Blocked".to_string(),
        }
    }
}


pub enum Due {
    Today,
    Tomorrow,
    ThisWeek,
    Sometime,
    ThisMonth,
    ThisYear,
    Overdue,
}

impl fmt::Display for Due {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Due::Today => write!(f, "Today"),
            Due::Tomorrow => write!(f, "Tomorrow"),
            Due::ThisWeek => write!(f, "This week"),
            Due::ThisMonth => write!(f, "This month"),
            Due::ThisYear => write!(f, "This year"),
            Due::Overdue => write!(f, "Overdue"),
            Due::Sometime => todo!(),
        }
    }
}

impl Due {
    pub fn to_string(&self) -> String {
        match self {
            Due::Today => "Today".to_string(),
            Due::Tomorrow => "Tomorrow".to_string(),
            Due::ThisWeek => "ThisWeek".to_string(),
            Due::ThisMonth => "ThisMonth".to_string(),
            Due::ThisYear => "ThisYear".to_string(),
            Due::Overdue => "Overdue".to_string(),
            Due::Sometime => "Sometime".to_string(),
        }
    }
}


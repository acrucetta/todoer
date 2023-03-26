use core::{fmt};
use std::{time::SystemTime};

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

    pub(crate) fn from_record(record: csv::StringRecord) -> Task {
        let task = Task {
            id: record.get(0).unwrap().parse().unwrap(),
            description: record[1].to_string(),
            tags: record[2].split(',').map(|s| s.to_string()).collect(),
            due: match record[3].as_ref() {
                "Today" => Due::Today,
                "Tomorrow" => Due::Tomorrow,
                "ThisWeek" => Due::ThisWeek,
                "Sometime" => Due::Sometime,
                _ => Due::Sometime,
            },
            timestamp: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(record[4].parse().unwrap()),
            priority: match record[5].as_ref() {
                "Low" => Priority::Low,
                "Medium" => Priority::Medium,
                "High" => Priority::High,
                _ => Priority::Low,
            },
            status: match record[6].as_ref() {
                "Todo" => Status::Todo,
                "Blocked" => Status::Blocked,
                "Done" => Status::Done,
                "Hold" => Status::Hold,
                _ => Status::Todo,
            },
        };
        task
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
            Due::Sometime => write!(f, "Sometime"),
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

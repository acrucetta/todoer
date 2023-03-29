use core::fmt;
use std::time::SystemTime;

use chrono::NaiveDate;

pub struct Task {
    pub id: u32,
    pub description: String,
    pub tags: Vec<String>,
    pub due: NaiveDate,
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
            due: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
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
            // Parse the date, it will be in the format of YYYY-MM-DD
            due: match NaiveDate::parse_from_str(&record[3], "%Y-%m-%d") {
                Ok(date) => date,
                Err(_) => {
                    // If the date is invalid, set it to 2023-01-01
                    NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
                }
            },
            timestamp: SystemTime::UNIX_EPOCH
                + std::time::Duration::from_secs(record[4].parse().unwrap()),
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

    pub(crate) fn cmp(&self, priority: &Priority) -> std::cmp::Ordering {
        match self {
            Priority::Low => match priority {
                Priority::Low => std::cmp::Ordering::Equal,
                Priority::Medium => std::cmp::Ordering::Less,
                Priority::High => std::cmp::Ordering::Less,
            },
            Priority::Medium => match priority {
                Priority::Low => std::cmp::Ordering::Greater,
                Priority::Medium => std::cmp::Ordering::Equal,
                Priority::High => std::cmp::Ordering::Less,
            },
            Priority::High => match priority {
                Priority::Low => std::cmp::Ordering::Greater,
                Priority::Medium => std::cmp::Ordering::Greater,
                Priority::High => std::cmp::Ordering::Equal,
            },
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

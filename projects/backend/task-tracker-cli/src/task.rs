use std::fmt::Display;

use chrono::{DateTime, Utc};

pub type Timestamp = DateTime<Utc>;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Id(pub String);

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Task {
    id: Id,
    description: String,
    status: Status,
    created_at: Timestamp,
    updated_at: Timestamp,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Status {
    #[default]
    Todo,
    InProgress,
    Done,
}

impl Task {
    pub fn new(
        id: Id,
        description: impl Into<String>,
        status: Status,
        created_at: Timestamp,
        updated_at: Timestamp,
    ) -> Self {
        Self {
            id,
            description: description.into(),
            status,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn description(&self) -> String {
        self.description.to_string()
    }

    pub fn status(&self) -> Status {
        self.status.clone()
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn updated_at(&self) -> Timestamp {
        self.updated_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_task() {
        let now = Utc::now();

        let task = Task::new(Id("1".to_string()), "Create a Task", Status::Todo, now, now);

        assert_eq!(task.id(), "1");
        assert_eq!(task.description(), "Create a Task");
        assert_eq!(task.status(), Status::Todo);
        assert_eq!(task.created_at(), now);
        assert_eq!(task.updated_at(), now);
    }
}

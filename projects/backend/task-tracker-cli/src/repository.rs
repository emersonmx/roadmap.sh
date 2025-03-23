use crate::task::Task;

pub trait TaskRepository {
    fn list_all(&self) -> Vec<Task>;
    fn save(&mut self, task: Task);
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MemoryTaskRepository(Vec<Task>);

impl MemoryTaskRepository {
    pub fn new() -> Self {
        MemoryTaskRepository(vec![])
    }
}

impl TaskRepository for MemoryTaskRepository {
    fn list_all(&self) -> Vec<Task> {
        self.0.clone()
    }

    fn save(&mut self, task: Task) {
        self.0.push(task);
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::task::{Id, Status};

    use super::*;

    #[test]
    fn list_all_tasks() {
        let now = Utc::now();
        let mut repo = MemoryTaskRepository::new();
        let task = Task::new(Id("1".to_string()), "a task", Status::Todo, now, now);
        repo.save(task.clone());

        let tasks = repo.list_all();

        assert_eq!(tasks, vec![task]);
    }
}

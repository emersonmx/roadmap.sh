use chrono::Utc;

use crate::{
    id_generator::IdGenerator,
    task::{Id, Status, Task},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Input {
    pub description: String,
}

pub fn execute(id_generator: &mut impl IdGenerator, input: Input) -> Task {
    let now = Utc::now();
    let next_id = id_generator.next_id();
    Task::new(Id(next_id), input.description, Status::Todo, now, now)
}

#[cfg(test)]
mod tests {
    use chrono::TimeDelta;

    use crate::{id_generator::NumberIdGenerator, task::Status};

    use super::*;

    fn make_id_gen() -> NumberIdGenerator {
        NumberIdGenerator(0)
    }

    #[test]
    fn add_a_new_task() {
        let mut id_gen = make_id_gen();

        let task = execute(
            &mut id_gen,
            Input {
                description: "A new task".to_string(),
            },
        );

        assert_eq!(task.id(), "1");
        assert_eq!(task.description(), "A new task");
        assert_eq!(task.status(), Status::Todo);
        assert_eq!(task.created_at(), task.updated_at());
        assert!(Utc::now() - task.created_at() <= TimeDelta::milliseconds(1));
        assert!(Utc::now() - task.updated_at() <= TimeDelta::milliseconds(1));
    }
}

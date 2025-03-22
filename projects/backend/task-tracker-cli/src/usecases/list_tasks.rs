use crate::task::Task;

pub fn execute() -> Vec<Task> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_empty_tasks() {
        let tasks = execute();

        assert_eq!(tasks, vec![]);
    }
}

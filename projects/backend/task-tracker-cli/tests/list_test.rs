use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn add_task() {
    Command::cargo_bin("task-cli")
        .unwrap()
        .arg("add")
        .arg("a new task")
        .assert()
        .success()
        .stdout(contains("Task added successfully (ID: 1)"));
}

#[test]
fn empty_list_all() {
    let mut cmd = Command::cargo_bin("task-cli")
        .unwrap()
        .arg("list")
        .assert()
        .success();

    for c in ["TODO", "IN PROGRESS", "DONE"] {
        cmd = cmd.stdout(contains(c));
    }
}

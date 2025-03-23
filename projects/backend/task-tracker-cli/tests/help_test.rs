use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn show_help() {
    let mut cmd = Command::cargo_bin("task-cli")
        .unwrap()
        .arg("--help")
        .assert()
        .success();

    for c in ["add", "list"] {
        cmd = cmd.stdout(contains(c));
    }
}

#[test]
fn default_to_help() {
    let mut cmd = Command::cargo_bin("task-cli").unwrap();
    cmd.assert()
        .failure()
        .stderr(contains("A simple task tracker"));
}

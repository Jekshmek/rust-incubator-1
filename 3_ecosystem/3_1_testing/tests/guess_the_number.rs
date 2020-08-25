use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn empty_env_args() {
    let output = Command::new("cargo")
        .args(&["run", "-p", "step_3_1"])
        .output()
        .expect("Failed to run step_3_1");

    assert!(
        !output.status.success(),
        "Process did not fail without env argument"
    );
}

#[test]
fn too_big_number() {
    let output = Command::new("cargo")
        .args(&["run", "-p", "step_3_1", "--", &u64::MAX.to_string()])
        .output()
        .expect("Failed to run step_3_1");

    assert!(
        !output.status.success(),
        "Process did not fail with u64::MAX number"
    );
}

#[test]
fn negative_number() {
    let output = Command::new("cargo")
        .args(&["run", "-p", "step_3_1", "--", "-5"])
        .output()
        .expect("Failed to run step_3_1");

    assert!(
        !output.status.success(),
        "Process did not fail with negative number"
    );
}

#[test]
fn first_guess() {
    let mut child = Command::new("cargo")
        .args(&["run", "-p", "step_3_1", "--", "5"])
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to run step_3_1");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all("5".as_bytes())
            .expect("Failed to write to stdin");
    }

    assert_eq!(
        child
            .wait_with_output()
            .expect("Process did not end after right number was given")
            .stdout,
        "Guess the number!\nPlease input your guess.\nYou guessed: 5\nYou win!\n".as_bytes()
    );
}

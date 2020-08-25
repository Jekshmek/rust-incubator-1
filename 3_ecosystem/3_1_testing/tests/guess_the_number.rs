use std::ffi::OsStr;
use std::io::Write;
use std::process::{Child, Command, Output, Stdio};

#[test]
fn empty_env_args() {
    let empty: &[String] = &[];
    let output = get_output(empty);

    assert!(
        !output.status.success(),
        "Process did not fail without env argument"
    );
}

#[test]
fn too_big_number() {
    let output = get_output(&[&u64::MAX.to_string()]);

    assert!(
        !output.status.success(),
        "Process did not fail with u64::MAX number"
    );
}

#[test]
fn negative_number() {
    let output = get_output(&["-5"]);

    assert!(
        !output.status.success(),
        "Process did not fail with negative number"
    );
}

#[test]
fn first_guess() {
    let mut child = spawn_child(&["5"]);

    write(&mut child, "5".as_bytes());
    assert_eq!(
        get_stdout(child),
        "Guess the number!\nPlease input your guess.\nYou guessed: 5\nYou win!\n".as_bytes()
    );
}

#[test]
fn trailing_whitespaces() {
    let mut child = spawn_child(&["    25"]);

    write(&mut child, "25".as_bytes());

    assert_eq!(
        get_stdout(child),
        "Guess the number!\nPlease input your guess.\nYou guessed: 25\nYou win!\n".as_bytes()
    );
}

fn spawn_child<I, S>(args: I) -> Child
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new("cargo")
        .args(&["run", "-p", "step_3_1", "--"])
        .args(args)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to run step_3_1")
}

fn get_output<I, S>(args: I) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new("cargo")
        .args(&["run", "-p", "step_3_1", "--"])
        .args(args)
        .output()
        .expect("Failed to run step_3_1")
}

fn write<'a, B: Into<&'a [u8]>>(child: &mut Child, buf: B) {
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(buf.into())
        .expect("Failed to write to stdin");
}

fn get_stdout(child: Child) -> Vec<u8> {
    child
        .wait_with_output()
        .expect("Process did not end after right number was given")
        .stdout
}

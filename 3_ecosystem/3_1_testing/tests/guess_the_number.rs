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
fn non_number_in_env_args() {
    let output = get_output(&["not_a_number"]);
    assert!(!output.status.success());
}

#[test]
fn non_number_in_stdin() {
    let mut child = spawn_child(&["123"]);

    write(&mut child, &b"non_number\n"[..]);
    write(&mut child, &b"123"[..]);

    assert_eq!(
        get_stdout(child),
        "Guess the number!\nPlease input your guess.\nPlease input your guess.\nYou guessed: 123\nYou win!\n".as_bytes()
    );
}

#[test]
fn first_guess() {
    let mut child = spawn_child(&["5"]);

    write(&mut child, &b"5"[..]);
    assert_eq!(
        get_stdout(child),
        "Guess the number!\nPlease input your guess.\nYou guessed: 5\nYou win!\n".as_bytes()
    );
}

#[test]
fn trailing_whitespaces() {
    let mut child = spawn_child(&["    25"]);

    write(&mut child, &b"25"[..]);

    assert_eq!(
        get_stdout(child),
        "Guess the number!\nPlease input your guess.\nYou guessed: 25\nYou win!\n".as_bytes()
    );
}

#[test]
fn too_small() {
    let mut child = spawn_child(&["10"]);

    write(&mut child, &b"   9\n"[..]);
    write(&mut child, &b" 10\n"[..]);

    assert_eq!(
        get_stdout(child),
        "Guess the number!\nPlease input your guess.\nYou guessed: 9\nToo small!\nPlease input your guess.\nYou guessed: 10\nYou win!\n".as_bytes()
    );
}

#[test]
fn too_big() {
    let mut child = spawn_child(&["10"]);

    write(&mut child, &b"11\n"[..]);
    write(&mut child, &b"10\n"[..]);

    assert_eq!(
        get_stdout(child),
        "Guess the number!\nPlease input your guess.\nYou guessed: 11\nToo big!\nPlease input your guess.\nYou guessed: 10\nYou win!\n".as_bytes()
    );
}

#[test]
fn random_input() {
    let rand_array: [u32; 32] = rand::random();
    let guess_val: u32 = rand::random();
    let mut expected_result = "Guess the number!\n".to_string();

    let mut child = spawn_child(&[guess_val.to_string()]);

    rand_array
        .iter()
        .filter(|num| **num != guess_val)
        .for_each(|num| {
            let str = num.to_string() + "\n";
            write(&mut child, str.as_bytes());

            let expected_str = if *num > guess_val {
                format!("Please input your guess.\nYou guessed: {}\nToo big!\n", num)
            } else {
                format!(
                    "Please input your guess.\nYou guessed: {}\nToo small!\n",
                    num
                )
            };

            expected_result.push_str(expected_str.as_str());
        });

    write(&mut child, guess_val.to_string().as_bytes());
    expected_result.push_str(
        format!(
            "Please input your guess.\nYou guessed: {}\nYou win!\n",
            guess_val
        )
        .as_str(),
    );

    assert_eq!(get_stdout(child), expected_result.as_bytes());
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

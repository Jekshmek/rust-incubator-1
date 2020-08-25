use std::process::Command;

#[test]
fn empty_env_args() {
    let output = Command::new("cargo")
        .args(&["run", "-p", "step_3_1"])
        .output()
        .unwrap();

    assert!(!output.status.success());
}

#[test]
fn too_big_number() {
    let output = Command::new("cargo")
        .args(&["run", "-p", "step_3_1", "--", &u64::MAX.to_string()])
        .output()
        .unwrap();

    assert!(!output.status.success());
}

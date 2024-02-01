use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}

#[test]
fn runs() -> Result<()>{
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();

    Ok(())
}

#[test]
fn hi1() -> Result<()>{
    run(&["Hi there"], "tests/files/hi1.txt")
}

#[test]
fn hi2() -> Result<()>{
    run(&["Hi", "there"], "tests/files/hi2.txt")
}

#[test]
fn hi1_no_newline() -> Result<()>{
    run(&["Hi", "there", "-n"], "tests/files/hi1.n.txt")
}

#[test]
fn hi2_no_newline() -> Result<()>{
    run(&["-n", "Hi", "there"], "tests/files/hi2.n.txt")
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args).assert().success().stdout(expected);

    Ok(())
}

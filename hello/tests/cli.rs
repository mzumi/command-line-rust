use assert_cmd::Command;

#[test]
fn work() {
    assert!(true);
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn abort_not_ok() {
    let mut cmd = Command::cargo_bin("abort").unwrap();
    cmd.assert().failure();
}

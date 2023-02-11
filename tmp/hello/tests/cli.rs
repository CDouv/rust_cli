use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();;
    let res = cmd.output();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
}

#[test]
fn false_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();

    cmd.assert().failure();
}
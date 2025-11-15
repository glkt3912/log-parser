use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_help_message() {
    let mut cmd = Command::cargo_bin("log-parser").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("高性能ログファイル解析"));
}

#[test]
fn test_missing_file_argument() {
    let mut cmd = Command::cargo_bin("log-parser").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("FILE"));
}

#[test]
fn test_nonexistent_file() {
    let mut cmd = Command::cargo_bin("log-parser").unwrap();
    cmd.arg("nonexistent.log")
        .assert()
        .failure();
}

#[test]
fn test_basic_log_processing() {
    // Create a temporary log file
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(
        temp_file,
        "2024-01-01 12:00:00 [INFO] Application started\n\
         2024-01-01 12:01:00 [ERROR] Database connection failed\n\
         2024-01-01 12:02:00 [INFO] Retrying connection"
    ).unwrap();

    let mut cmd = Command::cargo_bin("log-parser").unwrap();
    cmd.arg(temp_file.path())
        .assert()
        .success();
        // Additional assertions will be added when core functionality is implemented
}
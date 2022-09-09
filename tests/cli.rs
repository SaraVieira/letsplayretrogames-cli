use assert_cmd::prelude::*;
use letsplayretrogames::{ERROR_NO_RESULTS, ERROR_PATTERN_EMPTY};
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs // Add methods on commands

use assert_fs::{prelude::*, NamedTempFile};

fn create_file() -> Result<NamedTempFile, Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    Ok(file)
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = create_file()?;
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}

#[test]
fn contents_not_found_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("nothing matches")?;
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(ERROR_NO_RESULTS));

    Ok(())
}

#[test]
fn empty_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let file = create_file()?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("").arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(ERROR_PATTERN_EMPTY));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}
#[test]
fn no_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("lol");
    cmd.assert().failure().stderr(predicate::str::contains(
        "The following required arguments were not provided:\n    <PATH>",
    ));

    Ok(())
}

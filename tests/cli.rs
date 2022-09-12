use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[cfg(test)]
#[test]
fn query_not_passed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("letsplayretrogames")?;
    cmd.arg("search");
    cmd.assert().failure().stderr(predicate::str::contains(
        "The following required arguments were not provided:\n    <QUERY>",
    ));

    Ok(())
}

#[test]
fn query_passed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("letsplayretrogames")?;
    cmd.arg("search").arg("elite");
    cmd.assert().success().stdout(predicate::str::contains(
        "Elite                         | NES",
    ));

    Ok(())
}

#[test]
fn random_console_passed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("letsplayretrogames")?;
    cmd.arg("random").arg("nes");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("✔ Found it!"));

    Ok(())
}

#[test]
fn random_console_passed_is_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("letsplayretrogames")?;
    cmd.arg("random").arg("ness");
    cmd.assert().failure().stderr(predicate::str::contains(
        r#"error: "ness" isn't a valid value for '<CONSOLE>"#,
    ));

    Ok(())
}

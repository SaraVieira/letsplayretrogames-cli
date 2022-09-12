use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

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

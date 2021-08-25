use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_splash() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command.assert();

    assert
        .failure()
        .code(1)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command.arg("help").assert();

    assert
        .success()
        .stdout(predicate::str::contains("USAGE:\n    rmqhgen [SUBCOMMAND]"))
        .stderr(predicate::str::is_empty());

    Ok(())
}

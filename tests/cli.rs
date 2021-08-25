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

#[test]
fn test_help_generate() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command.arg("help").arg("generate").assert();

    assert
        .success()
        .stdout(predicate::str::contains("USAGE:\n    rmqhgen generate"))
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_help_validate() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command.arg("help").arg("validate").assert();

    assert
        .success()
        .stdout(predicate::str::contains("USAGE:\n    rmqhgen validate"))
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_generate_bare() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command.arg("generate").arg("some clear password").assert();

    assert
        .success()
        .stdout(predicate::str::is_match("^.{48}\n$").unwrap())
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_generate_sha512_short_algo() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command
        .arg("generate")
        .arg("-a")
        .arg("sha512")
        .arg("some clear password")
        .assert();

    assert
        .success()
        .stdout(predicate::str::is_match("^.{92}\n$").unwrap())
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_generate_sha256_long_algo() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command
        .arg("generate")
        .arg("--algorithm")
        .arg("sha256")
        .arg("some clear password")
        .assert();

    assert
        .success()
        .stdout(predicate::str::is_match("^.{48}\n$").unwrap())
        .stderr(predicate::str::is_empty());

    Ok(())
}

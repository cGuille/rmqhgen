use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_splash() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command.assert();

    assert
        .failure()
        .code(2)
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
        .stdout(predicate::str::is_match("USAGE:\n    rmqhgen(\\.exe)? <SUBCOMMAND>").unwrap())
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_help_generate() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command.arg("help").arg("generate").assert();

    assert
        .success()
        .stdout(predicate::str::is_match("USAGE:\n    rmqhgen(\\.exe)? generate").unwrap())
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_help_validate() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command.arg("help").arg("validate").assert();

    assert
        .success()
        .stdout(predicate::str::is_match("USAGE:\n    rmqhgen(\\.exe)? validate").unwrap())
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

#[test]
fn test_validate_bare_ok() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command
        .arg("validate")
        .arg("xBAOVxyv8NQD4Y2Z1j1NPAve31P1ed5j++WJoBwuwtnwlC0K")
        .arg("some clear password")
        .assert();

    assert
        .success()
        .stdout(predicate::eq("OK\n"))
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_validate_bare_ko() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command
        .arg("validate")
        .arg("xBAOVxyv8NQD4Y2Z1j1NPAve31P1ed5j++WJoBwuwtnwlC0K")
        .arg("some other password")
        .assert();

    assert
        .code(1)
        .stdout(predicate::eq("Invalid password\n"))
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_validate_quiet_ok() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command
        .arg("validate")
        .arg("--quiet")
        .arg("xBAOVxyv8NQD4Y2Z1j1NPAve31P1ed5j++WJoBwuwtnwlC0K")
        .arg("some clear password")
        .assert();

    assert
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_validate_quiet_ko() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command
        .arg("validate")
        .arg("--quiet")
        .arg("xBAOVxyv8NQD4Y2Z1j1NPAve31P1ed5j++WJoBwuwtnwlC0K")
        .arg("some other password")
        .assert();

    assert
        .code(1)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_validate_quiet_md5_short_algo() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command
        .arg("validate")
        .arg("--quiet")
        .arg("-a")
        .arg("md5")
        .arg("cH4ItNINimvBXIJPvL7cU5ZSxr8=")
        .arg("password")
        .assert();

    assert
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_validate_quiet_sha256_short_algo() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command
        .arg("validate")
        .arg("--quiet")
        .arg("-a")
        .arg("sha256")
        .arg("xBAOVxyv8NQD4Y2Z1j1NPAve31P1ed5j++WJoBwuwtnwlC0K")
        .arg("some clear password")
        .assert();

    assert
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn test_validate_quiet_sha512_long_algo() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("rmqhgen").unwrap();

    let assert = command
        .arg("validate")
        .arg("--quiet")
        .arg("--algorithm")
        .arg("sha512")
        .arg("BpezesNT+szOgEDqsQiJU0QQwICMoviTGpNAF0OfQMqarixjDobLAQONuGownXGfkJ+mn0vsbtu5J9k7FTU/pu60flw=")
        .arg("some clear password")
        .assert();

    assert
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    Ok(())
}

extern crate base64;
extern crate clap;
extern crate crypto_hash;

use clap::{App, AppSettings, Arg, SubCommand};
use crypto_hash::{digest, Algorithm};

fn main() {
    let app_matches = App::new("RabbitMQ password hash generator")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.0")
        .about("Generate and validate RabbitMQ compliant password hashes")
        .subcommand(
            SubCommand::with_name("validate")
                .about("Validate a given password against a known hash")
                .arg(
                    Arg::with_name("quiet")
                        .help("Do not print anything; only the exit code will tell whether the validation passed or not")
                        .short("q")
                        .long("quiet")
                )
                .arg(
                    Arg::with_name("hash")
                        .help("The known hash to test the password against")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("password")
                        .help("The clear password to validate")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate a RabbitMQ compliant hash for the given password")
                .arg(
                    Arg::with_name("password")
                        .help("The clear password to generate a hash for")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(validate_matches) = app_matches.subcommand_matches("validate") {
        let quiet = validate_matches.is_present("quiet");
        let hash = validate_matches.value_of("hash").unwrap();
        let password = validate_matches.value_of("password").unwrap();

        if validate(hash, password) {
            if !quiet {
                println!("OK");
            }

            ::std::process::exit(0);
        }

        if !quiet {
            println!("Invalid password");
        }

        ::std::process::exit(1);
    }

    if let Some(generate_matches) = app_matches.subcommand_matches("generate") {
        // TODO
        println!(
            "PASSWORD: {}",
            generate_matches.value_of("password").unwrap()
        );
    }
}

fn validate(hash: &str, password: &str) -> bool {
    let decoded_hash =
        base64::decode(hash.as_bytes()).expect("Invalid hash: could not decode base64");
    let salt = decoded_hash
        .get(0..4)
        .expect("Invalid hash: could not extract salt");

    let expected_hash = generate_with_salt(salt, password);

    hash == &expected_hash
}

fn generate_with_salt(salt: &[u8], password: &str) -> String {
    let mut salted_pass = salt.to_vec();
    salted_pass.append(&mut password.as_bytes().to_vec());

    let mut hash = digest(Algorithm::SHA256, &salted_pass);

    let mut salted_hash = salt.to_vec();
    salted_hash.append(&mut hash);

    base64::encode(&salted_hash)
}
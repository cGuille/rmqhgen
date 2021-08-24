extern crate base64;
extern crate clap;
extern crate crypto_hash;
extern crate rand;

use clap::{App, AppSettings, Arg, SubCommand};
use crypto::util::fixed_time_eq;
use crypto_hash::{digest, Algorithm};
use rand::{thread_rng, Rng};

fn main() {
    let app_matches = App::new("RabbitMQ password hash generator")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.2.3")
        .about("Generates and validates RabbitMQ compliant password hashes")
        .subcommand(
            SubCommand::with_name("validate")
                .about("Validates a given password against a known hash")
                .arg(
                    Arg::with_name("algorithm")
                        .short("a")
                        .long("algorithm")
                        .help("The algorithm used to validate the given hash")
                        .takes_value(true)
                        .default_value("sha256")
                        .possible_values(&["sha256", "sha512", "md5"])
                        .required(true)
                )
                .arg(
                    Arg::with_name("quiet")
                        .help("Does not print anything; only the exit code will tell whether the validation passed or not")
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
                .about("Generates a RabbitMQ compliant hash for the given password")
                .arg(
                    Arg::with_name("algorithm")
                        .short("a")
                        .long("algorithm")
                        .help("The algorithm used to generate the hash")
                        .takes_value(true)
                        .default_value("sha256")
                        .possible_values(&["sha256", "sha512"])
                        .required(true)
                )
                .arg(
                    Arg::with_name("password")
                        .help("The clear password to generate a hash for")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(validate_matches) = app_matches.subcommand_matches("validate") {
        let algorithm = parse_algo(validate_matches.value_of("algorithm").unwrap());
        let quiet = validate_matches.is_present("quiet");
        let hash = validate_matches.value_of("hash").unwrap();
        let password = validate_matches.value_of("password").unwrap();

        if validate(algorithm, hash, password) {
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
        let algorithm = parse_algo(generate_matches.value_of("algorithm").unwrap());
        let password = generate_matches.value_of("password").unwrap();

        println!("{}", generate(algorithm, password));
    }
}

fn validate(algorithm: Algorithm, hash: &str, password: &str) -> bool {
    let decoded_hash =
        base64::decode(hash.as_bytes()).expect("Invalid hash: could not decode base64");
    let salt = decoded_hash
        .get(0..4)
        .expect("Invalid hash: could not extract salt");

    let expected_hash = generate_with_salt(algorithm, salt, password);

    fixed_time_eq(hash.as_bytes(), expected_hash.as_bytes())
}

fn generate_with_salt(algorithm: Algorithm, salt: &[u8], password: &str) -> String {
    let mut salted_pass = salt.to_vec();
    salted_pass.append(&mut password.as_bytes().to_vec());

    let mut hash = digest(algorithm, &salted_pass);

    let mut salted_hash = salt.to_vec();
    salted_hash.append(&mut hash);

    base64::encode(&salted_hash)
}

fn generate(algorithm: Algorithm, password: &str) -> String {
    let mut salt = [0u8; 4];
    thread_rng().fill(&mut salt[..]);

    generate_with_salt(algorithm, &salt, password)
}

fn parse_algo(algo_name: &str) -> Algorithm {
    match algo_name {
        "sha256" => Algorithm::SHA256,
        "sha512" => Algorithm::SHA512,
        "md5" => Algorithm::MD5,
        _ => panic!("Unexpected algorithm {}", algo_name),
    }
}

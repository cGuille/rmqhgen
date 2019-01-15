extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

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
        // TODO
        println!(
            "QUIET: {}",
            if validate_matches.is_present("quiet") {
                "yes"
            } else {
                "no"
            }
        );
        println!("KNOWN HASH: {}", validate_matches.value_of("hash").unwrap());
        println!(
            "PASSWORD: {}",
            validate_matches.value_of("password").unwrap()
        );
    }

    if let Some(generate_matches) = app_matches.subcommand_matches("generate") {
        // TODO
        println!(
            "PASSWORD: {}",
            generate_matches.value_of("password").unwrap()
        );
    }
}

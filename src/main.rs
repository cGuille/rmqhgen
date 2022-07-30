use clap::ArgEnum;
use clap::Parser;
use clap::Subcommand;
use digest::DynDigest;
use md5::Md5;
use rand::{thread_rng, Rng};
use sha2::Sha256;
use sha2::Sha512;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate a RabbitMQ compliant hash for the given password
    Generate {
        #[clap(short, long, default_value = "sha256", arg_enum)]
        #[clap(help = "The algorithm used to validate the given hash")]
        algorithm: GenerateAlgo,

        #[clap(help = "The clear password to generate a hash for")]
        password: String,
    },

    /// Validate a given password against a known hash
    Validate {
        #[clap(short, long, default_value = "sha256", arg_enum)]
        #[clap(help = "The algorithm used to validate the given hash")]
        algorithm: ValidateAlgo,

        #[clap(short, long, action)]
        #[clap(
            help = "Does not print anything; only the exit code will tell whether the validation passed or not"
        )]
        quiet: bool,

        #[clap(help = "The known hash to test the password against")]
        hash: String,

        #[clap(help = "The clear password to validate")]
        password: String,
    },
}

#[derive(Debug, Clone, ArgEnum)]
enum GenerateAlgo {
    Sha512,
    Sha256,
}

impl From<GenerateAlgo> for Box<dyn DynDigest> {
    fn from(algo: GenerateAlgo) -> Self {
        match algo {
            GenerateAlgo::Sha512 => Box::new(Sha512::default()),
            GenerateAlgo::Sha256 => Box::new(Sha256::default()),
        }
    }
}

#[derive(Debug, Clone, ArgEnum)]
enum ValidateAlgo {
    Sha512,
    Sha256,
    Md5,
}

impl From<ValidateAlgo> for Box<dyn DynDigest> {
    fn from(algo: ValidateAlgo) -> Self {
        match algo {
            ValidateAlgo::Sha512 => Box::new(Sha512::default()),
            ValidateAlgo::Sha256 => Box::new(Sha256::default()),
            ValidateAlgo::Md5 => Box::new(Md5::default()),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate {
            algorithm,
            quiet,
            hash,
            password,
        } => {
            if validate(algorithm.into(), &hash, &password) {
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

        Commands::Generate {
            algorithm,
            password,
        } => {
            println!("{}", generate(algorithm.into(), &password));
        }
    }
}

fn validate(digest: Box<dyn DynDigest>, hash: &str, password: &str) -> bool {
    let decoded_hash =
        base64::decode(hash.as_bytes()).expect("Invalid hash: could not decode base64");
    let salt = decoded_hash
        .get(0..4)
        .expect("Invalid hash: could not extract salt");

    let expected_hash = generate_with_salt(digest, salt, password);

    hash.as_bytes() == expected_hash.as_bytes()
}

fn generate_with_salt(mut digest: Box<dyn DynDigest>, salt: &[u8], password: &str) -> String {
    let mut salted_pass = salt.to_vec();
    salted_pass.append(&mut password.as_bytes().to_vec());

    digest.update(&salted_pass);
    let mut hash = digest.finalize().into();

    let mut salted_hash = salt.to_vec();
    salted_hash.append(&mut hash);

    base64::encode(&salted_hash)
}

fn generate(algorithm: Box<dyn DynDigest>, password: &str) -> String {
    let mut salt = [0u8; 4];
    thread_rng().fill(&mut salt[..]);

    generate_with_salt(algorithm, &salt, password)
}

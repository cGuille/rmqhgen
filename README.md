# rmqhgen

Generate and validate RabbitMQ password hashes, based on [the algorithm described in the RabbitMQ documentation](https://www.rabbitmq.com/passwords.html#computing-password-hash).

## Installation

- Go to the [releases page](https://github.com/cGuille/rmqhgen/releases).
- Download the archive of the appropriate version for your system.
- Extract the binary file from the archive.
- Move the binary file into your PATH.

## Usage

Run the command to display usage details. TL;DR:

```bash
rmqhgen generate <password>
rmqhgen validate [--quiet] <hash> <password>
```

## Testing

```bash
cargo test
```

## Building from sources

The program is written in Rust, so you will need the [Rust language toolchain](https://www.rust-lang.org/tools/install).

Once the Rust toolchain installed, the following command will build the executable:

```bash
cargo build --release
```

You can then either directly use the file at `target/release/rmqhgen`, or move it into your `PATH`.

## Create a new release

### Update the application's version:

1. Update the package version in `Cargo.toml`.
2. Run the tests (`cargo test`); this will update the version in `Cargo.lock`.
3. Commit the changes.
4. Push the commit.

### Create the version's tag

```bash
git tag "v$(cargo read-manifest | jq --raw-output .version)"
git push --tags
```

This will create a release on GitHub, and trigger the release GitHub action which will build the app and upload the
artifacts to the GitHub release.

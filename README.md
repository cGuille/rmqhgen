# rmqhgen

Generate and validate RabbitMQ password hashes, based on [the algorithm described in the RabbitMQ documentation](https://www.rabbitmq.com/passwords.html#computing-password-hash).

## Installation

- Go to the [releases page](https://github.com/cGuille/rmqhgen/releases).
- Download the archive of the appropriate version for your system.
- Extract the binary file from the archive.
- Move the binary file into your PATH.

## Usage

Run the command to display usage details. TL;DR:

```
rmqhgen generate <password>
rmqhgen validate [--quiet] <hash> <password>
```

## Building from sources

The program is written in Rust, so you will need the [Rust language toolchain](https://www.rust-lang.org/tools/install).

Once the Rust toolchain installed, the following command will build the executable:

```bash
cargo build --release
```

You can then either directly use the file at `target/release/rmqhgen`, or move it into your `PATH`.

## Create a new release

- Update the version number in:
  - `Cargo.toml`
  - `src/main.rs`
- Commit the change.
- Create a tag for the appropriate version: `git tag -a 'vX.Y.Z'`.
- Push the tag: `git push origin 'vX.Y.Z'`

# rmqhgen

Generate and validate RabbitMQ password hashes, based on [the algorithm described in the RabbitMQ documentation](https://www.rabbitmq.com/passwords.html#computing-password-hash).

## Installation

The program is written in Rust, so you will need the [Rust language toolchain](https://www.rust-lang.org/tools/install).

Once the Rust toolchain installed, the following command will build the executable:

```bash
cargo build --release
```

You can then either directly use the file at `target/release/rmqhgen`, or move it into your `PATH`.

## Usage

Run the command to display usage details. TL;DR:

```
rmqhgen generate <password>
rmqhgen validate [--quiet] <hash> <password>
```

# Safe Client Gateway
[![Actions Status](https://github.com/gnosis/safe-client-gateway/workflows/safe-client-gateway/badge.svg?branch=main)](https://github.com/gnosis/safe-client-gateway/actions)
[![Coverage Status](https://coveralls.io/repos/github/gnosis/safe-client-gateway/badge.svg)](https://coveralls.io/github/gnosis/safe-client-gateway)

## Motivation

This project is a gateway between the Safe clients ([Android](https://github.com/gnosis/safe-android)/ [iOS](https://github.com/gnosis/safe-ios)/ [web](https://github.com/gnosis/safe-react)) and the Safe backend services ([transaction service](https://github.com/gnosis/safe-transaction-service) and Ethereum nodes). It is providing a more UI-oriented mapping and multi-sourced data structures for ease of integration and rendering.

## Documentation

- [Client Gateway Wiki](https://gnosis.github.io/safe-client-gateway/)
- [Safe developer documentation](https://docs.gnosis.io/safe/)

## Quickstart

This project requires `rustup` and `redis`

```bash
git clone https://github.com/gnosis/safe-client-gateway.git
cd safe-client-gateway
cp .env.sample .env
redis-server
cargo run
./add_rustfmt_git_hook.sh  # It installs a git precommit hook that will autoformat the code on every commit
```

After doing any change code must be formatted using [Rustfmt](https://github.com/rust-lang/rustfmt)
- `cargo fmt --all`
Auto formatting can also [be configured in the most common code editors](https://github.com/rust-lang/rustfmt#running-rustfmt-from-your-editor)

## Configuration

Rocket specific configurations (including databases) can be configured via the `Rocket.toml` for local development (see https://rocket.rs/v0.4/guide/configuration/#rockettoml).

For configurations specific to this service the `.env` file can be used. See next section.

## Environment

Place a `.env` file in the root of the project containing URL pointing to the environment in which you want the gateway to run.

The contents of the file should be the following (see `.env.sample` for an example)

## Tests

In order to run the test suite of the project:

1. Have an instance of Redis running (as some of them test the integration with Redis).

```bash
redis-server
```

2. Make sure that `REDIS_URI` is set and points to the current Redis instance (assuming Redis is runnning on the default port `6379`):

```bash
export REDIS_URI=redis://localhost:6379
```

3. Run the tests

```bash
cargo test -- --test-threads 1
```

By default, `cargo test` will execute the tests in the test suite in parallel. Because some of the tests update some shared local state (eg.: environment variables) the tests should be executed on a single thread – thus `--test-threads 1`.

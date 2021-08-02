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
rustup default nightly # (Rocket currently requires a nightly version)
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

To run all tests use the `cargo test` command. If you want to run a specific subset of tests, then add additionally any info regarding the path of the tests and `cargo` will match it.

Example: `cargo test converters` will run every tests under the `converters` module. Matching occurs also at a test name level, so by writing the full name of a test, that single test can be run.

Additionally, for cache testing, we have included a script that fills up the cache as it would happen in production. You can find the script in `./scripts/load_tester/start.py`. To run the script, use the following commands: 

```shell
python3 -m venv venv
source venv/bin/activate && pip install -r scripts/cache_warmer/requirements.txt
python scripts/cache_warmer/start.py
# once you are done testing
deactivate
```

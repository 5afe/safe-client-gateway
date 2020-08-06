# Safe Client Gateway
[![Build Status](https://travis-ci.com/gnosis/safe-client-gateway.svg?branch=main)](https://travis-ci.com/gnosis/safe-client-gateway)
## Quickstart

This project requires `rustup` and `redis`

- Clone project and go to project folder
- `rustup default nightly` (Rocket currently requires a nightly version)
- `cp .env.sample .env`
- `redis-server`
- `cargo run`

## Configuration

Rocket specific configurations (including databases) can be configured via the `Rocket.toml` for local development (see https://rocket.rs/v0.4/guide/configuration/#rockettoml).

For configurations specific to this service the `.env` file can be used. See next section.

## Environment

Place a `.env` file in the root of the project containing URL pointing to the environment in which you want the gateway to run.

The contents of the file should be the following (see `.env.sample` for an example):

```
TRANSACTION_SERVICE_URL=<Transaction service host>
``` 

(NOTE: don't include any form of quotation marks)

Useful links:
- Staging(rinkeby): https://safe-transaction.staging.gnosisdev.com/
- Production(rinkeby): https://safe-transaction.rinkeby.gnosis.io/
- Production(mainnet): https://safe-transaction.gnosis.io/

Additional NOTE: the `structs` defined in this project match those in staging. Therefore, using this in any other environment could potentially panic if the endpoint in the transaction service API is not deployed to production yet, or the data layout looks differently.  

## Tests

To run all tests use the `cargo test` command. If you want to run a specific subset of tests, then add additionally any info regarding the path of the tests and `cargo` will match it.

Example: `cargo test converters` will run every tests under the `converters` module. Matching occurs also at a test name level, so by writing the full name of a test, that single test can be run.

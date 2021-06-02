#!/bin/bash

rm -rf ./target/doc

cargo doc --no-deps \
  --workspace \
  --locked \
  --open # TODO: remove

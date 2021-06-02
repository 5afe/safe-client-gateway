#!/bin/bash

rm -rf ./target/doc

cargo doc --no-deps \
  --workspace \
  --open # TODO: remove

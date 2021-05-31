#!/bin/bash

rm -rf ./target/doc

cargo doc --no-deps \
  --workspace

xdg-open ./target/doc/safe_client_gateway/index.html # eventually this would be a deployment step

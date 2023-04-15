#!/bin/bash
export GITHUB_RUN_NUMBER=1
export DOCKERHUB_ORG=syscoin
export DOCKERHUB_PROJECT=safe-client-gateway
export TARGETPLATFORM=linux/amd64
./scripts/build_docker.sh latest

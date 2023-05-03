#!/bin/bash

# Developer friendly (faster build)

DOCKERHUB_ORG=syscoin
DOCKERHUB_PROJECT=safe-client-gateway

docker build -t $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:dev -f Dockerfile.dev .

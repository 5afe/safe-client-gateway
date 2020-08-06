#!/bin/bash

set -euo pipefail

if [ "$TRAVIS_PULL_REQUEST" = "false" ]; then
    echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --password-stdin
    if [ "$1" = "develop" -o "$1" = "main" ]; then
        # If image does not exist, don't use cache
        docker pull $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$1 && \
        docker build -t $DOCKERHUB_PROJECT -f Dockerfile . --cache-from $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$1 || \
        docker build -t $DOCKERHUB_PROJECT -f Dockerfile .
    else
        docker pull $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:staging && \
        docker build -t $DOCKERHUB_PROJECT -f Dockerfile . --cache-from $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:staging || \
        docker build -t $DOCKERHUB_PROJECT -f Dockerfile .
    fi
    docker tag $DOCKERHUB_PROJECT $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$1
    docker push $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$1
fi
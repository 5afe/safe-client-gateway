#!/bin/bash

set -euo pipefail

if [ "$TRAVIS_PULL_REQUEST" = "false" ]; then

    # Use travis build number to identify build
    export BUILD_NUMBER=$TRAVIS_BUILD_NUMBER
    # strip the first char as that should always be "v" (as tags should be in the format "vX.X.X")
    description="$(git describe --tags --always)"
    export VERSION=${description:1}

    echo "Trigger docker build and upload for version $VERSION ($BUILD_NUMBER)"

    echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --password-stdin
    if [ "$1" = "develop" -o "$1" = "main" ]; then
        # If image does not exist, don't use cache
        docker pull $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$1 && \
        docker build -t $DOCKERHUB_PROJECT -f Dockerfile --build-arg VERSION --build-arg BUILD_NUMBER . --cache-from $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$1 || \
        docker build -t $DOCKERHUB_PROJECT -f Dockerfile --build-arg VERSION --build-arg BUILD_NUMBER .
    else
        docker pull $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:staging && \
        docker build -t $DOCKERHUB_PROJECT -f Dockerfile --build-arg VERSION --build-arg BUILD_NUMBER . --cache-from $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:staging || \
        docker build -t $DOCKERHUB_PROJECT -f Dockerfile --build-arg VERSION --build-arg BUILD_NUMBER .
    fi
    docker tag $DOCKERHUB_PROJECT $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$1
    docker push $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$1
fi
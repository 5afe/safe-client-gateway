#!/bin/bash

set -euo pipefail

# Workflow run number
export BUILD_NUMBER=$GITHUB_RUN_NUMBER
# strip the first char as that should always be "v" (as tags should be in the format "vX.X.X")
description="$(git describe --tags --always)"
export VERSION=${description:1}

echo "Trigger docker build and upload for version $VERSION ($BUILD_NUMBER)"

if [ "$1" = "develop" -o "$1" = "main" -o "$1" = "spectrum" ]; then
    cache_tag="$1"
else
    cache_tag="staging"
fi

# Load cached builder image
docker pull $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$cache_tag-builder || true
# Rebuild builder image if required
docker build \
    --target builder \
    --cache-from $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$cache_tag-builder \
    -t $DOCKERHUB_PROJECT \
    -f Dockerfile \
    --build-arg VERSION --build-arg BUILD_NUMBER \
    .

# Load cached runtime image
docker pull $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$cache_tag || true
# Rebuild runtime image if required
docker build \
    --cache-from $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$cache_tag-builder \
    --cache-from $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$cache_tag \
    -t $DOCKERHUB_PROJECT \
    -f Dockerfile \
    --build-arg VERSION --build-arg BUILD_NUMBER \
    .

# Push runtime images to remote repository
docker tag $DOCKERHUB_PROJECT $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$1
docker push $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$1

# Push builder image to remote repository for next build
docker push $DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$cache_tag-builder

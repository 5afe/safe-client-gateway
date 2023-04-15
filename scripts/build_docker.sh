#!/bin/bash

set -euo pipefail

# Workflow run number
export BUILD_NUMBER=$GITHUB_RUN_NUMBER
# strip the first char as that should always be "v" (as tags should be in the format "vX.X.X")
description="$(git describe --tags --always)"
export VERSION=${description:1}

echo "Trigger docker build for version $VERSION ($BUILD_NUMBER)"

if [ "$1" = "develop" -o "$1" = "main" ]; then
    cache_tag="$1"
else
    cache_tag="staging"
fi

cached_builder_image_id="$DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$cache_tag-builder"
cached_runtime_image_id="$DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$cache_tag"
runtime_image_id="$DOCKERHUB_ORG/$DOCKERHUB_PROJECT:$1"

echo "$cached_runtime_image_id $cached_builder_image_id"

# Rebuild builder image if required
docker build \
    --target builder \
    -t $cached_builder_image_id \
    -f Dockerfile.local \
    --build-arg VERSION --build-arg BUILD_NUMBER --build-arg TARGETPLATFORM \
    .

# Rebuild runtime image if required
docker build \
    --cache-from $cached_builder_image_id \
    --cache-from $cached_runtime_image_id \
    -t $runtime_image_id \
    -f Dockerfile.local \
    --build-arg VERSION --build-arg BUILD_NUMBER --build-arg TARGETPLATFORM \
    .

# If release, set latest docker tag
case $1 in v*)
    latest_image_id="$DOCKERHUB_ORG/$DOCKERHUB_PROJECT:latest"
    docker tag $runtime_image_id $latest_image_id
esac

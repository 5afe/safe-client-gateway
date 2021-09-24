#!/usr/bin/env bash

# Get reposotory URI from argument
repository=$1

# Push Docker images to the repository
for image in gateway_nginx gateway_web
do
  docker tag $image:latest $repository-$image:latest
  docker push $repository-$image:latest
  docker tag $image:latest $repository-$image:`date +%Y-%m-%d--%H-%M-%S`
  docker push $repository-$image:`date +%Y-%m-%d--%H-%M-%S`
done


#!/usr/bin/env bash

tag="$1"
latest="$2"

repo=vincentjorgensen/rust-helloworld

docker buildx build --platform linux/amd64,linux/arm64 -t "$repo":"$tag" .
docker push "$repo":"$tag"


if [[ -n "$latest" ]]; then
  docker tag "$repo":"$tag" "$repo":latest
  docker push "$repo":latest
fi

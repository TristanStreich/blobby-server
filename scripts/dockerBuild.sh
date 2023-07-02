#!/bin/bash

IMAGE=blobby-server

# Parse command-line options
while getopts "v:" opt; do
  case $opt in
    v) VERSION="$OPTARG";;
    \?) echo "Invalid option: -$OPTARG"; exit 1;;
  esac
done

if [ -z "$VERSION" ]
then
    echo "missing required flag -v"
    exit 1
fi

TAG="$IMAGE:$VERSION"


# Check if the image exists with the specified tag
if docker images --format "{{.Repository}}:{{.Tag}}" | grep -q "$TAG"
then
  echo "Image $IMAGE:$TAG already exists."
  exit 1
fi


echo "Building $TAG"
docker build -t $TAG .


echo "Tagging Latest"
docker tag $TAG $IMAGE:latest
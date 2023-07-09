#!/bin/bash

IMAGE=blobby-server

PREV_VERSION=$(docker images | grep blobby-server | grep -v latest | sort -k2 -r -V | head -n 1 | awk '{ print $2 }')

print_prev_version() {
  echo "Previous Docker Verson: $PREV_VERSION"
}

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
    print_prev_version
    exit 1
fi

TAG="$IMAGE:$VERSION"


# Check if the image exists with the specified tag
if docker images --format "{{.Repository}}:{{.Tag}}" | grep -q "$TAG"
then
  echo "Image $IMAGE:$TAG already exists."
  print_prev_version
  exit 1
fi


echo "Building $TAG"
docker build -t $TAG .


echo "Tagging Latest"
docker tag $TAG $IMAGE:latest

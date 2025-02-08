#!/bin/bash
set -e

# Define image name and Dockerfile path.
IMAGE_NAME="gemfile_updater"
DOCKERFILE="Dockerfiles/Dockerfile.bundle2"

echo "Building the Docker image"
docker build -t ${IMAGE_NAME} -f ${DOCKERFILE} .

# Create a container from the builder image.
CONTAINER_ID=$(docker create ${IMAGE_NAME})

# Copy the Gemfile.lock from the container to the host.
echo "Copying updated Gemfile.lock from container (${CONTAINER_ID})..."
docker cp "${CONTAINER_ID}:/srv/jekyll/Gemfile.lock" ./Gemfile.lock

# Clean up the temporary container.
docker rm ${CONTAINER_ID} > /dev/null

echo "Updated Gemfile.lock has been copied to the host."
#!/bin/bash
set -e

# Make sure this script is run from the project root.

echo "Building Docker image..."
docker build --progress=plain --platform=linux/amd64 -t argmin-gravitas/latest -f Dockerfiles/Dockerfile.jekyll .
echo "Running Docker container..."
docker run --platform=linux/amd64  -p 4000:4000 argmin-gravitas/latest
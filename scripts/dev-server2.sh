#!/bin/bash
set -e

# Make sure this script is run from the project root.

echo "Building Docker image..."
docker build --platform=linux/amd64 -t argmin-gravitas/jekyll2 -f Dockerfiles/Dockerfile.jekyll2 .
echo "Running Docker container..."
docker run --platform=linux/amd64  -p 4000:4000 argmin-gravitas/jekyll2
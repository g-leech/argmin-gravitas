#!/bin/bash
set -e

echo "Building Docker image for static site generation..."
docker build --platform=linux/amd64 -t argmin-gravitas/jekyll-build -f Dockerfiles/Dockerfile.build .

echo "Generating static site..."
# Mount your local "./_site" directory to the container's /srv/jekyll/_site directory.
mkdir -p _site
docker run --platform=linux/amd64 -v "$(pwd)/_site:/srv/jekyll/_site" argmin-gravitas/jekyll-build

echo "Site built successfully into the ./_site directory."

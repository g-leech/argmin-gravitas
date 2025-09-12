#!/bin/bash
set -e

echo "Building Docker image for Jekyll development..."
sudo docker build -t argmin-gravitas/jekyll-dev -f Dockerfiles/Dockerfile.jekyll .

echo "Running Docker container with live file editing enabled..."
# The -v flag mounts the current directory (source code) into /srv/jekyll in the container.
sudo docker run -p 4000:4000 -v "$(pwd)":/srv/jekyll argmin-gravitas/jekyll-dev

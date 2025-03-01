#!/bin/bash
set -e

# Build imagemin Dockerfile
docker build --platform linux/amd64 --progress plain -t argmin-gravitas/imagemogrify -f Dockerfiles/Dockerfile.imagemogrify .

echo "Size before image minification"
du -sh img

# Run the image minification command
docker run --rm --platform linux/amd64 -v "$(pwd)/img":/srv argmin-gravitas/imagemogrify

echo "Size after image minification"
du -sh img
#!/bin/bash
set -e

echo "Building Docker image for static site generation..."
docker build --platform=linux/amd64 -t argmin-gravitas/jekyll-dev -f Dockerfiles/Dockerfile.jekyll .

echo "Generating static site..."
# Mount your local "./_site" directory to the container's /srv/jekyll/_site directory.
mkdir -p _site

# If JEKYLL_CONFIG is set, use it as the config file.
if [ -z "$JEKYLL_CONFIG" ]; then
  echo "JEKYLL_CONFIG not set. Using default config."
  JEKYLL_CONFIG="_config.yml"
else
  echo "Using JEKYLL_CONFIG=$JEKYLL_CONFIG"
fi

docker run --platform=linux/amd64 -v "$(pwd)/_site:/srv/jekyll/_site" argmin-gravitas/jekyll-dev bundle exec jekyll build --config $JEKYLL_CONFIG

echo "Site built successfully into the ./_site directory."

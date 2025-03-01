#!/bin/bash
set -e

echo "Size before HTML/CSS/JS minification:"
du -sh _site

# Run the minification command (excluding the 'files' directory)
docker run --rm --platform linux/amd64 \
  -v "$(pwd)/_site":/srv \
  -w /srv \
  tdewolff/minify sh -c 'mkdir -p /tmp/minify && minify  --exclude "files/**" -r -o /tmp/minify . && cp -a /tmp/minify/. . && rm -rf /tmp/minify'


echo "Size after HTML/CSS/JS minification:"
du -sh _site


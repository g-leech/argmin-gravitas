#!/bin/bash

# Make sure this script is run from the project root.

docker build -t argmin-gravitas -f DockerfilesDockerfile.jekyll .
docker run -p 4000:4000 -v $(pwd):/srv/jekyll argmin-gravitas/latest
#!/bin/sh
set -e

echo "Starting image compression..."

echo "Processing JPEGs..."
find . -type f \( -iname "*.jpg" -o -iname "*.jpeg" \) -print | while IFS= read -r file; do
  echo "Processing $file"
  mogrify -strip -interlace Plane -quality 85 "$file"
done

echo "Processing WEBP files..."
find . -type f -iname "*.webp" -print | while IFS= read -r file; do
  echo "Processing $file"
  mogrify -strip -quality 85 "$file"
done

echo "Processing PNGs..."
find . -type f -iname "*.png" -print | while IFS= read -r file; do
  echo "Processing $file"
  mogrify -strip -quality 85 "$file"
done

echo "Processing GIFs..."
find . -type f -iname "*.gif" -print | while IFS= read -r file; do
  echo "Processing $file"
  mogrify -strip "$file"
done

echo "Image compression complete."

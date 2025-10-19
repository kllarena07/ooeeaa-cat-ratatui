#!/bin/bash

# Check if correct number of arguments provided
if [ $# -ne 2 ]; then
    echo "Usage: $0 <output_directory> <gif_file>"
    echo "Example: $0 ./frames animation.gif"
    exit 1
fi

OUTPUT_DIR="$1"
GIF_FILE="$2"

# Check if gif file exists
if [ ! -f "$GIF_FILE" ]; then
    echo "Error: GIF file '$GIF_FILE' does not exist"
    exit 1
fi

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# Check if ImageMagick is installed
if ! command -v magick &> /dev/null; then
    echo "Error: ImageMagick is not installed. Please install it first:"
    echo "  brew install imagemagick"
    exit 1
fi

# Extract frames from GIF
echo "Extracting frames from '$GIF_FILE' to '$OUTPUT_DIR'..."

# Use ImageMagick to convert GIF to individual frames
# -coalesce reconstructs full frames from optimized GIF frames
magick "$GIF_FILE" -coalesce "$OUTPUT_DIR/frame_%d.png"

# Count the number of frames created
FRAME_COUNT=$(ls -1 "$OUTPUT_DIR"/frame_*.png 2>/dev/null | wc -l)

if [ $FRAME_COUNT -eq 0 ]; then
    echo "Error: No frames were extracted. Please check if the GIF file is valid."
    exit 1
fi

echo "Successfully extracted $FRAME_COUNT frames to '$OUTPUT_DIR'"
echo "Frame files: frame_0.png, frame_1.png, ..., frame_$((FRAME_COUNT-1)).png"

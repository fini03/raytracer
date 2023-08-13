#!/bin/bash

# List all scene files in a directory
scenes_directory="../scenes"
scene_files=$(ls $scenes_directory/*.xml)

# Loop through each scene file and render it
for scene_file in $scene_files; do
    echo "Rendering $scene_file"
    cargo run --release $scene_file
done

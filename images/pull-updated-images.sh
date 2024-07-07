#!/bin/bash

versions=("main" "0.13")
channels=("stable" "nightly")

# Loop through each version and channel, and run the podman pull command
for version in "${versions[@]}"; do
    for channel in "${channels[@]}"; do
        tag="learnbevy-${version}-${channel}"
        echo "Pulling Podman image for version ${version} and channel ${channel}..."
        podman  pull ghcr.io/liamgallagher737/${tag}:main
    done
done

echo "All pulls completed successfully."


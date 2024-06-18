#!/bin/bash

versions=("main" "0.13")
channels=("stable" "nightly")

# Loop through each version and channel, and run the podman pull command
for version in "${versions[@]}"; do
    for channel in "${channels[@]}"; do
        tag="learnbevy-${version}-${channel}"

        echo "Pulling Podman image for version ${version} and channel ${channel}..."
        podman  pull ghcr.io/liamgallagher737/${tag}:main

        echo "Tagging Podman image for version ${version} and channel ${channel}..."
        podman tag ghcr.io/liamgallagher737/${tag}:main ${tag}:latest

        echo "Removing old Podman image for version ${version} and channel ${channel}..."
        podman rmi ghcr.io/liamgallagher737/${tag}:main

        # Check if the build was successful
        if [[ $? -eq 0 ]]; then
            echo "Successfully pulled ${tag}"
        else
            echo "Failed to pull ${tag}"
            exit 1  # Stop the script if the pull fails
        fi
    done
done

echo "All pulls completed successfully."


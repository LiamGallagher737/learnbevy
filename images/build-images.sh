#!/bin/bash

versions=("main" "0.16")
channels=("stable" "nightly")

# Loop through each version and channel, and run the podman build command
for version in "${versions[@]}"; do
    for channel in "${channels[@]}"; do
        tag="learnbevy-${version}-${channel}"

        # Run the Podman build command
        echo "Building Podman image for version ${version} and channel ${channel}..."
        podman build --build-arg="version=${version}" --build-arg="channel=${channel}" --tag "ghcr.io/liamgallagher737/${tag}" .

        # Check if the build was successful
        if [[ $? -eq 0 ]]; then
            echo "Successfully built ${tag}"
        else
            echo "Failed to build ${tag}"
            exit 1  # Stop the script if the build fails
        fi
    done
done

echo "All builds completed successfully."


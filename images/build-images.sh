#!/bin/bash

versions=("main", "0.13" "0.12" "0.11" "0.10")
channels=("stable" "nightly")

# Loop through each version and channel, and run the docker build command
for version in "${versions[@]}"; do
    for channel in "${channels[@]}"; do
        tag="liamg737/comp-${version}-${channel}"

        # Run the Docker build command
        echo "Building Docker image for version ${version} and channel ${channel}..."
        docker build --build-arg="version=${version}" --build-arg="channel=${channel}" --tag "${tag}" .

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


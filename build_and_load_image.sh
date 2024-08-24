#!/bin/bash

# Validate input parameter for TAG
if [ "$#" -ne 1 ]; then
  # Set default value for TAG if not provided
  TAG=${1:-0.1}
fi


TAG=$1

# Validate TAG format (should be in the form of 'X.Y')
if [[ ! $TAG =~ ^[0-9]+\.[0-9]+$ ]]; then
  echo "Error: Invalid tag format. Must be in the form of 'X.Y' where X and Y are integers."
  exit 1
fi

# Build the Docker image
echo "Building Docker image..."
docker build -q -t "arnecdn/greeting-rust:${TAG}" . || {
  echo "Error: Docker build failed."
  exit 1
}

# Create .docker directory and save the image
mkdir -p .docker
echo "Saving Docker image to .docker/greeting-rust.tar..."
docker image save -o .docker/greeting-rust.tar "arnecdn/greeting-rust:${TAG}" || {
  echo "Error: Failed to save Docker image."
  exit 1
}

# Load the image into Minikube
echo "Loading image into Minikube..."
minikube image load .docker/greeting-rust.tar || {
  echo "Error: Failed to load image into Minikube."
  exit 1
}

echo "Process completed successfully!"
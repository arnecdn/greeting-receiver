# Greeting Rust

Greeting Rust is an educational project designed to help you learn and explore various aspects of software architecture and modern technologies using the Rust programming language. This repository provides a hands-on environment for experimenting with development tools, architectural patterns, and integration techniques.

## Purpose

The main goal of this project is to facilitate practical learning in:
- Software architecture concepts
- Rust language fundamentals and advanced features
- Containerization and isolation using Docker
- Build automation with Makefile

## Dependencies

- **Rust**: The primary language for development.
- **Docker**: For containerizing and running the application.
- **Make**: To automate build and workflow processes.
- **Minikube**: For running a local Kubernetes cluster.
- **kubectl**: For controlling Kubernetes clusters.

## Building and Deploying to Minikube

This project includes a Makefile that automates the process of building the Docker image, loading it into Minikube, and deploying the application using a local Kubernetes manifest.

### Prerequisites

- Docker
- Minikube
- kubectl
- Make

### Steps

1. **Start Minikube**

   ```sh
   minikube start
   ```

2. **Build and Deploy**

   Use the Makefile to build the Docker image and deploy the Kubernetes manifest:

   ```sh
   make deploy
   ```

   This will:
   - Build the Rust application Docker image
   - Load the image into Minikube’s Docker environment
   - Apply the Kubernetes manifest to your local Minikube cluster

3. **Access the Application**

   To access the deployed service, run:

   ```sh
   minikube service <service-name>
   ```

   Replace `<service-name>` with the actual service name defined in your Kubernetes manifest.

---

This project is a work in progress and open for experimentation and learning.

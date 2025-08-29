# Greeting Distributor

Greeting Distributor is an application with a REST API for receiving greetings.
It is part of a larger system that includes components for ingesting and querying greetings.

## Purpose
It is an educational project designed to help learn and explore various aspects of software architecture and modern technologies using the Rust programming language.
This repository provides a hands-on environment for experimenting with development tools, architectural patterns, and integration techniques.
It is part of a pipeline of treating greeting messages, which includes:

- [greeting-reveiver] 
- [greeting-processor]
- [greeting-api]

- 
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
- **Kafka**: As a messaging system for the application.
- **KEDA**: For event-driven scaling of Kubernetes workloads.


## Usage
The application is a Rust program that provides a simple greeting service. 
It can be built and run locally or within a Minikube environment.
It dependends on a Kafka instance to send and receive greeting messages.


### Building the Application
To build the Rust application, use the following command:

```sh
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

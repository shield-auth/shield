# Syntax helps with consistent casing and enables BuildKit features
# syntax=docker/dockerfile:1

FROM rust:latest AS builder

# Set the working directory in the container
WORKDIR /usr/src/shield

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code and other necessary files
COPY src ./src
COPY migration ./migration
COPY entity ./entity
COPY config ./config

# Build the application
RUN cargo build --release

# Runtime stage
FROM ubuntu:22.04 AS runtime

# Install necessary dependencies
RUN apt-get update && \
    apt-get install -y \
    libpq-dev \
    curl && \
    rm -rf /var/lib/apt/lists/*

# Copy the built executable from the builder stage
COPY --from=builder /usr/src/shield/target/release/shield /usr/local/bin/shield

# Copy configuration files
COPY --from=builder /usr/src/shield/config /usr/local/bin/config

# Set the working directory
WORKDIR /usr/local/bin

# Set the environment variable for the application name
ENV CARGO_PKG_NAME=shield

# Set the startup command
CMD ["shield"]

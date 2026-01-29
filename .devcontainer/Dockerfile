# Use an official base image as a parent image
FROM rust:trixie

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive

# Install system packages
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/* \
    && apt-get -y install --no-install-recommends curl git
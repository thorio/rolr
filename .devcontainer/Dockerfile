FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

RUN apt-get update && \
	apt-get install ansible musl-tools -y && \
	rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

RUN apt-get update && \
	apt-get install ansible musl-tools cowsay -y && \
	rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

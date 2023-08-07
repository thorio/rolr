FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

RUN apt update && \
	apt install ansible musl-tools -y && \
	rm -rf /var/lib/apt/lists/*

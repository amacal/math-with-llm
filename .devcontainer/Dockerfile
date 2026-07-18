FROM mcr.microsoft.com/devcontainers/rust:bookworm
USER vscode

RUN sudo apt-get update
RUN sudo apt-get upgrade -y
RUN curl -fsSL https://claude.ai/install.sh | bash

RUN rustup update 1.97.1
RUN rustup default 1.97.1
RUN rustup component add rustfmt
RUN rustup component add clippy
RUN rustup target add x86_64-unknown-none

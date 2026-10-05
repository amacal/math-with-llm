FROM mcr.microsoft.com/devcontainers/rust:bookworm
USER vscode

RUN sudo apt-get update
RUN sudo apt-get upgrade -y
RUN GH_RELEASE=$(curl -fsSL https://api.github.com/repos/cli/cli/releases/latest) \
    && GH_DEB=$(echo "$GH_RELEASE" | grep -o 'https://[^"]*linux_amd64\.deb' | head -1) \
    && if [ -n "$GH_DEB" ]; then \
         curl -fsSL "$GH_DEB" -o /tmp/gh.deb \
         && sudo apt-get install -y /tmp/gh.deb \
         && rm -f /tmp/gh.deb; \
       else \
         GH_TAR=$(echo "$GH_RELEASE" | grep -o 'https://[^"]*linux_amd64\.tar\.gz' | head -1) \
         && curl -fsSL "$GH_TAR" -o /tmp/gh.tar.gz \
         && sudo tar -xzf /tmp/gh.tar.gz -C /usr/local --strip-components=1 \
         && rm -f /tmp/gh.tar.gz; \
       fi \
    && gh --version
RUN curl -fsSL https://claude.ai/install.sh | bash

RUN rustup update 1.97.1
RUN rustup default 1.97.1
RUN rustup component add rustfmt
RUN rustup component add clippy
RUN rustup target add x86_64-unknown-none

# Gebruik een lichte officiële Ubuntu-image
FROM ubuntu:22.04

# Voorkom interactieve prompts tijdens installatie
ENV DEBIAN_FRONTEND=noninteractive

# Update en installeer basis tools
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    pkg-config \
    libssl-dev \
    ca-certificates \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# Rust installeren via rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Voeg Rust toe aan PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Maak een standaard werkdirectory
WORKDIR /usr/src/app

# Cargo projectbestanden kopiëren (optioneel, voor build caching)
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Dependencies downloaden (dit helpt bij snellere builds)
RUN cargo fetch

# Standaard build
RUN cargo build --release || true

# Default command (bash zodat je er interactief in kan)
CMD ["/bin/bash"]

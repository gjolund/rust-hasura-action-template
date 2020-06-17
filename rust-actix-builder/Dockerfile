FROM rust:1.44.0

WORKDIR /home/rust

ENV TARGET_ENV x86_64-unknown-linux-musl
ENV PKG_CONFIG_ALLOW_CROSS=1

RUN apt-get update \
    && apt-get -y install \
        ca-certificates \
        cmake \
        musl-tools \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add ${TARGET_ENV} \
    && USER=root cargo new --bin main

COPY ./Cargo.toml ./main/

RUN cd main \
    && cargo build \
        --target ${TARGET_ENV} \
        --release \
    && rm src/*.rs
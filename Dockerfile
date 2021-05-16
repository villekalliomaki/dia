FROM rust:latest AS build
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

# Build dependencies before
# For caching
RUN USER=root cargo new dia
WORKDIR /usr/src/dia
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Build the actual application
COPY src ./src
COPY config.toml .

RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy to a new container
FROM scratch
COPY --from=build /usr/local/cargo/bin/dia .
COPY --from=build /usr/src/dia/config.toml .
USER 1000
CMD ["./dia"]
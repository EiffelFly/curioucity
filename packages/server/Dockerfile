
# Build Stage
FROM rust:1.64.0 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin server
WORKDIR /server

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/server*
RUN cargo build --release

# Serve
FROM debian:buster-slim
COPY --from=builder /server/target/release/server .
CMD ["./server"]
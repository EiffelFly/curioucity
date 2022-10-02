# Build Stage
FROM rust:1.64.0 as builder

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new --bin discord-bot
WORKDIR /discord-bot

# Copy the manifests
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src


# Build for release
# The deps file will look likes discord_bot-*
RUN rm ./target/release/deps/discord*
RUN cargo build --release

# Serve
FROM debian:buster-slim
COPY --from=builder /discord-bot/target/release/discord-bot .
USER 1000
CMD ["./discord-bot"]
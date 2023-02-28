# Build Stage
FROM rust:1.66.0 as builder

# Install protoc
ENV PROTOC_VERSION 21.9
ENV PROTOC_ZIP protoc-$PROTOC_VERSION-linux-aarch_64.zip

RUN apt update \
  && apt install -y \
  sudo \
  curl \ 
  unzip

RUN  curl -OL https://github.com/google/protobuf/releases/download/v$PROTOC_VERSION/$PROTOC_ZIP \
  && unzip -o $PROTOC_ZIP -d /usr/local bin/protoc \
  && unzip -o $PROTOC_ZIP -d /usr/local include/* \
  && rm -f $PROTOC_ZIP

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new --bin curioucity
WORKDIR /curioucity

# Copy the manifests
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/curioucity*
RUN cargo build --release

# Serve
FROM debian:buster-slim
COPY --from=builder /curioucity/target/release/curioucity .
CMD ["./curioucity"]
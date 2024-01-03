build-all:
  cargo build
  docker build .

test-all:
  cargo build
  cargo check
  cargo clippy
  cargo fmt
  cargo test

test:
  cargo nextest run

publish:
  docker buildx build --platform linux/arm/v7,linux/amd64,linux/arm64/v8 -t autumnskerritt/ares:latest --push .

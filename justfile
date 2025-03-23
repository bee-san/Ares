build-all:
  cargo build
  docker build .

test-all:
  cargo build
  cargo check
  cargo clippy
  cargo test

fix-all:
  git add .
  git commit -m 'Clippy and fmt'
  cargo clippy --fix
  cargo fmt
  cargo nextest run
  git add .
  git commit -m 'Clippy and fmt'

test:
  cargo nextest run

publish:
  docker buildx build --platform linux/arm/v7,linux/amd64,linux/arm64/v8 -t autumnskerritt/ciphey:latest --push .

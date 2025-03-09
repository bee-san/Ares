FROM rust:alpine as builder
RUN apk add --no-cache build-base

# Encourage some layer caching here rather then copying entire directory that includes docs to builder container ~CMN
WORKDIR /app/ciphey
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
COPY benches/ benches/
RUN cargo build --release

FROM alpine:3.12
COPY --from=builder /app/ciphey/target/release/ciphey /usr/local/bin/ciphey
ENTRYPOINT [ "/usr/local/bin/ciphey" ]

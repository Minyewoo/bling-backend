ARG rust_version=1.73.0
ARG alpine_version=3.18
FROM rust:${rust_version}-alpine${alpine_version} as builder
ARG arch=aarch64
RUN apk update
RUN apk add --no-cache pkgconfig openssl openssl-dev musl-dev
RUN rustup target add ${arch}-unknown-linux-musl
RUN rustup toolchain install stable-${arch}-unknown-linux-musl
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build --target ${arch}-unknown-linux-musl --release

FROM scratch
ARG arch=aarch64
COPY --from=builder /app/target/${arch}-unknown-linux-musl/release/bling-backend /
EXPOSE 8080
CMD ["/bling-backend"]
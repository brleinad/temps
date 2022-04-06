FROM liuchong/rustup:stable AS builder

ARG OPEN_WEATHER_MAP_API_KEY
ENV OPEN_WEATHER_MAP_API_KEY $OPEN_WEATHER_MAP_API_KEY

# Make a fake Rust app to keep a cached layer of compiled crates
RUN USER=root cargo new app
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
# Needs at least a main.rs file with a main function
RUN mkdir src && echo "fn main(){}" > src/main.rs
# Will build all dependent crates in release mode
RUN cargo build --release

# Copy the rest
COPY . .

# Build (install) the actual binaries
RUN cargo install --path .

# Runtime image
FROM debian:stretch

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /root/.cargo/bin/ /app/

EXPOSE 8080/tcp

# No CMD or ENTRYPOINT, see fly.toml with `cmd` override.



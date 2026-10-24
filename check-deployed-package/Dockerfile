FROM rust:1-alpine

# The logdash version to check, the newest when empty
ARG LOGDASH_VERSION

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev

# Copy the Cargo.toml and src files
COPY Cargo.toml /app/
COPY src /app/src/

RUN if [ -n "$LOGDASH_VERSION" ]; then cargo add "logdash@=$LOGDASH_VERSION"; fi

# Build the application
RUN cargo build --release

# Run the application
CMD ["./target/release/logdash-demo"]

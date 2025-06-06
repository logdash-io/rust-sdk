FROM rust:1.87-alpine

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev

# Copy the Cargo.toml and src files
COPY check-deployed-package/Cargo.toml /app/
COPY check-deployed-package/src /app/src/

# Build the application
RUN cargo build --release

# Run the application
CMD ["./target/release/logdash-demo"] 
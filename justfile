# List available commands
default:
    @just --list

# Build and run the desktop app in development mode
desktop-dev:
    cargo run -p mitmproxy-desktop

# Build the desktop app in release mode
desktop-build:
    cargo build -p mitmproxy-desktop --release

# Run the desktop app in release mode
desktop-run: desktop-build
    ./target/release/mitmproxy-desktop 
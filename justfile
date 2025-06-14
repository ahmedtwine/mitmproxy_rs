# Justfile for mitmproxy_rs Local Redirector
# This handles all build dependencies and setup for the demo

set shell := ["zsh", "-c"]

# Default recipe - show available commands
default:
    @echo "Available commands:"
    @echo "  demo      - Build and test traffic interception"
    @echo "  processes - Show running processes"
    @echo "  clean     - Clean build artifacts"

# Build and test traffic interception
demo:
    @echo "Building components..."
    cargo build --release
    cd mitmproxy-rs && maturin develop
    @echo "Running interception test..."
    python demo.py

# Show running processes with icons
processes:
    cargo build --release
    cargo run --bin process-list > processes.html
    open processes.html



# Clean all build artifacts
clean:
    cargo clean
    rm -rf target/
    rm -f processes.html

# Justfile for mitmproxy_rs Local Redirector
# This handles all build dependencies and setup for the demo

set shell := ["zsh", "-c"]

# Default recipe - show available commands
default:
    @echo "mitmproxy_rs Local Redirector"
    @echo "  just demo      - Complete end-to-end demo (builds everything, sets up permissions, tests interception)"
    @echo "  just processes - Show running processes"
    @echo "  just clean     - Clean build artifacts"

# Complete end-to-end demo setup and execution
demo:
    @echo "Setting up mitmproxy_rs complete demo..."
    @echo ""
    @echo "What this does:"
    @echo "  1. Build Rust core library (performance-critical networking)"
    @echo "  2. Build Python bindings with maturin (PyO3 wrapper)"
    @echo "  3. Install mitmproxy_rs package in current Python env"
    @echo "  4. Activate macOS Network Extension (prompts for permissions)"
    @echo "  5. Run demo.py (real traffic interception)"
    @echo ""
    @echo "Step 1: Building Rust components..."
    @echo "   - Core mitmproxy library with packet capture"
    @echo "   - Platform-specific redirectors (macOS/Windows/Linux)"
    @echo "   - Process enumeration tools"
    cargo build --release
    @echo ""
    @echo "Step 2: Building Python bindings..."
    @echo "   - maturin: Builds Rust -> Python wheel"
    @echo "   - PyO3: Enables async Rust functions in Python"
    @echo "   - Installs mitmproxy_rs package for import"
    cd mitmproxy-rs && maturin develop
    @echo ""
    @echo "Step 3: Setting up Network Extension and VPN configuration..."
    @echo "   - System Extension Status:"
    @systemextensionsctl list | grep -A1 -B1 "mitmproxy\|network_extension" || echo "     ❌ No mitmproxy system extension found"
    @echo "   - VPN Configuration Status:"
    @scutil --nc list | grep -i mitmproxy || echo "     ❌ No mitmproxy VPN configuration found - will create"
    @echo "   - Required: Both System Extension AND VPN tunnel must be active"
    @echo ""
    @echo "Triggering VPN configuration setup..."
    @echo "   - The redirector app must be launched with unix socket to create VPN tunnel"
    @echo "   - This will prompt for VPN permissions if not already granted"
    @if [ -d "/Applications/Mitmproxy Redirector.app" ]; then \
        echo "   - Launching redirector app to establish VPN tunnel..."; \
        echo "   - IMPORTANT: Click 'Allow' for any VPN permission prompts"; \
        open "x-apple.systempreferences:com.apple.LoginItems-Settings.extension" & \
        sleep 2; \
        echo "   - Creating test unix socket and launching redirector..."; \
        SOCKET_PATH="/tmp/mitmproxy-test-$$"; \
        touch "$$SOCKET_PATH"; \
        "/Applications/Mitmproxy Redirector.app/Contents/MacOS/Mitmproxy Redirector" "$$SOCKET_PATH" & \
        REDIR_PID=$$!; \
        sleep 8; \
        echo "   - Checking if VPN configuration was created..."; \
        scutil --nc list | grep -i mitmproxy && echo "     ✅ VPN configuration created!" || echo "VPN configuration not found - may need manual approval"; \
        kill $$REDIR_PID 2>/dev/null || true; \
        rm -f "$$SOCKET_PATH"; \
        echo "   - Check System Settings > VPN if prompted for permissions"; \
    else \
        echo "     ❌ Cannot setup VPN - Mitmproxy Redirector.app not found"; \
    fi
    @echo ""
    @echo "Step 4: Running traffic interception demo..."
    @echo "   - Enumerates running processes"
    @echo "   - Connects to Network Extension (requires system extension + VPN permissions)"
    @echo "   - Creates VPN tunnel for traffic interception"
    @echo "   - Intercepts curl traffic: curl -v http://httpbin.org/get"
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

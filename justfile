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

# Install npm dependencies for Tauri frontend
tauri-install:
    cd mitmproxy-desktop/src/neohtop && npm install

# Run Tauri app in development mode
tauri-dev: tauri-install
    cd mitmproxy-desktop/src/neohtop && npm run tauri dev

# Build Tauri app for production
tauri-build: tauri-install
    cd mitmproxy-desktop/src/neohtop && npm run tauri build

git-merge-to-main:
    #!/usr/bin/env bash
    set -euo pipefail
    # Get current branch name
    current_branch=$(git rev-parse --abbrev-ref HEAD)
    # Confirm with the user
    read -p "Are you sure you want to merge '$current_branch' into main and force push? (y/n): " confirm
    if [[ $confirm != [yY] ]]; then
        echo "Operation cancelled."
        exit 1
    fi
    # Switch to main and update it
    git checkout main
    git pull origin main
    # Merge the current branch into main, squashing all commits
    git merge --squash "$current_branch"
    # Commit the changes with a message referencing the original branch
    git commit -m "Merge branch '$current_branch' into main" --no-verify
    # Force push to origin main
    git push origin main --force
    sleep 7
    git branch -d "$current_branch"
    echo "Branch '$current_branch' has been merged into main and force pushed to origin."
    git pull
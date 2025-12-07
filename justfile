# tp - terminal teleprompter

# Default recipe: list available commands
default:
    @just --list

# Build in release mode
build:
    cargo build --release

# Run tests
test:
    cargo test

# Run clippy
lint:
    cargo clippy

# Build and install tp binary and man page
install: build
    #!/usr/bin/env bash
    set -euo pipefail

    # Install binary
    cargo install --path .

    # Install man page
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        mkdir -p /usr/local/share/man/man1
        cp man/tp.1 /usr/local/share/man/man1/
        echo "Installed man page to /usr/local/share/man/man1/tp.1"
    else
        # Linux
        sudo mkdir -p /usr/local/share/man/man1
        sudo cp man/tp.1 /usr/local/share/man/man1/
        sudo mandb 2>/dev/null || true
        echo "Installed man page to /usr/local/share/man/man1/tp.1"
    fi

    echo "Installation complete! Run 'tp --help' or 'man tp'"

# Uninstall tp binary and man page
uninstall:
    #!/usr/bin/env bash
    set -euo pipefail

    # Remove binary
    cargo uninstall tui_prompter 2>/dev/null || true

    # Remove man page
    if [[ "$OSTYPE" == "darwin"* ]]; then
        rm -f /usr/local/share/man/man1/tp.1
    else
        sudo rm -f /usr/local/share/man/man1/tp.1
        sudo mandb 2>/dev/null || true
    fi

    echo "Uninstalled tp"

# Install binary only (no man page, no sudo)
install-bin:
    cargo install --path .

# Clean build artifacts
clean:
    cargo clean

# Run tp with a test message
demo:
    #!/usr/bin/env bash
    cargo run -- <(echo -e "Hello from tp!\n\nThis is a demo of the terminal teleprompter.\n\nPress 'q' to quit.")

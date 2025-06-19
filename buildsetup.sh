#!/usr/bin/env bash

# Setup for a Python + Rust build environment for braveblock using uv, maturin, and rustup.
# script for MacOS and Linux
# DevHybrid 06/2025

set -e

echo ""
echo "Setting up build environment for Braveblock"
echo "-------------------------------------------"

# Detect platform
IS_MACOS=false
if [[ "$OSTYPE" == "darwin"* ]]; then
    IS_MACOS=true
fi

# Install uv via curl
if ! command -v uv &>/dev/null; then
    echo "Installing uv..."
    curl -LsSf https://astral.sh/uv/install.sh | sh
    export PATH="$HOME/.cargo/bin:$PATH"
else
    echo "uv is already installed."
fi

# Install Rust
if ! command -v cargo &>/dev/null; then
    echo "Installing Rust..."
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust is already installed."
fi

export PATH="$HOME/.cargo/bin:$PATH"

# macOS specific
if [ "$IS_MACOS" = true ]; then
    export MACOSX_DEPLOYMENT_TARGET=11.0
fi

# Create and activate virtual environment
if [ ! -d ".venv" ]; then
    echo "Creating virtual environment..."
    uv venv .venv
fi

source .venv/bin/activate

# Install maturin and build dependencies
echo "Installing Python build tools..."
uv pip install --upgrade maturin build

# Build the package
echo "Building package with maturin..."
maturin build --release

echo ""
echo "Build complete. Wheel is in: ./target/wheels/"
echo "Install with: uv pip install ./target/wheels/braveblock-*.whl"
echo ""
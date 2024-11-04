#!/bin/bash

# Exit on errors
set -e

PROJECT_NAME="memo"
INSTALL_PATH="/tmp/memo"

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check if Rust and Cargo are installed
if ! command_exists cargo; then
    echo "Error: Cargo (Rust) is not installed."
    echo "Please install Rust from https://rustup.rs/."
    exit 1
fi

# Clone the repository
echo "Downloading the project"
git clone https://github.com/pbrochar/memo.git $INSTALL_PATH
cd $INSTALL_PATH

# Build the project in release mode
echo "Building the project in release mode..."
cargo build --release

# Check if the binary was created successfully
if [[ ! -f "$INSTALL_PATH/target/release/$PROJECT_NAME" ]]; then
    echo "Error: Build failed or binary not found!"
    exit 1
fi

# Copy the binary to /usr/local/bin (requires sudo)
echo "Installing the binary to /usr/local/bin..."
sudo cp "$INSTALL_PATH/target/release/$PROJECT_NAME" /usr/local/bin/

# Set executable permissions (just in case)
sudo chmod +x /usr/local/bin/$PROJECT_NAME

# Clean up build artifacts (optional)
echo "Cleaning up build artifacts..."
cargo clean

mkdir -p ~/.memo
mv completion.sh ~/.memo/completion.sh


echo "$PROJECT_NAME has been successfully installed!"
echo "You can now run the project with the command: $PROJECT_NAME\n"

echo "Completion script copied to ~/.memo"
echo "Please add the following line to your .zshrc or .bashrc file:"
echo "source ~/.memo/completion.sh"


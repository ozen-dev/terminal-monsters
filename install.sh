#!/bin/sh

# Install the binaries
cargo install --path .

# Set up the alias for the watcher
WATCHER_PATH=$(which watcher)
echo "alias git='$WATCHER_PATH'" >> ~/.bashrc
echo "alias git='$WATCHER_PATH'" >> ~/.zshrc

# Source the updated shell configuration
source ~/.bashrc
source ~/.zshrc

echo "Installation complete. You can now run 'gitmons' to start the app and 'git' commands will be intercepted by the watcher."
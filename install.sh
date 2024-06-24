#!/bin/sh

# Install the Rust project
cargo install --path .

# Get path to the terminal-monsters-worker executable
WORKER_PATH=$(which terminal-monsters-worker)
if [ -z "$WORKER_PATH" ]; then
    echo "Error: terminal-monsters-worker not found in PATH."
    exit 1
fi

# Determine the shell configuration file to use based on the SHELL environment variable
SHELL_CONFIG="$HOME/.bashrc"
SHELL_NAME="Bash"
if [ "$(basename "$SHELL")" = "zsh" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
    SHELL_NAME="Zsh"
fi

# Print installation success message
printf "\n"
printf "\033[32m+ Terminal Monsters Inc. ---------------- +\033[0m\n"
printf "\033[32m|\033[0m\n"
printf "\033[32m|\033[0m Installation complete,\033[1m Shellora\033[0m joined your party!\n"
printf "\033[32m|\033[0m\n"
printf "\033[32m+ --------------------------------------- +\033[0m\n"

# Print manual configuration instructions
printf "\n"
printf "\033[33m+ Manual Configuration Required ----------+\033[0m\n"
printf "\033[33m|\033[0m\n"
printf "\033[33m|\033[0m - Add the Terminal Monsters function and hooks to\n"
printf "\033[33m|\033[0m   your \033[1m\033[34m$SHELL_CONFIG\033[0m\033[0m file by referring to\n"
printf "\033[33m|\033[0m   the documentation: \033[1m\033[34mhttps://github.com/enzo-rma/terminal-monsters\033[0m\033[0m\n"
printf "\033[33m|\033[0m\n"
printf "\033[33m|\033[0m - After saving, source your shell configuration\n"
printf "\033[33m|\033[0m   file by running the source command:\n"
printf "\033[33m|\033[0m   \033[1m\033[34msource $SHELL_CONFIG\033[0m\033[0m\n"
printf "\033[33m|\033[0m\n"
printf "\033[33m+-----------------------------------------+\033[0m\n"

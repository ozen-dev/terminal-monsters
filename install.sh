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
printf "\033[33m|\033[0m - Add the Terminal Monsters function, hooks and alias\n"
printf "\033[33m|\033[0m   to your \033[1m\033[34m$SHELL_CONFIG\033[0m\033[0m file by referring to\n"
printf "\033[33m|\033[0m   the documentation: \033[1m\033[34mhttps://github.com/enzo-rma/terminal-monsters\033[0m\033[0m\n"
printf "\033[33m|\033[0m\n"
printf "\033[33m|\033[0m - Save your changes and source your shell configuration\n"
printf "\033[33m|\033[0m   file by running the source command:\n"
printf "\033[33m|\033[0m   \033[1m\033[34msource $SHELL_CONFIG\033[0m\033[0m\n"
printf "\033[33m|\033[0m\n"
printf "\033[33m|\033[0m - Run \033[1m\033[34mterminal-monsters\033[0m\033[0m from any terminal window\n"
printf "\033[33m|\033[0m   to open the game UI.\n"
printf "\033[33m|\033[0m\n"
printf "\033[33m+-----------------------------------------+\033[0m\n"

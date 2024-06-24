#!/bin/sh

# Install the binaries
cargo install --path .

# Set up the watcher function
WORKER_PATH=$(which terminal-monsters-worker)
SHELL_CONFIG="$HOME/.bashrc"
if [ -n "$ZSH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
fi

# Check if the watcher function is already defined
if ! grep -q "watcher() {" "$SHELL_CONFIG"; then
    cat <<EOL >>$SHELL_CONFIG

# Terminal Monsters Inc. function to send commands to the worker.
watcher() {
    # Capture the command
    local cmd="\$@"
    # Execute the command
    eval "\$cmd"
    # Send the command to the worker
    echo "\$cmd" | $WORKER_PATH
}

# Terminal Monsters Inc. function to collect and train monsters.
preexec_invoke_exec() {
    [ -n "\$COMP_LINE" ] && return  # Do not intercept tab-completion
    local cmd
    if [ -n "\$BASH_VERSION" ]; then
        cmd="\$BASH_COMMAND"
    elif [ -n "\$ZSH_VERSION" ]; then
        cmd="\$1"
    fi
    echo "\$cmd" | $WORKER_PATH
}

# Terminal Monsters Inc. preexec function for Bash
if [[ -n "\$BASH_VERSION" ]]; then
    trap 'preexec_invoke_exec' DEBUG
fi

# Terminal Monsters Inc. preexec function for Zsh
if [[ -n "\$ZSH_VERSION" ]]; then
    autoload -Uz add-zsh-hook
    add-zsh-hook preexec preexec_invoke_exec
fi
EOL
fi

# Source the updated shell configuration
source $SHELL_CONFIG

# Notify success
printf "\033[32m+ Terminal Monsters Inc. -----------------+\033[0m\n"
printf "\033[32m|\033[0m\033[0m Installation complete,\033[0m\n"
printf "\033[32m|\033[0m\033[1m Shellora\033[0m joined your party!\033[0m\n"
printf "\033[32m+-----------------------------------------+\033[0m\n"

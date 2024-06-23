#!/bin/sh

# Install the binaries
cargo install --path .

# Set up the watcher function
WORKER_PATH=$(which worker)
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

# Terminal Monsters Inc. function to watch commands and collect monsters.
preexec_invoke_exec() {
    [ -n "$COMP_LINE" ] && return  # Do not intercept tab-completion
    local cmd="$BASH_COMMAND"
    echo "\$cmd" | $WORKER_PATH
}

# Set up the preexec function for Bash
if [[ -n "\$BASH_VERSION" ]]; then
    trap 'preexec_invoke_exec' DEBUG
fi

# Set up the preexec function for Zsh
if [[ -n "\$ZSH_VERSION" ]]; then
    autoload -Uz add-zsh-hook
    add-zsh-hook preexec preexec_invoke_exec
fi
EOL
fi

# Source the updated shell configuration
source $SHELL_CONFIG

echo "Installation complete."

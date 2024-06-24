# Terminal Monsters

Collect, train and battle monsters from your terminal!

<img width="1440" alt="terminal-monsters-preview" src="https://github.com/enzo-rma/terminal-monsters/assets/127135864/6045ccbe-1a10-43d1-b3f4-a89160f1c4e0">

## Install

1. Install Rust on your machine
   You can follow the ([instruction](https://doc.rust-lang.org/book/ch01-01-installation.html)) here.

2. Clone the repository

   ```shell
   git clone https://github.com/enzo-rma/terminal-monsters.git
   ```

3. Install the binary

```shell
   cd path/to/project/directory
   sh install.sh
```

Add this code in your .bashrc file or .zshrc file depending on your shell:

If you use bash:

```bash
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
```

If you use zsh:

```bash
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

# Terminal Monsters Inc. preexec function for Zsh
if [[ -n "\$ZSH_VERSION" ]]; then
    autoload -Uz add-zsh-hook
    add-zsh-hook preexec preexec_invoke_exec
fi
```

4. Locate and execute the terminal-monsters-app binary

The binary should be located in the .cargo directory at the following path: {your_home_directory}/.cargo/bin/terminal-monsters-app

5. Collect all the Terminal Monsters!

At this point, the game should be installed correctly. You can now go back to your programming routine and Terminal Monsters might join your party if you run the commands that summon them. Have fun and go catch'em all! 😋

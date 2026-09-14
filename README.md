# Hex Shell

A simple Unix shell built in Rust as a learning project to understand systems programming, process management, and Unix internals.

## Features

- **Interactive Line Editing & History**: Arrow-key navigation, multi-line editing, and persistent `.hex_history` (via `rustyline`).
- **Autosuggestions & History Hinter**: Real-time faint gray history suggestions as you type (press `→` Right Arrow to accept).
- **Real-Time Syntax Highlighting**: Command names dynamically highlighted in bold green.
- **Tab Autocompletion**: Native path and filename autocompletion.
- **Git Branch Aware Prompt**: Displays the active Git branch dynamically in the prompt (`git: <branch>`).
- **Multi-Line Bracket Validation**: Automatically handles multi-line inputs for unclosed brackets and syntax.
- **Signal & Process Handling**: `Ctrl+C` cancels current line without terminating the shell, `Ctrl+D` exits cleanly.
- **Core Built-ins**: `cd` (with `$HOME` fallback), `pwd`, `exit`.
- **External Command Execution**: Spawns and waits on system binaries via Unix `$PATH` lookup (`ls`, `git`, `cat`, etc.).
- **Piping (`|`)**: Inter-process communication linking `stdout` of one command to `stdin` of the next via Linux kernel pipe buffers.
- **Command Chaining (`&&`)**: Short-circuit execution inspecting process exit codes (`0` vs non-zero).
- **Execution Benchmark Timer**: Monotonic clock timing (`std::time::Instant`) for commands taking >= 100ms (`[took 2.00s]`).
- **Modern Colored Interface**: ANSI styled diagnostics and dynamic path rendering (via `colored`).

## How to Run

```bash
cargo run
```

## Example

```text
/home/hex(git: main) >> pwd
/home/hex

/home/knight/hex(git: main) >> ls | grep Cargo
Cargo.lock
Cargo.toml

/home/hex(git: main) >> echo "Step 1" && echo "Step 2"
Step 1
Step 2

/home/hex(git: main) >> sleep 2
[took 2.00s]

/home/hex(git: main) >> cd src
/home/hex/src(git: main) >> pwd
/home/hex/src

/home/hex/src(git: main) >> cd
/home >> exit
Bye-Bye, See you soon!!
```

# Hex Shell

A simple Unix shell built in Rust as a learning project to understand systems programming, process management, and Unix internals.

## Features

- Interactive line editing and command history (via `rustyline`)
- Signal handling (`Ctrl+C` cancels line, `Ctrl+D` exits)
- Built-ins: `cd`, `pwd`, `exit`
- External command execution via `$PATH` (`ls`, `git`, `cat`, etc.)
- Basic piping (`cmd1 | cmd2`)
- Command chaining with short-circuiting (`cmd1 && cmd2`)
- Execution timer for commands taking >= 100ms (`[took 2.00s]`)
- Colored prompt showing current working directory and colored error messages (via `colored`)

## How to Run

```bash
cargo run
```

## Example

```text
/home/hex >> pwd
/home/hex

/home/hex >> ls | grep Cargo
Cargo.lock
Cargo.toml

/home/hex >> echo "Step 1" && echo "Step 2"
Step 1
Step 2

/home/hex >> sleep 2
[took 2.00s]

/home/hex >> cd src
/home/hex/src >> pwd
/home/hex/src

/home/hex/src >> exit
Bye-Bye, See you soon!!
```

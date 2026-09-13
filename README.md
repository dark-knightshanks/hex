# Hex Shell

A simple Unix shell built in Rust as a learning project to understand systems programming, process management, and Unix internals.

## Features

- Interactive line editing and command history (via `rustyline`)
- Signal handling (`Ctrl+C` cancels line, `Ctrl+D` exits)
- Built-ins: `cd`, `pwd`, `exit`
- External command execution via `$PATH` (`ls`, `git`, `cat`, etc.)
- Basic piping (`cmd1 | cmd2`)
- Colored prompt and error messages (via `colored`)

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

/home/hex >> cd src
/home/hex/src >> pwd
/home/hex/src

/home/hex/src >> exit
Bye-Bye, See you soon!!
```

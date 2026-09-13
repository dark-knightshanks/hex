use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::process::{Command, Stdio};

fn execute_piped_command(input: &str) {
    let commands: Vec<&str> = input.split('|').collect();
    if commands.len() != 2 {
        println!("Error");
        return;
    }
    let left: Vec<&str> = commands[0].split_whitespace().collect();
    let right: Vec<&str> = commands[1].split_whitespace().collect();
    if left.is_empty() || right.is_empty() {
        println!("Error: invalid");
        return;
    }
    let mut left_child = match Command::new(left[0])
        .args(&left[1..])
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            println!("Error: Failed to execute {}: {}", left[0], e);
            return;
        }
    };
    if let Some(left_stdout) = left_child.stdout.take() {
        let mut right_child = match Command::new(right[0])
            .args(&right[1..])
            .stdin(left_stdout)
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                println!("Error: Failed to execute {}: {}", left[0], e);
                let _ = left_child.wait();
                return;
            }
        };
        let _ = right_child.wait();
    }
    let _ = left_child.wait();
    return;
}

fn main() {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline(">>");
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                let _ = rl.add_history_entry(trimmed);
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed.contains('|') {
                    execute_piped_command(trimmed);
                    continue;
                }
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                let command = parts[0];
                let args = &parts[1..];
                match command {
                    "exit" => {
                        println!("Bye-Bye, See you soon!!");
                        break;
                    }
                    "pwd" => match std::env::current_dir() {
                        Ok(path) => println!("path: {}", path.display()),
                        Err(e) => eprintln!("pwd error: {}", e),
                    },
                    "cd" => {
                        let target = match args.get(0) {
                            Some(path_arg) => path_arg.to_string(),
                            None => match std::env::var("HOME") {
                                Ok(home) => home,
                                Err(_) => {
                                    eprintln!("cd: HOME directory not set");
                                    continue;
                                }
                            },
                        };

                        if let Err(e) = std::env::set_current_dir(&target) {
                            eprintln!("cd error: {}", e);
                        }
                    }
                    _ => match std::process::Command::new(command).args(args).spawn() {
                        Ok(mut child) => {
                            if let Err(e) = child.wait() {
                                eprintln!("Error waiting for command: {}", e);
                            }
                        }
                        Err(_) => {
                            eprintln!("hex: command not found: {}", command);
                        }
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

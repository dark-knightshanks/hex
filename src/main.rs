use colored::*;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::process::{Command, Stdio};

fn execute_piped_command(input: &str) {
    let commands: Vec<&str> = input.split('|').collect();
    if commands.len() != 2 {
        eprintln!("{}: only single pipe '|' is supported", "pipe error".red().bold());
        return;
    }
    let left: Vec<&str> = commands[0].split_whitespace().collect();
    let right: Vec<&str> = commands[1].split_whitespace().collect();
    if left.is_empty() || right.is_empty() {
        eprintln!("{}: invalid pipe syntax", "pipe error".red().bold());
        return;
    }
    let mut left_child = match Command::new(left[0])
        .args(&left[1..])
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            eprintln!("{}: failed to start '{}': {}", "pipe error".red().bold(), left[0], e);
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
                eprintln!("{}: failed to start '{}': {}", "pipe error".red().bold(), right[0], e);
                let _ = left_child.wait();
                return;
            }
        };
        let _ = right_child.wait();
    }
    let _ = left_child.wait();
}

fn main() {
    let mut rl = DefaultEditor::new().unwrap();

    let history_file = ".hex_history";
    let _ = rl.load_history(history_file);

    println!("{}", "Welcome to Hex".bright_green().bold());
    println!("{}", "Type 'exit' or press Ctrl+D to quit.\n".dimmed());

    loop {
        let current_dir = std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "?".to_string());
        let prompt = format!("{}{} ", current_dir.cyan().bold(), " >>".bright_yellow().bold());

        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(trimmed);
                if trimmed.contains('|') {
                    execute_piped_command(trimmed);
                    continue;
                }
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                let command = parts[0];
                let args = &parts[1..];
                match command {
                    "exit" => {
                        println!("{}", "Bye-Bye, See you soon!!".bright_blue().bold());
                        break;
                    }
                    "pwd" => match std::env::current_dir() {
                        Ok(path) => println!("{}", path.display()),
                        Err(e) => eprintln!("{}: {}", "pwd error".red().bold(), e),
                    },
                    "cd" => {
                        let target = match args.get(0) {
                            Some(path_arg) => path_arg.to_string(),
                            None => match std::env::var("HOME") {
                                Ok(home) => home,
                                Err(_) => {
                                    eprintln!("{}: HOME directory not set", "cd error".red().bold());
                                    continue;
                                }
                            },
                        };

                        if let Err(e) = std::env::set_current_dir(&target) {
                            eprintln!("{}: {}: {}", "cd error".red().bold(), target, e);
                        }
                    }
                    _ => match std::process::Command::new(command).args(args).spawn() {
                        Ok(mut child) => {
                            if let Err(e) = child.wait() {
                                eprintln!("{}: {}", "error waiting for command".red(), e);
                            }
                        }
                        Err(_) => {
                            eprintln!("{}: command not found: {}", "hex".red().bold(), command.bright_white());
                        }
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "^C".yellow());
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "exit".dimmed());
                break;
            }
            Err(err) => {
                eprintln!("{}: {:?}", "Error".red().bold(), err);
                break;
            }
        }
    }

    let _ = rl.save_history(history_file);
}

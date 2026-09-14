use colored::*;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::process::{Command, Stdio};

enum ExecutionResult {
    Success,
    Failure,
    Exit,
}

fn execute_single_command(input: &str) -> ExecutionResult {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return ExecutionResult::Success;
    }
    if trimmed.contains('|') {
        return execute_piped_command(trimmed);
    }
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let command = parts[0];
    let args = &parts[1..];
    match command {
        "exit" => {
            println!("{}", "Bye-Bye, See you soon!!".bright_blue().bold());
            ExecutionResult::Exit
        }
        "pwd" => match std::env::current_dir() {
            Ok(path) => {
                println!("{}", path.display());
                ExecutionResult::Success
            }
            Err(e) => {
                eprintln!("{}: {}", "pwd error".red().bold(), e);
                ExecutionResult::Failure
            }
        },
        "cd" => {
            let target = match args.get(0) {
                Some(path_arg) => path_arg.to_string(),
                None => match std::env::var("HOME") {
                    Ok(home) => home,
                    Err(_) => {
                        eprintln!("{}: HOME directory not set", "cd error".red().bold());
                        return ExecutionResult::Failure;
                    }
                },
            };

            match std::env::set_current_dir(&target) {
                Ok(_) => ExecutionResult::Success,
                Err(e) => {
                    eprintln!("{}: {}: {}", "cd error".red().bold(), target, e);
                    ExecutionResult::Failure
                }
            }
        }
        _ => match std::process::Command::new(command).args(args).spawn() {
            Ok(mut child) => match child.wait() {
                Ok(status) => {
                    if status.success() {
                        ExecutionResult::Success
                    } else {
                        ExecutionResult::Failure
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "error waiting for command".red(), e);
                    ExecutionResult::Failure
                }
            },
            Err(_) => {
                eprintln!(
                    "{}: command not found: {}",
                    "hex".red().bold(),
                    command.bright_white()
                );
                ExecutionResult::Failure
            }
        },
    }
}

fn execute_piped_command(input: &str) -> ExecutionResult {
    let commands: Vec<&str> = input.split('|').collect();
    if commands.len() != 2 {
        eprintln!(
            "{}: only single pipe '|' is supported",
            "pipe error".red().bold()
        );
        return ExecutionResult::Failure;
    }
    let left: Vec<&str> = commands[0].split_whitespace().collect();
    let right: Vec<&str> = commands[1].split_whitespace().collect();
    if left.is_empty() || right.is_empty() {
        eprintln!("{}: invalid pipe syntax", "pipe error".red().bold());
        return ExecutionResult::Failure;
    }
    let mut left_child = match Command::new(left[0])
        .args(&left[1..])
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            eprintln!(
                "{}: failed to start '{}': {}",
                "pipe error".red().bold(),
                left[0],
                e
            );
            return ExecutionResult::Failure;
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
                eprintln!(
                    "{}: failed to start '{}': {}",
                    "pipe error".red().bold(),
                    right[0],
                    e
                );
                let _ = left_child.wait();
                return ExecutionResult::Failure;
            }
        };
        let status = match right_child.wait() {
            Ok(s) => {
                if s.success() {
                    ExecutionResult::Success
                } else {
                    ExecutionResult::Failure
                }
            }
            Err(_) => ExecutionResult::Failure,
        };
        let _ = left_child.wait();
        return status;
    }
    let _ = left_child.wait();
    ExecutionResult::Failure
}

fn main() {
    let mut rl = DefaultEditor::new().unwrap();

    let history_file = ".hex_history";
    let _ = rl.load_history(history_file);

    println!("{}", "Welcome to Hex".bright_green().bold());
    println!("{}", "Type 'exit' or press Ctrl+D to quit.\n".dimmed());

    'shell_loop: loop {
        let current_dir = std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "?".to_string());
        let prompt = format!(
            "{}{} ",
            current_dir.cyan().bold(),
            " >>".bright_yellow().bold()
        );

        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(trimmed);
                let cmd: Vec<&str> = trimmed.split("&&").collect();
                let start_time = std::time::Instant::now();
                for sub_cmd in cmd {
                    match execute_single_command(sub_cmd) {
                        ExecutionResult::Exit => break 'shell_loop,
                        ExecutionResult::Failure => break,
                        ExecutionResult::Success => (),
                    }
                }
                let duration = start_time.elapsed();
                if duration.as_millis() >= 100 {
                    let formatted_time = if duration.as_secs_f64() >= 1.0 {
                        format!("{:.2}s", duration.as_secs_f64())
                    } else {
                        format!("{}ms", duration.as_millis())
                    };
                    println!("{}", format!("[took {}]", formatted_time).dimmed());
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "^C".yellow());
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "exit".dimmed());
                break 'shell_loop;
            }
            Err(err) => {
                eprintln!("{}: {:?}", "Error".red().bold(), err);
                break;
            }
        }

        let _ = rl.save_history(history_file);
    }
}

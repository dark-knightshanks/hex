use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() {
    let mut rl = DefaultEditor::new().unwrap(); 
    
    loop{
            let readline = rl.readline(">>");
            match readline {
            Ok(line) => {
                let trimmed = line.trim();
                let _ = rl.add_history_entry(trimmed);
                if trimmed.is_empty(){
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
                        "pwd" => {                                                                                                              
                            match std::env::current_dir() {                                                                                     
                                Ok(path) => println!("path: {}", path.display()),                                                               
                                Err(e) => eprintln!("pwd error: {}", e),                                                                        
                            }                                                                                                                   
                        }
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
                        _ => {
                            match std::process::Command::new(command).args(args).spawn() {
                                Ok(mut child) => {
                                    if let Err(e) = child.wait() {
                                        eprintln!("Error waiting for command: {}", e);
                                    }
                                }
                                Err(_) => {
                                    eprintln!("hex: command not found: {}", command);
                                }
                            }
                        }
                    }

            },
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    
}

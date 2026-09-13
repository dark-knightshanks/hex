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
                println!("Line: {}", trimmed);
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

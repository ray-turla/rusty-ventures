use crossterm::{
    event::{poll, read, Event::{Key, Mouse, Resize, FocusGained, FocusLost, Paste, self}, KeyEvent, self, KeyEventState, KeyCode, KeyModifiers },
    terminal::{enable_raw_mode, Clear, ClearType}, ExecutableCommand,
};
use std::{
    env, fs,
    io::{prelude::*, stdin, stdout},
    time::Duration,
};
fn main() -> Result<(), String> {
    const VALID_ACTIONS: [&str; 4] = ["create", "read", "edit", "delete"];
    let command = get_arg(CommandArgs::Op)?;
    let file_name = get_arg(CommandArgs::Filename)?;

    if !validate_arg(&command) || !validate_arg(&file_name) {
        println!("|command: {} file_name: {}|", &command, &file_name);
        return Err(String::from("One of the arguments was empty"));
    }

    match command.to_lowercase().as_str() {
        "create" => {
            create_note(&file_name)?;
            println!("\nFile created");
            return Ok(());
        }
        "read" => {
            let _ = read_note(&file_name)?;
            Ok(())
        }
        "edit" => {
            let _ = edit_note(&file_name)?;
            Ok(())
        }
        _ => {
            return Err(String::from(format!(
                "Invalid Action. Must be in one of {:?}",
                VALID_ACTIONS
            )))
        }
    }
}

enum CommandArgs {
    Op,
    Filename,
}

fn get_arg(arg: CommandArgs) -> Result<String, String> {
    let index = match arg {
        CommandArgs::Op => 1,
        CommandArgs::Filename => 2,
    };

    env::args().nth(index).ok_or_else(|| {
        match arg {
            CommandArgs::Op => "No action provided.",
            CommandArgs::Filename => "No filename provided.",
        }
        .to_string()
    })
}

fn validate_arg(arg: &String) -> bool {
    if arg.is_empty() {
        return false;
    }
    true
}

fn create_note(file_name: &String) -> Result<(), String> {
    println!("Enter Text (Press CTRL+D to apply changes): ");
    let mut buffer = String::new();
    // let mut buffer = String::new();

    let stdin = stdin();
    {
        let mut handle = stdin.lock();

        let _ = match handle.read_to_string(&mut buffer) {
            Ok(size) => size,
            Err(e) => return Err(e.to_string()),
        };

        let mut file = match fs::File::create(file_name) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        let _ = file.write_all(&mut buffer.as_bytes());
    }
    Ok(())
}

fn read_note(file_name: &String) -> Result<String, String> {
    match fs::read_to_string(file_name) {
        Ok(content) => {
            print!("{content}");
            Ok(content)
        }
        Err(e) => Err(e.to_string()),
    }
}

fn edit_note(file_name: &String) -> Result<(), String> {
    // Create a new file or open an existing one
    let mut contents = read_note(&file_name)?;
    let mut stdin = stdin();
    let mut user_input = String::new();
    let mut buffer = String::from(contents);
    let mut stdout = stdout();

    enable_raw_mode().unwrap();
    print!("Enter your edits (press Esc when done): ");
    // stdout.execute(Clear(ClearType::All)).unwrap();

    stdout.flush().unwrap();

    loop {
        if poll(Duration::from_millis(500)).unwrap() {
            match read().unwrap() {
                Key(event) => {
                    event.modifiers;
                    if event.state == KeyEventState::Down {
                        match event.code {
                            KeyCode::Esc => {
                                break;
                             }
                            KeyCode::Enter => {
                                user_input.push('\n');
                                print!("\n");
                                stdout.flush().unwrap();
                            }
                            KeyCode::Backspace => {
                                if user_input.is_empty() {
                                    buffer.pop();
                                    println!("{buffer}");
                                } else {
                                    user_input.pop();
                                    print!("\u{8} \u{8}");
                                }
                            }
                            _ => {
                                let c = match event.modifiers {
                                    KeyModifiers::CONTROL => "".to_string(),
                                    _ => String::from(format!("{:?}", event.code)),
                                };
                                user_input.push_str(&c);
                                print!("{}", c);
                                stdout.flush().unwrap();
                            }
                        }
                    }
                },
                _ => {continue;}
            }
        }
    }



    Ok(())
}

// fn edit_note(file_name: &String) -> Result<String, String> {
//     println!("Enter Edit Text (Press CTRL+D to apply changes): ");
//     let content = read_note(&file_name)?;
//     let mut buffer = String::from(content);
//     // let mut buffer = String::new();

//     let stdin = stdin();
//     let _ = stdout().flush();
//     {
//         let mut handle = stdin.lock();

//         loop {
//             print!("=>");
//             let mut user_input = String::new();
//             let _ = match handle.read_line(&mut user_input) {
//                 Ok(size) => size,
//                 Err(e) => return Err(e.to_string())
//             };

//             if user_input.is_empty() {
//                 break;
//             }

//             for c in &mut user_input.chars() {
//                 if c == '\x08' || c == '\x1B' || c == '\x7F' {
//                     buffer.pop();
//                 } else if c == '\x04' {
//                     break;
//                 } else {
//                     buffer.push(c);
//                     // print!("{buffer}");
//                 }
//             }
//             println!("\n{buffer}");
//             // stdout().lock().write(buffer.as_bytes());
//             // stdout().flush();
//         }

//         let mut file = match fs::File::create(file_name) {
//             Ok(f) => f,
//             Err(e) => return Err(e.to_string())
//         };

//         let _ = file.write_all(&mut buffer.as_bytes());

//     }
//     Ok(String::from(""))
// }

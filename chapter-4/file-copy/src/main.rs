use std::{env, fs};

fn main() -> Result<(), String> {
    let file_path = get_arg(CommandArgs::FilePath)?;
    let target_path = get_arg(CommandArgs::TargetPath)?;

    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("File `{}` {}", &file_path, e.to_string()))?;

    fs::write(&target_path, contents)
        .map_err(|e| e.kind().to_string())?;

    Ok(())
}

enum CommandArgs {
    FilePath,
    TargetPath,
}

fn get_arg(arg: CommandArgs) -> Result<String, String> {
    let index = match arg {
        CommandArgs::FilePath => 1,
        CommandArgs::TargetPath => 2,
    };

    env::args()
        .nth(index)
        .ok_or_else(|| {
            match arg {
                CommandArgs::FilePath => "No file provided.",
                CommandArgs::TargetPath => "No path provided.",
            }
            .to_string()
        })
}

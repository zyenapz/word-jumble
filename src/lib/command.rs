use std::io;

pub(crate) enum MenuCommand {
    Play,
    Exit,
}

impl MenuCommand {
    pub fn new(input_string: &String) -> Result<MenuCommand, io::Error> {
        match input_string.trim().to_lowercase().as_str() {
            "1" | "p" => Ok(MenuCommand::Play),
            "2" | "e" => Ok(MenuCommand::Exit),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("\'{}\' is an invalid command!", input_string.trim()),
            )),
        }
    }
}

pub(crate) enum ExitCommand {
    DoExit,
    DontExit,
}

impl ExitCommand {
    pub fn new(input_string: &String) -> Result<ExitCommand, io::Error> {
        match input_string.trim().to_lowercase().as_str() {
            "y" | "yes" => Ok(ExitCommand::DoExit),
            "n" | "no" => Ok(ExitCommand::DontExit),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("\'{}\' is an invalid command!", input_string.trim()),
            )),
        }
    }
}

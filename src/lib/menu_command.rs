use std::io;

pub(crate) enum MenuCommand {
    Play,
    Credits,
    Exit,
}

impl MenuCommand {
    pub fn new(input_string: &String) -> Result<MenuCommand, io::Error> {
        match input_string.trim().to_lowercase().as_str() {
            "1" | "p" => Ok(MenuCommand::Play),
            "2" | "c" => Ok(MenuCommand::Credits),
            "3" | "e" => Ok(MenuCommand::Exit),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "That's an invalid input!",
            )),
        }
    }
}

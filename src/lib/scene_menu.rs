use std::io;
use std::io::Write;

use crate::{
    clear_console, display_prompt,
    lib::{
        command::MenuCommand,
        console_macros::display_notice,
        scene::{Exit, Play},
    },
};

use super::{
    persistent_data::PersistentData,
    scene::{Menu, Scene},
};

impl Scene for Menu {
    fn handle(self: Box<Self>, _data: &mut PersistentData) -> Box<dyn Scene> {
        let mut is_active = true;
        let mut next_state: Box<dyn Scene> = self;
        let mut error_message = String::new();

        while is_active {
            clear_console!();
            println!("Welcome to Word Jumble!");
            println!("[1] Play");
            println!("[2] Exit");
            display_notice(error_message.as_str());

            display_prompt!();

            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Error encountered when getting input!");

            match MenuCommand::new(&buf) {
                Ok(cmd) => {
                    match cmd {
                        MenuCommand::Play => next_state = Box::new(Play),
                        // MenuCommand::Scores => next_state = Box::new(Scores),
                        MenuCommand::Exit => next_state = Box::new(Exit),
                    }
                    is_active = false
                }
                Err(e) => error_message = String::from(format!("{}", e)),
            }
        }

        next_state
    }
}

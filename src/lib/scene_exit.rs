use std::io;
use std::io::Write;

use crate::{
    clear_console, display_prompt,
    lib::{command::ExitCommand, console_macros::display_notice, scene::Menu},
};

use super::{
    persistent_data::PersistentData,
    scene::{Exit, Scene},
};

impl Scene for Exit {
    fn handle(self: Box<Self>, data: &mut PersistentData) -> Box<dyn Scene> {
        let mut is_active: bool = true;
        let mut next_state: Box<dyn Scene> = self;
        let mut error_message = String::new();

        while is_active {
            clear_console!();
            println!("Do you really wish to exit? [Y/n]");

            display_notice(error_message.as_str());

            display_prompt!();

            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Error encountered when getting input!");

            match ExitCommand::new(&buf) {
                Ok(cmd) => {
                    match cmd {
                        ExitCommand::DoExit => {
                            if *data.has_played_once() {
                                println!("Thank you for playing Word Jumble, bye!");
                            } else {
                                println!("Hope to see you soon!");
                            }
                            data.set_running_to_inactive();
                        }
                        ExitCommand::DontExit => {
                            next_state = Box::new(Menu);
                        }
                    }
                    is_active = false;
                }
                Err(e) => error_message = String::from(format!("{}", e)),
            }
        }

        next_state
    }
}

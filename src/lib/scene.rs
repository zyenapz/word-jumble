use std::{
    io::{self, Write},
    ops::Add,
};

use crate::lib::command::{ExitCommand, MenuCommand};

use super::persistent_data::PersistentData;

fn display_notice(msg: &str) {
    if !msg.is_empty() {
        println!(": {}", msg);
    }
}

macro_rules! clear_console {
    () => {
        print!("\x1B[2J\x1B[1;1H");
    };
}

pub trait Scene {
    fn handle(self: Box<Self>, data: &mut PersistentData) -> Box<dyn Scene>;
}

pub struct Menu;
pub struct Play;
pub struct Scores;
pub struct Exit;

impl Scene for Menu {
    fn handle(self: Box<Self>, data: &mut PersistentData) -> Box<dyn Scene> {
        let mut is_active = true;
        let mut next_state: Box<dyn Scene> = self;
        let mut error_message = String::new();

        while is_active {
            clear_console!();
            println!("Welcome to Word Jumble!");
            println!("[1] Play");
            println!("[2] Credits");
            println!("[3] Exit");
            display_notice(error_message.as_str());

            print!("> ");
            io::stdout()
                .flush()
                .expect("Error encountered when flushing output!");

            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Error encountered when getting input!");

            match MenuCommand::new(&buf) {
                Ok(cmd) => {
                    match cmd {
                        MenuCommand::Play => next_state = Box::new(Play),
                        MenuCommand::Scores => next_state = Box::new(Scores),
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

impl Scene for Play {
    fn handle(self: Box<Self>, data: &mut PersistentData) -> Box<dyn Scene> {
        println!("Play");
        data.set_running_to_inactive();

        self
    }
}

impl Scene for Scores {
    fn handle(self: Box<Self>, data: &mut PersistentData) -> Box<dyn Scene> {
        println!("Scores");
        data.set_running_to_inactive();

        Box::new(Menu)
    }
}

impl Scene for Exit {
    fn handle(self: Box<Self>, data: &mut PersistentData) -> Box<dyn Scene> {
        let mut is_active: bool = true;
        let mut next_state: Box<dyn Scene> = self;
        let mut error_message = String::new();

        while is_active {
            clear_console!();
            println!("Do you really wish to exit? [Y/n]");

            display_notice(error_message.as_str());

            print!("> ");
            io::stdout()
                .flush()
                .expect("Error encountered when flushing output!");

            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Error encountered when getting input!");

            // TODO change later
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

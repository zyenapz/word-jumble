use std::{
    io::{self, Write},
    ops::Add,
};

use crate::lib::menu_command::MenuCommand;

use super::game_data::GameData;

macro_rules! clear_console {
    () => {
        print!("\x1B[2J\x1B[1;1H");
    };
}

pub trait Scene {
    fn handle(self: Box<Self>, data: &mut GameData) -> Box<dyn Scene>;
}

pub struct Menu;
pub struct Play;
pub struct Credits;
pub struct Exit;

impl Scene for Menu {
    fn handle(self: Box<Self>, data: &mut GameData) -> Box<dyn Scene> {
        let mut is_active = true;
        let mut next_state: Box<dyn Scene> = self;
        let mut error_message = String::new();

        while is_active {
            clear_console!();
            println!("Welcome to Word Jumble!!");
            println!("[1] Play");
            println!("[2] Credits");
            println!("[3] Exit");
            println!("{}", error_message);

            print!("> ");
            io::stdout()
                .flush()
                .expect("Error encountered when flushing output!");

            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Error encountered when getting input!");

            // TODO change later
            match MenuCommand::new(&buf) {
                Ok(cmd) => {
                    match cmd {
                        MenuCommand::Play => next_state = Box::new(Play),
                        MenuCommand::Credits => next_state = Box::new(Credits),
                        MenuCommand::Exit => next_state = Box::new(Exit),
                    }
                    is_active = false
                }
                Err(e) => error_message = String::from(format!(": {}", e)),
            }
        }

        next_state
    }
}

impl Scene for Play {
    fn handle(self: Box<Self>, data: &mut GameData) -> Box<dyn Scene> {
        println!("Play");
        data.set_running_to_inactive();

        self
    }
}

impl Scene for Credits {
    fn handle(self: Box<Self>, data: &mut GameData) -> Box<dyn Scene> {
        println!("Credits");
        data.set_running_to_inactive();

        self
    }
}

impl Scene for Exit {
    fn handle(self: Box<Self>, data: &mut GameData) -> Box<dyn Scene> {
        println!("Exit");
        data.set_running_to_inactive();
        self
    }
}

use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use console::style;
use rand::prelude::SliceRandom;

use crate::lib::{
    command::{ExitCommand, MenuCommand},
    word::Word,
};

use super::{persistent_data::PersistentData, theme::Theme};

use std::io::Read;

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

macro_rules! pause_console {
    ($msg: expr) => {
        let mut stdout = io::stdout();
        stdout.write($msg).unwrap();
        stdout.flush().unwrap();
        io::stdin().read(&mut [0]).unwrap();
    };
}

macro_rules! display_prompt {
    () => {
        print!("> ");
        io::stdout()
            .flush()
            .expect("Error encountered when flushing stdout!");
    };
}

const MAX_RESHUFFLES: u8 = 2;

pub trait Scene {
    fn handle(self: Box<Self>, data: &mut PersistentData) -> Box<dyn Scene>;
}

pub struct Menu;
pub struct Play;
pub struct Scores;
pub struct Exit;

impl Scene for Menu {
    fn handle(self: Box<Self>, _data: &mut PersistentData) -> Box<dyn Scene> {
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

            display_prompt!();

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
        data.set_has_played_once_to_true();
        clear_console!();

        let next_state: Box<dyn Scene> = Box::new(Menu);
        let theme: Theme = rand::random();

        let mut cur_q = 0;
        let mut questions = Self::generate_questions(&theme);

        let mut score: u8 = 0_u8;
        let mut notice_msg = String::new();
        let mut answer = String::new();

        let mut is_running = true;
        let mut reshuffles_left = MAX_RESHUFFLES;

        Self::display_instructions(theme.to_string().as_str());

        while is_running {
            if cur_q >= questions.len() {
                is_running = false;
            }

            clear_console!();
            answer.clear();

            println!(
                "| {} | {} | {} |. \n(Q{} of {}). Unjumble this word: {}",
                style(format!("Theme: {}", theme)).cyan(),
                style(format!("Score: {}", score)).green(),
                style("/lc to list commands").yellow(),
                cur_q + 1,
                questions.len(),
                questions[cur_q].get_jumbled_form(),
            );

            println!();
            display_notice(&notice_msg);

            display_prompt!();

            io::stdin()
                .read_line(&mut answer)
                .expect("Error encountered when reading answer!");

            let fmt_answer = answer.trim().to_lowercase();

            if let Some(c) = fmt_answer.chars().next() {
                if let "/" = c.to_string().as_str() {
                    match fmt_answer.as_str() {
                        "/t" | "/time" | "/timer" => {
                            todo!();
                            // Add a timer functionality
                        }
                        "/r" | "/reshuffle" | "/shuffle" => {
                            if reshuffles_left != 0 {
                                questions[cur_q].jumble_word(&mut rand::thread_rng());
                                reshuffles_left -= 1;
                                notice_msg =
                                    format!("You have {} reshuffles left!", reshuffles_left);
                            } else {
                                notice_msg = "You have no reshuffles left!".to_string();
                            }
                        }
                        "/s" | "/skip" => {
                            Self::display_skip_msg(questions[cur_q].get_normal_form(), &mut cur_q)
                        }
                        "/q" | "/quit" => {
                            is_running = false;
                        }
                        "/lc" => {
                            Self::display_commands();
                            pause_console!(b"Press [Return] to continue...");
                            pause_console!(b"");
                            notice_msg.clear();
                        }
                        _ => notice_msg = format!("\'{}\' is an invalid command!", fmt_answer),
                    }
                } else {
                    if let true = !fmt_answer.is_empty() {
                        if let true =
                            fmt_answer.as_str() == questions[cur_q].get_normal_form().to_lowercase()
                        {
                            println!("{}", style("\nCorrect!").green());
                            thread::sleep(Duration::from_secs(2));
                            score += 1;
                            cur_q += 1;
                        } else {
                            let msg = format!(
                                "\nWrong! Correct answer is \'{}\'",
                                questions[cur_q].get_normal_form()
                            );
                            println!("{}", style(msg).red());
                            thread::sleep(Duration::from_secs(2));
                            cur_q += 1;
                        }

                        reshuffles_left = MAX_RESHUFFLES; // reset reshuffles
                        notice_msg.clear();
                    } else {
                        notice_msg = "Cannot accept empty answers!".to_string()
                    }
                }
            } else {
                notice_msg = "Cannot accept empty answers!".to_string()
            }
        }

        // Display score
        clear_console!();
        println!("Game over! You got {} out of {}!", score, questions.len());
        thread::sleep(Duration::from_secs(2));
        pause_console!(b"Press [Return] to go back to Menu...");

        next_state
    }
}

impl Play {
    fn display_commands() {
        println!("\nCommands: ");
        println!("/t -> check timer ");
        println!("/r -> reshuffle the word");
        println!("/s -> skip question");
        println!("/q -> forfeit game");
        println!("/lc -> get commands list");
    }

    fn display_skip_msg(correct_answer: &str, cur_q: &mut usize) {
        let msg = format!("\nYou skipped! Correct answer is \'{}\'", correct_answer);
        println!("{}", style(msg).red());
        thread::sleep(Duration::from_secs(2));
        *cur_q += 1;
    }

    fn generate_questions(theme: &Theme) -> Vec<Word> {
        let mut rng = rand::thread_rng();
        let mut words = theme.get_word(&mut rng);
        let no_of_questions = 5;
        words.shuffle(&mut rng);
        let q_set = words[0..no_of_questions].to_vec();

        q_set
    }

    fn display_instructions(theme: &str) {
        println!("INSTRUCTIONS (1 of 2)");
        println!("Guess the word before the timer runs out!");
        println!("\nRules:");
        println!("- {} reshuffles per question", MAX_RESHUFFLES);
        println!("- Hints reduce your score");
        println!("- No score once timer runs out");
        println!();

        thread::sleep(Duration::from_secs(1));

        pause_console!(b"Press [Return] to continue...\n");

        clear_console!();

        println!("INSTRUCTIONS (2 of 2)");
        Self::display_commands();

        println!(
            "\nThe theme this game is \'{}\'! Good luck!\n",
            style(theme).green()
        );

        thread::sleep(Duration::from_secs(1));
        pause_console!(b""); // adding this here because rust is reading the input too darn fast and i'm too lazy to find a non-hacky fix kek
        pause_console!(b"Press [Return] to start!\n");
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

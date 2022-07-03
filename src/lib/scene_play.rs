use std::io::Read;
use std::io::Write;
use std::{
    io, thread,
    time::{Duration, Instant},
};

use console::style;
use rand::prelude::SliceRandom;

use crate::{
    clear_console, display_prompt,
    lib::{console_macros::display_notice, scene::Menu, theme::Theme},
    pause_console,
};

use super::{
    persistent_data::PersistentData,
    scene::{Play, Scene},
    word::Word,
};

const MAX_RESHUFFLES: u8 = 2;
const MAX_TIMELIMIT: u64 = 30;

impl Scene for Play {
    fn handle(self: Box<Self>, data: &mut PersistentData) -> Box<dyn Scene> {
        // Game session variables
        let next_state: Box<dyn Scene> = Box::new(Menu);
        let theme: Theme = rand::random();

        let mut cur_q = 0;
        let mut questions = Self::generate_questions(&theme);

        let mut score: u8 = 0_u8;
        let mut notice_msg = String::new();
        let mut answer = String::new();

        let mut is_running = true;
        let mut reshuffles_left = MAX_RESHUFFLES;

        // Set "has played once" variable to true
        data.set_has_played_once_to_true();

        // Display instructions
        Self::display_instructions(theme.to_string().as_str());
        clear_console!();

        // Start timer
        let mut timer = Instant::now();
        const TIME_LIMIT: Duration = Duration::from_secs(MAX_TIMELIMIT);

        // Game loop
        while is_running {
            // Check if game is over
            if cur_q >= questions.len() {
                is_running = false;
            } else {
                // Clear input and console
                clear_console!();
                answer.clear();

                // Print question and other relevant information
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

                // Get player's input
                display_notice(&notice_msg);

                display_prompt!();

                io::stdin()
                    .read_line(&mut answer)
                    .expect("Error encountered when reading answer!");

                let fmt_answer = answer.trim().to_lowercase();

                // Process player's input
                if let Some(c) = fmt_answer.chars().next() {
                    if "/" == c.to_string().as_str() {
                        // Match commands
                        match fmt_answer.as_str() {
                            "/t" | "/time" | "/timer" => {
                                match TIME_LIMIT.checked_sub(timer.elapsed()) {
                    Some(t) => {
                        notice_msg = format!("Remaining time is {}s", t.as_secs())
                    }
                    None => notice_msg = format!("You have run out of time! No score will be granted for this question now."),
                }
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
                            "/s" | "/skip" => Self::display_skip_msg(
                                questions[cur_q].get_normal_form(),
                                &mut cur_q,
                            ),
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
                        // Check if there is time left, first
                        if let Some(_) = TIME_LIMIT.checked_sub(timer.elapsed()) {
                            if !fmt_answer.is_empty() {
                                // Then match non-commands

                                if fmt_answer.as_str()
                                    == questions[cur_q].get_normal_form().to_lowercase()
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

                                // Reset some game variables
                                reshuffles_left = MAX_RESHUFFLES;
                                notice_msg.clear();
                                timer = Instant::now();
                            } else {
                                notice_msg = "Cannot accept empty answers!".to_string()
                            }
                        } else {
                            println!("{}", style("No time left, no points granted.").on_red());
                            thread::sleep(Duration::from_secs(2));
                            cur_q += 1;

                            // Reset some game variables
                            reshuffles_left = MAX_RESHUFFLES;
                            notice_msg.clear();
                            timer = Instant::now();
                        }
                    }
                } else {
                    notice_msg = "Cannot accept empty answers!".to_string()
                }
            }
        }

        // Display score
        clear_console!();
        println!("Game over! You got {} out of {}!", score, questions.len());
        thread::sleep(Duration::from_secs(2));
        pause_console!(b"Press [Return] to go back to Menu...");
        pause_console!(b"");

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
        pause_console!(b"");
    }
}

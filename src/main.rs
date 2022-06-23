use std::io;
use std::io::Write;

use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;

fn main() {
    let mut is_running: bool = true;
    let mut action: String = String::new();
    let mut state: Scene = Scene::Menu;

    while is_running {
        action = String::new();

        match state {
            Scene::Menu => {
                display_title();
                display_options();

                io::stdin()
                    .read_line(&mut action)
                    .expect("Error getting input");

                match &action.trim()[..] {
                    "1" => state = Scene::Play,
                    "2" => state = Scene::Scores,
                    "3" => state = Scene::Quit,
                    _ => println!("Invalid"),
                }
            }
            Scene::Play => {
                // let jumbled = jumble_word(&String::from("Hello"));
                // println!("{}", jumbled);
                todo!();
            }
            Scene::Scores => {
                todo!();
            }
            Scene::Quit => {
                println!("Thanks for playing!");
                is_running = false;
            }
        }
    }
}

#[derive(PartialEq)]
enum Scene {
    Menu,
    Play,
    Scores,
    Quit,
}
struct Word {
    pub(crate) normal_form: String,
    pub(crate) jumbled_form: String,
}

fn display_title() {
    println!("Jumble Rust!");
    println!("------------------------");
    println!("Version 1.0 | by zyenapz\n");
}

fn display_options() {
    println!("Select option no.:");
    println!("[1] Play");
    println!("[1] Hi-score");
    println!("[3] Exit");
    print!("> ");
    io::stdout().flush().unwrap();
}

fn jumble_word(word: &String) -> String {
    let mut jumbled: String = String::from("");

    let mut unjumbled: Vec<char> = word.chars().collect();
    let mut rng = rand::thread_rng();
    let mut irs = Irs::default();

    match irs.shuffle(&mut unjumbled, &mut rng) {
        Ok(_) => jumbled = unjumbled.into_iter().collect(),
        Err(_) => {
            println!("\n[Error]: something went wrong when shuffling the word.\n");
        }
    }

    return jumbled;
    // TODO
}

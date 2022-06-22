use std::io;
use std::io::Write;

fn main() {
    let mut is_running: bool = true;
    let mut action: String = String::new();

    while is_running {
        /* Display welcome screen and options
        TODO:
        1. Add welcome and title texts
        2. Add options (Play, Scores, Quit)
        3. Add attributions info at the bottom
        4. Add a way to get the user input
        */

        display_title();
        display_options();

        io::stdin()
            .read_line(&mut action)
            .expect("Error getting input");

        is_running = false;
    }
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
    let word_length: usize = word.len();

    let mut jumbled: String = String::from("");

    return jumbled;
    // TODO
}

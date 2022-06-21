use difficulty::Difficulty;
use game_data::SessionData;
use word::Word;

fn jumble_word(word: &String) {
    let word_length: usize = word.len();

    // TODO
}

fn main() {
    let mut data: SessionData = SessionData {
        mistakes: 0,
        score: 0,
        difficulty: Difficulty::Easy,
        words: vec![Word {
            normal_form: "SDAS".to_string(),
            jumbled_form: "SDAS".to_string(),
        }],
    };

    let mut is_running: bool = true;

    while is_running {
        /* Display welcome screen and options
        TODO:
        1. Add welcome and title texts
        2. Add options (Play, Scores, Quit)
        3. Add attributions info at the bottom
        4. Add a way to get the user input
        */
    }
}

mod difficulty {
    use crate::game_data::game_constants;

    pub enum Difficulty {
        Easy,
        Medium,
        Hard,
    }

    impl Difficulty {
        pub fn get_max_mistakes(&self) -> u8 {
            match self {
                &Difficulty::Easy => game_constants::MAX_MISTAKES_EASY,
                &Difficulty::Medium => game_constants::MAX_MISTAKES_MEDI,
                &Difficulty::Hard => game_constants::MAX_MISTAKES_HARD,
            }
        }
    }
}

mod word {
    pub struct Word {
        pub(crate) normal_form: String,
        pub(crate) jumbled_form: String,
    }
}

mod game_data {
    use crate::{difficulty::Difficulty, word::Word};

    pub mod game_constants {
        pub const QUESTIONS_PER_GAME: u8 = 15;
        pub const MAX_MISTAKES_EASY: u8 = 6;
        pub const MAX_MISTAKES_MEDI: u8 = 4;
        pub const MAX_MISTAKES_HARD: u8 = 3;
    }

    pub struct SessionData {
        pub(crate) mistakes: u8,
        pub(crate) score: u8,
        pub(crate) difficulty: Difficulty,
        pub(crate) words: Vec<Word>,
    }
}

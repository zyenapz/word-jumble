use difficulty::Difficulty;
use game_data::SessionData;
use words::Word;

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

mod words {
    pub struct Word {
        pub(crate) normal_form: String,
        pub(crate) jumbled_form: String,
    }
}

mod game_data {
    use crate::{difficulty::Difficulty, words::Word};

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

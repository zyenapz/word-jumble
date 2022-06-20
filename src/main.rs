enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    fn get_max_mistakes(&self) -> u8 {
        match self {
            &Difficulty::Easy => game_constants::MAX_MISTAKES_EASY,
            &Difficulty::Medium => game_constants::MAX_MISTAKES_MEDI,
            &Difficulty::Hard => game_constants::MAX_MISTAKES_HARD,
        }
    }
}

mod game_constants {
    pub const QUESTIONS_PER_GAME: u8 = 15;
    pub const MAX_MISTAKES_EASY: u8 = 6;
    pub const MAX_MISTAKES_MEDI: u8 = 4;
    pub const MAX_MISTAKES_HARD: u8 = 3;
}

fn jumble_word(word: &String) {
    let word_length: usize = word.len();

    // TODO
}

pub struct SessionData {
    mistakes: u8,
    score: u8,
    difficulty: Difficulty,
    words: Vec<Word>,
}

pub struct Word {
    normal_form: String,
    jumbled_form: String,
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

    data.score += 1;

    println!("Score: {}", data.score);
}

use std::io;

use rand::RngCore;

use super::{
    theme_data::{WORDS_ANIMALS, WORDS_COUNTRIES, WORDS_FOOD, WORDS_SPACE, WORDS_WEBSITES},
    word::Word,
};

#[derive(Debug, PartialEq)]
pub(crate) enum Theme {
    Space,
    Food,
    Websites,
    Animals,
    Countries,
}

impl Theme {
    pub fn new(input_string: &str) -> Result<Theme, io::Error> {
        match input_string.trim().to_lowercase().as_str() {
            "1" | "s" | "space" => Ok(Theme::Space),
            "2" | "f" | "food" => Ok(Theme::Food),
            "3" | "w" | "websites" => Ok(Theme::Websites),
            "4" | "a" | "animals" => Ok(Theme::Animals),
            "5" | "c" | "countries" => Ok(Theme::Countries),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "That is an invalid input!",
            )),
        }
    }

    pub fn get_word(&self, rng: &mut dyn RngCore) -> Vec<Word> {
        match self {
            Theme::Space => Self::generate_words(WORDS_SPACE, rng),
            Theme::Food => Self::generate_words(WORDS_FOOD, rng),
            Theme::Websites => Self::generate_words(WORDS_WEBSITES, rng),
            Theme::Animals => Self::generate_words(WORDS_ANIMALS, rng),
            Theme::Countries => Self::generate_words(WORDS_COUNTRIES, rng),
        }
    }

    fn generate_words(words: &[&'static str], rng: &mut dyn RngCore) -> Vec<Word> {
        let mut words_collection = Vec::<Word>::new();

        for word in words {
            let jumble = match Word::new(word.to_string(), rng) {
                Ok(w) => w,
                Err(_) => panic!("Error when generating words collection!"),
            };

            words_collection.push(jumble);
        }

        words_collection
    }
}

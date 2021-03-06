use std::io;

use rand::RngCore;
use shuffle::{irs::Irs, shuffler::Shuffler};

const MAX_WORD_LENGTH: u8 = 30;

macro_rules! is_alphabetic {
    ($str: expr) => {
        $str.chars().all(|c| matches!(c, 'a'..='z' | 'A' ..= 'Z'))
    };
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Word {
    normal: String,
    jumbled: String,
}

impl Word {
    pub fn new(normal_word: String, rng: &mut dyn RngCore) -> Result<Word, io::Error> {
        match is_alphabetic!(normal_word.clone()) && normal_word.len() <= MAX_WORD_LENGTH.into() {
            true => {
                let mut irs = Irs::default();

                let nw_clone = normal_word.clone();
                let mut data = nw_clone.into_bytes();

                irs.shuffle(&mut data, rng)
                    .expect("Error encountered when shuffling!");

                Ok(Word {
                    normal: normal_word.to_string(),
                    jumbled: match std::str::from_utf8(&data) {
                        Ok(r) => r.to_string(),
                        Err(e) => panic!("Invalid UTF-8 sequence: {}!", e),
                    },
                })
            }
            false => Err(std::io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Invalid word! Alphabetic words only with a maximum length of {} letters!",
                    MAX_WORD_LENGTH,
                ),
            )),
        }
    }

    pub fn jumble_word(&mut self, rng: &mut dyn RngCore) {
        let mut irs = Irs::default();

        let nw_clone = self.jumbled.clone();
        let mut data = nw_clone.into_bytes();

        irs.shuffle(&mut data, rng)
            .expect("Error encounterd when shuffling!");

        self.jumbled = match std::str::from_utf8(&data) {
            Ok(r) => r.to_string(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}!", e),
        };
    }

    pub fn get_normal_form(&self) -> &String {
        &self.normal
    }

    pub fn get_jumbled_form(&self) -> &String {
        &self.jumbled
    }
}

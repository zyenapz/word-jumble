use std::io;

use rand::rngs::mock::StepRng;

use crate::Word;

#[test]
pub fn test_new_word() {
    let mut rng = StepRng::new(1, 10);
    let word = Word::new("JUPITER".to_string(), &mut rng).unwrap();

    assert_ne!(*word.get_jumbled_form(), "JUPITER".to_string());
    assert_eq!(*word.get_normal_form(), "JUPITER".to_string());
}

#[test]
pub fn test_word_with_numbers() {
    let mut rng = StepRng::new(1, 10);
    let word = Word::new("JUPITER1231".to_string(), &mut rng);

    assert!(word.is_err());
}

#[test]
pub fn test_long_word() {
    let mut rng = StepRng::new(1, 10);
    let word = Word::new("IAMAVERYLONGSTRINGOFWORDYESINDEEDIAM".to_string(), &mut rng);

    assert!(word.is_err());
}

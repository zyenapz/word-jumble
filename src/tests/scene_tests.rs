use rand::prelude::SliceRandom;

use crate::lib::theme::Theme;

#[test]
pub fn test_fetching_of_words() {
    let mut rng = rand::thread_rng();
    let theme: Theme = rand::random();
    let mut words = theme.get_word(&mut rng);
    let no_of_questions = 5;
    words.shuffle(&mut rng);
    let slice = &words[0..no_of_questions];

    assert_eq!(slice.len(), 5);
}

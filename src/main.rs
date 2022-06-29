mod lib;

#[cfg(test)]
mod tests;

use rand::rngs::mock::StepRng;

use crate::lib::word::Word;

fn main() {
    let mut rng = StepRng::new(1, 10);
    let w1 = Word::new("JUPITER".to_string(), &mut rng).unwrap();

    println!("{}", w1.get_jumbled_form());
    println!("{}", w1.get_normal_form());
}

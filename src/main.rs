mod lib;

#[cfg(test)]
mod tests;

use crate::lib::{theme::Theme, word::Word};

fn main() {
    let mut rng = rand::thread_rng();
    let mut w1 = Word::new("JUPITER".to_string(), &mut rng).unwrap();

    println!("{}", w1.get_jumbled_form());
    w1.jumble_word(&mut rng);
    println!("{}", w1.get_jumbled_form());

    println!("Hello!!");

    let x = Theme::new("s").unwrap();
    let z = x.get_word(&mut rng);

    println!("Len is: {}", z.len());

    for e in z {
        println!("{} . {}", e.get_normal_form(), e.get_jumbled_form());
    }
}

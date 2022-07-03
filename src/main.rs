mod lib;

#[cfg(test)]
mod tests;

use lib::game::Game;

fn main() {
    let mut game = Game::new();

    game.run();
}

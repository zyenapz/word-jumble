mod lib;

#[cfg(test)]
mod tests;

use std::thread;
use std::time::{Duration, Instant};

use lib::game::Game;

fn main() {
    let mut game = Game::new();

    game.run();
}

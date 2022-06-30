use super::game::Game;

pub struct GameData {
    is_running: bool,
}

impl GameData {
    pub fn new() -> GameData {
        GameData { is_running: true }
    }

    pub fn is_running(&self) -> &bool {
        &self.is_running
    }

    pub fn set_running_to_inactive(&mut self) {
        self.is_running = false;
    }
}

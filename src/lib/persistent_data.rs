pub struct PersistentData {
    is_running: bool,
    has_played_once: bool,
}

impl PersistentData {
    pub fn new() -> PersistentData {
        PersistentData {
            is_running: true,
            has_played_once: false,
        }
    }

    pub fn is_running(&self) -> &bool {
        &self.is_running
    }

    pub fn has_played_once(&self) -> &bool {
        &self.has_played_once
    }

    pub fn set_running_to_inactive(&mut self) {
        self.is_running = false;
    }

    pub fn set_has_played_once_to_true(&mut self) {
        self.has_played_once = true;
    }
}

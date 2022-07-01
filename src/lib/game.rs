use super::{
    persistent_data::PersistentData,
    scene::{Menu, Scene},
};

pub struct Game {
    scene: Option<Box<dyn Scene>>,
    data: PersistentData,
}

impl Game {
    pub fn new() -> Game {
        Game {
            scene: Some(Box::new(Menu)),
            data: PersistentData::new(),
        }
    }

    pub fn run(&mut self) {
        while *self.data.is_running() {
            if let Some(s) = self.scene.take() {
                self.scene = Some(s.handle(&mut self.data))
            }
        }
    }
}

use super::persistent_data::PersistentData;

pub trait Scene {
    fn handle(self: Box<Self>, data: &mut PersistentData) -> Box<dyn Scene>;
}

pub struct Menu;
pub struct Play;
pub struct Exit;

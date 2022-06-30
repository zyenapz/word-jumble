pub struct Game {
    state: Option<Box<dyn State>>,
    is_running: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: Some(Box::new(Menu)),
            is_running: true,
        }
    }

    pub fn run(&mut self) {
        while self.is_running {
            println!("Hello!!");
        }
    }
}

pub trait State {
    fn run(&self);
}

pub struct Menu;
pub struct Play;
pub struct Credits;
pub struct Exit;

impl State for Menu {
    fn run(&self) {
        println!("Hello Menu!!");
    }
}

impl State for Credits {
    fn run(&self) {
        println!("Hello Credits!!");
    }
}

impl State for Exit {
    fn run(&self) {
        println!("Hello Exit!!");
    }
}

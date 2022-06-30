use std::io;

pub struct Game {
    scene: Option<Box<dyn Scene>>,
    data: GameData,
}

pub struct GameData {
    is_running: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            scene: Some(Box::new(Menu)),
            data: GameData { is_running: true },
        }
    }

    pub fn run(&mut self) {
        while self.data.is_running {
            if let Some(s) = self.scene.take() {
                self.scene = Some(s.handle(&mut self.data))
            }
        }
    }
}

pub trait Scene {
    fn handle(self: Box<Self>, data: &mut GameData) -> Box<dyn Scene>;
}

pub struct Menu;
pub struct Play;
pub struct Credits;
pub struct Exit;

impl Scene for Menu {
    fn handle(self: Box<Self>, data: &mut GameData) -> Box<dyn Scene> {
        let mut is_active = true;
        let mut next_state: Box<dyn Scene>;

        while is_active {
            println!("Welcome to Word Jumble!!");

            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Error encountered when getting input!");

            match buf.trim().to_lowercase().as_str() {
                "1" => is_active = false,
                _ => println!("Lol"),
            }
        }

        next_state = Box::new(Exit);
        next_state
    }
}

impl Scene for Play {
    fn handle(self: Box<Self>, data: &mut GameData) -> Box<dyn Scene> {
        println!("Play");
        self
    }
}

impl Scene for Credits {
    fn handle(self: Box<Self>, data: &mut GameData) -> Box<dyn Scene> {
        println!("Credits");
        self
    }
}

impl Scene for Exit {
    fn handle(self: Box<Self>, data: &mut GameData) -> Box<dyn Scene> {
        println!("Exit");
        data.is_running = false;
        self
    }
}

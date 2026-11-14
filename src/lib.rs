use std::sync::{ LazyLock, Mutex };
use raylib::prelude::*;

pub static BLOCKS: LazyLock<Mutex<Vec<Block>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub const G: f32 = 1.0;
pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;

#[derive(Debug, PartialEq)]
pub enum State {
    Falling,
    Grounded
}

#[derive(Debug, PartialEq)]
pub enum ObjType {
    Small,
    Medium,
    Big
}

#[derive(Debug, PartialEq, Clone)]
pub enum BlockType {
    Stone,
    Planks
}

#[derive(Debug, PartialEq)]
pub struct Player {
    pub position: Vector2,
    pub size: Vector2,
    pub falling_speed: f32,
    pub moving_speed: f32,
    pub state: State
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub size: Vector2,
    pub position: Vector2,
    pub block_type: BlockType
}

#[derive(Debug, PartialEq)]
pub struct Booster {
    pub booster_type: ObjType,
    pub speed_boost: f32
}

impl Block {
    pub fn new(size: Vector2, position: Vector2, block_type: BlockType) -> Block {
        let block = Block {
            size,
            position,
            block_type
        };

        BLOCKS.lock().unwrap().push(block.clone());
        block
    }
}

impl Player {
    pub fn new() -> Player {
        let player = Player {
            size: Vector2::new(50.0, 100.0),
            position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            falling_speed: 0.0,
            moving_speed: 0.0,
            state: State::Falling
        };

        player
    }

    pub fn collides_left(self: &Player, block: &Block) -> bool {
        self.position.x <= block.position.x + block.size.x &&
        self.position.x >= block.position.x &&
        self.position.y <= block.position.y + block.size.y &&
        self.position.y + self.size.y >= block.position.y
    }

    pub fn collides_right(self: &Player, block: &Block) -> bool {
        self.position.x + self.size.x >= block.position.x &&
        self.position.x <= block.position.x &&
        self.position.y <= block.position.y + block.size.y &&
        self.position.y + self.size.y >= block.position.y
    }

    pub fn is_on(self: &Player, block: &Block) -> bool {
        self.position.y + self.size.y >= block.position.y &&
        self.position.y <= block.position.y + block.size.y &&
        self.position.x + self.size.x >= block.position.x &&
        self.position.x <= block.position.x + block.size.x
    }

    pub fn is_roofed(self: &Player, block: &Block) -> bool{
        self.position.y <= block.position.y + block.size.y &&
        self.position.y >= block.position.y &&
        self.position.x + self.size.x >= block.position.x &&
        self.position.x <= block.position.x + block.size.x
    }
}

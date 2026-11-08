use std::sync::{ LazyLock, Mutex };

pub static BLOCKS: LazyLock<Mutex<Vec<Block>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub const G: f32 = 1.0;

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

#[derive(Debug, PartialEq)]
pub enum BlockType {
    Stone,
    Planks
}

#[derive(Debug, PartialEq)]
pub struct Player {
    size: i32,
    falling_speed: f32,
    moving_speed: f32,
    state: State
}

#[derive(Debug, PartialEq)]
pub struct Block {
    size: i32,
    block_type: BlockType
}

#[derive(Debug, PartialEq)]
pub struct Booster {
    booster_type: ObjType,
    speed_boost: f32
}

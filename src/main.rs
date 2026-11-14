mod lib;

use lib::{ State, Player, G, Block, BLOCKS, BlockType };
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .resizable()
        .vsync()
        .title("greets")
        .build();

    while !rl.window_should_close() {
        if rl.is_key_down(KEY_ENTER) {
            break;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Press Enter to start playing", 250, 250, 40, Color::GREEN)
    }
    run_game(rl, thread);
}




fn run_game(mut rl: RaylibHandle, thread: RaylibThread) {

    let mut tempgr = 0;
    let mut tempcoll = 0;
    let mut tempcolr = 0;

    let mut player = Player::new();

    let block1 = Block::new(Vector2::new(50.0, 20.0), Vector2::new(200.0, 450.0), BlockType::Planks);
    let block2 = Block::new(Vector2::new(50.0, 50.0), Vector2::new(150.0, 500.0), BlockType::Stone);

    let player_idle_tex = rl.load_texture(&thread, "assets/textures/player_idle.png").expect("Failed to load texture: 'player_jumping_tex'");
    let player_falling_tex = rl.load_texture(&thread, "assets/textures/player_falling.png").expect("Failed to load texture: 'player_falling_tex'");
    let player_jumping_tex = rl.load_texture(&thread, "assets/textures/player_jumping.png").expect("Failed to load texture: 'player_jumping_tex'");

    while !rl.window_should_close() {

        let player_tex = if player.state == State::Falling { &player_falling_tex } else { &player_idle_tex };
        for block in BLOCKS.lock().unwrap().iter() {

            if player.is_on(&block) {
                tempgr += 1
            }
            if player.collides_left(&block) {
                tempcoll += 1
            }
            if player.collides_right(&block) {
                tempcolr += 1
            }
            if player.is_roofed(&block) {
                player.falling_speed = 0.5
            }
        }

        if tempgr >= 1 { player.state = State::Grounded }

        if rl.is_key_down(KEY_SPACE) {
            if player.state == State::Grounded {
                player.state = State::Falling;
                player.falling_speed *= -0.8;
            }
        }
        if rl.is_key_down(KEY_A) {
            if tempcolr == 0 && player.position.x > 0.0 {
                player.position.x -= player.moving_speed
            }
        }
        if rl.is_key_down(KEY_D) {
            if tempcoll == 0 && player.position.x + player.size.x <= SCREEN_WIDTH{
                player.position.x += player.moving_speed
            }
        }


        if player.state == State::Falling {
            player.falling_speed += G;
            player.position.y += player.falling_speed
        }

        if player.position.y + player.size.y >= SCREEN_HEIGHT {
            player.state = State::Grounded;
        }

        if player.position.y + player.size.y < SCREEN_HEIGHT && tempgr == 0 { player.state = State::Falling }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture_ex(
            player_tex,
            Vector2::new(player.position.x - 20.0, player.position.y - 35.0),
            0.0,
            2.0,
            Color::WHITE
        );

        for block in BLOCKS.lock().unwrap().iter() {
            d.draw_rectangle_v(block.position, block.size, Color::BROWN);
        }
        d.draw_rectangle_v((0.0, 600.0), (800.0, 5.0), Color::BURLYWOOD);
        (tempgr, tempcoll, tempcolr) = (0, 0, 0)
    }
}

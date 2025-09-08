mod assets;
mod board;
mod game;
mod snake;

use crate::game::Game;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    assets::window_conf()
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;
    game.run().await;
}

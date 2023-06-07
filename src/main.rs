mod data;
mod state;
mod images;
mod player;
mod health;
mod tiles;
mod enemies;

use data::{WIN_WIDTH, WIN_HEIGHT};
use state::State;

use ggez::{
    GameResult, ContextBuilder,
    conf::{WindowSetup, WindowMode},
    event,
};

fn main() -> GameResult {
    // world data
    let tiles   = data::get_tiles();
    let enemies = data::get_enemies();

    // window settings
    let window_setup = WindowSetup::default().title("Rust Game");
    let window_mode = WindowMode::default()
        .dimensions(WIN_WIDTH, WIN_HEIGHT)
        .resizable(false);

    let (ctx, event_loop) = ContextBuilder::new("Rust Game", "Person")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()?;

    // set initial stat&e
    let state = State::new(&ctx, WIN_WIDTH / 2.0, WIN_HEIGHT / 2.0, 1, 1, tiles, enemies);

    // run game
    event::run(ctx, event_loop, state);
}

use crate::{
    images::Images,
    player::Player,
    tiles::{Tile, Tiles},
    enemies::{Enemy, Enemies},
};

use ggez::{
    Context, GameResult, GameError,
    graphics::{Canvas, DrawParam, Color},
    event::EventHandler,
    mint::Point2,
};

// different directions
#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Default for Direction {
    // get default direction
    fn default() -> Self {
        Self::Up
    }
}

// game state
pub struct State {
    // player state
    player: Player,

    // tiles to draw
    tiles: Tiles,

    // game enemies
    enemies: Enemies,

    // preloaded images
    images: Images,
}

impl State {
    // new state struct
    pub fn new(ctx: &Context, pos_x: f32, pos_y: f32, level_x: usize, level_y: usize, tiles: Vec<Vec<Vec<Vec<Tile>>>>, enemies: Vec<Vec<Vec<Enemy>>>) -> Self {
        State {
            tiles:   Tiles::new(tiles),
            enemies: Enemies::new(enemies),
            player:  Player::new(pos_x, pos_y, level_x, level_y),
            images:  Images::new(ctx),
        }
    }
}

impl EventHandler<GameError> for State {
    // update state
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // update player
        self.player.update(ctx, &mut self.tiles, &self.enemies);

        for enemy in self.enemies.cur_enemies_mut(self.player.level_x, self.player.level_y).iter_mut() {
            enemy.move_towards(self.player.pos_x, self.player.pos_y, self.player.level_x, self.player.level_y, &self.tiles);
        }

        Ok(())
    }

    // draw player and tiles
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // draw canvas
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        let param = DrawParam::default().dest(
            Point2 { x: 0.0, y: 0.0 },
        );

        // draw background
        canvas.draw(&self.images.background, param);

        // draw tiles
        self.tiles.draw(&mut canvas, &self.images, self.player.level_x, self.player.level_y);

        // draw enemies
        self.enemies.draw(&mut canvas, &self.images, self.player.level_x, self.player.level_y);

        // draw player
        self.player.draw(&mut canvas, &self.images);

        // draw health
        self.player.health.draw(&mut canvas, &self.images);

        canvas.finish(ctx)?;
        Ok(())
    }
}

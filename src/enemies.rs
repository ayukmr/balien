use crate::{
    images::Images,
    data::TILE_SIZE,
    tiles::Tiles,
};

use ggez::{
    graphics::{Canvas, DrawParam},
    mint::Point2,
};

use pathfinding::prelude::astar;

// game enemy
pub struct Enemy {
    // position
    pos_x: f32,
    pos_y: f32,
}

impl Enemy {
    // create enemy struct
    pub fn new(pos_x: f32, pos_y: f32) -> Self {
        Self { pos_x, pos_y }
    }

    // get current position
    pub fn position(&self) -> (f32, f32) {
        (self.pos_x, self.pos_y)
    }

    // move towards player
    pub fn get_successors(pos_x: f32, pos_y: f32, level_x: usize, level_y: usize, tiles: &Tiles) -> Vec<((i32, i32), i32)> {
        let mut points = Vec::new();

        for speed_x in 4..=5 {
            for speed_y in 4..=5 {
                let speed_x = speed_x as f32;
                let speed_y = speed_y as f32;

                points.push((-speed_x / 2f32.sqrt(), -speed_y / 2f32.sqrt()));
                points.push((0.0, -5.0));
                points.push((speed_x / 2f32.sqrt(), -speed_y / 2f32.sqrt()));
                points.push((-speed_x, 0.0));
                points.push((speed_x, 0.0));
                points.push((-speed_x / 2f32.sqrt(), speed_y / 2f32.sqrt()));
                points.push((0.0, 5.0));
                points.push((speed_x / 2f32.sqrt(), speed_y / 2f32.sqrt()));
            }
        }

        let mut successors = Vec::new();
        let level_tiles = tiles.cur_tiles(level_x, level_y);

        for (delta_x, delta_y) in points {
            let mut successor = true;

            for (y, row) in level_tiles.iter().enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    if !tile.collision() {
                        continue;
                    }

                    if Self::is_colliding(
                        pos_x, pos_y,
                        delta_x, delta_y,
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                        TILE_SIZE,
                    ) {
                        successor = false;
                    }
                }
            }

            if successor {
                successors.push((((pos_x + delta_x) as i32, (pos_y + delta_y) as i32), 1));
            }
        }

        successors
    }

    pub fn move_towards(&mut self, player_x: f32, player_y: f32, level_x: usize, level_y: usize, tiles: &Tiles) {
        let goal = (player_x as i32, player_y as i32);

        let result = astar(
            &(self.pos_x as i32, self.pos_y as i32),
            |(x, y)| Self::get_successors(*x as f32, *y as f32, level_x, level_y, &tiles),
            |(x, y)| ((x - goal.0).abs() + (y - goal.1).abs()) / 3,
            |p| *p == goal,
        );

        let results = result.unwrap().0;

        if results.len() > 2 {
            let position = results[1];

            self.pos_x = position.0 as f32;
            self.pos_y = position.1 as f32;
        }
    }

    // check if colliding with collider
    pub fn is_colliding(pos_x: f32, pos_y: f32, delta_x: f32, delta_y: f32, col_x: f32, col_y: f32, col_size: f32) -> bool {
        // check if colliding horizontally
        let colliding_x =
            (pos_x + delta_x >= col_x &&
             pos_x + delta_x <= col_x + col_size) ||
            (pos_x + delta_x + col_size >= col_x &&
             pos_x + delta_x + col_size <= col_x + col_size);

        // check if colliding vertically
        let colliding_y =
            (pos_y + delta_y >= col_y &&
             pos_y + delta_y <= col_y + col_size) ||
            (pos_y + delta_y + col_size >= col_y &&
             pos_y + delta_y + col_size <= col_y + col_size);

        colliding_x && colliding_y
    }
}

// world enemies
pub struct Enemies {
    // vector of tiles
    pub enemies: Vec<Vec<Vec<Enemy>>>,
}

impl Enemies {
    // create enemies struct
    pub fn new(enemies: Vec<Vec<Vec<Enemy>>>) -> Self {
        Enemies { enemies }
    }

    // get vector of enemies
    pub fn cur_enemies(&self, level_x: usize, level_y: usize) -> &Vec<Enemy> {
        &self.enemies[level_y][level_x]
    }

    // get vector of mutable enemies
    pub fn cur_enemies_mut(&mut self, level_x: usize, level_y: usize) -> &mut Vec<Enemy> {
        &mut self.enemies[level_y][level_x]
    }

    // draw enemies
    pub fn draw(&self, canvas: &mut Canvas, images: &Images, level_x: usize, level_y: usize) {
        let enemies = self.cur_enemies(level_x, level_y);

        for enemy in enemies.iter() {
            // enemy position
            let (x, y) = enemy.position();

            // enemy image
            let image = &images.enemy_slime;

            let param =
                DrawParam::default().dest(Point2 { x, y });

            // draw enemy
            canvas.draw(image, param);
        }
    }
}

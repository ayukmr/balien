use crate::{
    data::{WIN_WIDTH, WIN_HEIGHT, TILE_SIZE, SPEED},
    state::Direction,
    images::Images,
    health::Health,
    tiles::{Tile, Tiles},
    enemies::Enemies,
};

use ggez::{
    Context,
    graphics::{Canvas, DrawParam},
    input::keyboard::KeyCode,
    mint::Point2,
};

pub struct Player {
    // position
    pub pos_x: f32,
    pub pos_y: f32,

    // level position
    pub level_x: usize,
    pub level_y: usize,

    // iframes in frames
    iframes: u16,

    // current health
    pub health: Health,

    // current direction
    direction: Direction,
}

impl Player {
    // create player struct
    pub fn new(pos_x: f32, pos_y: f32, level_x: usize, level_y: usize) -> Self {
        Self {
            pos_x,
            pos_y,

            level_x,
            level_y,

            iframes: 0,

            health:    Health::new(),
            direction: Direction::default(),
        }
    }

    // change current level
    pub fn change_level(&mut self, delta_x: isize, delta_y: isize) {
        self.level_x = (self.level_x as isize + delta_x) as usize;
        self.level_y = (self.level_y as isize + delta_y) as usize;
    }

    // check if colliding with collider
    pub fn is_colliding(&self, col_x: f32, col_y: f32, col_size: f32, delta_x: f32, delta_y: f32) -> bool {
        // check if colliding horizontally
        let colliding_x =
            (self.pos_x + delta_x >= col_x &&
             self.pos_x + delta_x <= col_x + col_size) ||
            (self.pos_x + delta_x + col_size >= col_x &&
             self.pos_x + delta_x + col_size <= col_x + col_size);

        // check if colliding vertically
        let colliding_y =
            (self.pos_y + delta_y >= col_y &&
             self.pos_y + delta_y <= col_y + col_size) ||
            (self.pos_y + delta_y + col_size >= col_y &&
             self.pos_y + delta_y + col_size <= col_y + col_size);

        colliding_x && colliding_y
    }

    // take damage
    pub fn damage(&mut self, amount: u8) {
        if self.iframes == 0 {
            // take damage
            self.health.damage(amount);

            if self.health.health() <= 0 {
                self.health.reset();

                self.pos_x = WIN_WIDTH  / 2.0;
                self.pos_y = WIN_HEIGHT / 2.0;
            } else {
                // set invincibility period
                self.iframes = 50;
            }
        }
    }

    // update player
    pub fn update(&mut self, ctx: &Context, tiles: &mut Tiles, enemies: &Enemies) {
        // update iframes
        if self.iframes > 0 {
            self.iframes -= 1;
        }

        // current speed
        let mut delta_x = 0.0;
        let mut delta_y = 0.0;

        // horizontal movement
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            delta_x -= SPEED;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            delta_x += SPEED;
        }

        // vertical movement
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            delta_y -= SPEED;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            delta_y += SPEED;
        }

        // normalize diagonal speed
        if delta_x != 0.0 && delta_y != 0.0 {
            delta_x /= 2f32.sqrt();
            delta_y /= 2f32.sqrt();
        }

        // set player direction
        if delta_x < 0.0 {
            self.direction = Direction::Left;
        } else if delta_x > 0.0 {
            self.direction = Direction::Right;
        } else if delta_y < 0.0 {
            self.direction = Direction::Up;
        } else if delta_y > 0.0 {
            self.direction = Direction::Down;
        }

        let level_tiles   = tiles.cur_tiles(self.level_x, self.level_y);
        let level_enemies = enemies.cur_enemies(self.level_x, self.level_y);

        let mut level_delta_x = 0;
        let mut level_delta_y = 0;

        let mut door_direction = &Direction::default();

        for (y, row) in level_tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if !tile.collision() {
                    continue;
                }

                if self.is_colliding(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, TILE_SIZE, delta_x, delta_y) {
                    // stop moving if colliding
                    delta_x = 0.0;
                    delta_y = 0.0;

                    if tile.damaging() {
                        // take damage from tile
                        self.damage(1);
                    }

                    match tile {
                        Tile::Door(direction) => {
                            door_direction = direction;

                            match direction {
                                Direction::Left  => level_delta_x = -1,
                                Direction::Right => level_delta_x =  1,
                                Direction::Up    => level_delta_y = -1,
                                Direction::Down  => level_delta_y =  1,
                            };
                        }
                        _ => {}
                    }
                }
            }
        }

        for enemy in level_enemies.iter() {
            let (x, y) = enemy.position();

            if self.is_colliding(x, y, TILE_SIZE, delta_x, delta_y) {
                self.damage(1);
            }
        }

        if level_delta_x != 0 || level_delta_y != 0 {
            // change level
            self.change_level(level_delta_x, level_delta_y);

            // change position to spawn
            (self.pos_x, self.pos_y) = tiles.get_spawn(self.level_x, self.level_y, door_direction);
        }

        // change position using speed
        self.pos_x += delta_x;
        self.pos_y += delta_y;
    }

    // draw player
    pub fn draw(&self, canvas: &mut Canvas, images: &Images) {
        // player image
        let image = match self.direction {
            Direction::Left  => &images.player_left,
            Direction::Right => &images.player_right,
            Direction::Up    => &images.player_up,
            Direction::Down  => &images.player_down,
        };

        let param = DrawParam::default().dest(
            Point2 { x: self.pos_x, y: self.pos_y },
        );

        // draw image
        canvas.draw(image, param);
    }
}

use crate::{
    data::TILE_SIZE,
    state::Direction,
    images::Images,
};

use ggez::{
    graphics::{Canvas, DrawParam},
    mint::Point2,
};

// tile types
pub enum Tile {
    Background,
    Bricks,
    Spikes,
    Spawn(Direction),
    Door(Direction),
}

impl Tile {
    // check if tile should be drawn
    pub fn drawable(&self) -> bool {
        match self {
            Tile::Background | Tile::Spawn(_) => false,
            _ => true,
        }
    }

    // check if tile has collision
    pub fn collision(&self) -> bool {
        match self {
            Tile::Bricks | Tile::Spikes | Tile::Door(_) => true,
            _ => false,
        }
    }

    // check if tile is damaging
    pub fn damaging(&self) -> bool {
        match self {
            Tile::Spikes => true,
            _ => false,
        }
    }
}

// world tiles
pub struct Tiles {
    // vector of tiles
    tiles: Vec<Vec<Vec<Vec<Tile>>>>,
}

impl Tiles {
    // create tiles struct
    pub fn new(tiles: Vec<Vec<Vec<Vec<Tile>>>>) -> Self {
        Tiles { tiles }
    }

    // get vector of tiles
    pub fn cur_tiles(&self, level_x: usize, level_y: usize) -> &Vec<Vec<Tile>> {
        &self.tiles[level_y][level_x]
    }

    // get spawn point for level
    pub fn get_spawn(&self, level_x: usize, level_y: usize, door_direction: &Direction) -> (f32, f32) {
        let level = &self.tiles[level_y][level_x];

        for (y, row) in level.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    // get spawn for door direction
                    Tile::Spawn(direction) => {
                        if direction == door_direction {
                            return (x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
                        }
                    }
                    _ => {}
                }
            }
        }

        panic!("no spawn point found");
    }

    // draw tiles
    pub fn draw(&self, canvas: &mut Canvas, images: &Images, level_x: usize, level_y: usize) {
        let level = self.cur_tiles(level_x, level_y);

        for (y, row) in level.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                // don't draw background
                if !tile.drawable() {
                    continue;
                }

                // tile image
                let image = match tile {
                    Tile::Bricks  => &images.tile_bricks,
                    Tile::Spikes  => &images.tile_spikes,
                    Tile::Door(_) => &images.tile_door,
                    _ => unreachable!(),
                };

                let param = DrawParam::default().dest(
                    Point2 {
                        x: x as f32 * TILE_SIZE,
                        y: y as f32 * TILE_SIZE,
                    },
                );

                // draw tile
                canvas.draw(image, param);
            }
        }
    }
}

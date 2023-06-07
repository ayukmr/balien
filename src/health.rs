use crate::images::Images;

use ggez::{
    graphics::{Canvas, DrawParam},
    mint::Point2,
};

use std::cmp::min;

pub struct Health {
    // health
    health: u8,
    max_health: u8,
}

impl Health {
    // create health struct
    pub fn new() -> Self {
        Self {
            health: 5,
            max_health: 5,
        }
    }

    // get current health
    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn reset(&mut self) {
        self.health = self.max_health;
    }

    // regenerate hearts
    pub fn _regen(&mut self, amount: u8) {
        self.health += min(amount, self.max_health);
    }

    // take damage
    pub fn damage(&mut self, amount: u8) {
        self.health -= amount;
    }

    // draw hearts
    pub fn draw(&self, canvas: &mut Canvas, images: &Images) {
        for x in 0..self.max_health {
            // heart location
            let param = DrawParam::default().dest(
                Point2 { x: x as f32 * 100.0 + 25.0, y: 25.0 },
            );

            // draw image
            canvas.draw(
                if x < self.health {
                    &images.heart_full
                } else {
                    &images.heart_slot
                },
                param,
            );
        }
    }
}

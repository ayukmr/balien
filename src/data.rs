use crate::{
    tiles::Tile::{
        self,

        Background as Bkgnd,
        Bricks     as Brcks,
        Spikes     as Spike,

        Spawn as Sp,
        Door  as Dr,
    },

    state::Direction::{
        Left  as L,
        Right as R,
        Up    as U,
        Down  as D,
    },

    enemies::Enemy,
};

// window dimensions
pub const WIN_WIDTH:  f32 = 2560.0;
pub const WIN_HEIGHT: f32 = 1600.0;

// tile size
pub const TILE_SIZE: f32 = 80.0;

// player speed
pub const SPEED: f32 = 8.5;

// get enemies
pub fn get_enemies() -> Vec<Vec<Vec<Enemy>>> {
    let enemies_00 = vec![];
    let enemies_10 = vec![];
    let enemies_20 = vec![];
    let enemies_01 = vec![];
    let enemies_11 = vec![Enemy::new(1536.0, 320.0)];
    let enemies_21 = vec![];
    let enemies_02 = vec![];
    let enemies_12 = vec![];
    let enemies_22 = vec![];

    vec![
        vec![enemies_00, enemies_10, enemies_20],
        vec![enemies_01, enemies_11, enemies_21],
        vec![enemies_02, enemies_12, enemies_22],
    ]
}

// get tiles
pub fn get_tiles() -> Vec<Vec<Vec<Vec<Tile>>>> {
    let level_00 = vec![
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(L), Bkgnd, Dr(R)],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(U), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Dr(D), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
    ];

    let level_10 = vec![
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Dr(L), Bkgnd, Sp(R), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(L), Bkgnd, Dr(R)],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(U), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Dr(D), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
    ];

    let level_20 = vec![
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Dr(L), Bkgnd, Sp(R), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(U), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Dr(D), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
    ];

    let level_01 = vec![
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Dr(U), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(D), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(L), Bkgnd, Dr(R)],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(U), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Dr(D), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
    ];

    let level_11 = vec![
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Dr(U), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Brcks, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Spike, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(D), Bkgnd, Bkgnd, Brcks, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Spike, Spike, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Spike, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Spike, Spike, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Brcks, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Brcks, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Dr(L), Bkgnd, Sp(R), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(L), Bkgnd, Dr(R)],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Brcks, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Brcks, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Spike, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Brcks, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Spike, Spike, Bkgnd, Brcks, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Brcks, Brcks, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Brcks, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Bkgnd, Bkgnd, Spike, Spike, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Brcks, Bkgnd, Bkgnd, Spike, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(U), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Brcks, Brcks, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Dr(D), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
    ];

    let level_21 = vec![
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Dr(U), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(D), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Dr(L), Bkgnd, Sp(R), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(U), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Dr(D), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
    ];

    let level_02 = vec![
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Dr(U), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(D), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(L), Bkgnd, Dr(R)],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
    ];

    let level_12 = vec![
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Dr(U), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(D), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Dr(L), Bkgnd, Sp(R), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(L), Bkgnd, Dr(R)],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
    ];

    let level_22 = vec![
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Dr(U), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Sp(D), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Dr(L), Bkgnd, Sp(R), Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
        vec![Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd, Bkgnd],
    ];

    vec![
        vec![level_00, level_10, level_20],
        vec![level_01, level_11, level_21],
        vec![level_02, level_12, level_22],
    ]
}

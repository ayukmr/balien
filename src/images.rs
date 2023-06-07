use ggez::{
    Context,
    graphics::Image,
};

// preloaded images
pub struct Images {
    // player image
    pub player_left:  Image,
    pub player_right: Image,
    pub player_up:    Image,
    pub player_down:  Image,

    // tile images
    pub tile_bricks: Image,
    pub tile_spikes: Image,
    pub tile_door:   Image,

    // enemy images
    pub enemy_slime: Image,

    // heart images
    pub heart_full: Image,
    pub heart_slot: Image,

    // background image
    pub background: Image,
}

impl Images {
    pub fn new(ctx: &Context) -> Self {
        Self {
            player_left:  Images::get(ctx, "/player/left.png"),
            player_right: Images::get(ctx, "/player/right.png"),
            player_up:    Images::get(ctx, "/player/up.png"),
            player_down:  Images::get(ctx, "/player/down.png"),

            tile_bricks: Images::get(ctx, "/tiles/bricks.png"),
            tile_spikes: Images::get(ctx, "/tiles/spikes.png"),
            tile_door:   Images::get(ctx, "/tiles/door.png"),

            enemy_slime: Images::get(ctx, "/enemy/slime.png"),

            heart_full: Images::get(ctx, "/hearts/full.png"),
            heart_slot: Images::get(ctx, "/hearts/slot.png"),

            background: Images::get(ctx, "/misc/background.png"),
        }
    }

    fn get(ctx: &Context, path: &str) -> Image {
        Image::from_path(ctx, path, true).unwrap()
    }
}

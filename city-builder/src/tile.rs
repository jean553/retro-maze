//! Map tile.
//!
//! TODO: use a structure instead of a single sprite for now, as a tile might contains a lot of more information in the future.

#[derive(Copy, Clone)]
enum Sprite {
    DEFAULT,
}

#[derive(Copy, Clone)]
pub struct Tile {
    sprite: Sprite, 
}

impl Tile {

    /// Creates a new tile.
    pub fn new() -> Tile {
        Tile {
            sprite: Sprite::DEFAULT,
        }
    }
}

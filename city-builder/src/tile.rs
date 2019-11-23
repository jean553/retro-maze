//! Map tile.
//!
//! TODO: use a structure instead of a single sprite for now, as a tile might contains a lot of more information in the future.

#[derive(Copy, Clone)]
pub struct Tile {
    sprite: usize,
}

impl Tile {

    /// Creates a new tile.
    pub fn new() -> Tile {
        Tile { sprite: 0 }
    }

    /// Sets the sprite value.
    ///
    /// Args:
    ///
    /// `sprite` - the sprite index to display from the full available sprites list
    pub fn set_sprite(
        &mut self,
        sprite: usize,
    ) {
        self.sprite = sprite;
    }

    /// Returns the sprite value of the tile.
    pub fn get_sprite(&self) -> usize {
        self.sprite
    }
}

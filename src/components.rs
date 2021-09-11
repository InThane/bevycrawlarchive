pub use crate::prelude::*;

#[derive(Clone, Copy, Debug,PartialEq)]
/// Struct containing information about an entity to be rendered on the screen.
pub struct Render {
    pub color : ColorPair,
    pub glyph : FontCharType
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Struct containing data about the player entity.
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;
use crate::prelude::*;

/// Indicates the maximum number of tiles in a single map
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq)]
/// Enum that contains the tiles used by maps.
pub enum TileType {
    Wall,
    Floor,
}

/// Structure containing a vector of tiles
pub struct Map {
    pub tiles: Vec<TileType>,
}

/// Convert x,y to a map index value that works with the Map vector.
pub fn map_idx(
    x: i32, 
    y: i32
) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    /// Create a new blank map
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES] // Create a vector of size NUM_TILES filled with Floors
        }
    }

    /// Bounds checker for Map object
    ///
    /// Returns True if in bounds, False if not
    pub fn in_bounds(
        &self,
        point : Point
    ) -> bool {
        point.x >= 0 && 
        point.x < SCREEN_WIDTH &&
        point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /// Determines if a tile can be entered.
    pub fn can_enter_tile(
        &self,
        point : Point,
    ) -> bool {
        self.in_bounds(point)
            && self.tiles[map_idx(point.x, point.y)]==TileType::Floor
    }

    /// Returns a map index to the point if the point is valid, otherwise returns none.
    pub fn try_idx(
        &self,
        point : Point
    ) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
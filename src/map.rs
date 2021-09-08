use crate::prelude::*;

/// # NUM_TILES
/// Indicates the maximum number of tiles in a single map
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq)]
/// # TileType
/// Enum that contains the tiles used by maps.
pub enum TileType {
    Wall,
    Floor,
}

/// # Map
/// Structure containing a vector of tiles
pub struct Map {
    pub tiles: Vec<TileType>,
}

/// # map_idx
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

    /// # render
    /// Renders the map object to the linked BTerm.
    pub fn render(
        &self, 
        ctx: &mut BTerm
    ) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x,y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(
                            x, 
                            y, 
                            YELLOW, 
                            BLACK, 
                            to_cp437('.')
                        );
                    }
                    TileType::Wall => {
                        ctx.set(
                            x,
                            y,
                            GREEN,
                            BLACK,
                            to_cp437('#')
                        );
                    }
                }
            }
        }
    }

    /// # in_bounds
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

    /// # can_enter_tile
    /// Determines if a tile can be entered.
    pub fn can_enter_tile(
        &self,
        point : Point,
    ) -> bool {
        self.in_bounds(point)
            && self.tiles[map_idx(point.x, point.y)]==TileType::Floor
    }

    /// # try_idx
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


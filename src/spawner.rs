use crate::prelude::*;

/// Spawns the player object at the given position.
pub fn spawn_player(ecs : &mut World, pos : Point) {
    ecs.push(
        (
            Player,
            pos,
            Render{
                color: ColorPair::new(DF_FG_COLOR, DF_BG_COLOR),
                glyph: to_cp437('@')
            }
        )
    );
}

/// Spawns a random monster at the given point.
pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos: Point
) {
    ecs.push(
        (Enemy,
            pos,
            Render{
                color: ColorPair::new(DF_FG_COLOR, DF_BG_COLOR),
                glyph: match rng.range(0,4) {
                    0=> to_cp437('E'),
                    1=> to_cp437('O'),
                    2=> to_cp437('o'),
                    _=> to_cp437('g'),
                }
            }
        )
    );
}
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
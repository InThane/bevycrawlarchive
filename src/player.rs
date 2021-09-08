use crate::prelude::*;

/// # Player
/// The structure that contains player information
pub struct Player {
    pub position: Point
}

impl Player {
    pub fn new(
        position: Point
    ) -> Self {
        Self {
            position
        }
    }

    /// # render
    /// Renders the player character in the proper position on the BTerm.
    pub fn render(
        &self, 
        ctx: &mut BTerm,
        camera: &Camera
    ) {
        ctx.set_active_console(1);
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    /// # update
    /// Moves the player character based on input. Checks to make sure
    /// the space is a valid move destination before changing the position.
    pub fn update(
        &mut self, 
        ctx: &mut BTerm, 
        map : &Map,
        camera: &mut Camera
    ) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left  | VirtualKeyCode::H => Point::new(-1, 0),
                VirtualKeyCode::Right | VirtualKeyCode::L => Point::new(1, 0),
                VirtualKeyCode::Up    | VirtualKeyCode::K => Point::new(0, -1),
                VirtualKeyCode::Down  | VirtualKeyCode::J => Point::new(0, 1),
                _ => Point::zero(),
            };
            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
                camera.on_player_move(new_position);
            }
        }
    }
}
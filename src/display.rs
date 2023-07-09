use core::f32::consts::PI;

use libm::cosf;

use crate::{map::Map, state::State};

pub struct Display;

const COLUMNS: usize = 160;
const FOV: f32 = PI / 2.7; // The player's field of view.
const HALF_POV: f32 = FOV * 0.5; // Half the player's field of view.
const ANGLE_STEP: f32 = FOV / COLUMNS as f32; // The angle between each ray, one ray per pixel column.
const WALL_HEIGHT: f32 = 100.0; // The height of the wall at 1 unit away from the player.

impl Display {
    // Returns wall heights from the player's perspective, 1 wall height per pixel column
    pub fn get_view(player: &State, map: &Map) -> [(i32, bool); COLUMNS] {
        // The player's FOV is split in half by their viewing angle.
        // In order to get the ray's first angle we must
        // add half of the FOV to the player's angle to get
        // the edge of the player's FOV.
        let starting_angle = player.player_angle + HALF_POV;

        let mut walls = [(0, false); COLUMNS];

        for (idx, wall) in walls.iter_mut().enumerate() {
            // `idx` is what number ray we are, `wall` is
            // a mutable reference to a value in `walls`.
            let angle = starting_angle - idx as f32 * ANGLE_STEP;

            // Get both the closest horizontal and vertical wall
            // intersection for this angle.
            let h_dist = map.horizontal_intersection(player, angle);
            let v_dist = map.vertical_intersection(player, angle);

            let (min_dist, shadow) = if h_dist < v_dist {
                (h_dist, false)
            } else {
                (v_dist, true)
            };

            // Get the minimum of the two distances and
            // "convert" it into a wall height.
            *wall = (
                (WALL_HEIGHT / (min_dist * cosf(angle - player.player_angle))) as i32,
                shadow,
            );
        }

        walls
    }
}

use libm::{cosf, sinf};

use crate::{
    gamepad::{Gamepad, HorizontalDirection, VerticalDirection},
    map::Map,
};

const STEP_SIZE: f32 = 0.045;

pub struct State {
    pub player_x: f32,
    pub player_y: f32,
    pub player_angle: f32,
}

impl State {
    // Move the character
    pub fn update(&mut self, map: &Map, gamepad: &Gamepad) {
        let previous_position = (self.player_x, self.player_y);

        match gamepad.vertical {
            VerticalDirection::Up => {
                self.player_x += cosf(self.player_angle) * STEP_SIZE;
                self.player_y += -sinf(self.player_angle) * STEP_SIZE;
            }
            VerticalDirection::Down => {
                self.player_x -= cosf(self.player_angle) * STEP_SIZE;
                self.player_y -= -sinf(self.player_angle) * STEP_SIZE;
            }
            VerticalDirection::Neutral => {}
        };

        match gamepad.horizontal {
            HorizontalDirection::Left => self.player_angle += STEP_SIZE,
            HorizontalDirection::Right => self.player_angle -= STEP_SIZE,
            HorizontalDirection::Neutral => {}
        };

        // If moving us on this frame puts us into a wall just revert it
        if map.point_in_wall(self.player_x, self.player_y) {
            (self.player_x, self.player_y) = previous_position;
        }
    }
}

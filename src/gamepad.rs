const GAMEPAD1: *const u8 = 0x16 as *const u8;

const BUTTON_LEFT: u8 = 16; // 00010000
const BUTTON_RIGHT: u8 = 32; // 00100000
const BUTTON_UP: u8 = 64; // 01000000
const BUTTON_DOWN: u8 = 128; // 10000000

pub enum HorizontalDirection {
    Left,
    Right,
    Neutral,
}

pub enum VerticalDirection {
    Up,
    Down,
    Neutral,
}

pub struct Gamepad {
    pub vertical: VerticalDirection,
    pub horizontal: HorizontalDirection,
}

impl Gamepad {
    pub fn gamepad1_state() -> Gamepad {
        let up;
        let down;
        let left;
        let right;
        unsafe {
            up = *GAMEPAD1 & BUTTON_UP != 0;
            down = *GAMEPAD1 & BUTTON_DOWN != 0;
            left = *GAMEPAD1 & BUTTON_LEFT != 0;
            right = *GAMEPAD1 & BUTTON_RIGHT != 0;
        }

        let vertical = match (up, down) {
            (true, false) => VerticalDirection::Up,
            (false, true) => VerticalDirection::Down,
            _ => VerticalDirection::Neutral,
        };

        let horizontal = match (left, right) {
            (true, false) => HorizontalDirection::Left,
            (false, true) => HorizontalDirection::Right,
            _ => HorizontalDirection::Neutral,
        };

        Gamepad {
            vertical,
            horizontal,
        }
    }
}

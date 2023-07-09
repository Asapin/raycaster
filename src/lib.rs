#![no_std]

use core::{arch::wasm32, panic::PanicInfo};

use display::Display;
use gamepad::Gamepad;
use map::Map;
use state::State;

mod display;
mod gamepad;
mod map;
mod state;

const DRAW_COLORS: *mut u16 = 0x14 as *mut u16;

static mut STATE: State = State {
    player_x: 1.5,
    player_y: 1.5,
    player_angle: 0.0,
};

const MAP: Map = Map {
    layout: [
        0b1111111111111111,
        0b1000001010000101,
        0b1011100000110101,
        0b1000111010010001,
        0b1010001011110111,
        0b1011101001100001,
        0b1000100000001101,
        0b1111111111111111,
    ],
};

extern "C" {
    fn vline(x: i32, y: i32, len: u32);
}

#[panic_handler]
fn phandler(_: &PanicInfo<'_>) -> ! {
    wasm32::unreachable();
}

#[no_mangle]
unsafe fn update() {
    let gamepad1 = Gamepad::gamepad1_state();
    STATE.update(&MAP, &gamepad1);

    // go through each column on screen and draw walls in the center.
    for (x, wall) in Display::get_view(&STATE, &MAP).iter().enumerate() {
        let (height, shadow) = wall;
        if *shadow {
            // draw with color 2 for walls with "shadow"
            *DRAW_COLORS = 0x2;
        } else {
            // draw with color 3 for walls without "shadow"
            *DRAW_COLORS = 0x3;
        }

        vline(x as i32, 80 - (height / 2), *height as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

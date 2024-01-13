use raylib::prelude::*;
use std::thread::Thread;

const SQUARE_SIZE: i32 = 50;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Heyy").build();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::ORANGE);

        // let mut x_offset = 0;
        // let mut y_offset = 0;
        let is_even = |pos: usize| pos % 2 == 0;
        let is_odd = |pos: usize| !is_even(pos);
        let tile_color = |x: usize, y: usize| {
            if (is_odd(x) && is_odd(y)) || (is_even(x) && is_even(y)) {
                Color::WHITE
            } else {
                Color::GRAY
            }
        };
        assert!(is_even(2));
        for row in 0..8 {
            for col in 0..8 {
                let y_offset = col * SQUARE_SIZE;
                let x_offset = row * SQUARE_SIZE;
                let color = tile_color(row as usize, col as usize);
                d.draw_rectangle(x_offset, y_offset, SQUARE_SIZE, SQUARE_SIZE, color);
            }
        }
        // d.draw_rectangle(0, 0, SQUARE_SIZE, SQUARE_SIZE, Color::WHITE);
        // d.draw_rectangle(0, 100, SQUARE_SIZE, SQUARE_SIZE, Color::BLACK);
    }
}

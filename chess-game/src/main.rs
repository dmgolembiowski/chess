#![allow(unused_imports)]
use anyhow::Result;
use chess_core::{
    self,
    constants::*,
    game::{self, math},
    helper, msg,
    traits::*,
    types,
};
use crossbeam_channel::unbounded;
use crossbeam_utils::thread::scope;
use raylib::prelude::*;

const SQUARE_SIZE: i32 = 80;


fn get_y_from_col(col: i32) -> usize {
    match col {
        0 => 7,
        1 => 6,
        2 => 5,
        3 => 4,
        4 => 3,
        5 => 2,
        6 => 1,
        7 => 0,
        _ => panic!("Unintended usage"),
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(1080, 720).title("Funky Chess").build();
    let mut gm = chess_core::spawn_game_master();
    let game_id: chess_core::msg::GameId = gm.create_game().unwrap();

    let is_even = |pos: usize| pos % 2 == 0;
    let is_odd = |pos: usize| !is_even(pos);
    let tile_color = |x: usize, y: usize| {
        if (is_odd(x) && is_odd(y)) || (is_even(x) && is_even(y)) {
            Color::WHITE
        } else {
            Color::GRAY
        }
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        for row in 0..8 {
            for col in 0..8 {
                let y_offset = col * SQUARE_SIZE;
                let x_offset = row * SQUARE_SIZE;
                let color = tile_color(row as usize, col as usize);
                d.draw_rectangle(x_offset, y_offset, SQUARE_SIZE, SQUARE_SIZE, color);
                // If a chess piece exists at the XyPair corresponding to the given
                // row and column value, draw it here.
                let x = row;
                let y = get_y_from_col(col);
                d.draw_text(
                    &format!("({x}, {y})"),
                    x_offset,
                    y_offset,
                    16,
                    Color::BLACK,
                );
            }
        }
        // To prevent the board from being reset after every single turn
        /*
        'game: loop {

        }
        */
    }

    // Ok(())
}

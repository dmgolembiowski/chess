use chess_core::{
    game::{self, math::{self, XyPair}},
};
pub const SQUARE_SIZE: i32 = 96;

pub(crate) fn get_y_from_col(col: i32) -> usize {
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

pub(crate) fn xy_to_row_col(&XyPair { x, y }: &XyPair) -> (i32, i32) {
    let x = x as i32;
    let y = get_y_from_col(y as i32) as i32;
    (x, y)
}



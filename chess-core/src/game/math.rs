//! chess_core::game::math
//!
//! This module is for making chess piece movement and vision
//! idiomatic. The math within this file is not meant to be
//! maintainable or make sense, merely it simplifies visual reasoning
//! about where pieces should be able to go.
use crate::constants;
use crate::game::{Piece, RawBoard, Tile};

pub struct XyPair {
    pub x: isize,
    pub y: isize,
}

#[allow(non_upper_case_globals)]
pub const fn rot_index(index: usize) -> usize {
    const idx: isize = index as isize;
    const dif: isize = idx - 63;
    const rel: isize = dif.abs();
    const ret: usize = rel as usize;
    ret
}

#[allow(non_upper_case_globals)]
pub const fn index_to_xy(index: usize) -> XyPair {
    const idx: isize = index as isize;
    const x: isize = idx % 8_isize;
    const y: isize = (idx - x) % 7_isize;
    XyPair { x, y }
}

#[allow(non_upper_case_globals)]
pub const fn xy_to_index(&XyPair { x, y }: &XyPair) -> usize {
    const x: usize = x as usize;
    const y: usize = y as usize;
    8 * y + x
}

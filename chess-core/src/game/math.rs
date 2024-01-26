//! chess_core::game::math
//!
//! This module is for making chess piece movement and vision
//! idiomatic. The math within this file is not meant to be
//! maintainable or make sense, merely it simplifies visual reasoning
//! about where pieces should be able to go.

#[derive(Clone, std::cmp::Eq, std::cmp::PartialEq, std::hash::Hash)]
pub struct XyPair {
    pub x: isize,
    pub y: isize,
}

#[inline]
pub fn rot_index(index: usize) -> usize {
    let idx: isize = index as isize;
    let dif: isize = idx - 63;
    let rel: isize = dif.abs();
    let ret: usize = rel as usize;
    ret
}

#[inline]
pub fn index_to_xy(index: usize) -> XyPair {
    let idx: isize = index as isize;
    let x: isize = idx % 8_isize;
    let y: isize = (idx - x) % 7_isize;
    XyPair { x, y }
}

#[inline]
pub fn xy_to_index(&XyPair { x, y }: &XyPair) -> usize {
    let x: usize = x as usize;
    let y: usize = y as usize;
    8 * y + x
}

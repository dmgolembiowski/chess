//! chess_core::game::math
//!
//! This module is for making chess piece movement and vision
//! idiomatic. The math within this file is not meant to be
//! maintainable or make sense, merely it simplifies visual reasoning
//! about where pieces should be able to go.

#[derive(Debug, Clone, Copy, std::cmp::Eq, std::cmp::PartialEq, std::hash::Hash)]
pub struct XyPair {
    pub x: isize,
    pub y: isize,
}

impl From<(usize, usize)> for XyPair {
    fn from(it: (usize, usize)) -> XyPair {
        let (x, y): (isize, isize) = (it.0 as isize, it.1 as isize);
        XyPair { x, y }
    }
}

use std::convert::TryFrom;
impl TryFrom<(isize, isize)> for XyPair {
    type Error = String;
    fn try_from(it: (isize, isize)) -> Result<XyPair, Self::Error> {
        if it.0 + it.1 < 0 {
            return Result::Err(format!(
                "Cannot use negative coordinates: ({}, {})",
                &it.0, &it.1
            ));
        }
        let (x, y): (isize, isize) = (it.0, it.1);
        Ok(XyPair { x, y })
    }
}

use std::cmp::{Ord, Ordering, PartialOrd};

use crate::msg::TileId;

impl Ord for XyPair {
    fn cmp(&self, other: &Self) -> Ordering {
        let mysize = xy_to_index(*self);
        let theirsize = xy_to_index(*other);
        match (mysize, theirsize) {
            (this, that) if this == that => Ordering::Equal,
            (this, that) if this > that => Ordering::Greater,
            (this, that) if this < that => Ordering::Less,
            (_, _) => unreachable!(),
        }
    }
}

impl PartialOrd for XyPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[inline]
pub fn rot_index(index: TileId) -> usize {
    let idx: isize = index as isize;
    let dif: isize = idx - 63;
    let rel: isize = dif.abs();
    let ret: usize = rel as usize;
    ret
}

#[inline]
pub fn index_to_xy(index: TileId) -> XyPair {
    match index {
        0 => (0, 0).into(),
        1 => (1, 0).into(),
        2 => (2, 0).into(),
        3 => (3, 0).into(),
        4 => (4, 0).into(),
        5 => (5, 0).into(),
        6 => (6, 0).into(),
        7 => (7, 0).into(),
        8 => (0, 1).into(),
        9 => (1, 1).into(),
        10 => (2, 1).into(),
        11 => (3, 1).into(),
        12 => (4, 1).into(),
        13 => (5, 1).into(),
        14 => (6, 1).into(),
        15 => (7, 1).into(),
        16 => (0, 2).into(),
        17 => (1, 2).into(),
        18 => (2, 2).into(),
        19 => (3, 2).into(),
        20 => (4, 2).into(),
        21 => (5, 2).into(),
        22 => (6, 2).into(),
        23 => (7, 2).into(),
        24 => (0, 3).into(),
        25 => (1, 3).into(),
        26 => (2, 3).into(),
        27 => (3, 3).into(),
        28 => (4, 3).into(),
        29 => (5, 3).into(),
        30 => (6, 3).into(),
        31 => (7, 3).into(),
        32 => (0, 4).into(),
        33 => (1, 4).into(),
        34 => (2, 4).into(),
        35 => (3, 4).into(),
        36 => (4, 4).into(),
        37 => (5, 4).into(),
        38 => (6, 4).into(),
        39 => (7, 4).into(),
        40 => (0, 5).into(),
        41 => (1, 5).into(),
        42 => (2, 5).into(),
        43 => (3, 5).into(),
        44 => (4, 5).into(),
        45 => (5, 5).into(),
        46 => (6, 5).into(),
        47 => (7, 5).into(),
        48 => (0, 6).into(),
        49 => (1, 6).into(),
        50 => (2, 6).into(),
        51 => (3, 6).into(),
        52 => (4, 6).into(),
        53 => (5, 6).into(),
        54 => (6, 6).into(),
        55 => (7, 6).into(),
        56 => (0, 7).into(),
        57 => (1, 7).into(),
        58 => (2, 7).into(),
        59 => (3, 7).into(),
        60 => (4, 7).into(),
        61 => (5, 7).into(),
        62 => (6, 7).into(),
        63 => (7, 7).into(),
        _ => panic!("Semantic error giving an out of bounds index: {}", &index),
    }
}

#[inline]
pub fn xy_to_index(xypair: XyPair) -> TileId {
    let XyPair { x, y } = xypair;
    match (x, y) {
        (0, 0) => 0,
        (1, 0) => 1,
        (2, 0) => 2,
        (3, 0) => 3,
        (4, 0) => 4,
        (5, 0) => 5,
        (6, 0) => 6,
        (7, 0) => 7,
        (0, 1) => 8,
        (1, 1) => 9,
        (2, 1) => 10,
        (3, 1) => 11,
        (4, 1) => 12,
        (5, 1) => 13,
        (6, 1) => 14,
        (7, 1) => 15,
        (0, 2) => 16,
        (1, 2) => 17,
        (2, 2) => 18,
        (3, 2) => 19,
        (4, 2) => 20,
        (5, 2) => 21,
        (6, 2) => 22,
        (7, 2) => 23,
        (0, 3) => 24,
        (1, 3) => 25,
        (2, 3) => 26,
        (3, 3) => 27,
        (4, 3) => 28,
        (5, 3) => 29,
        (6, 3) => 30,
        (7, 3) => 31,
        (0, 4) => 32,
        (1, 4) => 33,
        (2, 4) => 34,
        (3, 4) => 35,
        (4, 4) => 36,
        (5, 4) => 37,
        (6, 4) => 38,
        (7, 4) => 39,
        (0, 5) => 40,
        (1, 5) => 41,
        (2, 5) => 42,
        (3, 5) => 43,
        (4, 5) => 44,
        (5, 5) => 45,
        (6, 5) => 46,
        (7, 5) => 47,
        (0, 6) => 48,
        (1, 6) => 49,
        (2, 6) => 50,
        (3, 6) => 51,
        (4, 6) => 52,
        (5, 6) => 53,
        (6, 6) => 54,
        (7, 6) => 55,
        (0, 7) => 56,
        (1, 7) => 57,
        (2, 7) => 58,
        (3, 7) => 59,
        (4, 7) => 60,
        (5, 7) => 61,
        (6, 7) => 62,
        (7, 7) => 63,
        _ => panic!("Semantic error giving an XyPair existing out of bounds"),
    }
}

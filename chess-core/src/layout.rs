use crate::game::math::{self, XyPair};
use crate::types::Piece;
use crate::types::Tile;
use crate::GameState;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::{Rc, Weak};

pub struct Layout<'a> {
    pub data: BTreeMap<XyPair, &'a Tile>,
}

impl<'a> Layout<'a> {
    #[inline]
    pub fn generate(game: &'a GameState) -> Self {
        let data = {
            let mut xy_to_tile: BTreeMap<XyPair, &Tile> = BTreeMap::new();
            for idx in 0..game.board.len() {
                let xy = math::index_to_xy(idx);
                if let Some(t) = &mut xy_to_tile.insert(xy, &game.board[idx]) {
                    eprintln!("Oh no, overwrote {t:#?} on {idx}");
                    panic!();
                } else {
                    continue;
                }
            }
            xy_to_tile
        };
        Self { data }
    }
}

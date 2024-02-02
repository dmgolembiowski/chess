use crate::types::Tile;
use crate::game::math::{self, XyPair};
use crate::GameState;
use std::collections::BTreeMap;

pub struct Layout<'a> {
    pub data: BTreeMap<XyPair, &'a Tile>, 
}

impl<'a> Layout<'a> {
    pub fn generate(game: &'a GameState) -> Self {
        let data = {
            let mut xy_to_tile: BTreeMap<XyPair, &Tile> = <_>::default();
            for tile in &game.board {
                let xy = math::index_to_xy(tile.index);
                let _ = xy_to_tile.insert(xy, &tile);
            }
            xy_to_tile 
        };
        Self { data }
    }
}


pub mod math;

use crate::msg::{PieceId, PlayerId, TileId};
use crate::types::{Color, Move, Piece, RawBoard, Tile, Type, VisionPiece};
use crate::{constants, types};
use anyhow::{anyhow, bail, Result};
// use serde::{Deserialize, Serialize};
// use serde_with::serde_as;
use std::{cell::RefCell, rc::Rc};

use self::math::XyPair;

// #[derive(Debug, Serialize, Deserialize)]
// #[serde_as]
#[derive(Debug, Clone)]
pub struct GameState {
    pub started: bool,
    pub finished: bool,
    pub p1_clock: Option<u32>,
    pub p2_clock: Option<u32>,
    pub p1: PlayerData,
    pub p2: PlayerData,
    // #[serde_as(as = "[_; constants::TILECOUNT]")]
    pub board: RawBoard,
    pub hist: History,
}

impl GameState {
    pub fn new() -> Self {
        let board: [Tile; constants::TILECOUNT] = crate::helper::chess_board();
        Self {
            started: false,
            finished: false,
            p1_clock: None,
            p2_clock: None,
            p1: PlayerData::default(),
            p2: PlayerData::default(),
            board,
            hist: History::default(),
        }
    }
    pub fn init(
        started: bool,
        finished: bool,
        p1_clock: Option<u32>,
        p2_clock: Option<u32>,
        p1: PlayerData,
        p2: PlayerData,
        board: RawBoard,
        hist: History,
    ) -> Self {
        Self {
            started,
            finished,
            p1_clock,
            p2_clock,
            p1,
            p2,
            board,
            hist,
        }
    }
    pub fn piece_by_id(&self, piece_id: &PieceId) -> Option<Rc<RefCell<Piece>>> {
        match *piece_id {
            pid @ 1..=16 => {
                for rc in &self.p1.pieces {
                    if rc.borrow().id == pid {
                        return Some(Rc::clone(&rc));
                    }
                }
                None
            }
            pid @ -16..=-1 => {
                for rc in &self.p2.pieces {
                    if rc.borrow().id == pid {
                        return Some(Rc::clone(&rc));
                    }
                }
                None
            }
            _ => None,
        }
    }
    pub fn calculate_vision(
        &self,
        piece: Rc<RefCell<Piece>>,
        board: &types::RawBoard,
    ) -> Result<VisionPiece> {
        let p = piece.borrow();
        let invert: bool = if &p.color == &Color::Black {
            true
        } else {
            false
        };
        if invert {
            bail!("not yet implemented: handling simple cases first");
        }
        // let XyPair { x, y } = crate::game::math::index_to_xy(p.loc);
        match p.ty {
            Type::Bishop => Err(anyhow!("Bishop movement not available yet")),
            Type::King => Err(anyhow!("King movement not yet available")),
            Type::Knight => Err(anyhow!("Knight movement not available")),
            Type::Pawn => {
                let nil = Move::new_nil(&piece);
                let f1 = Move::forward(&piece, 1);
                let f2 = Move::forward(&piece, 2);
                let moves = [nil, f1, f2]; // Ok(VisionPiece::new_with_moves(p.id, &[nil, f1, f2]))

                let piece_id: PieceId = ((*piece.clone()).borrow()).id;
                Ok(VisionPiece::new_with_moves(piece_id, moves))
                // Needs to be a Result<VisionPiece>
            }
            Type::Queen => Err(anyhow!("Queen movement not yet available")),
            Type::Rook => Err(anyhow!("Rook movemvent not available")),
        }
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct History {
    id: String,
    pub actions: Vec<Action>,
}
impl History {
    pub fn init(id: impl Into<String>) -> Self {
        let id = id.into();
        let actions: Vec<Action> = vec![];
        Self { id, actions }
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub enum Action {
    #[default]
    Nil,
    FixPlayerData,
    SetActivePlayer(PlayerId),
    Move(*mut u8),
}

#[derive(Default, Debug, Clone)]
pub struct PlayerData {
    pub color: Color,
    pub name: String,
    pub pieces: Vec<Rc<RefCell<Piece>>>,
}

impl PlayerData {
    pub fn incomplete_init(
        color: Color,
        name: Option<String>,
        pieces: Option<Vec<Rc<RefCell<Piece>>>>,
    ) -> Self {
        let name = if let Some(thing) = name {
            thing
        } else {
            match color {
                Color::White => String::from("player_1"),
                Color::Black => String::from("player_2"),
            }
        };
        let pieces = if let Some(thing) = pieces {
            thing
        } else {
            Vec::<Rc<RefCell<Piece>>>::new()
        };
        Self {
            color,
            name,
            pieces,
        }
    }
    pub fn new_white_player() -> Self {
        let pieces: Vec<Rc<RefCell<Piece>>> = Vec::with_capacity(16);
        PlayerData {
            color: Color::White,
            name: "player_1".to_string(),
            pieces,
        }
    }
    pub fn new_black_player() -> Self {
        let pieces: Vec<Rc<RefCell<Piece>>> = Vec::with_capacity(16);
        PlayerData {
            color: Color::Black,
            name: "player_2".to_string(),
            pieces,
        }
    }
    fn add_piece(&mut self, pz: Rc<RefCell<Piece>>) {
        self.pieces.push(pz);
    }
}

pub fn add_piece(
    board: &mut RawBoard,
    idx: usize,
    player: &mut PlayerData,
    mut pz: Piece,
) -> Result<()> {
    assert_eq!(board.as_ref().len(), 64);
    use crate::constants::TILECOUNT;
    assert_eq!(pz.color, player.color, "Forbidden enemy piece assignment");
    assert!(idx < TILECOUNT, "Out of bounds tile");
    if let Some(found) = &board[idx].pz {
        anyhow::bail!("Tried to overwrite an existing piece: {found:?}");
    }
    pz.update_loc(idx);
    let owned = Rc::new(RefCell::new(pz));
    let share = Some(Rc::downgrade(&owned));
    board[idx].pz = share;
    player.add_piece(owned);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{ChessFactory, StandardChess};
    use crate::types::*;

    #[test]
    fn black_queen_test() {
        let bq = Some(Piece {
            color: Color::Black,
            ty: Type::Queen,
            id: -5_i16,
            loc: <usize as Default>::default(),
        });
        bq.unwrap();
    }

    /*
    #[ignore = "Temporarily breaking serialization to allow movement serialization"]
    #[test]
    fn bq_json() {
        use serde_json;
        let it = r#"{"id":-4,"color":"Black","ty":"Queen","loc":0}"#;
        let bq = Piece {
            color: Color::Black,
            ty: Type::Queen,
            id: -4_i16,
            loc: <usize as Default>::default(),
        };
        assert_eq!(it, serde_json::to_string(&bq).unwrap().as_str());
    }
    */
    #[ignore = "Skipping new game until prerequisites are done"]
    #[test]
    fn create_new_local_game() {
        fn new_local_game() -> Result<()> {
            anyhow::bail!("Not yet implemented");
        }
        let _game = new_local_game().unwrap();
    }

    #[ignore = "Depends on prerequisite of populating the chess board"]
    #[test]
    fn game_construct() {
        let mut p1 = PlayerData::new_white_player();
        let mut p2 = PlayerData::new_black_player();
    }

    #[test]
    fn queen_color_matches_background() {
        // ChessWorld.com says that on the initial board setup
        // of a standard game, the color of the queen should equivalent
        // to their background tile shade.
        use crate::helper::chess_board;
        use chess_derive::{ChessFactory, StandardChess};
        let mut player_w = PlayerData::new_white_player();
        let mut player_b = PlayerData::new_black_player();

        #[allow(non_camel_case_types)]
        #[derive(StandardChess, ChessFactory)]
        struct ChessBoard;

        // Pregenerate the pieces before allocating them to players and tiles
        let w: [Piece; 16] = ChessBoard::gen_std_white();
        let b: [Piece; 16] = ChessBoard::gen_std_black();

        let mut board = chess_board();
        // For each player:
        // Clone the location index from the piece,
        // Wrap the piece in a Rc<RefCell<_>>,
        // create a weak refcount from the &Rc,
        // assign the weak variant to the `board`,
        // save the owned one to the player
        for white_pz in w.into_iter() {
            let pos: TileId = white_pz.loc.clone();
            let res: Result<()> = add_piece(&mut board, pos, &mut player_w, white_pz);
            assert!(res.is_ok());
        }
        for black_pz in b.into_iter() {
            let pos: TileId = black_pz.loc.clone();
            let res: Result<()> = add_piece(&mut board, pos, &mut player_b, black_pz);
            assert!(res.is_ok());
        }

        // Now for the actual test:
        // We're supplying the indices which should hold our white and black queens. slay gaga and
        // beyonce.
        use constants::{D1, D8};
        const ERR_REASON: &str = "Failed to match queen color with tile background";

        let beyonce = Rc::new(RefCell::new(ChessBoard::queen_black(D8, -5)));
        let beyonce_shared = Some(Rc::clone(&beyonce));
        let mut bt = Tile::dark(D8, false, true);

        bt.update_piece(beyonce_shared, true);

        assert_eq!(
            beyonce.clone().borrow().color,
            board[D8].color,
            "{}",
            ERR_REASON
        );

        let gaga = Rc::new(RefCell::new(ChessBoard::queen_white(D1, 4)));
        let gaga_shared = Some(Rc::clone(&gaga));
        let mut gt = Tile::light(D1, true, false);
        assert!(gt.pz.is_none());
        assert!(gt.update_piece(gaga_shared, true).is_ok());
    }
}

pub mod math;

use crate::constants;
use crate::types::{Color, Piece, RawBoard, Tile};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::{cell::RefCell, rc::Rc};

// #[derive(Debug, Serialize, Deserialize)]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameState {
    started: bool,
    finished: bool,
    p1_clock: Option<u32>,
    p2_clock: Option<u32>,
    p1: PlayerData,
    p2: PlayerData,
    #[serde_as(as = "[_; constants::TILECOUNT]")]
    board: RawBoard,
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
}

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct History {
    id: String,
    moves: Vec<Action>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    #[default]
    Nil,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerData {
    pub color: Color,
    pub name: String,
    pieces: Vec<Rc<RefCell<Piece>>>,
}

impl PlayerData {
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
            loc: <usize as Default>::default(),
        });
        bq.unwrap();
    }

    #[test]
    fn bq_json() {
        use serde_json;
        let it = r#"{"color":"Black","ty":"Queen","loc":0}"#;
        let bq = Piece {
            color: Color::Black,
            ty: Type::Queen,
            loc: <usize as Default>::default(),
        };
        assert_eq!(it, serde_json::to_string(&bq).unwrap().as_str());
    }

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
            let pos: usize = white_pz.loc.clone();
            let res: Result<()> = add_piece(&mut board, pos, &mut player_w, white_pz);
            assert!(res.is_ok());
        }
        for black_pz in b.into_iter() {
            let pos: usize = black_pz.loc.clone();
            let res: Result<()> = add_piece(&mut board, pos, &mut player_b, black_pz);
            assert!(res.is_ok());
        }

        // Now for the actual test:
        // We're supplying the indices which should hold our white and black queens. slay gaga and
        // beyonce.
        use constants::{D1, D8};
        const ERR_REASON: &str = "Failed to match queen color with tile background";

        let beyonce = Rc::new(RefCell::new(ChessBoard::queen_black(D8)));
        let beyonce_shared = Some(Rc::clone(&beyonce));
        let mut bt = Tile::dark(D8, false, true);

        bt.update_piece(beyonce_shared);

        assert_eq!(
            beyonce.clone().borrow().color,
            board[D8].color,
            "{}",
            ERR_REASON
        );

        let gaga = Rc::new(RefCell::new(ChessBoard::queen_white(D1)));
        let gaga_shared = Some(Rc::clone(&gaga));
        let mut gt = Tile::light(D1, true, false);

        gt.update_piece(gaga_shared);

        assert_eq!(
            gaga.clone().borrow().color,
            board[D1].color,
            "{}",
            ERR_REASON
        );
    }
}

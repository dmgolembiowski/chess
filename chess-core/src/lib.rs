pub mod constants;
pub mod game;
pub mod helper;
pub mod msg;
pub mod traits;
pub mod types;

use crate::msg::{GameId, PieceId};
use crate::traits::{ChessFactory, StandardChess};
use crate::types::VisionPiece;
use anyhow::Result;
use chess_derive::ChessFactory;
use chess_derive::StandardChess;
use game::GameState;
use std::thread;
use std::{collections::BTreeMap, sync::atomic::AtomicU64};

pub fn spawn_game_master() -> GameMaster {
    GameMaster::new()
}

pub struct GameMaster {
    indexer: Box<dyn Fn() -> GameId>,
    sessions: BTreeMap<GameId, ChessGame>,
}

impl GameMaster {
    pub fn new() -> Self {
        let indexer = Box::new(|| {
            use std::sync::atomic::Ordering;
            static GAME_ID: AtomicU64 = AtomicU64::new(0);
            let x = GAME_ID.load(Ordering::Relaxed);
            if x == u64::MAX {
                GAME_ID.store(0, Ordering::Relaxed);
            }
            let _ = GAME_ID.fetch_add(1, Ordering::Relaxed);
            GAME_ID.load(Ordering::Relaxed)
        });
        let sessions = BTreeMap::new();
        Self { indexer, sessions }
    }

    pub fn create_game(&mut self) -> Result<GameId> {
        let game_id = (self.indexer)();
        let new_game = ChessGame::new(game_id)?;
        let _ = self.sessions.insert(game_id, new_game);
        Ok(game_id)
    }

    pub fn request_game_state<'a, 'b>(&'a self, game_id: GameId) -> Result<&'b ChessGame>
    where
        'a: 'b,
    {
        if let Some(ref_game) = self.sessions.get(&game_id) {
            Ok(&ref_game)
        } else {
            anyhow::bail!("Game with {game_id} not found");
        }
    }

    pub fn request_vision(&self, game_id: GameId, piece_id: PieceId) -> Result<VisionPiece> {
        let chess = self.request_game_state(game_id)?;
        chess.request_vision(piece_id)
    }
}

#[derive(StandardChess, ChessFactory)]
pub struct ChessGame {
    pub game_id: u64,
    pub game: GameState,
}

impl ChessGame {
    #[allow(non_upper_case_globals)]
    pub fn new(game_id: u64) -> Result<Self> {
        use crate::{
            game::{add_piece, History, PlayerData},
            helper::chess_board,
        };

        let (started, finished): (bool, bool) = (false, false);
        let (p1_clock, p2_clock): (Option<u32>, Option<u32>) = (Some(5400000), Some(5400000));

        let mut p1 = PlayerData::new_white_player();
        let mut p2 = PlayerData::new_black_player();

        let w = <Self as StandardChess>::gen_std_white();
        let b = <Self as StandardChess>::gen_std_black();

        let mut board = chess_board();

        let mut ids_white = IntoIterator::into_iter(1_i16..=16_i16);
        let mut ids_black = IntoIterator::into_iter(-16_i16..=-1_i16).rev();

        for mut white_piece in w.into_iter() {
            let id = unsafe { &mut ids_white.next().unwrap_unchecked() };
            let _ = &mut white_piece.set_id(*id);
            add_piece(&mut board, white_piece.loc, &mut p1, white_piece)?;
        }

        for mut black_piece in b.into_iter() {
            let id = &mut ids_black.next().unwrap();
            let _ = &mut black_piece.set_id(*id);
            add_piece(&mut board, black_piece.loc, &mut p2, black_piece)?;
        }

        let hist = History::init(format!("History of Game {}", &stringify!(&game_id)));
        let game = GameState::init(started, finished, p1_clock, p2_clock, p1, p2, board, hist);

        Ok(Self { game_id, game })
    }

    pub fn request_vision(&self, piece_id: PieceId) -> Result<VisionPiece> {
        // First thing we're going to do is ask our GameState for a
        // reference to the piece corresponding to the PieceId we specify
        use crate::types::Piece;
        use std::{cell::RefCell, rc::Rc};
        let piece: Option<Rc<RefCell<Piece>>> = self.game.piece_by_id(&piece_id);
        if let Some(rc) = piece {
            self.game.calculate_vision(rc)
        } else {
            Err(anyhow::anyhow!("Piece not found: {piece_id}"))
        }
    }
}

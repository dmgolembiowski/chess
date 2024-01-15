#[allow(unused)]
pub mod constants;
pub mod game;
pub mod helper;
pub mod msg;
pub mod traits;
pub mod types;

use crate::traits::{ChessFactory, StandardChess};
use anyhow::Result;
use chess_derive::ChessFactory;
use chess_derive::StandardChess;
use game::GameState;
use std::{collections::BTreeMap, sync::atomic::AtomicU64};

pub struct GameMaster {
    indexer: Box<dyn Fn() -> u64>,
    sessions: BTreeMap<u64, ChessGame>,
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
    pub fn create_game(&mut self) -> Result<u64> {
        let game_id = (self.indexer)();
        let new_game = ChessGame::new(game_id)?;
        let _ = self.sessions.insert(game_id, new_game);
        Ok(game_id)
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
        const started: bool = false;
        const finished: bool = false;
        const p1_clock: Option<u32> = Some(5400000);
        const p2_clock: Option<u32> = Some(5400000);
        let mut p1 = PlayerData::new_white_player();
        let mut p2 = PlayerData::new_black_player();
        let w = <Self as StandardChess>::gen_std_white();
        let b = <Self as StandardChess>::gen_std_black();
        let mut board = chess_board();

        for white_piece in w.into_iter() {
            add_piece(&mut board, white_piece.loc.clone(), &mut p1, white_piece)?;
        }

        for black_piece in b.into_iter() {
            add_piece(&mut board, black_piece.loc.clone(), &mut p2, black_piece)?;
        }
        let hist = History::init(format!("History of Game {}", &stringify!(&game_id)));
        let game = GameState::init(started, finished, p1_clock, p2_clock, p1, p2, board, hist);
        Ok(Self { game_id, game })
    }
}

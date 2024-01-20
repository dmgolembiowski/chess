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

    fn request_vision(&self, piece_id: PieceId) -> Result<VisionPiece> {
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

#[test]
fn new_game_has_32_pieces() {
    let mut gm = spawn_game_master();
    let game_id = gm.create_game().unwrap();
    let game: Result<&'_ ChessGame> = gm.request_game_state(game_id);
    assert!(&game.is_ok(), "Failed to create a game");
    {
        let mut count: usize = 0;
        let state = game.unwrap().clone();
        count += state.game.p1.pieces.len();
        assert_eq!(&count, &16_usize, "p1 needs 16 pieces");
        count += state.game.p2.pieces.len();
        assert_eq!(count, 32, "incorrect board composition");
    }
}

#[test]
fn opening_white_pawn_mvmt() {
    todo!("White pawns should only move forwards by one or two spaces");
}

#[test]
fn opening_black_pawn_mvmt() {
    todo!("Black pawns should relatively move forwards, but their destination tile's index must be less than their starting index.");
}

#[test]
fn pawn_moves_two_spaces_only_once() {
    todo!("Y +/- 2 movement is only allowed for the pawn's first move");
}

#[ignore = "Future"]
#[test]
fn pawn_cannot_pass_thru_unless_en_passant() {
    todo!("Filter available movement by reducing options that allow blindspot passthru");
}

#[ignore = "Future"]
#[test]
fn pawn_en_passant_black_captures_white() {
    /* "When a pawn advances two squares on its initial move and
     * ends the turn adjacent to an enemy pawn on the same [row], it may be captured
     * _en passant_ by the enemy pawn as if it had moved only one square.
     *
     * This capture is legal only on the move immediately following the pawn's advance."
     */
    todo!(
        "Construct a valid game state, \
        verify en passant capture is possible, \
        and compare actual outcome against the expectation"
    );
}

#[ignore = "Future"]
#[test]
fn pawn_en_passant_white_captures_black() {
    /* "When a pawn advances two squares on its initial move and
     * ends the turn adjacent to an enemy pawn on the same [row], it may be captured
     * _en passant_ by the enemy pawn as if it had moved only one square.
     *
     * This capture is legal only on the move immediately following the pawn's advance."
     */
    todo!(
        "Construct a valid game state, \
        verify en passant capture is possible, \
        and compare actual outcome against the expectation"
    );
}

#[test]
fn knight_movement_can_pass_thru() {
    todo!("Allow knight passthru");
}

#[test]
fn prevent_accidental_knight_capturing_friendly_tile() {
    todo!("The superset of knight move options should be reduced so that ally tiles cannot be captured.");
}

#[test]
fn bishops_move_diagonally() {
    todo!("Program bishop movement");
}

#[test]
fn rooks_move_cardinally() {
    todo!("Program rook movement");
}

#[test]
fn kings_move_like_queen_eigenvectors() {
    todo!("Kings can move one tile in any non-L-shaped direction");
}

#[test]
fn queens_can_move_as_either_a_bishop_or_rook() {
    todo!("Program queen movement");
}

#[ignore = "Future"]
#[test]
fn resolve_before_end_of_game_reached_otherwise_panic() {
    todo!(
        "At the start of a turn, panic if an enemy king remains in a state of check or checkmate."
    );
}

#[ignore = "Future"]
#[test]
fn cannot_capture_king_only_threaten_check_or_checkmate() {
    todo!("Directly capturing the king should never happen");
}

#[test]
fn pawn_captures_forward_left_and_forward_right() {
    todo!("Allow pawn to capture correctly");
}

#[test]
fn bishop_queen_rook_movement_to_first_tile_in_any_direction_only() {
    todo!(
        "BQRs vision should be reduced such that they stop at the first capture in a given X-Y ray"
    );
}

#[test]
fn bishop_queen_rook_stop_before_ally_tile_aka_no_passthru() {
    todo!("Knight passthru privilege does not apply to queen, bishop, nor rook.");
}

#[ignore = "Future"]
#[test]
fn simple_white_pawn_promotion() {
    todo!("Non-interaction pawn promotion to queen succeeds");
}

#[ignore = "Future"]
#[test]
fn pawn_promotion_following_diagonal_capture() {
    todo!("Pawn promotion after capturing an enemy piece at endzone succeeds");
}

#[test]
fn simple_king_movement() {
    todo!("Ignoring check/checkmate logic, king can move to any defined surrounding tile");
}

#[ignore = "Future"]
#[test]
fn move_update_includes_check_info() {
    todo!("the act of sending the move should also convey one or more rays that make the capture possible");
}

#[ignore = "Future"]
#[test]
fn sacrificial_piece_to_protect_king_works() {
    todo!("Allow players to move their piece to protect the king from an existing enemy check");
}

#[ignore = "Future"]
#[test]
fn show_but_forbid_movement_that_exposes_check_or_checkmate() {
    todo!("Mark a move or capture that induces a check");
}

#[ignore = "Future"]
#[test]
fn raise_error_submitting_moves_that_cause_new_checks_or_new_checkmate() {
    todo!("Intentionally submit a move and confirm it yields an error");
}

#[ignore = "Future"]
#[test]
fn pawn_promotion_recalculates_check_on_enemy_king() {
    todo!(
        "Since pawn promotion radically alters the game state \
    the logic should take care to allow suppressing automatic game ending branches \
    from being reached since the player hasn't had a chance to respond to the promotion. \
    There ought to be an in-depth checkmate procedure run at the start of the turn. IFF this fails \
    to find a way out should the game end abruptly."
    );
}

#[ignore = "Future"]
#[test]
fn white_kingside_castling_works() {
    todo!("...");
}

#[ignore = "Future"]
#[test]
fn black_kingside_castling_works() {
    todo!("...");
}

#[ignore = "Future"]
#[test]
fn white_queenside_castling_works() {
    todo!("...");
}

#[ignore = "Future"]
#[test]
fn black_queenside_castling_works() {
    todo!("...");
}

#[ignore = "Future"]
#[test]
fn white_kingside_castling_works_unless_inducing_check() {
    todo!("...");
}

#[ignore = "Future"]
#[test]
fn black_kingside_castling_works_unless_inducing_check() {
    todo!("...");
}

#[ignore = "Future"]
#[test]
fn white_queenside_castling_works_unless_inducing_check() {
    todo!("...");
}

#[ignore = "Future"]
#[test]
fn black_queenside_castling_works_unless_inducing_check() {
    todo!("...");
}

#[ignore = "Future"]
#[test]
fn verify_unavailable_castling_because_rook_moved_already() {
    todo!("Remove possibility to perform castle maneuver b/c illegal move");
}

#[ignore = "Future"]
#[test]
fn verify_unavailable_castling_because_king_moved_already() {
    todo!(
        "Verify that the move representing a castling maneuver is not found \
    in the set of movement options on the king or the rook."
    );
}

#[ignore = "Future"]
#[test]
fn forbid_castling_when_king_in_check() {
    todo!("Remove (and verify) the king being unable to castle if it is in check or checkmate.");
}

#[ignore = "Future"]
#[test]
fn castling_requires_empty_tiles_between_king_and_rook() {
    todo!("Construct a context where castling is legal and see if the option to perform it exists");
}

#[ignore = "Future"]
#[test]
fn vision_cannot_exceed_endzone_or_sidelines() {
    todo!("Movement options existing beyond the 8x8 tile bounds cannot be valid");
}

#[ignore = "Future"]
#[test]
fn allow_forfeit_before_checkmate() {
    todo!("Players can concede the game");
}

#[ignore = "Future"]
#[test]
fn request_tie_from_opponent() {
    todo!(
        "Either player can request a draw from the opponent \
    such that agreeing ends the game."
    );
}

#[ignore = "Future"]
#[test]
fn disable_spamming_tie_plz() {
    todo!("Players can only request a tie twice.");
}

#[ignore = "Future"]
#[test]
fn answering_tie_request_does_not_eat_timer() {
    todo!("Pause both timers when either player requests a draw");
}

#[ignore = "Future"]
#[test]
fn checkmate_concessions_and_drawing_disable_further_movement() {
    todo!("... as well as freeze the clocks");
}

#[ignore = "Future"]
#[test]
fn time_elapsing_mutates_active_players_clock_only() {
    todo!("Use concurrent synchronization to accurately decrement a player's chess clock");
}

#[ignore = "Future"]
#[test]
fn pausing_game_suspends_all_clocks() {
    todo!("Verify that time remaining on clocks remains the same at the start and end of pausing");
}

#[ignore = "Future"]
#[test]
fn running_out_of_time_triggers_forfeit() {
    todo!("When a player's clock reaches zero, submit a forfeit action and do not accept more movement submissions");
}

#[ignore = "Future"]
#[test]
fn draw_when_game_reaches_dead_position_from_king_vs_king() {
    /* A dead position is defined as a position where
     * neither player can checkmate their opponent's king by any
     * sequence of legal moves.
     * Although in practice players sometimes play on in dead positions,
     * according to the rules of chess the game is immediately terminated
     * the moment a dead position appears on the board.
     */
    todo!(
        "Construct a contrived board state and submit a move that induces \
    the indicated kind of dead position. Verify that the game is immediately over."
    );
}

#[ignore = "Future"]
#[test]
fn draw_when_game_reaches_dead_position_from_king_vs_king_and_bishop() {
    /* A dead position is defined as a position where
     * neither player can checkmate their opponent's king by any
     * sequence of legal moves.
     * Although in practice players sometimes play on in dead positions,
     * according to the rules of chess the game is immediately terminated
     * the moment a dead position appears on the board.
     */
    todo!(
        "Construct a contrived board state and submit a move that induces \
    the indicated kind of dead position. Verify that the game is immediately over."
    );
}

#[ignore = "Future"]
#[test]
fn draw_when_game_reaches_dead_position_from_king_vs_king_and_knight() {
    /* A dead position is defined as a position where
     * neither player can checkmate their opponent's king by any
     * sequence of legal moves.
     * Although in practice players sometimes play on in dead positions,
     * according to the rules of chess the game is immediately terminated
     * the moment a dead position appears on the board.
     */
    todo!(
        "Construct a contrived board state and submit a move that induces \
    the indicated kind of dead position. Verify that the game is immediately over."
    );
}

#[ignore = "Future"]
#[test]
fn draw_when_game_reaches_dead_position_from_king_and_dark_bishop_vs_king_and_dark_bishop() {
    /* A dead position is defined as a position where
     * neither player can checkmate their opponent's king by any
     * sequence of legal moves.
     * Although in practice players sometimes play on in dead positions,
     * according to the rules of chess the game is immediately terminated
     * the moment a dead position appears on the board.
     */
    todo!(
        "Construct a contrived board state and submit a move that induces \
    the indicated kind of dead position. Verify that the game is immediately over."
    );
}

#[ignore = "Future"]
#[test]
fn draw_when_game_reaches_dead_position_from_king_and_light_bishop_vs_king_and_light_bishop() {
    /* A dead position is defined as a position where
     * neither player can checkmate their opponent's king by any
     * sequence of legal moves.
     * Although in practice players sometimes play on in dead positions,
     * according to the rules of chess the game is immediately terminated
     * the moment a dead position appears on the board.
     */
    todo!(
        "Construct a contrived board state and submit a move that induces \
    the indicated kind of dead position. Verify that the game is immediately over."
    );
}

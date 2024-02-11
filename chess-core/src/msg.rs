#[allow(unused_imports)]
use crate::{constants, game::GameState, helper, traits, types};
// use serde::{Deserialize, Serialize};

pub type GameId = u64;

// Positively valued integers are black (like the stock market).
// Negatively valued ones are white.
pub type PieceId = i16;
pub type TileId = usize;
pub type MoveOp = usize;
pub type Class = usize;

// [`PLAYER`'s']() boolean value coincides with
// the statement "this is player 2". If the value is value,
// then the player data in question involves player 1 / white.
pub type PlayerId = bool;

#[derive(Clone, Debug, Copy, PartialEq)]
pub(crate) enum CliMsg {
    Ping,
    Pong,
    Exit,
    Forfeit,
    NewGame,
    NewGameLan,
    NewGameInet,
    LookCheckmate,
    GotoMenu,
    Spectate(GameId),
    ReqGameState(GameId),
    ReqVisionAll(GameId),
    ReqVisionAllP1(GameId),
    ReqVisionAllP2(GameId),
    GotoGame((GameId, PlayerId)),
    EndTurn((GameId, PlayerId)),
    ReqVisionPiece((GameId, PieceId)),
    Promote((GameId, PieceId, Class)),
    Move(
        (
            GameId,
            /* is_player2: */ PlayerId,
            /* piece id: */ PieceId,
            /* move_op: */ MoveOp,
        ),
    ),
}

#[derive(Clone, Debug)]
pub(crate) enum Response {
    Ping,
    Pong,
    // NewGame,
    // NewGameLan,
    // NewGameInet,
    RenderUpdate,
    GameCreated(GameId),
    GameState(GameState),
}

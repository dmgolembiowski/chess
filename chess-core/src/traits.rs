use crate::msg::PieceId;
use crate::msg::TileId;
use crate::types::*;

pub trait ChessFactory {
    fn init(ty: Type, color: Color, loc: TileId, id: PieceId) -> Piece {
        Piece { color, ty, loc, id }
    }
    fn pawn_white(loc: TileId, id: PieceId) -> Piece {
        <Self as ChessFactory>::init(Type::Pawn, Color::White, loc, id)
    }
    fn rook_white(loc: TileId, id: PieceId) -> Piece {
        <Self as ChessFactory>::init(Type::Rook, Color::White, loc, id)
    }
    fn knight_white(loc: TileId, id: PieceId) -> Piece {
        <Self as ChessFactory>::init(Type::Knight, Color::White, loc, id)
    }
    fn bishop_white(loc: TileId, id: PieceId) -> Piece {
        <Self as ChessFactory>::init(Type::Bishop, Color::White, loc, id)
    }
    fn king_white(loc: TileId, id: PieceId) -> Piece {
        <Self as ChessFactory>::init(Type::King, Color::White, loc, id)
    }
    fn queen_white(loc: TileId, id: PieceId) -> Piece {
        <Self as ChessFactory>::init(Type::Queen, Color::White, loc, id)
    }
    fn pawn_black(loc: TileId, id: PieceId) -> Piece {
        <Self as ChessFactory>::init(Type::Pawn, Color::Black, loc, id)
    }
    fn rook_black(loc: TileId, id: PieceId) -> Piece {
        <Self as ChessFactory>::init(Type::Rook, Color::Black, loc, id)
    }
    fn knight_black(loc: TileId, id: PieceId) -> Piece {
        <Self as ChessFactory>::init(Type::Knight, Color::Black, loc, id)
    }
    fn bishop_black(loc: TileId, id: PieceId) -> Piece {
        <Self as ChessFactory>::init(Type::Bishop, Color::Black, loc, id)
    }
    fn king_black(loc: TileId, id: PieceId) -> Piece {
        <Self as ChessFactory>::init(Type::King, Color::Black, loc, id)
    }
    fn queen_black(loc: TileId, id: PieceId) -> Piece {
        <Self as ChessFactory>::init(Type::Queen, Color::Black, loc, id)
    }
}

pub trait StandardChess: ChessFactory {
    fn gen_std_white() -> [Piece; 16] {
        use crate::constants::*;
        [
            <Self as ChessFactory>::rook_white(A1, 1),
            <Self as ChessFactory>::knight_white(B1, 2),
            <Self as ChessFactory>::bishop_white(C1, 3),
            <Self as ChessFactory>::queen_white(D1, 4),
            <Self as ChessFactory>::king_white(E1, 5),
            <Self as ChessFactory>::bishop_white(F1, 6),
            <Self as ChessFactory>::knight_white(G1, 7),
            <Self as ChessFactory>::rook_white(H1, 8),
            <Self as ChessFactory>::pawn_white(A2, 9),
            <Self as ChessFactory>::pawn_white(B2, 10),
            <Self as ChessFactory>::pawn_white(C2, 11),
            <Self as ChessFactory>::pawn_white(D2, 12),
            <Self as ChessFactory>::pawn_white(E2, 13),
            <Self as ChessFactory>::pawn_white(F2, 14),
            <Self as ChessFactory>::pawn_white(G2, 15),
            <Self as ChessFactory>::pawn_white(H2, 16),
        ]
    }
    fn gen_std_black() -> [Piece; 16] {
        use crate::constants::*;
        [
            <Self as ChessFactory>::pawn_black(A7, -16),
            <Self as ChessFactory>::pawn_black(B7, -15),
            <Self as ChessFactory>::pawn_black(C7, -14),
            <Self as ChessFactory>::pawn_black(D7, -13),
            <Self as ChessFactory>::pawn_black(E7, -12),
            <Self as ChessFactory>::pawn_black(F7, -11),
            <Self as ChessFactory>::pawn_black(G7, -10),
            <Self as ChessFactory>::pawn_black(H7, -9),
            <Self as ChessFactory>::rook_black(A8, -8),
            <Self as ChessFactory>::knight_black(B8, -7),
            <Self as ChessFactory>::bishop_black(C8, -6),
            <Self as ChessFactory>::queen_black(D8, -5),
            <Self as ChessFactory>::king_black(E8, -4),
            <Self as ChessFactory>::bishop_black(F8, -3),
            <Self as ChessFactory>::knight_black(G8, -2),
            <Self as ChessFactory>::rook_black(H8, -1),
        ]
    }
}

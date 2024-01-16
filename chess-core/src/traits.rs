use crate::types::*;

pub trait ChessFactory {
    fn init(ty: Type, color: Color, loc: usize) -> Piece {
        let id = 0_i16;
        Piece { color, ty, loc, id }
    }
    fn pawn_white(loc: usize) -> Piece {
        <Self as ChessFactory>::init(Type::Pawn, Color::White, loc)
    }
    fn rook_white(loc: usize) -> Piece {
        <Self as ChessFactory>::init(Type::Rook, Color::White, loc)
    }
    fn knight_white(loc: usize) -> Piece {
        <Self as ChessFactory>::init(Type::Knight, Color::White, loc)
    }
    fn bishop_white(loc: usize) -> Piece {
        <Self as ChessFactory>::init(Type::Bishop, Color::White, loc)
    }
    fn king_white(loc: usize) -> Piece {
        <Self as ChessFactory>::init(Type::King, Color::White, loc)
    }
    fn queen_white(loc: usize) -> Piece {
        <Self as ChessFactory>::init(Type::Queen, Color::White, loc)
    }
    fn pawn_black(loc: usize) -> Piece {
        <Self as ChessFactory>::init(Type::Pawn, Color::Black, loc)
    }
    fn rook_black(loc: usize) -> Piece {
        <Self as ChessFactory>::init(Type::Rook, Color::Black, loc)
    }
    fn knight_black(loc: usize) -> Piece {
        <Self as ChessFactory>::init(Type::Knight, Color::Black, loc)
    }
    fn bishop_black(loc: usize) -> Piece {
        <Self as ChessFactory>::init(Type::Bishop, Color::Black, loc)
    }
    fn king_black(loc: usize) -> Piece {
        <Self as ChessFactory>::init(Type::King, Color::Black, loc)
    }
    fn queen_black(loc: usize) -> Piece {
        <Self as ChessFactory>::init(Type::Queen, Color::Black, loc)
    }
}

pub trait StandardChess: ChessFactory {
    fn gen_std_white() -> [Piece; 16] {
        use crate::constants::*;
        [
            <Self as ChessFactory>::rook_white(A1),
            <Self as ChessFactory>::knight_white(B1),
            <Self as ChessFactory>::bishop_white(C1),
            <Self as ChessFactory>::queen_white(D1),
            <Self as ChessFactory>::king_white(E1),
            <Self as ChessFactory>::bishop_white(F1),
            <Self as ChessFactory>::knight_white(G1),
            <Self as ChessFactory>::rook_white(H1),
            <Self as ChessFactory>::pawn_white(A2),
            <Self as ChessFactory>::pawn_white(B2),
            <Self as ChessFactory>::pawn_white(C2),
            <Self as ChessFactory>::pawn_white(D2),
            <Self as ChessFactory>::pawn_white(E2),
            <Self as ChessFactory>::pawn_white(F2),
            <Self as ChessFactory>::pawn_white(G2),
            <Self as ChessFactory>::pawn_white(H2),
        ]
    }
    fn gen_std_black() -> [Piece; 16] {
        use crate::constants::*;
        [
            <Self as ChessFactory>::rook_black(A8),
            <Self as ChessFactory>::knight_black(B8),
            <Self as ChessFactory>::bishop_black(C8),
            <Self as ChessFactory>::queen_black(D8),
            <Self as ChessFactory>::king_black(E8),
            <Self as ChessFactory>::bishop_black(F8),
            <Self as ChessFactory>::knight_black(G8),
            <Self as ChessFactory>::rook_black(H8),
            <Self as ChessFactory>::pawn_black(A7),
            <Self as ChessFactory>::pawn_black(B7),
            <Self as ChessFactory>::pawn_black(C7),
            <Self as ChessFactory>::pawn_black(D7),
            <Self as ChessFactory>::pawn_black(E7),
            <Self as ChessFactory>::pawn_black(F7),
            <Self as ChessFactory>::pawn_black(G7),
            <Self as ChessFactory>::pawn_black(H7),
        ]
    }
}

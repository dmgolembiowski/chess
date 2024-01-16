use crate::msg::PieceId;
use crate::{constants::TILECOUNT, /*helper::chess_board_from_raw, */ msg::MoveOp};
use const_typed_builder::Builder;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

// [`RawBoard`] is a flat array of 64 [`Tile`s]().
// There is a useful collection of chess board tile codes
// for referring to given slots to this via [`crate::helper`]().
pub type RawBoard = [Tile; TILECOUNT];

pub struct Board(pub [[Tile; 8]; 8]);

impl Board {}
// [`Piece`]() is a shared data structure. [`PlayerData`]()
// instances are considered their owning source. That is, they have the
// main responsibility of enforcing [`Drop::drop`] wait until all strong reference
// counts on [`Rc<RefCell<Piece>>`]() be 1 and weak reference counts be 0.
//
// This detail is important because [`PlayerData`]() shares write-access (via
// interior mutability) with [`crate::core::types::Tile`s](), provided one calls
// [`Rc::upgrade`]() on the [`std::rc::Weak`]() refernce counted pointer to the piece.
#[derive(Builder, Debug, Clone, Serialize, Deserialize)]
pub struct Piece {
    pub id: PieceId,
    pub color: Color,
    pub ty: Type,
    pub loc: usize,
}

// [`update_loc`]() should be called whenever its position on the board changes.
// If a [`loc`] on a [`Piece`] reaches 64 or higher, its visibility should be updated
// so that it cannot be seen on the board.
impl Piece {
    pub fn update_loc(&mut self, new_loc: usize) {
        self.loc = new_loc;
    }
    pub fn set_id(&mut self, new_id: PieceId) {
        self.id = new_id;
    }
    /*
    pub fn relative_movement(&self, raw_board: &mut RawBoard) -> () /*&[Move]*/ {
        let board_rot = match self.color {
            Color::White => false,
            Color::Black => true,
        };

        let mut board = chess_board_from_raw(&raw_board);
        if board_rot {
            todo!("Implement a 8x8 matrix rotation algorithm");
            todo!("Transform all indicies into their rotated counterparts");
            todo!("Store the inverse index pairs as key-value tuples");
        }
        let it = *self;
        match it.ty {
            Type::Pawn => {}
            Type::Rook => {}
            Type::King => {}
            Type::Queen => {}
            Type::Bishop => {}
            Type::Knight => {}
        }
    }
    */
}

pub struct Move<'a> {
    on: &'a mut Piece,
    cap: bool,
    dir: Direction,
    len: usize,
    dest: usize,
    on_complete: Option<Box<dyn FnOnce() -> Move<'a>>>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
    Right,
    Left,
    ForwardRight,
    BackwardRight,
    ForwardLeft,
    BackwardLeft,
    #[default]
    Nil,
}

// Each of the playable kinds of chess [`Piece`]() has a
// particular [`Type`] that distinguishes its possible movement
// options, special properties, movement constraints, and subjective
// power-level in comparison to others.
#[derive(Serialize, Clone, Deserialize, Debug, Eq, PartialEq)]
pub enum Type {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

// [`Color`]() is the enum which associates owned player's pieces
// and that [`PlayerData`]()'s identity. Traditionally, turns will proceed
// in the order of [`PlayerData::White`]() followed by [`PlayerData::Black`]().
#[derive(Default, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Color {
    #[default]
    White,
    Black,
}

// The shading of the tile beneath any given chess piece is this
// this module's [`Background`]().
#[derive(Default, Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Background {
    Light,
    #[default]
    Dark,
}

// Comparison operations between Background and Color are used for
// sanity-testing to confirm commonplace chess rules. Namely,
// the color of each queen piece in a standard game corresponds with
// the underlying tile's color.
impl PartialEq<Background> for Color {
    fn eq(&self, other: &Background) -> bool {
        match (self, other) {
            (&Color::White, &Background::Light) | (&Color::Black, &Background::Dark) => true,
            _ => false,
        }
    }
}

// [`Tile`]() uses a weak, reference-counting smart pointer to share
// exclusive access to the underlying [`Piece`]().
//
// [`index`]() refers to the array index in the backing tile storage memory.
// There are helper values to access keyed indicies using constants defined
// in [`crate::constants`]().
//
// [`b_endzone`]() and [`w_endzone`]() respectively refer to the tile's
// ability to promote a pawn to another [`crate::type::Type`]() such as
// a [`Type::Queen`]().
#[derive(Builder, Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub w_endzone: bool,
    pub b_endzone: bool,
    pub color: Background,
    pub index: usize,
    pub pz: Option<Weak<RefCell<Piece>>>,
}

// When capturing a [`Piece`]() associated with a given tile,
// the caller of [`Self::update_piece`]() is responsible for passing a clone of their [`Rc<RefCell<Piece>>`]().
impl Tile {
    pub const fn dark(index: usize, w_endzone: bool, b_endzone: bool) -> Self {
        if w_endzone && b_endzone {
            panic!("Tile promotion is not shareable according to the standard chess rules")
        }
        Self {
            w_endzone,
            b_endzone,
            color: Background::Dark,
            index,
            pz: Option::<Weak<RefCell<Piece>>>::None,
        }
    }
    pub const fn light(index: usize, w_endzone: bool, b_endzone: bool) -> Self {
        if w_endzone && b_endzone {
            panic!("Tile promotion is not shareable according to the standard chess rules")
        }
        Self {
            w_endzone,
            b_endzone,
            color: Background::Light,
            index,
            pz: Option::<Weak<RefCell<Piece>>>::None,
        }
    }
    pub fn update_piece(&mut self, new_piece: Option<Rc<RefCell<Piece>>>) {
        let pz: Option<Weak<RefCell<Piece>>> = if let Some(owned) = new_piece {
            Some(Rc::downgrade(&owned))
        } else {
            None
        };
        self.pz = pz;
    }
}

pub struct VisionPiece<'a> {
    piece_id: PieceId,
    moves: [Option<Move<'a>>; 25],
}

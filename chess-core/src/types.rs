#![allow(dead_code)]
use crate::game::math::{index_to_xy, xy_to_index};
use crate::msg::PieceId;
use crate::{constants::TILECOUNT, game::math::XyPair};
// use const_typed_builder::Builder;
// use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt::Debug;
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
#[derive(Debug, Clone)]
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
}

pub struct Move<'a> {
    on: Rc<RefCell<Piece>>,
    cap: bool,
    dir: Direction,
    on_complete: Option<Box<dyn FnOnce() -> Move<'a>>>,
}

impl<'a> PartialEq for Move<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.dir == other.dir
    }
}

use std::fmt;
impl<'a> Debug for Move<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Move")
            .field("on", &self.on)
            .field("cap", &self.cap)
            .field("dir", &self.dir)
            .field("on_complete", &std::ptr::addr_of!(self.on_complete))
            .finish()
    }
}

impl<'a> Move<'a> {
    pub fn dest(&self) -> XyPair {
        if ((*self.on.clone()).borrow()).color == Color::Black {
            todo!("Perform the rotation transformation to interpolate values")
        }
        match self.dir {
            Direction::Nil => index_to_xy(((*self.on.clone()).borrow()).loc),
            Direction::Forward(delta_y) => {
                let XyPair { x, y } = index_to_xy(((*self.on.clone()).borrow()).loc);
                let new_y = y as usize + delta_y;
                XyPair {
                    x,
                    y: new_y as isize,
                }
            }
            _ => todo!("Handle remaining directions of movement"),
        }
    }
    pub fn new_nil(on: &Rc<RefCell<Piece>>) -> Self {
        let on = Rc::clone(on);
        Self {
            on,
            cap: false,
            dir: Direction::Nil,
            on_complete: None,
        }
    }
    pub fn forward(on: &Rc<RefCell<Piece>>, len: usize) -> Self {
        let on = Rc::clone(on);
        Self {
            on,
            cap: false,
            dir: Direction::Forward(len),
            on_complete: None,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Direction {
    Forward(usize),
    Backward(usize),
    Right(usize),
    Left(usize),
    ForwardRight(usize),
    BackwardRight(usize),
    ForwardLeft(usize),
    BackwardLeft(usize),
    ForwardTwoRightOne,
    ForwardOneRightTwo,
    BackwardTwoRightOne,
    BackwardOneRightTwo,
    ForwardTwoLeftOne,
    ForwardOneLeftTwo,
    BackwardTwoLeftOne,
    BacwardOneLeftTwo,
    #[default]
    Nil,
}

// Each of the playable kinds of chess [`Piece`]() has a
// particular [`Type`] that distinguishes its possible movement
// options, special properties, movement constraints, and subjective
// power-level in comparison to others.
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    #[default]
    White,
    Black,
}

// The shading of the tile beneath any given chess piece is this
// this module's [`Background`]().
#[derive(Default, Debug, Clone, Eq, PartialEq)]
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

// [`TileId`]() bootstraps the index it relates to within a [`RawBoard`]()
// so that its owning source can be queried for a [`Tile`]() that matches
// the requested ID. This is useful for graphical / visual implementors,
// like the one used in [`chess_game::RayTile`]().
pub type TileId = usize;

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
#[derive(Debug, Clone)]
pub struct Tile {
    pub w_endzone: bool,
    pub b_endzone: bool,
    pub color: Background,
    pub index: TileId,
    pub pz: Option<Weak<RefCell<Piece>>>,
}

// When capturing a [`Piece`]() associated with a given tile,
// the caller of [`Self::update_piece`]() is responsible for passing a clone of their [`Rc<RefCell<Piece>>`]().
impl Tile {
    pub const fn dark(index: TileId, w_endzone: bool, b_endzone: bool) -> Self {
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
    pub const fn light(index: TileId, w_endzone: bool, b_endzone: bool) -> Self {
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

#[derive(Default)]
pub struct VisionPiece<'a> {
    pub piece_id: PieceId,
    pub moves: [Option<Move<'a>>; 25],
}

impl<'a> VisionPiece<'a> {
    #[inline]
    pub fn new_empty(piece_id: PieceId) -> Self {
        let moves: [Option<Move<'a>>; 25] = Default::default();
        Self { piece_id, moves }
    }
    #[inline]
    pub fn new_with_moves<const N: usize>(piece_id: PieceId, moves: [Move<'a>; N]) -> Self {
        let mut buffer: [Option<Move>; 25] = Default::default();
        for (i, m) in moves.into_iter().enumerate() {
            if i >= 25 {
                break;
            }
            buffer[i] = Some(m);
        }
        Self {
            piece_id,
            moves: buffer,
        }
    }
}

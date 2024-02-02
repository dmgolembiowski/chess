#![allow(unused_imports)]
use anyhow::Result;
use chess_core::{
    self,
    constants::*,
    game::{
        self,
        math::{self, XyPair},
    },
    helper, msg,
    traits::*,
    types,
};
use crossbeam_channel::unbounded;
use crossbeam_utils::thread::scope;
use raylib::prelude::*;

const SQUARE_SIZE: i32 = 80;

#[inline]
fn get_y_from_col(col: i32) -> usize {
    match col {
        0 => 7,
        1 => 6,
        2 => 5,
        3 => 4,
        4 => 3,
        5 => 2,
        6 => 1,
        7 => 0,
        _ => panic!("Unintended usage"),
    }
}

#[inline]
fn xy_to_row_col(&XyPair { x, y }: &XyPair) -> (i32, i32) {
    let x = x as i32;
    let y = get_y_from_col(y as i32) as i32;
    (x, y)
}

fn main() {
    let (mut rl, thread) = raylib::init().size(1080, 720).title("Funky Chess").build();
    let mut gm = chess_core::spawn_game_master();
    let game_id: chess_core::msg::GameId = gm.create_game().unwrap();

    let is_even = |pos: usize| pos % 2 == 0;
    let is_odd = |pos: usize| !is_even(pos);
    let tile_color = |x: usize, y: usize| {
        if (is_odd(x) && is_odd(y)) || (is_even(x) && is_even(y)) {
            Color::WHITE
        } else {
            Color::LIGHTGRAY
        }
    };
    let white_pawn_texture = get_piece(COLOR::White, TYPE::Pawn, &mut rl, &thread);
    while !rl.window_should_close() {
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        use chess_core::types::Color as COLOR;
        use chess_core::types::Type as TYPE;

        for row in 0..8 {
            for col in 0..8 {
                let y_offset = col * SQUARE_SIZE;
                let x_offset = row * SQUARE_SIZE;
                let color = tile_color(row as usize, col as usize);
                let mut here = Rectangle::new(
                    x_offset as f32,
                    y_offset as f32,
                    SQUARE_SIZE as f32,
                    SQUARE_SIZE as f32,
                );
                // d.draw_rectangle(x_offset, y_offset, SQUARE_SIZE, SQUARE_SIZE, color);
                // If a chess piece exists at the XyPair corresponding to the given
                // row and column value, draw it here.
                let x = row;
                let y = get_y_from_col(col);
                d.draw_text(&format!("({x}, {y})"), x_offset, y_offset, 16, Color::BLACK);
                d.draw_rectangle_rec(&here, color);
                d.draw_texture(&white_pawn_texture, x_offset, y_offset, color);
            }
        }

        // To prevent the board from being reset after every single turn
        /*
        'game: loop {

        }
        */
    }

    // Ok(())
}

// This struct is the collection of related data specific to the raylib-specific
// graphical rectangle.
//
// It is loosely coupled to a [`chess_core::types::Tile`]() and is only concerned
// with dynamic UI interactions, and forwarding intent to the underpinning ChessGame.
pub struct RayTile<'a, 'b> {
    selected: bool,
    pub col_lower_bd: u32,
    pub col_upper_bd: u32,
    pub row_lower_bd: u32,
    pub row_upper_bd: u32,
    pub background_color: Color,
    pub texture_overlay: Option<&'a raylib::texture::Texture2D>,
    pub tile_id: types::TileId,
    pub raw_tile: &'b chess_core::types::Tile,
}

impl<'a, 'b> RayTile<'a, 'b> {
    pub fn select(&mut self) {
        self.selected = !self.selected;
    }
    pub fn is_selected(&self) -> bool {
        self.selected
    }
    // SAFETY:
    // The caller is responsible for ensuring that the exclusive reference
    // is not under contention between multiple threads.
    pub unsafe fn override_select(&mut self, new_status: bool) {
        self.selected = new_status;
    }
}

// By default a tile should not be in the selected state
// at the start. If a mouse clicks on it, we want the
// backing raytile to update its truthy "selected" state.
// Double tapping a tile should reset whether the tile is
// selected.
#[test]
#[allow(non_upper_case_globals)]
fn click_raytile_toggle_state() {
    use chess_core::types::Tile;
    const sel: bool = false;
    const tid: types::TileId = 9;
    const bg: Color = Color::WHITE;
    const tile: Tile = chess_core::types::Tile::light(8, false, false);
    const t2d: Option<&'_ raylib::texture::Texture2D> = None;
    const SQUARE_SIZE: u32 = 80;
    {
        let mut rt = RayTile {
            selected: sel,
            col_lower_bd: 0_u32,
            col_upper_bd: 80_u32,
            row_lower_bd: 7 * SQUARE_SIZE,
            row_upper_bd: 8 * SQUARE_SIZE,
            background_color: bg,
            texture_overlay: t2d,
            tile_id: tid,
            raw_tile: &tile,
        };
        assert_eq!(rt.is_selected(), false, "Sanity check failed");
        let _ = &mut rt.select();
        assert!(
            &rt.is_selected(),
            "Failed to toggle single-owner selected status"
        );
    }
}

// [ImgPack]() is an attempt to package piece assets directly into the binary
// at compile time.
//
// At runtime, this program uses late initialization to prepare raylib Images
// from the bytes, which can then be treated as a collection of
// [`raylib::texture::Texture2D`](https://docs.rs/raylib/4.0.0-dev.2/raylib/struct.Texture2D.html)
// instances.
pub struct ImgPack<'a> {
    filetype: &'a str,
    bytes: &'a [u8],
}

impl<'a> ImgPack<'a> {
    pub const fn new(filetype: &'a str, bytes: &'a [u8]) -> Self {
        Self { filetype, bytes }
    }
}

use std::convert::TryFrom;

// The from-memory image loader enforces a signature on the
// [`Image::load_image_from_memory`](https://docs.rs/raylib/4.0.0-dev.2/raylib/struct.Image.html#method.load_image_from_memory)
// call.
impl<'a> TryFrom<&ImgPack<'a>> for Image {
    type Error = String;

    fn try_from(pack: &ImgPack<'a>) -> Result<Image, Self::Error> {
        /*if pack.size < 0 {
            return Result::<Image, Self::Error>::Err(
                "Negative size is undefined behavior".to_string(),
            );
        }
        */
        // let realloc: Box<Vec<u8>> = Box::new(pack.bytes.to_vec());
        // Image::load_image_from_mem(pack.filetype, &realloc.as_ref())
        Image::load_image_from_mem(pack.filetype, pack.bytes)
    }
}

use chess_core::types::Color as COLOR;
use chess_core::types::Type as TYPE;
/*
fn get_piece<'a>(
    _color: COLOR,
    _piece_type: TYPE,
    raylib_handle: &mut RaylibHandle,
    raylib_thread: &RaylibThread,
) -> Texture2D {
    // Using the following approach, we get:
    //      WARNING: IMAGE: File format not supported
    //      thread 'main' panicked at chess-game/src/main.rs:171:9:
    //      image: Could not load the white pawn
    //
    // const white_pawn_bytes: &[u8] = include_bytes!("../assets/pawn-white.png");
    // dbg!("{:?}", white_pawn_bytes);
    // const white_pawn_size: i32 = 6742;
    // const white_pawn_pack: ImgPack = ImgPack::new("png", white_pawn_bytes, white_pawn_size);
    // let white_pawn_img: Result<Image, String> = Image::try_from(&white_pawn_pack);
    //
    // // TODO: Find a way to meaningfully store these on the heap so that
    // // this can return a borrowed local variable
    // let image = if let Ok(img) = white_pawn_img {
    //    Box::new(img)
    // } else {
    //     panic!("Could not load the white pawn");
    // };
    //
    // let piece: Texture2D = raylib_handle
    //     .load_texture_from_image(raylib_thread, image.as_ref())
    //     .unwrap();
    //
    // piece

    let mut image = Image::load_image("assets/pawn-white.png").unwrap();
    image.resize(SQUARE_SIZE, SQUARE_SIZE);
    let piece: Texture2D = raylib_handle
        .load_texture_from_image(raylib_thread, &image)
        .unwrap();
    piece
}
*/
fn get_piece<'a>(
    _color: COLOR,
    _piece_type: TYPE,
    raylib_handle: &mut RaylibHandle,
    raylib_thread: &RaylibThread,
) -> Texture2D {
    // Using the following approach, we get:
    //      WARNING: IMAGE: File format not supported
    //      thread 'main' panicked at chess-game/src/main.rs:171:9:
    //      image: Could not load the white pawn
    //
    const white_pawn_bytes: &[u8] = include_bytes!("../assets/pawn-white.png");
    dbg!("{:?}", white_pawn_bytes);
    const white_pawn_size: i32 = 6742;
    const white_pawn_pack: ImgPack = ImgPack::new(".png", white_pawn_bytes);
    let white_pawn_img: Result<Image, String> = Image::try_from(&white_pawn_pack);
    //
    // // TODO: Find a way to meaningfully store these on the heap so that
    // // this can return a borrowed local variable
    let image = if let Ok(img) = white_pawn_img {
        Box::new(img)
    } else {
        panic!("Could not load the white pawn");
    };
    //
    let piece: Texture2D = raylib_handle
        .load_texture_from_image(raylib_thread, image.as_ref())
        .unwrap();

    piece
}

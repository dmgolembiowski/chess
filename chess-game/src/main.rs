#![allow(unused_imports)]
mod image_data;
use anyhow::Result;
use chess_core::{
    self,
    constants::*,
    game::{
        self,
        math::{self, XyPair},
    },
    helper,
    layout::{self, Layout},
    msg::{self, PieceId, TileId},
    traits::*,
    types,
};
use crossbeam_channel::unbounded;
use crossbeam_utils::thread::scope;
use image_data as img;
use raylib::prelude::*;

const SQUARE_SIZE: i32 = 96;

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

fn main() -> Result<()> {
    const X_MARGIN: i32 = 312;
    const Y_MARGIN: i32 = 64;
    let (mut rl, thread): (RaylibHandle, RaylibThread) =
        raylib::init().size(1440, 932).title("funky-chess").build();
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

    let tile_mapping: HashMap<XyPair, Vertices> = {
        let mut it = HashMap::new();
        for row in 0..8 {
            for col in 0..8 {
                let y_lower_bound = Y_MARGIN + col * SQUARE_SIZE;
                let y_upper_bound = Y_MARGIN + (col + 1) * SQUARE_SIZE;
                let x_upper_bound = X_MARGIN + (row + 1) * SQUARE_SIZE;
                let x_lower_bound = X_MARGIN + row * SQUARE_SIZE;
                let color = tile_color(row as usize, col as usize);
                let x = row;
                let y = get_y_from_col(col);

                // prepare an xypair from the supplied data and make a set of mappings
                // between the chess tiles' XyPair and the offset bounds
                let norm_xy = XyPair {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                };

                // There are four vertices on each of the tiles. Each of the four vertices
                // is needed correlate the mouse's location with a wrapped tile border.
                let top_left = (x_lower_bound as u32, y_lower_bound as u32);
                let top_right = (x_upper_bound as u32, y_lower_bound as u32);
                let bot_left = (x_lower_bound as u32, y_upper_bound as u32);
                let bot_right = (x_upper_bound as u32, y_upper_bound as u32);
                it.insert(
                    norm_xy,
                    Vertices::new(top_left, top_right, bot_left, bot_right),
                );
            }
        }
        it
    };

    // At this point, need to query the game master for the current state of the game
    // so we can build a relation between the XyPairs of tiles and the RayTiles.
    // It is worth noting that the data field on Layout is a std::collections::BTreeMap of
    // XyPairs to &Tile data.
    let layout: Layout = gm.request_game_layout(game_id)?;

    // Finally: we bridge the two worlds of raylib and chess_core.
    // This is the main loop.
    use chess_core::types::{Color as COLOR, Type as TYPE};

    while !rl.window_should_close() {
        {
            let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);
        }
        {
            for (xy, vertices) in tile_mapping.iter() {
                let raw_tile: &Tile = layout.data.get(&xy).unwrap();
                let color = tile_color(xy.x as usize, xy.y as usize);
                let selected = false;
                let tile_id: TileId = raw_tile.clone().index;
                let raytile =
                    RayTile::init(raw_tile, xy.clone(), vertices.clone(), &mut rl, &thread);
                let _ = &raytile.draw(&mut rl, &thread);
            }
        }
        {
            // let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
            /*for (xy, vertex) in tile_mapping.iter() {
                println!("tile_mapping[{xy:?}] = {vertex:?}");
            }
            println!("  ");
            println!("----------------------------");
            */
            //println!("layout.data from Game Master");
            //println!("============================");
            //for (xy, tile_ref) in layout.data.iter() {
            //    println!(
            //        "layout.data[{xy:?}]@[{:?}] = {:?}",
            //        tile_ref.index, tile_ref.pz
            //    );
            //}
            //break;
        }
        /*
        for row in 0..8 {
            for col in 0..8 {
                let xy = XyPair { x: row, y: get_y_from_col(col) };

                // let y_offset = Y_MARGIN + col * SQUARE_SIZE;
                // let x_offset = X_MARGIN + row * SQUARE_SIZE;
                // let color = tile_color(row as usize, col as usize);
                let mut here = Rectangle::new(
                    x_offset as f32,
                    y_offset as f32,
                    SQUARE_SIZE as f32,
                    SQUARE_SIZE as f32,
                );
                // d.draw_rectangle(x_offset, y_offset, SQUARE_SIZE, SQUARE_SIZE, color);
                // If a chess piece exists at the XyPair corresponding to the given
                // row and column value, draw it here.
                // let x = row;
                // let y = get_y_from_col(col);
                // d.draw_rectangle_rec(&here, color);
                //d.draw_text(
                //    &format!("({x}, {y})\n({x_offset}, {y_offset})"),
                //    x_offset,
                //    y_offset,
                //    16,
                //    Color::BLACK,
                //);

                // d.draw_texture(&white_pawn_texture, x_offset, y_offset, color);
            }
        }
        */
    }
    Ok(())
}

use chess_core::types::Tile;
use raylib::texture::Texture2D;

// This struct is the collection of related data specific to the raylib-specific
// graphical rectangle.
//
// It is loosely coupled to a [`chess_core::types::Tile`]() and is only concerned
// with dynamic UI interactions, and forwarding intent to the underpinning ChessGame.
pub struct RayTile<'a> {
    pub selected: bool,
    pub vertices: Vertices,
    pub xy: XyPair,
    pub background_color: Color,
    pub piece_id: Option<msg::PieceId>,
    pub texture_overlay: Option<Texture2D>,
    pub tile_id: TileId,
    pub raw_tile: &'a Tile,
}

impl<'a> RayTile<'a> {
    pub fn new(
        selected: bool,
        raw_tile: &'a Tile,
        xy: XyPair,
        tile_id: TileId,
        vertices: Vertices,
        texture_overlay: Option<Texture2D>,
        background_color: Color,
        piece_id: Option<msg::PieceId>,
    ) -> Self {
        Self {
            selected,
            vertices,
            xy,
            background_color,
            texture_overlay,
            tile_id,
            raw_tile,
            piece_id,
        }
    }

    pub fn draw(&self, raylib_handle: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        // TODO: Handle `self.selected`
        // TODO: Handle whether the tile is shown as a friendly/controlled tile
        /*
            "Assuming `init` has already happened, we only need to case out the logic if
            a single Rectangle with an empty background color is needed, otherwise a multi-stage
            drawing with a Texture2D, some highlight/filtering to indicate its active state,
            or to do simply nothing because there is no Piece associated with the Tile."
        */
        {
            let mut d = raylib_handle.begin_drawing(raylib_thread);
            let mut rect = Rectangle::try_from(&self.vertices).unwrap();
            d.draw_rectangle_rec(&rect, self.background_color);
            if let Some(texture) = &self.texture_overlay {
                d.draw_texture(texture, rect.x as i32, rect.y as i32, Color::WHITE);
            }
        }
    }

    pub fn init(
        raw_tile: &'a Tile,
        xy: XyPair,
        vertices: Vertices,
        raylib_handle: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ) -> Self {
        let has_associated_piece = raw_tile.pz.is_some();
        let color_bg = match raw_tile.color {
            types::Background::Light => Color::WHITE,
            types::Background::Dark => Color::LIGHTGRAY,
        };
        let tile_id: TileId = raw_tile.index;
        if has_associated_piece {
            // Safety: already verified that the `pz` field is not `None`
            let pz = unsafe { raw_tile.pz.as_ref().unwrap_unchecked() };
            assert!(pz.weak_count() > 0, "PlayerData already dropped");
            match (*(unsafe { pz.upgrade().unwrap_unchecked() }))
                .borrow()
                .clone()
            {
                types::Piece {
                    id,
                    color,
                    ty,
                    loc: _loc,
                } => {
                    let texture = get_piece(color, ty, raylib_handle, raylib_thread);
                    return Self::new(
                        /* selected: bool = */ false,
                        /* raw_tile: &'a Tile = */ raw_tile,
                        /* xy: XyPair = */ xy,
                        /* tile_id: TileId = */ tile_id,
                        /* vertices: Vertices = */ vertices,
                        /* texture_overlay: Option<Texture2D> = */ Some(texture),
                        /* background_color: Color = */ color_bg,
                        /* piece_id: Option<msg::PieceId> = */ Some(id),
                    );
                }
            }
        } else {
            Self::new(
                /* selected: bool = */ false, /* raw_tile: &'a Tile = */ raw_tile,
                /* xy: XyPair = */ xy, /* tile_id: TileId = */ tile_id,
                /* vertices: Vertices = */ vertices,
                /* texture_overlay: Option<Texture2D> = */ None,
                /* background_color: Color = */ color_bg,
                /* piece_id: Option<msg::PieceId> = */ None,
            )
        }
    }

    pub fn select(&mut self) {
        self.selected = !self.selected;
    }
    #[deprecated]
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
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Vertices {
    pub top_left: (u32, u32),
    pub top_right: (u32, u32),
    pub bot_left: (u32, u32),
    pub bot_right: (u32, u32),
}

impl Vertices {
    pub fn new(
        top_left: (u32, u32),
        top_right: (u32, u32),
        bot_left: (u32, u32),
        bot_right: (u32, u32),
    ) -> Self {
        Self {
            top_left,
            top_right,
            bot_left,
            bot_right,
        }
    }
}

impl TryFrom<&Vertices> for Rectangle {
    type Error = String;
    fn try_from(vertices: &Vertices) -> Result<Self, Self::Error> {
        // Do not allow the construction of a rectangle that is
        // not mathematically sound. Ergo, we do not allow
        // Rectangle::try_from(&<Vertices as Default>::default()).unwrap()
        let width = vertices.top_right.0 - vertices.top_left.0;
        let height = vertices.bot_left.1 - vertices.top_left.1;

        if width * height <= 0 {
            return Err(Self::Error::from("Cannot construct zero-area rectangle"));
        }

        Ok(Rectangle::new(
            vertices.top_left.0 as f32,
            vertices.top_left.1 as f32,
            width as f32,
            height as f32,
        ))
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
    const tid: chess_core::msg::TileId = 9;
    const bg: Color = Color::WHITE;
    const tile: Tile = chess_core::types::Tile::light(8, false, false);
    const t2d: Option<raylib::texture::Texture2D> = None;
    let xy: XyPair = (7, 0).into();
    const piece_id: Option<PieceId> = Some(8);
    let vertices: Vertices = <Vertices as Default>::default();
    {
        let mut rt = RayTile {
            vertices,
            xy,
            selected: sel,
            background_color: bg,
            texture_overlay: t2d,
            tile_id: tid,
            raw_tile: &tile,
            piece_id,
        };
        assert_eq!(rt.is_selected(), false, "Sanity check failed");
        let _ = &mut rt.select();
        assert!(
            &rt.is_selected(),
            "Failed to toggle single-owner selected status"
        );
    }
}
use chess_core::types::Color as COLOR;
use chess_core::types::Type as TYPE;
use std::{
    collections::BTreeMap,
    sync::atomic::{
        AtomicPtr,
        Ordering::{Acquire, Release},
    },
};

#[inline]
fn get_piece<'a>(
    color: COLOR,
    piece_type: TYPE,
    raylib_handle: &mut RaylibHandle,
    raylib_thread: &RaylibThread,
) -> Texture2D {
    // Any resizing we do at runtime should be done on a clone that will not
    // distort the original. I believe this approach makes it possible to
    // maintain different views of an image, but always have the flexibility
    // to restore the original.
    let mut image = images().get_mut(&(color, piece_type)).unwrap().clone();
    image.resize(SQUARE_SIZE, SQUARE_SIZE);
    raylib_handle
        .load_texture_from_image(raylib_thread, &image)
        .unwrap()
}

#[inline]
fn images() -> &'static mut HashMap<(COLOR, TYPE), raylib::prelude::Image> {
    static PTR: AtomicPtr<HashMap<(COLOR, TYPE), Image>> = AtomicPtr::new(std::ptr::null_mut());
    let mut image_map = unsafe { PTR.load(Acquire) };

    if image_map.is_null() {
        let it = match generate_image_map() {
            Err(e) => {
                panic!("Build ought to fail because assets could not be loaded: {e}");
            }
            Ok(it) => it,
        };
        image_map = Box::into_raw(Box::new(it));
        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), image_map, Release, Acquire) {
            // Safety: image_map comes from Box::into_raw above. It was not shared with any other thread.
            drop(unsafe { Box::from_raw(image_map) });
            image_map = e;
        }
    }
    // Safety: image_map is not null and points to a properly initialized value
    unsafe { &mut *image_map }
}
/// change this error type parameter.
type LoadResult<E> = Result<HashMap<(COLOR, TYPE), Image>, E>;

use hashbrown::HashMap;

fn generate_image_map() -> LoadResult<String> {
    let mut map = HashMap::new();
    let lambda = |color: COLOR, piece_type: TYPE| match (color, piece_type) {
        (COLOR::White, TYPE::Pawn) => img::png_data_white_pawn,
        (COLOR::White, TYPE::Knight) => img::png_data_white_knight,
        (COLOR::White, TYPE::Bishop) => img::png_data_white_bishop,
        (COLOR::White, TYPE::Rook) => img::png_data_white_rook,
        (COLOR::White, TYPE::Queen) => img::png_data_white_queen,
        (COLOR::White, TYPE::King) => img::png_data_white_king,
        (COLOR::Black, TYPE::Pawn) => img::png_data_black_pawn,
        (COLOR::Black, TYPE::Knight) => img::png_data_black_knight,
        (COLOR::Black, TYPE::Bishop) => img::png_data_black_bishop,
        (COLOR::Black, TYPE::Rook) => img::png_data_black_rook,
        (COLOR::Black, TYPE::Queen) => img::png_data_black_queen,
        (COLOR::Black, TYPE::King) => img::png_data_black_king,
    };

    for color in [COLOR::White, COLOR::Black] {
        for piece_type in [
            TYPE::Pawn,
            TYPE::Knight,
            TYPE::Bishop,
            TYPE::Rook,
            TYPE::Queen,
            TYPE::King,
        ] {
            let key = (color.clone(), piece_type.clone());
            let data = lambda(color, piece_type);
            let image = Image::load_image_from_mem(".png", data.data)?;
            map.insert(key, image);
        }
    }
    Ok(map)
}

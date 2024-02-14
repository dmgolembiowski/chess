#![allow(non_upper_case_globals, unused)]

use paste::paste;
use raylib::texture::Image;

macro_rules! white_piece_constants {
    ($($name:ident = $value:expr),*) => {
        $(
            paste! {
                #[doc(hidden)]
                pub const [<white_ $name>]: &str = concat!("assets/", $value, "-white", ".png");
            }
        )*
    };
}

white_piece_constants! {
    king = "king",
    queen = "queen",
    rook = "rook",
    bishop = "bishop",
    knight = "knight",
    pawn = "pawn"
}

macro_rules! black_piece_constants {
    ($($name:ident = $value:expr),*) => {
        $(
            paste! {
                #[doc(hidden)]
                pub const [<black_ $name>]: &str = concat!("assets/", $value, "-black", ".png");
            }
        )*
    };
}

black_piece_constants! {
    king = "king",
    queen = "queen",
    rook = "rook",
    bishop = "bishop",
    knight = "knight",
    pawn = "pawn"
}
/*
#[deprecated(
    since = "0.0.2",
    note = r"Upstream raylib-rs wants to use an enum variant for the ext field.
    This struct gains a field in the future which will likely replace
    the `AsRef<CString>` for specifying the image extension kind.
    Get ready to use `self::ImgPack` instead."
)]*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PngPack<'a> {
    pub path: &'a str,
    pub data: &'a [u8],
}

impl<'a> PngPack<'a> {
    pub const fn new(path: &'a str, data: &'a [u8]) -> Self {
        Self { path, data }
    }
}

#[doc = "Const expression for statically allocated PNG [`Image`](raylib::texture::Image) data"]
pub static png_data_white_king: PngPack =
    PngPack::new(white_king, include_bytes!("../assets/king-white.png"));

#[doc = "Const expression for statically allocated PNG [`Image`](raylib::texture::Image) data"]
pub static png_data_white_queen: PngPack =
    PngPack::new(white_queen, include_bytes!("../assets/queen-white.png"));

#[doc = "Const expression for statically allocated PNG [`Image`](raylib::texture::Image) data"]
pub static png_data_white_rook: PngPack =
    PngPack::new(white_rook, include_bytes!("../assets/rook-white.png"));

#[doc = "Const expression for statically allocated PNG [`Image`](raylib::texture::Image) data"]
pub static png_data_white_bishop: PngPack =
    PngPack::new(white_bishop, include_bytes!("../assets/bishop-white.png"));

#[doc = "Const expression for statically allocated PNG [`Image`](raylib::texture::Image) data"]
pub static png_data_white_knight: PngPack =
    PngPack::new(white_knight, include_bytes!("../assets/knight-white.png"));

#[doc = "Const expression for statically allocated PNG [`Image`](raylib::texture::Image) data"]
pub static png_data_white_pawn: PngPack =
    PngPack::new(white_pawn, include_bytes!("../assets/pawn-white.png"));

#[doc = "Const expression for statically allocated PNG [`Image`](raylib::texture::Image) data"]
pub static png_data_black_king: PngPack =
    PngPack::new(black_king, include_bytes!("../assets/king-black.png"));

#[doc = "Const expression for statically allocated PNG [`Image`](raylib::texture::Image) data"]
pub static png_data_black_queen: PngPack =
    PngPack::new(black_queen, include_bytes!("../assets/queen-black.png"));

#[doc = "Const expression for statically allocated PNG [`Image`](raylib::texture::Image) data"]
pub static png_data_black_rook: PngPack =
    PngPack::new(black_rook, include_bytes!("../assets/rook-black.png"));

#[doc = "Const expression for statically allocated PNG [`Image`](raylib::texture::Image) data"]
pub static png_data_black_bishop: PngPack =
    PngPack::new(black_bishop, include_bytes!("../assets/bishop-black.png"));

#[doc = "Const expression for statically allocated PNG [`Image`](raylib::texture::Image) data"]
pub static png_data_black_knight: PngPack =
    PngPack::new(black_knight, include_bytes!("../assets/knight-black.png"));

#[doc = "Const expression for statically allocated PNG [`Image`](raylib::texture::Image) data"]
pub static png_data_black_pawn: PngPack =
    PngPack::new(black_pawn, include_bytes!("../assets/pawn-black.png"));

#[doc(hidden)]
pub struct ImgPack<'a> {
    pub path: &'a str,
    pub data: &'a [u8],
    pub ext: PlannedExtEnum,
}

#[doc(hidden)]
pub enum PlannedExtEnum {
    Gif,
    Png,
    Qoi,
    Jpg,
    Svg,
    Tga,
    Unk,
}

#[doc(hidden)]
impl<'a> ImgPack<'a> {
    pub const fn new(path: &'a str, data: &'a [u8], ext: PlannedExtEnum) -> Self {
        Self { path, data, ext }
    }
}

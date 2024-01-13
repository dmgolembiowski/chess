use rscenes::prelude::*;

#[allow(non_upper_case_globals, dead_code)]
pub mod img {

    pub struct SvgPack<'a> {
        filetype: &'a str,
        bytes: &'a [u8],
        size: i32,
    }
    impl<'a> SvgPack<'a> {
        pub const fn new(filetype: &'a str, bytes: &'a [u8], size: i32) -> Self {
            Self {
                filetype,
                bytes,
                size,
            }
        }
    }

    // Sigh
    //
    // The from-memory image loader enforces a signature on the
    // [`Image::load_image_from_mem`]() function requires data
    // to be passed as `&Vec<u8>` instead of `&[u8]`. As a result we're forced
    // to allocate two more pointers, an duplicate buffer, and more metadata.
    use raylib::core::texture::Image;

    impl<'a> TryFrom<&SvgPack<'a>> for Image {
        type Error = String;

        fn try_from(svgpack: &SvgPack<'_>) -> Result<Image, Self::Error> {
            if svgpack.size < 0 {
                return Result::<Image, Self::Error>::Err("Negative size undefined".to_string());
            }
            let realloc: Box<Vec<u8>> = Box::new(svgpack.bytes.to_vec());
            Image::load_image_from_mem(svgpack.filetype, &realloc.as_ref(), svgpack.size)
        }
    }

    pub static black_bishop: SvgPack<'static> = {
        SvgPack::new(
            /* filetype: */ "svg",
            /* bytes: */ include_bytes!("../assets/bishop-black.svg"),
            /* size: */ 795_i32,
        )
    };

    pub static white_bishop: SvgPack<'static> = {
        SvgPack::new(
            /* filetype: */ "svg",
            /* bytes: */ include_bytes!("../assets/bishop-white.svg"),
            /* size: */ 980_i32,
        )
    };
}

pub mod ui {
    pub mod resources {
        use chess_core::{self, constants, helper::*, types};
        use raylib::core::texture::Image;
        use rscenes::prelude::*;
        use std::borrow::{Borrow, BorrowMut};

        pub struct Resources {
            pub font: Option<Font>,
            pub pieces: Vec<ChessPiece>,
        }

        pub struct ChessPiece {
            pz: types::Piece,
            pub img: Image,
        }
        impl Resources {
            pub fn new() -> Self {
                Self {
                    font: None,
                    pieces: vec![],
                }
            }
        }
    }
}

fn main() {
    let mut builder = raylib::init();
    builder.size(1080, 720).title("funky-chess");
    let mut manager = SceneManager::new(builder, ui::resources::Resources::new());
    println!("Hello, world!");
}

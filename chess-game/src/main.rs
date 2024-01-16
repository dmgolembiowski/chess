#![allow(unused_variables, unused_imports, unused_assignments)]
use rscenes::prelude::*;

#[allow(non_upper_case_globals, dead_code)]
pub mod ui {
    pub mod fonts {
        use rscenes::prelude::*;
        pub fn get_font(handle: &mut RaylibHandle, thread: &RaylibThread) {
            todo!("Get font accessible");
        }
    }
    pub mod resources {
        use chess_core::{self, constants, helper::*, types};
        use const_typed_builder::Builder;
        use raylib::core::texture::Image;
        use rscenes::prelude::*;
        use std::borrow::{Borrow, BorrowMut};

        #[derive(Debug)]
        pub struct Resources {
            pub font: Option<Font>,
            pub pieces: Vec<ChessPiece>,
        }

        #[derive(Debug)]
        pub struct ChessPiece {
            pz: types::Piece,
            pub img: Image,
        }

        impl Resources {
            pub fn new() -> Self {
                /*use constants::C8;
                let piece = types::Piece::builder()
                    .color(types::Color::Black)
                    .ty(types::Type::Bishop)
                    .loc(C8)
                    .build();
                let img = Image::load_image("assets/bishop-black.png").unwrap();
                let chess_piece = ChessPiece { pz: piece, img };
                let pieces = vec![chess_piece];

                Self { font: None, pieces }
                */
                Self {
                    font: None,
                    pieces: Vec::with_capacity(32),
                }
            }
            pub fn set_font(&mut self, font: Font) {
                self.font = Some(font);
            }
        }

        impl Borrow<Font> for Resources {
            fn borrow(&self) -> &Font {
                match &self.font {
                    Some(f) => f,
                    None => panic!("Font not provided"),
                }
            }
        }

        impl BorrowMut<Font> for Resources {
            fn borrow_mut(&mut self) -> &mut Font {
                match &mut self.font {
                    Some(f) => f,
                    None => panic!("Font not provided"),
                }
            }
        }

        pub struct ImgPack<'a> {
            filetype: &'a str,
            bytes: &'a [u8],
            size: i32,
        }
        impl<'a> ImgPack<'a> {
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
        impl<'a> TryFrom<&ImgPack<'a>> for Image {
            type Error = String;

            fn try_from(pngpack: &ImgPack<'_>) -> Result<Image, Self::Error> {
                if pngpack.size < 0 {
                    return Result::<Image, Self::Error>::Err(
                        "Negative size undefined".to_string(),
                    );
                }
                let realloc: Box<Vec<u8>> = Box::new(pngpack.bytes.to_vec());
                Image::load_image_from_mem(pngpack.filetype, &realloc.as_ref(), pngpack.size)
            }
        }

        pub static black_bishop: ImgPack<'static> = {
            ImgPack::new(
                /* filetype: */ "png",
                /* bytes: */
                include_bytes!("/home/david/code/chess/chess-game/assets/bishop-black.png"),
                /* size: */ 5719_i32,
            )
        };

        pub static white_bishop: ImgPack<'static> = {
            ImgPack::new(
                /* filetype: */ "png",
                /* bytes: */ include_bytes!("../assets/bishop-white.png"),
                /* size: */ 7565_i32,
            )
        };

        pub static black_king: ImgPack<'static> = {
            ImgPack::new(
                /* filetype: */ "png",
                /* bytes: */ include_bytes!("../assets/king-black.png"),
                /* size: */ 4501_i32,
            )
        };

        pub static white_king: ImgPack<'static> = {
            ImgPack::new(
                /* filetype: */ "png",
                /* bytes: */ include_bytes!("../assets/king-white.png"),
                /* size: */ 4990_i32,
            )
        };

        pub static black_knight: ImgPack<'static> = {
            ImgPack::new(
                /* filetype: */ "png",
                /* bytes: */ include_bytes!("../assets/knight-black.png"),
                /* size: */ 4749_i32,
            )
        };

        pub static white_knight: ImgPack<'static> = {
            ImgPack::new(
                /* filetype: */ "png",
                /* bytes: */ include_bytes!("../assets/knight-white.png"),
                /* size: */ 6489_i32,
            )
        };

        pub static black_pawn: ImgPack<'static> = {
            ImgPack::new(
                /* filetype: */ "png",
                /* bytes: */ include_bytes!("../assets/pawn-black.png"),
                /* size: */ 5034_i32,
            )
        };

        pub static white_pawn: ImgPack<'static> = {
            ImgPack::new(
                /* filetype: */ "png",
                /* bytes: */ include_bytes!("../assets/pawn-white.png"),
                /* size: */ 6742_i32,
            )
        };

        pub static black_queen: ImgPack<'static> = {
            ImgPack::new(
                /* filetype: */ "png",
                /* bytes: */ include_bytes!("../assets/queen-black.png"),
                /* size: */ 7783_i32,
            )
        };

        pub static white_queen: ImgPack<'static> = {
            ImgPack::new(
                /* filetype: */ "png",
                /* bytes: */ include_bytes!("../assets/queen-white.png"),
                /* size: */ 9672_i32,
            )
        };

        pub static black_rook: ImgPack<'static> = {
            ImgPack::new(
                /* filetype: */ "png",
                /* bytes: */ include_bytes!("../assets/rook-black.png"),
                /* size: */ 4274_i32,
            )
        };

        pub static white_rook: ImgPack<'static> = {
            ImgPack::new(
                /* filetype: */ "png",
                /* bytes: */ include_bytes!("../assets/rook-white.png"),
                /* size: */ 4927_i32,
            )
        };

        pub static chess_board_regular: ImgPack<'static> = {
            ImgPack::new(
                "png",
                include_bytes!("../assets/chess-board-regular.png"),
                469_i32,
            )
        };

        #[ignore]
        #[test]
        fn test_load_runtime_file() {
            const IMG_PATH: &str = "../assets/chess-board-regular.png";
            let img: Result<Image, String> = Image::load_image(IMG_PATH);
            assert!(img.is_ok());
        }
    }
}

macro_rules! hide {
    ( $($tt:tt)+ ) => {};
}

fn main() -> anyhow::Result<()> {
    let mut builder = raylib::init();
    builder.size(1080, 720).title("funky-chess");
    let mut manager = SceneManager::new(builder, ui::resources::Resources::new());
    manager.config(|handle, thread, resources| {
        handle.set_window_title(thread, "Funky Chess");
        hide! {
            let font = ui::fonts::get_font(handle, thread).unwrap();
            resources.set_font(font);
        }
        hide! {
            use eyre;
            Ok::<(), eyre::Report>(())
        }
    });
    manager.add_first_scene(Box::<ChessSandboxScene>::default());
    manager.start()
}

pub struct Game {
    world: World,
}

pub fn create_game(world: &mut World) {
    // https://github.com/deltaphc/raylib-rs/blob/31776ec6a7499927144634bf6615788de70dea9d/samples/specs.rs#L192
    todo!("Need to understand how `world` is used: e.g. `world.create_entity()`")
}

use std::fmt::{self, Debug, Display, Formatter};

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Game(world: {:p})", &self.world)
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Game").field("world", &()).finish()
    }
}

impl Default for Game {
    fn default() -> Self {
        let mut world = World::default();
        create_game(&mut world);
        Self { world }
    }
}

#[derive(Default, Debug)]
pub struct Player;

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct ChessSandboxScene {
    game: Game,
    player: Player,
    rect: Rectangle,
}

use anyhow;
use rscenes::prelude::Rectangle;
use ui::resources::Resources;

impl Scene<Resources> for ChessSandboxScene {
    fn init(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(
        &mut self,
        _handle: &mut RaylibDrawHandle<'_>,
        _rect: Rectangle,
        resources: &Resources,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn update(
        &mut self,
        rl: (&mut RaylibHandle, &RaylibThread),
        dt: f32,
        resources: &mut Resources,
    ) -> anyhow::Result<State<Resources>> {
        Err(anyhow::anyhow!("Not implemented"))
    }
}

use crate::{constants::TILECOUNT, types::Tile};

#[allow(unused, clippy::style)]
pub const fn chess_board() -> [Tile; TILECOUNT] {
    use crate::constants::*;
    [
        Tile::dark(
            /* index = */ A1, /* w_endzone = */ false, /* b_endzone = */ true,
        ),
        Tile::light(
            /* index = */ B1, /* w_endzone = */ false, /* b_endzone = */ true,
        ),
        Tile::dark(
            /* index = */ C1, /* w_endzone = */ false, /* b_endzone = */ true,
        ),
        Tile::light(
            /* index = */ D1, /* w_endzone = */ false, /* b_endzone = */ true,
        ),
        Tile::dark(
            /* index = */ E1, /* w_endzone = */ false, /* b_endzone = */ true,
        ),
        Tile::light(
            /* index = */ F1, /* w_endzone = */ false, /* b_endzone = */ true,
        ),
        Tile::dark(
            /* index = */ G1, /* w_endzone = */ false, /* b_endzone = */ true,
        ),
        Tile::light(
            /* index = */ H1, /* w_endzone = */ false, /* b_endzone = */ true,
        ),
        Tile::light(
            /* index = */ A2, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ B2, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ C2, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ D2, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ E2, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ F2, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ G2, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ H2, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ A3, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ B3, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ C3, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ D3, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ E3, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ F3, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ G3, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ H3, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ A4, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ B4, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ C4, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ D4, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ E4, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ F4, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ G4, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ H4, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ A5, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ B5, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ C5, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ D5, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ E5, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ F5, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ G5, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ H5, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ A6, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ B6, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ C6, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ D6, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ E6, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ F6, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ G6, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ H6, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ A7, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ B7, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ C7, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ D7, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ E7, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ F7, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ G7, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ H7, /* w_endzone = */ false, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ A8, /* w_endzone = */ true, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ B8, /* w_endzone = */ true, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ C8, /* w_endzone = */ true, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ D8, /* w_endzone = */ true, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ E8, /* w_endzone = */ true, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ F8, /* w_endzone = */ true, /* b_endzone = */ false,
        ),
        Tile::light(
            /* index = */ G8, /* w_endzone = */ true, /* b_endzone = */ false,
        ),
        Tile::dark(
            /* index = */ H8, /* w_endzone = */ true, /* b_endzone = */ false,
        ),
    ]
}

#[test]
fn size_8_by_8_board() {
    assert_eq!(chess_board().len(), 64);
}

use chess_core;

pub struct ChessServer {
    id: Option<u64>,
    game: chess_core::Game,
}
pub fn start_server() -> () {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

use anyhow::Result;

fn main() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::main;
    use engine::game_engine;

    #[test]
    fn game_main() {
        main().unwrap()
    }

    #[ignore = "Game runs forever. Need to cancel it after confirming uptime."]
    #[test]
    fn engine_runs() {
        let mut game = game_engine();
        assert!(game.start().is_ok());
    }

    #[test]
    fn request_single_player_game() {}
}

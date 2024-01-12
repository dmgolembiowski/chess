#![allow(unused_imports, unused_import_braces, dead_code)]
use anyhow::Result;

use bevy::{
    app::{self as _app, ScheduleRunnerPlugin},
    prelude::*,
    utils::Duration,
};

pub struct Game {
    app: App,
}

impl Game {
    pub fn start(&mut self) -> Result<(), ()> {
        self.app.run();
        Ok(())
    }
}

pub fn game_engine() -> Game {
    let mut game = App::new();
    game.add_plugins(
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        ))),
    );
    game.add_systems(Update, tick);
    // game.run();
    Game { app: game }
}

fn tick(mut state: Local<GameState>) {
    if state.count == <u32>::MAX - 1 {
        state.count = 0_u32;
        return;
    }
    if state.count % 60 == 0 {
        println!("{}", state.count);
    }
    state.count += 1;
}

#[derive(Default)]
pub struct GameState {
    count: u32,
}

use bevy::prelude::*;

use crate::setup::SetupPlugin;

mod menu;
mod setup;
mod textures;
mod unit;
mod utils;

fn main() {
    App::new().add_plugins(SetupPlugin).run();
}

#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone, Copy)]
pub enum GameStates {
    #[default]
    Menu,
    InGame,
}

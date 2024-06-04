use alien_builder::AlienBuilder;
use bevy::prelude::*;

use crate::GameStates;

mod alien_builder;

#[derive(Component)]
pub struct Alien {
    _alien_type: AlienType,
}

#[derive(Clone, Copy)]
pub enum AlienType {
    Squid,
    Crab,
    Octopus,
}

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::InGame), AlienBuilder::spawn_aliens);
    }
}

use alien_builder::AlienBuilder;
use bevy::prelude::*;

use crate::GameStates;

mod alien_builder;
mod movement;

#[derive(Component)]
pub struct Alien {
    id: u8,
    _alien_type: AlienType,
}

#[derive(Resource)]
pub struct AlienCount(u8);
impl AlienCount {
    pub fn get_and_count(&mut self) -> u8 {
        let val = self.0;
        self.count();
        val
    }
    pub fn count(&mut self) {
        self.0 += 1;
    }
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
        app.insert_resource(AlienCount(0))
            .add_plugins(movement::MovementPlugin)
            .add_systems(OnEnter(GameStates::InGame), AlienBuilder::spawn_aliens);
    }
}

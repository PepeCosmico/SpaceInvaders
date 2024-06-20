use bevy::prelude::*;
use missile::{missile_collision_reader, MissileCollisionEvent};

use crate::{
    textures::{GameTextures, Textures},
    GameStates,
};

use super::Side;

pub mod missile;

#[derive(Event)]
pub struct AttackEvent {
    transform: Transform,
    side: Side,
}

impl AttackEvent {
    pub fn new(transform: Transform, side: Side) -> Self {
        Self { transform, side }
    }
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>()
            .add_event::<MissileCollisionEvent>()
            .add_systems(Update, spawn_attack.run_if(in_state(GameStates::InGame)))
            .add_systems(
                Update,
                missile_collision_reader.run_if(in_state(GameStates::InGame)),
            );
    }
}

fn spawn_attack(
    mut commands: Commands,
    mut ev_attack: EventReader<AttackEvent>,
    textures: Res<GameTextures>,
) {
    for ev in ev_attack.read() {
        missile::spawn_missile(
            &mut commands,
            textures.get_texture(Textures::SimpleMissile),
            ev.transform,
            ev.side,
        );
    }
}

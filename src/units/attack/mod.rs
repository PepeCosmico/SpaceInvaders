use bevy::prelude::*;

use crate::{
    fisics::Velocity,
    textures::{GameTextures, Textures},
    GameStates,
};

mod missile;

#[derive(Event)]
pub struct AttackEvent {
    transform: Transform,
}

impl AttackEvent {
    pub fn new(transform: Transform) -> Self {
        Self { transform }
    }
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>()
            .add_systems(Update, spawn_attack.run_if(in_state(GameStates::InGame)));
    }
}

fn spawn_attack(
    mut commands: Commands,
    mut ev_attack: EventReader<AttackEvent>,
    textures: Res<GameTextures>,
) {
    for ev in ev_attack.read() {
        commands.spawn((
            SpriteBundle {
                texture: textures.get_texture(Textures::SimpleMissile),
                transform: ev.transform,
                ..Default::default()
            },
            Velocity(Vec2::new(0.0, 2.0)),
        ));
    }
}

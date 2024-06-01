use bevy::prelude::*;

use crate::{
    textures::{GameTextures, Textures},
    GameStates,
};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::InGame), create_units);
    }
}

fn create_units(mut commands: Commands, textures: Res<GameTextures>) {
    let texture = textures.get_texture(Textures::Player).unwrap().clone();
    commands.spawn(SpriteBundle {
        texture,
        transform: Transform {
            scale: Vec3::splat(6.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use crate::{
    textures::{GameTextures, Textures},
    GameStates,
};

use super::Unit;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Moving(Move);
pub enum Move {
    Left,
    Right,
    Still,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::InGame), spawn_player)
            .add_systems(
                Update,
                (move_player_input, move_player).run_if(in_state(GameStates::InGame)),
            );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<GameTextures>) {
    let texture = textures.get_texture(Textures::Player).unwrap().clone();
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3::new(0.0, -400.0, 0.0),
                scale: Vec3::splat(4.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Unit,
        Player,
        Moving(Move::Still),
    ));
}

fn move_player(mut transform_query: Query<(&mut Transform, &Moving), With<Player>>) {
    let (mut transform, moving) = transform_query.get_single_mut().unwrap();
    transform.translation.x += match moving.0 {
        Move::Left => -1.0,
        Move::Right => 1.0,
        Move::Still => 0.0,
    }
}

fn move_player_input(
    mut input: EventReader<KeyboardInput>,
    mut player_query: Query<&mut Moving, With<Player>>,
) {
    let mut player = player_query.get_single_mut().unwrap();
    for event in input.read() {
        if event.state.is_pressed() && event.key_code == KeyCode::ArrowLeft {
            player.0 = Move::Left;
        }
        if event.state == ButtonState::Released && event.key_code == KeyCode::ArrowLeft {
            player.0 = Move::Still;
        }
        if event.state.is_pressed() && event.key_code == KeyCode::ArrowRight {
            player.0 = Move::Right;
        }
        if event.state == ButtonState::Released && event.key_code == KeyCode::ArrowRight {
            player.0 = Move::Still;
        }
    }
}

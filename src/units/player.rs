use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    math::bounding::Aabb2d,
    prelude::*,
};

use crate::{
    fisics::Hitbox,
    textures::{GameTextures, Textures},
    GameStates,
};

use super::{attack::AttackEvent, Side, Unit};

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
                (player_input, move_player).run_if(in_state(GameStates::InGame)),
            );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<GameTextures>) {
    let texture = textures.get_texture(Textures::Player);
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
        Side::Player,
        Moving(Move::Still),
        Hitbox::new(Vec2::new(26.0, 16.0)),
    ));
}

fn player_attack(ev_attack: &mut EventWriter<AttackEvent>, trans_player: Transform) {
    ev_attack.send(AttackEvent::new(trans_player, Side::Player));
}

fn move_player(mut transform_query: Query<(&mut Transform, &Moving), With<Player>>) {
    let (mut transform, moving) = transform_query.get_single_mut().unwrap();

    let next_x = transform.translation.x
        + match moving.0 {
            Move::Left => -1.5,
            Move::Right => 1.5,
            Move::Still => 0.0,
        };

    transform.translation.x = match next_x {
        ..=-359.0 => -359.0,
        359.0.. => 359.0,
        n => n,
    };
}

fn player_input(
    mut input: EventReader<KeyboardInput>,
    mut ev_attack: EventWriter<AttackEvent>,
    mut player_query: Query<(&Transform, &mut Moving), With<Player>>,
) {
    let (trans_player, mut player) = player_query.get_single_mut().unwrap();
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
        if event.state.is_pressed() && event.key_code == KeyCode::Space {
            player_attack(&mut ev_attack, trans_player.clone());
        }
    }
}

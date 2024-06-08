use bevy::prelude::*;

use crate::{fisics::Velocity, units::Side};

#[derive(Component)]
pub struct Missile;

pub fn spawn_missile(
    commands: &mut Commands,
    texture: Handle<Image>,
    transform: Transform,
    side: Side,
) {
    commands.spawn((
        SpriteBundle {
            texture,
            transform,
            ..Default::default()
        },
        Missile,
        Velocity(Vec2::new(
            0.0,
            match side {
                Side::Player => 2.0,
                Side::Alien => -2.0,
            },
        )),
        side,
    ));
}

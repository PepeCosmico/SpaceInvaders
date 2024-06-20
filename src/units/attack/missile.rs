use bevy::prelude::*;

use crate::{
    fisics::{Hitbox, Velocity},
    units::{hitbox::CollisionEvent, Side},
};

#[derive(Event)]
pub struct MissileCollisionEvent {
    missile: Entity,
    entity: Entity,
}

impl CollisionEvent for MissileCollisionEvent {
    fn new(ent1: Entity, ent2: Entity) -> Self {
        Self {
            missile: ent1,
            entity: ent2,
        }
    }
}

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
                Side::Player => 8.0,
                Side::Alien => -8.0,
            },
        )),
        side,
        Hitbox::new(Vec2::new(4.0, 12.0)),
    ));
}

pub fn missile_collision_reader(
    mut commands: Commands,
    mut event_reader: EventReader<MissileCollisionEvent>,
) {
    for event in event_reader.read() {
        commands.entity(event.missile).despawn_recursive();
        commands.entity(event.entity).despawn_recursive();
    }
}

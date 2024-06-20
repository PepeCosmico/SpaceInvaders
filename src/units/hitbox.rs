use bevy::{
    ecs::query::QueryFilter,
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{fisics::Hitbox, units::attack::missile::Missile, GameStates};

use super::{alien::Alien, attack::missile::MissileCollisionEvent};

pub trait CollisionEvent: Event {
    fn new(ent1: Entity, ent2: Entity) -> Self;
}

pub struct HitboxPlugin;

impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            collition_event_writer::<
                (With<Missile>, With<Hitbox>),
                (With<Alien>, With<Hitbox>),
                MissileCollisionEvent,
            >
                .run_if(in_state(GameStates::InGame)),
        );
    }
}

pub fn collition_event_writer<T, S, E>(
    ent_query_1: Query<(Entity, &Transform, &Hitbox), T>,
    ent_query_2: Query<(Entity, &Transform, &Hitbox), S>,
    mut event_writer: EventWriter<E>,
) where
    T: QueryFilter,
    S: QueryFilter,
    E: CollisionEvent,
{
    for (ent1, trans1, hitbox1) in ent_query_1.iter() {
        for (ent2, trans2, hitbox2) in ent_query_2.iter() {
            let aabb1 = Aabb2d::new(
                Vec2::new(trans1.translation.x, trans1.translation.y),
                Vec2::new(hitbox1.rect.x, hitbox1.rect.y),
            );
            let aabb2 = Aabb2d::new(
                Vec2::new(trans2.translation.x, trans2.translation.y),
                Vec2::new(hitbox2.rect.x, hitbox2.rect.y),
            );

            if aabb1.intersects(&aabb2) {
                event_writer.send(CollisionEvent::new(ent1, ent2));
                info!("collision");
            }
        }
    }
}

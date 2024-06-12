use bevy::{
    ecs::query::QueryFilter,
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{fisics::Hitbox, GameStates};

use super::Unit;

pub struct HitboxPlugin;

impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn collition_event_writer<T, S, E>(
    ent_query_1: Query<(Entity, &Transform, &Hitbox), T>,
    ent_query_2: Query<(Entity, &Transform, &Hitbox), S>,
    event_writer: EventWriter<E>,
) where
    T: QueryFilter,
    S: QueryFilter,
    E: Event,
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
                info!("collision");
            }
        }
    }
}

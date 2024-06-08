use bevy::prelude::*;

use crate::GameStates;

#[derive(Component)]
pub struct Velocity(pub Vec2);

pub struct FisicsPlugin;

impl Plugin for FisicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            calculate_movement.run_if(in_state(GameStates::InGame)),
        );
    }
}

fn calculate_movement(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
    }
}

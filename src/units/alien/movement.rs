use bevy::prelude::*;

use crate::GameStates;

use super::Alien;

#[derive(Resource)]
struct AlienMovementTurn {
    pub aliens: Vec<u8>,
    turn: u8,
}

impl AlienMovementTurn {
    pub fn new() -> Self {
        Self {
            aliens: (0..55).collect(),
            turn: 0,
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AlienMovementTurn::new())
            .add_systems(Update, move_alien.run_if(in_state(GameStates::InGame)));
    }
}

fn move_alien(
    mut alien_turn: ResMut<AlienMovementTurn>,
    mut aliens_query: Query<(&mut Transform, &Alien)>,
) {
    for (mut transform, alien) in aliens_query.iter_mut() {
        if alien.id == *alien_turn.aliens.get(alien_turn.turn as usize).unwrap() {
            transform.translation.x += 4.0;
            break;
        }
    }
    alien_turn.turn = (alien_turn.turn + 1) % alien_turn.aliens.len() as u8;
}

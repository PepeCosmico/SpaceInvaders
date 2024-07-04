use bevy::prelude::*;

use crate::{fisics::Hitbox, GameStates};

use super::Alien;

#[derive(Resource)]
struct AlienMovement {
    pub aliens: Vec<u8>,
    turn: u8,
    direction: Direction,
}

enum Direction {
    Left,
    Right,
}
impl Direction {
    fn get_num(&self) -> f32 {
        match self {
            Direction::Right => 1.0,
            Direction::Left => -1.0,
        }
    }
    fn change(&mut self) {
        *self = match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

impl AlienMovement {
    pub fn new() -> Self {
        Self {
            aliens: (0..55).collect(),
            turn: 0,
            direction: Direction::Right,
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AlienMovement::new())
            .add_systems(Update, move_alien.run_if(in_state(GameStates::InGame)));
    }
}

fn move_alien(
    mut alien_turn: ResMut<AlienMovement>,
    mut aliens_query: Query<(&mut Transform, &Alien, &Hitbox)>,
) {
    for (mut transform, alien, hitbox) in aliens_query.iter_mut() {
        if alien.id == *alien_turn.aliens.get(alien_turn.turn as usize).unwrap() {
            transform.translation.x += 4.0 * alien_turn.direction.get_num();
            if transform.translation.x > 384.0 - hitbox.rect.x {
                alien_turn.direction.change();
                transform.translation.x = 384.0 - hitbox.rect.x;
            } else if transform.translation.x > -384.0 + hitbox.rect.x {
                alien_turn.direction.change();
                transform.translation.x = -384.0 + hitbox.rect.x;
            }
            break;
        }
    }
    alien_turn.turn = (alien_turn.turn + 1) % alien_turn.aliens.len() as u8;
}

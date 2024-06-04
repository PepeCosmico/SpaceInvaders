use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use crate::{utils, GameStates};

#[derive(Component)]
struct OnMenuScreen;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Menu), spawn_menu_screen)
            .add_systems(Update, keyboard_input.run_if(in_state(GameStates::Menu)))
            .add_systems(
                OnExit(GameStates::Menu),
                utils::despawn_screen::<OnMenuScreen>,
            );
    }
}

fn spawn_menu_screen(mut commands: Commands) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Press Enter to start", TextStyle::default()),
            ..Default::default()
        },
        OnMenuScreen,
    ));
}

fn keyboard_input(
    mut char_input: EventReader<KeyboardInput>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    for event in char_input.read() {
        if event.state == ButtonState::Pressed && event.key_code == KeyCode::Enter {
            info!("Entered the game");
            next_state.set(GameStates::InGame);
        }
    }
}

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use crate::{
    textures::{GameTextures, Textures},
    GameStates,
};

#[derive(Component)]
pub struct Unit;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::InGame), create_units)
            .add_systems(Update, keyboard_input.run_if(in_state(GameStates::InGame)));
    }
}

fn create_units(
    mut commands: Commands,
    textures: Res<GameTextures>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = textures.get_texture(Textures::Player).unwrap().clone();
    commands.spawn(SpriteBundle {
        texture,
        transform: Transform {
            scale: Vec3::splat(6.0),
            ..Default::default()
        },
        ..Default::default()
    });

    let crab_text = textures.get_texture(Textures::Crab).unwrap().clone();
    let layout = TextureAtlasLayout::from_grid(Vec2::new(11.0, 8.0), 1, 2, None, None);
    let texture_atlas_sheet = texture_atlas_layouts.add(layout);
    let crab1 = SpriteSheetBundle {
        texture: crab_text.clone(),
        atlas: TextureAtlas {
            layout: texture_atlas_sheet.clone(),
            index: 0,
        },
        transform: Transform {
            translation: Vec3::new(100.0, 0.0, 0.0),
            scale: Vec3::splat(4.0),
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn((crab1, Unit));
}

fn keyboard_input(
    mut char_input: EventReader<KeyboardInput>,
    mut atlas_query: Query<&mut TextureAtlas, With<Unit>>,
) {
    for event in char_input.read() {
        if event.state == ButtonState::Pressed && event.key_code == KeyCode::ArrowRight {
            for mut atlas in &mut atlas_query {
                atlas.index = match atlas.index {
                    0 => 1,
                    1 => 0,
                    _ => 0,
                }
            }
        }
    }
}

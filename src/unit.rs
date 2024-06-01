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
#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Alien;
#[derive(Component, Clone)]
pub struct Squid;
#[derive(Component, Clone)]
pub struct Crab;
#[derive(Component, Clone)]
pub struct Octopus;

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
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    spawn_player(&mut commands, &textures);
    spawn_aliens(&mut commands, &textures, texture_atlas_layouts);
}

fn spawn_player(commands: &mut Commands, textures: &Res<GameTextures>) {
    let texture = textures.get_texture(Textures::Player).unwrap().clone();
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform {
                scale: Vec3::splat(4.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Unit,
        Player,
    ));
}

fn spawn_aliens(
    commands: &mut Commands,
    textures: &Res<GameTextures>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let squid_text = textures.get_texture(Textures::Squid).unwrap().clone();
    let crab_text = textures.get_texture(Textures::Crab).unwrap().clone();
    let octopus_text = textures.get_texture(Textures::Octopus).unwrap().clone();
    let squid_layout = TextureAtlasLayout::from_grid(Vec2::new(8.0, 8.0), 1, 2, None, None);
    let crab_layout = TextureAtlasLayout::from_grid(Vec2::new(11.0, 8.0), 1, 2, None, None);
    let octopus_layout = TextureAtlasLayout::from_grid(Vec2::new(12.0, 8.0), 1, 2, None, None);
    let squid_atlas_sheet = texture_atlas_layouts.add(squid_layout);
    let crab_atlas_sheet = texture_atlas_layouts.add(crab_layout);
    let octopus_atlas_sheet = texture_atlas_layouts.add(octopus_layout);

    create_alien_row(commands, Squid, 300.0, squid_text, squid_atlas_sheet);
    create_alien_row(commands, Crab, 200.0, crab_text, crab_atlas_sheet);
    create_alien_row(commands, Octopus, 100.0, octopus_text, octopus_atlas_sheet);
}

fn create_alien_row<T: Component + Clone>(
    commands: &mut Commands,
    alien: T,
    height: f32,
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
) {
    for i in 0..11 {
        let crab1 = SpriteSheetBundle {
            texture: texture.clone(),
            atlas: TextureAtlas {
                layout: layout.clone(),
                index: 0,
            },
            transform: Transform {
                translation: Vec3::new(-320.0 + i as f32 * 16.0 * 4.0, height, 0.0),
                scale: Vec3::splat(4.0),
                ..Default::default()
            },
            ..Default::default()
        };

        commands.spawn((crab1, Unit, Alien, alien.clone()));
    }
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

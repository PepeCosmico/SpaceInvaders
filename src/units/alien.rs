use bevy::prelude::*;

use crate::{
    textures::{GameTextures, Textures},
    units::Unit,
    GameStates,
};

#[derive(Component)]
pub struct Alien {
    alien_type: AlienType,
}

#[derive(Clone, Copy)]
pub enum AlienType {
    Squid,
    Crab,
    Octopus,
}

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::InGame), spawn_aliens);
    }
}

fn spawn_aliens(
    mut commands: Commands,
    textures: Res<GameTextures>,
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

    create_alien_row(
        &mut commands,
        AlienType::Squid,
        350.0,
        squid_text,
        squid_atlas_sheet,
    );
    create_alien_row(
        &mut commands,
        AlienType::Crab,
        286.0,
        crab_text.clone(),
        crab_atlas_sheet.clone(),
    );
    create_alien_row(
        &mut commands,
        AlienType::Crab,
        222.0,
        crab_text,
        crab_atlas_sheet,
    );
    create_alien_row(
        &mut commands,
        AlienType::Octopus,
        158.0,
        octopus_text.clone(),
        octopus_atlas_sheet.clone(),
    );
    create_alien_row(
        &mut commands,
        AlienType::Octopus,
        92.0,
        octopus_text,
        octopus_atlas_sheet,
    );
}

fn create_alien_row(
    commands: &mut Commands,
    alien: AlienType,
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

        commands.spawn((
            crab1,
            Unit,
            Alien {
                alien_type: alien.clone(),
            },
        ));
    }
}

use bevy::prelude::*;

use crate::{
    fisics::Hitbox,
    textures::{GameTextures, Textures},
    units::{Side, Unit},
};

use super::{Alien, AlienCount, AlienType};

pub struct AlienBuilder;

impl AlienBuilder {
    pub fn spawn_aliens(
        mut commands: Commands,
        textures: Res<GameTextures>,
        mut alien_count: ResMut<AlienCount>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let squid_text = textures.get_texture(Textures::Squid);
        let crab_text = textures.get_texture(Textures::Crab);
        let octopus_text = textures.get_texture(Textures::Octopus);
        let squid_layout = TextureAtlasLayout::from_grid(Vec2::new(8.0, 8.0), 1, 2, None, None);
        let crab_layout = TextureAtlasLayout::from_grid(Vec2::new(11.0, 8.0), 1, 2, None, None);
        let octopus_layout = TextureAtlasLayout::from_grid(Vec2::new(12.0, 8.0), 1, 2, None, None);
        let squid_atlas_sheet = texture_atlas_layouts.add(squid_layout);
        let crab_atlas_sheet = texture_atlas_layouts.add(crab_layout);
        let octopus_atlas_sheet = texture_atlas_layouts.add(octopus_layout);

        Self::spawn_line(
            &mut commands,
            &mut alien_count,
            AlienType::Squid,
            350.0,
            squid_text,
            squid_atlas_sheet,
        );
        Self::spawn_line(
            &mut commands,
            &mut alien_count,
            AlienType::Crab,
            286.0,
            crab_text.clone(),
            crab_atlas_sheet.clone(),
        );
        Self::spawn_line(
            &mut commands,
            &mut alien_count,
            AlienType::Crab,
            222.0,
            crab_text,
            crab_atlas_sheet,
        );
        Self::spawn_line(
            &mut commands,
            &mut alien_count,
            AlienType::Octopus,
            158.0,
            octopus_text.clone(),
            octopus_atlas_sheet.clone(),
        );
        Self::spawn_line(
            &mut commands,
            &mut alien_count,
            AlienType::Octopus,
            92.0,
            octopus_text,
            octopus_atlas_sheet,
        );
    }

    fn spawn_line(
        commands: &mut Commands,
        alien_count: &mut ResMut<AlienCount>,
        alien: AlienType,
        height: f32,
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
    ) {
        for i in 0..11 {
            Self::spawn_alien(
                commands,
                alien_count.get_and_count(),
                alien,
                Transform {
                    translation: Vec3::new(-320.0 + i as f32 * 16.0 * 4.0, height, 0.0),
                    scale: Vec3::splat(4.0),
                    ..Default::default()
                },
                texture.clone(),
                layout.clone(),
            );
        }
    }

    fn spawn_alien(
        commands: &mut Commands,
        alien_id: u8,
        alien: AlienType,
        transform: Transform,
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
    ) {
        commands.spawn((
            SpriteSheetBundle {
                texture: texture.clone(),
                atlas: TextureAtlas {
                    layout: layout.clone(),
                    index: 0,
                },
                transform,
                ..Default::default()
            },
            Unit,
            Alien {
                id: alien_id,
                _alien_type: alien.clone(),
            },
            Side::Alien,
            Hitbox::new(match alien {
                AlienType::Crab => Vec2::new(11.0 * 2.0, 8.0 * 2.0),
                AlienType::Octopus => Vec2::new(12.0 * 2.0, 8.0 * 2.0),
                AlienType::Squid => Vec2::new(8.0 * 2.0, 8.0 * 2.0),
            }),
        ));
    }
}

use bevy::prelude::*;

use crate::{
    menu::MenuPlugin,
    textures::{
        GameTextures, Textures, CRAB_IMG_PATH, OCTPUS_IMG_PATH, PLAYER_IMG_PATH, SQUID_IMG_PATH,
    },
    unit::UnitPlugin,
    GameStates,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .init_state::<GameStates>()
            .add_systems(Startup, setup_system)
            .add_plugins(MenuPlugin)
            .add_plugins(UnitPlugin);
    }
}

fn setup_system(mut commands: Commands, mut windows: Query<&mut Window>, assets: Res<AssetServer>) {
    let mut window = windows.single_mut();
    window.resolution.set(600.0, 900.0);

    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
    info!("Camera Spawned");

    let mut game_textures = GameTextures::new();
    game_textures.insert_texture(Textures::Player, assets.load(PLAYER_IMG_PATH));
    game_textures.insert_texture(Textures::Octopus, assets.load(OCTPUS_IMG_PATH));
    game_textures.insert_texture(Textures::Crab, assets.load(CRAB_IMG_PATH));
    game_textures.insert_texture(Textures::Squid, assets.load(SQUID_IMG_PATH));
    commands.insert_resource(game_textures);
    info!("Game Textures: loaded");
}

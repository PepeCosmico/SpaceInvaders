use bevy::prelude::*;

use crate::{
    fisics::FisicsPlugin,
    screens::ScreenPlugins,
    textures::{self, GameTextures, Textures},
    units::UnitsPlugins,
    GameStates,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .init_state::<GameStates>()
            .add_systems(Startup, setup_system)
            .add_plugins(FisicsPlugin)
            .add_plugins(ScreenPlugins)
            .add_plugins(UnitsPlugins);
    }
}

fn setup_system(mut commands: Commands, mut windows: Query<&mut Window>, assets: Res<AssetServer>) {
    let mut window = windows.single_mut();
    window.resolution.set(768.0, 900.0);

    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
    info!("Camera Spawned");

    let mut game_textures = GameTextures::new();
    game_textures.insert_texture(Textures::Player, assets.load(textures::PLAYER_IMG_PATH));
    game_textures.insert_texture(Textures::Squid, assets.load(textures::SQUID_IMG_PATH));
    game_textures.insert_texture(Textures::Crab, assets.load(textures::CRAB_IMG_PATH));
    game_textures.insert_texture(Textures::Octopus, assets.load(textures::OCTPUS_IMG_PATH));
    game_textures.insert_texture(
        Textures::SimpleMissile,
        assets.load(textures::SIMPLE_MISSILE_IMG_PATH),
    );
    commands.insert_resource(game_textures);
    info!("Game Textures: loaded");
}

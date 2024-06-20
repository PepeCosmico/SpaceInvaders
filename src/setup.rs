use bevy::prelude::*;

use crate::{
    fisics::FisicsPlugin, screens::ScreenPlugins, textures::GameTexturesBuilder,
    units::UnitsPlugins, GameStates,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .init_state::<GameStates>()
            .insert_resource(Time::<Fixed>::from_hz(60.0))
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

    commands.insert_resource(GameTexturesBuilder::build(assets));
    info!("Game Textures: loaded");
}

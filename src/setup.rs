use bevy::prelude::*;

use crate::{
    fisics::{FisicsPlugin, Hitbox},
    screens::ScreenPlugins,
    textures::{GameTextures, GameTexturesBuilder},
    units::{hitbox, player::Player, UnitsPlugins},
    GameStates,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .init_state::<GameStates>()
            .add_event::<TestEvent>()
            .add_systems(Startup, setup_system)
            .add_plugins(FisicsPlugin)
            .add_plugins(ScreenPlugins)
            .add_plugins(UnitsPlugins)
            .add_systems(OnEnter(GameStates::InGame), test)
            .add_systems(
                Update,
                hitbox::collition_event_writer::<With<Test>, With<Player>, TestEvent>,
            );
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

#[derive(Component)]
pub struct Test;
#[derive(Event)]
pub struct TestEvent;

fn test(mut commands: Commands, textures: Res<GameTextures>) {
    commands.spawn((
        SpriteBundle {
            texture: textures.get_texture(crate::textures::Textures::SimpleMissile),
            transform: Transform {
                translation: Vec3::new(300.0, -400.0, 0.0),
                scale: Vec3::splat(6.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Test,
        Hitbox::new(Vec2::new(3.0, 15.0)),
    ));
}

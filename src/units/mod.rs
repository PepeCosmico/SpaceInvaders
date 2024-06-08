use bevy::{app::PluginGroupBuilder, prelude::*};

mod alien;
mod attack;
mod player;

#[derive(Component)]
pub struct Unit;

pub struct UnitsPlugins;

impl PluginGroup for UnitsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();
        group
            .add(alien::AlienPlugin)
            .add(player::PlayerPlugin)
            .add(attack::AttackPlugin)
    }
}

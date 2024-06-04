use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod unit;

pub struct UnitsPlugins;

impl PluginGroup for UnitsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();
        group.add(unit::UnitPlugin)
    }
}

use bevy::{app::PluginGroupBuilder, prelude::*};

mod menu;

pub struct ScreenPlugins;

impl PluginGroup for ScreenPlugins {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();
        group.add(menu::MenuPlugin)
    }
}

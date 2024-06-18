use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_rapier3d::prelude::*;

pub mod level;
pub mod player;

struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(level::LevelPlugin)
            .add(player::PlayerPlugin)
    }
}

struct NeededPlugins;
impl PluginGroup for NeededPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RapierPhysicsPlugin::<NoUserData>::default())
            .add(RapierDebugRenderPlugin::default())
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NeededPlugins)
        .add_plugins(GamePlugins)
        .run();
}

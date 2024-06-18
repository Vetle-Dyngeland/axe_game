use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod input;
pub mod movement;

struct PlayerPlugins;
impl PluginGroup for PlayerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerSetPlugin)
            .add(input::PlayerInputPlugin)
            .add(movement::PlayerMovementPlugin)
    }
}

pub(super) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugins)
            .add_systems(Startup, init.in_set(PlayerSet::Main));
    }
}

#[derive(Component, Debug, Hash)]
pub struct Player;

fn init(mut cmd: Commands) {
    cmd.spawn((Player, Name::new("Player")));
}

struct PlayerSetPlugin;
impl Plugin for PlayerSetPlugin {
    fn build(&self, app: &mut App) {
        use PlayerSet::*;

        app.configure_sets(
            Startup,
            (Pre, Main, Input, Movement, Camera, Visuals, Post).chain(),
        )
        .configure_sets(
            Update,
            (Pre, Main, Input, Movement, Camera, Visuals, Post).chain(),
        )
        .add_systems(
            Startup,
            (
                apply_deferred.after(Pre).before(Main),
                apply_deferred.after(Main).before(Input),
                apply_deferred.after(Input).before(Movement),
                apply_deferred.after(Movement).before(Camera),
                apply_deferred.after(Camera).before(Visuals),
                apply_deferred.after(Visuals).before(Post),
            ),
        )
        .add_systems(
            Update,
            (
                apply_deferred.after(Pre).before(Main),
                apply_deferred.after(Main).before(Input),
                apply_deferred.after(Input).before(Movement),
                apply_deferred.after(Movement).before(Camera),
                apply_deferred.after(Camera).before(Visuals),
                apply_deferred.after(Visuals).before(Post),
            ),
        );
    }
}

#[derive(SystemSet, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum PlayerSet {
    Pre,
    #[default]
    Main,
    Input,
    Movement,
    Camera,
    Visuals,
    Post,
}

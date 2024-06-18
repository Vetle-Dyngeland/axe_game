use super::{Player, PlayerSet};
use bevy::{input::mouse::MouseButtonInput, prelude::*};
use leafwing_input_manager::prelude::*;

pub(super) struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(PlayerSet::Input));
    }
}

fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {
    let mut input_map = InputMap::new([
        (PlayerAction::Jump, KeyCode::Space),
        (PlayerAction::Slide, KeyCode::ShiftLeft),
        (PlayerAction::Sprint, KeyCode::ControlLeft),
        (PlayerAction::Interract, KeyCode::KeyE),
        (PlayerAction::SpecialAttack, KeyCode::KeyQ),
    ]);
    input_map.insert(PlayerAction::Move, VirtualDPad::wasd());
    input_map.insert(PlayerAction::PrimaryAttack, MouseButton::Left);
    input_map.insert(PlayerAction::SecondaryAttack, MouseButton::Right);

    cmd.entity(player_query.single())
        .insert(InputManagerBundle::with_map(input_map));
}

#[derive(Actionlike, Clone, Copy, Debug, Hash, PartialEq, Eq, Reflect)]
pub enum PlayerAction {
    // Movement
    Move,
    Jump,
    Sprint,
    Slide,
    // Interractions
    Interract,
    // Attacking
    PrimaryAttack,
    SecondaryAttack,
    SpecialAttack,
}

use super::{input::PlayerAction, Player, PlayerSet};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) struct PlayerMovementPlugin;
impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(PlayerSet::Movement))
            .insert_resource(KinematicGravity::default())
            .add_systems(
                Update,
                (
                    kinematic_velocity_collisions,
                    kinematic_gravity,
                    kinematic_velocity,
                )
                    .chain()
                    .in_set(PlayerSet::Movement),
            );
    }
}

fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {
    cmd.entity(player_query.single()).insert((
        SpatialBundle {
            transform: Transform::from_xyz(0f32, 1f32, 0f32),
            ..Default::default()
        },
        Collider::capsule_y(0.5f32, 0.25f32),
        RigidBody::KinematicPositionBased,
        KinematicVelocity(Vec3::new(1f32, 0f32, 0f32)),
        KinematicGravityUser::default(),
        KinematicCharacterController {
            slide: true,
            ..Default::default()
        },
    ));
}

#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct KinematicVelocity(pub Vec3);

fn kinematic_velocity(
    mut query: Query<(&KinematicVelocity, &mut KinematicCharacterController)>,
    time: Res<Time>,
) {
    for (vel, mut controller) in query.iter_mut() {
        controller.translation = Some(vel.0 * time.delta_seconds());
    }
}

fn kinematic_velocity_collisions(
    mut query: Query<(&mut KinematicVelocity, &KinematicCharacterControllerOutput)>,
    time: Res<Time>,
) {
    for (mut vel, output) in query.iter_mut() {
        vel.0 = output.effective_translation / time.delta_seconds();
    }
}

#[derive(Component, Clone, Debug, PartialEq)]
/// Gravity scale
pub struct KinematicGravityUser(pub f32);

impl Default for KinematicGravityUser {
    fn default() -> Self {
        Self(1f32)
    }
}

#[derive(Resource, Debug, PartialEq, Clone)]
/// Force
pub struct KinematicGravity(pub Vec3);
impl Default for KinematicGravity {
    fn default() -> Self {
        Self(-Vec3::Y * 4000f32)
    }
}

fn kinematic_gravity(
    gravity: Res<KinematicGravity>,
    mut users_query: Query<(&mut KinematicVelocity, &KinematicGravityUser)>,
) {
    for (mut vel, user) in users_query.iter_mut() {
        vel.0 += gravity.0 * user.0;
    }
}

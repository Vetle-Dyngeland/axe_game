use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub(super) struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
    }
}

fn init(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(50f32, 1f32, 50f32)),
            material: materials.add(Color::rgb_u8(155, 155, 155)),
            transform: Transform::from_xyz(0f32, -0.5f32, 0f32),
            ..Default::default()
        },
        Collider::cuboid(25f32, 0.5f32, 25f32),
        RigidBody::Fixed,
        Name::new("Ground"),
    ));

    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(2f32, 2f32, 2f32)),
            material: materials.add(Color::rgb_u8(200, 200, 200)),
            transform: Transform::from_xyz(5f32, 0f32, 0f32),
            ..Default::default()
        },
        Collider::cuboid(1f32, 1f32, 1f32),
        RigidBody::Fixed,
        Name::new("Box")
    ));

    cmd.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0f32, 10f32, 0f32).looking_at(Vec3::ZERO, Vec3::X),
            ..Default::default()
        },
        Name::new("Camera")
    ));

    cmd.spawn((
        PointLightBundle {
            transform: Transform::from_xyz(0f32, 3f32, 0f32),
            point_light: PointLight {
                intensity: 1500f32,
                range: 5000f32,
                shadows_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        },
        Name::new("Light"),
    ));
}

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::camera::CameraConfig;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, rotate_character);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct BipedalFrame;

fn setup(mut commands: Commands) {
    commands.spawn((
        RigidBody::Dynamic,
        crate::camera::CameraTarget,
        Collider::capsule(0.5, 1.0),
        Transform::from_translation(Vec3::Y * 10.0),
        Player,
        BipedalFrame,
    ));
}

fn rotate_character(
    time: Res<Time>,
    camera_config: Res<CameraConfig>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in &mut query {
        if camera_config
            .y_rotation()
            .angle_between(transform.rotation)
            .to_degrees()
            > 1.0
        {
            transform
                .rotation
                .smooth_nudge(&camera_config.y_rotation(), 20.0, time.delta_secs());
        }
    }
}

fn accellerate(
    mut query: Query<&mut LinearVelocity, (With<Player>, With<BipedalFrame>)>,
    gamepad: Single<&Gamepad, With<crate::input::PrimaryGamepad>>,
) {
}

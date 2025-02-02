use crate::{camera::CameraConfig, states::Vessel};
use avian3d::prelude::*;
use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);

        app.add_systems(FixedUpdate, rotate_character);

        app.add_systems(Update, (accellerate).run_if(in_state(Vessel::Bipedal)));

        app.add_observer(execute_bipedal_actions);
    }
}

#[derive(Event)]
pub enum BipedalAction {
    Accelerate(f32),
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct BipedalFrame {
    forward_acceleration: f32,
    reverse_acceleration: f32,
}

impl BipedalFrame {
    pub fn new(forward_acceleration: f32, reverse_acceleration: f32) -> Self {
        Self {
            forward_acceleration,
            reverse_acceleration,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::from("Bipedal Frame"),
        RigidBody::Dynamic,
        crate::camera::CameraTarget,
        Collider::capsule(0.5, 1.0),
        Transform::from_translation(Vec3::Y * 10.0),
        Player,
        BipedalFrame::new(20.0, 10.0),
        LockedAxes::ROTATION_LOCKED,
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

fn accellerate(mut commands: Commands, query: Query<&Gamepad, With<crate::input::PrimaryGamepad>>) {
    for input in &query {
        if let Some(right_trigger) = input.get(GamepadButton::RightTrigger2) {
            if right_trigger.abs() > 0.01 {
                println!("{}", right_trigger);
                commands.trigger(BipedalAction::Accelerate(right_trigger));
            }
        }
    }
}

fn execute_bipedal_actions(
    trigger: Trigger<BipedalAction>,
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &BipedalFrame, &Transform)>,
) {
    for (mut linear_velocity, frame, transform) in &mut query {
        match trigger.event() {
            BipedalAction::Accelerate(accel) => {
                linear_velocity.0 +=
                    transform.forward() * (accel * time.delta_secs() * frame.forward_acceleration);
            }
        }
    }
}

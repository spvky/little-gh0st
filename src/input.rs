use crate::ui::ToastNotification;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (assign_primary_gamepad, debug_toast));
    }
}

#[derive(Component)]
pub struct PrimaryGamepad;

fn assign_primary_gamepad(
    mut commands: Commands,
    query: Query<(Entity, &Gamepad)>,
    mut gamepad_assigned: Local<bool>,
) {
    if !*gamepad_assigned {
        for (entity, gamepad) in &query {
            if gamepad.just_pressed(GamepadButton::South) {
                commands.entity(entity).insert(PrimaryGamepad);
                *gamepad_assigned = true;
                commands.queue(ToastNotification::new(
                    "Gamepad Assigned",
                    "A Gamepad has been assigned, how neat!",
                ));
            }
        }
    }
}

fn debug_toast(mut commands: Commands, query: Query<&Gamepad, With<PrimaryGamepad>>) {
    for gamepad in &query {
        if gamepad.just_pressed(GamepadButton::RightTrigger) {
            commands.queue(ToastNotification::new(
                "Test Notification",
                "Is it  working yet?",
            ));
        }
    }
}

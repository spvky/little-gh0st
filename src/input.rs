use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, assign_primary_gamepad);
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
                println!("Primary gamepad assigned");
                *gamepad_assigned = true;
            }
        }
    }
}

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            WorldInspectorPlugin::default(),
        ))
        .add_plugins((
            character::CharacterPlugin,
            camera::CameraPlugin,
            environment::EnvironmentPlugin,
            input::InputPlugin,
            states::GameStatesPlugin,
        ))
        .run();
}

mod camera;
mod character;
mod environment;
mod input;
mod states;
mod ui;

use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraConfig::new(
            -30.0,
            0.0,
            Vec3::new(0.0, 6.0, 5.0),
            Limit(-50.0, -30.0),
        ))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (position_camera, rotate_camera))
        .add_observer(rotate_x)
        .add_observer(rotate_y);
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraTarget;

#[derive(Event, Default)]
pub struct RotateCameraX(f32);
#[derive(Event, Default)]
pub struct RotateCameraY(f32);

#[derive(Default)]
pub struct Limit(f32, f32);

impl Limit {
    pub fn clamp(&self, value: f32) -> f32 {
        value.clamp(self.0, self.1)
    }
}

#[derive(Resource, Default)]
pub struct CameraConfig {
    x_angle: f32,
    y_angle: f32,
    offset: Vec3,
    x_limit: Limit,
}

/// Resource to control the cameras rotation and position
impl CameraConfig {
    /// Create a new camera config
    fn new(x_angle: f32, y_angle: f32, offset: Vec3, x_limit: Limit) -> Self {
        Self {
            x_angle,
            y_angle,
            offset,
            x_limit,
        }
    }

    pub fn rotate_x(&mut self, rotation: f32) {
        self.x_angle = self.x_limit.clamp(self.x_angle + rotation);
    }

    pub fn rotate_y(&mut self, rotation: f32) {
        self.y_angle += rotation;
    }

    pub fn get_rotation(&self) -> Quat {
        Quat::from_axis_angle(Vec3::Y, self.y_angle.to_radians())
            * Quat::from_axis_angle(Vec3::X, self.x_angle.to_radians())
    }

    pub fn y_rotation(&self) -> Quat {
        Quat::from_axis_angle(Vec3::Y, self.y_angle.to_radians())
    }

    pub fn x_rotation(&self) -> Quat {
        Quat::from_axis_angle(Vec3::X, self.x_angle.to_radians())
    }

    pub fn get_translation(&self, target: Vec3) -> Vec3 {
        target + (self.y_rotation() * self.offset)
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera3d::default(), MainCamera));
}

fn position_camera(
    time: Res<Time>,
    camera_config: Res<CameraConfig>,
    camera: Single<&mut Transform, With<MainCamera>>,
    target: Single<&Transform, (With<CameraTarget>, Without<MainCamera>)>,
) {
    let mut camera_transform = camera.into_inner();
    let target_transform = target.into_inner();

    let desired_rotation = camera_config.get_rotation();
    let desired_translation = camera_config.get_translation(target_transform.translation);

    camera_transform
        .rotation
        .smooth_nudge(&desired_rotation, 10.0, time.delta_secs());
    camera_transform
        .translation
        .smooth_nudge(&desired_translation, 10.0, time.delta_secs());
}

fn rotate_x(trigger: Trigger<RotateCameraX>, mut camera_config: ResMut<CameraConfig>) {
    camera_config.rotate_x(trigger.event().0);
}

fn rotate_y(trigger: Trigger<RotateCameraY>, mut camera_config: ResMut<CameraConfig>) {
    camera_config.rotate_y(trigger.event().0);
}

fn rotate_camera(
    time: Res<Time>,
    mut commands: Commands,
    query: Query<&Gamepad, With<crate::input::PrimaryGamepad>>,
) {
    for gamepad in &query {
        let right_stick = gamepad.right_stick();

        if right_stick.x.abs() > 0.1 {
            commands.trigger(RotateCameraY(time.delta_secs() * -90.0 * right_stick.x));
        }

        if right_stick.y.abs() > 0.1 {
            commands.trigger(RotateCameraX(time.delta_secs() * 70.0 * right_stick.y));
        }
    }
}

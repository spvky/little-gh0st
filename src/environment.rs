use avian3d::prelude::*;
use bevy::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

pub struct Block {
    translation: Vec3,
    rotation: Quat,
    extents: Vec3,
    color: Color,
}

impl Block {
    pub fn new(translation: Vec3, rotation: Quat, extents: Vec3, color: Color) -> Self {
        Self {
            translation,
            rotation,
            extents,
            color,
        }
    }
}

impl Command for Block {
    fn apply(self, world: &mut World) {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let material_handle = materials.add(self.color);

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh_handle = meshes.add(Cuboid::new(self.extents.x, self.extents.y, self.extents.z));

        world.spawn((
            Transform::from_translation(self.translation).with_rotation(self.rotation),
            Mesh3d(mesh_handle),
            MeshMaterial3d(material_handle),
            RigidBody::Static,
            Collider::cuboid(self.extents.x, self.extents.y, self.extents.z),
        ));
    }
}

fn setup(mut commands: Commands) {
    use bevy::color::palettes::tailwind::*;

    let obelisk_extents = Vec3::new(1.0, 10.0, 1.0);

    commands.queue(Block::new(
        Vec3::ZERO,
        Quat::IDENTITY,
        Vec3::new(10.0, 0.5, 10.0),
        Color::srgb(0.2, 0.3, 0.7),
    ));

    commands.queue(Block::new(
        Vec3::X * 3.0,
        Quat::IDENTITY,
        obelisk_extents,
        AMBER_500.into(),
    ));
    commands.queue(Block::new(
        Vec3::Z * 3.0,
        Quat::IDENTITY,
        obelisk_extents,
        GREEN_500.into(),
    ));
    commands.queue(Block::new(
        Vec3::NEG_X * 3.0,
        Quat::IDENTITY,
        obelisk_extents,
        PURPLE_500.into(),
    ));
    commands.queue(Block::new(
        Vec3::NEG_Z * 3.0,
        Quat::IDENTITY,
        obelisk_extents,
        RED_500.into(),
    ));
}

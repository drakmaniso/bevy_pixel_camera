use bevy::prelude::{
    AppBuilder, Assets, Commands, CoreStage, Handle, IntoSystem, Mesh, Plugin, ResMut, StartupStage,
};
use bevy::render::camera;

/// Provides the camera system, and the quad resource for sprite meshes.
pub struct PixelCameraPlugin;

impl Plugin for PixelCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            setup_pixel_camera_plugin.system(),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            camera::camera_system::<super::PixelProjection>.system(),
        );
    }
}

fn setup_pixel_camera_plugin(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let quad = PixelSpriteQuad(meshes.add(make_quad()));
    commands.insert_resource(quad);
}

use bevy::math::vec2;
use bevy::render::mesh::Indices;

/// Resource inserted by `PixelCameraPlugin`, to replace bevy's default mesh for
/// sprite bundles.
#[derive(Clone)]
pub struct PixelSpriteQuad(Handle<Mesh>);

impl From<Handle<Mesh>> for PixelSpriteQuad {
    fn from(handle: Handle<Mesh>) -> Self {
        Self(handle)
    }
}

impl Into<Handle<Mesh>> for PixelSpriteQuad {
    fn into(self) -> Handle<Mesh> {
        self.0
    }
}

fn make_quad() -> Mesh {
    let north_west = vec2(0.0, 1.0);
    let north_east = vec2(1.0, 1.0);
    let south_west = vec2(0.0, 0.0);
    let south_east = vec2(1.0, 0.0);
    let vertices = [
        (
            [south_west.x, south_west.y, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0],
        ),
        (
            [north_west.x, north_west.y, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0],
        ),
        (
            [north_east.x, north_east.y, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0],
        ),
        (
            [south_east.x, south_east.y, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 1.0],
        ),
    ];

    let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);

    let mut positions = Vec::<[f32; 3]>::new();
    let mut normals = Vec::<[f32; 3]>::new();
    let mut uvs = Vec::<[f32; 2]>::new();
    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}
